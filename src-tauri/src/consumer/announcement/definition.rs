use serde::{Deserialize, Serialize};

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
    pub action: AnnouncementSyncAction,
    pub announcement_id: Option<i64>,
    pub announcement_ids: Option<Vec<i64>>,
    pub media_type: Option<String>,
    pub media_duration: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaPresignedURLResponse {
    pub filename: String,
    pub media: String,
}
