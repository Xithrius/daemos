use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    host: String,
    port: usize,
    worker_count: usize,

    environment: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 7070,
            worker_count: 2,
            environment: "dev".to_string(),
        }
    }
}

impl Config {
    #[must_use]
    pub fn new(config_path: String) -> Self {
        read_to_string(config_path).map_or_else(
            |_| {
                warn!("Could not find config, opting for default config");
                Self::default()
            },
            |file_content| match toml::from_str(&file_content) {
                Ok(c) => c,
                Err(err) => {
                    warn!("Error when parsing config: {err}");
                    warn!("Opting for default config");

                    Self::default()
                }
            },
        )
    }

    #[must_use]
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
