pub fn adopt(scene: &mut Scene, made: Vec<Made>, consumed: &[ObjectId]) -> Result<Vec<ObjectId>, &'static str> {
    let mut ids = Vec::new();
    let session = session_rust::Session::new("Created");
    for item in made {
        let id = crate::element_original::insert(scene, item.source.clone())?;
        ids.push(id);
        if let Some(plane) = item.plane {
            let plane = crate::plane_original::insert(scene, Rc::new(plane))?;
            Rc::make_mut(&mut scene.view).hide(&[plane], true);
            crate::layer_edit::attach_attributes(scene, id, vec![plane], "attributes")?;
        }
        crate::feature_display::append(scene, id, &session, &item.source)?;
    }
    for id in scene.with_descendants(consumed) { scene.remove(id); }
    Ok(ids)
}

fn group_ids(scene: &Scene, name: &str) -> Vec<ObjectId> {
    let mut ids = Vec::new();
    for doc in &scene.forest.documents {
        for (index, group) in doc.nodes.iter().enumerate() {
            if group.source.name != name || !group.readonly { continue; }
            let mut child = index + 1;
            while child < group.end {
                let node = &doc.nodes[child];
                if node.object.is_some() && !node.readonly {
                    child = node.end;
                    continue;
                }
                if let Some(id) = node.object.filter(|id| scene.contains(*id)) { ids.push(id); }
                child += 1;
            }
        }
    }
    ids.sort_unstable_by_key(|id| id.value());
    ids.dedup();
    ids
}
