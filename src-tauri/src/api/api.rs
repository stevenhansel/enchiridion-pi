use reqwest::header::{HeaderMap, HeaderValue};

use super::definition::{GetAnnouncementMediaResponse, MeResponse};

static BASE_URL: &str = "https://enchiridion.stevenhansel.com/device";

pub enum ApiError {
    NotAuthenticated(&'static str),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::ReqwestError(err)
    }
}

pub struct EnchiridionApi {
    client: reqwest::Client,

    access_key_id: Option<String>,
    secret_access_key: Option<String>,
}

impl EnchiridionApi {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

        EnchiridionApi {
            client,
            access_key_id: None,
            secret_access_key: None,
        }
    }

    pub fn set_auth_keys(&mut self, access_key_id: String, secret_access_key: String) {
        self.access_key_id = Some(access_key_id);
        self.secret_access_key = Some(secret_access_key);
    }

    pub fn get_auth_headers(&self) -> Result<HeaderMap, ApiError> {
        if self.access_key_id == None || self.secret_access_key == None {
            return Err(ApiError::NotAuthenticated(
                "Not authenticated, missing authentication keys",
            ));
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            "access-key-id",
            HeaderValue::from_str(&self.access_key_id.unwrap()).unwrap(),
        );
        headers.insert(
            "secret-access-key",
            HeaderValue::from_str(&self.secret_access_key.unwrap()).unwrap(),
        );

        Ok(headers)
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
