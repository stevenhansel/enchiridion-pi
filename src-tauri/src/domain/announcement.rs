use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Announcement {
    pub id: i64,
    // ID that is stored in the API
    pub announcement_id: i64,
    pub local_path: String,
}
