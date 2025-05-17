pub mod menu_bar;
pub mod playback;
pub mod settings;
pub mod track_table;
pub mod tree;
pub mod utils;

use crossbeam::channel::Sender;

use crate::{
    components::{
        menu_bar::MenuBar, playback::PlaybackBar, settings::Settings, track_table::TrackTable,
        tree::Tree,
    },
    config::core::CoreConfig,
    database::connection::DatabaseCommand,
    playback::state::PlayerCommand,
};

pub struct Components {
    pub top_menu_bar: MenuBar,
    pub track_table: TrackTable,
    pub playlist_tree: Tree,
    pub playback_bar: PlaybackBar,
    pub settings: Settings,
}

impl Components {
    pub fn new(
        config: CoreConfig,
        database_command_tx: Sender<DatabaseCommand>,
        player_command_tx: Sender<PlayerCommand>,
    ) -> Self {
        Self {
            top_menu_bar: MenuBar::default(),
            track_table: TrackTable::new(database_command_tx, player_command_tx.clone()),
            playlist_tree: Tree::default(),
            playback_bar: PlaybackBar::new(&config, player_command_tx),
            settings: Settings::new(config),
        }
    }
}
