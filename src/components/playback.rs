use std::{
    ops::RangeInclusive,
    rc::Rc,
    time::{Duration, Instant},
};

use egui::{ImageButton, ImageSource, RichText, include_image};
use tracing::{debug, warn};

use super::ComponentChannels;
use crate::{
    config::core::SharedConfig,
    context::{AutoplayType, PlayDirection, SharedContext},
    database::models::tracks::Track,
    playback::state::{PlayerCommand, PlayerEvent},
    utils::formatting::human_duration,
};

pub const PLAYBACK_BAR_HEIGHT: f32 = 75.0;

const DEFAULT_VOLUME_RANGE: RangeInclusive<f32> = 0.0..=1.0;

const LARGE_BUTTON_SIZE: f32 = 48.0;
const MEDIUM_BUTTON_SIZE: f32 = 32.0;
const SMALL_BUTTON_SIZE: f32 = 24.0;

const SKIP_BACK_IMAGE: egui::ImageSource<'_> = include_image!("../../static/assets/skip-back.png");
const SKIP_NEXT_IMAGE: egui::ImageSource<'_> = include_image!("../../static/assets/skip-next.png");
const PLAY_IMAGE: egui::ImageSource<'_> = include_image!("../../static/assets/play.png");
const PAUSE_IMAGE: egui::ImageSource<'_> = include_image!("../../static/assets/pause.png");
const VOLUME_IMAGE: egui::ImageSource<'_> = include_image!("../../static/assets/volume-up.png");

const AUTOPLAY_FONT_SIZE: f32 = 12.0;

const NOW_PLAYING_SPACE: f32 = 8.0;
const DEBUG_WINDOW_HEADER_SPACING: f32 = 5.0;
const SEEK_AND_AUTOPLAY_SPACING: f32 = 25.0;

const SEEK_BAR_WIDTH_RATIO: f32 = 2.5;
const MINUTES_SECONDS_PROGRESS_TEXT_WIDTH: f32 = 42.7;

#[derive(Debug, Clone)]
struct TrackState {
    track: Option<Track>,
    playing: bool,
    volume: f32,
    last_volume_sent: f32,

    progress_base: Option<Duration>,
    progress_timestamp: Option<Instant>,
    changing_track: bool,
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
            changing_track: false,
        }
    }
}

impl TrackState {
    pub fn new(volume: f32) -> Self {
        Self {
            playing: true,
            volume,
            last_volume_sent: volume,
            ..Default::default()
        }
    }

    fn current_progress(&self) -> Option<Duration> {
        match (self.progress_base, self.progress_timestamp) {
            (Some(base), Some(ts)) => Some(base + Instant::now().duration_since(ts)),
            (Some(base), _) => Some(base),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaybackBar {
    config: SharedConfig,
    context: SharedContext,
    channels: Rc<ComponentChannels>,
    track_state: TrackState,
}

impl PlaybackBar {
    pub fn new(
        config: SharedConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        let config_volume = config.borrow().volume.default;
        let track_state = TrackState::new(config_volume);

        Self {
            config,
            context,
            channels,
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
                    self.track_state.changing_track = false;
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
        let button = |ui: &mut egui::Ui, image: ImageSource, image_size: f32| -> bool {
            let image_button = ImageButton::new(image).frame(false);
            ui.add_sized([image_size, image_size], image_button)
                .clicked()
        };

        // TODO: Get rid of this terrible layout
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                // TODO: Dynamic spacing based on something else if this layout has to be kept?
                ui.add_space(8.0);

                // TODO: Configure based on autoplay direction
                // Skip back a track
                if button(ui, SKIP_BACK_IMAGE, MEDIUM_BUTTON_SIZE) {
                    if self.context.borrow().playback.is_autoplay_shuffle() {
                        // TODO: Save the previous track and go there instead of selecting another random one
                        self.context
                            .borrow_mut()
                            .playback
                            .set_select_new_track(true);
                    } else {
                        self.context.borrow_mut().playback.set_incoming_track(
                            true,
                            Some(AutoplayType::Iterative(PlayDirection::Backward)),
                        );
                    }
                }
            });

            let current_track = self.track_state.track.is_some();

            // Toggle pause/play on a track
            let toggle_playing_button = if self.track_state.playing && current_track {
                PAUSE_IMAGE
            } else {
                PLAY_IMAGE
            };

            if button(ui, toggle_playing_button, LARGE_BUTTON_SIZE) && current_track {
                let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
            }

            // Skip to the next track
            if button(ui, SKIP_NEXT_IMAGE, MEDIUM_BUTTON_SIZE) {
                if self.context.borrow().playback.is_autoplay_shuffle() {
                    self.context
                        .borrow_mut()
                        .playback
                        .set_select_new_track(true);
                } else {
                    self.context.borrow_mut().playback.set_incoming_track(
                        true,
                        Some(AutoplayType::Iterative(PlayDirection::Forward)),
                    );
                }
            }
        });
    }

