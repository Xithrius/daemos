use std::collections::BTreeSet;

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone)]
pub struct SelectedTrackContext {
    pub track: Track,
    pub index: usize,
    pub playing: bool,
}

impl SelectedTrackContext {
    pub fn new(track: Track, index: usize, playing: bool) -> Self {
        Self {
            track,
            index,
            playing,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistState {
    playlist: Playlist,
    tracks: Vec<Track>,
}

impl PlaylistState {
    pub fn new(playlist: Playlist, tracks: Vec<Track>) -> Self {
        Self {
            playlist,
            tracks,
        }
    }

    pub fn playlist(&self) -> Playlist {
        self.playlist.clone()
    }

    pub fn tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SelectedPlaylistContext {
    playlist: Option<PlaylistState>,
    // TODO: Change to UUIDs
    played_tracks: BTreeSet<usize>,
}

impl SelectedPlaylistContext {
    pub fn playlist(&self) -> Option<PlaylistState> {
        self.playlist.clone()
    }

    pub fn set_playlist(&mut self, playlist: Option<PlaylistState>) {
        self.playlist = playlist;
    }

    pub fn played_tracks(&self) -> BTreeSet<usize> {
        self.played_tracks.clone()
    }

    pub fn add_played_track(&mut self, index: usize) -> bool {
        self.played_tracks.insert(index)
    }

    pub fn clear_played_tracks(&mut self) {
        self.played_tracks.clear();
    }
}
