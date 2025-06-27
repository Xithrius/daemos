use crate::{
    config::{core::SharedConfig, search::SearchMatchingStrategy},
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

const SEARCH_STRATEGY_OPTIONS: [SearchMatchingStrategy; 3] = [
    SearchMatchingStrategy::Fuzzy,
    SearchMatchingStrategy::ContainsExact,
    SearchMatchingStrategy::ContainsLowercase,
];

#[derive(Debug, Clone)]
pub struct SelectedSettings {
    theme: AppTheme,
    autoplay: AutoplayType,
    search: SearchMatchingStrategy,
}

impl From<SharedConfig> for SelectedSettings {
    fn from(config: SharedConfig) -> Self {
        let c = config.borrow();

        Self {
            theme: c.ui.theme,
            autoplay: c.playback.autoplay.clone(),
            search: c.search.strategy.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsPopup {
    config: SharedConfig,
    context: SharedContext,
    selected: SelectedSettings,
}

impl SettingsPopup {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        let selected_settings = SelectedSettings::from(config.clone());

        Self {
            config,
            context,
            selected: selected_settings,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().ui.visibility.settings() {
            return;
        }

        let mut new_autoplay: Option<AutoplayType> = None;
        let mut new_search_strategy: Option<SearchMatchingStrategy> = None;

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().ui.visibility.settings_mut())
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
                            .selected_text(format!("{}", self.selected.theme))
                            .show_ui(ui, |ui| {
                                let mut make_theme_option =
                                    |ui: &mut egui::Ui, label: &str, value: AppTheme| {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected.theme,
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
                            .selected_text(format!("{}", self.selected.autoplay))
                            .show_ui(ui, |ui| {
                                let mut make_autoplay_option =
                                    |ui: &mut egui::Ui, label: &str, value: AutoplayType| {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected.autoplay,
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

                    // Search section
                    ui.horizontal(|ui| {
                        ui.label("Search strategy");

                        egui::ComboBox::from_id_salt("Search combobox")
                            .selected_text(format!("{}", self.selected.search))
                            .show_ui(ui, |ui| {
                                let mut make_search_strategy_option =
                                    |ui: &mut egui::Ui, label: &str, value: SearchMatchingStrategy| {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected.search,
                                                value.clone(),
                                                label,
                                            )
                                            .clicked()
                                        {
                                            new_search_strategy = Some(value);
                                        }
                                    };

                                for search_option in SEARCH_STRATEGY_OPTIONS {
                                    make_search_strategy_option(
                                        ui,
                                        &search_option.to_string(),
                                        search_option,
                                    );
                                }
                            });
                    });
                })
            });

        if let Some(autoplay) = new_autoplay {
            self.context
                .borrow_mut()
                .playback
                .autoplay
                .set_autoplay(autoplay);
        }

        if let Some(search_strategy) = new_search_strategy {
            self.config.borrow_mut().search.strategy = search_strategy;
        }
    }
}
