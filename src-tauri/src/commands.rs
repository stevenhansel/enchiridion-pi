use std::fs;
use serde::{Serialize, Deserialize};
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBuildingResponse {
    contents: Vec<ListBuildingContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBuildingContent {
    id: i32,
    name: String,
    color: String,
}

#[tauri::command]
pub async fn get_buildings() -> Result<Vec<ListBuildingContent>, String> {
    let response = match reqwest::get("https://enchiridion.stevenhansel.com/device/v1/buildings").await {
        Ok(res) => match res.text().await {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        }
        Err(e) => return Err(e.to_string()),
    };

    let data = match serde_json::from_str::<ListBuildingResponse>(response.as_str()) {
        Ok(data) => data.contents,
        Err(e) => return Err(e.to_string()),
    };

    Ok(data)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFloorResponse {
    count: i32,
    total_pages: i32,
    has_next: bool,
    contents: Vec<ListFloorContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFloorContent {
    id: i32,
    name: String,
    building: ListFloorBuildingContent,
    devices: Vec<ListFloorDeviceContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFloorBuildingContent {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFloorDeviceContent {
    pub id: i32,
    pub name: String,
    pub description: String,
    // pub total_announcements: i32,
}

#[tauri::command]
pub async fn get_floors() -> Result<Vec<ListFloorContent>, String> {
    let response = match reqwest::get("https://enchiridion.stevenhansel.com/device/v1/floors").await {
        Ok(res) => match res.text().await {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        }
        Err(e) => return Err(e.to_string()),
    };

    let data = match serde_json::from_str::<ListFloorResponse>(response.as_str()) {
        Ok(data) => data.contents,
        Err(e) => return Err(e.to_string()),
    };

    Ok(data)
}
