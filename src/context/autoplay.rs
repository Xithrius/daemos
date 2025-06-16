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

        write!(f, "{}", label)
    }
}
