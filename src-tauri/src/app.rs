use log::LevelFilter;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

use crate::{commands, settings::Settings};

pub fn run() {
    let settings = Settings::new();

    tauri::Builder::default()
        .manage(settings)
        .setup(|app| {
            // let config = app.config();
            // println!("app_data_dir: {:?}", app_data_dir(&config));
            // println!("app_local_data_dir: {:?}", app_local_data_dir(&config));

            if let Ok(matches) = app.get_cli_matches() {
                log::info!("{:?}", matches);
            }

            Ok(())
        })
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
