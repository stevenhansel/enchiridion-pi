use tauri::{api::path::resource_dir, async_runtime, Env};
use tauri_plugin_log::{LogTarget, LoggerBuilder};

use crate::{announcement::AnnouncementConsumer, api::EnchiridionApi, commands};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            async_runtime::spawn(async move {
                let api = EnchiridionApi::new(resource_dir(handle.package_info(), &Env::default()).unwrap());

                let redis_config =
                    deadpool_redis::Config::from_url("redis://:ac9772178d656aeb6533b2f05c164bade00b58c10fe30586642a319ce3431cee@45.76.147.56:6379".to_string());
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
