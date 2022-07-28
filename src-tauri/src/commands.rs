use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{api::path::resource_dir, Env};

use crate::device::{Device, DeviceInformation};

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
    println!("start request");
    let response =
        match reqwest::get("https://enchiridion.stevenhansel.com/device/v1/buildings").await {
            Ok(res) => match res.text().await {
                Ok(data) => data,
                Err(e) => return Err(e.to_string()),
            },
            Err(e) => return Err(e.to_string()),
        };

    let data = match serde_json::from_str::<ListBuildingResponse>(response.as_str()) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };

    Ok(data.contents)
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
pub async fn get_floors(building_id: i32) -> Result<Vec<ListFloorContent>, String> {
    let response = match reqwest::get(format!(
        "https://enchiridion.stevenhansel.com/device/v1/floors?buildingId={}",
        building_id
    ))
    .await
    {
        Ok(res) => match res.text().await {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        },
        Err(e) => return Err(e.to_string()),
    };

    let data = match serde_json::from_str::<ListFloorResponse>(response.as_str()) {
        Ok(data) => data.contents,
        Err(e) => return Err(e.to_string()),
    };

    Ok(data)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeviceBody {
    name: String,
    description: String,
    floor_id: i32,
    is_linked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceResponse {
    id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceDetailResponse {
    pub id: i32,
    pub name: String,
    pub location: String,
    // pub active_announcements: i32,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub async fn create_device(handle: tauri::AppHandle, body: CreateDeviceBody) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = match client
        .post("https://enchiridion.stevenhansel.com/device/v1/devices")
        .json(&body)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(e.to_string()),
    };
    let device_id = match serde_json::from_str::<CreateDeviceResponse>(&body) {
        Ok(body) => body.id,
        Err(e) => return Err(e.to_string()),
    };

    let response = match client
        .get(format!(
            "https://enchiridion.stevenhansel.com/device/v1/devices/{}",
            device_id
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err(e.to_string()),
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(e.to_string()),
    };
    let detail = match serde_json::from_str::<GetDeviceDetailResponse>(&body) {
        Ok(body) => body,
        Err(e) => return Err(e.to_string()),
    };

    let resource_dir = resource_dir(handle.package_info(), &Env::default()).unwrap();
    let device = Device::new(resource_dir);
    if let Err(e) = device.save(DeviceInformation {
        id: detail.id,
        name: detail.name,
        description: detail.description,
        location: detail.location,
    }) {
        return Err(e.to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn get_device_information(handle: tauri::AppHandle) -> Result<DeviceInformation, String> {
    let resource_dir = resource_dir(handle.package_info(), &Env::default()).unwrap();
    let device = Device::new(resource_dir);
    let device_information = match device.load() {
        Ok(info) => info,
        Err(e) => return Err(e.to_string()),
    };

    Ok(device_information)
}
