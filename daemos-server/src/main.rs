#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_field_names,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub mod config;
pub mod cors;
pub mod routers;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use config::Config;
use cors::default_cors;
use daemos_core::logging::initialize_logging_with_default;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging_with_default("DAEMOS_SERVER_LOG").expect("Failed to initialize logger");

    let config = Config::new("config.toml".to_string());

    let bind_address = config.bind_address();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(default_cors())
            .app_data(Data::new(config.clone()))
            .configure(routers::config)
    })
    .workers(2)
    .bind(bind_address)
    .expect("Failed to start Actix web service")
    .run()
    .await
}
