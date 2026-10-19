    if let Some(doc)=namespace.doc.as_ref(){source.graph=crate::graph_edit::saved(scene,doc,&names);}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    if let Some(metadata)=source.objects.as_ref(){objects.components=metadata.components.clone();}
    source.definitions=crate::definition_merge::merge(source.definitions.as_ref(),crate::instance_snapshot::definitions(scene,&namespace.ids)?)?;source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
