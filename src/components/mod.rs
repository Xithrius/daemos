pub mod menu_bar;
use crossbeam::channel::Sender;
use menu_bar::MenuBar;

pub mod playback;
use playback::PlaybackBar;

pub mod seek;
pub mod table;
use table::Table;

pub mod tree;
use tree::Tree;

use crate::{database::connection::SharedDatabase, playback::state::PlayerCommand};

pub struct Components {
    pub top_menu_bar: MenuBar,
    pub track_table: Table,
    pub playlist_tree: Tree,
    pub playback_bar: PlaybackBar,
}

impl Components {
    pub fn new(shared_database: SharedDatabase, tx: Sender<PlayerCommand>) -> Self {
        Self {
            top_menu_bar: Default::default(),
            track_table: Table::new(shared_database, tx.clone()),
            playlist_tree: Default::default(),
            playback_bar: PlaybackBar::new(tx),
        }
    }
}
