use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ProcessingContext {
    /// A map of optional playlist names to the remaining amount of tracks to be processed
    processing: HashMap<Option<String>, usize>,
}

impl ProcessingContext {
    /// Add a playlist name and track count to process to the processing map
    pub fn add(&mut self, playlist: Option<String>, track_count: usize) {
        self.processing.insert(playlist, track_count);
    }

    /// Saturating subtraction on the track count for a playlist
    /// If there are no tracks left for a key, remove the entry from the map
    pub fn decrement(&mut self, playlist: Option<String>) {
        let mut remove = false;

        if let Some(track_count) = self.processing.get_mut(&playlist) {
            *track_count = track_count.saturating_sub(1);
            if *track_count == 0 {
                remove = true;
            }
        }

        if remove {
            self.processing.remove(&playlist);
        }
    }

    /// How many tracks are left to process across all entries in the map
    pub fn total(&self) -> usize {
        self.processing.values().sum()
    }
}
