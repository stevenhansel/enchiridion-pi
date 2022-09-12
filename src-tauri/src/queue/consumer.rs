use redis::{
    streams::{StreamKey, StreamReadOptions, StreamReadReply},
    RedisResult, AsyncCommands,
};

pub enum RedisErrorCode {
    StreamGroupAlreadyExists,
}

impl std::fmt::Display for RedisErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RedisErrorCode::StreamGroupAlreadyExists => write!(f, "BUSYGROUP"),
        }
    }
}

#[derive(Debug)]
pub enum ConsumerError {
    RedisError(String),
}

pub struct Consumer {
    client: deadpool_redis::Pool,
    queue_name: String,
    group_name: String,
    consumer_name: String,

    is_group_exist: bool,
}

impl Consumer {
    pub fn new(
        client: deadpool_redis::Pool,
        queue_name: String,
        consumer_name: String,
    ) -> Consumer {
        Consumer {
            client,
            queue_name,
            group_name: consumer_name.clone(),
            consumer_name: consumer_name.clone(),

            is_group_exist: false,
        }
    }

    pub async fn consume(&mut self) -> Result<Vec<StreamKey>, ConsumerError> {
        let mut redis = self
            .client
            .get()
            .await
            .expect("Cannot get redis connection");

        if !self.is_group_exist {
            if let Err(e) = redis.xgroup_create_mkstream::<String, String, String, ()>(
                self.queue_name.clone(),
                self.group_name.clone(),
                "$".into(),
            ).await {
                let code = match e.code() {
                    Some(code) => code,
                    None => return Err(ConsumerError::RedisError(e.to_string())),
                };

                if code != RedisErrorCode::StreamGroupAlreadyExists.to_string() {
                    return Err(ConsumerError::RedisError(e.to_string()));
                }

                self.is_group_exist = true;
            };
        }

        let opts = StreamReadOptions::default()
            .group(self.group_name.clone(), self.consumer_name.clone())
            .block(0)
            .count(1);
        let results: RedisResult<StreamReadReply> =
            redis.xread_options(&[self.queue_name.clone()], &[">"], &opts).await;

        match results {
            Ok(r) => Ok(r.keys),
            Err(e) => Err(ConsumerError::RedisError(e.to_string())),
        }
    }

    pub async fn ack(&self, message_id: String) -> Result<(), ConsumerError> {
        let mut redis = self
            .client
            .get()
            .await
            .expect("Cannot get redis connection");

        if let Err(e) = redis.xack::<String, String, String, ()>(
            self.queue_name.clone(),
            self.group_name.clone(),
            &[message_id],
        ).await {
            return Err(ConsumerError::RedisError(e.to_string()));
        }

        Ok(())
    }
}
