mod commands;
mod types;

use sqlx::{Error, sqlite::SqlitePoolOptions};
use tauri_specta::collect_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Error> {
  let builder = tauri_specta::Builder::<tauri::Wry>::new()
    .commands(collect_commands![commands::new_feed]);

  #[cfg(all(debug_assertions, not(any(target_os = "android", target_os = "ios"))))]
  builder
    .export(specta_typescript::Typescript::default(), "../src/lib/util/bindings.ts")
    .expect("Failed to export TypeScript bindings!");

  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite::memory:")
    .await?;

  sqlx::query(
    "CREATE TABLE IF NOT EXISTS feed (

    )",
  )
  .execute(&pool)
  .await?;

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())  // <-- this line
    .invoke_handler(builder.invoke_handler())
    .setup(move |app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      builder.mount_events(app);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}