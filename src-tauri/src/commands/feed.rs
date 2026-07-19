use std::time::{SystemTime, UNIX_EPOCH};

use rss::Channel;
use atom_syndication::{Feed as AtomFeed, Link};
use nanoid::nanoid;
use url::Url;
use crate::types::*;

#[tauri::command]
#[specta::specta]
pub async fn new_feed(url_string: String) -> Result<FeedWithArticles, FeedError> {
  let url_parsed = Url::parse(&url_string).unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());

  let bytes = reqwest::get(url_string)
    .await
    .map_err(|_| FeedError::RequestFailed)?
    .bytes()
    .await
    .map_err(|_| FeedError::StreamFailed)?;

  if let Ok(channel) = Channel::read_from(&bytes[..]) {
    _new_rss_feed(channel, url_parsed)
  } else if let Ok(atomfeed) = AtomFeed::read_from(&bytes[..]) {
    _new_atom_feed(atomfeed, url_parsed)
  } else {
    Err(FeedError::ParseFailed)
  }
}

fn _new_rss_feed(channel: Channel, base_url: Url) -> Result<FeedWithArticles, FeedError> {
  let feed_id = nanoid!();
  Ok(FeedWithArticles {
    feed: Feed {
      id: feed_id.clone(),
      name: channel.title,
      feed_type: FeedType::Rss,
      favourited: false,
      url: channel.link,
      last_fetched: Some(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)),
    },
    articles: channel.items.into_iter()
      .map(|i| Article {
        id: nanoid!(),
        name: i.title.clone(),
        description: _html_to_desc(&i.description.clone().unwrap_or_default()),
        content: i.content
          .or_else(|| i.description.clone())
          .map(|html| _clean_html(html, base_url.clone())),
        feed_id: feed_id.clone(),
        media_type: MediaType::Text,
        read: false,
        saved: false,
        date: i.pub_date.clone(),
        url: i.link.clone(),
        enclosure: None,
        guid: i.guid.map(|g| g.value)
          .or_else(|| i.link.clone())
          .unwrap_or_else(|| format!("{}-{}",
            i.title.clone().as_deref().unwrap_or(""),
            i.pub_date.clone().as_deref().unwrap_or(""))),
      })
      .collect::<Vec<Article>>(),
  })
}

fn _new_atom_feed(atomfeed: AtomFeed, base_url: Url) -> Result<FeedWithArticles, FeedError> {
  let feed_id = nanoid!();
  Ok(FeedWithArticles {
    feed: Feed {
      id: feed_id.clone(),
      name: atomfeed.title.to_string(),
      feed_type: FeedType::Atom,
      favourited: false,
      url: _pick_link(&atomfeed.links, "alternate")
        .or_else(|| atomfeed.links.first().cloned())
        .map(|l| l.href.clone())
        .unwrap_or_default(),
      last_fetched: Some(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)),
    },
    articles: atomfeed.entries.into_iter()
      .map(|e| Article {
        id: nanoid!(),
        name: Some(e.title.to_string()),
        description: _html_to_desc(&e.summary.clone().unwrap_or_default()),
        content: e.content
          .and_then(|c| c.value)
          .or_else(|| e.summary.clone().map(|s| s.to_string()))
          .map(|html| _clean_html(html, base_url.clone())),
        feed_id: feed_id.clone(),
        media_type: MediaType::Text,
        read: false,
        saved: false,
        date: Some(e.published.unwrap_or(e.updated).to_rfc3339()),
        url: _pick_link(&e.links, "alternate")
          .or_else(|| e.links.first().cloned())
          .map(|l| l.href.clone()),
        enclosure: None,
        guid: e.id,
      })
      .collect::<Vec<Article>>(),
  })
}

fn _pick_link(links: &Vec<Link>, rel: &str) -> Option<Link> {
  return links.iter().cloned().find(|l| l.rel == rel)
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