    }
    for row in scene.objects().iter().filter(|row|ids.contains(&row.id)&&!scene.instances.contains_key(&row.id)) {
        if let Some(session_rust::Geometry::Element(source))=&row.original {push!(elements,row.id,source.to_proto());}
        else if let Some(session_rust::Geometry::Plane(source))=&row.original {push!(planes,row.id,source.to_proto());}
        else if let Some(session_rust::Geometry::OBB(source))=&row.original {push!(bboxes,row.id,source.to_proto());}
        else if let Some(cad)=row.cad.as_ref() {push!(breps,row.id,cad.source.to_proto());}
