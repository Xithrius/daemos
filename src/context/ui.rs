use crate::context::UIPlaylistContext;

#[derive(Debug, Clone, Default)]
pub struct UIVisibilityContext {
    create_playlist_modal: bool,
    settings_popup: bool,
    general_debug: bool,
    playback_debug: bool,
}

impl UIVisibilityContext {
    pub fn settings(&self) -> bool {
        self.settings_popup
    }

    pub fn settings_mut(&mut self) -> &mut bool {
        &mut self.settings_popup
    }

    pub fn set_settings(&mut self, visibility: bool) {
        self.settings_popup = visibility;
    }

    pub fn toggle_settings(&mut self) {
        self.settings_popup = !self.settings_popup;
    }

    pub fn debug(&self) -> bool {
        self.general_debug
    }

    pub fn debug_mut(&mut self) -> &mut bool {
        &mut self.general_debug
    }

    pub fn set_debug(&mut self, visibility: bool) {
        self.general_debug = visibility;
    }

    pub fn debug_playback(&self) -> bool {
        self.playback_debug
    }

    pub fn debug_playback_mut(&mut self) -> &mut bool {
        &mut self.playback_debug
    }

    pub fn set_debug_playback(&mut self, visibility: bool) {
        self.playback_debug = visibility;
    }

    pub fn playlist_modal(&self) -> bool {
        self.create_playlist_modal
    }

    pub fn playlist_modal_mut(&mut self) -> &mut bool {
        &mut self.create_playlist_modal
    }

    pub fn set_playlist_modal(&mut self, visibility: bool) {
        self.create_playlist_modal = visibility;
    }
}

#[derive(Debug, Clone, Default)]
pub struct UIContext {
    pub playlist: UIPlaylistContext,
    pub visibility: UIVisibilityContext,
}
