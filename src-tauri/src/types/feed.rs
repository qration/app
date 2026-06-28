use serde::{Serialize, Deserialize};
use specta::Type;
use thiserror::Error;

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Feed {
	pub id: String,
	pub name: String,
	pub feed_type: String,
	pub favourited: bool,
	pub url: String,
	pub last_fetched: u32,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub struct Article {
	pub id: String,
	pub name: String,
	pub feed_id: String,
	pub url: String,
	pub saved: bool,
	pub read: bool,
	pub date: String,
	pub media_type: String,
	pub content: String,
	pub media_url: Option<String>,
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