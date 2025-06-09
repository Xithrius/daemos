use std::rc::Rc;

use egui::{Id, Modal};
use tracing::error;

use crate::{
    components::{ComponentChannels, modals::UIModal},
    context::SharedContext,
    database::connection::DatabaseCommand,
};

const DEFAULT_PLAYLIST_MODAL_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];
const PLAYLIST_MODAL_ID: &str = "create_playlist_modal";

pub struct CreatePlaylistModal {
    context: SharedContext,
    channels: Rc<ComponentChannels>,

    playlist_name: String,
}

impl UIModal for CreatePlaylistModal {
    fn set_visibility(&mut self, visibility: bool) {
        self.context
            .borrow_mut()
            .ui
            .set_visible_playlist_modal(visibility);
    }
}

impl CreatePlaylistModal {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        Self {
            context,
            channels,
            playlist_name: String::default(),
        }
    }

    pub fn playlist_name(&self) -> &String {
        &self.playlist_name
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let show_modal = {
            let context = self.context.borrow();
            context.ui.visible_playlist_modal()
        };

        if !show_modal {
            return;
        }

        let mut should_close = false;

        let modal = Modal::new(Id::new(PLAYLIST_MODAL_ID)).show(ctx, |ui| {
            ui.set_min_size(DEFAULT_PLAYLIST_MODAL_WINDOW_SIZE.into());
            ui.label("New playlist name:");
            ui.text_edit_singleline(&mut self.playlist_name);

            ui.separator();

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Save").clicked() {
                        let new_playlist_name = self.playlist_name.clone().trim().to_string();

                        if !new_playlist_name.is_empty() {
                            if let Err(err) = self
                                .channels
                                .database_command_tx
                                .send(DatabaseCommand::InsertPlaylist(new_playlist_name))
                            {
                                error!(
                                    "Failed to send playlist command for new playlist: {:?}",
                                    err
                                );
                            }
                        }
                        should_close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                },
            );

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    // TODO: Send information somewhere for playlist creation
                    self.set_visibility(false);
                }

                if ui.button("Cancel").clicked() {
                    self.set_visibility(false);
                }
            });
        });

        if modal.should_close() || should_close {
            self.set_visibility(false);
        }
    }
}
