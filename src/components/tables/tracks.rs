use std::{
    collections::HashSet,
    rc::Rc,
    time::{Duration, Instant},
};

use egui::CursorIcon;
use egui_extras::{Column, TableBuilder, TableRow};
use rand::Rng;
use tracing::{debug, error};
use uuid::Uuid;

use super::{TABLE_HEADER_HEIGHT, TABLE_ROW_HEIGHT};
use crate::{
    components::ComponentChannels,
    config::core::SharedConfig,
    context::{
        AutoplayType, PlayDirection, SharedContext, ShuffleType,
        playback::{PlaylistState, SelectedTrackContext},
    },
    database::{connection::DatabaseCommand, models::tracks::Track},
    playback::state::PlayerCommand,
    utils::{formatting::human_duration, random::filtered_random_index},
};

const INDEX_COLUMN_WIDTH: f32 = 50.0;
const DURATION_COLUMN_WIDTH: f32 = 100.0;

#[derive(Debug, Clone, Default)]
pub struct TrackSearch {
    pub text: String,
    pub changed: bool,
    pub focused: bool,
    pub focus_requested: bool,
    pub duration: Option<Duration>,
}

// TODO: Move all items here that are or could be set in other components to a higher context
#[derive(Debug, Clone)]
pub struct TrackTable {
    config: SharedConfig,
    context: SharedContext,
    channels: Rc<ComponentChannels>,

    tracks: Vec<Track>,
    // TODO: Convert to vector of indices
    filtered_tracks: Vec<Track>,
    track_ids: HashSet<Uuid>,

    // selection: HashSet<usize>,
    scroll_to_selected: bool,

    search: TrackSearch,
}

impl TrackTable {
    pub fn new(
        config: SharedConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        // TODO: Config for default playlist selection
        let _ = channels
            .database_command_tx
            .send(DatabaseCommand::QueryTracks(None));

        Self {
            config,
            context,
            channels,

            tracks: Vec::default(),
            filtered_tracks: Vec::default(),
            track_ids: HashSet::default(),
            // selection: HashSet::default(),
            scroll_to_selected: false,
            search: TrackSearch::default(),
        }
    }

    pub fn set_scroll_to_selected(&mut self, scroll: bool) {
        self.scroll_to_selected = scroll;
    }

    pub fn search_focused(&self) -> bool {
        self.search.focused
    }

    pub fn request_search_focus(&mut self) {
        self.search.focus_requested = true;
    }

    pub fn set_tracks(&mut self, mut tracks: Vec<Track>) {
        self.track_ids = tracks.iter().map(|track| track.id).collect();

        tracks.sort_by(|track_a, track_b| track_a.name.cmp(&track_b.name));
        self.tracks = tracks;
    }

    pub fn add_track(&mut self, track: &Track) {
        if self.track_ids.insert(track.id) {
            // TODO: Insert as sorted
            self.tracks.push(track.clone());
            self.tracks
                .sort_by(|track_a, track_b| track_a.name.cmp(&track_b.name));
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
        // if the selected track is one that is playing, pause it.
        if self
            .context
            .borrow()
            .playback
            .selected_track
            .as_ref()
            .is_some_and(
                |SelectedTrackContext {
                     index: playing_index,
                     track: playing_track,
                     playing: _,
                 }| { (*playing_index == row_index) && (*playing_track == *track) },
            )
        {
            if let Err(err) = self.channels.player_command_tx.send(PlayerCommand::Toggle) {
                error!(
                    "Failed to toggle track state on path {:?}: {}",
                    track.path, err
                );
            }

            return;
        }

        let volume = self.config.borrow().volume.default;

        if let Err(err) = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(track.clone(), volume))
        {
            error!("Failed to start track on path {:?}: {}", track.path, err);
        }

        let new_track_context = SelectedTrackContext::new(track.clone(), row_index, true);

        // If we're currently in the context of a playlist
        let selected_playlist = self.context.borrow().ui_playlist.selected();
        // Set autoplay to the playlist we're in (if any)
        self.context
            .borrow_mut()
            .ui_playlist
            .set_autoplay(selected_playlist.clone());

        if let Some(playlist) = selected_playlist {
            let playlist_state = PlaylistState::new(playlist, self.tracks.clone());

            self.context
                .borrow_mut()
                .playback
                .selected_playlist
                .set_playlist(Some(playlist_state));
        } else {
            self.context
                .borrow_mut()
                .playback
                .selected_playlist
                .set_playlist(None);
        }

        self.context
            .borrow_mut()
            .playback
            .set_selected_track(Some(new_track_context));
    }

