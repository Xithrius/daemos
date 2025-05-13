pub mod menu_bar;
pub mod playback;
pub mod settings;
pub mod track_table;
pub mod tree;

use crossbeam::channel::Sender;

use crate::{
    components::{
        menu_bar::MenuBar, playback::PlaybackBar, settings::Settings, track_table::TrackTable, tree::Tree,
    },
    config::core::CoreConfig,
    database::connection::SharedDatabase,
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
        shared_database: SharedDatabase,
        player_cmd_tx: Sender<PlayerCommand>,
    ) -> Self {
        Self {
            top_menu_bar: MenuBar::default(),
            track_table: TrackTable::new(shared_database, player_cmd_tx.clone()),
            playlist_tree: Tree::default(),
            playback_bar: PlaybackBar::new(&config, player_cmd_tx),
            settings: Settings::new(config),
        }
    }
}
