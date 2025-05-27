use std::rc::Rc;

use egui::ahash::HashSet;
use uuid::Uuid;

use crate::{
    components::ComponentChannels,
    context::SharedContext,
    database::{connection::DatabaseCommand, models::playlists::playlist::Playlist},
};

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
            .send(DatabaseCommand::QueryAllPlaylists);

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

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("I am a table");
    }
}
