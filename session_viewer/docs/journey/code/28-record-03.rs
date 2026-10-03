use crate::prepared::PreparedMesh;
use session_rust::{Mesh, Point};
use std::rc::Rc;

#[test]
fn preparation_keeps_source_precision_attributes_and_identity() {
    let x = 100000000.125;
    let mut geometry = Mesh::from_vertices_and_faces(vec![
        Point::new(x, 0.0, 0.0), Point::new(x + 16.0, 0.0, 0.0),
        Point::new(x, 16.0, 0.0),
    ], vec![vec![0, 1, 2]]);
    geometry.name = "Precision specimen".into();
    geometry.is_visible = false;
    geometry.is_locked = true;
    let guid = geometry.guid().to_owned();
    let prepared = PreparedMesh::new(geometry).unwrap();
    assert_eq!(prepared.geometry.vertex_point(0).unwrap()[0], x);
    let rounded = x as f32 as f64;
    assert_ne!(rounded, x);
    assert!(prepared.display.vertices().iter().any(|v| v[0] as f64 == rounded));
    assert_eq!(prepared.geometry.name, "Precision specimen");
    assert!(!prepared.geometry.is_visible && prepared.geometry.is_locked);
    assert_eq!(prepared.geometry.guid(), guid);
    let shared = Rc::clone(&prepared.geometry);
    assert!(Rc::ptr_eq(&shared, &prepared.geometry));
    assert!(prepared.source.is_none());
}

#[test]
fn float_overflow_refuses_preparation() {
    let geometry = Mesh::from_vertices_and_faces(vec![
        Point::new(f64::MAX, 0.0, 0.0), Point::new(f64::MAX, 1.0, 0.0),
        Point::new(f64::MAX, 0.0, 1.0),
    ], vec![vec![0, 1, 2]]);
    assert!(PreparedMesh::new(geometry).is_err());
}
