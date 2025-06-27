use std::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as SerdeError, Unexpected},
};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub enum PlayDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
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

impl fmt::Display for AutoplayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            AutoplayType::Iterative(play_direction) => match play_direction {
                PlayDirection::Forward => "Forward",
                PlayDirection::Backward => "Reverse",
            },
            AutoplayType::Shuffle(shuffle_type) => match shuffle_type {
                ShuffleType::PseudoRandom => "Shuffle (Pseudo Random)",
                ShuffleType::TrueRandom => "Shuffle (True Random)",
            },
        };

        write!(f, "{label}")
    }
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

    pub fn consume_controlled(&mut self) -> Option<AutoplayType> {
        self.controlled_autoplay.take()
    }
}
