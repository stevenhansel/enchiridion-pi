use std::{
    fs::{self, File},
    io::Cursor,
    time::Duration,
};

use redis::{streams::StreamKey, Value};
use serde::{Deserialize, Serialize};
use tauri::{api::path::resource_dir, AppHandle, Env, Manager};
use tokio::time::sleep;

use crate::{api::EnchiridionApi, device::Device, events::ApplicationEvent, queue::Consumer};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnnouncementSyncAction {
    Create,
    Delete,
    Resync,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnnouncementConsumerPayload {
    action: AnnouncementSyncAction,
    announcement_id: Option<i32>,
    announcement_ids: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaPresignedURLResponse {
    filename: String,
    media: String,
}

pub struct AnnouncementConsumer {
    _redis: deadpool_redis::Pool,
    _api: EnchiridionApi,
    _handle: AppHandle,
}

impl AnnouncementConsumer {
    pub fn new(_redis: deadpool_redis::Pool, _api: EnchiridionApi, _handle: AppHandle) -> Self {
        AnnouncementConsumer {
            _redis,
            _api,
            _handle,
        }
    }

    pub fn queue_name_builder(&self, device_id: i32) -> String {
        format!("device-queue-{}", device_id)
    }

    async fn get_announcement_media(&self, announcement_id: i32) -> Result<(), String> {
        let presigned = match self._api.get_announcement_media(announcement_id).await {
            Ok(presigned) => presigned,
            Err(_) => {
                return Err(format!(
                    "Something went wrong when getting the announcement media",
                ))
            }
        };

        let image = match reqwest::get(presigned.media).await {
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
        let announcement_id = match payload.announcement_id {
            Some(id) => id,
            None => return Err("Unable to process action, announcement_id is null".into()),
        };
        match self.get_announcement_media(announcement_id).await {
            Ok(()) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn process_action_type_delete(
        &self,
        payload: AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        Ok(())
    }

    pub async fn process_action_type_resync(
        &self,
        payload: AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        Ok(())
    }

    pub async fn consume(&self) {
        loop {
            let device_information = match Device::load(
                resource_dir(self._handle.package_info(), &Env::default()).unwrap(),
            ) {
                Ok(info) => info,
                Err(_) => {
                    sleep(Duration::from_millis(250)).await;
                    continue;
                }
            };

            println!("id: {}", self.queue_name_builder(device_information.id));
            let mut consumer = Consumer::new(
                self._redis.clone(),
                self.queue_name_builder(device_information.id),
                device_information.id.to_string(),
            );

            let data = match consumer.consume().await {
                Ok(res) => res,
                Err(_) => {
                    log::warn!("An error occurred while consuming data");
                    continue;
                }
            };

            let payload = match self.parse_announcement_consumer_data(data) {
                Ok(payload) => payload,
                Err(e) => {
                    log::warn!("Unable to parse the announcement consumer data: {}", e);
                    continue;
                }
            };

            log::info!("Start consuming announcement data");

            match payload.action {
                AnnouncementSyncAction::Create => {
                    if let Err(e) = self.process_action_type_create(payload).await {
                        log::warn!(
                            "An error occurred while processing announcement creation: {}",
                            e
                        );
                        continue;
                    };
                }
                AnnouncementSyncAction::Delete => {
                    if let Err(e) = self.process_action_type_delete(payload).await {
                        log::warn!(
                            "An error occurred while processing announcement creation: {}",
                            e
                        );
                        continue;
                    }
                }
                AnnouncementSyncAction::Resync => {
                    if let Err(e) = self.process_action_type_resync(payload).await {
                        log::warn!(
                            "An error occurred while processing announcement creation: {}",
                            e
                        );
                        continue;
                    }
                }
            };

            log::info!("Finished consuming announcement data");

            self._handle
                .emit_all(ApplicationEvent::MediaUpdate.tag(), "emitted")
                .expect("Error when emitting");
        }
    }
}
