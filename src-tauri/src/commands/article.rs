use crate::db;
use crate::types::{article::*, error::*};
use sqlx::{Pool, Sqlite};
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn fetch_articles_light(
	pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<ArticleLight>, FeedError> {
	db::article::fetch_articles_light(&pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_article_content(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<(ArticleContent, Option<Transcript>), FeedError> {
	db::article::mark_article_read(&pool, id.clone()).await?;
	Ok((
		db::article::fetch_article_content(&pool, id.clone()).await?,
		db::article::fetch_transcript(&pool, id.clone()).await?,
	))
}

#[tauri::command]
#[specta::specta]
pub async fn set_save_article(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
	save: bool,
) -> Result<(), FeedError> {
	db::article::set_save_article(&pool, id, save).await
}
