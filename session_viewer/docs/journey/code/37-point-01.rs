use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Marker { pub center: [f32; 3], pub diameter: f32, pub colour: [f32; 4] }

#[derive(Clone)]
pub struct PreparedPoint { pub source: Rc<session_rust::Point>, pub display: Marker }

impl PreparedPoint {
    pub fn new(source: Rc<session_rust::Point>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let diameter = if source.width.is_finite() && source.width > 0.0 && (source.width - 1.0).abs() > 1e-9 {
            source.width as f32
        } else { 6.0 };
        let diameter = if diameter == 0.0 { 6.0 } else { diameter };
        let display = Marker { center: source.to_f32(), diameter, colour: source.pointcolor.to_f32() };
        if display.values().iter().any(|v| !v.is_finite()) { return Err("Point marker exceeds the display range"); }
        Ok(Self { source, display })
    }
}

impl Marker {
    pub fn values(&self) -> [f32; 8] {
        let [x, y, z] = self.center; let [r, g, b, a] = self.colour;
        [x, y, z, self.diameter, r, g, b, a]
    }

    pub fn bytes(&self) -> Vec<u8> { self.values().into_iter().flat_map(f32::to_ne_bytes).collect() }
}
