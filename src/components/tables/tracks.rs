use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use egui::CursorIcon;
use egui_extras::{Column, TableBuilder, TableRow};
use rand::Rng;
use tracing::{debug, error};

use super::{TABLE_HEADER_HEIGHT, TABLE_ROW_HEIGHT};
use crate::{
    components::ComponentChannels,
    config::{
        core::SharedConfig,
        search::{MatcherFn, SearchMatchingStrategy},
    },
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

pub struct TrackSearch {
    pub text: String,
    pub previous_text: Option<String>,
    pub changed: bool,
    pub focused: bool,
    pub focus_requested: bool,
    pub duration: Option<Duration>,
    pub yielded_results: usize,
    pub matcher: SearchMatchingStrategy,
    pub matcher_fn: MatcherFn,
}

impl Default for TrackSearch {
    fn default() -> Self {
        Self {
            text: String::new(),
            previous_text: None,
            changed: false,
            focused: false,
            focus_requested: false,
            duration: None,
            yielded_results: 0,
            matcher: SearchMatchingStrategy::default(),
            matcher_fn: Box::new(|_, _| false),
        }
    }
}

impl TrackSearch {
    pub fn new(config: SharedConfig) -> Self {
        let strategy = &config.borrow().search.strategy;

        Self {
            matcher: strategy.clone(),
            matcher_fn: strategy.get_matcher(),
            ..Default::default()
        }
    }
}

pub struct TrackTable {
    config: SharedConfig,
    context: SharedContext,
    channels: Rc<ComponentChannels>,
    search: TrackSearch,
    scroll_to_selected: bool,
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

        let search = TrackSearch::new(config.clone());

        Self {
            config,
            context,
            channels,
            search,
            scroll_to_selected: false,
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

        let volume = self.config.borrow().playback.volume;

        if let Err(err) = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(track.clone(), volume))
        {
            error!("Failed to start track on path {:?}: {}", track.path, err);
        }

        let new_track_context = SelectedTrackContext::new(track.clone(), row_index, true);

        // If we're currently in the context of a playlist
        let selected_playlist = self.context.borrow().ui.playlist.selected();
        // Set autoplay to the playlist we're in (if any)
        self.context
            .borrow_mut()
            .ui
            .playlist
            .set_autoplay(selected_playlist.clone());

        if let Some(playlist) = selected_playlist {
            let tracks = self
                .context
                .borrow()
                .storage
                .get_playlist_tracks(Some(&playlist))
                .map(|tracks| tracks.to_owned())
                .unwrap_or_default();
            let playlist_state = PlaylistState::new(playlist, tracks);

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
            .select_track(Some(new_track_context));
    }

    /// Selects the next track from the track table tracks attribute
    // TODO: Move handling selection of the new track into the playback context
    fn select_new_track(&mut self) {
        let track_context = {
            let context = self.context.borrow();

            let Some(track) = &context.playback.selected_track else {
                return;
            };

            if !context.playback.autoplay.select_new_track() {
                return;
            }

            Some(track.clone())
        };

        let mut context = self.context.borrow_mut();

        // If a button for playback control (forward/backward) was pressed, select that instead of autoplay
        let mut controlled = false;
        let autoplay_selector =
            if let Some(controlled_autoplay) = context.playback.autoplay.consume_controlled() {
                controlled = true;
                controlled_autoplay
            } else {
                context.playback.autoplay.autoplay().to_owned()
            };

        context.playback.autoplay.set_select_new_track(false);

        let tracks = if let Some(playlist_state) = &context.playback.selected_playlist.playlist() {
            &playlist_state.tracks()
        } else {
            &context
                .storage
                .get_playlist_tracks(None)
                .map(|tracks| tracks.to_owned())
                .unwrap_or_default()
        };

        let Some(index) = track_context.and_then(|track_context| {
            tracks
                .iter()
                .position(|track| track.hash == track_context.track.hash)
        }) else {
            // Could not find a new track to play, clearing sink
            let _ = self.channels.player_command_tx.send(PlayerCommand::Clear);
            context.playback.select_track(None);

            return;
        };

        // Only add a track once it's finished autoplaying, and we're selecting the next track to autoplay
        // or if the user intentionally skipped and the config is set to adding the seen track on skip
        if !controlled || self.config.borrow().playback.add_to_seen_on_skip {
            context.playback.selected_playlist.add_played_track(index);
        }

        let tracks_len = tracks.len();

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
            context.playback.select_track(None);

            return;
        };

        let volume = self.config.borrow().playback.volume;

        let _ = self
            .channels
            .player_command_tx
            .send(PlayerCommand::Create(new_track.clone(), volume));

        let new_track_context = SelectedTrackContext::new(new_track.clone(), new_index, true);

        debug!("Selected new track with autoplay: {:?}", new_track_context);

        context.playback.select_track(Some(new_track_context));

        self.scroll_to_selected = true;
    }

    fn table_body_row(&mut self, mut row: TableRow<'_, '_>, track: &Track) {
        let row_index = row.index();

        let playing = {
            let context = self.context.borrow();
            context.playback.selected_track.clone()
        };

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
                self.toggle_row_play(row_index, track);
            }
        });

        row.col(|ui| {
            let label = ui.label(&track.name).on_hover_cursor(CursorIcon::Default);
            if label.double_clicked() {
                self.toggle_row_play(row_index, track);
            }
        });

        row.col(|ui| {
            let track_duration = Duration::from_secs_f64(track.duration_secs);
            let readable_track_duration = human_duration(track_duration, false);

            let label = ui
                .label(readable_track_duration)
                .on_hover_cursor(CursorIcon::Default);

            if label.double_clicked() {
                self.toggle_row_play(row_index, track);
            }
        });

        let response = row.response();

        if response.double_clicked() {
            self.toggle_row_play(row_index, track);
        }
    }

    fn ui_table(&mut self, ui: &mut egui::Ui, height: f32) {
        self.select_new_track();

        let mut table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(INDEX_COLUMN_WIDTH).resizable(true))
            .column(Column::remainder())
            .column(Column::auto().at_least(DURATION_COLUMN_WIDTH))
            .sense(egui::Sense::click());

        let (filtered_tracks, selected_track, align_scroll) = {
            let context = self.context.borrow();
            let selected_playlist = context.ui.playlist.selected();

            let filtered_tracks = context
                .storage
                .filtered_tracks(selected_playlist.as_ref())
                .to_vec();

            let selected = context.playback.selected_track.clone();
            let align = self.config.borrow().ui.align_scroll;
            (filtered_tracks, selected, align)
        };

        // If the currently selected playlist in the UI matches the one that has a track playing
        let synced_selected_playlist = {
            let context = self.context.borrow();

            let ui_selected_playlist = context.ui.playlist.selected();
            let autoplay_selected_playlist = context
                .playback
                .selected_playlist
                .playlist()
                .map(|playlist_state| playlist_state.playlist());

            ui_selected_playlist == autoplay_selected_playlist
        };

        if self.scroll_to_selected {
            if let Some(playing_track) = selected_track
                && synced_selected_playlist
            {
                table = table.scroll_to_row(playing_track.index, align_scroll);
            }

            self.scroll_to_selected = false;
        }

        let num_rows = filtered_tracks.len();

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
                body.rows(TABLE_ROW_HEIGHT, num_rows, |row| {
                    let index = row.index();
                    let Some(track) = filtered_tracks.get(index) else {
                        return;
                    };

                    self.table_body_row(row, track);
                });
            });
    }

    // TODO
    // Keep track of previous search that yielded result such that if we
    // delete characters to go back to that same search length, then add more characters,
    // calculation of filtered tracks will begin again
    fn ui_search(&mut self, ui: &mut egui::Ui) {
        let mut outdated_search_strategy = false;
        {
            let context = self.context.borrow();
            let current_matcher = context.ui.search.matcher();

            if self.search.matcher != *current_matcher {
                self.search.matcher = current_matcher.clone();
                self.search.matcher_fn = self.config.borrow().search.strategy.get_matcher();
                outdated_search_strategy = true;
            }
        }

        let selected_playlist = { self.context.borrow().ui.playlist.selected() };

        let mut context = self.context.borrow_mut();
        let storage_context = &mut context.storage;

        let previous_text = self.search.text.clone();

        let search_text_edit =
            egui::TextEdit::singleline(&mut self.search.text).hint_text("Search...");

        let response = ui.add(search_text_edit);

        if self.search.focus_requested {
            response.request_focus();
            self.search.focus_requested = false;
        }

        self.search.changed = response.changed();
        self.search.focused = response.has_focus();

        // If the current search doesn't have the same input as the previous frame, or
        // the search box isn't empty and the search strategy was changed,
        // then the filtered tracks vector for the respective playlist are updated
        if self.search.text != previous_text
            || (!self.search.text.is_empty() && outdated_search_strategy)
        {
            let start = Instant::now();

            let search_text = self.search.text.clone();

            let matcher = &self.search.matcher_fn;

            let predicate =
                |track: &Track| search_text.is_empty() || (matcher)(&search_text, &track.name);

            if let Some(filtered) = storage_context.filter_with(&selected_playlist, predicate) {
                self.search.yielded_results = filtered.len();
            }

            let duration = start.elapsed();
            self.search.duration = Some(duration);
        }

        self.search.previous_text = Some(previous_text);

        if !self.search.text.is_empty()
            && let Some(search_duration) = self.search.duration
        {
            ui.label(format!(
                "{} results in {:?}",
                self.search.yielded_results, search_duration
            ));
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let height = ui.available_height();

        ui.vertical(|ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                self.ui_search(ui);
            });

            ui.separator();

            self.ui_table(ui, height);
        });
    }
}
