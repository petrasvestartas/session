/// The production command panel: folded input or resizable history.
pub(crate) fn panel(root: &egui::Ui, expanded: bool, extra: f32) -> egui::Panel {
    let panel = if !expanded {
        egui::Panel::bottom("command-line-collapsed").exact_size(30.0 + extra)
    } else {
        egui::Panel::bottom("command-line")
            .default_size(104.0)
            .resizable(true)
            .size_range((64.0 + extra)..=(root.available_height() * 0.75).max(104.0 + extra))
    };
    panel.show_separator_line(false).frame(
        egui::Frame::new()
            .fill(egui::Color32::WHITE)
            .inner_margin(egui::Margin::symmetric(6, 4)),
    )
}

/// Set the dock's spacing, type size and top rule.
pub(crate) fn prepare(ui: &mut egui::Ui) {
    ui.set_min_height(ui.max_rect().height());
    ui.painter().hline(
        ui.max_rect().x_range().expand(6.0),
        ui.max_rect().top() - 4.0,
        egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
    );
    ui.set_clip_rect(ui.max_rect().expand(6.0));
    ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
}

/// Draw the same text field for the introductory and complete command handlers.
pub(crate) fn field(
    ui: &mut egui::Ui,
    command: &mut String,
    hint: &str,
    option_width: f32,
    focused: bool,
) -> egui::Response {
    let id = egui::Id::new("command-input");
    let response = ui.add_sized(
        [(ui.available_width() - option_width - 28.0).max(40.0), 22.0],
        egui::TextEdit::singleline(command)
            .id(id)
            .font(egui::FontId::proportional(14.0))
            .vertical_align(egui::Align::Center)
            .frame(egui::Frame::NONE)
            .clip_text(true)
            .char_limit(2048)
            .hint_text(hint),
    );
    // caret visible on an empty field
    if focused && command.is_empty() {
        let y = response.rect.center().y;
        ui.painter().vline(
            response.rect.left(),
            y - 7.0..=y + 7.0,
            ui.visuals().text_cursor.stroke,
        );
    }
    response
}
