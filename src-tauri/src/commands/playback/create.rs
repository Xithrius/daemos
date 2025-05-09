use std::fs::File;

use rodio::{Decoder, OutputStream, Source};
use tauri::command;

// context: State<'_, Context>,

#[command]
pub fn create_player(file_path: String) -> Result<(), String> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.set_volume(0.5);
    let data = Decoder::new(File::open(file_path).unwrap())
        .unwrap()
        .buffered();
    sink.append(data);
    sink.play();
    sink.sleep_until_end();

    Ok(())
}
