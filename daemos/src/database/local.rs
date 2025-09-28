use std::{
    env, fs,
    path::{Path, PathBuf},
};

use color_eyre::{Result, eyre::Context};

use crate::BINARY_NAME;

const SQLITE_FILE_NAME: &str = "db.sqlite";

pub fn get_database_storage_path() -> Result<PathBuf> {
    let partial_path = match env::consts::OS {
        "linux" | "macos" => {
            let home = env::var("HOME").context("HOME environment variable not found")?;

            Path::new(&home).join(".local").join("share")
        }
        "windows" => {
            todo!("Windows local storage has not been implemented yet");
        }
        _ => unimplemented!(),
    };

    let mut full_path = partial_path.join(BINARY_NAME);

    if !(full_path.is_dir() && full_path.exists()) {
        fs::create_dir_all(&full_path)
            .context("Failed to create all directories for local share sqlite storage")?;
    }

    full_path = full_path.join(SQLITE_FILE_NAME);

    Ok(full_path)
}
