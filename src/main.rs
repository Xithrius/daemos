// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::thread;

    use crossbeam::channel;
    use drakn::{
        Context, config::load::load_config, database::connection::Database, fonts::set_fonts,
        logging::initialize_logging, playback::state::Player,
    };
    use tracing::{error, info};

    initialize_logging().expect("Failed to initialize logger");

    let config = load_config().expect("Failed to load config");

    let icon_data = eframe::icon_data::from_png_bytes(include_bytes!("../static/assets/icon.png"))
        .unwrap_or_default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_icon(icon_data),
        vsync: config.vsync,
        ..Default::default()
    };

    let database = Database::default();
    database.create_tables().expect("Failed to create tables");

    let (player_cmd_tx, player_cmd_rx) = channel::unbounded();
    let (player_event_tx, player_event_rx) = channel::unbounded();

    let (err_tx, err_rx) = channel::bounded(1);

    thread::spawn(move || {
        info!("Spawned player thread");

        let player = match Player::new(player_event_tx, player_cmd_rx) {
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
            set_fonts(cc);

            let context = Context::new(cc, config, database, player_cmd_tx, player_event_rx);

            Ok(Box::new(context))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!("Wasm32 is not implemented for Drakn")
}
