// Make macros available throughout the crate
#[macro_use]
mod macros;

// Individual module declarations for flattened structure
pub mod json_serialization;
pub mod point;
pub mod vector;
pub mod line;
pub mod pointcloud;
pub mod linecloud;
pub mod mesh;
pub mod arrow;
pub mod plane;
pub mod color;
pub mod pline;
pub mod xform;
pub mod quaternion;
pub mod pipe;

// Re-exports for convenience
pub use point::Point;
pub use vector::Vector;
pub use line::Line;
pub use pointcloud::PointCloud;
pub use linecloud::LineCloud;
pub use mesh::Mesh;
pub use arrow::Arrow;
pub use plane::Plane;
pub use color::Color;
pub use pline::Pline;
pub use xform::Xform;
pub use json_serialization::{FromJsonData, HasJsonData};
use serde::{Serialize, Deserialize};
use serde_json::Value;

// MeshInstances: 
#[derive(Serialize, Deserialize, Debug)]
pub struct MeshInstances {
    pub mesh_index: usize,              // or mesh GUID
    pub transforms: Vec<Xform>,
}

// Comprehensive geometry data structure with all geometry types
#[derive(Serialize, Deserialize, Debug)]
pub struct AllGeometryData {
    pub points: Vec<Point>,
    pub vectors: Vec<Vector>,
    pub lines: Vec<Line>,
    pub arrows: Vec<Arrow>,
    pub planes: Vec<Plane>,
    pub colors: Vec<Color>,
    pub point_clouds: Vec<PointCloud>,
    pub line_clouds: Vec<LineCloud>,
    pub plines: Vec<Pline>,
    pub xforms: Vec<Xform>,
    pub meshes: Vec<Mesh>,
    #[serde(default)]
    pub mesh_instances: Vec<MeshInstances>,
    pub pipe_mesh_index: Option<usize>,
    pub sphere_mesh_index: Option<usize>,
}

impl HasJsonData for AllGeometryData {
    fn to_json_data(&self, _minimal: bool) -> Value {
        serde_json::json!({
            "dtype": "AllGeometryData",
            "points": self.points.iter().map(|p| p.to_json_data(false)).collect::<Vec<_>>(),
            "vectors": self.vectors.iter().map(|v| v.to_json_data(false)).collect::<Vec<_>>(),
            "lines": self.lines.iter().map(|l| l.to_json_data(false)).collect::<Vec<_>>(),
            "arrows": self.arrows.iter().map(|a| a.to_json_data(false)).collect::<Vec<_>>(),
            "planes": self.planes.iter().map(|p| p.to_json_data(false)).collect::<Vec<_>>(),
            "colors": self.colors.iter().map(|c| c.to_json_data(false)).collect::<Vec<_>>(),
            "point_clouds": self.point_clouds.iter().map(|pc| pc.to_json_data(false)).collect::<Vec<_>>(),
            "line_clouds": self.line_clouds.iter().map(|lc| lc.to_json_data(false)).collect::<Vec<_>>(),
            "plines": self.plines.iter().map(|pl| pl.to_json_data(false)).collect::<Vec<_>>(),
            "xforms": self.xforms.iter().map(|x| serde_json::json!({"dtype": "Xform", "m": x.m})).collect::<Vec<_>>(),
            "meshes": self.meshes.iter().map(|m| m.to_json_data(false)).collect::<Vec<_>>(),
            "mesh_instances": self.mesh_instances,
            "pipe_mesh_index": self.pipe_mesh_index,
            "sphere_mesh_index": self.sphere_mesh_index
        })
    }
}

// Implement FromJsonData for AllGeometryData to work with json_load
impl FromJsonData for AllGeometryData {
    fn from_json_data(data: &serde_json::Value) -> Option<Self> {
        // Try COMPAS-style deserialization first
        if let Some(obj) = data.as_object() {
            let points = obj.get("points")?.as_array()?
                .iter().filter_map(|v| Point::from_json_data(v)).collect();
            let vectors = obj.get("vectors")?.as_array()?
                .iter().filter_map(|v| Vector::from_json_data(v)).collect();
            let lines = obj.get("lines")?.as_array()?
                .iter().filter_map(|v| Line::from_json_data(v)).collect();
            let arrows = obj.get("arrows")?.as_array()?
                .iter().filter_map(|v| Arrow::from_json_data(v)).collect();
            let planes = obj.get("planes")?.as_array()?
                .iter().filter_map(|v| Plane::from_json_data(v)).collect();
            let colors = obj.get("colors")?.as_array()?
                .iter().filter_map(|v| Color::from_json_data(v)).collect();
            let point_clouds = obj.get("point_clouds")?.as_array()?
                .iter().filter_map(|v| PointCloud::from_json_data(v)).collect();
            let line_clouds = obj.get("line_clouds")?.as_array()?
                .iter().filter_map(|v| LineCloud::from_json_data(v)).collect();
            let plines = obj.get("plines")?.as_array()?
                .iter().filter_map(|v| Pline::from_json_data(v)).collect();
            let xforms = obj.get("xforms")?.as_array()?
                .iter().filter_map(|v| {
                    if let Some(m_array) = v.get("m")?.as_array() {
                        let m: Vec<f32> = m_array.iter().filter_map(|x| x.as_f64().map(|f| f as f32)).collect();
                        if m.len() == 16 {
                            Some(Xform { m: [m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15]] })
                        } else { None }
                    } else { None }
                }).collect();
            let meshes = obj.get("meshes").unwrap_or(&serde_json::Value::Array(vec![])).as_array()
                .map(|arr| arr.iter().filter_map(|v| Mesh::from_json_data(v)).collect())
                .unwrap_or_else(Vec::new);
            
            Some(AllGeometryData {
                points,
                vectors,
                lines,
                arrows,
                planes,
                colors,
                point_clouds,
                line_clouds,
                plines,
                xforms,
                meshes,
                mesh_instances: obj.get("mesh_instances").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default(),
                pipe_mesh_index: obj.get("pipe_mesh_index").and_then(|v| v.as_u64().map(|u| u as usize)),
                sphere_mesh_index: obj.get("sphere_mesh_index").and_then(|v| v.as_u64().map(|u| u as usize)),
            })
        } else {
            // Fallback to serde deserialization for old format
            serde_json::from_value(data.clone()).ok()
        }
    }
}

