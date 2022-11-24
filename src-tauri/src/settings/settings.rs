use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub enchiridion_api_base_url: String,
    pub redis_addr: String,
}

impl Settings {
    pub fn new(path: &'static str) -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name(path))
            .build()?;

        builder.try_deserialize()
    }
}
