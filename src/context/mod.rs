use std::{cell::RefCell, rc::Rc};

pub mod playback;
pub use playback::*;

pub mod ui;
pub use ui::UIContext;

pub mod playlist;
pub use playlist::UIPlaylistContext;

pub mod processing;
pub use processing::ProcessingContext;

pub mod latency;
pub use latency::LatencyContext;

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub playback: PlaybackContext,
    pub ui: UIContext,
    // TODO: Combine into UIContext
    pub ui_playlist: UIPlaylistContext,
    pub processing: ProcessingContext,
    pub latency: LatencyContext,
}

pub type SharedContext = Rc<RefCell<Context>>;
