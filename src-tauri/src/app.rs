use tauri_plugin_log::{LogTarget, LoggerBuilder};

use crate::{
    commands,
    settings::Settings,
};

pub fn run() {
    let settings = Settings::new();

    tauri::Builder::default()
        .manage(settings)
        .setup(|app| {
            if let Ok(matches) = app.get_cli_matches() {
                println!("{:?}", matches);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_device_information,
            commands::link,
            commands::unlink,
            commands::is_network_connected,
            commands::spawn_camera,
            commands::spawn_announcement_consumer,
        ])
        .plugin(
            LoggerBuilder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .run(tauri::generate_context!())
        .unwrap_or_else(|_| log::warn!("Something when wrong when initializing the application"));

    log::info!("Application has started running");
}
