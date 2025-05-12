use crossbeam::channel::Sender;

use crate::playback::state::PlayerCommand;

#[derive(Debug, Clone)]
pub struct VolumeBar {
    tx: Sender<PlayerCommand>,
}

impl VolumeBar {
    pub fn new(tx: Sender<PlayerCommand>) -> Self {
        Self { tx }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {}
}
