#[test]
fn import_retains_source_and_undo_removes_the_whole_file() {
    let mut editor = Editor::default();
    editor.apply(Action::Import(specimen::bytes())).unwrap();
    assert_eq!(editor.scene.objects().len(), 5);
    let source = editor.scene.objects()[2].source.as_ref().unwrap();
    let document = Rc::clone(&source.document);
    let guid = source.guid.clone();
    let id = editor.scene.objects()[2].id;
    assert_eq!(document.name, "Three-piece frame");
    assert!(document.objects.meshes.iter().any(|mesh| mesh.guid() == guid));
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 2);
    editor.apply(Action::Redo).unwrap();
    let restored = &editor.scene.objects()[2];
    assert_eq!(restored.id, id);
    assert!(Rc::ptr_eq(&restored.source.as_ref().unwrap().document, &document));
}

#[test]
fn a_failed_import_leaves_selection_and_redo_intact() {
    let mut editor = Editor::default();
    editor.apply(Action::AddBox).unwrap();
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::SelectNext).unwrap();
    let selected = editor.selected;
    assert!(editor.apply(Action::Import(vec![255])).is_err());
    assert_eq!(editor.scene.objects().len(), 2);
    assert_eq!(editor.selected, selected);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects().len(), 3);
}

#[test]
fn repeated_imports_keep_distinct_viewer_ids_and_original_source_guids() {
    let bytes = specimen::bytes();
    let mut editor = Editor::default();
    editor.apply(Action::Import(bytes.clone())).unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let objects = editor.scene.objects();
    assert_ne!(objects[2].id, objects[5].id);
    assert_eq!(objects[2].source.as_ref().unwrap().guid, objects[5].source.as_ref().unwrap().guid);
    assert!(!Rc::ptr_eq(&objects[2].source.as_ref().unwrap().document, &objects[5].source.as_ref().unwrap().document));
}

#[test]
fn malformed_and_unsupported_records_are_refused_before_construction()