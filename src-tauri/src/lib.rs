use rss::Channel;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code")]
pub enum FeedError {
  #[error("request failed")]
  RequestFailed,
  #[error("stream failed")]
  StreamFailed,
  #[error("parse failed")]
  ParseFailed,
}

#[tauri::command]
async fn new_feed(url: String) -> Result<&'static str, FeedError> {
  println!("URL: {}", url);
  let resp = reqwest::get(url)
    .await
    .map_err(|_| FeedError::RequestFailed)?;
  let bytes = resp.bytes()
    .await
    .map_err(|_| FeedError::StreamFailed)?;

  Channel::read_from(&bytes[..])
    .map(|_| "Added feed")
    .map_err(|_| FeedError::ParseFailed)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![new_feed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
