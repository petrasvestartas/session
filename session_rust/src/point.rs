use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::Color;
use serde_json::Value;
use std::fs;

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
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub guid: Uuid,
    pub name: String,
    pub pointcolor: Color,
    pub width: f32,
}

impl Point {
    /// Create new point.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            guid: Uuid::new_v4(),
            name: "Point".to_string(),
            pointcolor: Color::white(),
            width: 1.0,
        }
    }
}

// JSON
impl Point {
    /// Convert to JSON data.
    pub fn to_json_data(&self, minimal: bool) -> Value {
        if minimal {
            serde_json::json!({"dtype": "Point", "x": self.x, "y": self.y, "z": self.z})
        } else {
            serde_json::json!({
                "dtype": "Point", "x": self.x, "y": self.y, "z": self.z,
                "guid": self.guid.to_string(), "name": self.name,
                "pointcolor": self.pointcolor.to_json_data(false), "width": self.width
            })
        }
    }

    /// Create from JSON data.
    pub fn from_json_data(data: &Value) -> Self {
        Point {
            x: data["x"].as_f32().unwrap(),
            y: data["y"].as_f32().unwrap(),
            z: data["z"].as_f32().unwrap(),
            guid: data.get("guid").and_then(|g| g.as_str()).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or_else(Uuid::new_v4),
            name: data.get("name").and_then(|n| n.as_str()).unwrap_or("Point").to_string(),
            pointcolor: data.get("pointcolor").map(Color::from_json_data).unwrap_or_else(Color::white),
            width: data.get("width").and_then(|w| w.as_f32()).unwrap_or(1.0),
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self, minimal: bool) -> String {
        serde_json::to_string(&self.to_json_data(minimal)).unwrap()
    }

    /// Deserialize from JSON string.
    pub fn from_json(json_str: &str) -> Self {
        let data: Value = serde_json::from_str(json_str).unwrap();
        Self::from_json_data(&data)
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
