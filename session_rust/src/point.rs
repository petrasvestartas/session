use crate::Color;
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// A 3D point with visual properties and JSON serialization support.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Point")]
pub struct Point {
    pub guid: String,      // Unique identifier
    pub name: String,      // Name of the point
    pub x: f32,            // X coordinate
    pub y: f32,            // Y coordinate
    pub z: f32,            // Z coordinate
    pub width: f32,        // Width of the point
    pub pointcolor: Color, // Color of the point
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            guid: Uuid::new_v4().to_string(),
            name: "my_point".to_string(),
            pointcolor: Color::white(),
            width: 1.0,
        }
    }
}

impl Point {
    /// Creates a new Point with specified coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Point to a JSON string.
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Point from a JSON string.
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Point to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Point from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point({}, {}, {}, {}, {}, {}, {})",
            self.x, self.y, self.z, self.guid, self.name, self.pointcolor, self.width
        )
    }
}
