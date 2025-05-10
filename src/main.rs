#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use drakn::Context;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "eframe template",
        options,
        Box::new(|cc| Ok(Box::new(Context::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!("Wasm32 is not implemented for Drakn")
}
