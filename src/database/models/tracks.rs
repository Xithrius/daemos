use std::path::PathBuf;

use chrono::{DateTime, Utc};
use color_eyre::{Result, eyre::Context};
use rusqlite::{Row, types::Type};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::connection::SharedDatabase;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Track {
    pub id: Uuid,
    pub path: PathBuf,
    pub hash: Option<String>,
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
            valid: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl TryFrom<&Row<'_>> for Track {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Track {
            id: row
                .get::<_, String>("id")?
                .parse::<Uuid>()
                .map_err(|e| Self::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?,
            path: PathBuf::from(row.get::<_, String>("path")?),
            hash: row.get("hash")?,
            valid: row.get("valid")?,
            created_at: row
                .get::<_, String>("created_at")?
                .parse::<DateTime<Utc>>()
                .map_err(|e| Self::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?,
            updated_at: row
                .get::<_, String>("updated_at")?
                .parse::<DateTime<Utc>>()
                .map_err(|e| Self::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?,
        })
    }
}

impl Track {
    pub fn insert(database: SharedDatabase, path: PathBuf) -> Result<()> {
        let db = database.borrow();
        let conn = &db.conn;

        let query = "INSERT INTO Tracks VALUES(?1, ?2, ?3, ?4)";

        let args = Track {
            path,
            ..Default::default()
        };

        conn.execute(
            query,
            (
                args.id.to_string(),
                args.path.to_str(),
                args.hash,
                args.valid,
            ),
        )
        .context("Failed to execute insert on tracks table")?;

        Ok(())
    }

    pub fn insert_many(database: SharedDatabase, paths: Vec<PathBuf>) -> Result<()> {
        let mut db = database.borrow_mut();
        let conn = &mut db.conn;

        let tx = conn.transaction()?;

        let query = "INSERT INTO Tracks VALUES(?1, ?2, ?3, ?4)";

        {
            let mut stmt = tx.prepare(query)?;

            for path in paths {
                let args = Track {
                    path,
                    ..Default::default()
                };

                stmt.execute((
                    args.id.to_string(),
                    args.path.to_str(),
                    args.hash,
                    args.valid,
                ))
                .with_context(|| format!("Failed to insert track with path {:?}", args.path))?;
            }
        }

        tx.commit()
            .context("Failed to commit track insert transaction")?;

        Ok(())
    }

    pub fn select_all(database: SharedDatabase) -> Result<Vec<Track>> {
        let db = database.borrow();
        let conn = &db.conn;

        let query = "SELECT id, path, valid, created_at, updated_at FROM tracks";
        let mut stmt = conn
            .prepare(query)
            .context("Failed to prepare query for select all from tracks")?;

        let tracks: Vec<Track> = stmt
            .query_map([], |row| Track::try_from(row))?
            .collect::<Result<_, _>>()?;

        Ok(tracks)
    }
}
