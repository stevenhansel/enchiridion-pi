use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tauri::{api::path::resource_dir, async_runtime, Env};

use crate::{commands, consumer::AnnouncementConsumer, device::Device};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnnouncementSyncAction {
    Create,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncCreateAnnouncementActionParams {
    action: AnnouncementSyncAction,
    announcement_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaPresignedURLResponse {
    media: String,
}

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
            commands::get_buildings,
            commands::get_floors,
            commands::create_device,
            commands::get_device_information,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
