use std::{path::PathBuf, thread};

use color_eyre::Result;
use crossbeam::channel::{Receiver, Sender, unbounded};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use super::{local::get_database_storage_path, models::tracks::Track};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DatabaseCommand {
    InsertTracks(Vec<PathBuf>),
    QueryAllTracks,
}

#[derive(Debug)]
pub enum DatabaseEvent {
    InsertTracks(Vec<Track>),
    QueryAllTracks(Result<Vec<Track>>),
}

#[derive(Debug)]
pub struct Database;

impl Database {
    pub fn start() -> (Sender<DatabaseCommand>, Receiver<DatabaseEvent>) {
        let database_path = get_database_storage_path().expect("Failed to get DB path");

        let (command_tx, command_rx) = unbounded();
        let (event_tx, event_rx) = unbounded();

        thread::spawn(move || {
            let mut conn =
                Connection::open(&database_path).expect("Failed to open database connection");

            if let Err(err) = Database::create_tables(&mut conn) {
                error!("Failed to create tables: {}", err);
                std::process::exit(1);
            }

            info!(
                "Database thread running with connection at {:?}",
                database_path
            );

            while let Ok(cmd) = command_rx.recv() {
                match cmd {
                    DatabaseCommand::InsertTracks(paths) => {
                        let mut new_tracks = Vec::new();
                        for path in paths {
                            match Track::insert(&mut conn, path) {
                                Ok(Some(track)) => new_tracks.push(track),
                                Ok(None) => {} // Skipped duplicate
                                Err(err) => {
                                    error!("Error when inserting track: {}", err);
                                }
                            }
                        }

                        let _ = event_tx.send(DatabaseEvent::InsertTracks(new_tracks));
                    }
                    DatabaseCommand::QueryAllTracks => {
                        let result = Track::select_all(&mut conn);
                        let _ = event_tx.send(DatabaseEvent::QueryAllTracks(result));
                    }
                }
            }
        });

        (command_tx, event_rx)
    }
}
