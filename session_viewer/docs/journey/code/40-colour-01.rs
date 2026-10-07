use session_rust::{Color, Mesh, Point};

#[derive(Clone, Copy)]
pub enum Mode { Object, Points, Faces }

pub fn specimen(mode: Mode) -> Mesh {
    let points = [-0.7, 0.0, 0.7].into_iter().map(|x| Point::new(x, -0.5, 0.0))
        .chain([-0.7, 0.0, 0.7].into_iter().map(|x| Point::new(x, 0.5, 0.0))).collect();
    let mut mesh = Mesh::from_vertices_and_faces(points, vec![vec![0, 1, 4, 3], vec![1, 2, 5, 4]]);
    mesh.set_objectcolor(Color::new(0.8, 0.6, 0.1, 1.0));
    let red = Color::new(1.0, 0.0, 0.0, 1.0); let green = Color::new(0.0, 1.0, 0.0, 1.0); let blue = Color::new(0.0, 0.0, 1.0, 1.0);
    match mode {
        Mode::Object => {},
        Mode::Points => mesh.set_pointcolors(vec![red.clone(), green.clone(), blue.clone(), red, green, blue]),
        Mode::Faces => mesh.set_facecolors(vec![red, blue]),
    }
    mesh
}
