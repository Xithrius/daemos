use crossbeam::channel::Sender;
use egui::RichText;
use serde::{Deserialize, Serialize};

use crate::playback::state::PlayerCommand;

const PLAYBACK_BUTTON_FONT_SIZE: f32 = 22.5;

const SKIP_BACKWARD_SYMBOL: &str = "\u{23EE}"; // ⏮
const PLAY_SYMBOL: &str = "\u{25B6}"; // ▶
const PAUSE_SYMBOL: &str = "\u{23F8}"; // ⏸
const SKIP_FORWARD_SYMBOL: &str = "\u{23ED}"; // ⏭

#[derive(Debug, Clone)]
pub struct PlaybackBar {
    tx: Sender<PlayerCommand>,
}

impl PlaybackBar {
    pub fn new(tx: Sender<PlayerCommand>) -> Self {
        Self { tx }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .button(RichText::new(SKIP_BACKWARD_SYMBOL).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
            {
                todo!();
            }
            if ui
                .button(RichText::new(PLAY_SYMBOL).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
            {
                todo!();
            }
            if ui
                .button(RichText::new(PAUSE_SYMBOL).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
            {
                todo!();
            }
            if ui
                .button(RichText::new(SKIP_FORWARD_SYMBOL).size(PLAYBACK_BUTTON_FONT_SIZE))
                .clicked()
            {
                todo!();
            }
        });

        // TableBuilder::new(ui)
        //     .max_scroll_height(available_height)
        //     .column(Column::auto().at_least(50.0).resizable(true))
        //     .column(Column::remainder())
        //     .header(TABLE_HEADER_HEIGHT, |mut header| {
        //         header.col(|ui| {
        //             ui.heading("Index");
        //         });
        //         header.col(|ui| {
        //             ui.heading("Track");
        //         });
        //     })
        //     .body(|body| {
        //         let num_rows = self.tracks.len();

        //         body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
        //             let row_index = row.index();

        //             let Some(track) = self.tracks.get(row_index) else {
        //                 return;
        //             };

        //             let Some(track_file_name) = get_track_file_name(track.to_path_buf()) else {
        //                 return;
        //             };

        //             row.col(|ui| {
        //                 ui.label(row_index.to_string());
        //             });

        //             row.col(|ui| {
        //                 ui.label(track_file_name);
        //             });
        //         });
        //     });
    }
}
