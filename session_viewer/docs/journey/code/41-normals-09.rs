use crate::{mesh::Mesh, prepared::PreparedMesh, normals::unit};

#[test]
fn original_vertex_normal_attributes_survive_normalized_display_preparation() {
    let mut source = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
    for key in source.vertices() {
        source.set_vertex_attribute(key, "nx", 4.0); source.set_vertex_attribute(key, "ny", 0.0); source.set_vertex_attribute(key, "nz", 3.0);
    }
    let prepared = PreparedMesh::new(source).unwrap();
    assert!(prepared.display.normals().iter().all(|n| (n[0] - 0.8).abs() < 1e-6 && (n[2] - 0.6).abs() < 1e-6));
    assert_eq!(prepared.geometry.vertex_attribute(0, "nx"), Some(4.0)); assert_eq!(prepared.geometry.vertex_attribute(0, "nz"), Some(3.0));
    let flat = PreparedMesh::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0)).unwrap(); assert!(flat.display.normals().iter().all(|n| *n == [0.0; 3]));
}

#[test]
fn normal_streams_refuse_mismatches_and_nonfinite_values_without_overflow() {
    assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).unwrap().with_normals(vec![[0.0; 3]; 2]).is_err());
    assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).unwrap().with_normals(vec![[f32::NAN; 3]; 3]).is_err());
    let huge = unit([f32::MAX, 0.0, f32::MAX]).unwrap(); assert!((huge[0] - 0.5f32.sqrt()).abs() < 1e-6);
    assert_eq!(unit([0.0; 3]).unwrap(), [0.0; 3]);
}
