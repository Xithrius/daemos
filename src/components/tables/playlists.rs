use std::rc::Rc;

use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};

use super::TABLE_ROW_HEIGHT;
use crate::{
    components::ComponentChannels,
    context::SharedContext,
    database::{connection::DatabaseCommand, models::playlists::playlist::Playlist},
};

const PLAYLIST_TABLE_WIDTH: f32 = 200.0;

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
        let height = ui.available_height();

        let table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(PLAYLIST_TABLE_WIDTH))
            .sense(egui::Sense::click());

        table
            // .header(TABLE_HEADER_HEIGHT, |mut header| {
            //     header.col(|ui| {
            //         ui.heading("Playlist");
            //     });
            //     // header.col(|ui| {
            //     //     ui.heading("Tracks");
            //     // });
            // })
            .body(|body| {

                let num_rows ={ let context = self.context.borrow(); context.playback.loaded.playlists().len()};


                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let playlists = self.context.borrow().playback.loaded.playlists();
                    let row_index = row.index();
                    let Some(playlist) = playlists.get(row_index).cloned() else {
                        return;
                    };

                    let selected = self.context.borrow().ui_playlist
                        .selected()
                        .is_some_and(|selected_playlist| selected_playlist.id == playlist.id);
                    row.set_selected(selected);

                    row.col(|ui| {
                        let label = ui
                            .label(playlist.name.clone())
                            .on_hover_cursor(egui::CursorIcon::Default);
                        if label.clicked() {
                            self.toggle_playlist_selection(&playlist);
                        }
                    });

                    if row.response().clicked() {
                        self.toggle_playlist_selection(&playlist);
                    }
                });
            });
    }

    fn toggle_playlist_selection(&mut self, playlist: &Playlist) {
        let mut query_all_tracks = false;
        if self
            .context
            .borrow()
            .ui_playlist
            .selected()
            .is_some_and(|selected_playlist| selected_playlist.id == playlist.id)
        {
            self.context.borrow_mut().ui_playlist.set_selected(None);
            query_all_tracks = true;
        } else {
            self.context
                .borrow_mut()
                .ui_playlist
                .set_selected(Some(playlist.clone()));
        }

        let query = if query_all_tracks {
            debug!("De-selected playlist, querying all");

            None
        } else {
            debug!(
                "Selected playlist: {:?}",
                self.context.borrow().ui_playlist.selected()
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
