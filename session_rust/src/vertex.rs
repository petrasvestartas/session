use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// A graph vertex with a unique identifier and attribute string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Vertex")]
pub struct Vertex {
    /// The unique identifier of the vertex.
    pub guid: String,
    /// The name of the vertex.
    pub name: String,
    /// Vertex attribute data as string.
    pub attribute: String,
    /// Integer index for the vertex. Set internally by Graph.
    pub index: i32,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            name: "my_vertex".to_string(),
            guid: Uuid::new_v4().to_string(),
            attribute: String::new(),
            index: -1,
        }
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vertex({}, {}, attr={}, index={})",
            self.name, self.guid, self.attribute, self.index
        )
    }
}

impl Vertex {
    /// Initialize a new Vertex.
    pub fn new(name: Option<String>, attribute: Option<String>) -> Self {
        Self {
            name: name.unwrap_or_default(),
            attribute: attribute.unwrap_or_default(),
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert the Vertex to a JSON-serializable string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Create Vertex from JSON string data.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }
}

#[cfg(test)]
#[path = "vertex_test.rs"]
mod vertex_test;
