    for (id,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        let mut source=owner.instance.to_proto();source.guid=scene.row_guid(*id).ok_or("Original instance owner is missing")?.into();source.flags=(source.flags&!(session_rust::InstanceRef::FLAG_HIDDEN|session_rust::InstanceRef::FLAG_LOCKED))|if scene.visible(*id) {0} else {session_rust::InstanceRef::FLAG_HIDDEN}|if scene.view.locked.contains(id) {session_rust::InstanceRef::FLAG_LOCKED} else {0};out.instances.push(source);
    }
    for row in scene.matrices().iter().filter(|row|ids.contains(&row.id)) {
        let mut source=row.source.to_proto();source.guid=row.guid.clone();
        if !guids.insert(source.guid.clone()){return Err("Matrix GUID occurs twice within one saved document".into());}out.components.push(source);
    }
    out.instances.sort_by(|a,b|a.guid.cmp(&b.guid));Ok(out)
}

