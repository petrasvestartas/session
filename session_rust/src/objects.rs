use crate::arrow::Arrow;
use crate::boundingbox::BoundingBox;
use crate::cylinder::Cylinder;
use crate::line::Line;
use crate::mesh::Mesh;
use crate::plane::Plane;
use crate::point::Point;
use crate::pointcloud::PointCloud;
use crate::polyline::Polyline;
use serde::{ser::Serialize as SerTrait, Deserialize, Serialize};
use std::fmt;
use std::fs;
use uuid::Uuid;

/// A collection of all geometry objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Objects")]
pub struct Objects {
    pub guid: String,
    pub name: String,
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub planes: Vec<Plane>,
    pub bboxes: Vec<BoundingBox>,
    pub polylines: Vec<Polyline>,
    pub pointclouds: Vec<PointCloud>,
    pub meshes: Vec<Mesh>,
    pub cylinders: Vec<Cylinder>,
    pub arrows: Vec<Arrow>,
}

impl Default for Objects {
    fn default() -> Self {
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_objects".to_string(),
            points: Vec::new(),
            lines: Vec::new(),
            planes: Vec::new(),
            bboxes: Vec::new(),
            polylines: Vec::new(),
            pointclouds: Vec::new(),
            meshes: Vec::new(),
            cylinders: Vec::new(),
            arrows: Vec::new(),
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
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        SerTrait::serialize(self, &mut ser)?;
        Ok(String::from_utf8(buf)?)
    }

    /// Deserializes Objects from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Objects to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes Objects from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }
}

impl fmt::Display for Objects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Objects({}, {}, points={})",
            self.name,
            self.guid,
            self.points.len()
        )
    }
}

#[cfg(test)]
#[path = "objects_test.rs"]
mod objects_test;
