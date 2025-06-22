use std::{path::PathBuf, thread};

use color_eyre::Result;
use crossbeam::channel::{Receiver, Sender, unbounded};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info};

use super::{local::get_database_storage_path, models::tracks::Track};
use crate::database::models::playlists::{playlist::Playlist, playlist_tracks::PlaylistTrack};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DatabaseCommand {
    /// All tracks to be added, and the optional playlist
    InsertTracks(Vec<PathBuf>, Option<String>),
    /// Get all tracks within a playlist, if provided then all tracks are returned
    QueryTracks(Option<Playlist>),
    /// Create a new playlist with the specified name
    InsertPlaylist(String),
    /// Get all the playlists
    QueryPlaylists,
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Track {0} already exists in general playlist")]
    DuplicateTrack(PathBuf),

    #[error("Track {0} already exists in playlist {1}")]
    DuplicatePlaylistTrack(PathBuf, Playlist),

    #[error("Playlist already exists")]
    DuplicatePlaylist,

    #[error("Database is unavailable")]
    DatabaseUnavailable,

    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug)]
pub enum DatabaseEvent {
    InsertTrack(Track, Option<Playlist>),
    QueryTracks(Vec<Track>),
    InsertPlaylist(Playlist),
    QueryPlaylists(Vec<Playlist>),
}

#[derive(Debug)]
pub struct Database;

impl Database {
    pub fn start() -> (
        Sender<DatabaseCommand>,
        Receiver<Result<DatabaseEvent, DatabaseError>>,
    ) {
        let database_path = get_database_storage_path().expect("Failed to get DB path");

        let (command_tx, command_rx) = unbounded();
        let (event_tx, event_rx) = unbounded::<Result<DatabaseEvent, DatabaseError>>();

        thread::spawn(move || {
            let conn =
                Connection::open(&database_path).expect("Failed to open database connection");

            if let Err(err) = Database::create_tables(&conn) {
                error!("Failed to create tables: {}", err);
                std::process::exit(1);
            }

            info!(
                "Database thread running with connection at {:?}",
                database_path
            );

            while let Ok(cmd) = command_rx.recv() {
                match cmd {
                    DatabaseCommand::InsertTracks(track_paths, playlist_name) => {
                        let playlist = if let Some(playlist_name) = playlist_name {
                            Playlist::create(&conn, playlist_name).unwrap_or_default()
                        } else {
                            None
                        };

                        for track_path in track_paths {
                            let track_result = Track::create(&conn, track_path.clone());

                            let track = match track_result {
                                Ok(Some(track)) => track,
                                Ok(None) => {
                                    let duplicate_error = if let Some(playlist) = playlist.as_ref()
                                    {
                                        DatabaseError::DuplicatePlaylistTrack(
                                            track_path,
                                            playlist.clone(),
                                        )
                                    } else {
                                        DatabaseError::DuplicateTrack(track_path)
                                    };
                                    let _ = event_tx.send(Err(duplicate_error));
                                    continue;
                                }
                                Err(err) => {
                                    error!("Error when inserting track: {}", err);
                                    continue;
                                }
                            };

                            let insert_track_event =
                                DatabaseEvent::InsertTrack(track.clone(), playlist.clone());
                            let _ = event_tx.send(Ok(insert_track_event));

                            if let Some(playlist_id) = playlist.as_ref().map(|playlist| playlist.id)
                            {
                                if let Err(err) =
                                    PlaylistTrack::create(&conn, playlist_id, track.id)
                                {
                                    error!("Error when inserting track to playlist: {}", err);
                                }
                            }
                        }
                    }
                    DatabaseCommand::QueryTracks(playlist) => {
                        let result = if let Some(playlist) = playlist {
                            Playlist::get_tracks(&conn, playlist.id)
                        } else {
                            Track::get_all(&conn)
                        };

                        if let Ok(tracks) = result {
                            let query_tracks_event = DatabaseEvent::QueryTracks(tracks);
                            let _ = event_tx.send(Ok(query_tracks_event));
                        }
                    }
                    DatabaseCommand::InsertPlaylist(playlist_name) => {
                        debug!("INSERT PLAYLIST CALLED");
                        let playlist_result = Playlist::create(&conn, playlist_name);

                        match playlist_result {
                            Ok(Some(playlist)) => {
                                let insert_playlist_event = DatabaseEvent::InsertPlaylist(playlist);
                                let _ = event_tx.send(Ok(insert_playlist_event));
                            }
                            Ok(None) => {}
                            Err(err) => {
                                error!("Error when inserting playlist: {}", err);
                            }
                        }
                    }
                    DatabaseCommand::QueryPlaylists => {
                        let result = Playlist::get_all(&conn);

                        let Ok(playlists) = result else {
                            error!("Something went wrong when querying all playlists");
                            continue;
                        };

                        let query_playlists_event = DatabaseEvent::QueryPlaylists(playlists);
                        let _ = event_tx.send(Ok(query_playlists_event));
                    }
                }
            }
        });

        (command_tx, event_rx)
    }
}
