use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke { pub start: [f32; 3], pub end: [f32; 3], pub colour: [f32; 4], pub width: f32 }

#[derive(Clone)]
pub struct PreparedLine { pub source: Rc<session_rust::Line>, pub display: Stroke }

impl PreparedLine {
    pub fn new(source: Rc<session_rust::Line>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let start = std::array::from_fn(|i| source.start()[i] as f32);
        let end = std::array::from_fn(|i| source.end()[i] as f32);
        let colour = source.linecolor.to_f32();
        let width = if source.width.is_finite() && source.width > 0.0 { source.width as f32 } else { 1.0 };
        let width = if width == 0.0 { 1.0 } else { width };
        let display = Stroke { start, end, colour, width };
        if display.values().iter().any(|value| !value.is_finite()) { return Err("Line exceeds the display range"); }
        Ok(Self { source, display })
    }
}

impl Stroke {
    pub fn values(&self) -> [f32; 11] {
        let [a, b, c] = self.start; let [d, e, f] = self.end; let [r, g, blue, alpha] = self.colour;
        [a, b, c, d, e, f, r, g, blue, alpha, self.width]
    }

    pub fn bytes(&self) -> Vec<u8> { self.values().into_iter().flat_map(f32::to_ne_bytes).collect() }
}
