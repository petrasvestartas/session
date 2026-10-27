pub fn label(text: &str, point: &Point, right: [f64; 3], up: [f64; 3], height: f64) -> TextLabel {
    let lift = height * f64::from(LINE_HEIGHT) / f64::from(FONT_SIZE); // the line box above the point
    TextLabel {
        bare: false, id: 0,
        object: None,
        text: text.into(),
        font_size: FONT_SIZE,
