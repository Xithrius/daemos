mod commands;
mod context;

use parking_lot::Mutex;
use tauri::Manager;

use crate::context::Context;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Context::default()));

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::files::read_music_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
