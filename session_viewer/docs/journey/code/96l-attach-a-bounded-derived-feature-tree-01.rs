 let mut count=0;let mut max=0;fn size(b:&Branch,depth:usize,count:&mut usize,max:&mut usize){*count+=1;*max=(*max).max(depth);for c in &b.children{size(c,depth+1,count,max)}}size(&root,0,&mut count,&mut max);if count>crate::hierarchy::MAX_NODES||max>crate::hierarchy::MAX_DEPTH{return Err("Element attributes exceed hierarchy limits")}replace(scene,owner,root);Ok(())
}

pub enum FeatureTree {
    Group(String, Vec<FeatureTree>),
    Piece(String, crate::scene::ObjectId),
}

pub fn attach_features(scene: &mut Scene, id: crate::scene::ObjectId, groups: Vec<FeatureTree>) -> Result<(), &'static str> {
    fn build(scene: &Scene, tree: FeatureTree) -> Result<Branch, &'static str> {
        let (label, object, children) = match tree {
            FeatureTree::Group(label, children) => (label, None, children),
            FeatureTree::Piece(label, id) => (label, Some(id), Vec::new()),
        };
        let name = match object {
            Some(id) => scene.row_guid(id).ok_or("Feature piece is missing")?.to_owned(),
            None => label.clone(),
        };
        let source = session_rust::proto::TreeNode {
            guid: uuid::Uuid::new_v4().to_string(), name, ..Default::default()
        };
        let node = Node { source: source.clone(), portable_guid: source.guid,
            object, label, parent: None, depth: 0, end: 1, readonly: true,
            derived: true, placement: Default::default() };
        let children = children.into_iter().map(|child| build(scene, child)).collect::<Result<_, _>>()?;
        Ok(Branch { node, children })
    }
    let (owner, index) = scene.forest.documents.iter().enumerate().find_map(|(owner, doc)|
        doc.nodes.iter().position(|node| node.object == Some(id)).map(|index| (owner, index)))
        .ok_or("Element tree is missing")?;
    let doc = &scene.forest.documents[owner];
    let guid = doc.nodes[index].source.guid.clone();
    let mut tree = branch(doc, 0);
    let parent = find(&mut tree, &guid).unwrap();
    for group in groups {
        let group = build(scene, group)?;
        if let Some(existing) = parent.children.iter_mut().find(|child|
            child.node.object.is_none() && child.node.source.name == group.node.source.name) {
            existing.children.extend(group.children);
        } else { parent.children.push(group); }
    }
    let mut nodes = Vec::new();
    flatten(tree.clone(), None, 0, &mut nodes);
    if nodes.len() > crate::hierarchy::MAX_NODES
        || nodes.iter().any(|node| node.depth > crate::hierarchy::MAX_DEPTH) {
        return Err("Element features exceed the tree limit");
    }
    replace(scene, owner, tree);
    Ok(())
}
