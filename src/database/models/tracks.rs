use std::path::PathBuf;

use chrono::{DateTime, Utc};
use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use rusqlite::{Row, types::Type};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use crate::{
    database::{connection::SharedDatabase, hash::hash_file},
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

        Ok(Track {
            id: parse_uuid(row.get::<_, String>("id")?)?,
            path: PathBuf::from(row.get::<_, String>("path")?),
            hash: row.get("hash")?,
            duration_secs: row.get::<_, f64>("duration_secs")?,
            valid: row.get("valid")?,
            created_at: parse_date(row.get::<_, String>("created_at")?)?,
            updated_at: parse_date(row.get::<_, String>("updated_at")?)?,
        })
    }
}

impl Track {
    pub fn insert(database: SharedDatabase, path: PathBuf) -> Result<()> {
        let db = database.borrow();
        let conn = &db.conn;

        let query = "INSERT INTO Tracks VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let hash = hash_file(&path)?.to_string();

        let track_metadata = extract_track_metadata(&path)?;
        let duration_secs = extract_track_duration(track_metadata)
            .context(format!("Failed to get duration from track {:?}", path))?
            .as_secs_f64();

        let args = Track {
            path: path.clone(),
            hash: Some(hash),
            duration_secs,
            ..Default::default()
        };

        conn.execute(
            query,
            (
                args.id.to_string(),
                args.path.to_str(),
                args.hash,
                args.duration_secs,
                args.valid,
                args.created_at.to_string(),
                args.updated_at.to_string(),
            ),
        )
        .context("Failed to execute insert on tracks table")?;

        debug!("Inserted track {:?} into database", path);

        Ok(())
    }

    pub fn insert_many(database: SharedDatabase, paths: Vec<PathBuf>) -> Result<()> {
        let mut db = database.borrow_mut();
        let conn = &mut db.conn;

        let tx = conn.transaction()?;

        let query = "INSERT INTO Tracks VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)";

        let paths_amount = paths.len();

        {
            let mut stmt = tx.prepare(query)?;

            for path in paths {
                let hash = hash_file(&path)?.to_string();

                let track_metadata = extract_track_metadata(&path)?;
                let duration_secs = extract_track_duration(track_metadata)
                    .context(format!("Failed to get duration from track {:?}", path))?
                    .as_secs_f64();

                let args = Track {
                    path: path.clone(),
                    hash: Some(hash),
                    duration_secs,
                    ..Default::default()
                };

                stmt.execute((
                    args.id.to_string(),
                    args.path.to_str(),
                    args.hash,
                    args.duration_secs,
                    args.valid,
                    args.created_at.to_string(),
                    args.updated_at.to_string(),
                ))
                .with_context(|| format!("Failed to insert track with path {:?}", args.path))?;

                debug!("Preparing track to be inserted into database: {:?}", path);
            }
        }

        tx.commit()
            .context("Failed to commit track insert transaction")?;

        debug!("Inserted {} track(s) into database", paths_amount);

        Ok(())
    }

    pub fn select_all(database: SharedDatabase) -> Result<Vec<Track>> {
        let db = database.borrow();
        let conn = &db.conn;

        let query =
            "SELECT id, path, hash, duration_secs, valid, created_at, updated_at FROM tracks";
        let mut stmt = conn
            .prepare(query)
            .context("Failed to prepare query for select all from tracks")?;

        let tracks: Vec<Track> = stmt
            .query_map([], |row| Track::try_from(row))?
            .collect::<Result<_, _>>()?;

        debug!("Found {} track(s) from table query", tracks.len());

        Ok(tracks)
    }
}
