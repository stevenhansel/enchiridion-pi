use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeBody {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub camera_enabled: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub id: i64,
    pub access_key_id: String,
    pub name: String,
    pub location: DeviceLocation,
    pub description: String,
    pub active_announcements: i64,
    pub camera_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceLocation {
    pub text: String,
    pub floor: DeviceLocationFloor,
    pub building: DeviceLocationBuilding,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceLocationFloor {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceLocationBuilding {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaResponse {
    pub filename: String,
    pub media: String,
}
