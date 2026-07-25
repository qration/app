use std::time::{SystemTime, UNIX_EPOCH};

use crate::{db, types::*};
use atom_syndication::{Feed as AtomFeed, Link};
use nanoid::nanoid;
use rss::Channel;
use sqlx::{Pool, Sqlite};
use tauri::State;
use url::Url;

#[tauri::command]
#[specta::specta]
pub async fn new_feed(
	pool: State<'_, Pool<Sqlite>>,
	url_string: String,
) -> Result<AddFeedResult, FeedError> {
	let url_parsed = Url::parse(&url_string)
		.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());
  
  let yt_feed_url = _resolve_youtube_feed_url(&url_parsed).await;
  let fetch_url = yt_feed_url.to_string().unwrap_or(url_string);
  let base_url = Url::parse(&fetch_url).unwrap_or(url_parsed);

	if let Ok(_) = db::fetch_feed_by_url(&pool, fetch_url).await {
		return Err(FeedError::AlreadySubscribed);
	}

	let pf = _fetch_live_feed(base_url, None).await?;

	db::add_feed(&pool, &pf.feed).await?;
	let new_articles =
		db::add_articles(&pool, &pf.articles_light, &pf.articles_content).await?;

	let afr = AddFeedResult {
		feed: pf.feed,
		new_count: new_articles.len(),
		articles_light: pf.articles_light,
	};

	Ok(afr)
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_feeds(
	pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Feed>, FeedError> {
	db::fetch_feeds(&pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<Feed, FeedError> {
	db::fetch_feed(&pool, id).await
}

#[tauri::command]
#[specta::specta]
pub async fn set_star_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
	star: bool,
) -> Result<(), FeedError> {
	db::set_star_feed(&pool, id, star).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<(), FeedError> {
	db::delete_feed(&pool, id).await
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<RefreshFeedResult, FeedError> {
	let f = db::fetch_feed(&pool, id).await?;
	let url_parsed = Url::parse(&f.feed_url)
		.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());
	let pf = _fetch_live_feed(url_parsed, Some(f.id)).await?;

	let new_articles =
		db::add_articles(&pool, &pf.articles_light, &pf.articles_content).await?;

	let rfr = RefreshFeedResult {
		new_count: new_articles.len(),
		articles_light: new_articles,
	};

	Ok(rfr)
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_feeds(
	pool: State<'_, Pool<Sqlite>>,
) -> Result<RefreshFeedResult, FeedError> {
	let vf = db::fetch_feeds(&pool).await?;
	let mut new_count = 0;
	let mut articles_light = vec![];
	for f in vf {
		let url_parsed = Url::parse(&f.feed_url)
			.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());
		let pf = _fetch_live_feed(url_parsed, Some(f.id)).await?;

		let new_articles =
			db::add_articles(&pool, &pf.articles_light, &pf.articles_content).await?;
		new_count += new_articles.len();
		articles_light.extend(new_articles);
	}

	let rfr = RefreshFeedResult {
		new_count,
		articles_light,
	};

	Ok(rfr)
}

async fn _fetch_live_feed(
	url: Url,
	feed_id: Option<String>,
) -> Result<ParsedFeed, FeedError> {
	let bytes = reqwest::get(url.clone())
		.await
		.map_err(|_| FeedError::RequestFailed)?
		.bytes()
		.await
		.map_err(|_| FeedError::StreamFailed)?;

	let feed_id = feed_id.unwrap_or(nanoid!());

	if let Ok(channel) = Channel::read_from(&bytes[..]) {
		_new_rss_feed(channel, url.clone(), feed_id.clone())
	} else if let Ok(atomfeed) = AtomFeed::read_from(&bytes[..]) {
		_new_atom_feed(atomfeed, url.clone(), feed_id.clone())
	} else {
		Err(FeedError::ParseFailed)
	}
}

fn _new_rss_feed(
	channel: Channel,
	base_url: Url,
	feed_id: String,
) -> Result<ParsedFeed, FeedError> {
	let feed = Feed {
		id: feed_id.clone(),
		feed_name: channel.title,
		feed_type: FeedType::Rss,
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
					media_type: MediaType::Text,
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

fn _new_atom_feed(
	atomfeed: AtomFeed,
	base_url: Url,
	feed_id: String,
) -> Result<ParsedFeed, FeedError> {
	let feed = Feed {
		id: feed_id.clone(),
		feed_name: atomfeed.title.to_string(),
		feed_type: FeedType::Atom,
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

	let (al, ac): (Vec<ArticleLight>, Vec<ArticleContent>) = atomfeed
		.entries
		.into_iter()
		.map(|e| {
			let article_id = nanoid!();

			(
				ArticleLight {
					id: article_id.clone(),
					article_name: Some(e.title.to_string()),
					article_description: _html_to_desc(
						&e.summary.clone().unwrap_or_default(),
					),
					feed_id: feed_id.clone(),
					media_type: MediaType::Text,
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
					content: e
						.content
						.and_then(|c| c.value)
						.or_else(|| e.summary.clone().map(|s| s.to_string()))
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

fn _pick_link(links: &Vec<Link>, rel: &str) -> Option<Link> {
	return links.iter().cloned().find(|l| l.rel == rel);
}

fn _html_to_desc(html: &str) -> Option<String> {
	let text = match html2text::from_read(html.as_bytes(), usize::MAX) {
		Ok(t) => t,
		Err(_) => return None,
	};

	Some(text.split_whitespace().collect::<Vec<&str>>().join(" "))
}

fn _clean_html(html: String, base_url: Url) -> String {
  ammonia::Builder::default()
    .url_relative(ammonia::UrlRelative::RewriteWithBase(base_url))
    .clean(&html)
    .to_string()
}

async fn _resolve_youtube_feed_url(url: &Url) -> Option<String> {
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
  _id_after(html, "channel_id=")
    .or_else(|| _id_after(html, "rel=\"canonical\" href=\"https://www.youtube.com/channel/"))
}

fn _id_after(html: &str, marker: &str) -> Option<String> {
  let start = html.find(marker)? + marker.len();
  let id = html.get(start..start + 24)?;
  _valid_channel_id(id).then(|| id.to_string())
}

fn _channel_feed_url(id: &str) -> String {
  format!("https://www.youtube.com/feeds/videos.xml?channel_id={id}")
}

fn _valid_channel_id(id: &str) -> bool {
  id.len() == 24
    && id.starts_with("UC")
    && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn channel_url_resolves_directly() {
    let url = Url::parse("https://www.youtube.com/channel/UCdc_JyNaB5VJ0gdWUtWfGDg").unwrap();
    assert_eq!(
      _resolve_youtube_feed_url(&url).await,
      Some("https://www.youtube.com/feeds/videos.xml?channel_id=UCdc_JyNaB5VJ0gdWUtWfGDg".to_string())
    );
  }

  #[tokio::test]
  async fn non_youtube_url_is_none() {
    let url = Url::parse("https://example.com/feed.xml").unwrap();
    assert_eq!(_resolve_youtube_feed_url(&url).await, None);
  }
}
