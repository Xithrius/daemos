use std::collections::{BTreeMap, HashMap};

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone, Default)]
pub struct CacheContext {
    all_tracks: Vec<Track>,
    filtered_all_tracks: Vec<Track>,
    playlist_tracks: BTreeMap<Playlist, Vec<Track>>,
    filtered_tracks: HashMap<Playlist, Vec<Track>>,
}

impl CacheContext {
    pub fn playlist_tracks(&self, playlist: Option<&Playlist>) -> Option<&Vec<Track>> {
        if let Some(playlist) = playlist {
            self.playlist_tracks.get(playlist)
        } else {
            Some(&self.all_tracks)
        }
    }

    pub fn set_playlist_tracks(&mut self, playlist: Option<Playlist>, tracks: Vec<Track>) {
        if let Some(playlist) = playlist {
            self.playlist_tracks.insert(playlist, tracks);
        } else {
            self.all_tracks = tracks;
        }
    }

    pub fn add_tracks_to_playlist(&mut self, playlist: Option<&Playlist>, tracks: Vec<Track>) {
        if let Some(playlist) = playlist {
            self.playlist_tracks
                .entry(playlist.clone())
                .or_default()
                .extend(tracks);
        } else {
            self.all_tracks.extend(tracks);
        }
    }

    pub fn add_empty_playlist(&mut self, playlist: &Playlist) {
        self.playlist_tracks.insert(playlist.clone(), Vec::new());
    }

    pub fn filtered_tracks(&self, playlist: Option<&Playlist>) -> &[Track] {
        if let Some(playlist) = playlist {
            if let Some(filtered) = self.filtered_tracks.get(playlist) {
                filtered.as_slice()
            } else if let Some(original) = self.playlist_tracks.get(playlist) {
                original.as_slice()
            } else {
                &[]
            }
        } else {
            self.filtered_all_tracks.as_slice()
        }
    }

    pub fn playlists(&self) -> impl Iterator<Item = &Playlist> {
        self.playlist_tracks.keys()
    }

    pub fn filter_with<F>(&mut self, playlist: &Playlist, predicate: F) -> Option<&Vec<Track>>
    where
        F: Fn(&Track) -> bool,
    {
        let source = self.playlist_tracks.get(playlist)?;

        let filtered = source.iter().filter(|&x| predicate(x)).cloned().collect();
        self.filtered_tracks.insert(playlist.clone(), filtered);
        self.filtered_tracks.get(playlist)
    }
}
