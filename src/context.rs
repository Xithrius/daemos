use std::{cell::RefCell, rc::Rc};

use crossbeam::channel::{Receiver, Sender};
use egui::{Key, KeyboardShortcut, Modifiers, Separator};
use serde::Serialize;
use tracing::{debug, error};

use crate::{
    components::{Components, playback::PLAYBACK_BAR_HEIGHT},
    config::core::CoreConfig,
    database::{
        connection::{Database, SharedDatabase},
        models::tracks::Track,
    },
    files::open::{get_tracks, select_folders_dialog},
    playback::state::{PlayerCommand, PlayerEvent},
    vertical_separator,
};

#[derive(Serialize)]
pub struct Context {
    config: CoreConfig,

    #[serde(skip)]
    #[allow(dead_code)]
    database: SharedDatabase,

    #[serde(skip)]
    player_cmd_tx: Sender<PlayerCommand>,
    #[serde(skip)]
    player_event_rx: Receiver<PlayerEvent>,

    #[serde(skip)]
    components: Components,
}

impl Context {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        config: CoreConfig,
        database: Database,
        player_cmd_tx: Sender<PlayerCommand>,
        player_event_rx: Receiver<PlayerEvent>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let shared_database = Rc::new(RefCell::new(database));

        let components = Components::new(
            config.clone(),
            shared_database.clone(),
            player_cmd_tx.clone(),
        );

        Self {
            config,
            database: shared_database.clone(),
            player_cmd_tx,
            player_event_rx,
            components,
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

        #[cfg(debug_assertions)]
        if ctx.input(|i| i.key_pressed(Key::F3)) {
            self.config.debug = !self.config.debug;

            if self.config.debug != ctx.debug_on_hover() {
                ctx.set_debug_on_hover(self.config.debug);
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

                let total_new_tracks = match Track::insert_many(self.database.clone(), tracks) {
                    Err(err) => {
                        error!("Failed to insert tracks into database: {}", err);
                        None
                    }
                    Ok(total_inserted) => Some(total_inserted),
                };

                if total_new_tracks.is_some_and(|new_tracks| new_tracks > 0) {
                    if let Err(err) = self
                        .components
                        .track_table
                        .refresh_tracks(self.database.clone())
                    {
                        error!("Failed to refresh tracks on track table: {}", err);
                    }
                } else {
                    debug!("Skipping refresh, no new tracks found");
                }
            }
        }

        if ctx.input(|i| i.key_pressed(Key::Space)) {
            let _ = self.player_cmd_tx.send(PlayerCommand::Toggle);
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
