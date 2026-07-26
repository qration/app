use atom_syndication::Entry;
use url::Url;

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

pub fn _get_yt_video_id(e: Entry) -> String {
	e.extensions()
		.get("yt")
		.and_then(|ns| ns.get("videoId"))
		.and_then(|exts| exts.first())
		.and_then(|ext| ext.value())
		.unwrap_or_default()
		.to_owned()
}
