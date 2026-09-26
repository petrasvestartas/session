// --8<-- [start:theme-fonts]
/// The subset label fonts inside the binary; the panels use them until the whole fonts are downloaded.
pub(super) const BUNDLED: [&[u8]; 3] = [
    crate::engine::text::FONT_BYTES,
    crate::engine::text::FALLBACK_BYTES,
    crate::engine::text::SYMBOL_BYTES,
];

/// The panel fonts: the label fonts, Noto Sans first, then its symbol fallbacks.
pub(super) fn fonts(faces: [&'static [u8]; 3]) -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::empty();
    let names = ["Noto Sans", "Noto Sans Symbols 2", "Noto Sans Symbols"];

    for (name, bytes) in names.into_iter().zip(faces) {
        // `from_static` borrows the bytes already in the binary instead of copying them
        let data = std::sync::Arc::new(egui::FontData::from_static(bytes));
        fonts.font_data.insert(name.to_owned(), data);

        // a family is a fallback list: a glyph missing from Noto Sans, e.g. ⌘, is looked up in the next font
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .push(name.to_owned());
        }
    }

    fonts
}
// --8<-- [end:theme-fonts]

// --8<-- [start:theme-visuals]
/// The white theme: black text, white fields, light grey panels, no borders.
pub(super) fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.panel_fill = egui::Color32::from_gray(245);
    visuals.window_stroke = egui::Stroke::NONE;
    visuals.extreme_bg_color = egui::Color32::WHITE; // the inside of text fields
    visuals.selection.bg_fill = egui::Color32::from_rgb(200, 222, 245);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::BLACK);
    visuals.text_cursor.stroke = egui::Stroke::new(1.5_f32, egui::Color32::BLACK);
    visuals.text_cursor.blink = false; // a blink would ask for a new frame twice a second; frames are drawn on demand
    visuals.indent_has_left_vline = false;

    // the five states a widget can be in, from plain label to open menu, share one flat look
    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.bg_stroke = egui::Stroke::NONE;
        widget.bg_fill = egui::Color32::from_gray(245);
        widget.weak_bg_fill = egui::Color32::WHITE;
        widget.fg_stroke.color = egui::Color32::BLACK;
    }

    visuals
}
// --8<-- [end:theme-visuals]
