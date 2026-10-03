use crate::editor::{Action, Editor};

#[test]
fn a_bad_second_version_rejects_the_entire_source_batch() {
    let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
    for mode in 0..2 {
        let url = Rc::new(crate::reload_url::ReloadUrl::new(format!("test:batch-{mode}")));
        let action = if mode == 0 { Action::ReplaceAt(bytes.clone(), url) } else { Action::ImportAt(bytes.clone(), url) };
        editor.apply(action).unwrap();
    }
    editor.apply(Action::UnloadSources).unwrap(); let keys = editor.reload_keys(); assert_eq!(keys.len(), 2);
    let displays: Vec<_> = editor.scene.objects().iter().map(|row| Rc::clone(&row.mesh)).collect();
    let ids: Vec<_> = editor.scene.objects().iter().map(|row| row.id).collect();
    let roots = editor.scenes().count(); let mut changed = bytes.clone(); changed.push(0);
    let result = editor.hydrate(vec![(keys[0].clone(), bytes.clone()), (keys[1].clone(), changed)]);
    assert_eq!(result, Err("Reloaded file version changed"));
    assert_eq!(editor.scenes().count(), roots);
    for (index, row) in editor.scene.objects().iter().enumerate() {
        assert!(row.geometry().is_none()); assert_eq!(row.id, ids[index]);
        assert!(Rc::ptr_eq(&row.mesh, &displays[index]));
    }
    assert!(editor.hydrate(keys.into_iter().map(|key| (key, bytes.clone())).collect()).unwrap());
    assert!(editor.scenes().flat_map(|scene| scene.objects()).all(|row| row.origin().is_none() || row.geometry().is_some()));
}

#[test]
fn missing_source_identity_or_changed_metadata_keeps_rows_cold() {
    for change_name in [false, true] {
        let mut editor = Editor::default(); let bytes = crate::specimen::bytes();
        let url = Rc::new(crate::reload_url::ReloadUrl::new("test:metadata".into()));
        editor.apply(Action::ReplaceAt(bytes.clone(), url)).unwrap(); editor.apply(Action::UnloadSources).unwrap();
        let key = editor.reload_keys().pop().unwrap(); let display = Rc::clone(&editor.scene.objects()[0].mesh);
        let metadata = Rc::make_mut(&mut editor.scene.objects_mut()[0].metadata);
        if change_name { metadata.name.push_str(" changed"); } else { metadata.source_guid = "missing".into(); }
        assert_eq!(editor.hydrate(vec![(key, bytes)]), Err("Reloaded source GUID or metadata does not match"));
        assert!(editor.scene.objects().iter().all(|row| row.geometry().is_none()));
        assert!(Rc::ptr_eq(&editor.scene.objects()[0].mesh, &display));
    }
}
