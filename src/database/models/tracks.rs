use std::path::PathBuf;

use chrono::{DateTime, Utc};
use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use super::utils::parse::{parse_date, parse_uuid};
use crate::{
    database::hash::hash_file,
    playback::track_metadata::{extract_track_duration, extract_track_metadata},
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Track {
    pub id: Uuid,
    pub path: PathBuf,
    pub hash: Option<String>,
    pub duration_secs: f64,
    pub valid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            path: PathBuf::new(),
            hash: None,
            duration_secs: 0.0,
            valid: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl TryFrom<&Row<'_>> for Track {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let id = parse_uuid(row.get::<_, String>("id")?)?;
        let path = PathBuf::from(row.get::<_, String>("path")?);
        let hash = row.get("hash")?;
        let duration_secs = row.get::<_, f64>("duration_secs")?;
        let valid = row.get("valid")?;
        let created_at = parse_date(row.get::<_, String>("created_at")?)?;
        let updated_at = parse_date(row.get::<_, String>("updated_at")?)?;

        let track = Track {
            id,
            path,
            hash,
            duration_secs,
            valid,
            created_at,
            updated_at,
        };

        Ok(track)
    }
}

impl Track {
    pub fn create(conn: &Connection, path: PathBuf) -> Result<Option<Track>> {
        let sql = "
            INSERT INTO Tracks
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT DO NOTHING
        ";

        let hash = hash_file(&path)?.to_string();

        let track_metadata = extract_track_metadata(&path)?;
        let duration_secs = extract_track_duration(track_metadata)
            .context(format!("Failed to get duration from track {:?}", path))?
            .as_secs_f64();

        let track = Track {
            path: path.clone(),
            hash: Some(hash),
            duration_secs,
            ..Default::default()
        };

        let inserted = conn
            .execute(
                sql,
                params![
                    track.id.to_string(),
                    track.path.to_str(),
                    track.hash,
                    track.duration_secs,
                    track.valid,
                    track.created_at,
                    track.updated_at,
                ],
            )
            .context("Failed to execute insert on tracks table")?;

        if inserted == 0 {
            debug!("Skipped duplicate track: {:?}", path);

            return Ok(None);
        }

        debug!("Inserted track into database: {:?}", path);

        Ok(Some(track))
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Track>> {
        let sql = "
            SELECT id, path, hash, duration_secs, valid, created_at, updated_at
            FROM tracks
        ";

        let mut stmt = conn
            .prepare(sql)
            .context("Failed to prepare query for select all from tracks")?;

        let tracks: Vec<Track> = stmt
            .query_map([], |row| Track::try_from(row))?
            .collect::<Result<_, _>>()?;

        debug!("Found {} track(s) from tracks table query", tracks.len());

        Ok(tracks)
    }
}
