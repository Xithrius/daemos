use std::{collections::HashSet, time::Duration};

use color_eyre::Result;
use crossbeam::channel::Sender;
use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};

use crate::{
    database::{connection::DatabaseCommand, models::tracks::Track},
    files::open::get_track_file_name,
    playback::state::{PlayerCommand, PlayerEvent},
    utils::formatting::human_duration,
};

const TABLE_HEADER_HEIGHT: f32 = 25.0;
const TABLE_ROW_HEIGHT: f32 = 20.0;

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
    tracks: Vec<Track>,
    #[allow(dead_code)]
    database_command_tx: Sender<DatabaseCommand>,
    player_command_tx: Sender<PlayerCommand>,

    #[allow(dead_code)]
    selection: HashSet<usize>,
    playing: Option<TrackState>,

    search_text: String,
    search_focused: bool,
}

impl TrackTable {
    pub fn new(
        database_command_tx: Sender<DatabaseCommand>,
        player_command_tx: Sender<PlayerCommand>,
    ) -> Self {
        let _ = database_command_tx.send(DatabaseCommand::QueryAllTracks);

        Self {
            tracks: Vec::new(),
            database_command_tx,
            player_command_tx,
            selection: HashSet::default(),
            playing: None,
            search_text: String::new(),
            search_focused: false,
        }
    }

    pub fn search_focused(&self) -> bool {
        self.search_focused
    }

    pub fn set_tracks(&mut self, tracks: Vec<Track>) {
        self.tracks = tracks;
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
            if let Err(err) = self.player_command_tx.send(PlayerCommand::Toggle) {
                error!(
                    "Failed to toggle track state on path {:?}: {}",
                    track.path, err
                );
            }

            return;
        }

        if let Err(err) = self
            .player_command_tx
            .send(PlayerCommand::Create(track.clone()))
        {
            error!("Failed to start track on path {:?}: {}", track.path, err);
        }

        let new_track_state = TrackState::new(row_index, track.clone(), true);

        self.playing = Some(new_track_state)
    }

    fn ui_table(&mut self, ui: &mut egui::Ui, height: f32) {
        // TODO: Don't clone here
        let filtered_tracks: Vec<Track> = self
            .tracks
            .iter()
            .filter(|track| {
                if let Some(track_file_name) = get_track_file_name(track.path.clone()) {
                    return if self.search_text.is_empty() {
                        true
                    } else {
                        track_file_name.to_lowercase().contains(&self.search_text)
                    };
                }

                false
            })
            .map(|track| track.to_owned())
            .collect();

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
                let num_rows = filtered_tracks.len();

                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let row_index = row.index();

                    let track = filtered_tracks.get(row_index).cloned();
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
        let search_text_edit =
            egui::TextEdit::singleline(&mut self.search_text).hint_text("Search...");

        let response = ui.add(search_text_edit);

        self.search_focused = response.has_focus();

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            debug!("Searched: {}", self.search_text);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: &Option<PlayerEvent>) {
        if let Some(event) = player_event {
            self.handle_player_event(event.clone());
            ui.ctx().request_repaint();
        }

        let height = ui.available_height();

        ui.vertical(|ui| {
            self.ui_search(ui);

            ui.separator();

            self.ui_table(ui, height);
        });
    }
}
