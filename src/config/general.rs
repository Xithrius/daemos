use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    pub debug: bool,
    pub vsync: bool,
}
