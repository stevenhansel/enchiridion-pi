use std::{io, sync::Arc};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::definition::{
    GetAnnouncementMediaResponse, MeBody, MeResponse, UpdateCameraEnabledBody,
};
use crate::repositories::DeviceRepository;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorObject {
    pub error_code: String,
    pub messages: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Device is not linked yet")]
    NotAuthenticated(&'static str),
    #[error("Request to Enchiridion API failed")]
    ReqwestError(#[from] reqwest::Error),
    #[error("An io error occurred")]
    IoError(#[from] io::Error),
    #[error("An application error occurred")]
    ApplicationError,
    #[error("An error has occurred from the API")]
    ClientError(ErrorObject),
}

pub struct EnchiridionApi {
    base_url: String,
    client: reqwest::Client,

    _device_repository: Arc<DeviceRepository>,
}

impl EnchiridionApi {
    pub fn new(base_url: String, _device_repository: Arc<DeviceRepository>) -> Self {
        let client = reqwest::Client::new();

        EnchiridionApi {
            base_url,
            client,
            _device_repository,
        }
    }

    pub async fn get_auth_headers(&self) -> Result<HeaderMap, ApiError> {
        let device = match self._device_repository.find().await {
            Ok(device) => device,
            Err(_) => return Err(ApiError::NotAuthenticated("This device is not linked yet")),
        };

        let mut headers = HeaderMap::new();
        headers.insert(
            "access-key-id",
            HeaderValue::from_str(&device.access_key_id).unwrap(),
        );
        headers.insert(
            "secret-access-key",
            HeaderValue::from_str(&device.secret_access_key).unwrap(),
        );

        Ok(headers)
    }

    pub async fn link(
        &self,
        access_key_id: String,
        secret_access_key: String,
    ) -> Result<(), ApiError> {
        let response = self
            .client
            .put(format!("{}/v1/link", self.base_url))
            .json(&MeBody {
                access_key_id,
                secret_access_key,
            })
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(ApiError::ClientError(response.json::<ErrorObject>().await?)),
        }
    }

    pub async fn unlink(&self) -> Result<(), ApiError> {
        let response = self
            .client
            .put(format!("{}/v1/unlink", self.base_url))
            .headers(self.get_auth_headers().await?)
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(ApiError::ClientError(response.json::<ErrorObject>().await?)),
        }
    }

    pub async fn me_with_keys(
        &self,
        access_key_id: String,
        secret_access_key: String,
    ) -> Result<MeResponse, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "access-key-id",
            HeaderValue::from_str(&access_key_id).unwrap(),
        );
        headers.insert(
            "secret-access-key",
            HeaderValue::from_str(&secret_access_key).unwrap(),
        );

        let response = self
            .client
            .get(format!("{}/v1/me", self.base_url))
            .headers(headers)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<MeResponse>().await?),
            _ => Err(ApiError::ClientError(response.json::<ErrorObject>().await?)),
        }
    }

    pub async fn me(&self) -> Result<MeResponse, ApiError> {
        let response = self
            .client
            .get(format!("{}/v1/me", self.base_url))
            .headers(self.get_auth_headers().await?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(response.json::<MeResponse>().await?),
            _ => Err(ApiError::ClientError(response.json::<ErrorObject>().await?)),
        }
    }

    pub async fn get_announcement_media(
        &self,
        announcement_id: i64,
    ) -> Result<GetAnnouncementMediaResponse, ApiError> {
        Ok(self
            .client
            .get(format!(
                "{}/v1/announcements/{}/media",
                self.base_url, announcement_id
            ))
            .headers(self.get_auth_headers().await?)
            .send()
            .await?
            .json::<GetAnnouncementMediaResponse>()
            .await?)
    }

    pub async fn update_camera_enabled(&self, camera_enabled: bool) -> Result<(), ApiError> {
        let response = self
            .client
            .put(format!("{}/v1/camera", self.base_url))
            .headers(self.get_auth_headers().await?)
            .json(&UpdateCameraEnabledBody { camera_enabled })
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(ApiError::ClientError(response.json::<ErrorObject>().await?)),
        }
    }
}
