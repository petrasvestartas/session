        if scene.created==Some(doc.key) {metadata.created_doc=Some(index);}
        if let Some(current)=scene.view.current.as_ref() {if let Some(guid)=current.strip_prefix(&format!("{}/",doc.key)) {if let Some(node)=doc.nodes.iter().find(|node|node.source.guid==guid) {metadata.current_layer=Some((index,node.portable_guid.clone()));}}}
        for node in &doc.nodes {if scene.groups.contains(&(doc.key,node.source.guid.clone())){metadata.groups.push((index,node.portable_guid.clone()));}}
    }Ok(())
}
