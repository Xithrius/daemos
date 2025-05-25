#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Default)]
pub enum ShuffleType {
    /// Play the next track after this one in the track table
    /// If the end has been reached, loop back to the first track
    #[default]
    AutoPlay,
    /// Select a random track that hasn't been played yet in the current session
    /// If all tracks have been played, select a random one to start with
    /// TODO: In the first half of played ones? We don't want the chance to replay a recent one
    PseudoRandom,
    /// Uses a random number generator on the loaded list of tracks, repeats are allowed
    TrueRandom,
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    select_previous_track: bool,
    select_new_track: bool,
    shuffle: ShuffleType,
    visible_settings: bool,
    debug_playback: bool,
}

impl Context {
    pub fn select_previous_track(&self) -> bool {
        self.select_previous_track
    }

    pub fn select_next_track(&self) -> bool {
        self.select_new_track
    }

    pub fn shuffle(&self) -> &ShuffleType {
        &self.shuffle
    }

    pub fn set_select_new_track(&mut self, new_track: bool) {
        self.select_new_track = new_track;
    }

    pub fn set_shuffle(&mut self, shuffle: ShuffleType) {
        self.shuffle = shuffle;
    }

    pub fn visible_settings_mut(&mut self) -> &mut bool {
        &mut self.visible_settings
    }

    pub fn visible_settings(&self) -> bool {
        self.visible_settings
    }

    pub fn set_visible_settings(&mut self, visibility: bool) {
        self.visible_settings = visibility;
    }

    pub fn debug_playback_mut(&mut self) -> &mut bool {
        &mut self.debug_playback
    }

    pub fn debug_playback(&self) -> bool {
        self.debug_playback
    }

    pub fn set_debug_playback(&mut self, visibility: bool) {
        self.debug_playback = visibility;
    }
}

pub type SharedContext = Rc<RefCell<Context>>;
