use egui::RichText;

use crate::{config::core::SharedConfig, context::SharedContext};

const DEFAULT_POPUP_SIZE: [f32; 2] = [300.0, 200.0];
const WINDOW_HEADER_SPACING: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct PlaybackDebugPopup {
    _config: SharedConfig,
    context: SharedContext,
}

impl PlaybackDebugPopup {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        Self {
            _config: config,
            context,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().ui.visibility.debug_playback() {
            return;
        }

        let context = self.context.borrow();
        let playback_context = context.playback.clone();

        egui::Window::new("Playback Debug")
            .open(self.context.borrow_mut().ui.visibility.debug_playback_mut())
            .resizable(true)
            .title_bar(true)
            .min_size(egui::Vec2::from(DEFAULT_POPUP_SIZE))
            .show(ctx, |ui| {
                let track_context = &playback_context.selected_track;
                let control = &playback_context.control;

                ui.group(|ui| {
                    ui.label(RichText::new("Track Info").underline().heading());
                    ui.add_space(WINDOW_HEADER_SPACING);

                    ui.label(format!("Loaded: {}", track_context.is_some()));

                    if let Some(track_context) = &track_context {
                        ui.label(format!("Path: {:?}", track_context.track.path));
                        ui.label(format!(
                            "Duration: {} seconds",
                            track_context.track.duration_secs
                        ));
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Playback State").underline().heading());
                    ui.add_space(WINDOW_HEADER_SPACING);

                    ui.label(format!(
                        "Playing: {:?}",
                        track_context.as_ref().map(|track| track.playing)
                    ));

                    if let Some(base) = control.progress_base {
                        ui.label(format!("Progress Base: {base:.2?}"));
                    } else {
                        ui.label("Progress Base: None");
                    }

                    if let Some(ts) = control.progress_timestamp {
                        ui.label(format!("Progress Timestamp: {ts:?}"));
                    } else {
                        ui.label("Progress Timestamp: None");
                    }

                    if let Some(simulated) = control.current_progress() {
                        ui.label(format!("Simulated Current Progress: {simulated:.2?}"));
                    } else {
                        ui.label("Simulated Current Progress: None");
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Volume State").underline().heading());
                    ui.add_space(WINDOW_HEADER_SPACING);

                    ui.label(format!("Volume: {:.2}", control.volume));
                    ui.label(format!("Last Volume Sent: {:.2}", control.last_volume_sent));
                });
            });
    }
}
