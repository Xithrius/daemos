use chrono::{DateTime, Utc};
use color_eyre::Result;
use rusqlite::{Connection, Row, params};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::utils::parse::{parse_date, parse_uuid};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct PlaylistTrack {
    playlist_id: Uuid,
    track_id: Uuid,
    added_at: DateTime<Utc>,
}

impl TryFrom<&Row<'_>> for PlaylistTrack {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let playlist_id = parse_uuid(row.get("playlist_id")?)?;
        let track_id = parse_uuid(row.get("track_id")?)?;
        let added_at = parse_date(row.get::<_, String>("added_at")?)?;

        let playlist_track = PlaylistTrack {
            playlist_id,
            track_id,
            added_at,
        };

        Ok(playlist_track)
    }
}

impl PlaylistTrack {
    pub fn create(conn: &Connection, playlist_id: Uuid, track_id: Uuid) -> Result<()> {
        let sql = "
            INSERT INTO playlist_tracks (playlist_id, track_id)
            VALUES (?1, ?2)
        ";

        conn.execute(sql, params![playlist_id.to_string(), track_id.to_string()])?;

        Ok(())
    }

    pub fn get(conn: &Connection, playlist_id: Uuid, track_id: Uuid) -> Result<PlaylistTrack> {
        let sql = "
            SELECT playlist_id, track_id, added_at
            FROM playlist_tracks
            WHERE playlist_id = ?1 AND track_id = ?2
        ";

        let mut stmt = conn.prepare(sql)?;

        let track = stmt.query_row(
            params![playlist_id.to_string(), track_id.to_string()],
            |row| PlaylistTrack::try_from(row),
        )?;

        Ok(track)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<PlaylistTrack>> {
        let sql = "
            SELECT playlist_id, track_id, added_at
            FROM playlist_tracks
        ";

        let mut stmt = conn.prepare(sql)?;

        let tracks = stmt
            .query_map([], |row| PlaylistTrack::try_from(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tracks)
    }

    pub fn get_by_playlist(conn: &Connection, playlist_id: Uuid) -> Result<Vec<PlaylistTrack>> {
        let sql = "
            SELECT playlist_id, track_id, added_at
            FROM playlist_tracks
            WHERE playlist_id = ?1
        ";

        let mut stmt = conn.prepare(sql)?;

        let tracks = stmt
            .query_map(params![playlist_id.to_string()], |row| {
                PlaylistTrack::try_from(row)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tracks)
    }

    pub fn delete(conn: &Connection, playlist_id: Uuid, track_id: Uuid) -> Result<()> {
        let sql = "
            DELETE FROM playlist_tracks
            WHERE playlist_id = ?1 AND track_id = ?2
        ";

        conn.execute(sql, params![playlist_id.to_string(), track_id.to_string()])?;

        Ok(())
    }
}
