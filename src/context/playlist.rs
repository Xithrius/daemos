use crate::database::models::playlists::playlist::Playlist;

#[derive(Debug, Clone, Default)]
pub struct UIPlaylistContext {
    /// Which playlist is currently selected in the UI
    /// This is different compared to SelectedPlaylistContext as it's only what the user
    /// has selected in the UI, and not what's being used for autoplay
    selected: Option<Playlist>,
    autoplay: Option<Playlist>,
}

impl UIPlaylistContext {
    pub fn selected(&self) -> Option<Playlist> {
        self.selected.clone()
    }

    pub fn set_selected(&mut self, playlist: Option<Playlist>) {
        self.selected = playlist;
    }

    pub fn autoplay(&self) -> Option<Playlist> {
        self.autoplay.clone()
    }

    pub fn set_autoplay(&mut self, playlist: Option<Playlist>) {
        self.autoplay = playlist;
    }
}
