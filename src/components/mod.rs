pub mod menu_bar;
pub mod modals;
pub mod playback;
pub mod popups;
pub mod tables;
pub mod utils;

use std::{fmt, rc::Rc};

use crossbeam::channel::Sender;
use egui_dock::{DockState, NodeIndex, TabViewer};
use popups::settings::Settings;
use tables::{playlists::PlaylistTable, tags::TagTable, tasks::TaskTable, tracks::TrackTable};

use crate::{
    components::{
        menu_bar::MenuBar, modals::create_playlist::CreatePlaylistModal, playback::PlaybackBar,
    },
    config::core::SharedConfig,
    context::SharedContext,
    database::connection::DatabaseCommand,
    playback::state::{PlayerCommand, PlayerEvent},
};

pub enum ComponentTab {
    Playlists,
    Tracks,
    Tags,
    Tasks,
}

impl fmt::Display for ComponentTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ComponentTab::Playlists => "Playlists",
            ComponentTab::Tracks => "Tracks",
            ComponentTab::Tags => "Tags",
            ComponentTab::Tasks => "Tasks",
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
    pub playback_bar: PlaybackBar,

    pub playlist_table: PlaylistTable,
    pub track_table: TrackTable,
    pub tag_table: TagTable,
    pub task_table: TaskTable,

    pub settings: Settings,
    pub create_playlist: CreatePlaylistModal,

    current_player_event: Option<PlayerEvent>,
}

impl Components {
    pub fn new(
        config: SharedConfig,
        context: SharedContext,
        channels: Rc<ComponentChannels>,
    ) -> Self {
        Self {
            top_menu_bar: MenuBar::new(context.clone()),
            playback_bar: PlaybackBar::new(config.clone(), context.clone(), channels.clone()),

            playlist_table: PlaylistTable::new(context.clone(), channels.clone()),
            track_table: TrackTable::new(config.clone(), context.clone(), channels.clone()),
            tag_table: TagTable::default(),
            task_table: TaskTable::default(),

            create_playlist: CreatePlaylistModal::new(context.clone(), channels.clone()),
            settings: Settings::new(context.clone()),

            current_player_event: None,
        }
    }

    pub fn component_tab_layout(&self) -> DockState<ComponentTab> {
        let mut dock_state = DockState::new(vec![
            ComponentTab::Tracks,
            ComponentTab::Tags,
            ComponentTab::Tasks,
        ]);

        let surface = dock_state.main_surface_mut();

        let [_, _] = surface.split_left(NodeIndex::root(), 0.20, vec![ComponentTab::Playlists]);

        dock_state
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
            ComponentTab::Tags => {
                self.tag_table.ui(ui);
            }
            ComponentTab::Tasks => {
                self.task_table.ui(ui);
            }
        }
    }
}
