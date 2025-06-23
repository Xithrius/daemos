use std::collections::{BTreeMap, HashMap};

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone, Default)]
pub struct CacheContext {
    all_tracks: Vec<Track>,
    filtered_all_tracks: Option<Vec<Track>>,
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

    pub fn set_playlist_tracks(&mut self, playlist: Option<Playlist>, mut tracks: Vec<Track>) {
        tracks.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(playlist) = playlist {
            self.playlist_tracks.insert(playlist, tracks);
        } else {
            self.all_tracks = tracks;
        }
    }

    fn insert_sorted_to_all_tracks(&mut self, tracks: Vec<Track>) {
        for track in tracks {
            let pos = self
                .all_tracks
                .binary_search_by(|other_track| other_track.name.cmp(&track.name))
                .unwrap_or_else(|e| e);
            self.all_tracks.insert(pos, track);
        }
    }

    fn insert_sorted_to_playlist(playlist_tracks: &mut Vec<Track>, track: Track) {
        let pos = playlist_tracks
            .binary_search_by(|other_track| other_track.name.cmp(&track.name))
            .unwrap_or_else(|e| e);
        playlist_tracks.insert(pos, track);
    }

    pub fn add_tracks_to_playlist(&mut self, playlist: Option<&Playlist>, tracks: Vec<Track>) {
        // TODO: Make this more efficient, don't loop twice
        if let Some(playlist) = playlist {
            let playlist_tracks = self.playlist_tracks.entry(playlist.clone()).or_default();

            for track in &tracks {
                Self::insert_sorted_to_playlist(playlist_tracks, track.clone());
            }
        }

        self.insert_sorted_to_all_tracks(tracks);
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
        } else if let Some(filtered_all_tracks) = &self.filtered_all_tracks {
            filtered_all_tracks.as_slice()
        } else {
            self.all_tracks.as_slice()
        }
    }

    pub fn playlists(&self) -> impl Iterator<Item = &Playlist> {
        self.playlist_tracks.keys()
    }

    pub fn filter_with<F>(
        &mut self,
        playlist: &Option<Playlist>,
        predicate: F,
    ) -> Option<&Vec<Track>>
    where
        F: Fn(&Track) -> bool,
    {
        if let Some(playlist) = playlist {
            let source = self.playlist_tracks.get(playlist)?;

            let filtered = source.iter().filter(|&x| predicate(x)).cloned().collect();
            self.filtered_tracks.insert(playlist.clone(), filtered);
            self.filtered_tracks.get(playlist)
        } else {
            self.filtered_all_tracks = Some(
                self.all_tracks
                    .iter()
                    .filter(|&x| predicate(x))
                    .cloned()
                    .collect(),
            );
            self.filtered_all_tracks.as_ref()
        }
    }
}
