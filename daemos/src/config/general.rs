use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct GeneralConfig {
    /// Enables wireframe rendering for debugging purposes.
    pub debug_wireframe: bool,
    /// Enables eframe's VSync.
    pub vsync: bool,
}
