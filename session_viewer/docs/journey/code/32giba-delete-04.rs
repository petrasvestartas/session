use crate::{edit_intent::Intent, editor::{Action, Change, Editor}, reload_url::ReloadUrl};
use std::rc::Rc;

#[test]
fn captured_delete_keeps_later_selection_and_one_undo() {
    let mut e = Editor::default(); let id = e.scene.objects()[0].id;
    let intent = Intent::capture(&Action::Delete, Some(id)).unwrap().unwrap();
    let later = e.scene.objects()[1].id; e.selected = Some(later);
    e.apply(Action::Orbit(0.2, 0.1)).unwrap(); let camera = e.camera.uniform();
    let Intent::Delete { id } = intent else { panic!("Delete intent") };
    assert_eq!(e.delete_object(id).unwrap(), Change::Scene);
    assert!(!e.scene.contains(id)); assert_eq!(e.selected, Some(later));
    assert_eq!(e.camera.uniform(), camera);
    e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects().len(), 2);
    e.apply(Action::Undo).unwrap(); assert_eq!(e.scene.objects().len(), 2);
    e.apply(Action::Redo).unwrap(); assert!(!e.scene.contains(id));
    assert_eq!(e.selected, Some(later));
}

#[test]
fn refused_delete_keeps_redo_and_selection() {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(crate::specimen::bytes(), Rc::new(ReloadUrl::new("test:delete".into())))).unwrap();
    let cold = e.scene.objects()[0].id; e.selected = Some(cold);
    e.apply(Action::UnloadSources).unwrap();
    e.apply(Action::AddBox).unwrap(); let added = e.scene.objects().last().unwrap().id;
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.delete_object(cold), Err("Reload editable sources before Delete"));
    assert_eq!(e.selected, Some(cold));
    assert_eq!(e.delete_object(added), Err("Object not found"));
    e.selected = None; assert_eq!(e.apply(Action::Delete).unwrap(), Change::View);
    e.apply(Action::Redo).unwrap(); assert!(e.scene.contains(added));
    e.selected = Some(added); e.delete_object(added).unwrap(); assert_eq!(e.selected, None);
    e.apply(Action::Undo).unwrap(); assert!(e.scene.contains(added));
}
