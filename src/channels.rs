use crossbeam::channel::{Receiver, Sender};

use crate::{
    database::connection::{DatabaseCommand, DatabaseEvent},
    playback::state::{PlayerCommand, PlayerEvent},
};

#[derive(Debug, Clone)]
pub struct Channels {
    pub database_command_tx: Sender<DatabaseCommand>,
    pub database_event_rx: Receiver<DatabaseEvent>,
    pub player_command_tx: Sender<PlayerCommand>,
    pub player_event_rx: Receiver<PlayerEvent>,
}

impl Channels {
    pub fn new(
        database_command_tx: Sender<DatabaseCommand>,
        database_event_rx: Receiver<DatabaseEvent>,
        player_command_tx: Sender<PlayerCommand>,
        player_event_rx: Receiver<PlayerEvent>,
    ) -> Self {
        Self {
            database_command_tx,
            database_event_rx,
            player_command_tx,
            player_event_rx,
        }
    }
}
