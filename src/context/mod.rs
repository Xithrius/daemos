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
    /// All the tracks and playlists that have been loaded into memory from the database
    /// It is also possible to modify this loaded data in order to be up to date with database entries
    pub cache: CacheContext,
    pub playback: PlaybackContext,
    pub ui: UIContext,
    // TODO: Combine into UIContext
    pub ui_playlist: UIPlaylistContext,
    pub processing: ProcessingContext,
    pub latency: LatencyContext,
}

pub type SharedContext = Rc<RefCell<Context>>;
