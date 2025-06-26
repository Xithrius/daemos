use std::{collections::VecDeque, ops::RangeInclusive, time::Duration};

use egui::{Color32, NumExt};
use egui_plot::{
    AxisHints, GridMark, HPlacement, Legend, Line, Plot, PlotBounds, PlotPoint, PlotPoints,
};

use crate::{
    config::core::SharedConfig,
    context::{SharedContext, latency::MAX_LATENCY_RECORD_COUNT},
};

const DEFAULT_DEBUG_WINDOW_SIZE: [f32; 2] = [300.0, 200.0];

fn format_latency_mark(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
    let val = mark.value;
    if val < 1.0 {
        format!("{:.0} µs", val * 1000.0)
    } else {
        format!("{:.1} ms", val)
    }
}

fn format_latency_label(_series_name: &str, val: &PlotPoint) -> String {
    let y = val.y;
    if y < 1.0 {
        format!("Latency: {:.0} µs", y * 1000.0)
    } else {
        format!("Latency: {:.2} ms", y)
    }
}

#[derive(Debug, Clone, Default)]
struct LatencyLineGraph {
    time: f64,
}

impl LatencyLineGraph {
    const NAME: &str = "Main Thread UI Render Latency";
    const Y_AXIS_LABEL: &str = "Latency";
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

        Line::new(Self::NAME, line_points).color(Self::LINE_COLOR)
    }

    fn ui(&mut self, ui: &mut egui::Ui, latencies: &VecDeque<Duration>) {
        ui.ctx().request_repaint();
        self.time += ui.input(|i| i.unstable_dt).at_most(1.0 / 30.0) as f64;

        let y_axes = vec![
            AxisHints::new_y()
                .label(Self::Y_AXIS_LABEL)
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

        let plot = Plot::new(Self::NAME)
            .legend(Legend::default())
            .custom_x_axes(vec![])
            .custom_y_axes(y_axes)
            .label_formatter(format_latency_label);

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
    }
}

#[derive(Debug, Clone)]
pub struct DebugPopup {
    _config: SharedConfig,
    context: SharedContext,

    latency_graph: LatencyLineGraph,
}

impl DebugPopup {
    pub fn new(config: SharedConfig, context: SharedContext) -> Self {
        Self {
            _config: config,
            context,
            latency_graph: LatencyLineGraph::default(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        if !self.context.borrow().ui.visibility.debug() {
            return;
        }

        let latencies = self.context.borrow().latency.timings();

        egui::Window::new("Debug")
            .open(self.context.borrow_mut().ui.visibility.debug_mut())
            .resizable(true)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.set_min_size(DEFAULT_DEBUG_WINDOW_SIZE.into());

                self.latency_graph.ui(ui, &latencies);
            });
    }
}
