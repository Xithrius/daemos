use egui::Separator;
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

use crate::{
    components::{table::Table, tree::Tree},
    vertical_separator,
};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Context {
    track_table: Table,
    playlist_tree: Tree,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
}

#[allow(clippy::derivable_impls)]
impl Default for Context {
    fn default() -> Self {
        Self {
            track_table: Table::new(),
            playlist_tree: Tree::new(),
        }
    }
}

impl Context {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Context {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Adding files, folders, playlists, importing, exporting, etc
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Something to do with editing things
                ui.menu_button("Edit", |_ui| todo!());

                // Something to do with the window
                ui.menu_button("Window", |_ui| todo!());

                // Useful links
                ui.menu_button("Help", |ui| {
                    ui.hyperlink_to("Github Repository", "https://github.com/Xithrius/drakn");
                });

                // Theme switcher
                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    egui::widgets::global_theme_preference_switch(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                // Playlist tree
                self.playlist_tree.ui(ui);

                vertical_separator!(ui);

                // Audio table
                ui.vertical(|ui| {
                    self.track_table.ui(ui);
                });
                // ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

                // });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
