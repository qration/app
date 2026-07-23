use serde::{Serialize, Deserialize};
use specta::Type;
use specta_typescript::Number;

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum FeedType {
	Rss,
	Atom,
}

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum MediaType {
	Text,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Feed {
	pub id: String,
	pub feed_name: String,
	pub feed_type: FeedType,
	pub favourited: bool,
	pub feed_url: String,
	#[specta(type = Number)]
	pub last_fetched: Option<i64>,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Enclosure {
	pub url: String,
	pub mime_type: Option<String>,
	#[specta(type = Number)]
	pub length: Option<i64>
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Article {
	pub id: String,
	pub feed_id: String,
	pub guid: String,
	pub name: Option<String>,
	pub description: Option<String>,
	pub content: Option<String>,
	pub url: Option<String>,
	pub date: Option<String>,
	pub media_type: MediaType,
	pub enclosure: Option<Enclosure>,
	pub read: bool,
	pub saved: bool,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct FeedWithArticles {
	pub feed: Feed,
	pub articles: Vec<Article>,
}