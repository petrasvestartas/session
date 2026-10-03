use crate::{editor::{Action, Editor}, specimen};
use std::rc::Rc;

#[test]
fn generated_source_and_display_survive_move_and_history() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    for _ in 0..3 { editor.apply(Action::SelectNext).unwrap(); }
    let object = &editor.scene.objects()[2];
    let geometry = Rc::clone(&object.geometry);
    let display = Rc::clone(&object.mesh);
    let point = geometry.vertex_point(0).unwrap();
    let guid = geometry.guid().to_owned();
    let before = object.model.m;
    editor.apply(Action::Translate([2.0, -1.0, 0.5])).unwrap();
    let moved = editor.scene.objects()[2].model.m;
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects()[2].model.m, before);
    editor.apply(Action::Redo).unwrap();
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.model.m, moved);
    assert!(Rc::ptr_eq(&geometry, &restored.geometry));
    assert!(Rc::ptr_eq(&display, &restored.mesh));
    assert_eq!(restored.geometry.vertex_point(0).unwrap(), point);
    assert_eq!(restored.geometry.guid(), guid);
    assert!(restored.source.is_none());
}

#[test]
fn imported_source_survives_a_change_of_display_row() {
    let mut editor = Editor::default();
    editor.apply(Action::Import(specimen::bytes())).unwrap();
    let imported = editor.scene.objects()[2].clone();
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first); editor.scene.remove(second);
    let current = &editor.scene.objects()[0];
    assert_eq!(current.id, imported.id);
    assert!(Rc::ptr_eq(&current.geometry, &imported.geometry));
    let source = current.source.as_ref().unwrap();
    assert_eq!(current.geometry.guid(), source.guid);
    assert!(source.document.objects.meshes.iter().any(|m| m.guid() == source.guid));
    assert!(Rc::ptr_eq(&source.document, &imported.source.as_ref().unwrap().document));
}
