use egui::Separator;
use serde::{Deserialize, Serialize};

use crate::{
    components::{playback::PlaybackBar, table::Table, tree::Tree},
    config::core::CoreConfig,
    horizontal_separator, vertical_separator,
};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Context {
    config: CoreConfig,

    // Widgets
    track_table: Table,
    playlist_tree: Tree,
    playback_bar: PlaybackBar,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            track_table: Table::new(),
            playlist_tree: Tree::new(),
            playback_bar: PlaybackBar::new(),
            config: CoreConfig::default(),
        }
    }
}

impl Context {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: CoreConfig) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Self {
            config,
            ..Default::default()
        }
    }
}

impl eframe::App for Context {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.config.debug != ctx.debug_on_hover() {
            ctx.set_debug_on_hover(self.config.debug);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Adding files, folders, playlists, importing, exporting, etc
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Something to do with editing things
                ui.menu_button("Edit", |_ui| {
                    todo!();
                });

                // Something to do with the window
                ui.menu_button("Window", |_ui| {
                    todo!();
                });

                // Useful links
                ui.menu_button("Help", |ui| {
                    ui.hyperlink_to("Github Repository", "https://github.com/Xithrius/drakn");
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    ui.horizontal(|ui| {
                        // Theme switcher
                        egui::widgets::global_theme_preference_switch(ui);

                        // Debug build status
                        egui::warn_if_debug_build(ui);
                    })
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let total_height = ui.available_height();
            let playback_bar_height = 60.0;

            let table_area_height = (total_height - playback_bar_height).max(100.0);

            let width = ui.available_width();

            ui.allocate_ui(egui::vec2(width, total_height), |ui| {
                ui.vertical(|ui| {
                    ui.allocate_ui(egui::vec2(width, table_area_height), |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            self.playlist_tree.ui(ui);
                            vertical_separator!(ui);

                            ui.vertical(|ui| {
                                self.track_table.ui(ui, table_area_height);
                            });
                        });
                    });

                    horizontal_separator!(ui);

                    ui.allocate_ui(egui::vec2(width, playback_bar_height), |ui| {
                        self.playback_bar.ui(ui);
                    });
                });
            });
        });
    }
}
