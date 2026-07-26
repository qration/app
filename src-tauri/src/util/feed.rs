use crate::{
	types::{article::*, error::*, feed::*},
	util,
};
use atom_syndication::{Entry, Feed as AtomFeed, Link};
use futures::future;
use nanoid::nanoid;
use rss::Channel;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub async fn _fetch_live_feed(
	url: Url,
	feed_id: Option<String>,
	feed_type: Option<FeedType>,
	media_type: Option<MediaType>,
) -> Result<ParsedFeed, FeedError> {
	let bytes = reqwest::get(url.clone())
		.await
		.map_err(|_| FeedError::RequestFailed)?
		.bytes()
		.await
		.map_err(|_| FeedError::StreamFailed)?;

	let feed_id = feed_id.unwrap_or(nanoid!());

	if let Ok(channel) = Channel::read_from(&bytes[..]) {
		_new_rss_feed(channel, url.clone(), feed_id.clone(), feed_type, media_type)
	} else if let Ok(atomfeed) = AtomFeed::read_from(&bytes[..]) {
		_new_atom_feed(
			atomfeed,
			url.clone(),
			feed_id.clone(),
			feed_type,
			media_type,
		)
		.await
	} else {
		Err(FeedError::ParseFailed)
	}
}

fn _new_rss_feed(
	channel: Channel,
	base_url: Url,
	feed_id: String,
	feed_type: Option<FeedType>,
	media_type: Option<MediaType>,
) -> Result<ParsedFeed, FeedError> {
	let feed = Feed {
		id: feed_id.clone(),
		feed_name: channel.title,
		feed_type: feed_type.unwrap_or(FeedType::Rss),
		favourited: false,
		feed_url: base_url.to_string(),
		site_url: Some(channel.link),
		last_fetched: Some(
			SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.map(|d| d.as_secs())
				.unwrap_or(0) as i64,
		),
	};

	let (al, ac): (Vec<ArticleLight>, Vec<ArticleContent>) = channel
		.items
		.into_iter()
		.map(|i| {
			let article_id = nanoid!();
			let guid = i
				.guid
				.map(|g| g.value)
				.or_else(|| i.link.clone())
				.unwrap_or_else(|| {
					format!(
						"{}-{}",
						i.title.clone().as_deref().unwrap_or(""),
						i.pub_date.clone().as_deref().unwrap_or("")
					)
				});

			(
				ArticleLight {
					id: article_id.clone(),
					article_name: i.title.clone(),
					article_description: _html_to_desc(
						&i.description.clone().unwrap_or_default(),
					),
					feed_id: feed_id.clone(),
					media_type: media_type.unwrap_or(MediaType::Text),
					article_read: false,
					article_saved: false,
					article_date: dateparser::parse(
						&i.pub_date.clone().unwrap_or_default(),
					)
					.ok()
					.map(|d| d.timestamp()),
					article_url: i.link.clone(),
					article_guid: guid,
				},
				ArticleContent {
					id: article_id.clone(),
					content: i
						.content
						.or_else(|| i.description.clone())
						.map(|html| _clean_html(html, base_url.clone())),
					enclosure_url: None,
					enclosure_mime_type: None,
					enclosure_length: None,
				},
			)
		})
		.unzip();

	Ok(ParsedFeed {
		feed: feed,
		articles_light: al,
		articles_content: ac,
	})
}

