use egui::Color32;

use crate::context::MenuContextAccess;

const GITHUB_REPOSITORY_URL: &str = "https://github.com/Xithrius/daemos";

#[derive(Debug)]
pub struct MenuBar {
    context: MenuContextAccess,
}

impl MenuBar {
    pub fn new(context: MenuContextAccess) -> Self {
        Self { context }
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Adding files, folders, playlists, importing, exporting, etc
        self.ui_file(ctx, ui);

        // Something to do with the window
        self.ui_window(ui);

        // Useful links
        self.ui_help(ui);

        // Extra
        self.ui_extra(ui);
    }

    fn ui_file(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            ui.menu_button("New", |ui| {
                if ui.button("Playlist").clicked() {
                    self.context.with_ui_mut(|ui_context| {
                        ui_context.visibility.set_playlist_modal(true);
                    });
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("Preferences").clicked() {
                self.context.with_ui_mut(|ui_context| {
                    ui_context.visibility.set_settings(true);
                });
                ui.close_menu();
            } else if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                ui.close_menu();
            }
        });
    }

    fn ui_window(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Window", |ui| {
            ui.menu_button("Debug", |ui| {
                if ui.button("General").clicked() {
                    self.context.with_ui_mut(|ui_context| {
                        ui_context.visibility.set_debug(true);
                    });
                    ui.close_menu();
                } else if ui.button("Playback").clicked() {
                    self.context.with_ui_mut(|ui_context| {
                        ui_context.visibility.set_debug_playback(true);
                    });
                    ui.close_menu();
                }
            });
        });
    }

    fn ui_help(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            ui.hyperlink_to("Github Repository", GITHUB_REPOSITORY_URL);
        });
    }

    fn processing_spinner(&mut self, ui: &mut egui::Ui) {
        // Note: This would need a ProcessingContextAccess if we want to access processing data
        // For now, we'll need to pass this data differently or create a ProcessingContextAccess
        // let processing_tracks = self.context.with_processing(|processing| processing.total());

        // Placeholder - you'd need to add ProcessingContextAccess to MenuContextAccess
        // or pass this data through a different mechanism
        let processing_tracks = 0; // TODO: Implement proper access

        if processing_tracks > 0 {
            let processing_tracks_text = format!("Processing {processing_tracks} track(s)");
            ui.label(processing_tracks_text);

            let spinner = egui::Spinner::new().size(14.0).color(Color32::GRAY);
            ui.add(spinner);
        }
    }

    fn ui_extra(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                // Debug build status
                egui::warn_if_debug_build(ui);

                // Track processing spinner
                self.processing_spinner(ui);
            })
        });
    }
}
