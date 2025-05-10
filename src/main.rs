mod app;
mod files;
mod logging;

use color_eyre::{Result, eyre::Context};
use logging::initialize_logging;

use crate::app::Application;

pub fn main() -> Result<()> {
    initialize_logging()?;

    iced::application("Drakn", Application::update, Application::view)
        .subscription(Application::subscription)
        .theme(Application::theme)
        .run_with(Application::new)
        .context("Application failed to launch")
}
