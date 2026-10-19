        }};
    }
    for row in scene.objects().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {
        if let Some(cad)=row.cad.as_ref() {push!(breps,row.id,cad.source.to_proto());}
        else if let Some(surface)=row.surface.as_ref() {push!(nurbssurfaces,row.id,surface.source.to_proto());}
        else {push!(meshes,row.id,row.geometry().ok_or("Reload editable sources before saving")?.to_proto());}
    }
    for row in scene.lines().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {push!(lines,row.id,row.prepared.source.to_proto());}
    for row in scene.paths().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {
        match &row.prepared.source {
            crate::chain::Source::Line(source)=>push!(lines,row.id,source.to_proto()),
