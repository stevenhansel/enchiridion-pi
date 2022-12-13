use std::time::Duration;

use redis::cmd;
use tokio::time::sleep;

const DEVICE_STATUS_REDIS_KEY: &'static str = "device_status";
const INTERVAL_DURATION: Duration = Duration::from_secs(1);

pub async fn run(redis: deadpool_redis::Pool, device_id: i64) {
    let mut conn = redis.get().await.expect("Cannot get redis connection");

    loop {
        sleep(INTERVAL_DURATION).await;

        if let Err(_) = cmd("HSET")
            .arg(&[
                DEVICE_STATUS_REDIS_KEY,
                device_id.to_string().as_str(),
                chrono::Utc::now().to_rfc3339().as_str(),
            ])
            .query_async::<_, ()>(&mut conn)
            .await
        {
            continue;
        };
    }
}
