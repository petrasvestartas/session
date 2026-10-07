use crate::{prepared::PreparedMesh, scene::Scene};

#[test]
fn invalid_initial_normal_placement_does_not_consume_an_object_identity() {
    let mut scene = Scene::demo(); scene.clear(); let baseline = scene.clone();
    let mut prepared = PreparedMesh::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0)).unwrap(); prepared.model.m[0] = f64::NAN;
    assert!(scene.insert(prepared).is_err()); assert!(scene.objects().is_empty());
    let source = session_rust::Mesh::create_box(1.0, 1.0, 1.0); let actual = scene.insert(PreparedMesh::new(source.clone()).unwrap()).unwrap();
    let mut baseline = baseline; let expected = baseline.insert(PreparedMesh::new(source).unwrap()).unwrap(); assert_eq!(actual, expected);
}
