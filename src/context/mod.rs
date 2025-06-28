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

pub mod cache;
pub use cache::CacheContext;

#[derive(Debug, Clone, Default)]
pub struct Context {
    /// All the tracks and playlists that have been loaded into memory from the database.
    /// It is also possible to modify this loaded data in order to be up to date with database entries.
    pub cache: CacheContext,
    /// Selected items that are used for currently playing tracks within a playlist,
    /// configured shuffle direction and type from autoplay, and the state of progression
    /// through the playing track along with volume level.
    pub playback: PlaybackContext,
    /// What's currently being shown from the perspective of the UI.
    pub ui: UIContext,
    /// Background jobs that are actively processing.
    pub processing: ProcessingContext,
    /// Main thread UI render latency data.
    // TODO: Go to nested methods that exist within `App::update`
    pub latency: LatencyContext,
}

pub type SharedContext = Rc<RefCell<Context>>;

// Component-specific context accessors
#[derive(Debug, Clone)]
pub struct PlaybackContextAccess {
    context: SharedContext,
}

impl PlaybackContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_playback<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&PlaybackContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.playback)
    }

    pub fn with_playback_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut PlaybackContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.playback)
    }

    pub fn with_cache<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&CacheContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.cache)
    }

    pub fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut CacheContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.cache)
    }
}

#[derive(Debug, Clone)]
pub struct UIContextAccess {
    context: SharedContext,
}

impl UIContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_ui<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&UIContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.ui)
    }

    pub fn with_ui_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut UIContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.ui)
    }
}

#[derive(Debug, Clone)]
pub struct TableContextAccess {
    context: SharedContext,
}

impl TableContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_cache<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&CacheContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.cache)
    }

    pub fn with_cache_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut CacheContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.cache)
    }

    pub fn with_playback<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&PlaybackContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.playback)
    }

    pub fn with_playback_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut PlaybackContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.playback)
    }

    pub fn with_ui<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&UIContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.ui)
    }

    pub fn with_ui_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut UIContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.ui)
    }
}

// Settings-specific context
#[derive(Debug, Clone)]
pub struct SettingsContextAccess {
    context: SharedContext,
}

impl SettingsContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_ui<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&UIContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.ui)
    }

    pub fn with_ui_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut UIContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.ui)
    }
}

// Menu-specific context
#[derive(Debug, Clone)]
pub struct MenuContextAccess {
    context: SharedContext,
}

impl MenuContextAccess {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn with_ui<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&UIContext) -> R,
    {
        let context = self.context.borrow();
        f(&context.ui)
    }

    pub fn with_ui_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut UIContext) -> R,
    {
        let mut context = self.context.borrow_mut();
        f(&mut context.ui)
    }
}
