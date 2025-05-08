use tauri::{command, State};

use crate::context::Context;

#[command]
pub fn create_player(context: State<'_, Context>, file_path: String) -> Result<(), String> {
    let mut context = context.lock();

    context.create(file_path);
    context.play();

    Ok(())
}
