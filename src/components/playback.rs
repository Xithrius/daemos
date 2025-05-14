use std::{
    ops::RangeInclusive,
    time::{Duration, Instant},
};

use crossbeam::channel::Sender;
use egui::{Align, Layout, RichText, Vec2};
use tracing::{debug, warn};

use crate::{
    config::core::CoreConfig,
    database::models::tracks::Track,
    playback::state::{PlayerCommand, PlayerEvent},
    utils::formatting::human_duration,
};

const DEFAULT_VOLUME_RANGE: RangeInclusive<f32> = 0.0..=1.0;
const PLAYBACK_BUTTON_FONT_SIZE: f32 = 22.5;
const SEEKBAR_WIDTH: f32 = 500.0;

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
    last_progress_poll: Option<Instant>,
}

impl TrackState {
    pub fn new(volume: f32) -> Self {
        Self {
            track: None,
            progress: None,
            playing: true,
            volume,
            last_volume_sent: volume,
            last_progress_poll: None,
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
        debug!(
            "Playback bar UI component received event: {:?}",
            player_event
        );

        match player_event {
            PlayerEvent::TrackChanged(track) => {
                self.track_state.track = Some(track);
                self.track_state.playing = true;
            }
            PlayerEvent::TrackPlayingStatus(playing) => {
                self.track_state.playing = playing;
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

    pub fn ui_playback_controls(&mut self, ui: &mut egui::Ui) {
        let button = |ui: &mut egui::Ui, text: &str| -> bool {
            ui.button(RichText::new(text).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
        };

        if button(ui, SKIP_BACKWARD_SYMBOL) {
            let _ = self.player_cmd_tx.send(PlayerCommand::SkipPrevious);
        }

        let current_track = self.track_state.track.is_some();

        let toggle_playing_button = if self.track_state.playing && current_track {
            PAUSE_SYMBOL
        } else {
            PLAY_SYMBOL
        };

        if button(ui, toggle_playing_button) && current_track {
            let _ = self.player_cmd_tx.send(PlayerCommand::Toggle);
        }

        if button(ui, SKIP_FORWARD_SYMBOL) {
            let _ = self.player_cmd_tx.send(PlayerCommand::SkipNext);
        }
    }

    fn ui_volume(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.track_state.volume, DEFAULT_VOLUME_RANGE).text("Volume"),
        );

        let volume_dx = (self.track_state.volume - self.track_state.last_volume_sent).abs();

        if volume_dx > f32::EPSILON {
            let _ = self
                .player_cmd_tx
                .send(PlayerCommand::SetVolume(self.track_state.volume));

            self.track_state.last_volume_sent = self.track_state.volume;
        }
    }

    fn ui_seek(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.set_width(SEEKBAR_WIDTH);

            if let (Some(progress), Some(track)) =
                (&self.track_state.progress, &self.track_state.track)
            {
                let mut playback_secs = progress.as_secs_f64();
                let total_duration_secs = track.duration_secs;

                let slider = egui::Slider::new(&mut playback_secs, 0.0..=total_duration_secs)
                    .text("Playback");

                if ui.add(slider).changed() {
                    let _ = self.player_cmd_tx.send(PlayerCommand::SetPosition(
                        std::time::Duration::from_secs_f64(playback_secs),
                    ));
                }

                let current_time = Duration::from_secs_f64(playback_secs.ceil());
                let total_time = Duration::from_secs_f64(total_duration_secs.ceil());

                ui.label(format!(
                    "{}/{}",
                    human_duration(current_time),
                    human_duration(total_time)
                ));
            } else {
                let mut dummy = 0.0;
                let slider = egui::Slider::new(&mut dummy, 0.0..=1.0).text("Playback");

                ui.add_enabled(false, slider);
                ui.label("00:00/00:00");
            }
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: &Option<PlayerEvent>) {
        ui.ctx().request_repaint_after(Duration::from_millis(100));

        if self.track_state.track.is_some() && self.track_state.playing {
            let now = Instant::now();
            let poll_interval = Duration::from_millis(500);

            if self
                .track_state
                .last_progress_poll
                .map_or_else(|| true, |last| now.duration_since(last) >= poll_interval)
            {
                let _ = self.player_cmd_tx.send(PlayerCommand::Position);
                self.track_state.last_progress_poll = Some(now);
            }
        }

        if let Some(event) = player_event {
            self.handle_player_event(event.clone());
        }

        let width = ui.available_width();
        let playback_bar_height = 60.0;

        ui.allocate_ui(Vec2::new(width, playback_bar_height), |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    self.ui_playback_controls(ui);
                });

                let seek_bar_width = 300.0;
                let remaining_width = ui.available_width();
                let left_padding = (remaining_width - seek_bar_width) / 2.0;

                ui.add_space(left_padding.max(0.0));
                ui.allocate_ui(Vec2::new(seek_bar_width, playback_bar_height), |ui| {
                    self.ui_seek(ui);
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    self.ui_volume(ui);
                });
            });
        });
    }
}
