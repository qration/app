use crate::types::{article::TranscriptSnippet, error::FeedError};
use atom_syndication::{extension::Extension, Entry};
use url::Url;

// InnerTube's public web key. The WEB client no longer returns caption tracks,
// so we ask as the ANDROID client, which still does.
const INNERTUBE_KEY: &str = "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";
const INNERTUBE_PLAYER_URL: &str = "https://www.youtube.com/youtubei/v1/player";

pub async fn _resolve_youtube_feed_url(url: &Url) -> Option<String> {
	let host = url.host_str()?;
	if !matches!(host, "youtube.com" | "www.youtube.com" | "m.youtube.com") {
		return None;
	}

	let mut segments = url.path_segments()?;
	let first = segments.next()?;

	if first == "channel" {
		let id = segments.next()?;
		return _valid_channel_id(id).then(|| _channel_feed_url(id));
	}

	if first.starts_with('@') {
		let html = reqwest::get(url.as_str()).await.ok()?.text().await.ok()?;
		let id = _scrape_channel_id(&html)?;
		return Some(_channel_feed_url(&id));
	}

	None
}

fn _scrape_channel_id(html: &str) -> Option<String> {
	_id_after(html, "channel_id=").or_else(|| {
		_id_after(
			html,
			"rel=\"canonical\" href=\"https://www.youtube.com/channel/",
		)
	})
}

fn _id_after(html: &str, marker: &str) -> Option<String> {
	let start = html.find(marker)? + marker.len();
	let id = html.get(start..start + 24)?;
	_valid_channel_id(id).then(|| id.to_string())
}

fn _channel_feed_url(id: &str) -> String {
	format!("https://www.youtube.com/feeds/videos.xml?channel_id={id}")
}

pub fn _valid_channel_id(id: &str) -> bool {
	id.len() == 24
		&& id.starts_with("UC")
		&& id
			.bytes()
			.all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn child<'a>(ext: &'a Extension, name: &str) -> Option<&'a Extension> {
	ext.children().get(name)?.first()
}

pub fn _get_yt_video_id(e: Entry) -> String {
	e.extensions()
		.get("yt")
		.and_then(|ns| ns.get("videoId"))
		.and_then(|exts| exts.first())
		.and_then(|ext| ext.value())
		.unwrap_or_default()
		.to_owned()
}

pub async fn _fetch_transcript(
	video_id: &str,
	lang: &str,
) -> Result<Vec<TranscriptSnippet>, FeedError> {
	let client = reqwest::Client::new();

	let body = client
		.post(format!("{INNERTUBE_PLAYER_URL}?key={INNERTUBE_KEY}"))
		.json(&serde_json::json!({
			"videoId": video_id,
			"context": {
				"client": {
					"clientName": "ANDROID",
					"clientVersion": "20.10.38",
					"androidSdkVersion": 30,
					"hl": lang,
				}
			}
		}))
		.send()
		.await
		.map_err(|e| FeedError::RequestFailed(e.to_string()))?
		.text()
		.await
		.map_err(|e| FeedError::StreamFailed(e.to_string()))?;

	let player: serde_json::Value = serde_json::from_str(&body)
		.map_err(|e| FeedError::ParseFailed(e.to_string()))?;

	let tracks = player
		.pointer("/captions/playerCaptionsTracklistRenderer/captionTracks")
		.and_then(|t| t.as_array())
		.ok_or(FeedError::TranscriptUnavailable(format!(
			"Couldn't get transcript for video {}",
			video_id
		)))?;

	let track_url =
		_pick_caption_track(tracks).ok_or(FeedError::TranscriptUnavailable(
			format!("Couldn't get transcript for video {}", video_id),
		))?;

	let timedtext = client
		.get(track_url)
		.send()
		.await
		.map_err(|e| FeedError::RequestFailed(e.to_string()))?
		.text()
		.await
		.map_err(|e| FeedError::StreamFailed(e.to_string()))?;

	let parsed: serde_json::Value = serde_json::from_str(&timedtext)
		.map_err(|e| FeedError::ParseFailed(e.to_string()))?;

	Ok(_parse_timedtext(&parsed))
}

