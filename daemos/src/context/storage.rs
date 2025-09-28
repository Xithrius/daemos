use std::collections::{BTreeMap, HashMap};

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone, Default)]
pub struct StorageContext {
    /// Global playlist for tracks. Tracks that belong to any amount of playlist(s) are included here.
    all_tracks: Vec<Track>,
    /// Filtered version of [`Self::all_tracks`] based on a search input.
    filtered_all_tracks: Option<Vec<Track>>,
    /// All playlists and the respective tracks loaded from the database at startup.
    /// Using a BTreeMap since this is used directly in the UI and we don't want the possibility of
    /// the playlists list looking different each startup.
    playlist_tracks: BTreeMap<Playlist, Vec<Track>>,
    /// Filtered tracks based on a search.
    /// Hashmap is used due to the playlists not being displayed directly in the UI, while [`Self::playlist_tracks`] is.
    /// Probably will switch to BTreeMap in the future once searching for playlists is implemented.
    filtered_playlist_tracks: HashMap<Playlist, Vec<Track>>,
}

impl StorageContext {
    /// Gets tracks from a playlist in [`Self::playlist_tracks`],
    /// if none is selected then [`Self::all_tracks`] is returned.
    pub fn get_playlist_tracks(&self, playlist: Option<&Playlist>) -> Option<&Vec<Track>> {
        if let Some(playlist) = playlist {
            self.playlist_tracks.get(playlist)
        } else {
            Some(&self.all_tracks)
        }
    }

    /// Get an iterator of all playlist keys from the unfiltered [`Self::playlist_tracks`] playlist tracks.
    pub fn playlists(&self) -> impl Iterator<Item = &Playlist> {
        self.playlist_tracks.keys()
    }

    /// Sets a playlist's tracks to the passed track vector after sorting it based on comparing track paths.
    /// If no playlist is passed, then all tracks are set to the sorted track vector.
    pub fn set_playlist_tracks(&mut self, playlist: Option<Playlist>, mut tracks: Vec<Track>) {
        tracks.sort_by(|a, b| a.path.cmp(&b.path));

        if let Some(playlist) = playlist {
            self.playlist_tracks.insert(playlist, tracks);
        } else {
            self.all_tracks = tracks;
        }
    }

    /// Sorted in-place insertion of a track to [`Self::all_tracks`], comparing by path on insert.
    fn insert_sorted_to_all_tracks(&mut self, track: Track) {
        let pos = self
            .all_tracks
            .binary_search_by(|other_track| other_track.path.cmp(&track.path))
            .unwrap_or_else(|e| e);
        self.all_tracks.insert(pos, track);
    }

    /// Sorted in-place insertion of a track to a playlist's vector
    /// in [`Self::playlist_tracks`], comparing by path on insert.
    fn insert_sorted_to_playlist(playlist_tracks: &mut Vec<Track>, track: Track) {
        let pos = playlist_tracks
            .binary_search_by(|other_track| other_track.path.cmp(&track.path))
            .unwrap_or_else(|e| e);
        playlist_tracks.insert(pos, track);
    }

    /// Extends the track list in a playlist of [`Self::playlist_tracks`] with the passed through tracks.
    ///
    /// If the playlist is None, then all tracks is inserted to.
    /// When the playlist is some but doesn't exist in the [`Self::playlist_tracks`] attribute,
    /// a new vector is created and tracks are added to it.
    pub fn add_tracks_to_playlist(&mut self, playlist: Option<&Playlist>, tracks: Vec<Track>) {
        for track in tracks {
            if let Some(playlist) = playlist {
                let playlist_tracks = self.playlist_tracks.entry(playlist.clone()).or_default();
                Self::insert_sorted_to_playlist(playlist_tracks, track.clone());
            }
            self.insert_sorted_to_all_tracks(track);
        }
    }

    /// Create a playlist in [`Self::playlist_tracks`] with an empty vector of tracks.
    pub fn add_empty_playlist(&mut self, playlist: &Playlist) {
        self.playlist_tracks.insert(playlist.clone(), Vec::new());
    }

