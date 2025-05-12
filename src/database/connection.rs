use std::{cell::RefCell, rc::Rc};

use rusqlite::Connection;
use tracing::info;

use super::local::get_database_storage_path;

#[derive(Debug)]
pub struct Database {
    pub(crate) conn: Connection,
}

impl Default for Database {
    fn default() -> Self {
        let database_path = get_database_storage_path().unwrap();

        let conn = Connection::open(&database_path).expect("Failed to open database connection");

        info!("Successfully connected to database at {:?}", database_path);

        Self { conn }
    }
}

pub type SharedDatabase = Rc<RefCell<Database>>;
