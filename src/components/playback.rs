use std::{ops::RangeInclusive, time::Duration};

use crossbeam::channel::Sender;
use egui::RichText;
use tracing::{debug, warn};

use crate::{
    config::core::CoreConfig,
    database::models::tracks::Track,
    playback::state::{PlayerCommand, PlayerEvent},
};

const DEFAULT_VOLUME_RANGE: RangeInclusive<f32> = 0.0..=1.0;

const PLAYBACK_BUTTON_FONT_SIZE: f32 = 22.5;

const SKIP_BACKWARD_SYMBOL: &str = "\u{23EE}"; // ⏮
const PLAY_SYMBOL: &str = "\u{25B6}"; // ▶
const PAUSE_SYMBOL: &str = "\u{23F8}"; // ⏸
const SKIP_FORWARD_SYMBOL: &str = "\u{23ED}"; // ⏭

#[derive(Debug, Clone)]
struct TrackState {
    track: Option<Track>,
    progress: Option<Duration>,
    playing: bool,
    volume: f32,
    last_volume_sent: f32,
}

impl TrackState {
    pub fn new(volume: f32) -> Self {
        Self {
            track: None,
            progress: None,
            playing: false,
            volume,
            last_volume_sent: volume,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackBar {
    player_cmd_tx: Sender<PlayerCommand>,
    track_state: TrackState,
}

impl PlaybackBar {
    pub fn new(config: &CoreConfig, player_cmd_tx: Sender<PlayerCommand>) -> Self {
        let track_state = TrackState::new(config.volume);

        Self {
            player_cmd_tx,
            track_state,
        }
    }

    fn handle_player_event(&mut self, player_event: PlayerEvent) {
        debug!("UI received event: {:?}", player_event);

        match player_event {
            PlayerEvent::TrackChanged(track) => {
                self.track_state.track = Some(track);
                self.track_state.playing = true;
            }
            PlayerEvent::TrackProgress(duration) => {
                self.track_state.progress = Some(duration);
            }
            PlayerEvent::CurrentVolume(volume) => {
                if self.track_state.volume != volume {
                    warn!(
                        "Volume desync detected UI track state does not equal player volume ({} != {})",
                        self.track_state.volume, volume
                    );
                    self.track_state.volume = volume;
                }
            }
        }
    }

    // fn ui_button()

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: Option<PlayerEvent>) {
        if let Some(event) = player_event {
            self.handle_player_event(event);
        }

        let button = |ui: &mut egui::Ui, text: &str| -> bool {
            ui.button(RichText::new(text).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
        };

        ui.horizontal(|ui| {
            if button(ui, SKIP_BACKWARD_SYMBOL) {
                let _ = self.player_cmd_tx.send(PlayerCommand::SkipPrevious);
            }

            // TODO: Make sure this is synced with the handler for player events
            let toggle_playing_button = if self.track_state.playing {
                PLAY_SYMBOL
            } else {
                PAUSE_SYMBOL
            };

            if button(ui, toggle_playing_button) {
                let _ = self.player_cmd_tx.send(PlayerCommand::Toggle);
                self.track_state.playing = !self.track_state.playing;
            }

            if button(ui, SKIP_FORWARD_SYMBOL) {
                let _ = self.player_cmd_tx.send(PlayerCommand::SkipNext);
            }

            ui.add(
                egui::Slider::new(&mut self.track_state.volume, DEFAULT_VOLUME_RANGE)
                    .text("Volume"),
            );

            let volume_dx = (self.track_state.volume - self.track_state.last_volume_sent).abs();

            if volume_dx > f32::EPSILON {
                let _ = self
                    .player_cmd_tx
                    .send(PlayerCommand::SetVolume(self.track_state.volume));

                self.track_state.last_volume_sent = self.track_state.volume;
            }
        });
    }
}
