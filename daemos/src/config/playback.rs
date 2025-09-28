use serde::{Deserialize, Serialize};

use crate::context::AutoplayType;

const DEFAULT_PLAYER_VOLUME: f32 = 0.5;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct PlaybackConfig {
    pub autoplay: AutoplayType,
    pub volume: f32,
    pub add_to_seen_on_skip: bool,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            autoplay: AutoplayType::default(),
            volume: DEFAULT_PLAYER_VOLUME,
            add_to_seen_on_skip: true,
        }
    }
}
