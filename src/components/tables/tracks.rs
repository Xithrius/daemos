use std::{
    collections::HashSet,
    rc::Rc,
    time::{Duration, Instant},
};

use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};
use uuid::Uuid;

use super::{TABLE_HEADER_HEIGHT, TABLE_ROW_HEIGHT};
use crate::{
    components::ComponentChannels,
    context::{PlayDirection, SharedContext},
    database::{connection::DatabaseCommand, models::tracks::Track},
    files::open::get_track_file_name,
    playback::state::{PlayerCommand, PlayerEvent},
    utils::formatting::human_duration,
};

#[derive(Debug, Clone)]
struct TrackState {
    index: usize,
    track: Track,
    playing: bool,
}

impl TrackState {
    fn new(index: usize, track: Track, playing: bool) -> Self {
        Self {
            index,
            track,
            playing,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackTable {
    context: SharedContext,
    channels: Rc<ComponentChannels>,

    tracks: Vec<Track>,
    filtered_tracks: Vec<Track>,
    track_ids: HashSet<Uuid>,

    #[allow(dead_code)]
    selection: HashSet<usize>,
    playing: Option<TrackState>,

    search_text: String,
    search_changed: bool,
    search_focused: bool,
    search_focus_requested: bool,
    search_duration: Option<Duration>,
}

impl TrackTable {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        let _ = channels
            .database_command_tx
            .send(DatabaseCommand::QueryAllTracks);

        Self {
            context,
            channels,

            tracks: Vec::default(),
            filtered_tracks: Vec::default(),
            track_ids: HashSet::default(),
            selection: HashSet::default(),
            playing: None,
            search_text: String::default(),
            search_changed: false,
            search_focused: false,
            search_focus_requested: false,
            search_duration: None,
        }
    }

    pub fn search_focused(&self) -> bool {
        self.search_focused
    }

    pub fn request_search_focus(&mut self) {
        self.search_focus_requested = true
    }

    pub fn set_tracks(&mut self, tracks: Vec<Track>) {
        self.track_ids = tracks.iter().map(|track| track.id).collect();
        self.tracks = tracks;
    }

    pub fn add_track(&mut self, track: &Track) {
        if self.track_ids.insert(track.id) {
            self.tracks.push(track.clone());
        }
    }

    pub fn remove(&mut self, id: &Uuid) -> bool {
        if self.track_ids.remove(id) {
            if let Some(pos) = self.tracks.iter().position(|t| &t.id == id) {
                self.tracks.remove(pos);
            }

            true
        } else {
            false
        }
    }

    fn handle_player_event(&mut self, player_event: PlayerEvent) {
        debug!(
            "Track table UI component received event: {:?}",
            player_event
        );

        #[allow(clippy::single_match)]
        match player_event {
            PlayerEvent::TrackPlayingStatus(playing) => {
                if let Some(track_state) = self.playing.as_mut() {
                    track_state.playing = playing;
                }
            }
            _ => {}
        }
    }

    // fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
    //     if row_response.clicked() {
    //         if self.selection.contains(&row_index) {
    //             self.selection.remove(&row_index);
    //         } else {
    //             self.selection.insert(row_index);
    //         }
    //     }
    // }

    fn toggle_row_play(&mut self, row_index: usize, track: &Track) {
        if self.playing.as_ref().is_some_and(
            |TrackState {
                 index: playing_index,
                 track: playing_track,
                 playing: _,
             }| { (*playing_index == row_index) && (*playing_track == *track) },
        ) {
            if let Err(err) = self.channels.player_command_tx.send(PlayerCommand::Toggle) {
                error!(
                    "Failed to toggle track state on path {:?}: {}",
                    track.path, err
                );
            }

            return;
        }

        if let Err(err) = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(track.clone()))
        {
            error!("Failed to start track on path {:?}: {}", track.path, err);
        }

        let new_track_state = TrackState::new(row_index, track.clone(), true);

        self.playing = Some(new_track_state)
    }

    fn select_new_track(&mut self) {
        let Some(autoplay_direction) = self.context.borrow().select_new_track() else {
            return;
        };

        let Some(playing) = &self.playing else {
            return;
        };

        self.context.borrow_mut().set_select_new_track(None);

        let Some(index) = self
            .filtered_tracks
            .iter()
            .position(|track| track.hash == playing.track.hash)
        else {
            // Could not find a new track to play, clearing sink
            let _ = self.channels.player_command_tx.send(PlayerCommand::Clear);
            self.playing = None;

            return;
        };

        let filtered_len = self.filtered_tracks.len();

        // TODO: Configurable default autoplay direction
        let new_index = if matches!(autoplay_direction, PlayDirection::Forward) {
            (index + 1) % filtered_len
        } else {
            (index + filtered_len.saturating_sub(1)) % filtered_len
        };

        let Some(new_track) = self.filtered_tracks.get(new_index) else {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Clear);
            self.playing = None;

            return;
        };

        let _ = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(new_track.clone()));

