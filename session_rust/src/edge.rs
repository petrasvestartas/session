use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge({}, {}, {} -> {}, attr={}, index={})",
            self.name, self.guid, self.v0, self.v1, self.attribute, self.index
        )
    }
}

/// A graph edge with a unique identifier and attribute string.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Edge")]
pub struct Edge {
    /// The unique identifier of the edge.
    pub guid: String,
    /// The name of the edge.
    pub name: String,
    /// The first vertex of the edge.
    pub v0: String,
    /// The second vertex of the edge.
    pub v1: String,
    /// Edge attribute data as string.
    pub attribute: String,
    /// Integer index for the edge.
    pub index: i32,
}

impl Default for Edge {
    fn default() -> Self {
        Self {
            name: "my_edge".to_string(),
            guid: Uuid::new_v4().to_string(),
            v0: String::new(),
            v1: String::new(),
            attribute: String::new(),
            index: -1,
        }
    }
}

impl Edge {
    /// Initialize a new Edge.
    pub fn new(
        name: Option<String>,
        v0: Option<String>,
        v1: Option<String>,
        attribute: Option<String>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            v0: v0.unwrap_or_default(),
            v1: v1.unwrap_or_default(),
            attribute: attribute.unwrap_or_default(),
            ..Default::default()
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Convert the Edge to a JSON-serializable string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Create Edge from JSON string data.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Get the edge vertices as a tuple.
    pub fn vertices(&self) -> (String, String) {
        (self.v0.clone(), self.v1.clone())
    }

    /// Check if this edge connects to a given vertex.
    pub fn connects(&self, vertex_id: &str) -> bool {
        self.v0 == vertex_id || self.v1 == vertex_id
    }

    /// Get the other vertex ID connected by this edge.
    pub fn other_vertex(&self, vertex_id: &str) -> String {
        if self.v0 == vertex_id {
            self.v1.clone()
        } else {
            self.v0.clone()
        }
    }
}

#[cfg(test)]
#[path = "edge_test.rs"]
mod edge_test;
