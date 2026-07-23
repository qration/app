use sqlx::{Pool, Sqlite};
use crate::types::{Feed, FeedError, FeedType};

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

pub async fn add_feed(pool: &Pool<Sqlite>, feed: &Feed) -> Result<(), FeedError> {
  sqlx::query!(
    "INSERT INTO feeds (id, feed_name, feed_type, favourited, feed_url, last_fetched)
     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
      feed.id,
      feed.feed_name,
      feed.feed_type,
      feed.favourited,
      feed.feed_url,
      feed.last_fetched)
  .execute(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(())
}