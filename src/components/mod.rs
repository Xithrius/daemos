pub mod menu_bar;
pub mod playback;
pub mod settings;
pub mod tables;
pub mod utils;

use std::{fmt, rc::Rc};

use crossbeam::channel::Sender;
use egui_dock::TabViewer;
use tables::{playlists::PlaylistTable, tracks::TrackTable};

use crate::{
    components::{menu_bar::MenuBar, playback::PlaybackBar, settings::Settings},
    config::core::CoreConfig,
    context::SharedContext,
    database::connection::DatabaseCommand,
    playback::state::{PlayerCommand, PlayerEvent},
};

pub enum ComponentTab {
    Playlists,
    Tracks,
}

impl fmt::Display for ComponentTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ComponentTab::Playlists => "Playlists",
            ComponentTab::Tracks => "Tracks",
        };

        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone)]
pub struct ComponentChannels {
    database_command_tx: Sender<DatabaseCommand>,
    player_command_tx: Sender<PlayerCommand>,
}

impl ComponentChannels {
    pub fn new(
        database_command_tx: Sender<DatabaseCommand>,
        player_command_tx: Sender<PlayerCommand>,
    ) -> Self {
        Self {
            database_command_tx,
            player_command_tx,
        }
    }
}

pub struct Components {
    pub top_menu_bar: MenuBar,
    pub track_table: TrackTable,
    pub playlist_table: PlaylistTable,
    pub playback_bar: PlaybackBar,
    pub settings: Settings,

    current_player_event: Option<PlayerEvent>,
}

impl Components {
    pub fn new(
        config: CoreConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        Self {
            top_menu_bar: MenuBar::new(context.clone()),
            track_table: TrackTable::new(context.clone(), channels.clone()),
            playlist_table: PlaylistTable::new(context.clone(), channels.clone()),
            playback_bar: PlaybackBar::new(&config, context.clone(), channels),
            settings: Settings::new(config, context),
            current_player_event: None,
        }
    }

    pub fn maybe_current_player_event(&mut self, player_event: Option<PlayerEvent>) {
        self.current_player_event = player_event;
    }
}

impl TabViewer for Components {
    type Tab = ComponentTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            ComponentTab::Playlists => {
                self.playlist_table.ui(ui);
            }
            ComponentTab::Tracks => {
                self.track_table.ui(ui, &self.current_player_event);
            }
        }
    }
}
