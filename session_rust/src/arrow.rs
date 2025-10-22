use crate::{Line, Mesh, Point, Vector, Xform};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An arrow geometry defined by a line and radius, the head is uniformly scaled.
///
/// The arrow is generated as a 10-sided cylinder body and an 8-sided cone head
/// that is oriented along the line direction and scaled to match the line length and specified radius.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "Arrow")]
pub struct Arrow {
    pub line: Line,
    pub mesh: Mesh,
    pub radius: f64,
    pub guid: String,
    pub name: String,
    #[serde(default = "Xform::identity")]
    pub xform: Xform,
}

impl Arrow {
    /// Creates a new `Arrow` from a line and radius.
    ///
    /// # Arguments
    ///
    /// * `line` - The centerline of the arrow
    /// * `radius` - The radius of the arrow body
    ///
    /// # Returns
    ///
    /// A new `Arrow` with a cylinder body and cone head mesh
    pub fn new(line: Line, radius: f64) -> Self {
        let mesh = Self::create_arrow_mesh(&line, radius);
        Self {
            line,
            mesh,
            radius,
            guid: Uuid::new_v4().to_string(),
            name: "my_arrow".to_string(),
            xform: Xform::identity(),
        }
    }

    fn create_arrow_mesh(line: &Line, radius: f64) -> Mesh {
        let start = line.start();
        let line_vec = line.to_vector();
        let length = line.length();

        let z_axis = line_vec.normalize();
        let x_axis = if z_axis.z().abs() < 0.9 {
            Vector::new(0.0, 0.0, 1.0).cross(&z_axis).normalize()
        } else {
            Vector::new(1.0, 0.0, 0.0).cross(&z_axis).normalize()
        };
        let y_axis = z_axis.cross(&x_axis).normalize();

        let cone_length = length * 0.2;
        let body_length = length * 0.8;

        let body_center = Point::new(
            start.x() + line_vec.x() * 0.4,
            start.y() + line_vec.y() * 0.4,
            start.z() + line_vec.z() * 0.4,
        );

        let cone_base_center = Point::new(
            start.x() + line_vec.x() * 0.9,
            start.y() + line_vec.y() * 0.9,
            start.z() + line_vec.z() * 0.9,
        );

        let body_scale = Xform::scale_xyz(radius * 2.0, radius * 2.0, body_length);
        let rotation = Xform::from_cols(x_axis, y_axis, z_axis);
        let body_translation =
            Xform::translation(body_center.x(), body_center.y(), body_center.z());
        let body_xform = &body_translation * &(&rotation * &body_scale);

        let cone_scale = Xform::scale_xyz(radius * 3.0, radius * 3.0, cone_length);
        let cone_translation = Xform::translation(
            cone_base_center.x(),
            cone_base_center.y(),
            cone_base_center.z(),
        );
        let cone_xform = &cone_translation * &(&rotation * &cone_scale);

        let body_geometry = Self::unit_cylinder_geometry();
        let cone_geometry = Self::unit_cone_geometry();

        let mut mesh = Mesh::new();

        let mut body_vertex_map = Vec::new();
        for v in &body_geometry.0 {
            let transformed = body_xform.transformed_point(v);
            let key = mesh.add_vertex(transformed, None);
            body_vertex_map.push(key);
        }

        for tri in &body_geometry.1 {
            let face_vertices = vec![
                body_vertex_map[tri[0]],
                body_vertex_map[tri[1]],
                body_vertex_map[tri[2]],
            ];
            mesh.add_face(face_vertices, None);
        }

        let mut cone_vertex_map = Vec::new();
        for v in &cone_geometry.0 {
            let transformed = cone_xform.transformed_point(v);
            let key = mesh.add_vertex(transformed, None);
            cone_vertex_map.push(key);
        }

        for tri in &cone_geometry.1 {
            let face_vertices = vec![
                cone_vertex_map[tri[0]],
                cone_vertex_map[tri[1]],
                cone_vertex_map[tri[2]],
            ];
            mesh.add_face(face_vertices, None);
        }

        mesh
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

    fn unit_cone_geometry() -> (Vec<Point>, Vec<[usize; 3]>) {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.5),
            Point::new(0.5, 0.0, -0.5),
            Point::new(0.353553, -0.353553, -0.5),
            Point::new(0.0, -0.5, -0.5),
            Point::new(-0.353553, -0.353553, -0.5),
            Point::new(-0.5, 0.0, -0.5),
            Point::new(-0.353553, 0.353553, -0.5),
            Point::new(0.0, 0.5, -0.5),
            Point::new(0.353553, 0.353553, -0.5),
        ];

        let triangles = vec![
            [0, 2, 1],
            [0, 3, 2],
            [0, 4, 3],
            [0, 5, 4],
            [0, 6, 5],
            [0, 7, 6],
            [0, 8, 7],
            [0, 1, 8],
        ];

        (vertices, triangles)
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

    /// Serializes the Arrow to a JSON string.
    pub fn jsondump(&self) -> Result<String, Box<dyn std::error::Error>> {
        let data = serde_json::json!({
            "type": "Arrow",
            "guid": self.guid,
            "name": self.name,
            "radius": self.radius,
            "line": self.line,
            "mesh": self.mesh.jsondump()
        });
        Ok(serde_json::to_string_pretty(&data)?)
    }

    /// Deserializes an Arrow from a JSON string.
    pub fn jsonload(json_data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json_data)?)
    }

    /// Serializes the Arrow to a JSON file.
    pub fn to_json(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = self.jsondump()?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Deserializes an Arrow from a JSON file.
    pub fn from_json(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(filepath)?;
        Self::jsonload(&json)
    }
}

#[cfg(test)]
#[path = "arrow_test.rs"]
mod arrow_test;
