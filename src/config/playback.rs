use serde::{Deserialize, Serialize};

use crate::context::AutoplayType;

const DEFAULT_PLAYER_VOLUME: f32 = 0.5;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlaybackConfig {
    pub autoplay: AutoplayType,
    pub volume: f32,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            autoplay: AutoplayType::default(),
            volume: DEFAULT_PLAYER_VOLUME,
        }
    }
}
