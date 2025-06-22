use std::{collections::VecDeque, time::Duration};

#[derive(Debug, Clone)]
pub struct LatencyContext {
    timings: VecDeque<Duration>,
    max_length: usize,
}

impl Default for LatencyContext {
    fn default() -> Self {
        Self {
            timings: VecDeque::default(),
            max_length: 1000,
        }
    }
}

impl LatencyContext {
    pub fn timings(&self) -> VecDeque<Duration> {
        self.timings.clone()
    }

    pub fn add(&mut self, latency: Duration) {
        self.timings.push_front(latency);
        self.timings.truncate(self.max_length);
    }
}
