mod commands;
mod types;

use commands::*;
use tauri_specta::collect_commands;
use specta_typescript::Typescript;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri_specta::Builder::<tauri::Wry>::new()
    .commands(collect_commands![new_feed]);

  #[cfg(debug_assertions)]
  builder
    .export(Typescript::default(), "../src/lib/util/bindings.ts")
    .expect("Failed to export TypeScript bindings!");

  tauri::Builder::default()
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
}
