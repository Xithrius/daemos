use tauri::{command, State};

use crate::context::Context;

#[command]
pub fn toggle_audio(context: State<'_, Context>) -> Result<(), String> {
    let context = context.lock();

    context.toggle();

    Ok(())
}
