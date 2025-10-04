use std::collections::VecDeque;

use crossbeam::channel::{Receiver, TryRecvError};

#[derive(Debug)]
pub struct PcmChunk {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub struct PcmSource {
    pub rx: Receiver<PcmChunk>,
    pub buffer: VecDeque<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

impl PcmSource {
    pub fn new(
        rx: Receiver<PcmChunk>,
        buffer: VecDeque<f32>,
        channels: u16,
        sample_rate: u32,
    ) -> Self {
        Self {
            rx,
            buffer,
            channels,
            sample_rate,
        }
    }
}

impl Iterator for PcmSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.buffer.pop_front() {
            return Some(s);
        }

        // Block for at least one chunk when empty, then drain any immediately available
        let first = match self.rx.recv() {
            Ok(c) => c,
            Err(_) => return None,
        };

        debug_assert_eq!(first.channels, self.channels);
        debug_assert_eq!(first.sample_rate, self.sample_rate);
        self.buffer = first.samples.into();

        loop {
            match self.rx.try_recv() {
                Ok(mut more) => {
                    debug_assert_eq!(more.channels, self.channels);
                    debug_assert_eq!(more.sample_rate, self.sample_rate);

                    if self.buffer.is_empty() {
                        self.buffer = more.samples.into();
                    } else {
                        self.buffer.extend(more.samples.drain(..));
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        self.buffer.pop_front()
    }
}

impl rodio::Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
