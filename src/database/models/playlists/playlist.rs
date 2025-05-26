use chrono::{DateTime, Utc};
use color_eyre::{Result, eyre::Context};
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::models::utils::parse::{parse_date, parse_uuid};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Playlist {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl TryFrom<&Row<'_>> for Playlist {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id = parse_uuid(row.get::<_, String>("id")?)?;
        let created_at = parse_date(row.get::<_, String>("created_at")?)?;
        let updated_at = parse_date(row.get::<_, String>("updated_at")?)?;

        let playlist = Playlist {
            id,
            created_at,
            updated_at,
        };

        Ok(playlist)
    }
}

impl Playlist {
    pub fn create(&self, conn: &Connection) -> rusqlite::Result<()> {
        let sql = "
            INSERT INTO playlists (id, created_at, updated_at)
            VALUES (?1, ?2, ?3)
        ";

        conn.execute(
            sql,
            params![
                self.id.to_string(),
                self.created_at.to_rfc3339(),
                self.updated_at.to_rfc3339(),
            ],
        )?;

        Ok(())
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
}