    /// Selects the next track from the track table tracks attribute
    fn select_new_track(&mut self) {
        let track_context = {
            let context = self.context.borrow();

            let Some(track) = &context.playback.selected_track else {
                return;
            };

            if !context.playback.select_new_track() {
                return;
            }

            Some(track.clone())
        };

        let mut context = self.context.borrow_mut();

        // If a button for playback control (forward/backward) was pressed, select that instead of autoplay
        let autoplay_selector =
            if let Some(controlled_autoplay) = context.playback.consume_controlled_autoplay() {
                controlled_autoplay
            } else {
                context.playback.autoplay.autoplay().to_owned()
            };

        context.playback.set_select_new_track(false);

        let tracks = if let Some(playlist_state) = &context.playback.selected_playlist.playlist() {
            &playlist_state.tracks()
        } else {
            &self.tracks
        };

        let Some(index) = track_context.and_then(|track_context| {
            tracks
                .iter()
                .position(|track| track.hash == track_context.track.hash)
        }) else {
            // Could not find a new track to play, clearing sink
            let _ = self.channels.player_command_tx.send(PlayerCommand::Clear);
            context.playback.set_selected_track(None);

            return;
        };

        // Only add a track once it's finished autoplaying, and we're selecting the next track to autoplay
        context.playback.selected_playlist.add_played_track(index);

        let tracks_len = tracks.len();

        // TODO: Configurable default autoplay direction
        let new_index = match autoplay_selector {
            AutoplayType::Iterative(play_direction) => match play_direction {
                PlayDirection::Forward => (index + 1) % tracks_len,
                PlayDirection::Backward => (index + tracks_len.saturating_sub(1)) % tracks_len,
            },
            AutoplayType::Shuffle(shuffle_type) => match shuffle_type {
                // TODO: There's the possibility of indices being offset during tracks being added to playlist(s)
                ShuffleType::PseudoRandom => {
                    let played_tracks = context.playback.selected_playlist.played_tracks();

                    if let Some(filtered_index) = filtered_random_index(tracks_len, &played_tracks)
                    {
                        filtered_index
                    } else {
                        debug!("All tracks have been in the Pseudo random shuffler -- resetting");

                        context.playback.selected_playlist.clear_played_tracks();

                        let mut rng = rand::rng();
                        rng.random_range(0..tracks_len)
                    }
                }
                ShuffleType::TrueRandom => {
                    let mut rng = rand::rng();
                    rng.random_range(0..tracks_len)
                }
            },
        };

        // TODO: Configurable value to autoplay from filtered tracks
        let Some(new_track) = tracks.get(new_index) else {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Clear);
            context.playback.set_selected_track(None);

            return;
        };

        let volume = self.config.borrow().volume.default;

        let _ = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(new_track.clone(), volume));

        let new_track_context = SelectedTrackContext::new(new_track.clone(), new_index, true);

        debug!("Selected new track with autoplay: {:?}", new_track_context);

        context.playback.set_selected_track(Some(new_track_context));

        self.scroll_to_selected = true;
    }

    fn table_body_row(&mut self, mut row: TableRow<'_, '_>) {
        let row_index = row.index();

        let track = self.filtered_tracks.get(row_index).cloned();
        let playing = self.context.borrow().playback.selected_track.clone();

        if let Some(track) = track {
            row.set_selected(playing.as_ref().is_some_and(
                |SelectedTrackContext {
                     index: _,
                     track:
                         Track {
                             id: _,
                             path: _,
                             name: _,
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
                    .on_hover_cursor(CursorIcon::Default);
                if label.double_clicked() {
                    self.toggle_row_play(row_index, &track);
                }
            });

            row.col(|ui| {
                let label = ui.label(&track.name).on_hover_cursor(CursorIcon::Default);
                if label.double_clicked() {
                    self.toggle_row_play(row_index, &track);
                }
            });

            row.col(|ui| {
                let track_duration = Duration::from_secs_f64(track.duration_secs);
                let readable_track_duration = human_duration(track_duration, false);

                let label = ui
                    .label(readable_track_duration)
                    .on_hover_cursor(CursorIcon::Default);

                if label.double_clicked() {
                    self.toggle_row_play(row_index, &track);
                }
            });

            let response = row.response();

            if response.double_clicked() {
                self.toggle_row_play(row_index, &track);
            }
            // else if row.response().clicked() && shift_hit {}
        }
    }

    fn ui_table(&mut self, ui: &mut egui::Ui, height: f32) {
        self.select_new_track();

        // let shift_hit = ui.ctx().input(|i| i.modifiers.shift);

        let mut table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(INDEX_COLUMN_WIDTH).resizable(true))
            .column(Column::remainder())
            .column(Column::auto().at_least(DURATION_COLUMN_WIDTH))
            .sense(egui::Sense::click());

        if self.scroll_to_selected {
            if let Some(playing_track) = &self.context.borrow().playback.selected_track {
                // TODO: Auto-scroll alignment configuration
                let autoplay_align = self.config.borrow().autoplay.align_scroll;
                table = table.scroll_to_row(playing_track.index, autoplay_align);
                self.scroll_to_selected = false;
            }
        }

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

                body.rows(TABLE_ROW_HEIGHT, num_rows, |row| {
                    self.table_body_row(row);
                });
            });
    }

    fn ui_search(&mut self, ui: &mut egui::Ui) {
        if self.search.text.is_empty() {
            self.filtered_tracks = self.tracks.clone();
        }
        // Only recalculate the filtered tracks if the search input has changed
        // and the previous search yielded some results

        // TODO
        // Keep track of previous search that yielded result such that if we
        // delete characters to go back to that same search length, then add more characters,
        // calculation of filtered tracks will begin again
        else if self.search.changed && !self.filtered_tracks.is_empty() {
            let search_lower = self.search.text.to_lowercase();

            let start = Instant::now();

            let filtered_tracks: Vec<Track> = self
                .tracks
                .iter()
                .filter_map(|track| {
                    if track.name.to_lowercase().contains(&search_lower) {
                        Some(track.clone())
                    } else {
                        None
                    }
                })
                .collect();

            let duration = start.elapsed();
            self.search.duration = Some(duration);
            debug!(
                "Filtered into {} tracks in {:?}",
                filtered_tracks.len(),
                duration
            );

            self.filtered_tracks = filtered_tracks;
        }

        let search_text_edit =
            egui::TextEdit::singleline(&mut self.search.text).hint_text("Search...");

        let response = ui.add(search_text_edit);

        if self.search.focus_requested {
            response.request_focus();
            self.search.focus_requested = false;
        }

        self.search.changed = response.changed();
        self.search.focused = response.has_focus();
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let height = ui.available_height();

        ui.vertical(|ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                self.ui_search(ui);

                if let Some(search_duration) = self.search.duration {
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
