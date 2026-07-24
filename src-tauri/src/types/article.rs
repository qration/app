
use serde::{Serialize, Deserialize};
use specta::Type;
use specta_typescript::Number;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct ArticleLight {
	pub id: String,
	pub feed_id: String,
	pub article_guid: String,
	pub article_name: Option<String>,
	pub article_description: Option<String>,
	pub article_url: Option<String>,
	#[specta(type = Number)]
	pub article_date: Option<i64>,
	pub media_type: MediaType,
	pub article_read: bool,
	pub article_saved: bool,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct ArticleContent {
	pub id: String,
	pub content: Option<String>,
	pub enclosure_url: Option<String>,
	pub enclosure_mime_type: Option<String>,
	#[specta(type = Number)]
	pub enclosure_length: Option<i64>
}

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum MediaType {
	Text,
}
