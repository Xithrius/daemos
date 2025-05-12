use std::collections::HashSet;

use color_eyre::Result;
use crossbeam::channel::Sender;
use egui_extras::{Column, TableBuilder};
use tracing::{debug, error};

use crate::{
    database::{connection::SharedDatabase, models::tracks::Track},
    files::open::get_track_file_name,
    playback::state::PlayerCommand,
};

const TABLE_HEADER_HEIGHT: f32 = 25.0;
const TABLE_ROW_HEIGHT: f32 = 20.0;

#[derive(Debug, Clone)]
pub struct Table {
    tracks: Vec<Track>,
    #[allow(dead_code)]
    selection: HashSet<usize>,
    playing: Option<(usize, Track)>,

    tx: Sender<PlayerCommand>,
}

impl Table {
    pub fn new(database: SharedDatabase, tx: Sender<PlayerCommand>) -> Self {
        let tracks = match Track::select_all(database).map(|tracks| tracks.to_vec()) {
            Ok(tracks) => {
                debug!(
                    "Initial load of track table found {} track(s)",
                    tracks.len()
                );

                tracks
            }
            Err(err) => {
                error!("Failed getting tracks: {}", err);

                Vec::new()
            }
        };

        Self {
            tracks,
            selection: HashSet::default(),
            playing: None,
            tx,
        }
    }

    pub fn refresh_tracks(&mut self, database: SharedDatabase) -> Result<()> {
        let tracks = match Track::select_all(database).map(|tracks| tracks.to_vec()) {
            Ok(tracks) => {
                debug!("Refreshed tracks list with {} track(s)", tracks.len());

                tracks
            }
            Err(err) => {
                error!("Failed getting tracks: {}", err);

                Vec::new()
            }
        };

        self.tracks = tracks;

        Ok(())
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, height: f32) {
        let mut table = TableBuilder::new(ui)
            .max_scroll_height(height)
            .column(Column::auto().at_least(50.0).resizable(true))
            .column(Column::remainder());

        table = table.sense(egui::Sense::click());

        table
            .header(TABLE_HEADER_HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.heading("Index");
                });
                header.col(|ui| {
                    ui.heading("Track");
                });
            })
            .body(|body| {
                let num_rows = self.tracks.len();

                body.rows(TABLE_ROW_HEIGHT, num_rows, |mut row| {
                    let row_index = row.index();

                    let track = self.tracks.get(row_index).cloned();
                    let playing = self.playing.clone();

                    if let Some(track) = track {
                        if let Some(track_file_name) = get_track_file_name(track.path.clone()) {
                            row.set_selected(
                                playing
                                    .as_ref()
                                    .is_some_and(|(index, _)| *index == row_index),
                            );

                            row.col(|ui| {
                                let label = ui
                                    .label(row_index.to_string())
                                    .on_hover_cursor(egui::CursorIcon::Default);
                                if label.double_clicked() {
                                    self.toggle_row_play(row_index, &track);
                                }
                            });

                            row.col(|ui| {
                                let label = ui
                                    .label(track_file_name)
                                    .on_hover_cursor(egui::CursorIcon::Default);
                                if label.double_clicked() {
                                    self.toggle_row_play(row_index, &track);
                                }
                            });

                            if row.response().double_clicked() {
                                self.toggle_row_play(row_index, &track);
                            }
                        }
                    }
                });
            });
    }

    // fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
    //     if row_response.clicked() {
    //         if self.selection.contains(&row_index) {
    //             self.selection.remove(&row_index);
    //         } else {
    //             self.selection.insert(row_index);
    //         }
    //     }
    // }

    fn toggle_row_play(&mut self, row_index: usize, track: &Track) {
        // TODO: If paused, and you hit double click again, then it will send another pause.
        // TODO: Keep track of track state.
        if self
            .playing
            .as_ref()
            .is_some_and(|(playing_index, playing_track)| {
                (*playing_index == row_index) && (*playing_track == *track)
            })
        {
            if let Err(err) = self.tx.send(PlayerCommand::Pause) {
                error!("Failed to pause track on path {:?}: {}", track.path, err);
            }

            return;
        }

        if let Err(err) = self.tx.send(PlayerCommand::Create(track.clone())) {
            error!("Failed to start track on path {:?}: {}", track.path, err);
            return;
        }

        self.playing = Some((row_index, track.clone()))
    }
}

// impl Widget for Table {
//     fn ui(self, ui: &mut egui::Ui) -> Response {
//         let mut reset = false;

//         ui.vertical_centered(|ui| {
//             ui.heading("Table Demo");
//             if ui.button("Reset").clicked() {
//                 reset = true;
//             }
//         });

//         Response::default()
//     }
// }

