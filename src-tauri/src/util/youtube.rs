use atom_syndication::{extension::Extension, Entry};
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

pub fn _get_yt_content(e: Entry) -> Option<String> {
	let group = e.extensions().get("media")?.get("group")?.first()?;

	let description = child(group, "description")?.value()?;

	let cmm = child(group, "community")?;
	let views = child(cmm, "statistics")?.attrs().get("views")?;
	let likes = child(cmm, "starRating")?.attrs().get("count")?;

	Some(format!(
		"<p>{}</p>
<p>{} views &bullet; {} likes</p>",
		description, views, likes
	))
}

pub fn _get_yt_desc(e: Entry) -> Option<String> {
	let group = e.extensions().get("media")?.get("group")?.first()?;

	let description = child(group, "description")?.value()?;

	Some(description.to_owned())
}
