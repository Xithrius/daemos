use crate::context::SharedContext;

// #[derive(Debug, Clone)]
// struct PlaylistState {
//     index: usize,
//     playlist: Playlist,
//     playing: bool,
// }

// impl PlaylistState {
//     fn new(index: usize, playlist: Playlist, playing: bool) -> Self {
//         Self {
//             index,
//             playlist,
//             playing,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct TagTable {
    context: SharedContext,
    // playlists: Vec<Playlist>,
}

impl TagTable {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("I am a tag table");
    }
}
