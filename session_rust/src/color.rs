use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use serde_json::Value;

/// A color with RGBA values.
///
/// # Fields
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)  
/// * `b` - Blue component (0-255)
/// * `a` - Alpha component (0-100)
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create new color.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    /// Create white color.
    pub fn white() -> Self {
        Color::new(255, 255, 255, 100)
    }

    /// Create black color.
    pub fn black() -> Self {
        Color::new(0, 0, 0, 100)
    }

    /// Convert to float array [0-1].
    pub fn to_float_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 100.0]
    }

    /// Create from float values [0-1].
    pub fn from_float(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 100.0).round() as u8,
        )
    }

}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={}, a={})", self.r, self.g, self.b, self.a)
    }
}

// Custom Serialize implementation
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Color", 4)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("a", &self.a)?;
        state.end()
    }
}

// JSON
impl Color {
    /// Convert to JSON data.
    pub fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "Color",
            "r": self.r,
            "g": self.g,
            "b": self.b,
            "a": self.a
        })
    }

    /// Create from JSON data.
    pub fn from_json_data(data: &Value) -> Self {
        Color {
            r: data["r"].as_u64().unwrap() as u8,
            g: data["g"].as_u64().unwrap() as u8,
            b: data["b"].as_u64().unwrap() as u8,
            a: data["a"].as_u64().unwrap() as u8,
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self, _minimal: bool) -> String {
        serde_json::to_string(&self.to_json_data(_minimal)).unwrap()
    }

    /// Deserialize from JSON string.
    pub fn from_json(json_str: &str) -> Self {
        let data: Value = serde_json::from_str(json_str).unwrap();
        Self::from_json_data(&data)
    }
}
