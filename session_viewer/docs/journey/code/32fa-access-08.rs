fn snapshot_uses_live_local_sources_and_separate_placements() {
    let mut editor = Editor::default();
    editor.apply(Action::SelectNext).unwrap();
    editor.apply(Action::Delete).unwrap();
    editor.apply(Action::AddBox).unwrap();
    let object = editor.scene.objects().last().unwrap().clone();
    let source = object.geometry().unwrap().to_proto();
    let message = proto::Session::decode(document::snapshot(&editor.scene).unwrap().as_slice()).unwrap();
    let meshes = &message.objects.as_ref().unwrap().meshes;
    assert_eq!(meshes.len(), 2);
    let saved = meshes.iter().find(|mesh| mesh.guid == object.guid).unwrap();
    assert_eq!(saved.vertices, source.vertices);
    assert_eq!(saved.faces, source.faces);
    assert_eq!(saved.objectcolor, source.objectcolor);
    assert_eq!((saved.is_visible, saved.is_locked), (source.is_visible, source.is_locked));
    let placement = message.xforms.iter().find(|entry| entry.guid == object.guid).unwrap();
    assert_eq!(placement.xform.as_ref().unwrap().matrix, object.model.m.to_vec());
    assert_eq!(message.tree.unwrap().root.unwrap().children.len(), meshes.len());
    assert_eq!(object.geometry().unwrap().guid(), source.guid);
    editor.apply(Action::Undo).unwrap();
    assert_eq!(editor.scene.objects().len(), 1, "Saving must not create a history entry");
}
