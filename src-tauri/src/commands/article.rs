use sqlx::{Pool, Sqlite};
use tauri::State;
use crate::{db, types::*};

#[tauri::command]
#[specta::specta]
pub async fn fetch_articles_light(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<ArticleLight>, FeedError> {
  db::fetch_articles_light(&pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_article_content(pool: State<'_, Pool<Sqlite>>, id: String) -> Result<ArticleContent, FeedError> {
  db::fetch_article_content(&pool, id).await
}
