use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};

use crate::domain::Device;

pub struct InsertDeviceParams {
    pub device_id: i64,
    pub name: String,
    pub description: String,
    pub location: String,
    pub floor_id: i64,
    pub floor_name: String,
    pub building_id: i64,
    pub building_name: String,
    pub building_color: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub camera_enabled: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub struct DeviceRepository {
    _db: Pool<Sqlite>,
}

impl DeviceRepository {
    pub fn new(_db: Pool<Sqlite>) -> Self {
        DeviceRepository { _db }
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT
                COUNT(*) AS "count"
            FROM "device"
            "#,
        )
        .map(|row: SqliteRow| row.try_get("count").unwrap())
        .fetch_one(&self._db)
        .await?;

        Ok(result)
    }

    pub async fn insert(&self, params: InsertDeviceParams) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO "device"
            ("device_id", "name", "description", "location", "floor_id", "floor_name", "building_id", "building_name", "building_color", "access_key_id", "secret_access_key", "camera_enabled", "created_at", "updated_at")
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
             "#,
        )
        .bind(params.device_id)
        .bind(params.name)
        .bind(params.description)
        .bind(params.location)
        .bind(params.floor_id)
        .bind(params.floor_name)
        .bind(params.building_id)
        .bind(params.building_name)
        .bind(params.building_color)
        .bind(params.access_key_id)
        .bind(params.secret_access_key)
        .bind(params.camera_enabled)
        .bind(params.created_at)
        .bind(params.updated_at)
        .execute(&self._db)
        .await?
        .last_insert_rowid();

        Ok(result)
    }

    pub async fn find(&self) -> Result<Device, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT
                "id",
                "device_id",
                "name",
                "description",
                "location",
                "floor_id",
                "floor_name",
                "building_id",
                "building_name",
                "building_color",
                "access_key_id",
                "secret_access_key",
                "camera_enabled",
                "created_at",
                "updated_at"
            FROM "device"
            "#,
        )
        .map(|row: SqliteRow| Device {
            id: row.try_get("id").unwrap(),
            device_id: row.try_get("device_id").unwrap(),
            name: row.try_get("name").unwrap(),
            description: row.try_get("description").unwrap(),
            location: row.try_get("location").unwrap(),
            floor_id: row.try_get("floor_id").unwrap(),
            floor_name: row.try_get("floor_name").unwrap(),
            building_id: row.try_get("building_id").unwrap(),
            building_name: row.try_get("building_name").unwrap(),
            building_color: row.try_get("building_color").unwrap(),
            access_key_id: row.try_get("access_key_id").unwrap(),
            secret_access_key: row.try_get("secret_access_key").unwrap(),
            camera_enabled: row.try_get("camera_enabled").unwrap(),
            created_at: row.try_get("created_at").unwrap(),
            updated_at: row.try_get("updated_at").unwrap(),
        })
        .fetch_one(&self._db)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query("DELETE FROM device")
            .execute(&self._db)
            .await?
            .rows_affected();
        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
