impl PreparedMesh {
    pub fn triangle(points: [[f64; 3]; 3], colour: [f32; 3]) -> Result<Self, &'static str> {
        let vertices = points.into_iter().map(|p| session_rust::Point::new(p[0], p[1], p[2])).collect();
        let mut geometry = session_rust::Mesh::from_vertices_and_faces(vertices, vec![vec![0, 1, 2]]);
        geometry.set_objectcolor(session_rust::Color::new(colour[0], colour[1], colour[2], 1.0));
        Self::new(geometry)
    }

