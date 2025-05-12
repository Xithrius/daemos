use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MenuBar;

impl MenuBar {
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, visible_settings: &mut bool) {
        // Adding files, folders, playlists, importing, exporting, etc
        self.ui_file(ctx, ui, visible_settings);

        // Something to do with editing things
        self.ui_edit(ui);

        // Something to do with the window
        self.ui_window(ui);

        // Useful links
        self.ui_help(ui);

        // Extra
        self.ui_extra(ui);
    }

    fn ui_file(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, visible_settings: &mut bool) {
        ui.menu_button("File", |ui| {
            if ui.button("Preferences").clicked() {
                *visible_settings = true;
            } else if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn ui_edit(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |_ui| {
            todo!();
        });
    }

    fn ui_window(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Window", |_ui| {
            todo!();
        });
    }

    fn ui_help(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            ui.hyperlink_to("Github Repository", "https://github.com/Xithrius/drakn");
        });
    }

    fn ui_extra(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                // Theme switcher
                egui::widgets::global_theme_preference_switch(ui);

                // Debug build status
                egui::warn_if_debug_build(ui);
            })
        });
    }
}
