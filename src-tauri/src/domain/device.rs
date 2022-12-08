use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: i64,
    pub device_id: i64,
    pub name: String,
    pub description: String,
    pub location: String,
    pub floor_id: i64,
    pub floor_name: String,
    pub building_id: i64,
    pub building_name: String,
    pub building_color: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub camera_enabled: i64,
    pub created_at: String,
    pub updated_at: String,
}
