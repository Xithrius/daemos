pub mod menu_bar;
pub mod playback;
pub mod settings;
pub mod table;
pub mod tree;
pub mod volume_bar;

use crossbeam::channel::Sender;

use crate::{
    components::{
        menu_bar::MenuBar, playback::PlaybackBar, settings::Settings, table::Table, tree::Tree,
    },
    config::core::CoreConfig,
    database::connection::SharedDatabase,
    playback::state::PlayerCommand,
};

pub struct Components {
    pub top_menu_bar: MenuBar,
    pub track_table: Table,
    pub playlist_tree: Tree,
    pub playback_bar: PlaybackBar,
    pub settings: Settings,
}

impl Components {
    pub fn new(
        config: CoreConfig,
        shared_database: SharedDatabase,
        tx: Sender<PlayerCommand>,
    ) -> Self {
        Self {
            top_menu_bar: MenuBar::default(),
            track_table: Table::new(shared_database, tx.clone()),
            playlist_tree: Tree::default(),
            playback_bar: PlaybackBar::new(tx),
            settings: Settings::new(config),
        }
    }
}
