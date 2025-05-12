// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::thread;

    use crossbeam::channel;
    use drakn::{
        Context, config::load::load_config, database::connection::Database,
        logging::initialize_logging, playback::state::Player,
    };
    use tracing::{error, info};

    initialize_logging().expect("Failed to initialize logger");

    let config = load_config().expect("Failed to load config");

    let icon_data =
        eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")).unwrap_or_default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_icon(icon_data),
        vsync: config.vsync,
        ..Default::default()
    };

    let database = Database::default();
    database.create_tables().expect("Failed to create tables");

    let (tx, rx) = channel::unbounded();
    let (err_tx, err_rx) = channel::bounded(1);

    thread::spawn(move || {
        info!("Spawned player thread");

        let player = match Player::new(rx) {
            Err(e) => {
                let _ = err_tx.send(Some(e));
                return;
            }
            Ok(player) => {
                let _ = err_tx.send(None);
                player
            }
        };

        player.create();
    });

    if let Ok(Some(err)) = err_rx.recv() {
        error!("Failed to initialize player: {:?}", err);
        std::process::exit(1);
    }

    eframe::run_native(
        "Drakn",
        options,
        Box::new(|cc| {
            let context = Context::new(cc, config, database, tx);

            Ok(Box::new(context))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!("Wasm32 is not implemented for Drakn")
}
