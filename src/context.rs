use std::{cell::RefCell, rc::Rc};

use egui::{Key, Separator};
use serde::{Deserialize, Serialize};

use crate::{
    components::{menu_bar::MenuBar, playback::PlaybackBar, table::Table, tree::Tree},
    config::core::CoreConfig,
    database::connection::{Database, SharedDatabase},
    horizontal_separator, vertical_separator,
};

#[derive(Deserialize, Serialize)]
pub struct Context {
    config: CoreConfig,

    #[serde(skip)]
    #[allow(dead_code)]
    database: SharedDatabase,

    // Components
    top_menu_bar: MenuBar,
    track_table: Table,
    playlist_tree: Tree,
    playback_bar: PlaybackBar,
}

impl Context {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: CoreConfig, database: Database) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let shared_database = Rc::new(RefCell::new(database));

        Self {
            config,
            database: shared_database.clone(),

            top_menu_bar: Default::default(),
            track_table: Table::new(shared_database),
            playlist_tree: Default::default(),
            playback_bar: Default::default(),
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

        if ctx.input(|i| i.key_pressed(Key::F3)) {
            self.config.debug = !self.config.debug;

            if self.config.debug != ctx.debug_on_hover() {
                ctx.set_debug_on_hover(self.config.debug);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.top_menu_bar.ui(ctx, ui);
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
