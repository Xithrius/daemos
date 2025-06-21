use std::collections::HashSet;

use uuid::Uuid;

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone, Default)]
pub struct LoadedTracksContext {
    tracks: Vec<Track>,
    ids: HashSet<Uuid>,

    // TODO: Convert to vector of indices or UUIDs
    filtered: Vec<Track>,
}

impl LoadedTracksContext {
    pub fn set(&mut self, mut tracks: Vec<Track>) {
        self.ids = tracks.iter().map(|track| track.id).collect();

        tracks.sort_by(|track_a, track_b| track_a.name.cmp(&track_b.name));
        self.tracks = tracks;
    }

    pub fn add(&mut self, track: &Track) {
        if self.ids.insert(track.id) {
            // TODO: Insert as sorted
            self.tracks.push(track.clone());
            self.tracks
                .sort_by(|track_a, track_b| track_a.name.cmp(&track_b.name));
        }
    }

    pub fn remove(&mut self, id: &Uuid) -> bool {
        if self.ids.remove(id) {
            if let Some(pos) = self.tracks.iter().position(|t| &t.id == id) {
                self.tracks.remove(pos);
            }

            true
        } else {
            false
        }
    }

    pub fn set_filtered(&mut self, filtered: Vec<Track>) {
        self.filtered = filtered;
    }

    pub fn filtered(&self) -> &Vec<Track> {
        self.filtered.as_ref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoadedPlaylistsContext {
    playlists: Vec<Playlist>,
    ids: HashSet<Uuid>,
}

impl LoadedPlaylistsContext {
    pub fn set(&mut self, mut playlists: Vec<Playlist>) {
        self.ids = playlists.iter().map(|playlist| playlist.id).collect();

        playlists.sort_by(|playlist_a, playlist_b| playlist_a.name.cmp(&playlist_b.name));
        self.playlists = playlists;
    }

    pub fn add(&mut self, playlist: &Playlist) {
        if self.ids.insert(playlist.id) {
            self.playlists.push(playlist.clone());
            self.playlists
                .sort_by(|playlist_a, playlist_b| playlist_a.name.cmp(&playlist_b.name));
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    pub tracks: LoadedTracksContext,
    pub playlists: LoadedPlaylistsContext,
}

impl LoadedContext {
    pub fn tracks(&self) -> Vec<Track> {
        self.tracks.tracks.clone()
    }

    pub fn playlists(&self) -> Vec<Playlist> {
        self.playlists.playlists.clone()
    }
}
