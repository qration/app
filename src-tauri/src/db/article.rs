use crate::types::article::*;
use crate::types::error::FeedError;
use crate::types::feed::FeedType;
use sqlx::types::Json;
use sqlx::{Pool, Sqlite};

pub async fn add_articles(
	pool: &Pool<Sqlite>,
	articles_light: &Vec<ArticleLight>,
	articles_content: &Vec<ArticleContent>,
	transcripts: &Vec<Transcript>,
) -> Result<Vec<ArticleLight>, FeedError> {
	let mut tx = pool.begin().await.map_err(|_| FeedError::DbError)?;
	let mut articles = vec![];

	for (al, ac) in articles_light.iter().zip(articles_content) {
		let existing_row = sqlx::query!(
			"SELECT l.id, l.article_name, l.article_description, l.article_date, c.content
			FROM articles_light l LEFT JOIN articles_content c ON c.id = l.id
			WHERE l.feed_id = ? AND l.article_guid = ?",
			al.feed_id,
			al.article_guid,
		)
		.fetch_optional(&mut *tx)
		.await
		.map_err(|_| FeedError::DbError)?;

		let article_id = match &existing_row {
			Some(row) => {
				let unchanged = row.article_name == al.article_name.clone().unwrap()
					&& row.article_description == al.article_description
					&& row.article_date == al.article_date
					&& row.content == ac.content;
				if unchanged {
					continue;
				}
				row.id.clone()
			}
			None => al.id.clone(),
		};

		sqlx::query!(
			"INSERT INTO articles_light
        (id, feed_id, article_guid, article_name, article_description,
        article_url, article_date, media_type, article_read, article_saved)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT(feed_id, article_guid) DO UPDATE SET
				article_name = excluded.article_name,
				article_description = excluded.article_description,
				article_date = excluded.article_date",
			article_id,
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
		.execute(&mut *tx)
		.await
		.map_err(|_| FeedError::DbError)?;

		sqlx::query!(
			"INSERT INTO articles_content
        (id, content, enclosure_url, enclosure_mime_type, enclosure_length)
      VALUES (?, ?, ?, ?, ?)
			ON CONFLICT(id) DO UPDATE SET
				content = excluded.content",
			al.id,
			ac.content,
			ac.enclosure_url,
			ac.enclosure_mime_type,
			ac.enclosure_length,
		)
		.execute(&mut *tx)
		.await
		.map_err(|_| FeedError::DbError)?;

		let mut a = al.clone();
		a.id = article_id;
		articles.push(a);
	}

	for tr in transcripts.iter() {
		sqlx::query!(
			"INSERT INTO transcripts
				(id, article_id, video_id, lang, feed_type, snippets)
			VALUES (?, ?, ?, ?, ?, ?)
			ON CONFLICT(video_id, lang) DO UPDATE SET
				snippets = excluded.snippets",
			tr.id,
			tr.article_id,
			tr.video_id,
			tr.lang,
			tr.feed_type,
			Json(tr.snippets.clone())
		)
		.execute(&mut *tx)
		.await
		.map_err(|_| FeedError::DbError)?;
	}

	tx.commit().await.map_err(|_| FeedError::DbError)?;
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

pub async fn fetch_transcript(
	pool: &Pool<Sqlite>,
	id: String,
) -> Result<Option<Transcript>, FeedError> {
	let tr = sqlx::query_as!(
		Transcript,
		r#"SELECT id,
						article_id,
						video_id,
						lang,
						feed_type AS "feed_type!: FeedType",
						snippets AS "snippets: Json<Vec<TranscriptSnippet>>"
    FROM transcripts
    WHERE article_id = ?"#,
		id
	)
	.fetch_optional(pool)
	.await
	.map_err(|_| FeedError::DbError)?;

	Ok(tr)
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
