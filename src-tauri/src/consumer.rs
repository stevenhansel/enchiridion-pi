use std::{
    fs::{self, File},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use redis::{streams::StreamKey, Value};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaPresignedURLResponse {
    media: String,
}

pub struct AnnouncementConsumer {
    _device: DeviceInformation,
    _redis: Arc<Mutex<redis::Connection>>,

    directory: PathBuf,
    handle: AppHandle,
}

impl AnnouncementConsumer {
    pub fn new(
        _device: DeviceInformation,
        _redis: Arc<Mutex<redis::Connection>>,
        directory: PathBuf,
        handle: AppHandle,
    ) -> Self {
        AnnouncementConsumer {
            _device,
            _redis,
            directory,
            handle,
        }
    }

    pub fn queue_name_builder(&self) -> String {
        format!("device-queue-{}", self._device.id)
    }

    async fn get_announcement_media(&self, announcement_id: i32) -> Result<String, String> {
        let response_data = match reqwest::get(format!(
            "https://enchiridion.stevenhansel.com/device/v1/announcements/{}/media",
            announcement_id
        ))
        .await
        {
            Ok(res) => match res.text().await {
                Ok(data) => data,
                Err(e) => {
                    return Err(format!(
                        "Something went wrong when getting the announcement media: {}",
                        e.to_string(),
                    ))
                }
            },
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };

        let presigned_url = match serde_json::from_str::<GetAnnouncementMediaPresignedURLResponse>(
            response_data.as_str(),
        ) {
            Ok(data) => data.media,
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };
        let image = match reqwest::get(presigned_url).await {
            Ok(res) => res,
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };
        let text = match image.text().await {
            Ok(text) => text,
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };

        Ok(text)
    }

    fn save_media(&self, filename: String, mut bytes: &[u8]) -> Result<(), String> {
        let images_dir = self.directory.join("images");
        if !images_dir.exists() {
            if let Err(e) = fs::create_dir_all(images_dir.clone()) {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ));
            }
        }

        let mut file = match File::create(images_dir.clone().join(filename)) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ))
            }
        };

        match std::io::copy(&mut bytes, &mut file) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ))
            }
        }
    }

    fn parse_announcement_consumer_data(
        &self,
        data: Vec<StreamKey>,
    ) -> Result<AnnouncementConsumerPayload, String> {
        let mut raw_data: Option<String> = None;
        for res in data {
            for id in res.ids {
                if let Some(data) = id.map.get("data") {
                    if let Value::Data(buffer) = data {
                        if let Ok(value) = std::str::from_utf8(buffer) {
                            raw_data = Some(value.to_string());
                            break;
                        }
                    }
                }
            }
        }

        let data = match raw_data {
            Some(data) => data,
            None => return Err("Something went wrong when parsing the consumer data".into()),
        };

        match serde_json::from_str::<AnnouncementConsumerPayload>(&data) {
            Ok(payload) => Ok(payload),
            Err(e) => {
                return Err(format!(
                    "Something went wrong when parsing the consumer data: {}",
                    e.to_string()
                ))
            }
        }
    }

    pub async fn process_action_type_create(
        &self,
        payload: AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        let media = match self.get_announcement_media(payload.announcement_id).await {
            Ok(media) => media,
            Err(e) => return Err(e.to_string()),
        };

        match self
            .save_media(payload.announcement_id.to_string(), media.as_bytes())
        {
            Ok(()) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn consume(&self) {
        let mut consumer = Consumer::new(
            self._redis.clone(),
            self.queue_name_builder(),
            self._device.id.to_string(),
        );

        loop {
            let data = match consumer.consume() {
                Ok(res) => res,
                Err(_) => break,
            };

            let payload = match self.parse_announcement_consumer_data(data) {
                Ok(payload) => payload,
                Err(_) => break,
            };

            if payload.action == AnnouncementSyncAction::Create {
                self.process_action_type_create(payload).await;
            }

            self.handle
                .emit_all("listen_media_update", "emitted")
                .expect("Error when emitting");
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

//         if let Some(data) = raw_data {
//             if let Ok(result) = serde_json::from_str::<SyncCreateAnnouncementActionParams>(&data) {
//                 if result.action == AnnouncementSyncAction::Create {

//
//                 }

//             }
//         }
//     }
// });
