use crate::{document, editor::{Action, Editor}, prepared::PreparedMesh, specimen};
use session_rust::{Mesh, Point, Xform};

#[test]
fn source_precision_attributes_and_placement_reopen_exactly() {
    let mut editor = Editor::default();
    let ids: Vec<_> = editor.scene.objects().iter().map(|object| object.id).collect();
    for id in ids { editor.scene.remove(id); }
    let x = 100000000.125;
    let mut source = Mesh::from_vertices_and_faces(vec![
        Point::new(x, 0.0, 0.0), Point::new(x + 16.0, 0.0, 0.0),
        Point::new(x, 16.0, 0.0),
    ], vec![vec![0, 1, 2]]);
    source.name = "Exact source".into(); source.is_visible = false; source.is_locked = true;
    let id = editor.scene.insert(PreparedMesh::new(source).unwrap()).unwrap();
    let model = Xform::translation(1.25, -2.5, 3.75);
    editor.scene.place(id, model.clone()).unwrap();
    let bytes = document::snapshot(&editor.scene).unwrap();
    let reopened = document::load(&bytes).unwrap();
    let mesh = &reopened.meshes[0];
    assert_eq!(mesh.geometry.vertex_point(0).unwrap()[0], x);
    assert_eq!(mesh.geometry.name, "Exact source");
    assert!(!mesh.geometry.is_visible && mesh.geometry.is_locked);
    assert_eq!(mesh.geometry.guid(), editor.scene.objects()[0].guid);
    assert_eq!(mesh.model.m, model.m);
    assert_ne!(x as f32 as f64, x);
}

#[test]
fn both_imported_copies_reopen_with_their_stored_identity() {
    let mut editor = Editor::default();
    let bytes = specimen::bytes();
    editor.apply(Action::Import(bytes.clone())).unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let saved = document::snapshot(&editor.scene).unwrap();
    let reopened = document::load(&saved).unwrap();
    assert_eq!(reopened.meshes.len(), editor.scene.objects().len());
    for object in editor.scene.objects() {
        let prepared = reopened.meshes.iter().find(|mesh| mesh.geometry.guid() == object.guid).unwrap();
        assert_eq!(prepared.model.m, object.model.m);
        assert_eq!(prepared.geometry.name, object.geometry.name);
    }
    assert_eq!(editor.scene.objects()[2].geometry.guid(), editor.scene.objects()[5].geometry.guid());
    assert_ne!(editor.scene.objects()[2].guid, editor.scene.objects()[5].guid);
}
