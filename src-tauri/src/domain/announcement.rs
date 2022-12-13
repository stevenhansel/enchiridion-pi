use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Announcement {
    pub id: i64,
    // ID that is stored in the API
    pub announcement_id: i64,
    pub local_path: String,
    pub media_type: String,
    pub media_duration: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct AnnouncementMedia {
    pub filename: String,
    pub media: String,
}
