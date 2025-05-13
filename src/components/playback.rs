use crossbeam::channel::Sender;
use egui::RichText;

use crate::playback::state::PlayerCommand;

const PLAYBACK_BUTTON_FONT_SIZE: f32 = 22.5;

const SKIP_BACKWARD_SYMBOL: &str = "\u{23EE}"; // ⏮
const PLAY_SYMBOL: &str = "\u{25B6}"; // ▶
const PAUSE_SYMBOL: &str = "\u{23F8}"; // ⏸
const SKIP_FORWARD_SYMBOL: &str = "\u{23ED}"; // ⏭

#[derive(Debug, Clone)]
pub struct PlaybackBar {
    tx: Sender<PlayerCommand>,
}

impl PlaybackBar {
    pub fn new(tx: Sender<PlayerCommand>) -> Self {
        Self { tx }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let button = |ui: &mut egui::Ui, text: &str| -> bool {
            ui.button(RichText::new(text).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
        };

        ui.horizontal(|ui| {
            if button(ui, SKIP_BACKWARD_SYMBOL) {
                let _ = self.tx.send(PlayerCommand::SkipPrevious);
            }
            if button(ui, PLAY_SYMBOL) {
                let _ = self.tx.send(PlayerCommand::Play);
            }
            if button(ui, PAUSE_SYMBOL) {
                let _ = self.tx.send(PlayerCommand::Pause);
            }
            if button(ui, SKIP_FORWARD_SYMBOL) {
                let _ = self.tx.send(PlayerCommand::SkipNext);
            }
        });
    }
}
