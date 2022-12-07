use std::fs;

use online::check;
use serde::Serialize;
use tauri::{async_runtime, api::process::{Command, CommandEvent}, State};

use crate::{
    api::{APIErrorResponse, APIResponse, EnchiridionApi},
    appconfig::{
        ApplicationConfig, ApplicationConfigError, AuthenticationKey, DeviceBuildingInformation,
        DeviceFloorInformation, DeviceInformation,
    },
    settings::Settings,
    util::get_data_directory,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandError {
    error_code: String,
    messages: Vec<String>,
}

impl CommandError {
    pub fn new(error_code: String, messages: Vec<String>) -> Self {
        CommandError {
            error_code,
            messages,
        }
    }

    pub fn application_error(message: String) -> Self {
        CommandError {
            error_code: "APPLICATION_ERROR".into(),
            messages: vec![message],
        }
    }
}

impl From<APIErrorResponse> for CommandError {
    fn from(err: APIErrorResponse) -> Self {
        CommandError {
            error_code: err.error_code,
            messages: err.messages,
        }
    }
}

impl From<ApplicationConfigError> for CommandError {
    fn from(err: ApplicationConfigError) -> Self {
        CommandError {
            error_code: err.code().to_string(),
            messages: vec![err.to_string()],
        }
    }
}

#[tauri::command]
pub fn get_images() -> Result<Vec<String>, CommandError> {
    let images_dir = get_data_directory().join("images");

    if !images_dir.exists() {
        fs::create_dir_all(images_dir.clone()).expect("Error when creating image directory");
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
pub fn get_device_information() -> Result<DeviceInformation, CommandError> {
    let config = match ApplicationConfig::load(get_data_directory()) {
        Ok(config) => config,
        Err(e) => return Err(CommandError::new(e.code().to_string(), vec![e.to_string()])),
    };

    match config.get_device() {
        Ok(device) => Ok(device),
        Err(e) => Err(CommandError::new(e.code().to_string(), vec![e.to_string()])),
    }
}

#[tauri::command]
pub async fn link(
    settings: State<'_, Settings>,
    access_key_id: String,
    secret_access_key: String,
) -> Result<DeviceInformation, CommandError> {
    let api = EnchiridionApi::new(
        get_data_directory(),
        settings.enchiridion_api_base_url.to_string(),
    );

    match api
        .link(access_key_id.clone(), secret_access_key.clone())
        .await
    {
        Ok(response) => {
            if let APIResponse::Error(error) = response {
                return Err(error.into());
            }
        }
        Err(_) => {
            return Err(CommandError::application_error(String::from(
                "Something when wrong when linking the device",
            )))
        }
    };

    let device = match api
        .me_with_keys(access_key_id.clone(), secret_access_key.clone())
        .await
    {
        Ok(response) => match response {
            APIResponse::Success(device) => device,
            APIResponse::Error(error) => return Err(error.into()),
        },
        Err(_) => {
            return Err(CommandError::application_error(String::from(
                "Something when wrong when getting the device information",
            )))
        }
    };

    let device = DeviceInformation {
        id: device.id,
        name: device.name,
        description: device.description,
        location: device.location.text,
        floor: DeviceFloorInformation {
            id: device.location.floor.id,
            name: device.location.floor.name,
        },
        building: DeviceBuildingInformation {
            id: device.location.building.id,
            name: device.location.building.name,
            color: device.location.building.color,
        },
        created_at: device.created_at,
        updated_at: device.updated_at,
    };

    // Watchout saving auth keys and device individually with lead to data race (in one process)
    if let Err(e) = ApplicationConfig::new(
        get_data_directory(),
        AuthenticationKey::new(access_key_id, secret_access_key),
        device.clone(),
    )
    .save()
    {
        return Err(e.into());
    }

    Ok(device)
}

#[tauri::command]
pub async fn unlink(settings: State<'_, Settings>) -> Result<(), CommandError> {
    let api = EnchiridionApi::new(
        get_data_directory(),
        settings.enchiridion_api_base_url.to_string(),
    );

    match api.unlink().await {
        Ok(response) => {
            if let APIResponse::Error(error) = response {
                return Err(error.into());
            }
        }
        Err(_) => {
            return Err(CommandError::application_error(String::from(
                "Something when wrong when unlinking the device",
            )));
        }
    };

    ApplicationConfig::load(get_data_directory())?.remove()?;

    Ok(())
}

#[tauri::command]
pub async fn is_network_connected() -> bool {
    if let Ok(()) = check(None) {
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
pub fn spawn_camera(settings: State<'_, Settings>) -> Result<(), String> {
    let config = ApplicationConfig::load(get_data_directory()).unwrap();
    let device = config.get_device().unwrap();

    let device_id = device.id.to_string();
    let device_id = device_id.as_str();

    let (mut rx, _child) = Command::new_sidecar("camera")
    .expect("failed to create `camera` binary command")
    .args(["-id", device_id, "-ip", settings.srs_ip])
    .spawn()
    .expect("Failed to spawn sidecar");

    async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("{}", line);
            }
        }
    });

    Ok(())
}
