use std::{fs::{self, File}, path::PathBuf};

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

pub struct Device {
    directory: PathBuf,
    data: Option<DeviceInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInformation {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub floor_id: i32,
}

impl Device {
    pub fn new(resource_dir: PathBuf) -> Self {
        Device {
            directory: resource_dir,
            data: None,
        }
    }

    pub fn load(&self) -> Result<DeviceInformation, DeviceError> {
        let file_path = self.directory.join(DEVICE_INFORMATION_FILENAME);
        if !file_path.exists() {
            return Err(DeviceError::DeviceNotCreated);
        }

        let raw = match fs::read_to_string(file_path) {
            Ok(data) => data,
            Err(_) => return Err(DeviceError::ApplicationError),
        };

        match serde_json::from_str::<DeviceInformation>(raw.as_str()) {
            Ok(data) => Ok(data),
            Err(_) => Err(DeviceError::ApplicationError),
        }
    }

    pub fn save(&self, data: DeviceInformation) -> Result<(), DeviceError> {
        let file_path = self.directory.join(DEVICE_INFORMATION_FILENAME);
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
            Ok(_) => Ok(()),
            Err(_) => Err(DeviceError::ApplicationError),
        }
    }
}
