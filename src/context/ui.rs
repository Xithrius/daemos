#[derive(Debug, Clone, Default)]
pub struct UIContext {
    visible_settings: bool,
    visible_debug: bool,
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

    pub fn visible_debug(&self) -> bool {
        self.visible_debug
    }

    pub fn visible_debug_mut(&mut self) -> &mut bool {
        &mut self.visible_debug
    }

    pub fn set_visible_debug(&mut self, visibility: bool) {
        self.visible_debug = visibility;
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
