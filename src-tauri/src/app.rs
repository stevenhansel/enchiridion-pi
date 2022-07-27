use serde::{Deserialize, Serialize};

use crate::commands;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnnouncementSyncAction {
    Create,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncCreateAnnouncementActionParams {
    action: AnnouncementSyncAction,
    announcement_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAnnouncementMediaPresignedURLResponse {
    media: String,
}

pub fn run() {
    tauri::Builder::default()
        // .setup(|app| {
        //     // let handle = app.handle();
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![commands::get_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
