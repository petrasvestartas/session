
/// A tool's parts, else a measured answer.
pub(super) fn marks(painter: &egui::Painter, state: &crate::State, scale: f32) {
    let mut marks = state.tool_marks();

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
