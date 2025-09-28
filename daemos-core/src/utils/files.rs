use std::path::PathBuf;

pub fn get_file_name(file_path: PathBuf) -> Option<String> {
    let extension = file_path.extension().and_then(|ext| ext.to_str());

    file_path
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(|name| {
            if let Some(ext) = extension {
                name.strip_suffix(ext).unwrap_or(name).trim_end_matches('.')
            } else {
                name.rsplit('.').next().unwrap_or(name)
            }
            .to_string()
        })
}
