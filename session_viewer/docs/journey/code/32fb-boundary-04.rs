fn duplicate_imports_keep_distinct_stored_identity_through_history() {
    let mut editor = Editor::default();
    let bytes = specimen::bytes();
    editor.apply(Action::Import(bytes.clone())).unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let objects = editor.scene.objects();
    assert_eq!(objects.len(), 8);
    let distinct: HashSet<_> = objects.iter().map(|object| object.guid.as_str()).collect();
    assert_eq!(distinct.len(), objects.len());
    assert_eq!(objects[2].guid, objects[2].geometry().unwrap().guid());
    assert_eq!(objects[2].geometry().unwrap().guid(), objects[5].geometry().unwrap().guid());
    assert_eq!(objects[2].source().unwrap().guid, objects[5].source().unwrap().guid);
    assert_ne!(objects[2].guid, objects[5].guid);
    assert!(uuid::Uuid::parse_str(&objects[5].guid).is_ok());
    let copy = objects[5].clone();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    let current = editor.scene.objects().iter().find(|object| object.id == copy.id).unwrap();
    assert_eq!(current.guid, copy.guid);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::Redo).unwrap();
    let current = editor.scene.objects().iter().find(|object| object.id == copy.id).unwrap();
    assert_eq!(current.guid, copy.guid);
    assert_eq!(current.geometry().unwrap().guid(), copy.geometry().unwrap().guid());
}
