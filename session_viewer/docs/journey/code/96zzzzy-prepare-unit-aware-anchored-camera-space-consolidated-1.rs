#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Unit {
    Millimeters,
    Meters,
}
impl Unit {
    pub fn suffix(self, power: usize) -> &'static str {
        let values = match self {
            Self::Millimeters => ["mm", "mm²", "mm³"],
            Self::Meters => ["m", "m²", "m³"],
        };
        values[power.clamp(1, 3) - 1]
    }
}

pub const NEAR_FRACTION:f64=1.0e-4;