        let new_track_state = TrackState::new(new_index, new_track.clone(), true);

        debug!("Selected new track with autoplay: {:?}", new_track_state);

        self.playing = Some(new_track_state);
    }

    fn ui_table(&mut self, ui: &mut egui::Ui, height: f32) {
        self.select_new_track();

        let table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(50.0).resizable(true))
            .column(Column::remainder())
            .column(Column::auto().at_least(50.0))
            .sense(egui::Sense::click());

        table
            .header(TABLE_HEADER_HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.heading("Index");
                });
                header.col(|ui| {
                    ui.heading("Track");
                });
                header.col(|ui| {
                    ui.heading("Duration");
                });
            })
            .body(|body| {
                let num_rows = self.filtered_tracks.len();

                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let row_index = row.index();

                    let track = self.filtered_tracks.get(row_index).cloned();
                    let playing = self.playing.clone();

                    if let Some(track) = track {
                        if let Some(track_file_name) = get_track_file_name(track.path.clone()) {
                            row.set_selected(playing.as_ref().is_some_and(
                                |TrackState {
                                     index: _,
                                     track:
                                         Track {
                                             id: _,
                                             path: _,
                                             hash,
                                             duration_secs: _,
                                             valid: _,
                                             created_at: _,
                                             updated_at: _,
                                         },
                                     playing: _,
                                 }| { *hash == track.hash },
                            ));

                            row.col(|ui| {
                                let label = ui
                                    .label(row_index.to_string())
                                    .on_hover_cursor(egui::CursorIcon::Default);
                                if label.double_clicked() {
                                    self.toggle_row_play(row_index, &track);
                                }
                            });

                            row.col(|ui| {
                                let label = ui
                                    .label(track_file_name)
                                    .on_hover_cursor(egui::CursorIcon::Default);
                                if label.double_clicked() {
                                    self.toggle_row_play(row_index, &track);
                                }
                            });

                            row.col(|ui| {
                                let track_duration = Duration::from_secs_f64(track.duration_secs);
                                let readable_track_duration = human_duration(track_duration, false);

                                let label = ui
                                    .label(readable_track_duration)
                                    .on_hover_cursor(egui::CursorIcon::Default);

                                if label.double_clicked() {
                                    self.toggle_row_play(row_index, &track);
                                }
                            });

                            if row.response().double_clicked() {
                                self.toggle_row_play(row_index, &track);
                            }
                        }
                    }
                });
            });
    }

    fn ui_search(&mut self, ui: &mut egui::Ui) {
        if self.search_text.is_empty() {
            self.filtered_tracks = self.tracks.clone();
        }
        // Only recalculate the filtered tracks if the search input has changed
        // and the previous search yielded some results

        // TODO
        // Keep track of previous search that yielded result such that if we
        // delete characters to go back to that same search length, then add more characters,
        // calculation of filtered tracks will begin again
        else if self.search_changed && !self.filtered_tracks.is_empty() {
            let search_lower = self.search_text.to_lowercase();

            let start = Instant::now();

            let filtered_tracks: Vec<Track> = self
                .tracks
                .iter()
                .filter_map(|track| {
                    get_track_file_name(track.path.clone()).and_then(|name| {
                        if name.to_lowercase().contains(&search_lower) {
                            Some(track.clone())
                        } else {
                            None
                        }
                    })
                })
                .collect();

            let duration = start.elapsed();
            self.search_duration = Some(duration);
            debug!(
                "Filtered into {} tracks in {:?}",
                filtered_tracks.len(),
                duration
            );

            self.filtered_tracks = filtered_tracks;
        }

        let search_text_edit =
            egui::TextEdit::singleline(&mut self.search_text).hint_text("Search...");

        let response = ui.add(search_text_edit);

        if self.search_focus_requested {
            response.request_focus();
            self.search_focus_requested = false;
        }

        self.search_changed = response.changed();
        self.search_focused = response.has_focus();
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: &Option<PlayerEvent>) {
        if let Some(event) = player_event {
            self.handle_player_event(event.clone());
            ui.ctx().request_repaint();
        }

        let height = ui.available_height();

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                self.ui_search(ui);

                if let Some(search_duration) = self.search_duration {
                    if self.filtered_tracks.len() != self.tracks.len() {
                        ui.label(format!(
                            "{} results in {:?}",
                            self.filtered_tracks.len(),
                            search_duration
                        ));
                    }
                }
            });

            ui.separator();

            self.ui_table(ui, height);
        });
    }
}
