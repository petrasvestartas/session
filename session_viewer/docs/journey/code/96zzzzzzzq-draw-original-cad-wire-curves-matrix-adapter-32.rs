
    pub(super) fn label(text: &str) -> TextLabel {
        TextLabel {
            bare: false, object: None,
            id: 1,
            text: text.into(),
            font_size: 16.0,
