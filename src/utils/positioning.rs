pub fn centered_position(ctx: &egui::Context, window_size: [f32; 2]) -> egui::Pos2 {
    let screen_rect = ctx.screen_rect();
    let center = screen_rect.center();

    egui::pos2(
        center.x - window_size[0] / 2.0,
        center.y - window_size[1] / 2.0,
    )
}
