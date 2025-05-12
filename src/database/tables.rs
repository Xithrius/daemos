use color_eyre::Result;

use super::connection::Database;

const TRACKS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS tracks (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL,
    hash TEXT UNIQUE,
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

impl Database {
    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch(&format!(
            "
            BEGIN;
            {}
            {}
            COMMIT;
            ",
            TRACKS_TABLE, PLAYLISTS_TABLE
        ))?;

        Ok(())
    }
}
