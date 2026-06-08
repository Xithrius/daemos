use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ControlContext {
    pub volume: f32,
    /// The last volume received from the client.
    ///
    /// This attribute exists mainly to prevent the volume being re-set every frame.
    ///
    /// Instead waiting to see if we've changed from the last frame compared to [`ControlContext::volume`],
    /// and if we're different, then a new volume has been set.
    pub volume_snapshot: f32,

    /// Where the track was started from (should always be zero)
    pub progress_base: Option<Duration>,
    /// Where we have resumed the track from, given that we had previously paused it
    pub progress_snapshot: Option<Instant>,
    pub changing_track: bool,
}

impl Default for ControlContext {
    fn default() -> Self {
        Self {
            volume: 0.5,
            volume_snapshot: 0.5,
            progress_base: None,
            progress_snapshot: None,
            changing_track: false,
        }
    }
}

impl ControlContext {
    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn volume_mut(&mut self) -> &mut f32 {
        &mut self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn current_progress(&self) -> Option<Duration> {
        match (self.progress_base, self.progress_snapshot) {
            (Some(base), Some(ts)) => Some(base + Instant::now().duration_since(ts)),
            (Some(base), _) => Some(base),
            _ => None,
        }
    }

    pub fn set_progress(
        &mut self,
        progress_base: Option<Duration>,
        progress_timestamp: Option<Instant>,
    ) {
        self.progress_base = progress_base;
        self.progress_snapshot = progress_timestamp;
    }
}
