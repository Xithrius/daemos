use crate::{config::core::CoreConfig, utils::positioning::centered_position};

const DEFAULT_SETTINGS_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];

#[derive(Debug, Clone)]
pub struct Settings {
    #[allow(dead_code)]
    config: CoreConfig,
    visible: bool,
}

impl Settings {
    pub fn new(config: CoreConfig) -> Self {
        Self {
            config,
            visible: false,
        }
    }

    pub fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.visible {
            return;
        }

        egui::Window::new("Settings")
            .open(&mut self.visible)
            .resizable(true)
            .title_bar(true)
            .default_pos(centered_position(ctx, DEFAULT_SETTINGS_WINDOW_SIZE))
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_SETTINGS_WINDOW_SIZE.into());

                ui.label("Placeholder for the settings window");
            });
    }
}
