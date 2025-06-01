use std::rc::Rc;

use egui::{Frame, Key, KeyboardShortcut, Modifiers};
use egui_dock::{DockArea, DockState};
use tracing::{debug, error};

use crate::{
    channels::Channels,
    components::{ComponentChannels, ComponentTab, Components, playback::PLAYBACK_BAR_HEIGHT},
    config::core::CoreConfig,
    context::SharedContext,
    database::connection::{DatabaseCommand, DatabaseEvent},
    files::open::{get_tracks, select_folders_dialog},
    playback::state::PlayerCommand,
};

pub struct App {
    config: CoreConfig,
    context: SharedContext,
    channels: Rc<Channels>,
    components: Components,
    dock_state: DockState<ComponentTab>,
}

impl App {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        config: CoreConfig,
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

    fn handle_database_events(&mut self) {
        let Some(database_event) = self.channels.database_event_rx.try_recv().ok() else {
            return;
        };

        // debug!("UI received database event: {:?}", database_event);

        match database_event {
            DatabaseEvent::InsertTrack(track) => {
                self.components.track_table.add_track(&track);
                self.context
                    .borrow_mut()
                    .processing
                    .finished_processing_track();
            }
            DatabaseEvent::QueryAllTracks(tracks) => match tracks {
                Ok(tracks) => self.components.track_table.set_tracks(tracks),
                Err(err) => {
                    error!("Error when querying track table: {}", err);
                }
            },
            DatabaseEvent::InsertPlaylist(playlist) => {
                self.components.playlist_table.add_playlist(&playlist);
            }
            DatabaseEvent::QueryAllPlaylists(playlists) => match playlists {
                Ok(playlists) => self.components.playlist_table.set_playlists(playlists),
                Err(err) => {
                    error!("Error when querying playlists table: {}", err);
                }
            },
        }
    }

    fn handle_keybinds(&mut self, ctx: &egui::Context) {
        // Debug wireframe
        #[cfg(debug_assertions)]
        if ctx.input(|i| i.key_pressed(Key::F3)) {
            debug!("`F3` Has been used to toggle the debug wireframe");

            self.config.general.debug = !self.config.general.debug;

            if self.config.general.debug != ctx.debug_on_hover() {
                ctx.set_debug_on_hover(self.config.general.debug);
            }
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
                    let folder_tracks = get_tracks(&folder, false);

                    debug!(
                        "Found {} total track(s) in selected folders",
                        folder_tracks.len()
                    );

                    self.context
                        .borrow_mut()
                        .processing
                        .set_processing_tracks(folder_tracks.len());

                    // TODO: Ask the user with a popup if a playlist should be created from this
                    let playlist_name = folder
                        .file_name()
                        .and_then(|file_name| file_name.to_str())
                        .map(|folder| folder.to_string());

                    let insert_tracks = DatabaseCommand::InsertTracks(folder_tracks, playlist_name);

                    if let Err(err) = self.channels.database_command_tx.send(insert_tracks) {
                        error!("Failed to send insert tracks command to database: {}", err);
                    }
                }
            }
        }

        // Focus search input box
        if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::F,
            })
        }) {
            debug!("`Ctrl + F` has been used to focus user input for searching");

            self.components.track_table.request_search_focus();
        }

        // Toggle settings popup window
        if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL,
                logical_key: Key::Comma,
            })
        }) {
            debug!("`Ctrl + ,` has been used to toggle the settings popup window");

            self.context.borrow_mut().ui.toggle_settings();
        }
    }
}

impl eframe::App for App {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // TODO: Is there a way around this?
        ctx.request_repaint_after(std::time::Duration::from_millis(16));

        let player_event = self.channels.player_event_rx.try_recv().ok();
        self.components
            .maybe_current_player_event(player_event.clone());

        self.handle_database_events();
        self.handle_keybinds(ctx);

        // TODO: If I have a bunch of input boxes, then this is going to get bad
        if ctx.input(|i| i.key_pressed(Key::Space))
            && !self.components.track_table.search_focused()
            && !self.context.borrow().ui.visible_playlist_modal()
        {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.components.top_menu_bar.ui(ctx, ui);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.set_height(PLAYBACK_BAR_HEIGHT);

            self.components.playback_bar.ui(ui, &player_event);
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
        self.components.create_playlist.ui(ctx);
    }
}
