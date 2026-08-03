use crate::db;
use crate::types::{error::*, feed::*};
use crate::util;
use sqlx::{Pool, Sqlite};
use tauri::State;
use url::Url;

#[tauri::command]
#[specta::specta]
pub async fn new_feed(
	pool: State<'_, Pool<Sqlite>>,
	url_string: String,
) -> Result<AddFeedResult, FeedError> {
	let url_parsed = Url::parse(&url_string)
		.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());

	let yt_feed_url = util::youtube::_resolve_youtube_feed_url(&url_parsed).await;
	let fetch_url = yt_feed_url.clone().unwrap_or(url_string);
	let base_url = Url::parse(&fetch_url).unwrap_or(url_parsed);

	if let Ok(_) = db::feed::fetch_feed_by_url(&pool, fetch_url.clone()).await {
		return Err(FeedError::AlreadySubscribed(format!(
			"Already subscribed to {}",
			fetch_url.clone()
		)));
	}

	let feed_type = util::feed::_get_feed_type(yt_feed_url.clone());
	let media_type = util::feed::_get_media_type(feed_type);
	let pf =
		util::feed::_fetch_live_feed(base_url, None, feed_type, media_type).await?;

	db::feed::add_feed(&pool, &pf.feed).await?;
	let new_articles = db::article::add_articles(
		&pool,
		&pf.articles_light,
		&pf.articles_content,
		&pf.transcripts,
	)
	.await?;

	let afr = AddFeedResult {
		feed: pf.feed,
		new_count: new_articles.len(),
		articles_light: pf.articles_light,
	};

	Ok(afr)
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_feeds(
	pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Feed>, FeedError> {
	db::feed::fetch_feeds(&pool).await
}

#[tauri::command]
#[specta::specta]
pub async fn fetch_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<Feed, FeedError> {
	db::feed::fetch_feed(&pool, id).await
}

#[tauri::command]
#[specta::specta]
pub async fn set_star_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
	star: bool,
) -> Result<(), FeedError> {
	db::feed::set_star_feed(&pool, id, star).await
}

#[tauri::command]
#[specta::specta]
pub async fn delete_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<(), FeedError> {
	db::feed::delete_feed(&pool, id).await
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_feed(
	pool: State<'_, Pool<Sqlite>>,
	id: String,
) -> Result<RefreshFeedResult, FeedError> {
	let f = db::feed::fetch_feed(&pool, id).await?;
	let url_parsed = Url::parse(&f.feed_url)
		.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());
	let pf = util::feed::_fetch_live_feed(
		url_parsed,
		Some(f.id),
		Some(f.feed_type),
		util::feed::_get_media_type(Some(f.feed_type)),
	)
	.await?;

	let new_articles = db::article::add_articles(
		&pool,
		&pf.articles_light,
		&pf.articles_content,
		&pf.transcripts,
	)
	.await?;

	let rfr = RefreshFeedResult {
		new_count: new_articles.len(),
		articles_light: new_articles,
	};

	Ok(rfr)
}

#[tauri::command]
#[specta::specta]
pub async fn refresh_feeds(
	pool: State<'_, Pool<Sqlite>>,
) -> Result<RefreshFeedResult, FeedError> {
	let vf = db::feed::fetch_feeds(&pool).await?;
	let mut new_count = 0;
	let mut articles_light = vec![];
	for f in vf {
		println!("refreshing {}", f.feed_name);
		let url_parsed = Url::parse(&f.feed_url)
			.unwrap_or_else(|_| Url::parse("https://qration.net").unwrap());
		let pf = util::feed::_fetch_live_feed(
			url_parsed,
			Some(f.id),
			Some(f.feed_type),
			util::feed::_get_media_type(Some(f.feed_type)),
		)
		.await?;

		println!("parsed {}", f.feed_name);

		let new_articles = db::article::add_articles(
			&pool,
			&pf.articles_light,
			&pf.articles_content,
			&pf.transcripts,
		)
		.await?;
		println!("added {} new articles", new_articles.len());
		new_count += new_articles.len();
		articles_light.extend(new_articles);
	}

	let rfr = RefreshFeedResult {
		new_count,
		articles_light,
	};

	Ok(rfr)
}
