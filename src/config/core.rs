use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub debug: bool,
    pub vsync: bool,
}
