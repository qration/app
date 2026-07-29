mod commands;
mod db;
mod types;
mod util;

use commands::*;
use sqlx::sqlite::{
	SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions,
};
use sqlx::{Pool, Sqlite};
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;
use tauri::Manager;
use tauri_specta::collect_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn Error>> {
	let builder =
		tauri_specta::Builder::<tauri::Wry>::new().commands(collect_commands![
			feed::new_feed,
			feed::fetch_feeds,
			feed::fetch_feed,
			article::fetch_article_content,
			article::fetch_articles_light,
			article::fetch_transcript,
			feed::set_star_feed,
			article::set_save_article,
			feed::delete_feed,
			feed::refresh_feed,
			feed::refresh_feeds,
		]);

	#[cfg(all(
		debug_assertions,
		not(any(target_os = "android", target_os = "ios"))
	))]
	builder
		.export(
			specta_typescript::Typescript::default(),
			"../src/lib/util/bindings.ts",
		)
		.expect("Failed to export TypeScript bindings!");

	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init()) // <-- this line
		.plugin(tauri_plugin_notifications::init()) // <-- this line
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			if cfg!(debug_assertions) {
				app.handle().plugin(
					tauri_plugin_log::Builder::default()
						.level(log::LevelFilter::Info)
						.build(),
				)?;
			}

			let dir = app.path().data_dir()?;
			std::fs::create_dir_all(&dir)?;

			let pool = _db_setup(dir)?;

			app.manage(pool);
			builder.mount_events(app);
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	Ok(())
}

fn _db_setup(dir: PathBuf) -> Result<Pool<Sqlite>, Box<dyn Error>> {
	let sqlx_opts = SqliteConnectOptions::new()
		.filename(dir.join("app.db"))
		.create_if_missing(true)
		.journal_mode(SqliteJournalMode::Wal)
		.foreign_keys(true)
		.busy_timeout(Duration::from_secs(5));

	let pool = tauri::async_runtime::block_on(async {
		let pool = SqlitePoolOptions::new()
			.max_connections(4)
			.connect_with(sqlx_opts)
			.await?;

		sqlx::migrate!("./migrations").run(&pool).await?;
		Ok::<_, Box<dyn Error>>(pool)
	})?;

	Ok(pool)
}
