        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.bboxes {add!(source,crate::bounds_original::insert(scene,source.clone()));}
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.lines {add!(source,scene.insert_line(crate::stroke::PreparedLine::new(source.clone()).map_err(str::to_owned)?));}
