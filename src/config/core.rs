use serde::{Deserialize, Serialize};

const DEFAULT_PLAYER_VOLUME: f32 = 0.5;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub general: GeneralConfig,
    pub volume: VolumeConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    pub debug: bool,
    pub vsync: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeConfig {
    pub default: f32,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        Self {
            default: DEFAULT_PLAYER_VOLUME,
        }
    }
}
