use egui::Align;
use serde::{Deserialize, Serialize};

use crate::themes::AppTheme;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(default)]
pub struct UIConfig {
    pub theme: AppTheme,
    pub align_scroll: Option<Align>,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: AppTheme::default(),
            align_scroll: Some(Align::Center),
        }
    }
}
