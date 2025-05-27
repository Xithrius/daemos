use crate::context::SharedContext;

#[derive(Debug, Clone)]
pub struct TaskTable {
    context: SharedContext,
}

impl TaskTable {
    pub fn new(context: SharedContext) -> Self {
        Self { context }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("I am a tasks table");
    }
}
