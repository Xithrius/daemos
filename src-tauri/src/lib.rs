mod commands;
mod context;

use std::error::Error;

use context::ContextInner;
use parking_lot::Mutex;
use tauri::{App, Manager};

fn create_state(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.manage(Mutex::new(ContextInner::new()));

    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .setup(create_state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::files::read_music_files,
            commands::playback::create::create_player
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
