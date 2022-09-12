use std::{
    fs::{self, File},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const DEVICE_INFORMATION_FILENAME: &str = "device.json";

#[derive(Debug)]
pub enum DeviceError {
    DeviceNotCreated,
    ApplicationError,
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceError::DeviceNotCreated => write!(f, "Device is not created or linked yet"),
            DeviceError::ApplicationError => {
                write!(f, "Unexpected error happened inside the application")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub floor_id: i32,
    pub building_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl Device {
    pub fn load(directory: PathBuf) -> Result<Self, DeviceError> {
        let file_path = directory.join(DEVICE_INFORMATION_FILENAME);
        if !file_path.exists() {
            return Err(DeviceError::DeviceNotCreated);
        }

        let raw = match fs::read_to_string(file_path) {
            Ok(data) => data,
            Err(_) => return Err(DeviceError::ApplicationError),
        };

        match serde_json::from_str::<Device>(raw.as_str()) {
            Ok(data) => Ok(data),
            Err(_) => Err(DeviceError::ApplicationError),
        }
    }

    pub fn save(directory: PathBuf, data: Device) -> Result<Self, DeviceError> {
        let file_path = directory.join(DEVICE_INFORMATION_FILENAME);
        if !file_path.exists() {
            if let Err(_) = File::create(file_path.clone()) {
                return Err(DeviceError::ApplicationError);
            };
        }

        let deserialized_data = match serde_json::to_string(&data) {
            Ok(raw) => raw,
            Err(_) => return Err(DeviceError::ApplicationError),
        };

        match fs::write(file_path, deserialized_data) {
            Ok(_) => Ok(data),
            Err(_) => Err(DeviceError::ApplicationError),
        }
    }
}
