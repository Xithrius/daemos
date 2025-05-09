use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use rfd::FileDialog;
use walkdir::WalkDir;

pub static ALLOWED_AUDIO_FORMATS: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["flac", "mp3"]);

pub fn get_audio_files<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut result = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if ALLOWED_AUDIO_FORMATS.contains(&ext_str) {
                        result.push(path.display().to_string());
                    }
                }
            }
        }
    }

    result
}

pub fn select_folders_dialog() -> Option<Vec<PathBuf>> {
    FileDialog::new()
        .add_filter("extensions", &ALLOWED_AUDIO_FORMATS)
        .pick_folders()
}
