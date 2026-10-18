    let placement = message.xforms.iter().find(|entry| entry.guid == object.guid).unwrap();
    assert_eq!(placement.xform.as_ref().unwrap().matrix, object.model.m.to_vec());
    let root=message.tree.as_ref().unwrap().root.as_ref().unwrap();assert_eq!(root.children.len(),1);let created=&root.children[0];assert_eq!(created.name,"Created");
    assert_eq!(created.children.len(),meshes.len());assert!(created.children.iter().all(|node|meshes.iter().any(|mesh|mesh.guid==node.name)));
    assert_eq!(object.geometry().unwrap().guid(), source.guid);
    editor.apply(Action::Undo).unwrap();