// use egui::{TextStyle, TextWrapMode, Widget};
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, PartialEq)]
// enum DemoType {
//     Manual,
//     ManyHomogeneous,
//     ManyHeterogenous,
// }

// #[derive(Deserialize, Serialize, PartialEq)]
// pub struct TableDemo {
//     demo: DemoType,
//     striped: bool,
//     overline: bool,
//     resizable: bool,
//     clickable: bool,
//     num_rows: usize,
//     scroll_to_row_slider: usize,
//     scroll_to_row: Option<usize>,
//     selection: std::collections::HashSet<usize>,
//     checked: bool,
//     reversed: bool,
// }

// impl Default for TableDemo {
//     fn default() -> Self {
//         Self {
//             demo: DemoType::Manual,
//             striped: true,
//             overline: true,
//             resizable: true,
//             clickable: true,
//             num_rows: 10_000,
//             scroll_to_row_slider: 0,
//             scroll_to_row: None,
//             selection: Default::default(),
//             checked: false,
//             reversed: false,
//         }
//     }
// }

// const NUM_MANUAL_ROWS: usize = 20;

// impl Widget for TableDemo {
//     fn ui(self, ui: &mut egui::Ui) -> egui::Response {
//         let mut reset = false;

//         ui.vertical(|ui| {
//             ui.horizontal(|ui| {
//                 ui.checkbox(&mut self.striped, "Striped");
//                 ui.checkbox(&mut self.overline, "Overline some rows");
//                 ui.checkbox(&mut self.resizable, "Resizable columns");
//                 ui.checkbox(&mut self.clickable, "Clickable rows");
//             });

//             ui.label("Table type:");
//             ui.radio_value(&mut self.demo, DemoType::Manual, "Few, manual rows");
//             ui.radio_value(
//                 &mut self.demo,
//                 DemoType::ManyHomogeneous,
//                 "Thousands of rows of same height",
//             );
//             ui.radio_value(
//                 &mut self.demo,
//                 DemoType::ManyHeterogenous,
//                 "Thousands of rows of differing heights",
//             );

//             if self.demo != DemoType::Manual {
//                 ui.add(
//                     egui::Slider::new(&mut self.num_rows, 0..=100_000)
//                         .logarithmic(true)
//                         .text("Num rows"),
//                 );
//             }

//             {
//                 let max_rows = if self.demo == DemoType::Manual {
//                     NUM_MANUAL_ROWS
//                 } else {
//                     self.num_rows
//                 };

//                 let slider_response = ui.add(
//                     egui::Slider::new(&mut self.scroll_to_row_slider, 0..=max_rows)
//                         .logarithmic(true)
//                         .text("Row to scroll to"),
//                 );
//                 if slider_response.changed() {
//                     self.scroll_to_row = Some(self.scroll_to_row_slider);
//                 }
//             }

//             reset = ui.button("Reset").clicked();
//         });

//         ui.separator();

//         // Leave room for the source code link after the table demo:
//         let body_text_size = TextStyle::Body.resolve(ui.style()).size;
//         use egui_extras::{Size, StripBuilder};
//         StripBuilder::new(ui)
//             .size(Size::remainder().at_least(100.0)) // for the table
//             .size(Size::exact(body_text_size)) // for the source code link
//             .vertical(|mut strip| {
//                 strip.cell(|ui| {
//                     egui::ScrollArea::horizontal().show(ui, |ui| {
//                         self.table_ui(ui, reset);
//                     });
//                 });
//                 strip.cell(|ui| {
//                     ui.vertical_centered(|ui| {
//                         ui.add(crate::egui_github_link_file!());
//                     });
//                 });
//             });
//     }
// }

// impl TableDemo {
//     fn table_ui(&mut self, ui: &mut egui::Ui, reset: bool) {
//         use egui_extras::{Column, TableBuilder};

//         let text_height = egui::TextStyle::Body
//             .resolve(ui.style())
//             .size
//             .max(ui.spacing().interact_size.y);

//         let available_height = ui.available_height();
//         let mut table = TableBuilder::new(ui)
//             .striped(self.striped)
//             .resizable(self.resizable)
//             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
//             .column(Column::auto())
//             .column(
//                 Column::remainder()
//                     .at_least(40.0)
//                     .clip(true)
//                     .resizable(true),
//             )
//             .column(Column::auto())
//             .column(Column::remainder())
//             .column(Column::remainder())
//             .min_scrolled_height(0.0)
//             .max_scroll_height(available_height);

//         if self.clickable {
//             table = table.sense(egui::Sense::click());
//         }

//         if let Some(row_index) = self.scroll_to_row.take() {
//             table = table.scroll_to_row(row_index, None);
//         }

//         if reset {
//             table.reset();
//         }

