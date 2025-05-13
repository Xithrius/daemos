use serde::Serialize;

// #[derive(Serialize, Debug)]
// pub struct SystemInfo {
//     sys: System,
//     pid: Pid,
//     cpu_usage: f32,

//     #[serde(skip)]
//     last_update: Instant,
// }

// impl Default for SystemInfo {
//     fn default() -> Self {
//         let mut sys = System::new_all();
//         sys.refresh_all();
//         let pid = sysinfo::get_current_pid().unwrap();

//         Self {
//             sys,
//             pid,
//             cpu_usage: 0.0,
//             last_update: Instant::now(),
//         }
//     }
// }

#[derive(Serialize, Debug, Default)]
pub struct MenuBar {
    // system_info: SystemInfo,
}

impl MenuBar {
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, visible_settings: &mut bool) {
        // if self.system_info.last_update.elapsed().as_secs_f32() >= 1.0 {
        //     self.system_info.sys.refresh_processes(
        //         sysinfo::ProcessesToUpdate::Some(&[self.system_info.pid]),
        //         true,
        //     );

        //     if let Some(process) = self.system_info.sys.process(self.system_info.pid) {
        //         self.system_info.cpu_usage = process.cpu_usage();
        //     }

        //     self.system_info.last_update = Instant::now();
        // }

        // Adding files, folders, playlists, importing, exporting, etc
        self.ui_file(ctx, ui, visible_settings);

        // Something to do with editing things
        self.ui_edit(ui);

        // Something to do with the window
        self.ui_window(ui);

        // Useful links
        self.ui_help(ui);

        // Extra
        self.ui_extra(ui);
    }

    fn ui_file(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, visible_settings: &mut bool) {
        ui.menu_button("File", |ui| {
            if ui.button("Preferences").clicked() {
                *visible_settings = true;
            } else if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn ui_edit(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Edit", |_ui| {
            todo!();
        });
    }

    fn ui_window(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Window", |_ui| {
            todo!();
        });
    }

    fn ui_help(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Help", |ui| {
            ui.hyperlink_to("Github Repository", "https://github.com/Xithrius/drakn");
        });
    }

    fn ui_extra(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                // Theme switcher
                egui::widgets::global_theme_preference_switch(ui);

                // Debug build status
                egui::warn_if_debug_build(ui);

                // CPU usage
                // ui.label(format!("App CPU Usage: {:.2}%", self.system_info.cpu_usage));
            })
        });
    }
}
