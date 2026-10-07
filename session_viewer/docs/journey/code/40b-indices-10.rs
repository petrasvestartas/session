use crate::{mesh::Mesh, prepared::PreparedMesh};
use session_rust::Point;

#[test]
fn original_high_vertex_references_survive_checked_display_preparation() {
    let mut points = vec![Point::new(0.0, 0.0, 0.0); 65_537];
    points[0] = Point::new(-0.5, -0.5, 0.0); points[65_535] = Point::new(0.5, -0.5, 0.0); points[65_536] = Point::new(0.0, 0.5, 0.0);
    let source = session_rust::Mesh::from_vertices_and_faces(points, vec![vec![0, 65_535, 65_536]]);
    let prepared = PreparedMesh::new(source).unwrap(); assert_eq!(prepared.geometry.number_of_vertices(), 65_537);
    assert_eq!(prepared.display.indices(), &[0, 65_535, 65_536]); assert_eq!(prepared.display.vertices().len(), 65_537);
    assert_eq!(prepared.display.vertices()[65_536][..3], [0.0, 0.5, 0.0]);
    assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, u32::MAX]).is_err());
}
