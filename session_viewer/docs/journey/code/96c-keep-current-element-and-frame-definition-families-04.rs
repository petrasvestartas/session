    for (_,owner) in scene.instances.iter().filter(|(id,_)|ids.contains(id)) {
        match &owner.definition {
            Geometry::Element(source)=>keep!(elements,source),Geometry::Plane(source)=>keep!(planes,source),Geometry::OBB(source)=>keep!(bboxes,source),Geometry::Mesh(source)=>keep!(meshes,source),Geometry::BRep(source)=>keep!(breps,source),Geometry::NurbsSurface(source)=>keep!(nurbssurfaces,source),Geometry::NurbsCurve(source)=>keep!(nurbscurves,source),Geometry::Polyline(source)=>keep!(polylines,source),Geometry::Line(source)=>keep!(lines,source),Geometry::Point(source)=>keep!(points,source),Geometry::PointCloud(source)=>keep!(pointclouds,source),_=>return Err("Unsupported saved original definition".into()),
        }
    }
    macro_rules! order {($field:ident)=>{out.$field.sort_by(|a,b|a.guid.cmp(&b.guid));};}
    order!(elements);order!(planes);order!(bboxes);order!(meshes);order!(breps);order!(nurbssurfaces);order!(nurbscurves);order!(polylines);order!(lines);order!(points);order!(pointclouds);
    Ok((!records.is_empty()).then_some(out))
}
