        add!(source,scene.insert(prepared));
    }
    for source in &loaded.document.objects.planes {add!(source,crate::plane_original::insert(scene,source.clone()));}
    for source in &loaded.document.objects.bboxes {add!(source,crate::bounds_original::insert(scene,source.clone()));}
    for source in &loaded.document.objects.points {add!(source,scene.insert_point(crate::marker::PreparedPoint::new(source.clone()).map_err(str::to_owned)?));}
