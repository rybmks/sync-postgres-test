use serde_json::Value;
use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    pub connection_string: String,
}

impl Config {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }

    pub fn get_from_cfg_json() -> Self {
        let key = "connectionString";
        let path_to_cfg = "cfg.json";

        let config_data = fs::read_to_string("cfg.json").unwrap_or_else(|err: io::Error| {
            panic!("Failed to read cfg path: {}. Error: {}", path_to_cfg, err)
        });
        let json: Value = serde_json::from_str(&config_data).expect("Failed to parse json");

        let connection_string = json
            .get(key)
            .and_then(|value| value.as_str())
            .unwrap_or_else(|| panic!("Failed to get value by {} key.", key));

        Self {
            connection_string: connection_string.to_string(),
        }
    }
}
