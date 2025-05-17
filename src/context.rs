use crossbeam::channel::{Receiver, Sender};
use egui::{Key, KeyboardShortcut, Modifiers, Separator};
use serde::Serialize;
use tracing::{debug, error};

use crate::{
    components::{Components, playback::PLAYBACK_BAR_HEIGHT},
    config::core::CoreConfig,
    database::connection::{DatabaseCommand, DatabaseEvent},
    files::open::{get_tracks, select_folders_dialog},
    playback::state::{PlayerCommand, PlayerEvent},
    vertical_separator,
};

#[derive(Serialize)]
pub struct Context {
    config: CoreConfig,

    #[serde(skip)]
    database_command_tx: Sender<DatabaseCommand>,
    #[serde(skip)]
    database_event_rx: Receiver<DatabaseEvent>,

    #[serde(skip)]
    player_command_tx: Sender<PlayerCommand>,
    #[serde(skip)]
    player_event_rx: Receiver<PlayerEvent>,

    #[serde(skip)]
    components: Components,
}

impl Context {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        config: CoreConfig,
        database_command_tx: Sender<DatabaseCommand>,
        database_event_rx: Receiver<DatabaseEvent>,
        player_command_tx: Sender<PlayerCommand>,
        player_event_rx: Receiver<PlayerEvent>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let components = Components::new(
            config.clone(),
            database_command_tx.clone(),
            player_command_tx.clone(),
        );

        Self {
            config,
            components,

            database_command_tx,
            database_event_rx,

            player_command_tx,
            player_event_rx,
        }
    }

    fn handle_database_events(&mut self) {
        let Some(database_event) = self.database_event_rx.try_recv().ok() else {
            return;
        };

        match database_event {
            DatabaseEvent::InsertTracks(new_tracks) => match new_tracks {
                Ok(new_track_amount) => {
                    if new_track_amount > 0 {
                        // Refresh the track list
                        let _ = self
                            .database_command_tx
                            .send(DatabaseCommand::QueryAllTracks);
                    }
                }
                Err(err) => {
                    error!("Error when inserting tracks: {}", err);
                }
            },
            DatabaseEvent::QueryAllTracks(tracks) => match tracks {
                Ok(tracks) => self.components.track_table.set_tracks(tracks),
                Err(_) => todo!(),
            },
        }
    }

    fn handle_keybinds(&mut self, ctx: &egui::Context) {
        #[cfg(debug_assertions)]
        if ctx.input(|i| i.key_pressed(Key::F3)) {
            self.config.general.debug = !self.config.general.debug;

            if self.config.general.debug != ctx.debug_on_hover() {
                ctx.set_debug_on_hover(self.config.general.debug);
            }
        }

        if ctx.input_mut(|i| {
            i.consume_shortcut(&KeyboardShortcut {
                modifiers: Modifiers::CTRL | Modifiers::SHIFT,
                logical_key: Key::O,
            })
        }) {
            if let Some(selected_folders) = select_folders_dialog() {
                let mut tracks = Vec::new();

                for folder in selected_folders {
                    let folder_tracks = get_tracks(&folder);
                    tracks.extend(folder_tracks);
                }

                debug!("Found {} total track(s) in selected folders", tracks.len());

                if let Err(err) = self
                    .database_command_tx
                    .send(DatabaseCommand::InsertTracks(tracks))
                {
                    error!("Failed to send insert tracks command to database: {}", err);
                }
            }
        }
    }
}

impl eframe::App for Context {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let player_event = self.player_event_rx.try_recv().ok();

        self.handle_keybinds(ctx);
        self.handle_database_events();

        if ctx.input(|i| i.key_pressed(Key::Space)) && !self.components.track_table.search_focused()
        {
            let _ = self.player_command_tx.send(PlayerCommand::Toggle);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.components
                    .top_menu_bar
                    .ui(ctx, ui, self.components.settings.visible_mut());
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.set_height(PLAYBACK_BAR_HEIGHT);

            self.components.playback_bar.ui(ui, &player_event);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    self.components.playlist_tree.ui(ui);
                    vertical_separator!(ui);

                    self.components.track_table.ui(ui, &player_event);
                });
            });
        });

        self.components.settings.ui(ctx);
    }
}
