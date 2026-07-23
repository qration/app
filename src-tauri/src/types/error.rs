use serde::Serialize;
use specta::Type;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Type)]
pub enum FeedError {
	#[error("request failed")]
  RequestFailed,
  #[error("stream failed")]
  StreamFailed,
  #[error("parse failed")]
  ParseFailed,
  #[error("db failed")]
  DbFailed,
}