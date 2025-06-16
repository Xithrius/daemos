use std::{cell::RefCell, rc::Rc};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as SerdeError, Unexpected},
};

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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum ShuffleType {
    /// Select a random track that hasn't been played yet in the current session or playlist
    /// If all tracks have been played, select a random one to start with
    PseudoRandom,
    #[default]
    /// Pick any track regardless if it's been played before
    TrueRandom,
}

impl Serialize for AutoplayType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            AutoplayType::Iterative(PlayDirection::Forward) => "iterative_forward",
            AutoplayType::Iterative(PlayDirection::Backward) => "iterative_backward",
            AutoplayType::Shuffle(ShuffleType::PseudoRandom) => "pseudo_shuffle",
            AutoplayType::Shuffle(ShuffleType::TrueRandom) => "true_shuffle",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for AutoplayType {
    fn deserialize<D>(deserializer: D) -> Result<AutoplayType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "iterative_forward" => Ok(AutoplayType::Iterative(PlayDirection::Forward)),
            "iterative_backward" => Ok(AutoplayType::Iterative(PlayDirection::Backward)),
            "pseudo_shuffle" => Ok(AutoplayType::Shuffle(ShuffleType::PseudoRandom)),
            "true_shuffle" => Ok(AutoplayType::Shuffle(ShuffleType::TrueRandom)),
            _ => Err(D::Error::invalid_value(
                Unexpected::Str(s),
                &"Invalid autoplay type passed",
            )),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlaybackContext {
    select_new_track: bool,
    autoplay: AutoplayType,
    controlled_autoplay: Option<AutoplayType>,
}

impl PlaybackContext {
    pub fn select_new_track(&self) -> bool {
        self.select_new_track
    }

    pub fn set_select_new_track(&mut self, select_new_track: bool) {
        self.select_new_track = select_new_track;
    }

    pub fn set_incoming_track(&mut self, select_new_track: bool, autoplay: Option<AutoplayType>) {
        self.select_new_track = select_new_track;
        self.controlled_autoplay = autoplay;
    }

    pub fn consume_incoming_track(&mut self) -> Option<AutoplayType> {
        self.select_new_track = false;
        self.controlled_autoplay.take()
    }

    pub fn autoplay(&self) -> &AutoplayType {
        &self.autoplay
    }

    pub fn set_autoplay(&mut self, autoplay: AutoplayType) {
        self.autoplay = autoplay;
    }

    pub fn controlled_autoplay(&self) -> Option<AutoplayType> {
        self.controlled_autoplay.clone()
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
