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
    playback::state::PlayerCommand,
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
        let config_volume = config.borrow().playback.volume;
        context
            .borrow_mut()
            .playback
            .control
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

        let mut context = self.context.borrow_mut();

        // TODO: Get rid of this terrible layout
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                // TODO: Dynamic spacing based on something else if this layout has to be kept?
                ui.add_space(8.0);

                // TODO: Configure based on autoplay direction
                // Skip back a track
                if button(ui, SKIP_BACK_IMAGE, MEDIUM_BUTTON_SIZE) {
                    if context.playback.autoplay.is_shuffle() {
                        // TODO: Save the previous track and go there instead of selecting another random one
                        context.playback.autoplay.set_select_new_track(true);
                    } else {
                        context.playback.autoplay.set_incoming_track(
                            true,
                            Some(AutoplayType::Iterative(PlayDirection::Backward)),
                        );
                    }
                }
            });

            let current_track = &context.playback.selected_track;

            // Toggle pause/play on a track
            let toggle_playing_button = if current_track.as_ref().is_some_and(|track| track.playing)
            {
                PAUSE_IMAGE
            } else {
                PLAY_IMAGE
            };

            if button(ui, toggle_playing_button, LARGE_BUTTON_SIZE) && current_track.is_some() {
                let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
            }

            // Skip to the next track
            if button(ui, SKIP_NEXT_IMAGE, MEDIUM_BUTTON_SIZE) {
                if context.playback.autoplay.is_shuffle() {
                    context.playback.autoplay.set_select_new_track(true);
                } else {
                    context.playback.autoplay.set_incoming_track(
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
            let volume = context.playback.control.volume_mut();
            ui.add(egui::Slider::new(volume, DEFAULT_VOLUME_RANGE).show_value(false));
            let volume_button = ImageButton::new(VOLUME_IMAGE).frame(false);
            ui.add_sized([SMALL_BUTTON_SIZE, SMALL_BUTTON_SIZE], volume_button);
        }

        let volume = context.playback.control.volume;
        let last_volume_sent = context.playback.control.last_volume_sent;

        let volume_dx = (volume - last_volume_sent).abs();

        if volume_dx > f32::EPSILON {
            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SetVolume(volume));

            context.playback.control.last_volume_sent = volume;
            self.config.borrow_mut().playback.volume = volume;
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

        let (mut playback_secs, total_duration_secs, has_hours, needs_new_track) = {
            let playback = &mut context.playback;

            if let (Some(progress), Some(track_context)) = (
                playback.control.current_progress(),
                &playback.selected_track,
            ) {
                let playback_secs = progress.as_secs_f64();
                let total_duration_secs = track_context.track.duration_secs;
                let control = &mut playback.control;

                let needs_new_track =
                    playback_secs >= total_duration_secs && !control.changing_track;

                let has_hours =
                    (Duration::from_secs_f64(total_duration_secs.floor()).as_secs() / 3600) > 0;

                (
                    playback_secs,
                    total_duration_secs,
                    has_hours,
                    needs_new_track,
                )
            } else {
                let mut dummy = 0.0;
                let slider = egui::Slider::new(&mut dummy, 0.0..=1.0).show_value(false);

                ui.label("--:--");
                ui.add_enabled(false, slider);
                ui.label("--:--");
                return;
            }
        };

        if needs_new_track {
            let playback = &mut context.playback;
            playback.control.changing_track = true;
            playback.autoplay.set_select_new_track(true);
        }

        let current_time = Duration::from_secs_f64(playback_secs.floor());
        let total_time = Duration::from_secs_f64(total_duration_secs.floor());

        let slider =
            egui::Slider::new(&mut playback_secs, 0.0..=total_duration_secs).show_value(false);

        let human_current_time = human_duration(current_time, has_hours).to_string();
        let human_total_time = human_duration(total_time, has_hours).to_string();

        ui.label(human_current_time);
        let response = ui.add(slider);
        ui.label(human_total_time);

        let playback = &mut context.playback;
        let control = &mut playback.control;

        if !control.changing_track && response.drag_stopped() {
            control.progress_base = Some(Duration::from_secs_f64(playback_secs));
            control.progress_timestamp = Some(Instant::now());

            let _ = self
                .channels
                .player_command_tx
                .send(PlayerCommand::SetPosition(Duration::from_secs_f64(
                    playback_secs,
                )));
        }
    }

    fn ui_currently_playing(&mut self, ui: &mut egui::Ui) {
        let Some(track_context) = &self.context.borrow().playback.selected_track else {
            return;
        };

        let context = self.context.borrow();

        let autoplay_playlist_context = if let Some(playlist) = context.ui.playlist.autoplay() {
            playlist.name
        } else {
            "All tracks".to_string()
        };

        let autoplay_type = context.playback.autoplay.autoplay();

        let autoplay_text = {
            let text = if matches!(
                autoplay_type,
                AutoplayType::Iterative(PlayDirection::Forward)
            ) {
                format!("Autoplay: {autoplay_playlist_context}")
            } else {
                format!("Autoplay {autoplay_type}: {autoplay_playlist_context}")
            };

            RichText::new(text)
        };

        let track_text = RichText::new(&track_context.track.name).strong();

        ui.vertical(|ui| {
            ui.add_space(NOW_PLAYING_SPACE);
            ui.label(autoplay_text.size(AUTOPLAY_FONT_SIZE));
            ui.label(track_text);
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
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
