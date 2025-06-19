use std::{
    ops::RangeInclusive,
    rc::Rc,
    time::{Duration, Instant},
};

use egui::{ImageButton, ImageSource, RichText, include_image};

use super::ComponentChannels;
use crate::{
    config::core::SharedConfig,
    context::{AutoplayType, PlayDirection, SharedContext},
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
pub struct PlaybackBar {
    config: SharedConfig,
    context: SharedContext,
    channels: Rc<ComponentChannels>,
}

impl PlaybackBar {
    pub fn new(
        config: SharedConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        let config_volume = config.borrow().volume.default;
        context
            .borrow_mut()
            .playback
            .track
            .set_volume(config_volume);

        Self {
            config,
            context,
            channels,
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

            let context = self.context.borrow();
            let current_track = context.playback.track.track();

            // Toggle pause/play on a track
            let toggle_playing_button = if current_track.is_some_and(|track| track.playing) {
                PAUSE_IMAGE
            } else {
                PLAY_IMAGE
            };

            if button(ui, toggle_playing_button, LARGE_BUTTON_SIZE) && current_track.is_some() {
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
        let mut context = self.context.borrow_mut();

        {
            let volume = context.playback.track.volume_mut();
            ui.add(egui::Slider::new(volume, DEFAULT_VOLUME_RANGE).show_value(false));
            let volume_button = ImageButton::new(VOLUME_IMAGE).frame(false);
            ui.add_sized([SMALL_BUTTON_SIZE, SMALL_BUTTON_SIZE], volume_button);
        }

        let volume = context.playback.track.volume;
        let last_volume_sent = context.playback.track.volume;

        let volume_dx = (volume - last_volume_sent).abs();

        if volume_dx > f32::EPSILON {
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SetVolume(volume));

            context.playback.track.last_volume_sent = volume;
            context.playback.track.volume = volume;
        }
    }

    fn ui_seek(&mut self, ui: &mut egui::Ui) {
        // TODO: Get rid of this terrible UI centering calculation
        let available_width = ui.available_width();
        let slider_width = available_width / SEEK_BAR_WIDTH_RATIO;
        let side_spacing =
            (available_width - slider_width - (MINUTES_SECONDS_PROGRESS_TEXT_WIDTH * 2.0)) / 2.0;
        ui.spacing_mut().slider_width = slider_width;
        ui.add_space(side_spacing);

        let mut context = self.context.borrow_mut();



        if let (Some(progress), Some(track)) =
            (context.playback.track.current_progress(), &context.playback.track.track)
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
        let Some(track) = &self.context.borrow().playback.track.track else {
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

        let track_text = RichText::new(&track.track.name).strong();

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
                let ts = &self.context.borrow().playback.track;

                ui.group(|ui| {
                    ui.label(RichText::new("Track Info").underline().heading());
                    ui.add_space(DEBUG_WINDOW_HEADER_SPACING);

                    ui.label(format!("Loaded: {}", ts.track.is_some()));

                    if let Some(track) = &ts.track {
                        ui.label(format!("Path: {:?}", track.track.path));
                        ui.label(format!("Duration: {} seconds", track.track.duration_secs));
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Playback State").underline().heading());
                    ui.add_space(DEBUG_WINDOW_HEADER_SPACING);

                    ui.label(format!(
                        "Playing: {:?}",
                        ts.track.as_ref().map(|track| track.playing)
                    ));

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
        {
            let mut context = self.context.borrow_mut();

            if let Some(event) = player_event {
                context.playback.handle_player_event(event.clone());
                ui.ctx().request_repaint();
            }

            if context
                .playback
                .track
                .track()
                .is_some_and(|track| track.playing)
            {
                ui.ctx().request_repaint();
            }
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
