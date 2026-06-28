use std::time::{SystemTime, UNIX_EPOCH};

use rss::Channel;
use nanoid::nanoid;
use crate::types::*;

#[tauri::command]
#[specta::specta]
pub async fn new_feed(url: String) -> Result<FeedWithArticles, FeedError> {
  let bytes = reqwest::get(url)
    .await
    .map_err(|_| FeedError::RequestFailed)?
    .bytes()
    .await
    .map_err(|_| FeedError::StreamFailed)?;

  let channel = Channel::read_from(&bytes[..])
  .map_err(|_| FeedError::ParseFailed)?;

  println!("channel items: {:?}", channel.items);
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
        content: i.description,
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
