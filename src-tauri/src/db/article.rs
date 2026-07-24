use sqlx::{Pool, Sqlite};

use crate::types::{ArticleContent, ArticleLight, FeedError, MediaType};

pub async fn add_articles(pool: &Pool<Sqlite>, articles_light: &Vec<ArticleLight>, articles_content: &Vec<ArticleContent>) -> Result<usize, FeedError> {
  let mut inserted = 0;

  for (al, ac) in articles_light.iter().zip(articles_content) {
    let n = sqlx::query!(
      "INSERT INTO articles_light
        (id, feed_id, article_guid, article_name, article_description,
        article_url, article_date, media_type, article_read, article_saved)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
      al.id,
      al.feed_id,
      al.article_guid,
      al.article_name,
      al.article_description,
      al.article_url,
      al.article_date,
      al.media_type as _,
      al.article_read,
      al.article_saved
    )
    .execute(pool)
    .await
    .map_err(|_| FeedError::DbFailed)?;

    if n.rows_affected() == 0 {
      continue;
    }

    sqlx::query!(
      "INSERT INTO articles_content
        (id, content, enclosure_url, enclosure_mime_type, enclosure_length)
      VALUES (?, ?, ?, ?, ?)",
      al.id,
      ac.content,
      ac.enclosure_url,
      ac.enclosure_mime_type,
      ac.enclosure_length,
    )
    .execute(pool)
    .await
    .map_err(|_| FeedError::DbFailed)?;

    inserted += 1;
  }

  Ok(inserted)
}

pub async fn fetch_articles_light(pool: &Pool<Sqlite>) -> Result<Vec<ArticleLight>, FeedError> {
  let rows = sqlx::query_as!(
    ArticleLight,
    r#"SELECT id,
              feed_id,
              article_guid,
              article_name,
              article_description,
              article_url,
              article_date,
              media_type AS "media_type!: MediaType",
              article_read,
              article_saved
       FROM articles_light"#
  )
  .fetch_all(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(rows)
}

pub async fn fetch_article_content(pool: &Pool<Sqlite>, id: String) -> Result<ArticleContent, FeedError> {
  let ac = sqlx::query_as!(
    ArticleContent,
    "SELECT id,
            content,
            enclosure_url,
            enclosure_mime_type,
            enclosure_length
    FROM articles_content
    WHERE id = ?",
    id
  )
  .fetch_one(pool)
  .await
  .map_err(|_| FeedError::DbFailed)?;

  Ok(ac)
}