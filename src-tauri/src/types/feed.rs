use serde::{Serialize, Deserialize};
use specta::Type;
use specta_typescript::Number;

use crate::types::*;

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum FeedType {
	Rss,
	Atom,
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
pub struct AddFeedResult {
	pub feed: Feed,
	pub articles_light: Vec<ArticleLight>,
	pub articles_content: Vec<ArticleContent>,
}