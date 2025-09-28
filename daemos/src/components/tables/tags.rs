#[derive(Debug, Clone, Default)]
pub struct TagTable {
    // context: SharedContext,
    // tags: Vec<Playlist>,
}

impl TagTable {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("I am a tag table");
    }
}
