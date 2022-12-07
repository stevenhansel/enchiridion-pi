use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

static APPLICATION_CONFIG_FILENAME: &str = "config.json";

#[derive(Debug)]
pub enum ApplicationConfigError {
    ConfigurationUnavailable(&'static str),
    DeviceUnavailable(&'static str),
    AuthenticationUnavailable(&'static str),
    ApplicationError(&'static str),
}

impl ApplicationConfigError {
    pub fn code(&self) -> &'static str {
        match self {
            ApplicationConfigError::ConfigurationUnavailable(_) => "CONFIGURATION_UNAVAILABLE",
            ApplicationConfigError::DeviceUnavailable(_) => "DEVICE_UNAVAILABLE",
            ApplicationConfigError::AuthenticationUnavailable(_) => "AUTHENTICATION_UNAVAILABLE",
            ApplicationConfigError::ApplicationError(_) => "APPLICATION_ERROR",
        }
    }
}

impl std::fmt::Display for ApplicationConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationConfigError::ConfigurationUnavailable(message) => write!(f, "{}", message),
            ApplicationConfigError::DeviceUnavailable(message) => write!(f, "{}", message),
            ApplicationConfigError::AuthenticationUnavailable(message) => write!(f, "{}", message),
            ApplicationConfigError::ApplicationError(message) => write!(f, "{}", message),
        }
    }
}

impl From<std::io::Error> for ApplicationConfigError {
    fn from(_err: std::io::Error) -> Self {
        ApplicationConfigError::ApplicationError("Something when wrong with the application")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConfig {
    #[serde(skip)]
    directory: PathBuf,

    pub auth: Option<AuthenticationKey>,
    pub device: Option<DeviceInformation>,
}

impl ApplicationConfig {
    pub fn new(directory: PathBuf, auth: AuthenticationKey, device: DeviceInformation) -> Self {
        ApplicationConfig {
            directory,
            auth: Some(auth),
            device: Some(device),
        }
    }

    pub fn default(directory: PathBuf) -> Self {
        ApplicationConfig {
            directory,
            auth: None,
            device: None,
        }
    }

    pub fn load(directory: PathBuf) -> Result<Self, ApplicationConfigError> {
        let path = directory.join(APPLICATION_CONFIG_FILENAME);
        let file = match OpenOptions::new().read(true).open(path) {
            Ok(file) => file,
            Err(_) => return Ok(ApplicationConfig::default(directory)),
        };

        let mut config = match serde_json::from_reader::<File, ApplicationConfig>(file) {
            Ok(config) => config,
            Err(_) => {
                return Err(ApplicationConfigError::ApplicationError(
                    "Something when wrong when loading the application configuration",
                ))
            }
        };

        config.directory = directory;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ApplicationConfigError> {
        let path = self.directory.join(APPLICATION_CONFIG_FILENAME);

        if let Err(_) = fs::write(path, serde_json::to_string(self).unwrap()) {
            return Err(ApplicationConfigError::ApplicationError(
                "Something when wrong when writing the file",
            ));
        };

        Ok(())
    }

    pub fn remove(&self) -> Result<(), ApplicationConfigError> {
        if let Err(_) = fs::remove_dir_all(self.directory.clone()) {
            return Err(ApplicationConfigError::ApplicationError(
                "Unable to find the configuration file",
            ));
        }

        Ok(())
    }

    pub fn save_device_information(
        directory: PathBuf,
        device: DeviceInformation,
    ) -> Result<(), ApplicationConfigError> {
        ApplicationConfig::load(directory)?
            .set_device(device)
            .save()?;

        Ok(())
    }

    pub fn set_device(mut self, device: DeviceInformation) -> Self {
        self.device = Some(device);
        self
    }

    pub fn save_authentication_key(
        directory: PathBuf,
        auth: AuthenticationKey,
    ) -> Result<(), ApplicationConfigError> {
        ApplicationConfig::load(directory)?
            .set_authentication_key(auth)
            .save()?;

        Ok(())
    }

    pub fn set_authentication_key(mut self, auth: AuthenticationKey) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn get_device(self) -> Result<DeviceInformation, ApplicationConfigError> {
        match self.device {
            Some(device) => Ok(device),
            None => Err(ApplicationConfigError::DeviceUnavailable(
                "Device is not linked",
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInformation {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub floor: DeviceFloorInformation,
    pub building: DeviceBuildingInformation,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFloorInformation {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceBuildingInformation {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationKey {
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl AuthenticationKey {
    pub fn new(access_key_id: String, secret_access_key: String) -> Self {
        AuthenticationKey {
            access_key_id,
            secret_access_key,
        }
    }
}
