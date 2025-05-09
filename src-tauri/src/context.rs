use std::{fs::File, io::BufReader};

use parking_lot::Mutex;
use rodio::{Decoder, OutputStream, Sink, Source};

pub struct ContextInner {
    sink: Option<Sink>,
}

pub type Context = Mutex<ContextInner>;

impl ContextInner {
    pub fn new() -> Self {
        Self { sink: None }
    }

    pub fn set_sink(&mut self, sink: Sink) {
        self.sink = Some(sink);
    }

    pub fn sink(&self) -> Option<&Sink> {
        self.sink.as_ref()
    }

    pub fn new_track(&mut self, file_path: String) {
        let Some(sink) = self.sink.as_mut() else {
            println!("SINK NOT FOUND");
            return;
        };

        let data = Decoder::new(File::open(file_path).unwrap())
            .unwrap()
            .buffered();

        sink.append(data);
        sink.play();
    }

    pub fn play(&self) {
        let Some(sink) = self.sink.as_ref() else {
            println!("SINK NOT FOUND");
            return;
        };

        sink.play();
    }

    // pub fn pause(&self) {
    //     self.sink.pause();
    // }

    pub fn toggle(&self) {
        let Some(sink) = self.sink.as_ref() else {
            println!("SINK NOT FOUND");
            return;
        };

        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }

    // pub fn volume_up(&self, value_delta: f32) {
    //     let new_volume = (self.sink.volume() + value_delta).min(1.0);

    //     self.sink.set_volume(new_volume);
    // }

    // pub fn volume_down(&self, value_delta: f32) {
    //     let new_volume = (self.sink.volume() - value_delta).max(0.0);

    //     self.sink.set_volume(new_volume);
    // }

    // pub fn set_volume(&self, value: f32) {
    //     self.sink.set_volume(value);
    // }

    // pub fn stop(&self) {
    //     self.sink.clear();
    // }
}
