#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod queue;

use queue::Consumer;
use redis::Value;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{api::path::resource_dir, async_runtime, App, Env, Manager};

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

static mut RESOURCE_DIR: Option<PathBuf> = None;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            setup(app);

            // async_runtime::spawn(async move {
            //     let redis_instance =
            //         redis::Client::open("")
            //             .expect("Failed to create redis instance");
            //     let redis_connection = Arc::new(Mutex::new(
            //         redis_instance
            //             .get_connection()
            //             .expect("Failed to open redis connection"),
            //     ));

            //     // todo: tauri env
            //     let mut consumer = Consumer::new(redis_connection.clone(), "device-queue-3".into(), "client".into());
            //     loop {
            //         let result = match consumer.consume() {
            //             Ok(result) => result,
            //             Err(_) => break,
            //         };

            //         let mut raw_data: Option<String>= None;
            //         for res in result {
            //             for id in res.ids {
            //                 if let Some(data) = id.map.get("data") {
            //                     if let Value::Data(buffer) = data {
            //                         if let Ok(value) = std::str::from_utf8(buffer) {
            //                             raw_data = Some(value.to_string());
            //                         }
            //                     }
            //                 }
            //             }
            //         }

            //         if let Some(data) = raw_data {
            //             if let Ok(result) = serde_json::from_str::<SyncCreateAnnouncementActionParams>(&data) {
            //                 if result.action == AnnouncementSyncAction::Create {

            //                     let response_data = match reqwest::get(format!("https://enchiridion.stevenhansel.com/device/v1/announcements/{}/media", result.announcement_id)).await {
            //                         Ok(res) => match res.text().await {
            //                             Ok(data) => data,
            //                             Err(e) => panic!("err: {}", e.to_string()),
            //                         },
            //                         Err(e) => panic!("err: {}", e.to_string()),
            //                     };

            //                     println!("response_data: {}", response_data);
            //                     let presigned_url = match serde_json::from_str::<GetAnnouncementMediaPresignedURLResponse>(response_data.as_str()) {
            //                         Ok(data) => data.media,
            //                         Err(e) => panic!("err: {}", e.to_string()),
            //                     };
            //                     let image = match reqwest::get(presigned_url).await {
            //                         Ok(res) => res,
            //                         Err(e) => panic!("err: {}", e.to_string()),
            //                     };
            //                     let text = match image.text().await {
            //                         Ok(text) => text,
            //                         Err(e) => panic!("err: {}", e.to_string()),
            //                     };
            //                     let mut bytes = text.as_bytes();

            //                     unsafe {
            //                         let resource_dir = RESOURCE_DIR
            //                             .clone()
            //                             .expect("Resource Directory is not available");
            //                         let images_dir = &resource_dir.join("images");
            //                         if !images_dir.exists() {
            //                             fs::create_dir_all(images_dir).expect("Error when creating image directory");
            //                         }
                                    
            //                         let mut file = match File::create(images_dir) {
            //                             Err(why) => panic!("couldn't create file: {}", why.to_string()),
            //                             Ok(file) => file,
            //                         };

            //                         match std::io::copy(&mut bytes, &mut file) {
            //                             Err(why) => panic!("couldn't write to: {}", why.to_string()),
            //                             Ok(_) => println!("successfully wrote file"),
            //                         }
            //                     }
            //                 }

            //                 handle
            //                     .emit_all("listen_media_update", "emitted")
            //                     .expect("Error when emitting");
            //             }
            //         }
            //     }
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &App) {
    let resource_dir = resource_dir(app.package_info(), &Env::default()).unwrap();
    unsafe {
        RESOURCE_DIR = Some(resource_dir.clone());
        println!("{}", resource_dir.to_str().unwrap())
    }
}

#[tauri::command]
fn get_images() -> Result<Vec<String>, String> {
    unsafe {
        let resource_dir = RESOURCE_DIR
            .clone()
            .expect("Resource Directory is not available");
        let images_dir = &resource_dir.join("images");

        if !images_dir.exists() {
            fs::create_dir_all(images_dir).expect("Error when creating image directory");
        }

        let images = fs::read_dir(images_dir).expect("Error when reading directory");
        let res = images
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.path().to_str().map(|s| ["asset:///", s].concat()))
            })
            .collect::<Vec<String>>();

        return Ok(res);
    }
}
