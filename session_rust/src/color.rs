use std::fmt;
use serde::{Deserialize, Serialize, ser::Serialize as SerTrait};
use uuid::Uuid;

/// A color with RGBA values.
///
/// # Fields
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)
/// * `b` - Blue component (0-255)
/// * `a` - Alpha component (0-255)
/// * `guid` - Unique identifier
/// * `name` - Color name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Color")]
pub struct Color {
    pub guid: Uuid,
    pub name: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create new color.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { 
            guid: Uuid::new_v4(),
            name: "Color".to_string(),
            r, 
            g, 
            b, 
            a 
        }
    }

    /// Create white color.
    pub fn white() -> Self {
        let mut color = Color::new(255, 255, 255, 255);
        color.name = "white".to_string();
        color
    }

    /// Create black color.
    pub fn black() -> Self {
        let mut color = Color::new(0, 0, 0, 255);
        color.name = "black".to_string();
        color
    }

    /// Convert to float array [0-1].
    pub fn to_float_array(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }

    /// Create from float values [0-1].
    pub fn from_float(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        )
    }

    /// Serialize to JSON string (for cross-language compatibility)
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserialize from JSON string (for cross-language compatibility)
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serialize to JSON file
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserialize from JSON file
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r={}, g={}, b={}, a={}, name={})", self.r, self.g, self.b, self.a, self.name)
    }
}

