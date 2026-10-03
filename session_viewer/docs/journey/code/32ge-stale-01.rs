use crate::editor::{Action, Editor};

#[test]
fn obsolete_and_duplicate_keys_are_ignored_before_payload_decoding() {
    let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
    let url = Rc::new(crate::reload_url::ReloadUrl::new("test:stale".into()));
    editor.apply(Action::ReplaceAt(bytes.clone(), Rc::clone(&url))).unwrap(); editor.apply(Action::UnloadSources).unwrap();
    let old = editor.reload_keys().pop().unwrap();
    editor.apply(Action::Close).unwrap();
    assert!(!editor.hydrate(vec![(old.clone(), vec![255])]).unwrap());
    editor.apply(Action::ImportAt(bytes.clone(), url)).unwrap(); editor.apply(Action::UnloadSources).unwrap();
    let current = editor.reload_keys().pop().unwrap();
    assert!(!editor.hydrate(vec![(old, vec![255])]).unwrap());
    assert!(!editor.hydrate(vec![(current.clone(), bytes.clone()), (current.clone(), vec![255])]).unwrap());
    assert!(editor.scene.objects().iter().all(|row| row.geometry().is_none()));
    assert!(!editor.hydrate(Vec::new()).unwrap());
    assert!(editor.hydrate(vec![(current.clone(), bytes.clone())]).unwrap());
    assert!(!editor.hydrate(vec![(current.clone(), vec![255])]).unwrap());
    editor.apply(Action::UnloadSources).unwrap(); let next = editor.reload_keys().pop().unwrap();
    assert!(next.epoch > current.epoch);
    assert!(!editor.hydrate(vec![(current, vec![255])]).unwrap());
    assert!(editor.scene.objects().iter().all(|row| row.release_epoch() == Some(next.epoch)));
    assert!(editor.hydrate(vec![(next, bytes)]).unwrap());
}
