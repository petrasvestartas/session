use crate::{editor::{Action, Editor}, scene::ObjectId};
use std::rc::Rc;

fn imported() -> Editor {
    let mut editor = Editor::default();
    editor.scene = crate::scene::Scene::empty();
    crate::scene_document::import(&mut editor.scene, &crate::interaction_example::bytes(),
        "fixture", &session_rust::Xform::identity(), 0.).unwrap();
    editor
}
fn owner(editor: &Editor, name: &str) -> ObjectId {
    editor.scene.objects().iter().find(|row| row.metadata.name == name).unwrap().id
}
fn piece_key(editor: &Editor, parent: ObjectId) -> String {
    crate::hierarchy::rows(&editor.scene).into_iter().find(|row|
        row.label.starts_with("piece ")
        && crate::feature_owner::row_owner(&editor.scene, &row.key) == Some(parent)).unwrap().key
}

#[test]
fn feature_rows_target_the_nearest_original_and_respect_owner_visibility_and_locks() {
    let mut e = imported();
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    let host = owner(&e, "host"); let child = owner(&e, "child");
    let host_key = piece_key(&e, host); let child_key = piece_key(&e, child);
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &host_key), Some(host));
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &child_key), Some(child));
    for row in crate::hierarchy::rows(&e.scene).into_iter().filter(|row| row.readonly) {
        assert!(crate::feature_owner::row_owner(&e.scene, &row.key).is_some());
    }
    Rc::make_mut(&mut e.scene.view).hide(&[host], true);
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &host_key), None);
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &child_key), Some(child));
    Rc::make_mut(&mut e.scene.view).hide(&[host], false);
    Rc::make_mut(&mut e.scene.view).lock(&[host], true);
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &host_key), None);
    assert_eq!(crate::feature_owner::row_owner(&e.scene, "missing/key"), None);
}

#[test]
fn display_highlighting_stops_at_independent_children_and_never_widens_editing_selection() {
    let mut e = imported();
    e.apply(Action::ElementAttributes(Some(true))).unwrap();
    let host = owner(&e, "host"); let child = owner(&e, "child");
    e.apply(Action::TreePick {ids: vec![host], additive: false}).unwrap();
    assert_eq!(e.selected_objects(), [host]);
    let highlight = e.highlighted_objects();
    assert_eq!(highlight.len(), 6);
    assert!(!highlight.contains(&child));
    let child_piece = crate::hierarchy::rows(&e.scene).into_iter().find(|row|
        row.label.starts_with("piece ")
        && crate::feature_owner::row_owner(&e.scene, &row.key) == Some(child)).unwrap().ids[0];
    assert!(!highlight.contains(&child_piece));
    e.apply(Action::ElementInteractions(Some(false))).unwrap();
    assert_eq!(e.highlighted_objects().len(), 2);
    assert_eq!(e.selected_objects(), [host]);
    assert!(e.gizmo_origin().is_some());
}

#[test]
fn feature_owner_delete_and_undo_restore_original_and_generated_paths_once() {
    let mut e = imported();
    let host = owner(&e, "host"); let child = owner(&e, "child");
    let source = match e.scene.objects().iter().find(|row| row.id == host).unwrap().original.as_ref().unwrap() {
        session_rust::Geometry::Element(source) => source.clone(), _ => panic!("Element expected"),
    };
    let key = piece_key(&e, host);
    let target = crate::feature_owner::row_owner(&e.scene, &key).unwrap();
    e.apply(Action::TreePick {ids: vec![target], additive: false}).unwrap();
    assert_eq!(e.selected_objects(), [host]);
    e.apply(Action::Delete).unwrap();
    assert!(!e.scene.contains(host)); assert!(!e.scene.contains(child));
    assert_eq!(e.scene.paths().len(), 0);
    e.apply(Action::Undo).unwrap();
    assert!(e.scene.contains(host)); assert!(e.scene.contains(child));
    assert_eq!(e.scene.paths().len(), 6);
    let restored = match e.scene.objects().iter().find(|row| row.id == host).unwrap().original.as_ref().unwrap() {
        session_rust::Geometry::Element(source) => source, _ => panic!("Element expected"),
    };
    assert!(Rc::ptr_eq(restored, &source));
    assert_eq!(crate::feature_owner::row_owner(&e.scene, &key), Some(host));
}
