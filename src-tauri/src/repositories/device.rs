use sqlx::{Pool, Sqlite};

pub struct DeviceRepository {
    _db: Pool<Sqlite>,
}

impl DeviceRepository {
    pub fn new(_db: Pool<Sqlite>) -> Self {
        DeviceRepository { _db }
    }
}
