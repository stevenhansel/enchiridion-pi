use std::fs;
use tauri::{api::path::resource_dir, Env};

use crate::{api::EnchiridionApi, device::Device};

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

#[tauri::command]
pub fn get_device_information(handle: tauri::AppHandle) -> Result<Device, String> {
    let resource_dir = resource_dir(handle.package_info(), &Env::default()).unwrap();

    match Device::load(resource_dir) {
        Ok(info) => Ok(info),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn authenticate(
    handle: tauri::AppHandle,
    access_key_id: String,
    secret_access_key: String,
) -> Result<Device, String> {
    let directory = resource_dir(handle.package_info(), &Env::default()).unwrap();
    let api = EnchiridionApi::new(directory.clone());

    let device = match api
        .authenticate(access_key_id.clone(), secret_access_key.clone())
        .await
    {
        Ok(device) => device,
        Err(_) => {
            return Err("Authentication failed".into());
        }
    };

    if let Err(_) = api.save_auth_keys(access_key_id.clone(), secret_access_key.clone()) {
        return Err("Something when wrong when authenticating".into());
    };

    let device = Device {
        id: device.id,
        name: device.name,
        location: device.location,
        floor_id: device.floor_id,
        building_id: device.building_id,
        description: device.description,
        created_at: device.created_at,
        updated_at: device.updated_at,
    };

    if let Err(_) = Device::save(directory, device.clone()) {
        return Err("Something when wrong when saving the device".into());
    }

    Ok(device)
}
