    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}
