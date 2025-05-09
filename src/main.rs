mod app;
mod files;

use crate::app::Application;

pub fn main() -> iced::Result {
    iced::application("Drakn", Application::update, Application::view)
        .subscription(Application::subscription)
        .theme(Application::theme)
        .run_with(Application::new)
}
