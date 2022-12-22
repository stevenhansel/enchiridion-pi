use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};

use crate::domain::Announcement;

pub struct InsertAnnouncementParams {
    pub announcement_id: i64,
    pub local_path: String,
    pub media_type: String,
    pub media_duration: Option<f64>,
}

pub struct AnnouncementRepository {
    _db: Pool<Sqlite>,
}

impl AnnouncementRepository {
    pub fn new(_db: Pool<Sqlite>) -> Self {
        AnnouncementRepository { _db }
    }

    pub async fn insert(&self, params: InsertAnnouncementParams) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO "announcement"
            ("announcement_id", "local_path", "media_type", "media_duration")
            VALUES (?1, ?2, ?3, ?4)
            "#,
        )
        .bind(params.announcement_id)
        .bind(params.local_path)
        .bind(params.media_type)
        .bind(params.media_duration)
        .execute(&self._db)
        .await?
        .last_insert_rowid();

        Ok(result)
    }

    pub async fn delete(&self, announcement_id: i64) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM "announcement"
            WHERE announcement_id = ?1
            "#,
        )
        .bind(announcement_id)
        .execute(&self._db)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Announcement>, sqlx::Error> {
        let result = sqlx::query(
            r#"
            SELECT
                "id",
                "announcement_id",
                "local_path",
                "media_type",
                "media_duration"
            FROM "announcement"
            "#,
        )
        .map(|row: SqliteRow| Announcement {
            id: row.try_get("id").unwrap(),
            announcement_id: row.try_get("announcement_id").unwrap(),
            local_path: row.try_get("local_path").unwrap(),
            media_type: row.try_get("media_type").unwrap(),
            media_duration: row.try_get("media_duration").unwrap(),
        })
        .fetch_all(&self._db)
        .await?;

        Ok(result)
    }

    pub async fn reset(&self) -> Result<(), sqlx::Error> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM "announcement"
            "#,
        )
        .execute(&self._db)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
