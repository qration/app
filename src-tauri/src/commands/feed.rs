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
      name: channel.title,
      url: channel.link,
      feed_type: String::from("rss"),
      id: feed_id.clone(),
      favourited: false,
      last_fetched: 0,
    },
    articles: channel.items.into_iter()
      .map(|i| Article {
        id: nanoid!(),
        name: i.title.unwrap_or(String::from("")),
        content: i.description.unwrap_or(String::from("")),
        feed_id: feed_id.clone(),
        media_type: String::from("rss"),
        read: false,
        saved: false,
        date: i.pub_date.unwrap_or(String::from("")),
        url: i.link.unwrap_or(String::from("")),
        media_url: None,
      })
      .collect::<Vec<Article>>(),
  })
}
