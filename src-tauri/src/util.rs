use std::path::PathBuf;

use tauri::api::path::data_dir;

static BASE_FOLDER: &str = "enchiridion";

pub fn get_data_directory() -> PathBuf {
    let dir = data_dir().unwrap().join(BASE_FOLDER);
    if !dir.exists() {
        std::fs::create_dir_all(dir.clone()).expect("Error when creating data directory");
    }

    dir
}
