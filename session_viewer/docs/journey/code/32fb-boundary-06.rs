fn repeated_imports_keep_distinct_viewer_ids_and_original_source_guids() {
    let bytes = specimen::bytes();
    let mut editor = Editor::default();
    editor.apply(Action::Import(bytes.clone())).unwrap();
    editor.apply(Action::Import(bytes)).unwrap();
    let objects = editor.scene.objects();
    assert_ne!(objects[2].id, objects[5].id);
    assert_eq!(objects[2].source().unwrap().guid, objects[5].source().unwrap().guid);
    assert!(!Rc::ptr_eq(&objects[2].source().unwrap().document, &objects[5].source().unwrap().document));
}
