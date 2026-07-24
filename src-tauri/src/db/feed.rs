use sqlx::{Pool, Sqlite};
use crate::types::{Feed, FeedError, FeedType};

pub async fn add_feed(pool: &Pool<Sqlite>, feed: &Feed) -> Result<(), FeedError> {
  sqlx::query!(
    "INSERT INTO feeds (id, feed_name, feed_type, favourited, feed_url, last_fetched)
     VALUES (?, ?, ?, ?, ?, ?)
     ON CONFLICT (id) DO NOTHING",
      feed.id,
      feed.feed_name,
      feed.feed_type as _,
      feed.favourited,
      feed.feed_url,
      feed.last_fetched)
  .execute(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(())
}

pub async fn fetch_feeds(pool: &Pool<Sqlite>) -> Result<Vec<Feed>, FeedError> {
  let rows = sqlx::query_as!(
    Feed,
    r#"SELECT id,
              feed_name,
              feed_type AS "feed_type!: FeedType",
              favourited,
              feed_url,
              last_fetched
       FROM feeds"#)
  .fetch_all(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(rows)
}

pub async fn fetch_feed(pool: &Pool<Sqlite>, id: String) -> Result<Feed, FeedError> {
  let feed = sqlx::query_as!(
    Feed,
    r#"SELECT id,
              feed_name,
              feed_type AS "feed_type!: FeedType",
              favourited,
              feed_url,
              last_fetched
       FROM feeds
       WHERE id = ?"#, id)
  .fetch_one(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(feed)
}