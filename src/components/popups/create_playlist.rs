use std::rc::Rc;

use tracing::error;

use crate::{
    components::ComponentChannels, context::SharedContext, database::connection::DatabaseCommand,
};

pub struct CreatePlaylistModal {
    context: SharedContext,
    channels: Rc<ComponentChannels>,

    playlist_name: String,
}

impl CreatePlaylistModal {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        Self {
            context,
            channels,
            playlist_name: String::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let show_modal = {
            let context = self.context.borrow();
            context.visible_playlist_modal()
        };

        if !show_modal {
            return;
        }

        let mut should_close = false;

        egui::Window::new("Create Playlist")
            .open(self.context.borrow_mut().visible_playlist_modal_mut())
            .resizable(false)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.set_width(250.0);
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
            });

        if should_close {
            self.context.borrow_mut().set_visible_playlist_modal(false);
        }
    }
}
