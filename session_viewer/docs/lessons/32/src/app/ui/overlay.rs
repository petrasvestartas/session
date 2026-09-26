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

// --8<-- [start:23a-tool-marks]
// --8<-- [start:tool-marks]
/// A tool's parts, else a measured answer.
pub(super) fn marks(painter: &egui::Painter, state: &crate::State, scale: f32) {
    let mut marks = state.tool_marks();
    marks = marks.or_else(|| state.mark_overlay()); // register:annotate

    if let Some(marks) = &marks {
        tool_marks(painter, marks, scale);
    }
}

/// Paint a tool's strokes, squares and label; points arrive in device pixels.
pub(super) fn tool_marks(
    painter: &egui::Painter,
    marks: &crate::app::command::tool::Overlay,
    scale: f32,
) {
    // a closure named once, used for every stroke, square and label below
    let at = |p: &(f64, f64)| egui::pos2(p.0 as f32 / scale, p.1 as f32 / scale);

    for stroke in &marks.strokes {
        let points: Vec<egui::Pos2> = stroke.points.iter().map(at).collect();
        let [r, g, b] = stroke.color;
        let pen = egui::Stroke::new(stroke.width, egui::Color32::from_rgb(r, g, b));

        if stroke.dashed {
            // 8 points of line, 5 of gap
            painter.extend(egui::Shape::dashed_line(&points, pen, 8.0, 5.0));
        } else {
            painter.line(points, pen);
        }
    }

    let pen = egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(20, 20, 20));

    for mark in &marks.marks {
        painter.rect_stroke(
            egui::Rect::from_center_size(at(mark), egui::vec2(7.0, 7.0)),
            0.0,
            pen,
            egui::StrokeKind::Middle,
        );
    }

    // a white-on-black chip like the selected-name label
    if let Some((p, text)) = &marks.label {
        let galley = painter.layout_no_wrap(
            text.clone(),
            egui::FontId::proportional(13.5),
            egui::Color32::WHITE,
        );
        // Galley = a laid-out piece of text that knows its size before it is painted
        let height = galley.size().y + 6.0;
        let chip =
            egui::Rect::from_center_size(at(p), egui::vec2(galley.size().x + height, height));
        painter.rect_filled(chip, height * 0.5, egui::Color32::BLACK);
        painter.galley(
            chip.center() - galley.size() * 0.5,
            galley,
            egui::Color32::WHITE,
        );
    }
}
// --8<-- [end:tool-marks]
// --8<-- [end:23a-tool-marks]
