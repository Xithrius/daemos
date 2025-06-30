use tracing::{error, info};

use crate::{
    config::{
        core::{CoreConfig, SharedConfig},
        save_config,
        search::SearchMatchingStrategy,
    },
    context::{AutoplayType, PlayDirection, SharedContext, ShuffleType},
    themes::AppTheme,
};

const DEFAULT_SETTINGS_WINDOW_SIZE: [f32; 2] = [150.0, 200.0];

const AUTOPLAY_OPTIONS: [AutoplayType; 4] = [
    AutoplayType::Iterative(PlayDirection::Backward),
    AutoplayType::Iterative(PlayDirection::Forward),
    AutoplayType::Shuffle(ShuffleType::PseudoRandom),
    AutoplayType::Shuffle(ShuffleType::TrueRandom),
];

const SEARCH_STRATEGY_OPTIONS: [SearchMatchingStrategy; 3] = [
    SearchMatchingStrategy::Fuzzy,
    SearchMatchingStrategy::ContainsExact,
    SearchMatchingStrategy::ContainsLowercase,
];

#[derive(Debug, Clone)]
pub struct SettingsPopup {
    config: SharedConfig,
    context: SharedContext,
    selected: CoreConfig,
    changed: bool,
}

impl SettingsPopup {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        let selected = config.borrow().clone();

        Self {
            config,
            context,
            selected,
            changed: false,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        {
            if !self.context.borrow().ui.visibility.settings() {
                return;
            }
        }

        let mut changed = self.changed;
        let mut ok_clicked = false;
        let mut apply_clicked = false;
        let mut cancel_clicked = false;

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().ui.visibility.settings_mut())
            .resizable(true)
            .title_bar(true)
            .min_size(egui::Vec2::from(DEFAULT_SETTINGS_WINDOW_SIZE))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    Self::render_theme_section(
                        ui,
                        self.selected.ui.theme,
                        &mut self.selected.ui.theme,
                        &mut changed,
                    );

                    Self::render_autoplay_section(
                        ui,
                        self.selected.playback.autoplay.clone(),
                        &mut self.selected.playback.autoplay,
                        &mut changed,
                    );

                    Self::render_search_section(
                        ui,
                        self.selected.search.strategy.clone(),
                        &mut self.selected.search.strategy,
                        &mut changed,
                    );

                    ui.add_space(10.0);
                    ui.separator();

                    Self::render_buttons(
                        ui,
                        &mut changed,
                        &mut ok_clicked,
                        &mut apply_clicked,
                        &mut cancel_clicked,
                    );

