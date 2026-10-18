        }
    }
    for row in scene.points().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {push!(points,row.id,row.prepared.source.to_proto());}
    for row in scene.clouds().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {push!(pointclouds,row.id,row.prepared.source.to_proto());}
    for (id,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        let mut source=owner.instance.to_proto();source.guid=scene.row_guid(*id).ok_or("Original instance owner is missing")?.into();source.flags=(source.flags&!(session_rust::InstanceRef::FLAG_HIDDEN|session_rust::InstanceRef::FLAG_LOCKED))|if scene.visible(*id) {0} else {session_rust::InstanceRef::FLAG_HIDDEN}|if scene.view.locked.contains(id) {session_rust::InstanceRef::FLAG_LOCKED} else {0};out.instances.push(source);
    }
    out.instances.sort_by(|a,b|a.guid.cmp(&b.guid));Ok(out)
}

