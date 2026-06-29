use serde::{Serialize, Deserialize};
use specta::Type;
use specta_typescript::Number;
use thiserror::Error;

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FeedType {
	Rss,
	Atom,
}

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
	Text,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Feed {
	pub id: String,
	pub name: String,
	pub feed_type: FeedType,
	pub favourited: bool,
	pub url: String,
	#[specta(type = Number)]
	pub last_fetched: Option<u64>,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Enclosure {
	pub url: String,
	pub mime_type: Option<String>,
	#[specta(type = Number)]
	pub length: Option<u64>
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

#[derive(Debug, Error, Serialize, Type)]
pub enum FeedError {
	#[error("request failed")]
  RequestFailed,
  #[error("stream failed")]
  StreamFailed,
  #[error("parse failed")]
  ParseFailed,
}