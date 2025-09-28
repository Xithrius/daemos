use std::rc::Rc;

use egui::Vec2;
use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};

use super::TABLE_ROW_HEIGHT;
use crate::{
    components::ComponentChannels,
    context::SharedContext,
    database::{connection::DatabaseCommand, models::playlists::playlist::Playlist},
};

#[derive(Debug, Clone)]
pub struct PlaylistTable {
    context: SharedContext,
    channels: Rc<ComponentChannels>,
}

impl PlaylistTable {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        let _ = channels
            .database_command_tx
            .send(DatabaseCommand::QueryPlaylists);

        Self { context, channels }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let Vec2 {
            x: width,
            y: height,
        } = ui.available_size();

        let table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(width).at_most(width))
            .sense(egui::Sense::click());

        table.body(|body| {
            let playlist_keys: Vec<_> = {
                let context = self.context.borrow();
                context.storage.playlists().cloned().collect()
            };

            let num_rows = playlist_keys.len();

            body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                let row_index = row.index();
                let Some(playlist) = playlist_keys.get(row_index) else {
                    return;
                };

                let selected = self
                    .context
                    .borrow()
                    .ui
                    .playlist
                    .selected()
                    .is_some_and(|selected_playlist| selected_playlist.id == playlist.id);
                row.set_selected(selected);

                row.col(|ui| {
                    let label = ui
                        .label(playlist.name.clone())
                        .on_hover_cursor(egui::CursorIcon::Default);
                    if label.clicked() {
                        self.toggle_playlist_selection(playlist);
                    }
                });

                if row.response().clicked() {
                    self.toggle_playlist_selection(playlist);
                }
            });
        });
    }

    fn toggle_playlist_selection(&mut self, playlist: &Playlist) {
        let mut query_all_tracks = false;
        if self
            .context
            .borrow()
            .ui
            .playlist
            .selected()
            .is_some_and(|selected_playlist| selected_playlist.id == playlist.id)
        {
            self.context.borrow_mut().ui.playlist.set_selected(None);
            query_all_tracks = true;
        } else {
            self.context
                .borrow_mut()
                .ui
                .playlist
                .set_selected(Some(playlist.clone()));
        }

        let query = if query_all_tracks {
            debug!("De-selected playlist, querying all");

            None
        } else {
            debug!(
                "Selected playlist: {:?}",
                self.context.borrow().ui.playlist.selected()
            );

            Some(playlist.clone())
        };

        if let Err(err) = self
            .channels
            .database_command_tx
            .send(DatabaseCommand::QueryTracks(query))
        {
            error!("Error when sending playlist track query: {}", err)
        }
    }
}
