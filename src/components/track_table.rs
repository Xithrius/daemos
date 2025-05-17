use std::{collections::HashSet, time::Duration};

use color_eyre::Result;
use crossbeam::channel::Sender;
use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};

use crate::{
    database::{connection::SharedDatabase, models::tracks::Track},
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
    selection: HashSet<usize>,
    playing: Option<TrackState>,

    tx: Sender<PlayerCommand>,
}

impl TrackTable {
    pub fn new(database: SharedDatabase, tx: Sender<PlayerCommand>) -> Self {
        let tracks = match Track::select_all(database).map(|tracks| tracks.to_vec()) {
            Ok(tracks) => {
                debug!(
                    "Initial load of track table found {} track(s)",
                    tracks.len()
                );

                tracks
            }
            Err(err) => {
                error!("Failed getting tracks: {}", err);

                Vec::new()
            }
        };

        Self {
            tracks,
            selection: HashSet::default(),
            playing: None,
            tx,
        }
    }

    pub fn refresh_tracks(&mut self, database: SharedDatabase) -> Result<()> {
        let tracks = match Track::select_all(database).map(|tracks| tracks.to_vec()) {
            Ok(tracks) => {
                debug!("Refreshed tracks list with {} track(s)", tracks.len());

                tracks
            }
            Err(err) => {
                error!("Failed getting tracks: {}", err);

                Vec::new()
            }
        };

        self.tracks = tracks;

        Ok(())
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

    pub fn ui(&mut self, ui: &mut egui::Ui, player_event: &Option<PlayerEvent>) {
        if let Some(event) = player_event {
            self.handle_player_event(event.clone());
            ui.ctx().request_repaint();
        }

        let height = ui.available_height();

        let mut table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(50.0).resizable(true))
            .column(Column::remainder())
            .column(Column::auto().at_least(50.0));

        table = table.sense(egui::Sense::click());

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
                let num_rows = self.tracks.len();

                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let row_index = row.index();

                    let track = self.tracks.get(row_index).cloned();
                    let playing = self.playing.clone();

                    if let Some(track) = track {
                        if let Some(track_file_name) = get_track_file_name(track.path.clone()) {
                            row.set_selected(playing.as_ref().is_some_and(
                                |TrackState {
                                     index,
                                     track: _,
                                     playing: _,
                                 }| *index == row_index,
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
            if let Err(err) = self.tx.send(PlayerCommand::Toggle) {
                error!(
                    "Failed to toggle track state on path {:?}: {}",
                    track.path, err
                );
            }

            return;
        }

        if let Err(err) = self.tx.send(PlayerCommand::Create(track.clone())) {
            error!("Failed to start track on path {:?}: {}", track.path, err);
        }

        let new_track_state = TrackState::new(row_index, track.clone(), true);
        self.playing = Some(new_track_state)
    }
}
