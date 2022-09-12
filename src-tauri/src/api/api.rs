use std::{fs, path};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use super::definition::{GetAnnouncementMediaResponse, MeResponse};

static BASE_URL: &str = "https://enchiridion.stevenhansel.com/device";
static AUTH_KEYS_FILENAME: &str = "auth-keys.json";

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

    fn load_auth_keys(&self) -> Result<AuthKeys, ApiError> {
        let file_path = self.directory.join(AUTH_KEYS_FILENAME);
        if !file_path.exists() {
            return Err(ApiError::NotAuthenticated(""));
        }

        let raw = match fs::read_to_string(file_path) {
            Ok(data) => data,
            Err(_) => return Err(ApiError::ApplicationError),
        };

        match serde_json::from_str::<AuthKeys>(raw.as_str()) {
            Ok(data) => Ok(data),
            Err(_) => Err(ApiError::ApplicationError),
        }
    }

    pub fn save_auth_keys(
        &self,
        access_key_id: String,
        secret_access_key: String,
    ) -> Result<(), ApiError> {
        let file_path = self.directory.join(AUTH_KEYS_FILENAME);
        if !file_path.exists() {
            if let Err(_) = fs::File::create(file_path.clone()) {
                return Err(ApiError::ApplicationError);
            };
        }

        let data = AuthKeys {
            access_key_id,
            secret_access_key,
        };

        let deserialized_data = match serde_json::to_string(&data) {
            Ok(raw) => raw,
            Err(_) => return Err(ApiError::ApplicationError),
        };

        match fs::write(file_path, deserialized_data) {
            Ok(_) => Ok(()),
            Err(_) => Err(ApiError::ApplicationError),
        }
    }

    pub fn get_auth_headers(&self) -> Result<HeaderMap, ApiError> {
        let keys = self.load_auth_keys()?;
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

    pub async fn authenticate(
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

        Ok(self
            .client
            .get(format!("{}/v1/me", BASE_URL))
            .headers(headers)
            .send()
            .await?
            .json::<MeResponse>()
            .await?)
    }

    pub async fn me(&self) -> Result<MeResponse, ApiError> {
        Ok(self
            .client
            .get(format!("{}/v1/me", BASE_URL))
            .headers(self.get_auth_headers()?)
            .send()
            .await?
            .json::<MeResponse>()
            .await?)
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
