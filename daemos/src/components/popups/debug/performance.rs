use std::{collections::VecDeque, ops::RangeInclusive, time::Duration};

use egui::{Color32, NumExt};
use egui_plot::{
    AxisHints, GridMark, HPlacement, Legend, Line, Plot, PlotBounds, PlotPoint, PlotPoints,
};

use crate::{
    config::core::SharedConfig,
    context::{SharedContext, performance::MAX_LATENCY_RECORD_COUNT},
};

const DEFAULT_POPUP_WINDOW_SIZE: [f32; 2] = [500.0, 500.0];
const DEFAULT_PLOT_WINDOW_SIZE: [f32; 2] = [400.0, 400.0];

fn format_latency_mark(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
    let val = mark.value;
    if val < 1.0 {
        format!("{:.0} µs", val * 1000.0)
    } else {
        format!("{val:.1} ms")
    }
}

fn format_latency_label(_series_name: &str, val: &PlotPoint) -> String {
    let y = val.y;
    if y < 1.0 {
        format!("Latency: {:.0} µs", y * 1000.0)
    } else {
        format!("Latency: {y:.2} ms")
    }
}

#[derive(Debug, Clone, Default)]
struct LatencyLineGraph {
    time: f64,
}

impl LatencyLineGraph {
    const TITLE: &str = "Main Thread UI Render Latency";
    const LABEL: &str = "Latency";
    const LINE_COLOR: Color32 = Color32::from_rgb(100, 200, 100);

    fn latency<'a>(&self, y: &VecDeque<Duration>) -> Line<'a> {
        let max_x = (MAX_LATENCY_RECORD_COUNT as f64) - 1.0;
        let line_points: PlotPoints = y
            .iter()
            .enumerate()
            .map(|(i, dur)| {
                let x = max_x - (i as f64);
                let y = dur.as_secs_f64() * 1000.0;
                [x, y]
            })
            .collect();

        Line::new(Self::LABEL, line_points).color(Self::LINE_COLOR)
    }

    fn ui(&mut self, ui: &mut egui::Ui, latencies: &VecDeque<Duration>) {
        // ui.ctx().request_repaint();
        self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;

        let y_axes = vec![
            AxisHints::new_y()
                .label(Self::LABEL)
                .formatter(format_latency_mark)
                .placement(HPlacement::Left),
        ];

        let (min_latency_ms, max_latency_ms) = latencies
            .iter()
            .map(|d| d.as_secs_f64() * 1000.0)
            .fold((0.0f64, 0.0f64), |(min, max), v| (min.min(v), max.max(v)));

        // Ensure we have a sane default when empty
        let (min_y, max_y) = if latencies.is_empty() {
            (0.0, 1.0)
        } else if min_latency_ms == max_latency_ms {
            // Expand tiny ranges so it doesn't collapse to a line
            (min_latency_ms - 1.0, max_latency_ms + 1.0)
        } else {
            (min_latency_ms, max_latency_ms)
        };

        let min_y_points = MAX_LATENCY_RECORD_COUNT as f64;

        let padding = (max_y - min_y).max(1.0) * 0.05;
        let padded_max = max_y + padding;

        let plot = Plot::new(Self::TITLE)
            .legend(Legend::default())
            .custom_x_axes(vec![])
            .custom_y_axes(y_axes)
            .label_formatter(format_latency_label)
            .min_size(egui::Vec2::from(DEFAULT_PLOT_WINDOW_SIZE));

        ui.allocate_ui(egui::Vec2::from(DEFAULT_PLOT_WINDOW_SIZE), |ui| {
            plot.show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                    [0.0, 0.0],
                    [min_y_points - 1.0, padded_max],
                ));
                plot_ui.line(self.latency(latencies));
                // plot_ui.line(self.circle());
                // plot_ui.line(self.sin());
                // plot_ui.line(self.thingy());
            });
        });
    }
}

#[derive(Debug, Clone)]
struct PerformanceStats {
    average: f64,
    min: f64,
    max: f64,
    average_fps: f64,
    min_fps: f64,
    max_fps: f64,
}

impl PerformanceStats {
    fn calculate_stats(latencies: &VecDeque<Duration>) -> Self {
        if latencies.is_empty() {
            return Self {
                average: 0.0,
                min: 0.0,
                max: 0.0,
                average_fps: 0.0,
                min_fps: 0.0,
                max_fps: 0.0,
            };
        }

        let latency_ms: Vec<f64> = latencies.iter().map(|d| d.as_secs_f64() * 1000.0).collect();

        let average = latency_ms.iter().sum::<f64>() / latency_ms.len() as f64;
        let min = latency_ms
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(0.0);
        let max = latency_ms
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(0.0);

        // Calculate FPS: FPS = 1000 / latency_ms
        let average_fps = if average > 0.0 { 1000.0 / average } else { 0.0 };
        let min_fps = if max > 0.0 { 1000.0 / max } else { 0.0 }; // Note: min latency = max FPS
        let max_fps = if min > 0.0 { 1000.0 / min } else { 0.0 }; // Note: max latency = min FPS

        Self {
            average,
            min,
            max,
            average_fps,
            min_fps,
            max_fps,
        }
    }

    fn render_labels(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(format!(
                "Average: {:.2} ms ({:.1} FPS)",
                self.average, self.average_fps
            ));
            ui.label(format!("Min: {:.2} ms ({:.1} FPS)", self.min, self.max_fps));
            ui.label(format!("Max: {:.2} ms ({:.1} FPS)", self.max, self.min_fps));
        });
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceMetricsPopup {
    config: SharedConfig,
    context: SharedContext,

    latency_graph: LatencyLineGraph,
}

impl PerformanceMetricsPopup {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        Self {
            config,
            context,
            latency_graph: LatencyLineGraph::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().ui.visibility.performance_debug() {
            return;
        }

        let latencies = self
            .context
            .borrow()
            .performance_metrics
            .render_latency
            .timings();
        let mut vsync = self.config.borrow().general.vsync;

        let stats = PerformanceStats::calculate_stats(&latencies);

        egui::Window::new("Performance Metrics")
            .open(
                self.context
                    .borrow_mut()
                    .ui
                    .visibility
                    .performance_debug_mut(),
            )
            .resizable(true)
            .title_bar(true)
            .min_size(egui::Vec2::from(DEFAULT_POPUP_WINDOW_SIZE))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.latency_graph.ui(ui, &latencies);

                    ui.vertical(|ui| {
                        ui.add_enabled(false, egui::widgets::Checkbox::new(&mut vsync, "VSync"));

                        ui.separator();

                        stats.render_labels(ui);
                    });
                });
            });
    }
}
