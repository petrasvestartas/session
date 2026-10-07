    let mut focus_canvas = false;
    // as tall as the field, so the label and the typed text share one centre line
    let size = egui::vec2(ui.available_width(), 22.0);
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    ui.allocate_ui_with_layout(size, layout, |ui| {
