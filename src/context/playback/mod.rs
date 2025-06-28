pub mod autoplay;
pub use autoplay::{AutoplayContext, AutoplayType, PlayDirection, ShuffleType};

pub mod control;
pub use control::ControlContext;

pub mod selected;
use std::time::{Duration, Instant};

pub use selected::{PlaylistState, SelectedPlaylistContext, SelectedTrackContext};
use tracing::{debug, warn};

use super::{CacheContext, SharedContext};
use crate::playback::state::PlayerEvent;

#[derive(Debug, Clone, Default)]
pub struct PlaybackContext {
    // TODO: Combine into single selected context?
    /// Which track is currently selected for playback
    pub selected_track: Option<SelectedTrackContext>,
    /// What playlist currently is selected for tracks to be autoplayed in
    pub selected_playlist: SelectedPlaylistContext,
    pub control: ControlContext,
    pub autoplay: AutoplayContext,
}

impl PlaybackContext {
    pub fn select_track(&mut self, track: Option<SelectedTrackContext>) {
        self.selected_track = track;
    }

    pub fn select_playlist(&mut self, playlist: SelectedPlaylistContext) {
        self.selected_playlist = playlist;
    }

    pub fn handle_player_event(&mut self, player_event: PlayerEvent) {
        debug!("Handling playback event: {:?}", player_event);

        match player_event {
            PlayerEvent::TrackChanged(track) => {
                if let Some(track_state) = self.selected_track.as_mut() {
                    track_state.track = track;
                    track_state.playing = true;
                }
                self.control.progress_base = Some(Duration::ZERO);
                self.control.progress_timestamp = Some(Instant::now());
                self.control.changing_track = false;
            }
            PlayerEvent::TrackPlayingStatus(playing) => {
                // If we are pausing, freeze current progress
                if !playing
                    && self
                        .selected_track
                        .as_ref()
                        .is_some_and(|track| track.playing)
                {
                    // Capture how much time has passed
                    if let (Some(base), Some(ts)) =
                        (self.control.progress_base, self.control.progress_timestamp)
                    {
                        let elapsed = Instant::now().duration_since(ts);
                        self.control.progress_base = Some(base + elapsed);
                        self.control.progress_timestamp = None;
                    }
                }

                // If we are resuming, set the timestamp so progress resumes from base
                if playing
                    && !self
                        .selected_track
                        .as_ref()
                        .is_some_and(|track| track.playing)
                {
                    self.control.progress_timestamp = Some(Instant::now());
                }

                if let Some(track) = self.selected_track.as_mut() {
                    track.playing = playing;
                }
            }
            PlayerEvent::TrackProgress(duration) => {
                // If duration is not synced properly, do it here
                if self
                    .control
                    .progress_base
                    .is_some_and(|progress_base| progress_base < duration)
                {
                    warn!(
                        "Track progress desync detected, setting progress base to received player position"
                    );
                    self.control.progress_base = Some(duration);
                    self.control.progress_timestamp = Some(Instant::now());
                }
            }
            PlayerEvent::CurrentVolume(volume) => {
                if self.control.volume != volume {
                    warn!(
                        "Volume desync detected, UI track state does not equal player volume ({} != {})",
                        self.control.volume, volume
                    );
                    self.control.volume = volume;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackContextAccess {
    context: SharedContext,
}

impl PlaybackContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_playback<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&PlaybackContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.playback)
    }

    pub fn with_playback_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut PlaybackContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.playback)
    }

    pub fn with_cache<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&CacheContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.cache)
    }

    pub fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut CacheContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.cache)
    }
}
