use std::{fs, str::FromStr};

use log::LevelFilter;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

use crate::{commands, settings::Settings};

#[tokio::main]
pub async fn run() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let settings = Settings::new();
    let app_local_data_dir = tauri::api::path::local_data_dir()
        .unwrap()
        .join(settings.bundle_identifier);

    if !app_local_data_dir.exists() {
        fs::create_dir_all(app_local_data_dir.clone())
            .expect("Error when creating app local data dir");
    }

    let sqlite_opt = SqliteConnectOptions::from_str(
        format!(
            "sqlite://{}/data.db",
            app_local_data_dir.to_str().unwrap().to_string()
        )
        .as_str(),
    )
    .unwrap()
    .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(sqlite_opt)
        .await
        .unwrap();

    let migrator_pool = pool.clone();

    tauri::Builder::default()
        .setup(move |app| {
            let migrations_path = app.path_resolver().resolve_resource("migrations").unwrap();

            tokio::spawn(async move {
                let migrator = Migrator::new(migrations_path).await.unwrap();

                migrator.run(&migrator_pool).await.unwrap();
            });

            Ok(())
        })
        .manage(settings)
        .plugin(
            LoggerBuilder::default()
                .with_colors(ColoredLevelConfig::default())
                .level(LevelFilter::Info)
                .level(LevelFilter::Error)
                .level(LevelFilter::Warn)
                .targets([LogTarget::Stdout])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_device_information,
            commands::link,
            commands::unlink,
            commands::is_network_connected,
            commands::spawn_camera,
            commands::spawn_announcement_consumer,
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|_| log::warn!("Something when wrong when initializing the application"));

    log::info!("Application has started running");
}
