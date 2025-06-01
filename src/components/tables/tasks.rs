#[derive(Debug, Clone, Default)]
pub struct TaskTable {
    // context: SharedContext,
}

impl TaskTable {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("I am a tasks table");
    }
}
