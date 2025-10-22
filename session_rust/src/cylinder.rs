use crate::{Line, Mesh, Point, Vector, Xform};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A cylinder geometry defined by a line and radius.
///
/// The cylinder is generated as a 10-sided cylinder mesh that is oriented
/// along the line direction and scaled to match the line length and specified radius.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Cylinder")]
pub struct Cylinder {
    pub guid: String,
    pub name: String,
    pub radius: f64,
    pub line: Line,
    pub mesh: Mesh,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl Cylinder {
    /// Creates a new `Cylinder` from a line and radius.
    ///
    /// # Arguments
    ///
    /// * `line` - The centerline of the cylinder
    /// * `radius` - The radius of the cylinder
    ///
    /// # Returns
    ///
    /// A new `Cylinder` with a generated 10-sided cylinder mesh
    pub fn new(line: Line, radius: f64) -> Self {
        let mesh = Self::create_cylinder_mesh(&line, radius);
        Self {
            guid: Uuid::new_v4().to_string(),
            name: "my_cylinder".to_string(),
            radius,
            line,
            mesh,
            xform: Xform::identity(),
        }
    }

    fn create_cylinder_mesh(line: &Line, radius: f64) -> Mesh {
        let unit_cylinder = Self::unit_cylinder_geometry();
        let xform = Self::line_to_cylinder_transform(line, radius);
        Self::transform_geometry(&unit_cylinder, &xform)
    }

    fn unit_cylinder_geometry() -> (Vec<Point>, Vec<[usize; 3]>) {
        let vertices = vec![
            Point::new(0.5, 0.0, -0.5),
            Point::new(0.404508, 0.293893, -0.5),
            Point::new(0.154508, 0.475528, -0.5),
            Point::new(-0.154508, 0.475528, -0.5),
            Point::new(-0.404508, 0.293893, -0.5),
            Point::new(-0.5, 0.0, -0.5),
            Point::new(-0.404508, -0.293893, -0.5),
            Point::new(-0.154508, -0.475528, -0.5),
            Point::new(0.154508, -0.475528, -0.5),
            Point::new(0.404508, -0.293893, -0.5),
            Point::new(0.5, 0.0, 0.5),
            Point::new(0.404508, 0.293893, 0.5),
            Point::new(0.154508, 0.475528, 0.5),
            Point::new(-0.154508, 0.475528, 0.5),
            Point::new(-0.404508, 0.293893, 0.5),
            Point::new(-0.5, 0.0, 0.5),
            Point::new(-0.404508, -0.293893, 0.5),
            Point::new(-0.154508, -0.475528, 0.5),
            Point::new(0.154508, -0.475528, 0.5),
            Point::new(0.404508, -0.293893, 0.5),
        ];

        let triangles = vec![
            [0, 1, 11],
            [0, 11, 10],
            [1, 2, 12],
            [1, 12, 11],
            [2, 3, 13],
            [2, 13, 12],
            [3, 4, 14],
            [3, 14, 13],
            [4, 5, 15],
            [4, 15, 14],
            [5, 6, 16],
            [5, 16, 15],
            [6, 7, 17],
            [6, 17, 16],
            [7, 8, 18],
            [7, 18, 17],
            [8, 9, 19],
            [8, 19, 18],
            [9, 0, 10],
            [9, 10, 19],
        ];

        (vertices, triangles)
    }

    fn line_to_cylinder_transform(line: &Line, radius: f64) -> Xform {
        let start = line.start();
        let end = line.end();
        let line_vec = line.to_vector();
        let length = line.length();

        let z_axis = line_vec.normalize();
        let x_axis = if z_axis.z().abs() < 0.9 {
            Vector::new(0.0, 0.0, 1.0).cross(&z_axis).normalize()
        } else {
            Vector::new(1.0, 0.0, 0.0).cross(&z_axis).normalize()
        };
        let y_axis = z_axis.cross(&x_axis).normalize();

        let scale = Xform::scale_xyz(radius * 2.0, radius * 2.0, length);
        let rotation = Xform::from_cols(x_axis, y_axis, z_axis);
        let center = Point::new(
            (start.x() + end.x()) * 0.5,
            (start.y() + end.y()) * 0.5,
            (start.z() + end.z()) * 0.5,
        );
        let translation = Xform::translation(center.x(), center.y(), center.z());

        &translation * &(&rotation * &scale)
    }

    fn transform_geometry(geometry: &(Vec<Point>, Vec<[usize; 3]>), xform: &Xform) -> Mesh {
        let (vertices, triangles) = geometry;
        let mut mesh = Mesh::new();

        let vertex_keys: Vec<usize> = vertices
            .iter()
            .map(|v| {
                let transformed = xform.transformed_point(v);
                mesh.add_vertex(transformed, None)
            })
            .collect();

        for tri in triangles {
            let face_vertices = vec![
                vertex_keys[tri[0]],
                vertex_keys[tri[1]],
                vertex_keys[tri[2]],
            ];
            mesh.add_face(face_vertices, None);
        }

        mesh
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Transformation
    ///////////////////////////////////////////////////////////////////////////////////////////

    pub fn transform(&mut self) {
        self.line.transform();
        self.xform = Xform::identity();
    }

    pub fn transformed(&self) -> Self {
        let mut result = self.clone();
        result.transform();
        result
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // JSON
    ///////////////////////////////////////////////////////////////////////////////////////////

    /// Serializes the Cylinder to a JSON string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let data = serde_json::json!({
            "type": "Cylinder",
            "guid": self.guid,
            "name": self.name,
            "radius": self.radius,
            "line": self.line,
            "mesh": self.mesh.jsondump()
        });
        Ok(serde_json::to_string_pretty(&data)?)
    }

    /// Deserializes a Cylinder from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Cylinder to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes a Cylinder from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }
}

#[cfg(test)]
#[path = "cylinder_test.rs"]
mod cylinder_test;
