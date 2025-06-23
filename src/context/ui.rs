use crate::context::UIPlaylistContext;

#[derive(Debug, Clone, Default)]
pub struct UIVisibilityContext {
    create_playlist_modal: bool,
    settings_popup: bool,
    general_debug: bool,
    debug_playback: bool,
}

#[derive(Debug, Clone, Default)]
pub struct UIContext {
    pub playlist: UIPlaylistContext,
    pub visibility: UIVisibilityContext,
}

impl UIContext {
    pub fn visible_settings(&self) -> bool {
        self.visibility.settings_popup
    }

    pub fn visible_settings_mut(&mut self) -> &mut bool {
        &mut self.visibility.settings_popup
    }

    pub fn set_visible_settings(&mut self, visibility: bool) {
        self.visibility.settings_popup = visibility;
    }

    pub fn visible_debug(&self) -> bool {
        self.visibility.general_debug
    }

    pub fn visible_debug_mut(&mut self) -> &mut bool {
        &mut self.visibility.general_debug
    }

    pub fn set_visible_debug(&mut self, visibility: bool) {
        self.visibility.general_debug = visibility;
    }

    pub fn toggle_settings(&mut self) {
        self.visibility.settings_popup = !self.visibility.settings_popup;
    }

    pub fn debug_playback(&self) -> bool {
        self.visibility.debug_playback
    }

    pub fn debug_playback_mut(&mut self) -> &mut bool {
        &mut self.visibility.debug_playback
    }

    pub fn set_debug_playback(&mut self, visibility: bool) {
        self.visibility.debug_playback = visibility;
    }

    pub fn visible_playlist_modal(&self) -> bool {
        self.visibility.create_playlist_modal
    }

    pub fn visible_playlist_modal_mut(&mut self) -> &mut bool {
        &mut self.visibility.create_playlist_modal
    }

    pub fn set_visible_playlist_modal(&mut self, visibility: bool) {
        self.visibility.create_playlist_modal = visibility;
    }
}
