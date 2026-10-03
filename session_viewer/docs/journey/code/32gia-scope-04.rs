use crate::{edit_intent::Intent, editor::{Action, Editor}, reload_url::ReloadUrl};
use std::rc::Rc;

fn file(editor: &mut Editor, replace: bool, url: &str) {
    let bytes = crate::specimen::bytes();
    let location = Rc::new(ReloadUrl::new(url.into()));
    editor.apply(if replace { Action::ReplaceAt(bytes, location) }
        else { Action::ImportAt(bytes, location) }).unwrap();
}

#[test]
fn target_and_save_scope_differ() {
    let mut editor = Editor::default();
    file(&mut editor, true, "test:first"); file(&mut editor, false, "test:second");
    editor.apply(Action::UnloadSources).unwrap();
    let row = &editor.scene.objects()[0]; let id = row.id;
    let origin = row.origin().unwrap().id;
    editor.selected = Some(editor.scene.objects()[3].id);
    for intent in [Intent::Move { id, offset: [0.25, 0.0, 0.0] }, Intent::Delete { id }] {
        let keys = intent.keys(&editor).unwrap();
        assert_eq!(keys.len(), 1); assert_eq!(keys[0].origin.id, origin);
        assert_eq!(keys[0].epoch, 1);
    }
    let keys = Intent::Save.keys(&editor).unwrap();
    assert_eq!(editor.scene.objects().len(), 6); assert_eq!(keys.len(), 2);
    assert_ne!(keys[0].origin.id, keys[1].origin.id);
}

#[test]
fn save_ignores_history_only_sources() {
    let mut editor = Editor::default();
    file(&mut editor, true, "test:old");
    let old = editor.scene.objects()[0].origin().unwrap().id;
    file(&mut editor, true, "test:current");
    let current = editor.scene.objects()[0].origin().unwrap().id;
    editor.apply(Action::UnloadSources).unwrap();
    let keys = Intent::Save.keys(&editor).unwrap();
    assert_eq!(keys.len(), 1); assert_eq!(keys[0].origin.id, current);
    assert_ne!(keys[0].origin.id, old);
    assert!(editor.scenes().skip(1).flat_map(|scene| scene.objects()).any(|row|
        row.origin().is_some_and(|origin| origin.id == old) && row.release_epoch().is_some()));
}

#[test]
fn loaded_or_missing_target() {
    let mut editor = Editor::default();
    let id = editor.scene.objects()[0].id;
    assert!(Intent::Delete { id }.keys(&editor).unwrap().is_empty());
    assert!(Intent::Save.keys(&editor).unwrap().is_empty());
    editor.scene.remove(id);
    assert_eq!(Intent::Delete { id }.keys(&editor).err(), Some("Object not found"));
    file(&mut editor, true, "test:loaded");
    let id = editor.scene.objects()[0].id;
    assert!(Intent::Move { id, offset: [1.0, 0.0, 0.0] }.keys(&editor).unwrap().is_empty());
}
