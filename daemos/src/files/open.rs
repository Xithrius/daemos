use std::{
    env, fs,
    path::{Path, PathBuf},
};

use rfd::FileDialog;
use walkdir::WalkDir;

const ALLOWED_AUDIO_FORMATS: [&str; 3] = ["mp3", "wav", "flac"];

pub fn select_file_dialog() -> Option<PathBuf> {
    let home = env::var("HOME").unwrap_or_default();

    FileDialog::new().set_directory(home).pick_file()
}

pub fn select_folders_dialog() -> Option<Vec<PathBuf>> {
    // TODO: Support for Windows through APPDATA
    let home = env::var("HOME").unwrap_or_default();

    FileDialog::new()
        // .add_filter("extensions", &ALLOWED_AUDIO_FORMATS)
        .set_directory(home)
        .pick_folders()
}

/// Returns a list of audio track paths from the given directory.
/// If `recursive` is true, subdirectories will also be searched.
pub fn get_folder_tracks<P: AsRef<Path>>(dir: &P, recursive: bool) -> Vec<PathBuf> {
    let mut tracks = Vec::new();

    if recursive {
        for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file()
                && let Some(extension) = path.extension().and_then(|e| e.to_str())
                && ALLOWED_AUDIO_FORMATS.contains(&extension)
            {
                tracks.push(path.to_path_buf());
            }
        }
    } else if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file()
                && let Some(extension) = path.extension().and_then(|e| e.to_str())
                && ALLOWED_AUDIO_FORMATS.contains(&extension)
            {
                tracks.push(path);
            }
        }
    }

    tracks
}

