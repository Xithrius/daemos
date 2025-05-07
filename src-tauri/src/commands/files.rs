use std::{path::Path, sync::LazyLock};

use walkdir::WalkDir;

pub static DEFAULT_ALLOWED_FORMATS: LazyLock<Vec<&str>> = LazyLock::new(|| vec!["flac", "mp3"]);

fn get_files_with_extensions<P: AsRef<Path>>(dir: P, extensions: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if extensions.contains(&ext_str.to_string()) {
                        result.push(path.display().to_string());
                    }
                }
            }
        }
    }

    result
}

#[tauri::command]
pub fn read_music_files(
    file_path: String,
    allowed_formats: Option<Vec<String>>,
) -> Result<Vec<String>, String> {
    let path = Path::new(&file_path);

    let formats = if let Some(user_formats) = allowed_formats {
        user_formats
    } else {
        DEFAULT_ALLOWED_FORMATS
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    let found_files = get_files_with_extensions(path, &formats);

    Ok(found_files)
}
