use std::fmt;

use chrono::{DateTime, Utc};
use color_eyre::{Result, eyre::Context};
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};
use uuid::Uuid;

use crate::database::models::{
    tracks::Track,
    utils::parse::{parse_date, parse_uuid},
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl fmt::Display for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TryFrom<&Row<'_>> for Playlist {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id = parse_uuid(row.get::<_, String>("id")?)?;
        let name = row.get::<_, String>("name")?;
        let created_at = parse_date(row.get::<_, String>("created_at")?)?;
        let updated_at = parse_date(row.get::<_, String>("updated_at")?)?;

        let playlist = Playlist {
            id,
            name,
            created_at,
            updated_at,
        };

        Ok(playlist)
    }
}

impl Playlist {
    pub fn create(conn: &Connection, name: String) -> Result<Option<Playlist>> {
        let sql = "
            INSERT INTO playlists (id, name, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4)
        ";

        let playlist = Playlist {
            name: name.clone(),
            ..Default::default()
        };

        let inserted = conn.execute(
            sql,
            params![
                playlist.id.to_string(),
                playlist.name,
                playlist.created_at,
                playlist.updated_at,
            ],
        )?;

        if inserted == 0 {
            error!(
                "Something went wrong while creating the new playlist {}",
                name
            );

            return Ok(None);
        }

        debug!("Inserted playlist into database: {}", name);

        Ok(Some(playlist))
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Self>> {
        let query = "SELECT * FROM playlists";

        let mut stmt = conn
            .prepare(query)
            .context("Failed to prepare query for select all from playlists")?;

        let playlists: Vec<Playlist> = stmt
            .query_map([], |row| Playlist::try_from(row))?
            .collect::<Result<_, _>>()?;

        Ok(playlists)
    }

    pub fn delete(conn: &Connection, id: Uuid) -> rusqlite::Result<()> {
        let sql = "
            DELETE FROM playlists
            WHERE id = ?1
        ";

        conn.execute(sql, params![id.to_string()])?;

        Ok(())
    }

    pub fn get_tracks(conn: &Connection, id: Uuid) -> Result<Vec<Track>> {
        let sql = "
            SELECT t.id, t.path, t.name, t.hash, t.duration_secs, t.valid, t.created_at, t.updated_at
            FROM tracks t
            JOIN playlist_tracks pt ON t.id = pt.track_id
            WHERE pt.playlist_id = ?1;
        ";

        let mut stmt = conn
            .prepare(sql)
            .context("Failed to prepare query for select all tracks from playlist")?;

        let playlist_tracks: Vec<Track> = stmt
            .query_map(params![id.to_string()], |row| Track::try_from(row))?
            .collect::<Result<_, _>>()?;

        debug!(
            "Found {} track(s) from playlist/tracks query",
            playlist_tracks.len()
        );

        Ok(playlist_tracks)
    }
}
