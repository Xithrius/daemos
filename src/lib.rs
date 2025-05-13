mod context;
pub use context::Context;

pub mod components;
pub mod config;
pub mod database;
pub mod files;
pub mod logging;
pub mod playback;

const BINARY_NAME: &str = "drakn";
