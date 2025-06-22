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
    files::open::get_file_name,
    playback::track_metadata::{extract_track_duration, extract_track_metadata},
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Track {
    pub id: Uuid,
    pub path: PathBuf,
    pub name: String,
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
            name: String::default(),
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
        let name = row.get::<_, String>("name")?;
        let hash = row.get("hash")?;
        let duration_secs = row.get::<_, f64>("duration_secs")?;
        let valid = row.get("valid")?;
        let created_at = parse_date(row.get::<_, String>("created_at")?)?;
        let updated_at = parse_date(row.get::<_, String>("updated_at")?)?;

        let track = Track {
            id,
            path,
            name,
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
    /// Creates a track, or returns the one it conflicts with on hash and path.
    /// When creating a new track, the hash of the file is generated, along with a new UUID.
    /// All other attributes of the track are generated with defaults.
    // TODO: Return an enum to tell if a new track has been created, or the old one was returned
    pub fn create(conn: &Connection, path: PathBuf) -> Result<Option<Track>> {
        let sql = "
            INSERT INTO Tracks
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT (hash, path) DO UPDATE SET
                hash = excluded.hash
            RETURNING *
        ";

        let hash = hash_file(&path)?.to_string();

        let track_metadata = extract_track_metadata(&path)?;
        let duration_secs = extract_track_duration(track_metadata)
            .context(format!("Failed to get duration from track {:?}", path))?
            .as_secs_f64();

        let name = get_file_name(path.clone())
            .context(format!("Failed to get track file name from {:?}", path))?;

        let track = Track {
            path: path.clone(),
            name,
            hash: Some(hash),
            duration_secs,
            ..Default::default()
        };

        let mut stmt = conn.prepare(sql)?;

        let mut rows = stmt.query(params![
            track.id.to_string(),
            track.path.to_str(),
            track.name,
            track.hash,
            track.duration_secs,
            track.valid,
            track.created_at,
            track.updated_at,
        ])?;

        if let Some(row) = rows.next()? {
            let returned_track = Track::try_from(row)?;
            debug!(
                "Inserted or found track in database: {:?}",
                returned_track.path
            );
            Ok(Some(returned_track))
        } else {
            debug!("No track returned for path: {:?}", path);
            Ok(None)
        }
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Track>> {
        let sql = "
            SELECT id, path, name, hash, duration_secs, valid, created_at, updated_at
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
