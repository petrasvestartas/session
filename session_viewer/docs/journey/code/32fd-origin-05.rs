use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn origin_does_not_pin_a_closed_source_document() {
    let mut editor = Editor::default(); editor.apply(Action::Replace(crate::specimen::bytes())).unwrap();
    let row = &editor.scene.objects()[0]; let source = row.source().unwrap();
    let origin = Rc::clone(&source.origin);
    let document = Rc::downgrade(&source.document); let mesh = Rc::downgrade(row.geometry().unwrap());
    assert!(editor.scene.objects().iter().all(|row| Rc::ptr_eq(&row.source().unwrap().origin, &origin)));
    editor.apply(Action::Close).unwrap();
    assert!(document.upgrade().is_none() && mesh.upgrade().is_none());
    assert_eq!(origin.header.name, "Three-piece frame"); assert!(origin.header.objects.is_none());
}

#[test]
fn duplicate_imports_and_history_keep_their_own_origins() {
    let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
    editor.apply(Action::Replace(bytes.clone())).unwrap(); editor.apply(Action::Import(bytes)).unwrap();
    let first = Rc::clone(&editor.scene.objects()[0].source().unwrap().origin);
    let second = Rc::clone(&editor.scene.objects()[3].source().unwrap().origin);
    assert_ne!(first.id, second.id); assert_eq!(first.version, second.version);
    editor.apply(Action::SelectNext).unwrap(); editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(Rc::ptr_eq(&first, &editor.scene.objects()[0].source().unwrap().origin));
    assert!(Rc::ptr_eq(&second, &editor.scene.objects()[3].source().unwrap().origin));
}
