use std::sync::{Arc, Mutex};

use tauri::async_runtime;
use tauri_plugin_log::{LogTarget, LoggerBuilder};

use crate::{announcement::AnnouncementConsumer, commands};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            async_runtime::spawn(async move {
                let redis_instance =
                    redis::Client::open("redis://:ac9772178d656aeb6533b2f05c164bade00b58c10fe30586642a319ce3431cee@45.76.147.56:6379").expect("Failed to create redis instance");
                let redis_connection = Arc::new(Mutex::new(
                    redis_instance
                        .get_connection()
                        .expect("Failed to open redis connection"),
                ));

                let consumer = AnnouncementConsumer::new(redis_connection, handle);
                consumer.consume().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_images,
            commands::get_device_information,
            commands::authenticate,
        ])
        .plugin(LoggerBuilder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview,
        ]).build())
        .run(tauri::generate_context!())
        .unwrap_or_else(|_| {
            log::warn!("Something when wrong when initializing the application")
        });

    log::info!("Application has started running");
}
