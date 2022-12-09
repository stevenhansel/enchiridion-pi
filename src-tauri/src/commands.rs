use std::{path::PathBuf, str::FromStr, sync::Arc};

use online::check;
use serde::Serialize;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::{
    api::process::{Command, CommandEvent},
    async_runtime, State,
};

use crate::{
    api::{ApiError, EnchiridionApi},
    consumer,
    domain::{Announcement, Device},
    repositories::{AnnouncementRepository, DeviceRepository},
    services::{AnnouncementService, DeviceService, LinkDeviceError, UnlinkDeviceError},
    settings::Settings,
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

#[tauri::command]
pub async fn get_announcements(
    announcement_service: State<'_, Arc<AnnouncementService>>,
) -> Result<Vec<Announcement>, CommandError> {
    match announcement_service.find_all().await {
        Ok(announcements) => Ok(announcements),
        Err(_) => {
            return Err(CommandError::application_error(
                "Unable to fetch announcements from the local database".to_string(),
            ))
        }
    }
}

#[tauri::command]
pub async fn get_device_information(
    device_service: State<'_, Arc<DeviceService>>,
) -> Result<Device, CommandError> {
    match device_service.get_device().await {
        Ok(device) => Ok(device),
        Err(e) => return Err(CommandError::new(e.to_string(), vec![])),
    }
}

#[tauri::command]
pub async fn link(
    device_service: State<'_, Arc<DeviceService>>,
    access_key_id: String,
    secret_access_key: String,
    camera_enabled: bool,
) -> Result<Device, CommandError> {
    match device_service
        .link(access_key_id, secret_access_key, camera_enabled)
        .await
    {
        Ok(device) => Ok(device),
        Err(e) => match e {
            LinkDeviceError::ApiError(api_error) => match api_error {
                ApiError::ClientError(client_error) => {
                    return Err(CommandError::new(
                        client_error.error_code,
                        client_error.messages,
                    ))
                }
                _ => return Err(CommandError::new(api_error.to_string(), vec![])),
            },
            _ => return Err(CommandError::new(e.to_string(), vec![])),
        },
    }
}

#[tauri::command]
pub async fn unlink(device_service: State<'_, Arc<DeviceService>>) -> Result<(), CommandError> {
    if let Err(e) = device_service.unlink().await {
        match e {
            UnlinkDeviceError::ApiError(api_error) => match api_error {
                ApiError::ClientError(client_error) => {
                    return Err(CommandError::new(
                        client_error.error_code,
                        client_error.messages,
                    ))
                }
                _ => return Err(CommandError::new(api_error.to_string(), vec![])),
            },
            _ => return Err(CommandError::new(e.to_string(), vec![])),
        }
    }

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
pub async fn is_camera_enabled(device_service: State<'_, Arc<DeviceService>>) -> Result<bool, ()> {
    let stdout = match Command::new("printenv").output() {
        Ok(output) => output.stdout,
        Err(_) => return Ok(false),
    };

    let raw_envs = stdout
        .split('\n')
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    let mut envs: Vec<(String, String)> = Vec::new();
    for raw in raw_envs {
        let splitted: Vec<String> = raw.split("=").map(|str| str.to_string()).collect();
        if splitted.len() < 2 {
            continue;
        }

        envs.push((splitted[0].clone(), splitted[1].clone()));
    }

    let enabled = match envs.into_iter().find(|env| env.0 == "CAMERA") {
        Some(env) => match env.1.parse::<bool>() {
            Ok(enabled) => enabled,
            Err(_) => false,
        },
        None => false,
    };

    if let Err(_) = device_service.update_camera_enabled(enabled).await {
        return Ok(false);
    }

    Ok(enabled)
}

#[tauri::command]
pub async fn spawn_camera(
    settings: State<'_, Settings>,
    device_service: State<'_, Arc<DeviceService>>,
) -> Result<(), CommandError> {
    let device = match device_service.get_device().await {
        Ok(device) => device,
        Err(e) => return Err(CommandError::new(e.to_string(), vec![])),
    };

    let device_id = device.device_id.to_string();
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

#[tauri::command]
pub fn spawn_announcement_consumer(
    handle: tauri::AppHandle,
    settings: State<'_, Settings>,
    app_local_data_dir: State<'_, PathBuf>,
) -> Result<(), String> {
    println!("Announcement consumer is starting");

    let app_local_data_dir_path_buf = app_local_data_dir.to_path_buf();
    let app_local_data_dir = app_local_data_dir.to_str().unwrap().to_string();
    let redis_addr = settings.redis_addr.clone();
    let enchiridion_api_base_url = settings.enchiridion_api_base_url.clone();

    async_runtime::spawn(async move {
        let sqlite_opt = SqliteConnectOptions::from_str(
            format!("sqlite://{}/data.db", app_local_data_dir).as_str(),
        )
        .unwrap()
        .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(sqlite_opt)
            .await
            .unwrap();

        let redis_config = deadpool_redis::Config::from_url(redis_addr);
        let redis_pool = redis_config
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .expect("[error] Failed to open redis connection");

        let device_repository = Arc::new(DeviceRepository::new(pool.clone()));
        let announcement_repository = Arc::new(AnnouncementRepository::new(pool.clone()));

        let enchiridion_api = Arc::new(EnchiridionApi::new(
            enchiridion_api_base_url.to_string(),
            device_repository.clone(),
        ));

        let device_service = Arc::new(DeviceService::new(
            device_repository.clone(),
            enchiridion_api.clone(),
        ));
        let announcement_service = Arc::new(AnnouncementService::new(
            app_local_data_dir_path_buf,
            announcement_repository.clone(),
            enchiridion_api.clone(),
        ));

        let device = device_service.get_device().await.unwrap();

        consumer::announcement::consume(device, handle, redis_pool, announcement_service).await;
    });

    Ok(())
}
