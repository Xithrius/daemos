use egui::RichText;

use crate::context::SharedContext;

const DEFAULT_POPUP_SIZE: [f32; 2] = [300.0, 200.0];
const WINDOW_HEADER_SPACING: f32 = 5.0;

#[derive(Debug, Clone)]
pub struct PlaybackDebugPopup {
    context: SharedContext,
}

impl PlaybackDebugPopup {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let playback_context = {
            let context = self.context.borrow();

            if !self.context.borrow().ui.visibility.debug_playback() {
                return;
            }

            context.playback.to_owned()
        };

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
                            "Duration: {} second(s)",
                            track_context.track.duration_secs as usize
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
                        ui.label(format!("Progress base: {base:.2?}"));
                    } else {
                        ui.label("Progress base: None");
                    }

                    if let Some(snapshot) = control.progress_snapshot {
                        // TODO: Make this prettier, current output is of
                        // Progress snapshot: Instant { tv_sec: 243289, tv_nsec: 299833956 }
                        ui.label(format!("Progress snapshot: {snapshot:.2?}"));
                    } else {
                        ui.label("Progress snapshot: None");
                    }

                    if let Some(current_progress) = control.current_progress() {
                        ui.label(format!("Current Progress: {current_progress:.2?}"));
                    } else {
                        ui.label("Current Progress: None");
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.label(RichText::new("Volume State").underline().heading());
                    ui.add_space(WINDOW_HEADER_SPACING);

                    ui.label(format!("Volume: {:.2}", control.volume));
                    ui.label(format!(
                        "Last volume snapshot: {:.2}",
                        control.volume_snapshot
                    ));
                });
            });
    }
}
