use std::path;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    StatusCode,
};
use serde::{Deserialize, Serialize};

use super::definition::{GetAnnouncementMediaResponse, MeBody, MeResponse};
use crate::config::{ApplicationConfig, ApplicationConfigError};

static BASE_URL: &str = "https://enchiridion.stevenhansel.com/device";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct APIErrorResponse {
    pub error_code: String,
    pub messages: Vec<String>,
}

pub enum APIResponse<T> {
    Success(T),
    Error(APIErrorResponse),
}

#[derive(Debug)]
pub enum ApiError {
    NotAuthenticated(&'static str),
    ReqwestError(reqwest::Error),
    ApplicationError,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::ReqwestError(err)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(_err: std::io::Error) -> Self {
        ApiError::NotAuthenticated("Failed to load application configuration")
    }
}

impl From<ApplicationConfigError> for ApiError {
    fn from(err: ApplicationConfigError) -> Self {
        match err {
            ApplicationConfigError::ConfigurationUnavailable(message)
            | ApplicationConfigError::AuthenticationUnavailable(message) => {
                ApiError::NotAuthenticated(message)
            }
            _ => ApiError::ApplicationError,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthKeys {
    pub access_key_id: String,
    pub secret_access_key: String,
}

pub struct EnchiridionApi {
    client: reqwest::Client,
    directory: path::PathBuf,
}

impl EnchiridionApi {
    pub fn new(directory: path::PathBuf) -> Self {
        let client = reqwest::Client::new();

        EnchiridionApi { client, directory }
    }

    pub fn get_auth_headers(&self) -> Result<HeaderMap, ApiError> {
        let keys = match ApplicationConfig::load(self.directory.clone())?.auth {
            Some(keys) => keys,
            None => return Err(ApiError::NotAuthenticated("Device is not linked")),
        };

        let mut headers = HeaderMap::new();
        headers.insert(
            "access-key-id",
            HeaderValue::from_str(&keys.access_key_id).unwrap(),
        );
        headers.insert(
            "secret-access-key",
            HeaderValue::from_str(&keys.secret_access_key).unwrap(),
        );

        Ok(headers)
    }

    pub async fn link(
        &self,
        access_key_id: String,
        secret_access_key: String,
    ) -> Result<APIResponse<()>, ApiError> {
        let response = self
            .client
            .put(format!("{}/v1/link", BASE_URL))
            .json(&MeBody {
                access_key_id,
                secret_access_key,
            })
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(APIResponse::Success(())),
            _ => Ok(APIResponse::Error(
                response.json::<APIErrorResponse>().await?,
            )),
        }
    }

    pub async fn unlink(&self) -> Result<APIResponse<()>, ApiError> {
        let response = self
            .client
            .put(format!("{}/v1/unlink", BASE_URL))
            .headers(self.get_auth_headers()?)
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(APIResponse::Success(())),
            _ => Ok(APIResponse::Error(
                response.json::<APIErrorResponse>().await?,
            )),
        }
    }

    pub async fn me_with_keys(
        &self,
        access_key_id: String,
        secret_access_key: String,
    ) -> Result<APIResponse<MeResponse>, ApiError> {
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
            .get(format!("{}/v1/me", BASE_URL))
            .headers(headers)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(APIResponse::Success(response.json::<MeResponse>().await?)),
            _ => Ok(APIResponse::Error(
                response.json::<APIErrorResponse>().await?,
            )),
        }
    }

    pub async fn me(&self) -> Result<APIResponse<MeResponse>, ApiError> {
        let response = self
            .client
            .get(format!("{}/v1/me", BASE_URL))
            .headers(self.get_auth_headers()?)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => Ok(APIResponse::Success(response.json::<MeResponse>().await?)),
            _ => Ok(APIResponse::Error(
                response.json::<APIErrorResponse>().await?,
            )),
        }
    }

    pub async fn get_announcement_media(
        &self,
        announcement_id: i32,
    ) -> Result<GetAnnouncementMediaResponse, ApiError> {
        Ok(self
            .client
            .get(format!(
                "{}/v1/announcements/{}/media",
                BASE_URL, announcement_id
            ))
            .headers(self.get_auth_headers()?)
            .send()
            .await?
            .json::<GetAnnouncementMediaResponse>()
            .await?)
    }
}
