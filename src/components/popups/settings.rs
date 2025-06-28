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
        let mut new_theme: Option<AppTheme> = None;

        let current_theme = self.selected.theme;
        let current_autoplay = self.selected.autoplay.clone();
        let current_search = self.selected.search.clone();

        egui::Window::new("Settings")
            .open(self.context.borrow_mut().ui.visibility.settings_mut())
            .resizable(true)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_SETTINGS_WINDOW_SIZE.into());

                ui.vertical(|ui| {
                    Self::render_theme_section(ui, current_theme, &mut new_theme);
                    Self::render_autoplay_section(ui, current_autoplay, &mut new_autoplay);
                    Self::render_search_section(ui, current_search, &mut new_search_strategy);
                })
            });

        if let Some(theme) = new_theme {
            self.selected.theme = theme;
            match theme {
                AppTheme::Dark => {
                    ctx.set_visuals(egui::Visuals::dark());
                }
                AppTheme::Latte => catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE),
                AppTheme::Frappe => catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE),
                AppTheme::Macchiato => catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO),
                AppTheme::Mocha => catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA),
            }
        }

        if let Some(autoplay) = new_autoplay {
            self.selected.autoplay = autoplay.clone();
            self.context
                .borrow_mut()
                .playback
                .autoplay
                .set_autoplay(autoplay);
        }

        if let Some(search_strategy) = new_search_strategy {
            self.selected.search = search_strategy.clone();
            self.config.borrow_mut().search.strategy = search_strategy;
        }
    }

    fn render_theme_section(
        ui: &mut egui::Ui,
        current_theme: AppTheme,
        new_theme: &mut Option<AppTheme>,
    ) {
        ui.horizontal(|ui| {
            ui.label("Theme");

            let mut selected_theme = current_theme;
            let theme_options = [
                ("Dark", AppTheme::Dark),
                ("Latte", AppTheme::Latte),
                ("Frappe", AppTheme::Frappe),
                ("Macchiato", AppTheme::Macchiato),
                ("Mocha", AppTheme::Mocha),
            ];

            egui::ComboBox::from_id_salt("Theme combobox")
                .selected_text(format!("{selected_theme}"))
                .show_ui(ui, |ui| {
                    for (label, value) in theme_options {
                        if ui
                            .selectable_value(&mut selected_theme, value, label)
                            .clicked()
                        {
                            *new_theme = Some(value);
                        }
                    }
                });
        });
    }

    fn render_autoplay_section(
        ui: &mut egui::Ui,
        current_autoplay: AutoplayType,
        new_autoplay: &mut Option<AutoplayType>,
    ) {
        ui.horizontal(|ui| {
            ui.label("Autoplay");

            let mut selected_autoplay = current_autoplay;
            egui::ComboBox::from_id_salt("Autoplay combobox")
                .selected_text(format!("{selected_autoplay}"))
                .show_ui(ui, |ui| {
                    for autoplay_option in AUTOPLAY_OPTIONS {
                        if ui
                            .selectable_value(
                                &mut selected_autoplay,
                                autoplay_option.clone(),
                                autoplay_option.to_string(),
                            )
                            .clicked()
                        {
                            *new_autoplay = Some(autoplay_option);
                        }
                    }
                });
        });
    }

    fn render_search_section(
        ui: &mut egui::Ui,
        current_search: SearchMatchingStrategy,
        new_search_strategy: &mut Option<SearchMatchingStrategy>,
    ) {
        ui.horizontal(|ui| {
            ui.label("Search strategy");

            let mut selected_search = current_search;
            egui::ComboBox::from_id_salt("Search combobox")
                .selected_text(format!("{selected_search}"))
                .show_ui(ui, |ui| {
                    for search_option in SEARCH_STRATEGY_OPTIONS {
                        if ui
                            .selectable_value(
                                &mut selected_search,
                                search_option.clone(),
                                search_option.to_string(),
                            )
                            .clicked()
                        {
                            *new_search_strategy = Some(search_option);
                        }
                    }
                });
        });
    }
}
