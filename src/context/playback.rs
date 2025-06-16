use crate::context::AutoplayType;

#[derive(Debug, Clone, Default)]
pub struct PlaybackContext {
    select_new_track: bool,
    autoplay: AutoplayType,
    controlled_autoplay: Option<AutoplayType>,
}

impl PlaybackContext {
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

    pub fn autoplay(&self) -> &AutoplayType {
        &self.autoplay
    }

    pub fn set_autoplay(&mut self, autoplay: AutoplayType) {
        self.autoplay = autoplay;
    }

    pub fn controlled_autoplay(&self) -> Option<AutoplayType> {
        self.controlled_autoplay.clone()
    }
}
