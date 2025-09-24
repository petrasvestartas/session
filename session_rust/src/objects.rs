use crate::Point;
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use std::fs;
use uuid::Uuid;

/// A collection of objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Objects")]
pub struct Objects {
    pub guid: String,
    pub name: String,
    #[serde(rename = "points")]
    pub vec: Vec<Point>,
}

impl Default for Objects {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_objects".to_string(),
            vec: Vec::new(),
        }
    }
}

impl Objects {
    pub fn new() -> Self {
        Self {
            name: "my_objects".to_string(),
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Objects to a JSON string.
    pub fn to_json_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes Objects from a JSON string.
    pub fn from_json_data(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Objects to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.to_json_data()?;
        fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes Objects from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        Self::from_json_data(&json)
    }
}

impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Objects({}, {}, points={})",
            self.name,
            self.guid,
            self.vec.len()
        )
    }
}
