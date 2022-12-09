use std::sync::Arc;

use thiserror::Error;

use crate::{
    api::{ApiError, EnchiridionApi},
    domain::Device,
    repositories::{DeviceRepository, InsertDeviceParams},
};

#[derive(Error, Debug)]
pub enum LinkDeviceError {
    #[error("An error occurred with the request to the api")]
    ApiError(#[from] ApiError),
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum UnlinkDeviceError {
    #[error("An error occurred with the request")]
    ApiError(#[from] ApiError),
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum GetDeviceError {
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum UpdateCameraEnabledError {
    #[error("An error occurred with the request to the api")]
    ApiError(#[from] ApiError),
    #[error("An error occurred with the request to the database")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct DeviceService {
    _device_repository: Arc<DeviceRepository>,
    _enchiridion_api: Arc<EnchiridionApi>,
}

impl DeviceService {
    pub fn new(
        _device_repository: Arc<DeviceRepository>,
        _enchiridion_api: Arc<EnchiridionApi>,
    ) -> Self {
        DeviceService {
            _device_repository,
            _enchiridion_api,
        }
    }

    pub async fn link(
        &self,
        access_key_id: String,
        secret_access_key: String,
        camera_enabled: bool,
    ) -> Result<Device, LinkDeviceError> {
        self._enchiridion_api
            .link(
                access_key_id.clone(),
                secret_access_key.clone(),
                camera_enabled,
            )
            .await?;

        let device = self
            ._enchiridion_api
            .me_with_keys(access_key_id.clone(), secret_access_key.clone())
            .await?;

        let id = self
            ._device_repository
            .insert(InsertDeviceParams {
                device_id: device.id,
                name: device.name.clone(),
                description: device.description.clone(),
                location: device.location.text.clone(),
                floor_id: device.location.floor.id,
                floor_name: device.location.floor.name.clone(),
                building_id: device.location.building.id,
                building_name: device.location.building.name.clone(),
                building_color: device.location.building.color.clone(),
                camera_enabled: if device.camera_enabled { 1 } else { 0 },
                created_at: device.created_at.clone(),
                updated_at: device.updated_at.clone(),
                access_key_id: access_key_id.clone(),
                secret_access_key: secret_access_key.clone(),
            })
            .await?;

        Ok(Device {
            access_key_id,
            secret_access_key,
            id,
            device_id: device.id,
            name: device.name,
            description: device.description,
            location: device.location.text,
            floor_id: device.location.floor.id,
            floor_name: device.location.floor.name,
            building_id: device.location.building.id,
            building_name: device.location.building.name,
            building_color: device.location.building.color,
            camera_enabled: if device.camera_enabled { 1 } else { 0 },
            created_at: device.created_at,
            updated_at: device.updated_at,
        })
    }

    pub async fn unlink(&self) -> Result<(), UnlinkDeviceError> {
        self._enchiridion_api.unlink().await?;
        self._device_repository.delete().await?;

        Ok(())
    }

    pub async fn get_device(&self) -> Result<Device, GetDeviceError> {
        Ok(self._device_repository.find().await?)
    }

    pub async fn update_camera_enabled(
        &self,
        camera_enabled: bool,
    ) -> Result<(), UpdateCameraEnabledError> {
        let device = self._device_repository.find().await?;
        let device_camera_enabled = if device.camera_enabled == 1 {
            true
        } else {
            false
        };

        if camera_enabled != device_camera_enabled {
            self._enchiridion_api
                .update_camera_enabled(camera_enabled)
                .await?;

            self._device_repository
                .update_camera_enabled(device.id, camera_enabled)
                .await?;
        }

        Ok(())
    }
}
