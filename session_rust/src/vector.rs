use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// A 3D vector with visual properties and JSON serialization support.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename = "Vector")]
pub struct Vector {
    pub guid: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// Creates a new Vector with specified coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            guid: Uuid::new_v4().to_string(),
            name: "my_vector".to_string(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Vector to a JSON string.
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes a Vector from a JSON string.
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Vector to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Vector from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vector({}, {}, {}, {}, {})",
            self.x, self.y, self.z, self.guid, self.name
        )
    }
}
