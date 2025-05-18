use std::rc::Rc;

use egui::{Key, KeyboardShortcut, Modifiers, Separator};
use tracing::{debug, error};

use crate::{
    channels::Channels,
    components::{ComponentChannels, Components, playback::PLAYBACK_BAR_HEIGHT},
    config::core::CoreConfig,
    context::SharedContext,
    database::connection::{DatabaseCommand, DatabaseEvent},
    files::open::{get_tracks, select_folders_dialog},
    playback::state::PlayerCommand,
    vertical_separator,
};

pub struct App {
    config: CoreConfig,
    channels: Rc<Channels>,
    components: Components,
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
        let components = Components::new(config.clone(), component_channels);

        Self {
            config,
            components,
            channels,
        }
    }

    fn handle_database_events(&mut self) {
        let Some(database_event) = self.channels.database_event_rx.try_recv().ok() else {
            return;
        };

        // debug!("UI received database event: {:?}", database_event);

        match database_event {
            DatabaseEvent::InsertTracks(new_tracks) => {
                for track in new_tracks {
                    self.components.track_table.add_track(&track);
                }
            }
            DatabaseEvent::QueryAllTracks(tracks) => match tracks {
                Ok(tracks) => self.components.track_table.set_tracks(tracks),
                Err(err) => {
                    error!("Error when querying track table: {}", err);
                }
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
                    .channels
                    .database_command_tx
                    .send(DatabaseCommand::InsertTracks(tracks))
                {
                    error!("Failed to send insert tracks command to database: {}", err);
                }
            }
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

        self.handle_database_events();
        self.handle_keybinds(ctx);

        if ctx.input(|i| i.key_pressed(Key::Space)) && !self.components.track_table.search_focused()
        {
            let _ = self.channels.player_command_tx.send(PlayerCommand::Toggle);
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

        let select_next_track = self.components.playback_bar.find_next_track_mut();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    self.components.playlist_tree.ui(ui);
                    vertical_separator!(ui);

                    self.components
                        .track_table
                        .ui(ui, &player_event, select_next_track);
                });
            });
        });

        self.components.settings.ui(ctx);
    }
}
