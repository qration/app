use crate::types::error::FeedError;
use crate::types::feed::{Feed, FeedType};
use sqlx::{Pool, Sqlite};

pub async fn add_feed(
	pool: &Pool<Sqlite>,
	feed: &Feed,
) -> Result<(), FeedError> {
	sqlx::query!(
    "INSERT INTO feeds (id, feed_name, feed_type, favourited, feed_url, last_fetched)
     VALUES (?, ?, ?, ?, ?, ?)
     ON CONFLICT (feed_url) DO NOTHING",
      feed.id,
      feed.feed_name,
      feed.feed_type as _,
      feed.favourited,
      feed.feed_url,
      feed.last_fetched)
  .execute(pool)
  .await
  .map_err(|e| match e {
    sqlx::Error::Database(ref db) if db.is_unique_violation() => FeedError::AlreadySubscribed(format!("Already subscribed to {}", &feed.feed_url)),
    _ => FeedError::DbError(e.to_string()),
  })?;

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
              site_url,
              last_fetched
       FROM feeds"#
	)
	.fetch_all(pool)
	.await
	.map_err(|e| FeedError::DbError(e.to_string()))?;

	Ok(rows)
}

pub async fn fetch_feed(
	pool: &Pool<Sqlite>,
	id: String,
) -> Result<Feed, FeedError> {
	let feed = sqlx::query_as!(
		Feed,
		r#"SELECT id,
              feed_name,
              feed_type AS "feed_type!: FeedType",
              favourited,
              feed_url,
              site_url,
              last_fetched
       FROM feeds
       WHERE id = ?"#,
		id
	)
	.fetch_one(pool)
	.await
	.map_err(|e| FeedError::DbError(e.to_string()))?;

	Ok(feed)
}

pub async fn fetch_feed_by_url(
	pool: &Pool<Sqlite>,
	url: String,
) -> Result<Feed, FeedError> {
	let feed = sqlx::query_as!(
		Feed,
		r#"SELECT id,
              feed_name,
              feed_type AS "feed_type!: FeedType",
              favourited,
              feed_url,
              site_url,
              last_fetched
       FROM feeds
       WHERE feed_url = ?"#,
		url
	)
	.fetch_one(pool)
	.await
	.map_err(|e| FeedError::DbError(e.to_string()))?;

	Ok(feed)
}

pub async fn set_star_feed(
	pool: &Pool<Sqlite>,
	id: String,
	star: bool,
) -> Result<(), FeedError> {
	sqlx::query_as!(
		ArticleContent,
		"UPDATE feeds
     SET favourited = ?
     WHERE id = ?",
		star,
		id
	)
	.execute(pool)
	.await
	.map_err(|e| FeedError::DbError(e.to_string()))?;

	Ok(())
}

pub async fn delete_feed(
	pool: &Pool<Sqlite>,
	id: String,
) -> Result<(), FeedError> {
	sqlx::query_as!(
		Feed,
		r#"DELETE FROM feeds
       WHERE id = ?"#,
		id
	)
	.execute(pool)
	.await
	.map_err(|e| FeedError::DbError(e.to_string()))?;

	Ok(())
}
