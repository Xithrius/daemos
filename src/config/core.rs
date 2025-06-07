use std::{cell::RefCell, rc::Rc};

use egui::Align;
use serde::{Deserialize, Serialize};

use crate::themes::AppTheme;

const DEFAULT_PLAYER_VOLUME: f32 = 0.5;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub general: GeneralConfig,
    pub volume: VolumeConfig,
    pub autoplay: AutoplayConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GeneralConfig {
    pub theme: AppTheme,
    pub debug: bool,
    pub vsync: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AutoplayConfig {
    pub align: Option<Align>,
}

pub type SharedConfig = Rc<RefCell<CoreConfig>>;
