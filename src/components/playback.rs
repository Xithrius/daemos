use std::{
    ops::RangeInclusive,
    rc::Rc,
    time::{Duration, Instant},
};

use egui::RichText;
use tracing::{debug, warn};

use super::ComponentChannels;
use crate::{
    config::core::CoreConfig,
    context::SharedContext,
    database::models::tracks::Track,
    playback::state::{PlayerCommand, PlayerEvent},
    utils::formatting::human_duration,
};

pub const PLAYBACK_BAR_HEIGHT: f32 = 60.0;

const DEFAULT_VOLUME_RANGE: RangeInclusive<f32> = 0.0..=1.0;
const PLAYBACK_BUTTON_FONT_SIZE: f32 = 22.5;

const SKIP_BACKWARD_SYMBOL: &str = "\u{23EE}"; // ⏮
const PLAY_SYMBOL: &str = "\u{25B6}"; // ▶
const PAUSE_SYMBOL: &str = "\u{23F8}"; // ⏸
const SKIP_FORWARD_SYMBOL: &str = "\u{23ED}"; // ⏭

#[derive(Debug, Clone)]
struct TrackState {
    track: Option<Track>,
    playing: bool,
    volume: f32,
    last_volume_sent: f32,

    progress_base: Option<Duration>,
    progress_timestamp: Option<Instant>,
}

impl Default for TrackState {
    fn default() -> Self {
        Self {
            track: None,
            playing: false,
            volume: 0.5,
            last_volume_sent: 0.5,
            progress_base: None,
            progress_timestamp: None,
        }
    }
}

impl TrackState {
    pub fn new(volume: f32) -> Self {
        Self {
            track: None,
            playing: true,
            volume,
            last_volume_sent: volume,
            progress_base: None,
            progress_timestamp: None,
        }
    }

