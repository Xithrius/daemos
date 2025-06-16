use std::{cell::RefCell, rc::Rc};

pub mod autoplay;
pub use autoplay::{AutoplayType, PlayDirection, ShuffleType};

pub mod playback;
pub use playback::PlaybackContext;

pub mod ui;
pub use ui::UIContext;

pub mod playlist;
pub use playlist::PlaylistContext;

pub mod processing;
pub use processing::ProcessingContext;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub playback: PlaybackContext,
    pub ui: UIContext,
    pub playlist: PlaylistContext,
    pub processing: ProcessingContext,
}

pub type SharedContext = Rc<RefCell<Context>>;