    fn ui_volume(&mut self, ui: &mut egui::Ui) {
        ui.add(
            egui::Slider::new(&mut self.track_state.volume, DEFAULT_VOLUME_RANGE).show_value(false),
        );
        let volume_button = ImageButton::new(VOLUME_IMAGE).frame(false);
        ui.add_sized([SMALL_BUTTON_SIZE, SMALL_BUTTON_SIZE], volume_button);

        let volume_dx = (self.track_state.volume - self.track_state.last_volume_sent).abs();

        if volume_dx > f32::EPSILON {
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SetVolume(self.track_state.volume));

            self.track_state.last_volume_sent = self.track_state.volume;
            self.config.borrow_mut().volume.default = self.track_state.volume;
        }
    }

    fn ui_seek(&mut self, ui: &mut egui::Ui) {
        // TODO: Get rid of this terribleness
        let available_width = ui.available_width();
        let slider_width = available_width / SEEK_BAR_WIDTH_RATIO;
        let side_spacing =
            (available_width - slider_width - (MINUTES_SECONDS_PROGRESS_TEXT_WIDTH * 2.0)) / 2.0;
        ui.spacing_mut().slider_width = slider_width;
        ui.add_space(side_spacing);

        if let (Some(progress), Some(track)) =
            (self.track_state.current_progress(), &self.track_state.track)
        {
            let mut playback_secs = progress.as_secs_f64();
            let total_duration_secs = track.duration_secs;

            if playback_secs >= total_duration_secs && !self.track_state.changing_track {
                self.track_state.changing_track = true;
                self.context
                    .borrow_mut()
                    .playback
                    .set_select_new_track(true);
            }

            let current_time = Duration::from_secs_f64(playback_secs.floor());
            let total_time = Duration::from_secs_f64(total_duration_secs.floor());

            let has_hours = (total_time.as_secs() / 3600) > 0;

            let slider =
                egui::Slider::new(&mut playback_secs, 0.0..=total_duration_secs).show_value(false);

            let human_current_time = human_duration(current_time, has_hours).to_string();
            let human_total_time = human_duration(total_time, has_hours).to_string();

            ui.label(human_current_time);
            let response = ui.add(slider);
            ui.label(human_total_time);

            if !self.track_state.changing_track && response.drag_stopped() {
                self.track_state.progress_base = Some(Duration::from_secs_f64(playback_secs));
                self.track_state.progress_timestamp = Some(Instant::now());

                let _ = self
                    .channels
                    .player_command_tx
                    .send(PlayerCommand::SetPosition(Duration::from_secs_f64(
                        playback_secs,
                    )));
            }
        } else {
            // This state should only be reached when there is no track playing,
            // and we're not currently selecting a new track
            let mut dummy = 0.0;
            let slider = egui::Slider::new(&mut dummy, 0.0..=1.0).show_value(false);

            ui.label("--:--");
            ui.add_enabled(false, slider);
            ui.label("--:--");
        }
    }

    fn ui_currently_playing(&mut self, ui: &mut egui::Ui) {
        let Some(track) = &self.track_state.track else {
            return;
        };

        let context = self.context.borrow();

        let autoplay_playlist_context = if let Some(playlist) = context.playlist.autoplay() {
            playlist.name
        } else {
            "All tracks".to_string()
        };

        let autoplay_type = context.playback.autoplay();

        let autoplay_text = if matches!(
            autoplay_type,
            AutoplayType::Iterative(PlayDirection::Forward)
        ) {
            RichText::new(format!("Autoplay: {}", autoplay_playlist_context))
        } else {
            RichText::new(format!(
                "Autoplay {}: {}",
                autoplay_type, autoplay_playlist_context
            ))
        };

        let track_text = RichText::new(&track.name).strong();

        ui.vertical(|ui| {
            ui.add_space(NOW_PLAYING_SPACE);
            ui.label(autoplay_text.size(AUTOPLAY_FONT_SIZE));
            ui.label(track_text);
        });
    }

    fn debug_window(&mut self, ui: &mut egui::Ui) {
        egui::Window::new("Playback Debug Info")
            .open(self.context.borrow_mut().ui.debug_playback_mut())
            .collapsible(true)
            .resizable(true)
            .default_size([400.0, 250.0])
            .show(ui.ctx(), |ui| {
                let ts = &self.track_state;

                ui.group(|ui| {
                    ui.label(RichText::new("Track Info").underline().heading());
                    ui.add_space(DEBUG_WINDOW_HEADER_SPACING);

                    ui.label(format!("Loaded: {}", ts.track.is_some()));

                    if let Some(track) = &ts.track {
                        ui.label(format!("Path: {:?}", track.path));
                        ui.label(format!("Duration: {} seconds", track.duration_secs));
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Playback State").underline().heading());
                    ui.add_space(DEBUG_WINDOW_HEADER_SPACING);

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
                    ui.label(RichText::new("Volume State").underline().heading());
                    ui.add_space(DEBUG_WINDOW_HEADER_SPACING);

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

        if self.context.borrow().ui.debug_playback() {
            self.debug_window(ui);
        }

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                self.ui_playback_controls(ui);

                ui.add_space(SEEK_AND_AUTOPLAY_SPACING);

                self.ui_currently_playing(ui);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.horizontal_centered(|ui| {
                        self.ui_volume(ui);
                    })
                });
            });

            ui.horizontal(|ui| {
                self.ui_seek(ui);
            });
        });
    }
}