    /// Returns a playlist's tracks, checking filtered track attributes first, eventually narrowing down to the global tracks playlist [`Self::all_tracks`].
    ///
    /// If playlist is [`Some`], [`Self::filtered_playlist_tracks`] is checked first to see if it contains the playlist.
    /// If so, the tracks that key referred to are returned. Otherwise [`Self::playlist_tracks`] is checked. If that still doesn't contain
    /// the playlist then it is assumed no filtering was applied and no tracks exist for the playlist, and an empty slice is returned.
    /// If the playlist is [`None`], [`Self::filtered_all_tracks`] is checked to see if it is [`Some`], otherwise return [`Self::all_tracks`].
    pub fn filtered_tracks(&self, playlist: Option<&Playlist>) -> &[Track] {
        if let Some(playlist) = playlist {
            if let Some(filtered) = self.filtered_playlist_tracks.get(playlist) {
                filtered.as_slice()
            } else if let Some(playlist_tracks) = self.playlist_tracks.get(playlist) {
                playlist_tracks.as_slice()
            } else {
                &[]
            }
        } else if let Some(filtered_all_tracks) = &self.filtered_all_tracks {
            filtered_all_tracks.as_slice()
        } else {
            self.all_tracks.as_slice()
        }
    }

    /// Apply a filter function on a playlist's tracks and create a new track vector from it.
    ///
    /// If the playlist is [`Some`], the predicate filters the playlist in [`Self::playlist_tracks`] to see what is valid in it.
    /// Once the filtered vector is created, the playlist is set to the new vector in [`Self::filtered_playlist_tracks`].
    /// If playlist is [`None`], the same filtering logic is instead applied to [`Self::all_tracks`],
    /// and any filtered tracks is set on [`Self::filtered_all_tracks`].
    ///
    /// An example of a predicate filtering track names down to ones that only contain "foo":
    /// ```
    /// use daemos::{context::StorageContext, database::models::tracks::Track};
    ///
    /// let mut storage = StorageContext::default();
    /// let tracks = vec![
    ///     Track { name: "foo 1".to_string(), ..Default::default() },
    ///     Track { name: "bar 2".to_string(), ..Default::default() },
    /// ];
    /// storage.set_playlist_tracks(None, tracks);
    ///
    /// let result = storage.filter_with(&None, |track| track.name.contains("foo"));
    /// assert_eq!(result.unwrap().len(), 1);
    /// assert_eq!(result.unwrap()[0].name, "foo 1");
    /// ```
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
            self.filtered_playlist_tracks
                .insert(playlist.clone(), filtered);
            self.filtered_playlist_tracks.get(playlist)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_with_global_tracks() {
        let mut storage = StorageContext::default();

        let tracks = vec![
            Track {
                name: "foo 1".to_string(),
                ..Default::default()
            },
            Track {
                name: "bar 2".to_string(),
                ..Default::default()
            },
        ];
        storage.set_playlist_tracks(None, tracks);

        let result = storage.filter_with(&None, |track| track.name.contains("foo"));
        assert_eq!(result.unwrap().len(), 1);
        assert_eq!(result.unwrap()[0].name, "foo 1");
    }

    #[test]
    fn test_filter_with_playlist_tracks() {
        let mut storage = StorageContext::default();

        let tracks = vec![
            Track {
                name: "song 1".to_string(),
                ..Default::default()
            },
            Track {
                name: "song 2".to_string(),
                ..Default::default()
            },
        ];
        let playlist = Playlist {
            name: "test".to_string(),
            ..Default::default()
        };
        storage.set_playlist_tracks(Some(playlist.clone()), tracks);

        let result = storage.filter_with(&Some(playlist), |track| track.name.contains("song 1"));
        assert_eq!(result.unwrap().len(), 1);
        assert_eq!(result.unwrap()[0].name, "song 1");
    }
}