    fn current_progress(&self) -> Option<Duration> {
        match (self.progress_base, self.progress_timestamp) {
            (Some(base), Some(ts)) if self.playing => {
                Some(base + Instant::now().duration_since(ts))
            }
            (Some(base), _) => Some(base),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackBar {
    context: SharedContext,
    channels: Rc<ComponentChannels>,
    track_state: TrackState,
    debug: bool,
}

impl PlaybackBar {
    pub fn new(
        config: &CoreConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        let track_state = TrackState::new(config.volume.default);

        Self {
            context,
            channels,
            track_state,
            debug: false,
        }
    }

    fn reset_track_state(&mut self) {
        self.track_state = TrackState::default();
    }

    fn handle_player_event(&mut self, player_event: PlayerEvent) {
        debug!(
            "Playback bar UI component received event: {:?}",
            player_event
        );

        match player_event {
            PlayerEvent::TrackChanged(track) => {
                // Only if the track hash is different or track doesn't exist, then we should restart the state
                if self
                    .track_state
                    .track
                    .as_ref()
                    .is_none_or(|prev| prev.hash != track.hash)
                {
                    self.track_state.track = Some(track.clone());
                    self.track_state.playing = true;
                    self.track_state.progress_base = Some(Duration::ZERO);
                    self.track_state.progress_timestamp = Some(Instant::now());
                }
            }
            PlayerEvent::TrackPlayingStatus(playing) => {
                // If we are pausing, freeze current progress
                if !playing && self.track_state.playing {
                    // Capture how much time has passed
                    if let (Some(base), Some(ts)) = (
                        self.track_state.progress_base,
                        self.track_state.progress_timestamp,
                    ) {
                        let elapsed = Instant::now().duration_since(ts);
                        self.track_state.progress_base = Some(base + elapsed);
                        self.track_state.progress_timestamp = None;
                    }
                }

                // If we are resuming, set the timestamp so progress resumes from base
                if playing && !self.track_state.playing {
                    self.track_state.progress_timestamp = Some(Instant::now());
                }

                self.track_state.playing = playing;
            }
            PlayerEvent::TrackProgress(duration) => {
                // If duration is not synced properly, do it here
                if self
                    .track_state
                    .progress_base
                    .is_some_and(|progress_base| progress_base < duration)
                {
                    warn!(
                        "Track progress desync detected, setting progress base to received player position"
                    );
                    self.track_state.progress_base = Some(duration);
                    self.track_state.progress_timestamp = Some(Instant::now());
                }
            }
            PlayerEvent::CurrentVolume(volume) => {
                if self.track_state.volume != volume {
                    warn!(
                        "Volume desync detected, UI track state does not equal player volume ({} != {})",
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
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SkipPrevious);
        }

        let current_track = self.track_state.track.is_some();

        let toggle_playing_button = if self.track_state.playing && current_track {
            PAUSE_SYMBOL
        } else {
            PLAY_SYMBOL
        };

        if button(ui, toggle_playing_button) && current_track {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
        }

        if button(ui, SKIP_FORWARD_SYMBOL) {
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SkipNext);
        }
    }

    fn ui_volume(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.track_state.volume, DEFAULT_VOLUME_RANGE)
                .text("Volume")
                .show_value(false),
        );

        let volume_dx = (self.track_state.volume - self.track_state.last_volume_sent).abs();

        if volume_dx > f32::EPSILON {
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SetVolume(self.track_state.volume));

            self.track_state.last_volume_sent = self.track_state.volume;
        }
    }

    fn ui_seek(&mut self, ui: &mut egui::Ui) {
        if let (Some(progress), Some(track)) =
            (self.track_state.current_progress(), &self.track_state.track)
        {
            let mut playback_secs = progress.as_secs_f64();
            let total_duration_secs = track.duration_secs;

            if playback_secs >= total_duration_secs {
                self.reset_track_state();
                self.context.borrow_mut().set_select_new_track(true);
                return;
            }

            let slider =
                egui::Slider::new(&mut playback_secs, 0.0..=total_duration_secs).show_value(false);

            let response = ui.add(slider);

            if response.drag_stopped() {
                self.track_state.progress_base = Some(Duration::from_secs_f64(playback_secs));
                self.track_state.progress_timestamp = Some(Instant::now());

                let _ = self
                    .channels
                    .player_command_tx
                    .send(PlayerCommand::SetPosition(
                        std::time::Duration::from_secs_f64(playback_secs),
                    ));
            }

            let current_time = Duration::from_secs_f64(playback_secs.floor());
            let total_time = Duration::from_secs_f64(total_duration_secs.floor());

            let has_hours = (total_time.as_secs() / 3600) > 0;

            ui.label(format!(
                "{}/{}",
                human_duration(current_time, has_hours),
                human_duration(total_time, has_hours)
            ));
        } else {
            let mut dummy = 0.0;
            let slider = egui::Slider::new(&mut dummy, 0.0..=1.0).show_value(false);

            ui.add_enabled(false, slider);
            ui.label("--:--/--:--");
        }
    }

    fn debug_window(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Playback Debug Info")
            .open(&mut self.debug)
            .collapsible(true)
            .resizable(true)
            .default_size([400.0, 250.0])
            .show(ui.ctx(), |ui| {
                let ts = &self.track_state;

                ui.group(|ui| {
                    ui.label("🎵 Track Info:");
                    ui.label(format!("Loaded: {}", ts.track.is_some()));
                    if let Some(track) = &ts.track {
                        ui.label(format!("Path: {:?}", track.path));
                        ui.label(format!("Duration: {} seconds", track.duration_secs));
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label("⏯ Playback State:");
                    ui.label(format!("Playing: {}", ts.playing));

                    if let Some(base) = ts.progress_base {
                        ui.label(format!("Progress Base: {:.2?}", base));
                    } else {
                        ui.label("Progress Base: None");
                    }

                    if let Some(ts) = ts.progress_timestamp {
                        ui.label(format!("Progress Timestamp: {:?}", ts));
                    } else {
                        ui.label("Progress Timestamp: None");
                    }

                    if let Some(simulated) = ts.current_progress() {
                        ui.label(format!("Simulated Current Progress: {:.2?}", simulated));
                    } else {
                        ui.label("Simulated Current Progress: None");
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label("🔊 Volume State:");
                    ui.label(format!("Volume: {:.2}", ts.volume));
                    ui.label(format!("Last Volume Sent: {:.2}", ts.last_volume_sent));
                });
            });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: &Option<PlayerEvent>) {
        if let Some(event) = player_event {
            self.handle_player_event(event.clone());
            ui.ctx().request_repaint();
        }

        if self.track_state.track.is_some() && self.track_state.playing {
            ui.ctx().request_repaint();
        }

        if self.debug {
            self.debug_window(ui);
        }

        let playback_bar = ui.horizontal_centered(|ui| {
            self.ui_playback_controls(ui);

            self.ui_seek(ui);

            self.ui_volume(ui);
        });

        playback_bar.response.context_menu(|ui| {
            if ui.button("Toggle Debug Info").clicked() {
                self.debug = !self.debug;
                ui.close_menu();
            }
        });
    }
}
