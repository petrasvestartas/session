use crate::{editor::{Action, Editor}, specimen};
use std::rc::Rc;

#[test]
fn import_shares_the_retained_sessions_source_allocation() {
    let mut editor = Editor::default();
    editor.apply(Action::Import(specimen::bytes())).unwrap();
    let imported = editor.scene.objects()[2].clone();
    let geometry = Rc::clone(imported.geometry.as_ref().unwrap());
    let source = imported.source.as_ref().unwrap();
    assert!(source.document.objects.meshes.iter().any(|m| Rc::ptr_eq(m, &geometry)));
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first); editor.scene.remove(second);
    let moved_row = &editor.scene.objects()[0];
    assert_eq!(moved_row.id, imported.id);
    assert!(Rc::ptr_eq(moved_row.geometry.as_ref().unwrap(), &geometry));
}
