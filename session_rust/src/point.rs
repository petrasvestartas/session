use std::fmt;
use serde::{Deserialize, Serialize, ser::Serialize as SerTrait};
use uuid::Uuid;
use crate::Color;

/// A point with XYZ coordinates and display properties.
///
/// # Fields
/// * `x` - X coordinate
/// * `y` - Y coordinate  
/// * `z` - Z coordinate
/// * `guid` - Unique identifier
/// * `name` - Point name
/// * `pointcolor` - Point color
/// * `width` - Point width
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Point")]
pub struct Point {
    pub guid: Uuid,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub pointcolor: Color,
    
}


impl Point {
    /// Create new point.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            guid: Uuid::new_v4(),
            name: "my_point".to_string(),
            pointcolor: Color::white(),
            width: 1.0,
        }
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

impl Default for Point {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
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
