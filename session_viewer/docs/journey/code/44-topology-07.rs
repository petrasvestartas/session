use crate::{mesh::Mesh, mesh_parts::Parts};
use session_rust::Point;

#[test]
fn source_polygon_keys_survive_cached_triangulation_and_face_colors() {
    let mut source = session_rust::Mesh::from_vertices_and_faces(vec![Point::new(0., 0., 0.), Point::new(1., 0., 0.), Point::new(1., 1., 0.), Point::new(0., 1., 0.)], vec![vec![0, 1, 2, 3]]);
    source.set_face_triangulation(0, vec![[0, 1, 3], [1, 2, 3]]);
    for colors in [false, true] {
        if colors { source.set_facecolors(vec![session_rust::Color::new(1., 0., 0., 1.)]); }
        let display = Mesh::from_kernel(&source).unwrap(); let parts = Parts::new(&source, display.indices().len() / 3).unwrap();
        assert_eq!(parts.faces, vec![0]); assert_eq!(parts.triangles, vec![0, 0]); assert_eq!(parts.edges.len(), 4);
        assert!(!parts.edges.contains(&(1, 3)), "The rendering diagonal is not a source edge");
        assert_eq!(parts.endpoints[0], [[0., 0., 0.], [1., 0., 0.]]);
    }
    assert!(Parts::new(&source, 3).is_err());
}

