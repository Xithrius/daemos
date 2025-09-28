use eframe::CreationContext;
use egui::{FontFamily, FontId, TextStyle};

const FONT_NAME: &str = "Space Mono";
static FONT_BYTES: &[u8; 98320] = include_bytes!("../static/fonts/SpaceMono-Regular.ttf");

// const FONT_NAME: &str = "JetBrains Mono Regular";
// static FONT_BYTES: &[u8; 273900] = include_bytes!("../static/fonts/JetBrainsMono-Regular.ttf");

const FONT_HEADING_SIZE: f32 = 16.0;
const FONT_BODY_SIZE: f32 = 14.0;
const FONT_BUTTON_SIZE: f32 = 12.0;
const FONT_SMALL_SIZE: f32 = 12.0;
const FONT_MONOSPACE_SIZE: f32 = 14.0;

pub fn set_fonts(cc: &CreationContext) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        String::from(FONT_NAME),
        egui::FontData::from_static(FONT_BYTES).into(),
    );
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .clear();
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push(FONT_NAME.to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push(FONT_NAME.to_owned());

    let mut style = (*cc.egui_ctx.style()).clone();

    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(FONT_HEADING_SIZE, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(FONT_BODY_SIZE, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(FONT_BUTTON_SIZE, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(FONT_SMALL_SIZE, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(FONT_MONOSPACE_SIZE, FontFamily::Monospace),
    );

    cc.egui_ctx.set_style(style);
}
