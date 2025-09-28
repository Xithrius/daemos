use std::{path::PathBuf, rc::Rc};

use egui::{Id, Modal};
use tracing::error;

use crate::{
    components::{ComponentChannels, modals::UIModal},
    context::SharedContext,
    database::connection::DatabaseCommand,
    files::open::{get_file_name, get_folder_tracks, select_folders_dialog},
    utils::regex::RegexExtract,
};

const DEFAULT_PLAYLIST_MODAL_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];
const PLAYLIST_MODAL_ID: &str = "create_playlist_modal";

#[derive(Debug, Clone, Default)]

pub struct CreatePlaylistState {
    name: String,
    track_paths: Vec<PathBuf>,
    regex_match: String,
    regex_group: String,
    regex_extract: Option<RegexExtract>,
    example_output: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreatePlaylistModal {
    context: SharedContext,
    channels: Rc<ComponentChannels>,

    state: CreatePlaylistState,
}

impl UIModal for CreatePlaylistModal {
    fn visibility(&self) -> bool {
        self.context.borrow().ui.visibility.playlist_modal()
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.context
            .borrow_mut()
            .ui
            .visibility
            .set_playlist_modal(visibility);

        // TODO: Config to save state of modal
        if !visibility {
            self.state = CreatePlaylistState::default();
        }
    }
}

impl CreatePlaylistModal {
    pub fn new(context: SharedContext, channels: Rc<ComponentChannels>) -> Self {
        Self {
            context,
            channels,
            state: CreatePlaylistState::default(),
        }
    }

    pub fn playlist_name(&self) -> &String {
        &self.state.name
    }

    pub fn create_playlist(&mut self) {
        let new_playlist_name = self.state.name.clone().trim().to_string();

        if new_playlist_name.is_empty() {
            error!("Cannot create playlist with empty name");

            return;
        }

        if self.state.track_paths.is_empty() {
            if let Err(err) = self
                .channels
                .database_command_tx
                .send(DatabaseCommand::InsertPlaylist(new_playlist_name))
            {
                error!(
                    "Failed to send playlist command for new playlist: {:?}",
                    err
                );
            }

            return;
        }

        self.send_tracks();
    }

    pub fn select_files(&mut self) {
        if let Some(selected_folders) = select_folders_dialog() {
            for folder in selected_folders {
                let folder_tracks = get_folder_tracks(&folder, false);

                self.state.track_paths.extend(folder_tracks);
            }
        }
    }

    pub fn send_tracks(&self) {
        let tracks = &self.state.track_paths;

        if tracks.is_empty() {
            return;
        }

        let playlist_name = self.state.name.clone();
        self.context
            .borrow_mut()
            .processing
            .add(Some(playlist_name), tracks.len());

        let playlist_name = self.playlist_name().to_owned();

        let regex_extract = self
            .state
            .regex_extract
            .clone()
            .map(|regex_extract| regex_extract.extract());
        let insert_tracks =
            DatabaseCommand::InsertTracks(tracks.to_vec(), Some(playlist_name), regex_extract);

        if let Err(err) = self.channels.database_command_tx.send(insert_tracks) {
            error!("Failed to send insert tracks command to database: {}", err);
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        let show_modal = {
            let context = self.context.borrow();
            context.ui.visibility.playlist_modal()
        };

        if !show_modal {
            return;
        }

        let mut should_close = false;

        let modal = Modal::new(Id::new(PLAYLIST_MODAL_ID)).show(ctx, |ui| {
            ui.set_min_size(DEFAULT_PLAYLIST_MODAL_WINDOW_SIZE.into());

            ui.heading("New Playlist");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.state.name);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let folder_button = ui.button("Open folder");
                if folder_button.clicked() {
                    self.select_files();
                }

                let selected_text = format!("{} track(s) selected", self.state.track_paths.len());
                ui.label(selected_text);
            });

            ui.add_space(10.0);

            ui.vertical(|ui| {
                let mut apply_clicked = false;

                ui.horizontal(|ui| {
                    ui.label("Regex group match");
                    ui.text_edit_singleline(&mut self.state.regex_match);
                    ui.label("Group");
                    ui.text_edit_singleline(&mut self.state.regex_group);

                    apply_clicked = ui.button("Apply").clicked();
                });

                ui.horizontal(|ui| {
                    if let Some(example_output) = self.state.example_output.as_ref() {
                        let example_track_name = format!("Example track name: {example_output}");

                        ui.label(example_track_name);
                    }

                    if !apply_clicked {
                        return;
                    }

                    let Some(first_track) = self.state.track_paths.first() else {
                        return;
                    };

                    let Some(track_name) = get_file_name(first_track.to_path_buf()) else {
                        return;
                    };

                    let Ok(regex_group) = self.state.regex_group.parse::<usize>() else {
                        return;
                    };

                    let regex_extract =
                        RegexExtract::new(self.state.regex_match.clone(), regex_group).ok();
                    self.state.regex_extract = regex_extract.clone();

                    let Some(regex_extract) = regex_extract else {
                        return;
                    };

                    let Some(example_track) = regex_extract.extract_group(&track_name) else {
                        return;
                    };

                    self.state.example_output = Some(example_track);
                });
            });

            ui.separator();

            egui::Sides::new().show(
                ui,
                |_ui| {},
                |ui| {
                    if ui.button("Create").clicked() {
                        self.create_playlist();

                        should_close = true;
                    }

                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                },
            );
        });

        if modal.should_close() || should_close {
            self.set_visibility(false);
        }
    }
}
