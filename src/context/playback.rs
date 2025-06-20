use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use tracing::{debug, warn};

use crate::{
    context::AutoplayType,
    database::models::{playlists::playlist::Playlist, tracks::Track},
    playback::state::PlayerEvent,
};

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
        // self.last_volume_sent = self.volume;
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

#[derive(Debug, Clone)]
pub struct PlaylistState {
    _playlist: Playlist,
    tracks: Vec<Track>,
}

impl PlaylistState {
    pub fn new(playlist: Playlist, tracks: Vec<Track>) -> Self {
        Self {
            _playlist: playlist,
            tracks,
        }
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

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    tracks: Vec<Track>,
    // TODO: Convert to vector of indices
    // filtered_tracks: Vec<Track>,
    // track_ids: HashSet<Uuid>,

    playlists: Vec<Playlist>,
    // playlists: Vec<Playlist>,
    // playlist_ids: HashSet<Uuid>,
}


#[derive(Debug, Clone, Default)]
pub struct AutoplayContext {
    select_new_track: bool,
    autoplay: AutoplayType,
    controlled_autoplay: Option<AutoplayType>,
}

impl AutoplayContext {
    pub fn autoplay(&self) -> &AutoplayType {
        &self.autoplay
    }

    pub fn set_autoplay(&mut self, autoplay: AutoplayType) {
        self.autoplay = autoplay;
    }

    pub fn is_shuffle(&self) -> bool {
        matches!(self.autoplay, AutoplayType::Shuffle(_))
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlaybackContext {
    // TODO: Combine into single selected context?
    pub selected_track: Option<SelectedTrackContext>,
    pub selected_playlist: SelectedPlaylistContext,
    pub loaded: LoadedContext,
    pub control: ControlContext,
    pub autoplay: AutoplayContext,
}

impl PlaybackContext {
    pub fn set_selected_track(&mut self, track: Option<SelectedTrackContext>) {
        self.selected_track = track;
    }

    pub fn select_new_track(&self) -> bool {
        self.autoplay.select_new_track
    }

    pub fn set_select_new_track(&mut self, select_new_track: bool) {
        self.autoplay.select_new_track = select_new_track;
    }

    pub fn set_incoming_track(&mut self, select_new_track: bool, autoplay: Option<AutoplayType>) {
        self.autoplay.select_new_track = select_new_track;
        self.autoplay.controlled_autoplay = autoplay;
    }

    pub fn consume_incoming_track(&mut self) -> Option<AutoplayType> {
        self.autoplay.select_new_track = false;
        self.autoplay.controlled_autoplay.take()
    }

    pub fn consume_controlled_autoplay(&mut self) -> Option<AutoplayType> {
        self.autoplay.controlled_autoplay.take()
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
