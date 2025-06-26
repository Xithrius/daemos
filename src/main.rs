// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::{cell::RefCell, rc::Rc, thread};

    use crossbeam::channel;
    use daemos::{
        app::App, channels::Channels, database::connection::Database, fonts::set_fonts,
        logging::initialize_logging, playback::state::Player,
    };
    use egui_extras::install_image_loaders;
    use tracing::{error, info};

    initialize_logging().expect("Failed to initialize logger");

    let config = {
        use daemos::config::load_config;

        let core_config = load_config().expect("Failed to load config");
        Rc::new(RefCell::new(core_config))
    };

    let icon_data = eframe::icon_data::from_png_bytes(include_bytes!("../static/assets/icon.png"))
        .unwrap_or_default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_icon(icon_data),
        vsync: config.borrow().general.vsync,
        ..Default::default()
    };

    let (database_command_tx, database_event_rx) = Database::start();

    let (player_command_tx, player_cmd_rx) = channel::unbounded();
    let (player_event_tx, player_event_rx) = channel::unbounded();

    let (err_tx, err_rx) = channel::bounded(1);

    thread::spawn(move || {
        info!("Spawned player thread");

        let player = match Player::new(player_event_tx, player_cmd_rx) {
            Err(err) => {
                let _ = err_tx.send(Some(err));
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

    let channels = Rc::new(Channels::new(
        database_command_tx,
        database_event_rx,
        player_command_tx,
        player_event_rx,
    ));

    eframe::run_native(
        "Daemos",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            set_fonts(cc);

            let app = App::new(cc, config, channels);

            Ok(Box::new(app))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unimplemented!("Wasm32 is not implemented for Daemos")
}
