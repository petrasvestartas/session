// --8<-- [start:overlay-drawing]
// Painter = egui's pen for loose lines, boxes and text, clipped here to the scene area.
/// The shape being drawn: its points joined, a square and the prompt at the last; in device pixels.
pub(super) fn drawing(painter: &egui::Painter, drawing: &(Vec<(f64, f64)>, String), scale: f32) {
    let points: Vec<egui::Pos2> = drawing
        .0
        .iter()
        // device pixels to egui points: at scale 2.0, pixel 400 is point 200
        .map(|p| egui::pos2(p.0 as f32 / scale, p.1 as f32 / scale))
        .collect();

    // `windows(2)` yields each neighbouring pair: 4 points give 3 segments
    for pair in points.windows(2) {
        painter.line_segment(
            [pair[0], pair[1]],
            egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(30, 110, 170)),
        );
    }

    if let Some(p) = points.last() {
        painter.rect_stroke(
            egui::Rect::from_center_size(*p, egui::vec2(8.0, 8.0)),
            0.0,
            egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(30, 110, 170)),
            egui::StrokeKind::Middle,
        );
        painter.text(
            *p + egui::vec2(10.0, -12.0),
            egui::Align2::LEFT_BOTTOM,
            &drawing.1,
            egui::FontId::proportional(14.0),
            egui::Color32::from_rgb(20, 80, 130),
        );
    }
}
// --8<-- [end:overlay-drawing]
