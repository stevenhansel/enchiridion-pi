use std::{
    fs::{self, File},
    sync::{Arc, Mutex}, io::Cursor,
};

use redis::{streams::StreamKey, Value};
use serde::{Deserialize, Serialize};
use tauri::{api::path::resource_dir, AppHandle, Env, Manager};

use crate::{device::Device, queue::Consumer};

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
    filename: String,
    media: String,
}

pub struct AnnouncementConsumer {
    _redis: Arc<Mutex<redis::Connection>>,
    _handle: AppHandle,
}

impl AnnouncementConsumer {
    pub fn new(_redis: Arc<Mutex<redis::Connection>>, _handle: AppHandle) -> Self {
        AnnouncementConsumer { _redis, _handle }
    }

    pub fn queue_name_builder(&self, device_id: i32) -> String {
        format!("device-queue-{}", device_id)
    }

    async fn get_announcement_media(&self, announcement_id: i32) -> Result<(), String> {
        let client = reqwest::Client::new();
        let response_data = match client
            .get(format!(
                "https://enchiridion.stevenhansel.com/device/v1/announcements/{}/media",
                announcement_id
            ))
            .send()
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

        let presigned = match serde_json::from_str::<GetAnnouncementMediaPresignedURLResponse>(
            response_data.as_str(),
        ) {
            Ok(data) => data,
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };
        let image = match client.get(presigned.media).send().await {
            Ok(res) => res,
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };
        let mut image_bytes = match image.bytes().await {
            Ok(bytes) => Cursor::new(bytes),
            Err(e) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media: {}",
                    e.to_string(),
                ))
            }
        };

        let resource_dir = resource_dir(self._handle.package_info(), &Env::default()).unwrap();
        let images_dir = resource_dir.join("images");
        if !images_dir.exists() {
            if let Err(e) = fs::create_dir_all(images_dir.clone()) {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ));
            }
        }

        let mut file = match File::create(images_dir.clone().join(presigned.filename)) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ))
            }
        };

        match std::io::copy(&mut image_bytes, &mut file) {
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
        match self.get_announcement_media(payload.announcement_id).await {
            Ok(()) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn consume(&self) {
        loop {
            let resource_dir = resource_dir(self._handle.package_info(), &Env::default()).unwrap();
            let device = Device::new(resource_dir);
            let device_information = match device.load() {
                Ok(info) => info,
                Err(_) => {
                    println!("device information failed");
                    continue
                },
            };

            let mut consumer = Consumer::new(
                self._redis.clone(),
                self.queue_name_builder(device_information.id),
                device_information.id.to_string(),
            );

            let data = match consumer.consume() {
                Ok(res) => res,
                Err(_) => {
                    println!("consume failed");
                    continue;
                }
            };

            let payload = match self.parse_announcement_consumer_data(data) {
                Ok(payload) => payload,
                Err(e) => {
                    println!("e: {}", e.to_string());
                    continue;
                }
            };

            if payload.action == AnnouncementSyncAction::Create {
                if let Err(e) = self.process_action_type_create(payload).await {
                    println!("e: {}", e.to_string());
                    continue;
                };
            }

            println!("success");

            self._handle
                .emit_all("listen_media_update", "emitted")
                .expect("Error when emitting");
        }
    }
}
