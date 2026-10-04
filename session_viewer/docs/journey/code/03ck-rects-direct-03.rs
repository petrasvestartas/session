    false
}

pub(crate) fn collapse(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
    let response = ui
        .button(if model.command_expanded { "–" } else { "+" })
        .on_hover_text("Collapse or expand history");
    record(
        controls,
        "command/collapse",
        "Collapse or expand history",
        &response,
    );
    if response.clicked() {
        model.command_expanded = !model.command_expanded;
        ui.ctx().request_repaint();
    }
}
