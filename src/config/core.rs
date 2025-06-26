use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::config::{general::GeneralConfig, playback::PlaybackConfig, ui::UIConfig};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct CoreConfig {
    pub general: GeneralConfig,
    pub ui: UIConfig,
    pub playback: PlaybackConfig,
}

pub type SharedConfig = Rc<RefCell<CoreConfig>>;
