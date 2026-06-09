use rss::Channel;

#[tauri::command]
async fn new_feed(url: String) -> Result<&'static str, &'static str> {
  println!("URL: {}", url);
  let resp = reqwest::get(url).await;
  if resp.is_ok() {
    let res = resp.unwrap();
    let bytes = res.bytes().await;
    if bytes.is_ok() {
      let channel = Channel::read_from(&(bytes.unwrap())[..]);
      if channel.is_ok() {
        Ok("Added feed")
      } else {
        Err("Could not parse feed!")
      }
    } else {
      Err("Something went wrong. Try again later.")
    }
  } else {
    Err("Something went wrong. Try again later.")
  }
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