//         table
//             .header(20.0, |mut header| {
//                 header.col(|ui| {
//                     egui::Sides::new().show(
//                         ui,
//                         |ui| {
//                             ui.strong("Row");
//                         },
//                         |ui| {
//                             self.reversed ^=
//                                 ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
//                         },
//                     );
//                 });
//                 header.col(|ui| {
//                     ui.strong("Clipped text");
//                 });
//                 header.col(|ui| {
//                     ui.strong("Expanding content");
//                 });
//                 header.col(|ui| {
//                     ui.strong("Interaction");
//                 });
//                 header.col(|ui| {
//                     ui.strong("Content");
//                 });
//             })
//             .body(|mut body| match self.demo {
//                 DemoType::Manual => {
//                     for row_index in 0..NUM_MANUAL_ROWS {
//                         let row_index = if self.reversed {
//                             NUM_MANUAL_ROWS - 1 - row_index
//                         } else {
//                             row_index
//                         };

//                         let is_thick = thick_row(row_index);
//                         let row_height = if is_thick { 30.0 } else { 18.0 };
//                         body.row(row_height, |mut row| {
//                             row.set_selected(self.selection.contains(&row_index));
//                             row.set_overline(self.overline && row_index % 7 == 3);

//                             row.col(|ui| {
//                                 ui.label(row_index.to_string());
//                             });
//                             row.col(|ui| {
//                                 ui.label(long_text(row_index));
//                             });
//                             row.col(|ui| {
//                                 expanding_content(ui);
//                             });
//                             row.col(|ui| {
//                                 ui.checkbox(&mut self.checked, "Click me");
//                             });
//                             row.col(|ui| {
//                                 ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
//                                 if is_thick {
//                                     ui.heading("Extra thick row");
//                                 } else {
//                                     ui.label("Normal row");
//                                 }
//                             });

//                             self.toggle_row_selection(row_index, &row.response());
//                         });
//                     }
//                 }
//                 DemoType::ManyHomogeneous => {
//                     body.rows(text_height, self.num_rows, |mut row| {
//                         let row_index = if self.reversed {
//                             self.num_rows - 1 - row.index()
//                         } else {
//                             row.index()
//                         };

//                         row.set_selected(self.selection.contains(&row_index));
//                         row.set_overline(self.overline && row_index % 7 == 3);

//                         row.col(|ui| {
//                             ui.label(row_index.to_string());
//                         });
//                         row.col(|ui| {
//                             ui.label(long_text(row_index));
//                         });
//                         row.col(|ui| {
//                             expanding_content(ui);
//                         });
//                         row.col(|ui| {
//                             ui.checkbox(&mut self.checked, "Click me");
//                         });
//                         row.col(|ui| {
//                             ui.add(
//                                 egui::Label::new("Thousands of rows of even height")
//                                     .wrap_mode(TextWrapMode::Extend),
//                             );
//                         });

//                         self.toggle_row_selection(row_index, &row.response());
//                     });
//                 }
//                 DemoType::ManyHeterogenous => {
//                     let row_height = |i: usize| if thick_row(i) { 30.0 } else { 18.0 };
//                     body.heterogeneous_rows((0..self.num_rows).map(row_height), |mut row| {
//                         let row_index = if self.reversed {
//                             self.num_rows - 1 - row.index()
//                         } else {
//                             row.index()
//                         };

//                         row.set_selected(self.selection.contains(&row_index));
//                         row.set_overline(self.overline && row_index % 7 == 3);

//                         row.col(|ui| {
//                             ui.label(row_index.to_string());
//                         });
//                         row.col(|ui| {
//                             ui.label(long_text(row_index));
//                         });
//                         row.col(|ui| {
//                             expanding_content(ui);
//                         });
//                         row.col(|ui| {
//                             ui.checkbox(&mut self.checked, "Click me");
//                         });
//                         row.col(|ui| {
//                             ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
//                             if thick_row(row_index) {
//                                 ui.heading("Extra thick row");
//                             } else {
//                                 ui.label("Normal row");
//                             }
//                         });

//                         self.toggle_row_selection(row_index, &row.response());
//                     });
//                 }
//             });
//     }

//     fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
//         if row_response.clicked() {
//             if self.selection.contains(&row_index) {
//                 self.selection.remove(&row_index);
//             } else {
//                 self.selection.insert(row_index);
//             }
//         }
//     }
// }

// fn expanding_content(ui: &mut egui::Ui) {
//     ui.add(egui::Separator::default().horizontal());
// }

// fn long_text(row_index: usize) -> String {
//     format!(
//         "Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!"
//     )
// }

// fn thick_row(row_index: usize) -> bool {
//     row_index % 6 == 0
// }
