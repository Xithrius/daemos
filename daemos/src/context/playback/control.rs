use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ControlContext {
    pub volume: f32,
    pub last_volume_sent: f32,

    pub progress_base: Option<Duration>,
    pub progress_timestamp: Option<Instant>,
    pub changing_track: bool,
}

impl Default for ControlContext {
    fn default() -> Self {
        Self {
            volume: 0.5,
            last_volume_sent: 0.5,
            progress_base: None,
            progress_timestamp: None,
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
        match (self.progress_base, self.progress_timestamp) {
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
        self.progress_timestamp = progress_timestamp;
    }
}
