use notify_rust::Notification;
use tracing::error;

pub fn now_playing(track_name: String) {
    if let Err(err) = Notification::new()
        .summary("Daemos")
        .body(&format!("Now playing - {}", track_name))
        .icon("daemos")
        .show()
    {
        error!("Failed to send now playing notification: {}", err)
    }
}
