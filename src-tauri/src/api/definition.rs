use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub id: i32,
    pub access_key_id: String,
    pub name: String,
    pub location: String,
    pub floor_id: i32,
    pub building_id: i32,
    pub description: String,
    pub active_announcements: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct GetAnnouncementMediaResponse {
    pub filename: String,
    pub media: String,
}
