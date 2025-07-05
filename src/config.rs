
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct WatchmenConfig {
    pub chat_id: String,
    pub bot_token: String,
    pub server_addr: String,
    pub poll_interval: u64,
}

impl WatchmenConfig {
    pub fn load_from_file(path: String) -> WatchmenConfig {
        let content = fs::read_to_string(path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
