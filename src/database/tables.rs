use color_eyre::Result;
use rusqlite::Connection;

use super::connection::Database;

const TRACKS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL,
    hash TEXT UNIQUE,
    duration_secs REAL NOT NULL,
    valid BOOLEAN NOT NULL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
";

const PLAYLISTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS playlists (
    id TEXT PRIMARY KEY,
    parent_id TEXT,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (parent_id) REFERENCES playlists(id)
);
";

const PLAYLIST_TRACKS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS playlist_tracks (
    playlist_id TEXT NOT NULL,
    track_id TEXT NOT NULL,

    added_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (playlist_id, track_id),
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (track_id) REFERENCES tracks(id) ON DELETE CASCADE
);
";

const TABLES: [&str; 3] = [TRACKS_TABLE, PLAYLISTS_TABLE, PLAYLIST_TRACKS_TABLE];

impl Database {
    pub(crate) fn create_tables(conn: &mut Connection) -> Result<()> {
        for table in TABLES {
            conn.execute(table, ())?;
        }

        Ok(())
    }
}
