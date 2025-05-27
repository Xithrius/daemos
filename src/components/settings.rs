use std::fmt;

use crate::{config::core::CoreConfig, context::SharedContext};

const DEFAULT_SETTINGS_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum AppTheme {
    #[default]
    Dark,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl fmt::Display for AppTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            AppTheme::Dark => "Dark",
            AppTheme::Latte => "Latte",
            AppTheme::Frappe => "Frappe",
            AppTheme::Macchiato => "Macchiato",
            AppTheme::Mocha => "Mocha",
        };

        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    #[allow(dead_code)]
    config: CoreConfig,
    context: SharedContext,
    selected: AppTheme,
}

impl Settings {
    pub fn new(config: CoreConfig, context: SharedContext) -> Self {
        Self {
            config,
            context,
            selected: AppTheme::Dark,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().visible_settings() {
            return;
        }

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().visible_settings_mut())
            .resizable(true)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_SETTINGS_WINDOW_SIZE.into());

                ui.horizontal(|ui| {
                    ui.label("Theme");

                    egui::ComboBox::from_id_salt("Theme combobox")
                        .selected_text(format!("{:?}", self.selected))
                        .show_ui(ui, |ui| {
                            let mut make_option =
                                |ui: &mut egui::Ui, label: &str, value: AppTheme| {
                                    if ui
                                        .selectable_value(&mut self.selected, value, label)
                                        .clicked()
                                    {
                                        match value {
                                            AppTheme::Dark => {
                                                ctx.set_visuals(egui::Visuals::dark());
                                            }
                                            AppTheme::Latte => catppuccin_egui::set_theme(
                                                ctx,
                                                catppuccin_egui::LATTE,
                                            ),
                                            AppTheme::Frappe => catppuccin_egui::set_theme(
                                                ctx,
                                                catppuccin_egui::FRAPPE,
                                            ),
                                            AppTheme::Macchiato => catppuccin_egui::set_theme(
                                                ctx,
                                                catppuccin_egui::MACCHIATO,
                                            ),
                                            AppTheme::Mocha => catppuccin_egui::set_theme(
                                                ctx,
                                                catppuccin_egui::MOCHA,
                                            ),
                                        }
                                    }
                                };

                            make_option(ui, "Dark", AppTheme::Dark);
                            make_option(ui, "Latte", AppTheme::Latte);
                            make_option(ui, "Frappe", AppTheme::Frappe);
                            make_option(ui, "Macchiato", AppTheme::Macchiato);
                            make_option(ui, "Mocha", AppTheme::Mocha);
                        });
                });
            });
    }
}