async fn _new_atom_feed(
	atomfeed: AtomFeed,
	base_url: Url,
	feed_id: String,
	feed_type: Option<FeedType>,
	media_type: Option<MediaType>,
) -> Result<ParsedFeed, FeedError> {
	let feed = Feed {
		id: feed_id.clone(),
		feed_name: atomfeed.title.to_string(),
		feed_type: feed_type.unwrap_or(FeedType::Atom),
		favourited: false,
		feed_url: base_url.to_string(),
		site_url: _pick_link(&atomfeed.links, "alternate")
			.or_else(|| atomfeed.links.first().cloned())
			.map(|l| l.href.clone()),
		last_fetched: Some(
			SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.map(|d| d.as_secs())
				.unwrap_or(0) as i64,
		),
	};

	let article_iter = atomfeed.entries.into_iter();

	let (al, ac) = future::join_all(article_iter.map(async |e| {
		let article_id = nanoid!();
		let article_content =
			_get_atomfeed_content(e.clone(), base_url.clone(), feed_type)
				.await
				.unwrap();
		let article_description = _get_atomfeed_description(e.clone(), feed_type);

		(
			ArticleLight {
				id: article_id.clone(),
				article_name: Some(e.title.to_string()),
				article_description: article_description,
				feed_id: feed_id.clone(),
				media_type: media_type.unwrap_or(MediaType::Text),
				article_read: false,
				article_saved: false,
				article_date: Some(e.published.unwrap_or(e.updated).timestamp()),
				article_url: _pick_link(&e.links, "alternate")
					.or_else(|| e.links.first().cloned())
					.map(|l| l.href.clone()),
				article_guid: e.id,
			},
			ArticleContent {
				id: article_id.clone(),
				content: article_content,
				enclosure_url: None,
				enclosure_mime_type: None,
				enclosure_length: None,
			},
		)
	}))
	.await
	.into_iter()
	.unzip();

	Ok(ParsedFeed {
		feed: feed,
		articles_light: al,
		articles_content: ac,
	})
}

fn _pick_link(links: &Vec<Link>, rel: &str) -> Option<Link> {
	return links.iter().cloned().find(|l| l.rel == rel);
}

fn _clean_html(html: String, base_url: Url) -> String {
	ammonia::Builder::default()
		.url_relative(ammonia::UrlRelative::RewriteWithBase(base_url))
		.clean(&html)
		.to_string()
}

pub fn _get_feed_type(youtube: Option<String>) -> Option<FeedType> {
	if youtube.is_some() {
		Some(FeedType::Youtube)
	} else {
		None
	}
}

pub fn _get_media_type(ft: Option<FeedType>) -> Option<MediaType> {
	if ft.is_some() {
		let ftu = ft.unwrap();
		if ftu == FeedType::Youtube {
			Some(MediaType::Video)
		} else {
			Some(MediaType::Text)
		}
	} else {
		None
	}
}

async fn _get_atomfeed_content(
	e: Entry,
	base_url: Url,
	feed_type: Option<FeedType>,
) -> Result<Option<String>, FeedError> {
	if feed_type.is_some_and(|f| f == FeedType::Youtube) {
		// TODO: make our own transcript fetcher, because this one doesn't work

		// let api = YouTubeTranscriptApi::new(None, None, None)
		// 	.map_err(|_| FeedError::ParseFailed)?;
		// let id = _get_yt_video_id(e);
		// println!("id {}", id);
		// match api.fetch_transcript(&id, &["en", "en-US"], false).await {
		// 	Ok(res) => Ok(Some(res.snippets.iter().map(|s|
		// 		format!("[{:.1}-{:.1}s] {}",
		// 			s.start,
		// 			s.start + s.duration,
		// 			s.text)
		// 	).collect::<Vec<String>>().join("\n"))),
		// 	Err(e) => Ok(Some(e.to_string())),
		// }
		Ok(util::youtube::_get_yt_content(e))
	} else {
		Ok(
			e.content
				.and_then(|c| c.value)
				.or_else(|| e.summary.clone().map(|s| s.to_string()))
				.map(|html| _clean_html(html, base_url.clone())),
		)
	}
}

fn _get_atomfeed_description(
	e: Entry,
	feed_type: Option<FeedType>,
) -> Option<String> {
	if feed_type.is_some_and(|f| f == FeedType::Youtube) {
		util::youtube::_get_yt_desc(e)
	} else {
		_html_to_desc(&e.summary.clone().unwrap_or_default())
	}
}

fn _html_to_desc(html: &str) -> Option<String> {
	let text = match html2text::from_read(html.as_bytes(), usize::MAX) {
		Ok(t) => t,
		Err(_) => return None,
	};

	Some(text.split_whitespace().collect::<Vec<&str>>().join(" "))
}
