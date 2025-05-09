use rodio::{OutputStream, Sink};
use tauri::{command, State};

use crate::context::Context;

#[command]
pub fn play_audio(context: State<'_, Context>, file_path: String) -> Result<(), String> {
    let mut context = context.lock();

    if context.sink().is_none() {
        let (_, stream_handle) = OutputStream::try_default().unwrap();

        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_volume(0.5);

        context.set_sink(sink);

        // context.create_sink();
    }

    context.new_track(file_path);
    context.play();

    Ok(())
}