                    ui.allocate_space(egui::Vec2::new(
                        0.0,
                        ui.available_rect_before_wrap().height(),
                    ));
                })
            });

        self.changed = changed;

        let mut should_close = false;

        // Reset to shared config

        if cancel_clicked {
            self.selected = self.config.borrow().clone();
            self.changed = false;
            should_close = true;
        }

        // Apply to shared config only
        if ok_clicked {
            self.apply_to_shared_config(ctx);
            self.changed = false;
            should_close = true;
        }

        // Apply to shared config and save to file system
        if apply_clicked {
            self.apply_to_shared_config(ctx);
            self.save_to_file_system();
            self.changed = false;
            should_close = true;
        }

        if should_close {
            self.context.borrow_mut().ui.visibility.set_settings(false);
        }
    }

    fn apply_to_shared_config(&mut self, ctx: &egui::Context) {
        // Get the current shared config for comparison and clone it
        let current_config = self.config.borrow().clone();

        // Apply immediate UI/playback changes that need special handling
        Self::apply_immediate_changes(ctx, &current_config, &self.selected, &mut self.context);

        // Replace the entire shared config with the selected config
        *self.config.borrow_mut() = self.selected.clone();
    }

    fn apply_immediate_changes(
        ctx: &egui::Context,
        current_config: &CoreConfig,
        selected_config: &CoreConfig,
        context: &mut SharedContext,
    ) {
        // Theme
        if selected_config.ui.theme != current_config.ui.theme {
            match selected_config.ui.theme {
                AppTheme::Dark => ctx.set_visuals(egui::Visuals::dark()),
                AppTheme::Latte => catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE),
                AppTheme::Frappe => catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE),
                AppTheme::Macchiato => catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO),
                AppTheme::Mocha => catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA),
            }
        }

        // Autoplay
        if selected_config.playback.autoplay != current_config.playback.autoplay {
            context
                .borrow_mut()
                .playback
                .autoplay
                .set_autoplay(selected_config.playback.autoplay.clone());
        }
    }

    fn save_to_file_system(&self) {
        match save_config(&self.selected) {
            Ok(()) => info!("Config saved successfully"),
            Err(err) => error!("Failed to save config: {}", err),
        }
    }

    fn render_theme_section(
        ui: &mut egui::Ui,
        current_theme: AppTheme,
        selected_theme: &mut AppTheme,
        changed: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.label("Theme");

            let mut local_theme = current_theme;
            let theme_options = [
                ("Dark", AppTheme::Dark),
                ("Latte", AppTheme::Latte),
                ("Frappe", AppTheme::Frappe),
                ("Macchiato", AppTheme::Macchiato),
                ("Mocha", AppTheme::Mocha),
            ];

            egui::ComboBox::from_id_salt("Theme combobox")
                .selected_text(format!("{local_theme}"))
                .show_ui(ui, |ui| {
                    for (label, value) in theme_options {
                        if ui
                            .selectable_value(&mut local_theme, value, label)
                            .clicked()
                        {
                            *selected_theme = value;
                            *changed = true;
                        }
                    }
                });
        });
    }

    fn render_autoplay_section(
        ui: &mut egui::Ui,
        current_autoplay: AutoplayType,
        selected_autoplay: &mut AutoplayType,
        changed: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.label("Autoplay");

            let mut local_autoplay = current_autoplay;
            egui::ComboBox::from_id_salt("Autoplay combobox")
                .selected_text(format!("{local_autoplay}"))
                .show_ui(ui, |ui| {
                    for autoplay_option in AUTOPLAY_OPTIONS {
                        if ui
                            .selectable_value(
                                &mut local_autoplay,
                                autoplay_option.clone(),
                                autoplay_option.to_string(),
                            )
                            .clicked()
                        {
                            *selected_autoplay = autoplay_option;
                            *changed = true;
                        }
                    }
                });
        });
    }

    fn render_search_section(
        ui: &mut egui::Ui,
        current_search: SearchMatchingStrategy,
        selected_search: &mut SearchMatchingStrategy,
        changed: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.label("Search strategy");

            let mut local_search = current_search;
            egui::ComboBox::from_id_salt("Search combobox")
                .selected_text(format!("{local_search}"))
                .show_ui(ui, |ui| {
                    for search_option in SEARCH_STRATEGY_OPTIONS {
                        if ui
                            .selectable_value(
                                &mut local_search,
                                search_option.clone(),
                                search_option.to_string(),
                            )
                            .clicked()
                        {
                            *selected_search = search_option;
                            *changed = true;
                        }
                    }
                });
        });
    }

    fn render_buttons(
        ui: &mut egui::Ui,
        changed: &mut bool,
        ok_clicked: &mut bool,
        apply_clicked: &mut bool,
        cancel_clicked: &mut bool,
    ) {
        egui::Sides::new().show(
            ui,
            |_ui| {},
            |ui| {
                let apply_button_response = {
                    ui.add_enabled(*changed, egui::Button::new("Apply"))
                        .on_hover_text("Apply changes to shared config and save to file system")
                };

                if apply_button_response.clicked() {
                    *apply_clicked = true;
                }

                ui.add_space(5.0);

                let ok_button_response = {
                    ui.add_enabled(*changed, egui::Button::new("OK"))
                        .on_hover_text("Apply changes to shared config only")
                };

                if ok_button_response.clicked() {
                    *ok_clicked = true;
                }

                ui.add_space(5.0);

                if ui.button("Cancel").clicked() {
                    info!("Cancel button clicked");
                    *cancel_clicked = true;
                }
            },
        );
    }
}
