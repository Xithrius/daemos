// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use drakn::{Context, config::load::load_config, logging::initialize_logging};

    initialize_logging().expect("Failed to initialize logger");

    let config = load_config().expect("Failed to load config");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        vsync: config.vsync,
        ..Default::default()
    };

    eframe::run_native(
        "Drakn",
        options,
        Box::new(|cc| {
            let context = Context::new(cc, config);

            Ok(Box::new(context))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!("Wasm32 is not implemented for Drakn")
}
