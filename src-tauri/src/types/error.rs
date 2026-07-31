use serde::Serialize;
use specta::Type;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Type)]
#[serde(tag = "type", content = "message")]
pub enum FeedError {
	#[error("request failed")]
	RequestFailed(String),
	#[error("stream failed")]
	StreamFailed(String),
	#[error("parse failed")]
	ParseFailed(String),
	#[error("db failed")]
	DbError(String),
	#[error("already subscribed")]
	AlreadySubscribed(String),
	#[error("transcript unavailable")]
	TranscriptUnavailable(String),
}
