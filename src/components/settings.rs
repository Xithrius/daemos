use crate::{config::core::CoreConfig, context::SharedContext};

use super::utils::positioning::centered_position;

const DEFAULT_SETTINGS_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];

#[derive(Debug, Clone)]
pub struct Settings {
    #[allow(dead_code)]
    config: CoreConfig,
    context: SharedContext,
}

impl Settings {
    pub fn new(config: CoreConfig, context: SharedContext) -> Self {
        Self { config, context }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().visible_settings() {
            return;
        }

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().visible_settings_mut())
            .resizable(true)
            .title_bar(true)
            .default_pos(centered_position(ctx, DEFAULT_SETTINGS_WINDOW_SIZE))
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_SETTINGS_WINDOW_SIZE.into());

                ui.label("Placeholder for the settings window");
            });
    }
}
