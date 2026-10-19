    if let Some((index,guid))=metadata.current_layer.as_ref() {let doc=scene.forest.documents.iter().find(|doc|doc.key==keys[*index]).unwrap();let node=doc.nodes.iter().find(|node|node.source.guid==*guid).ok_or("Saved current layer is missing")?;let key=format!("{}/{}",doc.key,node.source.guid);crate::layer_edit::locate(scene,&key).map_err(str::to_owned)?;Rc::make_mut(&mut scene.view).current=Some(key);}
    if let Some(index)=metadata.created_doc {scene.created=Some(keys[index]);}
    for (index,guid) in &metadata.groups{let doc=scene.forest.documents.iter().find(|doc|doc.key==keys[*index]).ok_or("Saved group document is missing")?;let node=doc.nodes.iter().find(|n|n.source.guid==*guid&&n.object.is_none()).ok_or("Saved group layer is missing")?;Rc::make_mut(&mut scene.groups).insert((doc.key,node.source.guid.clone()));}
    Ok(())
}
