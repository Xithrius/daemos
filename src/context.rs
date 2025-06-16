use std::{cell::RefCell, rc::Rc};

use crate::database::models::playlists::playlist::Playlist;

#[derive(Debug, Clone)]
pub enum AutoplayType {
    /// Play the next (or previous) track in the track list
    /// If the end has been reached, loop back around to the other side
    Iterative(PlayDirection),
    Shuffle(ShuffleType),
}

impl Default for AutoplayType {
    fn default() -> Self {
        Self::Iterative(PlayDirection::Forward)
    }
}

#[derive(Debug, Clone, Default)]
pub enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Debug, Clone, Default)]
pub enum ShuffleType {
    /// Select a random track that hasn't been played yet in the current session
    /// If all tracks have been played, select a random one to start with
    // TODO: In the first half of played ones? We don't want the chance to replay a recent one
    // TODO: Make this the default once implementation is done
    PseudoRandom,
    #[default]
    /// Uses a random number generator on the loaded list of tracks, repeats are allowed
    TrueRandom,
}

#[derive(Debug, Clone, Default)]
pub struct PlaybackContext {
    select_new_track: bool,
    autoplay: AutoplayType,
}

impl PlaybackContext {
    pub fn select_new_track(&self) -> bool {
        self.select_new_track.clone()
    }

    pub fn set_select_new_track(&mut self, select_new_track: bool) {
        self.select_new_track = select_new_track;
    }

    pub fn set_incoming_track(&mut self, select_new_track: bool, autoplay: AutoplayType) {
        self.select_new_track = select_new_track;
        self.autoplay = autoplay;
    }

    pub fn autoplay(&self) -> &AutoplayType {
        &self.autoplay
    }

    pub fn set_autoplay(&mut self, autoplay: AutoplayType) {
        self.autoplay = autoplay;
    }
}

#[derive(Debug, Clone, Default)]
pub struct UIContext {
    visible_settings: bool,
    debug_playback: bool,
    visible_playlist_modal: bool,
}

impl UIContext {
    pub fn visible_settings(&self) -> bool {
        self.visible_settings
    }

    pub fn visible_settings_mut(&mut self) -> &mut bool {
        &mut self.visible_settings
    }

    pub fn set_visible_settings(&mut self, visibility: bool) {
        self.visible_settings = visibility;
    }

    pub fn toggle_settings(&mut self) {
        self.visible_settings = !self.visible_settings;
    }

    pub fn debug_playback(&self) -> bool {
        self.debug_playback
    }

    pub fn debug_playback_mut(&mut self) -> &mut bool {
        &mut self.debug_playback
    }

    pub fn set_debug_playback(&mut self, visibility: bool) {
        self.debug_playback = visibility;
    }

    pub fn visible_playlist_modal(&self) -> bool {
        self.visible_playlist_modal
    }

    pub fn visible_playlist_modal_mut(&mut self) -> &mut bool {
        &mut self.visible_playlist_modal
    }

    pub fn set_visible_playlist_modal(&mut self, visibility: bool) {
        self.visible_playlist_modal = visibility;
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlaylistContext {
    selected: Option<Playlist>,
    autoplay: Option<Playlist>,
}

impl PlaylistContext {
    pub fn selected(&self) -> Option<Playlist> {
        self.selected.clone()
    }

    pub fn set_selected(&mut self, playlist: Option<Playlist>) {
        self.selected = playlist;
    }

    pub fn autoplay(&self) -> Option<Playlist> {
        self.autoplay.clone()
    }

    pub fn set_autoplay(&mut self, playlist: Option<Playlist>) {
        self.autoplay = playlist;
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub playback: PlaybackContext,
    pub ui: UIContext,
    pub playlist: PlaylistContext,
    pub processing: ProcessingContext,
}

pub type SharedContext = Rc<RefCell<Context>>;
