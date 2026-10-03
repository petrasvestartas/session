use crate::editor::{Action, Editor};
use std::rc::Rc;

#[test]
fn unloading_drops_imported_kernel_owners_but_keeps_display_and_history() {
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new("test:unload".into()));
    editor.apply(Action::ReplaceAt(crate::specimen::bytes(), url)).unwrap();
    editor.apply(Action::SelectNext).unwrap(); let row = &editor.scene.objects()[0];
    let id = row.id; let guid = row.guid.clone(); let display = Rc::clone(&row.mesh);
    let source = Rc::downgrade(row.geometry().unwrap()); let document = Rc::downgrade(&row.source().unwrap().document);
    let initial = row.model.m; let view = editor.camera.uniform();
    editor.apply(Action::Translate([0.25, 0.0, 0.0])).unwrap();
    let moved = editor.scene.objects()[0].model.m; let roots = editor.scenes().count();
    editor.apply(Action::UnloadSources).unwrap();
    assert!(source.upgrade().is_none() && document.upgrade().is_none());
    let row = &editor.scene.objects()[0]; assert_eq!(row.id, id); assert_eq!(row.guid, guid);
    assert_eq!(row.model.m, moved); assert!(Rc::ptr_eq(&display, &row.mesh));
    assert_eq!(row.release_epoch(), Some(1)); assert!(row.geometry().is_none() && row.source().is_none());
    assert_eq!(editor.scenes().count(), roots); assert_eq!(editor.camera.uniform(), view);
    assert!(crate::document::snapshot(&editor.scene).is_err());
    assert!(editor.apply(Action::Translate([1.0, 0.0, 0.0])).is_err());
    assert_eq!(editor.scenes().count(), roots);
    editor.apply(Action::Undo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, initial);
    assert_eq!(editor.scene.objects()[0].release_epoch(), Some(1));
    editor.apply(Action::Redo).unwrap(); assert_eq!(editor.scene.objects()[0].model.m, moved);
    assert!(source.upgrade().is_none());
}
