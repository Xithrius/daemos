#[derive(Debug, Clone, Default)]
pub struct ProcessingContext {
    processing_tracks: usize,
}

impl ProcessingContext {
    pub fn processing_tracks(&self) -> usize {
        self.processing_tracks
    }

    pub fn set_processing_tracks(&mut self, processing: usize) {
        self.processing_tracks = processing;
    }

    pub fn finished_processing_track(&mut self) {
        self.processing_tracks = self.processing_tracks.saturating_sub(1);
    }
}
