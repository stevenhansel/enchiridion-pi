use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub enchiridion_api_base_url: &'static str,
    pub redis_addr: &'static str,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            enchiridion_api_base_url: "https://api.beesmart.stevenhansel.com/device",
            redis_addr: "redis://:ac9772178d656aeb6533b2f05c164bade00b58c10fe30586642a319ce3431cee@18.143.23.68:6379",
        }
    }
}