/// Prefers an English track, falling back to whatever the video offers, and
/// swaps the track's `fmt=srv3` for `fmt=json3` so it comes back as JSON.
fn _pick_caption_track(tracks: &[serde_json::Value]) -> Option<Url> {
	let track = tracks
		.iter()
		.find(|t| {
			t.get("languageCode")
				.and_then(|l| l.as_str())
				.is_some_and(|l| l.starts_with("en"))
		})
		.or_else(|| tracks.first())?;

	let mut url = Url::parse(track.get("baseUrl")?.as_str()?).ok()?;
	let pairs: Vec<(String, String)> = url
		.query_pairs()
		.filter(|(k, _)| k != "fmt")
		.map(|(k, v)| (k.into_owned(), v.into_owned()))
		.collect();

	url
		.query_pairs_mut()
		.clear()
		.extend_pairs(pairs)
		.append_pair("fmt", "json3");

	Some(url)
}

fn _parse_timedtext(timedtext: &serde_json::Value) -> Vec<TranscriptSnippet> {
	let Some(events) = timedtext.get("events").and_then(|e| e.as_array()) else {
		return Vec::new();
	};

	events
		.iter()
		.filter_map(|e| {
			let text = e
				.get("segs")?
				.as_array()?
				.iter()
				.filter_map(|s| s.get("utf8").and_then(|u| u.as_str()))
				.collect::<String>();

			let text = text.trim();
			if text.is_empty() {
				return None;
			}

			Some(TranscriptSnippet {
				text: text.to_string(),
				start: e.get("tStartMs")?.as_f64()? / 1000.0,
				duration: e.get("dDurationMs").and_then(|d| d.as_f64()).unwrap_or(0.0)
					/ 1000.0,
			})
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn fetches_transcript_snippets() {
		let snippets = _fetch_transcript("jNQXAC9IVRw", "en").await.unwrap();

		assert!(!snippets.is_empty(), "expected at least one snippet");
		assert!(
			snippets.iter().all(|s| !s.text.trim().is_empty()),
			"snippets should never carry empty text"
		);
		assert!(
			snippets.windows(2).all(|w| w[0].start <= w[1].start),
			"snippets should be in chronological order"
		);
	}

	#[tokio::test]
	async fn reports_unavailable_for_video_without_captions() {
		// Video ids are 11 chars, so this parses as an id but resolves to nothing.
		let res = _fetch_transcript("00000000000", "en").await;

		assert!(matches!(res, Err(FeedError::TranscriptUnavailable(_))));
	}

	#[test]
	fn parse_timedtext_joins_segments_and_converts_ms() {
		let raw = serde_json::json!({
			"events": [
				{ "tStartMs": 1200, "dDurationMs": 2160,
					"segs": [{ "utf8": "hello" }, { "utf8": " world" }] },
				{ "tStartMs": 5000, "dDurationMs": 1000, "segs": [{ "utf8": "  " }] },
				{ "tStartMs": 7000, "segs": [{ "utf8": "no duration" }] }
			]
		});

		let snippets = _parse_timedtext(&raw);

		assert_eq!(snippets.len(), 2, "blank-only events should be dropped");
		assert_eq!(snippets[0].text, "hello world");
		assert_eq!(snippets[0].start, 1.2);
		assert_eq!(snippets[0].duration, 2.16);
		assert_eq!(snippets[1].duration, 0.0);
	}
}

pub fn _get_yt_content(e: Entry) -> Option<String> {
	let group = e.extensions().get("media")?.get("group")?.first()?;

	let description = child(group, "description")?.value()?;

	let cmm = child(group, "community")?;
	let views = child(cmm, "statistics")?.attrs().get("views")?;
	let likes = child(cmm, "starRating")?.attrs().get("count")?;

	Some(format!(
		r#"<p>{}</p>
<p><Icon icon="tabler:eye"/> {} views &bullet;<Icon icon="tabler:thumb-up"/> {} likes</p>"#,
		description, views, likes
	))
}

pub fn _get_yt_desc(e: Entry) -> Option<String> {
	let group = e.extensions().get("media")?.get("group")?.first()?;

	let description = child(group, "description")?.value()?;

	Some(description.to_owned())
}
