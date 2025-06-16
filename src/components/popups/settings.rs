use crate::{
    config::core::SharedConfig,
    context::{AutoplayType, PlayDirection, SharedContext, ShuffleType},
    themes::AppTheme,
};

const DEFAULT_SETTINGS_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];

const AUTOPLAY_OPTIONS: [AutoplayType; 4] = [
    AutoplayType::Iterative(PlayDirection::Backward),
    AutoplayType::Iterative(PlayDirection::Forward),
    AutoplayType::Shuffle(ShuffleType::PseudoRandom),
    AutoplayType::Shuffle(ShuffleType::TrueRandom),
];

#[derive(Debug, Clone)]
pub struct Settings {
    // config: CoreConfig,
    context: SharedContext,
    selected_theme: AppTheme,
    selected_autoplay: AutoplayType,
}

impl Settings {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        let c = config.borrow();

        Self {
            context,
            selected_theme: c.general.theme,
            selected_autoplay: c.autoplay.autoplay.clone(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().ui.visible_settings() {
            return;
        }

        let mut new_autoplay: Option<AutoplayType> = None;

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().ui.visible_settings_mut())
            .resizable(true)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_SETTINGS_WINDOW_SIZE.into());

                // TODO: Figure out how to separate this into methods
                ui.vertical(|ui| {
                    // Theme selection
                    ui.horizontal(|ui| {
                        ui.label("Theme");

                        egui::ComboBox::from_id_salt("Theme combobox")
                            .selected_text(format!("{}", self.selected_theme))
                            .show_ui(ui, |ui| {
                                let mut make_theme_option =
                                    |ui: &mut egui::Ui, label: &str, value: AppTheme| {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected_theme,
                                                value,
                                                label,
                                            )
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

                                make_theme_option(ui, "Dark", AppTheme::Dark);
                                make_theme_option(ui, "Latte", AppTheme::Latte);
                                make_theme_option(ui, "Frappe", AppTheme::Frappe);
                                make_theme_option(ui, "Macchiato", AppTheme::Macchiato);
                                make_theme_option(ui, "Mocha", AppTheme::Mocha);
                            });
                    });

                    // Autoplay selection
                    ui.horizontal(|ui| {
                        ui.label("Autoplay");

                        egui::ComboBox::from_id_salt("Autoplay combobox")
                            .selected_text(format!("{}", self.selected_autoplay))
                            .show_ui(ui, |ui| {
                                let mut make_autoplay_option =
                                    |ui: &mut egui::Ui, label: &str, value: AutoplayType| {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected_autoplay,
                                                value.clone(),
                                                label,
                                            )
                                            .clicked()
                                        {
                                            new_autoplay = Some(value);
                                        }
                                    };

                                for autoplay_option in AUTOPLAY_OPTIONS {
                                    make_autoplay_option(
                                        ui,
                                        &autoplay_option.to_string(),
                                        autoplay_option,
                                    );
                                }
                            });
                    });
                })
            });

        if let Some(autoplay) = new_autoplay {
            self.context.borrow_mut().playback.set_autoplay(autoplay);
        }
    }
}
