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

    // pub fn create(&mut self, file_path: String) {
    //     self.sink.clear();

    //     let decoder = Decoder::new(File::open(file_path).unwrap())
    //         .unwrap()
    //         .buffered();

    //     self.sink.append(decoder);
    // }

    // pub fn play(&self) {
    //     self.sink.play();
    // }

    // pub fn pause(&self) {
    //     self.sink.pause();
    // }

    // pub fn toggle(&self) {
    //     if self.sink.is_paused() {
    //         self.sink.play();
    //     } else {
    //         self.sink.pause();
    //     }
    // }

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
