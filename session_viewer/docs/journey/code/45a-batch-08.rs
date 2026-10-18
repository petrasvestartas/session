    let bytes = message.encode_to_vec();
    let mut editor = Editor::default();
    let url = Rc::new(crate::reload_url::ReloadUrl::new("test:restore".into()));
    editor.apply(Action::ReplaceAt(bytes.clone(), url)).unwrap();
    let id = editor.scene.objects()[0].id;
    editor.move_object(id, [0.25, 0.0, 0.0]).unwrap();
    let row = &editor.scene.objects()[0]; let id = row.id; let guid = row.guid.clone(); let model = row.model.m;
    let display = Rc::clone(&row.mesh); let metadata = Rc::clone(&row.metadata);
    let old = Rc::downgrade(row.geometry().unwrap()); let roots = editor.scenes().count();
