use std::path::PathBuf;

use chrono::{DateTime, Utc};
use color_eyre::Result;
use rusqlite::{Connection, Row, types::Type};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Playlist {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Playlist {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl TryFrom<&Row<'_>> for Playlist {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let parse_date = |value: String| {
            value
                .parse::<DateTime<Utc>>()
                .map_err(|e| Self::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))
        };

        let parse_uuid = |value: String| {
            value
                .parse::<Uuid>()
                .map_err(|e| Self::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))
        };

        let track = Playlist {
            id: parse_uuid(row.get::<_, String>("id")?)?,
            parent_id: row.get::<_, String>("parent_id").and_then(parse_uuid).ok(),
            created_at: parse_date(row.get::<_, String>("created_at")?)?,
            updated_at: parse_date(row.get::<_, String>("updated_at")?)?,
        };

        Ok(track)
    }
}

impl Playlist {
    pub fn insert(conn: &mut Connection, path: PathBuf) -> Result<Option<Playlist>> {
        todo!()
        // let query = "INSERT INTO Tracks VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7) ON CONFLICT DO NOTHING";

        // let hash = hash_file(&path)?.to_string();

        // let track_metadata = extract_track_metadata(&path)?;
        // let duration_secs = extract_track_duration(track_metadata)
        //     .context(format!("Failed to get duration from track {:?}", path))?
        //     .as_secs_f64();

        // let track = Playlist {
        //     path: path.clone(),
        //     hash: Some(hash),
        //     duration_secs,
        //     ..Default::default()
        // };

        // let inserted = conn
        //     .execute(
        //         query,
        //         (
        //             track.id.to_string(),
        //             track.path.to_str(),
        //             track.hash.clone(),
        //             track.duration_secs,
        //             track.valid,
        //             track.created_at.to_string(),
        //             track.updated_at.to_string(),
        //         ),
        //     )
        //     .context("Failed to execute insert on tracks table")?;

        // if inserted == 0 {
        //     debug!("Skipped duplicate track: {:?}", path);
        //     return Ok(None);
        // }

        // debug!("Inserted track into database: {:?}", path);

        // Ok(Some(track))
    }

    pub fn select_all(conn: &mut Connection) -> Result<Vec<Playlist>> {
        todo!()

        // let query =
        //     "SELECT id, path, hash, duration_secs, valid, created_at, updated_at FROM tracks";

        // let mut stmt = conn
        //     .prepare(query)
        //     .context("Failed to prepare query for select all from tracks")?;

        // let tracks: Vec<Playlist> = stmt
        //     .query_map([], |row| Playlist::try_from(row))?
        //     .collect::<Result<_, _>>()?;

        // debug!("Found {} track(s) from table query", tracks.len());

        // Ok(tracks)
    }
}
