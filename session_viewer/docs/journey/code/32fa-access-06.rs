fn imported_source_survives_a_change_of_display_row() {
    let mut editor = Editor::default();
    editor.apply(Action::Import(specimen::bytes())).unwrap();
    let imported = editor.scene.objects()[2].clone();
    let first = editor.scene.objects()[0].id;
    let second = editor.scene.objects()[1].id;
    editor.scene.remove(first); editor.scene.remove(second);
    let current = &editor.scene.objects()[0];
    assert_eq!(current.id, imported.id);
    assert!(Rc::ptr_eq(current.geometry().unwrap(), imported.geometry().unwrap()));
    let source = current.source().unwrap();
    assert_eq!(current.geometry().unwrap().guid(), source.guid);
    assert!(source.document.objects.meshes.iter().any(|m| m.guid() == source.guid));
    assert!(Rc::ptr_eq(&source.document, &imported.source().unwrap().document));
}
