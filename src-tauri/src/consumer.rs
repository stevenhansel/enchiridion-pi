use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Serialize, Deserialize};

use crate::{device::DeviceInformation, queue::Consumer};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnnouncementSyncAction {
    Create,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementConsumerPayload {
    action: AnnouncementSyncAction,
    announcement_id: i32,
}

pub struct AnnouncementConsumer {
    _device: DeviceInformation,
    _redis: Arc<Mutex<redis::Connection>>,

    directory: PathBuf,
}

impl AnnouncementConsumer {
    pub fn new(_device: DeviceInformation, _redis: Arc<Mutex<redis::Connection>>, directory: PathBuf) -> Self {
        AnnouncementConsumer {
            _device,
            _redis,
            directory,
        }
    }

    pub fn queue_name_builder(&self) -> String {
        format!("device-queue-{}", self._device.id)
    }

    pub async fn consume(&self) {
        let mut consumer = Consumer::new(
            self._redis.clone(),
            self.queue_name_builder(),
            self._device.id.to_string(),
        );

        loop {
//             let result = match consumer.consume() {
//                 Ok(res) => res,
//                 Err(_) => break,
//             };
        }
    }
}

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

//                     let resource_dir = RESOURCE_DIR
//                         .clone()
//                         .expect("Resource Directory is not available");
//                     let images_dir = &resource_dir.join("images");
//                     if !images_dir.exists() {
//                         fs::create_dir_all(images_dir).expect("Error when creating image directory");
//                     }
            
//                     let mut file = match File::create(images_dir) {
//                         Err(why) => panic!("couldn't create file: {}", why.to_string()),
//                         Ok(file) => file,
//                     };

//                     match std::io::copy(&mut bytes, &mut file) {
//                         Err(why) => panic!("couldn't write to: {}", why.to_string()),
//                         Ok(_) => println!("successfully wrote file"),
//                     }
//                 }

//                 handle
//                     .emit_all("listen_media_update", "emitted")
//                     .expect("Error when emitting");
//             }
//         }
//     }
// });

