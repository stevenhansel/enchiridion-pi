use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub bundle_identifier: &'static str,
    pub enchiridion_api_base_url: &'static str,
    pub redis_addr: &'static str,
    pub srs_ip: &'static str,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            bundle_identifier: "com.enchiridion.app",
            enchiridion_api_base_url: "http://api.beesmart.binus.local/device",
            // enchiridion_api_base_url: "http://localhost:8080/device",
            redis_addr: "redis://:ac9772178d656aeb6533b2f05c164bade00b58c10fe30586642a319ce3431cee@10.240.200.229:6379",
            // redis_addr: "redis://:15f699f37f0c30f2ec051cf2ea72f66055d525d5c6d44cea467e5ff0618fb2f3@localhost:6379",
            srs_ip: "18.143.23.68",
        }
    }
}
