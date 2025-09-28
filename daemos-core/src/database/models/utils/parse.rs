use chrono::{DateTime, Utc};
use rusqlite::{Error, types::Type};
use uuid::Uuid;

/// Parses a string into `DateTime<Utc>`, returning a `rusqlite::Error` on failure.
pub fn parse_date(value: String) -> Result<DateTime<Utc>, Error> {
    value
        .parse::<DateTime<Utc>>()
        .map_err(|e| Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))
}

/// Parses a string into `Uuid`, returning a `rusqlite::Error` on failure.
pub fn parse_uuid(value: String) -> Result<Uuid, Error> {
    value
        .parse::<Uuid>()
        .map_err(|e| Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))
}
