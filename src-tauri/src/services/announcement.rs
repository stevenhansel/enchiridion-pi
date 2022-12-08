use std::{
    fs::{self, File},
    io::Cursor,
    path::PathBuf,
    sync::Arc,
};
use thiserror::Error;

use crate::{
    api::{ApiError, EnchiridionApi},
    consumer::announcement::definition::AnnouncementConsumerPayload,
    domain::Announcement,
    repositories::{AnnouncementRepository, InsertAnnouncementParams},
};

#[derive(Error, Debug)]
pub enum FindAllAnnouncementError {
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum CreateAnnouncementError {
    #[error("Invalid payload for the corresponding action type")]
    InvalidPayload(&'static str),
    #[error("An error occurred with the request to the api")]
    ApiError(#[from] ApiError),
    #[error("An error occurred when sending a http request")]
    ReqwestError(#[from] reqwest::Error),
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An unknown error has occurred")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum DeleteAnnouncementError {
    #[error("Invalid payload for the corresponding action type")]
    InvalidPayload(&'static str),
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An unknown error has occurred")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ResyncAnnouncementError {}

pub struct AnnouncementService {
    _images_dir: PathBuf,
    _announcement_repository: Arc<AnnouncementRepository>,
    _enchiridion_api: Arc<EnchiridionApi>,
}

impl AnnouncementService {
    pub fn new(
        _local_data_dir: PathBuf,
        _announcement_repository: Arc<AnnouncementRepository>,
        _enchiridion_api: Arc<EnchiridionApi>,
    ) -> Self {
        AnnouncementService {
            _images_dir: _local_data_dir.join("images"),
            _announcement_repository,
            _enchiridion_api,
        }
    }

    pub async fn find_all(&self) -> Result<Vec<Announcement>, FindAllAnnouncementError> {
        Ok(self._announcement_repository.find_all().await?)
    }

    pub async fn create(
        &self,
        payload: &AnnouncementConsumerPayload,
    ) -> Result<(), CreateAnnouncementError> {
        let announcement_id = match payload.announcement_id {
            Some(id) => id,
            None => {
                return Err(CreateAnnouncementError::InvalidPayload(
                    "Announcement Id should not be null",
                ))
            }
        };

        let presigned = self
            ._enchiridion_api
            .get_announcement_media(announcement_id)
            .await?;

        let image = reqwest::get(presigned.media).await?;
        let mut image_bytes = Cursor::new(image.bytes().await?);

        if !self._images_dir.exists() {
            if let Err(_) = fs::create_dir_all(self._images_dir.clone()) {
                return Err(CreateAnnouncementError::Unknown);
            }
        }

        let filetype: Vec<&str> = presigned.filename.split(".").collect();
        let filetype = filetype[filetype.len() - 1];

        let filename = format!("{}.{}", announcement_id, filetype);
        let file_path = self._images_dir.clone().join(filename);

        let mut file = match File::create(file_path.clone()) {
            Ok(file) => file,
            Err(_) => {
                return Err(CreateAnnouncementError::Unknown);
            }
        };

        if let Err(_) = std::io::copy(&mut image_bytes, &mut file) {
            return Err(CreateAnnouncementError::Unknown);
        }

        let local_path = file_path.to_str().unwrap().to_string();

        self._announcement_repository
            .insert(InsertAnnouncementParams {
                announcement_id,
                local_path,
            })
            .await?;

        Ok(())
    }

    pub async fn delete(
        &self,
        payload: &AnnouncementConsumerPayload,
    ) -> Result<(), DeleteAnnouncementError> {
        let announcement_id = match payload.announcement_id {
            Some(id) => id,
            None => {
                return Err(DeleteAnnouncementError::InvalidPayload(
                    "Announcement Id should not be null",
                ))
            }
        };

        let files: Vec<String> = fs::read_dir(self._images_dir.clone())
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
            let path = self._images_dir.join(filename);
            if let Err(_) = fs::remove_file(path) {
                return Err(DeleteAnnouncementError::Unknown);
            };
        }

        self._announcement_repository
            .delete(announcement_id)
            .await?;

        Ok(())
    }

    pub async fn resync(
        &self,
        _payload: &AnnouncementConsumerPayload,
    ) -> Result<(), ResyncAnnouncementError> {
        Ok(())
    }
}
