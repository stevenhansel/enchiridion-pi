use std::fs;
use tauri::{api::path::resource_dir, Env};

#[tauri::command]
pub fn get_images(handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let resource_dir = resource_dir(handle.package_info(), &Env::default()).unwrap();
    let images_dir = &resource_dir.join("images");

    if !images_dir.exists() {
        fs::create_dir_all(images_dir).expect("Error when creating image directory");
    }

    let images = fs::read_dir(images_dir).expect("Error when reading directory");
    let res = images
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|e| e.path().to_str().map(|s| ["asset:///", s].concat()))
        })
        .collect::<Vec<String>>();

    return Ok(res);
}
