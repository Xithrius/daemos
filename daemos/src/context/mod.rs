use std::{cell::RefCell, rc::Rc};

pub mod playback;
pub use playback::*;

pub mod ui;
pub use ui::UIContext;

pub mod playlist;
pub use playlist::UIPlaylistContext;

pub mod processing;
pub use processing::ProcessingContext;

pub mod performance;
pub use performance::PerformanceMetricsContext;

pub mod storage;
pub use storage::StorageContext;

#[derive(Debug, Clone, Default)]
pub struct Context {
    /// All the tracks and playlists that have been loaded into memory from the database.
    /// It is also possible to modify this loaded data in order to be up to date with database entries.
    pub storage: StorageContext,
    /// Selected items that are used for currently playing tracks within a playlist,
    /// configured shuffle direction and type from autoplay, and the state of progression
    /// through the playing track along with volume level.
    pub playback: PlaybackContext,
    /// What's currently being shown from the perspective of the UI.
    pub ui: UIContext,
    /// Background jobs that are actively processing.
    pub processing: ProcessingContext,
    /// Performance-related metrics data (latency, FPS, etc.).
    pub performance_metrics: PerformanceMetricsContext,
}

pub type SharedContext = Rc<RefCell<Context>>;
