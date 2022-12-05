use tauri::{api::process::Command, async_runtime};
use tauri_plugin_log::{LogTarget, LoggerBuilder};

use crate::{
    announcement::AnnouncementConsumer, api::EnchiridionApi, commands, settings::Settings,
    util::get_data_directory,
};

pub fn run() {
    let settings = Settings::new("src/settings/Settings.toml").unwrap();

    Command::new_sidecar("camera")
        .expect("failed to create `camera` binary command")
        .spawn()
        .expect("Failed to spawn sidecar");

    let redis_addr = settings.redis_addr.clone();
    let enchiridion_api_base_url = settings.enchiridion_api_base_url.clone();

    tauri::Builder::default()
        .manage(settings)
        .setup(|app| {
            let handle = app.handle();

            async_runtime::spawn(async move {
                let api = EnchiridionApi::new(get_data_directory(), enchiridion_api_base_url);

                let redis_config = deadpool_redis::Config::from_url(redis_addr);
                let redis_pool = redis_config
                    .create_pool(Some(deadpool_redis::Runtime::Tokio1))
                    .expect("[error] Failed to open redis connection");

                let consumer = AnnouncementConsumer::new(redis_pool, api, handle);
                consumer.consume().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_device_information,
            commands::link,
            commands::unlink,
            commands::is_network_connected,
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
