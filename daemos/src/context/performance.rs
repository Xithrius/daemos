use std::{collections::VecDeque, time::Duration};

pub const MAX_LATENCY_RECORD_COUNT: usize = 1000;

#[derive(Debug, Clone)]
pub struct RenderLatency {
    timings: VecDeque<Duration>,
    max_length: usize,
}

impl Default for RenderLatency {
    fn default() -> Self {
        Self {
            timings: VecDeque::default(),
            max_length: MAX_LATENCY_RECORD_COUNT,
        }
    }
}

impl RenderLatency {
    pub fn timings(&self) -> VecDeque<Duration> {
        self.timings.clone()
    }

    pub fn add(&mut self, latency: Duration) {
        self.timings.push_front(latency);
        self.timings.truncate(self.max_length);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceMetricsContext {
    pub render_latency: RenderLatency,
}

impl PerformanceMetricsContext {
    pub fn render_latency(&self) -> VecDeque<Duration> {
        self.render_latency.timings()
    }

    pub fn add_render_latency(&mut self, latency: Duration) {
        self.render_latency.add(latency);
    }
}
