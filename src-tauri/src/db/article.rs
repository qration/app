use sqlx::{Pool, Sqlite};

use crate::types::article::*;
use crate::types::error::FeedError;

pub async fn add_articles(
	pool: &Pool<Sqlite>,
	articles_light: &Vec<ArticleLight>,
	articles_content: &Vec<ArticleContent>,
) -> Result<Vec<ArticleLight>, FeedError> {
	let mut articles = vec![];

	for (al, ac) in articles_light.iter().zip(articles_content) {
		let n = sqlx::query!(
			"INSERT INTO articles_light
        (id, feed_id, article_guid, article_name, article_description,
        article_url, article_date, media_type, article_read, article_saved)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT DO NOTHING",
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
		.map_err(|_| FeedError::DbError)?;

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
		.map_err(|_| FeedError::DbError)?;

		articles.push(al.clone());
	}

	Ok(articles)
}

pub async fn fetch_articles_light(
	pool: &Pool<Sqlite>,
) -> Result<Vec<ArticleLight>, FeedError> {
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
       FROM articles_light
       ORDER BY article_date DESC"#
	)
	.fetch_all(pool)
	.await
	.map_err(|_| FeedError::DbError)?;

	Ok(rows)
}

pub async fn fetch_article_content(
	pool: &Pool<Sqlite>,
	id: String,
) -> Result<ArticleContent, FeedError> {
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
	.map_err(|_| FeedError::DbError)?;

	Ok(ac)
}

pub async fn mark_article_read(
	pool: &Pool<Sqlite>,
	id: String,
) -> Result<(), FeedError> {
	sqlx::query_as!(
		ArticleContent,
		"UPDATE articles_light
     SET article_read = TRUE
     WHERE id = ?",
		id
	)
	.execute(pool)
	.await
	.map_err(|_| FeedError::DbError)?;

	Ok(())
}

pub async fn set_save_article(
	pool: &Pool<Sqlite>,
	id: String,
	save: bool,
) -> Result<(), FeedError> {
	sqlx::query_as!(
		ArticleContent,
		"UPDATE articles_light
     SET article_saved = ?
     WHERE id = ?",
		save,
		id
	)
	.execute(pool)
	.await
	.map_err(|_| FeedError::DbError)?;

	Ok(())
}
