use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// A color with RGBA values and JSON serialization support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Color")]
pub struct Color {
    pub guid: String,
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
            guid: Uuid::new_v4().to_string(),
            name: "Color".to_string(),
            r,
            g,
            b,
            a,
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Presets
    ///////////////////////////////////////////////////////////////////////////////////////////

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

    /// Create grey color.
    pub fn grey() -> Self {
        let mut color = Color::new(128, 128, 128, 255);
        color.name = "grey".to_string();
        color
    }

    /// Create red color.
    pub fn red() -> Self {
        let mut color = Color::new(255, 0, 0, 255);
        color.name = "red".to_string();
        color
    }

    /// Create orange color.
    pub fn orange() -> Self {
        let mut color = Color::new(255, 128, 0, 255);
        color.name = "orange".to_string();
        color
    }

    /// Create yellow color.
    pub fn yellow() -> Self {
        let mut color = Color::new(255, 255, 0, 255);
        color.name = "yellow".to_string();
        color
    }

    /// Create lime color.
    pub fn lime() -> Self {
        let mut color = Color::new(128, 255, 0, 255);
        color.name = "lime".to_string();
        color
    }

    /// Create green color.
    pub fn green() -> Self {
        let mut color = Color::new(0, 255, 0, 255);
        color.name = "green".to_string();
        color
    }

    /// Create mint color.
    pub fn mint() -> Self {
        let mut color = Color::new(0, 255, 128, 255);
        color.name = "mint".to_string();
        color
    }

    /// Create cyan color.
    pub fn cyan() -> Self {
        let mut color = Color::new(0, 255, 255, 255);
        color.name = "cyan".to_string();
        color
    }

    /// Create azure color.
    pub fn azure() -> Self {
        let mut color = Color::new(0, 128, 255, 255);
        color.name = "azure".to_string();
        color
    }

    /// Create blue color.
    pub fn blue() -> Self {
        let mut color = Color::new(0, 0, 255, 255);
        color.name = "blue".to_string();
        color
    }

    /// Create violet color.
    pub fn violet() -> Self {
        let mut color = Color::new(128, 0, 255, 255);
        color.name = "violet".to_string();
        color
    }

    /// Create magenta color.
    pub fn magenta() -> Self {
        let mut color = Color::new(255, 0, 255, 255);
        color.name = "magenta".to_string();
        color
    }

    /// Create pink color.
    pub fn pink() -> Self {
        let mut color = Color::new(255, 0, 128, 255);
        color.name = "pink".to_string();
        color
    }

    /// Create maroon color.
    pub fn maroon() -> Self {
        let mut color = Color::new(128, 0, 0, 255);
        color.name = "maroon".to_string();
        color
    }

    /// Create brown color.
    pub fn brown() -> Self {
        let mut color = Color::new(128, 64, 0, 255);
        color.name = "brown".to_string();
        color
    }

    /// Create olive color.
    pub fn olive() -> Self {
        let mut color = Color::new(128, 128, 0, 255);
        color.name = "olive".to_string();
        color
    }

    /// Create teal color.
    pub fn teal() -> Self {
        let mut color = Color::new(0, 128, 128, 255);
        color.name = "teal".to_string();
        color
    }

    /// Create navy color.
    pub fn navy() -> Self {
        let mut color = Color::new(0, 0, 128, 255);
        color.name = "navy".to_string();
        color
    }

    /// Create purple color.
    pub fn purple() -> Self {
        let mut color = Color::new(128, 0, 128, 255);
        color.name = "purple".to_string();
        color
    }

    /// Create silver color.
    pub fn silver() -> Self {
        let mut color = Color::new(192, 192, 192, 255);
        color.name = "silver".to_string();
        color
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert to float array [0-1].
    pub fn to_float_array(&self) -> [f64; 4] {
        [
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
            self.a as f64 / 255.0,
        ]
    }

    /// Create from float values [0-1].
    pub fn from_float(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        )
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serialize to JSON string (for cross-language compatibility)
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserialize from JSON string (for cross-language compatibility)
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serialize to JSON file
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserialize from JSON file
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color(r={}, g={}, b={}, a={}, name={})",
            self.r, self.g, self.b, self.a, self.name
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.r == other.r
            && self.g == other.g
            && self.b == other.b
            && self.a == other.a
    }
}

#[cfg(test)]
#[path = "color_test.rs"]
mod color_test;
