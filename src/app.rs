use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use egui::{Frame, Key, KeyboardShortcut, Modifiers};
use egui_dock::{DockArea, DockState};
use tracing::{debug, error};

use crate::{
    channels::Channels,
    components::{ComponentChannels, ComponentTab, Components, playback::PLAYBACK_BAR_HEIGHT},
    config::core::SharedConfig,
    context::SharedContext,
    database::connection::{DatabaseCommand, DatabaseError, DatabaseEvent},
    files::open::{get_folder_tracks, select_file_dialog, select_folders_dialog},
    playback::state::PlayerCommand,
};

pub struct App {
    config: SharedConfig,
    context: SharedContext,
    channels: Rc<Channels>,
    components: Components,
    dock_state: DockState<ComponentTab>,
}

impl App {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        config: SharedConfig,
        channels: Rc<Channels>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let context = SharedContext::default();

        let component_channels = Rc::new(ComponentChannels::new(
            channels.database_command_tx.clone(),
            channels.player_command_tx.clone(),
        ));
        let components = Components::new(config.clone(), context.clone(), component_channels);
        let dock_state = components.component_tab_layout();

        Self {
            config,
            context,
            channels,
            components,
            dock_state,
        }
    }

    fn handle_database_event_error(&mut self, err: DatabaseError) {
        match err {
            DatabaseError::DuplicateTrack(_track) => {
                self.context.borrow_mut().processing.decrement(None);
            }
            DatabaseError::DuplicatePlaylistTrack(_track, playlist) => {
                self.context
                    .borrow_mut()
                    .processing
                    .decrement(Some(playlist.name));
            }
            DatabaseError::DuplicatePlaylist => {
                todo!();
            }
            DatabaseError::DatabaseUnavailable => {
                todo!();
            }
            DatabaseError::Unknown => {
                todo!();
            }
        }
    }

    fn handle_database_events(&mut self) {
        let Some(database_event_result) = self.channels.database_event_rx.try_recv().ok() else {
            return;
        };

        // debug!("UI received database event: {:?}", database_event);

        let database_event = match database_event_result {
            Ok(database_event) => database_event,
            Err(err) => {
                self.handle_database_event_error(err);
                return;
            }
        };

        match database_event {
            DatabaseEvent::InsertTrack(track, playlist) => {
                let mut context = self.context.borrow_mut();

                context
                    .storage
                    .add_tracks_to_playlist(playlist.as_ref(), vec![track]);

                let playlist_name = playlist.map(|playlist| playlist.name);
                context.processing.decrement(playlist_name);
            }
            DatabaseEvent::QueryTracks(tracks, playlist) => {
                let mut context = self.context.borrow_mut();
                let storage_context = &mut context.storage;
                storage_context.set_playlist_tracks(playlist, tracks);
            }
            DatabaseEvent::InsertPlaylist(playlist) => {
                let mut context = self.context.borrow_mut();
                let storage_context = &mut context.storage;
                storage_context.add_empty_playlist(&playlist);
            }
            DatabaseEvent::QueryPlaylists(playlists) => {
                let mut context = self.context.borrow_mut();
                let storage_context = &mut context.storage;
                for playlist in playlists {
                    storage_context.add_empty_playlist(&playlist);
                }
            }
        }
    }

    fn handle_keybinds(&mut self, ctx: &egui::Context) {
        // Debug wireframe
        #[cfg(debug_assertions)]
        if ctx.input(|i| i.key_pressed(Key::F3)) {
            debug!("`F3` Has been used to toggle the debug wireframe");

            let debug = self.config.borrow().general.debug_wireframe;
            self.config.borrow_mut().general.debug_wireframe = !debug;

            if debug != ctx.debug_on_hover() {
                ctx.set_debug_on_hover(debug);
            }
        }

        // TODO: If there's a bunch of input boxes, then this is going to get bad
        if ctx.input(|i| i.key_pressed(Key::Space))
                // TODO: Change to UI context
                && !self.components.track_table.search_focused()
                && !self.context.borrow().ui.visibility.playlist_modal()
        {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
            return;
        }

        // Open OS file explorer to select folder of tracks
        if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL | Modifiers::SHIFT,
                logical_key: Key::O,
            })
        }) {
            debug!(
                "`Ctrl + Shift + O` has been used to open OS file explorer for track folder selection"
            );

            if let Some(selected_folders) = select_folders_dialog() {
                for folder in selected_folders {
                    let folder_tracks = get_folder_tracks(&folder, false);

                    debug!(
                        "Found {} total track(s) in selected folders",
                        folder_tracks.len()
                    );

                    // TODO: Ask the user with a popup if a playlist should be created from this
                    let playlist_name = folder
                        .file_name()
                        .and_then(|file_name| file_name.to_str())
                        .map(|folder| folder.to_string());

                    self.context
                        .borrow_mut()
                        .processing
                        .add(playlist_name.clone(), folder_tracks.len());

                    let insert_tracks =
                        DatabaseCommand::InsertTracks(folder_tracks, playlist_name, None);

                    if let Err(err) = self.channels.database_command_tx.send(insert_tracks) {
                        error!("Failed to send insert tracks command to database: {}", err);
                    }
                }
            }
        }
        // Open OS file explorer to select a file as a track
        else if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::O,
            })
        }) {
            debug!("`Ctrl + O` has been used to open OS file explorer for track file selection");

            if let Some(selected_file) = select_file_dialog() {
                self.context.borrow_mut().processing.add(None, 1);

                let insert_tracks = DatabaseCommand::InsertTracks(vec![selected_file], None, None);

                if let Err(err) = self.channels.database_command_tx.send(insert_tracks) {
                    error!("Failed to send insert track command to database: {}", err);
                }
            }
        }
        // Focus on currently playing track
        else if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::E,
            })
        }) {
            debug!("`Ctrl + E` has been used to focus currently playing track in playlist");

            // TODO: Change to UI context
            self.components.track_table.set_scroll_to_selected(true);
        }
        // Focus search input box
        else if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::F,
            })
        }) {
            debug!("`Ctrl + F` has been used to focus user input for searching");

            // TODO: Change to UI context
            self.components.track_table.request_search_focus();
        }
        // Toggle settings popup window
        else if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::Comma,
            })
        }) {
            debug!("`Ctrl + ,` has been used to toggle the settings popup window");

            self.context.borrow_mut().ui.visibility.toggle_settings();
        }
    }

    fn check_search_matcher(&mut self) {
        let search_config = &self.config.borrow().search;
        self.context
            .borrow_mut()
            .ui
            .search
            .check_matcher(search_config);
    }

    fn handle_player_event_repaint(&mut self, ctx: &egui::Context) {
        let mut context = self.context.borrow_mut();

        // TODO: Are these repaints necessary?
        if let Ok(player_event) = self.channels.player_event_rx.try_recv() {
            context.playback.handle_player_event(player_event.clone());
            ctx.request_repaint();
        } else if context
            .playback
            .selected_track
            .as_ref()
            .is_some_and(|track| track.playing)
        {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.components.top_menu_bar.ui(ctx, ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.set_height(PLAYBACK_BAR_HEIGHT);

            self.components.playback_bar.ui(ui);
        });

        egui::CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                DockArea::new(&mut self.dock_state)
                    .show_close_buttons(false)
                    .show_leaf_collapse_buttons(false)
                    .show_secondary_button_hint(false)
                    .secondary_button_on_modifier(false)
                    .secondary_button_context_menu(false)
                    .show_leaf_close_all_buttons(false)
                    .draggable_tabs(false)
                    .show_inside(ui, &mut self.components);
            });

        self.components.settings.ui(ctx);
        self.components.debug.ui(ctx);
        self.components.create_playlist.ui(ctx);
    }
}

impl eframe::App for App {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let start = if self.context.borrow().ui.visibility.performance_debug() {
            Some(Instant::now())
        } else {
            None
        };

        ctx.request_repaint_after(Duration::from_millis(16));

        self.handle_database_events();
        self.handle_keybinds(ctx);
        self.check_search_matcher();
        self.handle_player_event_repaint(ctx);

        self.ui(ctx);

        if let Some(start) = start {
            let duration = start.elapsed();
            self.context
                .borrow_mut()
                .performance_metrics
                .add_render_latency(duration);
        }
    }
}
