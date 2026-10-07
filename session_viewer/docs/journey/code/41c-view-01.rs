use session_rust::{Color, Mesh, Point};

#[derive(Clone, Copy)]
pub enum Mode { Sharp, Smooth, Winding }

pub fn specimen(mode: Mode) -> Mesh {
    let winding = matches!(mode, Mode::Winding);
    let xs: &[f64] = if winding { &[-0.7, 0.0, 0.7] } else { &[-0.7, 0.7] };
    let mut points: Vec<Point> = xs.iter().map(|&x| Point::new(x, -0.5, 0.0)).chain(xs.iter().map(|&x| Point::new(x, 0.5, 0.0))).collect();
    if winding { points.extend([Point::new(0.0, -0.5, 0.0), Point::new(0.0, 0.5, 0.0)]); }
    let faces = if winding { vec![vec![0, 1, 4, 3], vec![6, 7, 5, 2]] } else { vec![vec![0, 1, 3, 2]] };
    let mut mesh = Mesh::from_vertices_and_faces(points, faces); mesh.set_objectcolor(Color::new(0.5, 0.5, 0.5, 1.0));
    if matches!(mode, Mode::Smooth) {
        for key in mesh.vertices() {
            let right = key == 1 || key == 3;
            mesh.set_vertex_attribute(key, "nx", if right { 1.0 } else { 0.0 });
            mesh.set_vertex_attribute(key, "ny", 0.0);
            mesh.set_vertex_attribute(key, "nz", if right { 0.0 } else { 1.0 });
        }
    }
    mesh
}
