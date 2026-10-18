    if !scene.points().is_empty() { return Err("This checkpoint cannot save point objects yet"); }
    if !scene.lines().is_empty() || !scene.paths().is_empty() { return Err("This checkpoint cannot save line objects yet"); }
    let mut objects = proto::Objects::default();
    let mut xforms = Vec::new();
    let (mut children,placed)=scene.forest.snapshot(scene,&mut xforms)?;
    for object in scene.objects() {
        let geometry = object.geometry().ok_or("Reload editable sources before saving")?;
        let mut mesh = geometry.to_proto();
        mesh.guid = object.guid.clone();
        objects.meshes.push(mesh);
        if !placed.contains(&object.id) {children.push(proto::TreeNode { name: object.guid.clone(), ..Default::default() });}
        if !placed.contains(&object.id) && object.model.m != session_rust::Xform::identity().m {
            xforms.push(proto::XformEntry {
                guid: object.guid.clone(), xform: Some(object.model.to_proto()),
            });
