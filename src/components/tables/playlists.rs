use std::rc::Rc;

use egui::ahash::HashSet;
use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};
use uuid::Uuid;

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

    playlists: Vec<Playlist>,
    playlist_ids: HashSet<Uuid>,
}

impl PlaylistTable {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        let _ = channels
            .database_command_tx
            .send(DatabaseCommand::QueryPlaylists);

        Self {
            context,
            channels,
            playlists: Vec::default(),
            playlist_ids: HashSet::default(),
        }
    }

    pub fn set_playlists(&mut self, playlists: Vec<Playlist>) {
        self.playlist_ids = playlists.iter().map(|playlist| playlist.id).collect();
        self.playlists = playlists;
    }

    pub fn add_playlist(&mut self, playlist: &Playlist) {
        if self.playlist_ids.insert(playlist.id) {
            self.playlists.push(playlist.clone());
        }
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
                let num_rows = self.playlists.len();

                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let row_index = row.index();
                    let Some(playlist) = self.playlists.get(row_index).cloned() else {
                        return;
                    };

                    let selected = self
                        .context
                        .borrow().playlist
                        .selected_playlist()
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
        if self
            .context
            .borrow()
            .playlist
            .selected_playlist()
            .is_some_and(|selected_playlist| selected_playlist.id == playlist.id)
        {
            self.context
                .borrow_mut()
                .playlist
                .set_selected_playlist(None);
        } else {
            self.context
                .borrow_mut()
                .playlist
                .set_selected_playlist(Some(playlist.clone()));
        }

        if let Err(err) = self
            .channels
            .database_command_tx
            .send(DatabaseCommand::QueryTracks(Some(playlist.clone())))
        {
            error!("Error when sending playlist track query: {}", err);
            return;
        }

        debug!(
            "Selected playlist: {:?}",
            self.context.borrow().playlist.selected_playlist()
        );
    }
}
