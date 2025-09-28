use serde::{Deserialize, Serialize};

const DEFAULT_SERVER_ADDRESS: &str = "http://localhost:9090";

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct ServerConfig {
    enabled: bool,
    address: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            address: DEFAULT_SERVER_ADDRESS.to_string(),
        }
    }
}

impl ServerConfig {
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
