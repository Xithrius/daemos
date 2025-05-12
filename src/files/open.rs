use std::{
    env,
    path::{Path, PathBuf},
};

use rfd::FileDialog;
use walkdir::WalkDir;

const ALLOWED_AUDIO_FORMATS: [&str; 3] = ["mp3", "wav", "flac"];

pub fn select_folders_dialog() -> Option<Vec<PathBuf>> {
    // TODO: Windows
    let home = env::var("HOME").unwrap_or_default();

    FileDialog::new()
        // .add_filter("extensions", &ALLOWED_AUDIO_FORMATS)
        .set_directory(home)
        .pick_folders()
}

pub fn get_tracks<P: AsRef<Path>>(dir: &P) -> Vec<PathBuf> {
    let mut tracks = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if ALLOWED_AUDIO_FORMATS.contains(&ext_str) {
                        tracks.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    tracks
}

pub fn get_track_file_name(track_file_path: PathBuf) -> Option<String> {
    let extension = track_file_path.extension().and_then(|ext| ext.to_str());

    track_file_path
        .file_name()
        .and_then(|track_file_name| track_file_name.to_str())
        .map(|name| {
            if let Some(ext) = extension {
                name.strip_suffix(ext).unwrap_or(name).trim_end_matches('.')
            } else {
                name.rsplit('.').next().unwrap_or(name)
            }
            .to_string()
        })
}
