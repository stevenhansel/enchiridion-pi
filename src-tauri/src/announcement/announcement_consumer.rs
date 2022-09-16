use std::{
    fs::{self, File},
    io::Cursor,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::time::sleep;

use crate::{
    api::EnchiridionApi, config::ApplicationConfig, events::ApplicationEvent, queue::Consumer,
    util::get_data_directory,
};

use super::{DeleteAnnouncementMediaError, ResyncAnnouncementsError};

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
    announcement_ids: Option<Vec<i32>>,
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

        let images_dir = get_data_directory().join("images");
        if !images_dir.exists() {
            if let Err(e) = fs::create_dir_all(images_dir.clone()) {
                return Err(format!(
                    "Something when wrong when saving the media: {}",
                    e.to_string()
                ));
            }
        }

        let filetype: Vec<&str> = presigned.filename.split(".").collect();
        let filetype = filetype[filetype.len() - 1];

        let filename = format!("{}.{}", announcement_id, filetype);

        let mut file = match File::create(images_dir.clone().join(filename)) {
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

    async fn delete_announcement_media(
        &self,
        announcement_id: i32,
    ) -> Result<(), DeleteAnnouncementMediaError> {
        let images_dir = get_data_directory().join("images");

        let files: Vec<String> = fs::read_dir(images_dir.clone())
            .unwrap()
            .map(|file| file.unwrap().path().display().to_string())
            .map(|path| {
                let splitted_paths: Vec<&str> = path.split("/").collect();
                splitted_paths[splitted_paths.len() - 1].to_string()
            })
            .collect();

        if let Some(filename) = files
            .into_iter()
            .find(|file| file.contains(&announcement_id.to_string()))
        {
            let path = images_dir.join(filename);
            if let Err(_) = fs::remove_file(path) {
                return Err(DeleteAnnouncementMediaError::ApplicationError);
            };
        }

        Ok(())
    }

    async fn resync_announcements(
        &self,
        announcement_ids: Vec<i32>,
    ) -> Result<(), ResyncAnnouncementsError> {
        for id in &announcement_ids {
            if let Err(_) = self.delete_announcement_media(*id).await {
                return Err(ResyncAnnouncementsError::ApplicationError);
            }
        }

        for id in &announcement_ids {
            if let Err(_) = self.get_announcement_media(*id).await {
                return Err(ResyncAnnouncementsError::ApplicationError);
            }
        }

        Ok(())
    }

    pub async fn process_action_type_create(
        &self,
        payload: &AnnouncementConsumerPayload,
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
        payload: &AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        let announcement_id = match payload.announcement_id {
            Some(id) => id,
            None => return Err("Unable to process action, announcement_id is null".into()),
        };

        match self.delete_announcement_media(announcement_id).await {
            Ok(()) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn process_action_type_resync(
        &self,
        payload: &AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        let announcement_ids = match &payload.announcement_ids {
            Some(ids) => ids,
            None => return Err("Unable to process action, announcement_id is null".into()),
        };

        match self.resync_announcements(announcement_ids.to_vec()).await {
            Ok(()) => Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub async fn process_announcement(
        &self,
        payload: &AnnouncementConsumerPayload,
    ) -> Result<(), String> {
        match payload.action {
            AnnouncementSyncAction::Create => {
                if let Err(e) = self.process_action_type_create(payload).await {
                    return Err(e.to_string());
                };
            }
            AnnouncementSyncAction::Delete => {
                if let Err(e) = self.process_action_type_delete(payload).await {
                    return Err(e.to_string());
                }
            }
            AnnouncementSyncAction::Resync => {
                if let Err(e) = self.process_action_type_resync(payload).await {
                    return Err(e.to_string());
                }
            }
        };

        Ok(())
    }

    pub async fn consume(&self) {
        loop {
            let config = match ApplicationConfig::load(get_data_directory()) {
                Ok(config) => config,
                Err(_) => {
                    sleep(Duration::from_millis(250)).await;
                    continue;
                }
            };
            let device_information = match config.device {
                Some(device) => device,
                None => {
                    sleep(Duration::from_millis(250)).await;
                    continue;
                }
            };

            let mut consumer = Consumer::new(
                self._redis.clone(),
                self.queue_name_builder(device_information.id),
            );

            let pending_message_id = match consumer.get_pending_message_id().await {
                Ok(id) => id,
                Err(e) => {
                    log::warn!("An error occurred while getting pending message: {}", e);
                    continue;
                }
            };

            if let Some(message_id) = pending_message_id {
                let data = match consumer
                    .read_by_message_id::<AnnouncementConsumerPayload>(message_id.to_string())
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        log::warn!("An error occurred while consuming data: {}", e);
                        continue;
                    }
                };

                if data.len() == 0 {
                    if let Err(e) = consumer.ack(message_id.to_string()).await {
                        log::warn!(
                            "An error occurred while acknowledging the announcement: {}",
                            e
                        );
                    }

                    continue;
                }

                let (message_id, payload) = &data[0];
                log::info!(
                    "Start processing announcement with message_id: {}",
                    message_id.to_string()
                );

                if let Err(e) = self.process_announcement(payload).await {
                    log::warn!(
                        "Something when wrong when processing the announcements: {}",
                        e
                    );
                    continue;
                }

                if let Err(e) = consumer.ack(message_id.to_string()).await {
                    log::warn!(
                        "An error occurred while acknowledging the announcement: {}",
                        e
                    );
                }

                log::info!(
                    "Finished processing announcement with message_id: {}",
                    message_id.to_string()
                );
            } else {
                let data = match consumer.consume::<AnnouncementConsumerPayload>().await {
                    Ok(res) => res,
                    Err(e) => {
                        log::warn!("An error occurred while consuming data: {}", e);
                        continue;
                    }
                };
                if data.len() == 0 {
                    continue;
                }

                let (message_id, payload) = &data[0];
                log::info!(
                    "Start processing announcement with message_id: {}",
                    message_id.to_string()
                );

                if let Err(e) = self.process_announcement(payload).await {
                    log::warn!(
                        "Something when wrong when processing the announcements: {}",
                        e
                    );
                    continue;
                }

                if let Err(e) = consumer.ack(message_id.to_string()).await {
                    log::warn!(
                        "An error occurred while acknowledging the announcement: {}",
                        e
                    );
                }

                log::info!(
                    "Finished processing announcement with message_id: {}",
                    message_id.to_string()
                );
            }

            self._handle
                .emit_all(ApplicationEvent::MediaUpdate.tag(), "emitted")
                .expect("Error when emitting");
        }
    }
}
