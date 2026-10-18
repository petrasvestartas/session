}

pub fn remove(scene:&mut Scene,id:ObjectId){
    let Some(guid)=scene.row_guid(id).map(str::to_owned)else{return};
    let Some(doc)=scene.forest.documents.iter().find(|doc|doc.nodes.iter().any(|n|n.object==Some(id))).cloned()else{return};
    if !scene.graphs.contains_key(&doc.key)&&doc.header.as_ref().and_then(|h|h.header.graph.as_ref()).is_none(){return}
    let mut graph=current(scene,&doc);graph.remove_node(&guid);Rc::make_mut(&mut scene.graphs).insert(doc.key,Rc::new(graph));
}

pub fn transferred(scene:&Scene,doc:&crate::hierarchy::Document,names:&std::collections::HashMap<String,String>)->session_rust::proto::Graph{
    let mut graph=current(scene,doc).to_proto();let remap=|s:&str|names.get(s).cloned().unwrap_or_else(||s.into());graph.vertices=graph.vertices.into_iter().map(|(key,mut vertex)|{vertex.name=remap(&vertex.name);(remap(&key),vertex)}).collect();for edge in &mut graph.edges{edge.v0=remap(&edge.v0);edge.v1=remap(&edge.v1)}graph
}

pub fn rows(scene:&Scene)->Vec<(String,String,String,String)>{
    scene.forest.documents.iter().flat_map(|doc|{let graph=scene.graphs.get(&doc.key).map(|g|g.to_proto()).or_else(||doc.header.as_ref().and_then(|h|h.header.graph.clone()));let name=|guid:&str|doc.nodes.iter().find(|n|n.source.name==guid||n.object.is_some_and(|id|scene.row_guid(id)==Some(guid))).map_or_else(||guid.to_owned(),|n|n.label.clone());graph.into_iter().flat_map(|g|g.edges.into_iter().map(|edge|(doc.label.clone(),name(&edge.v0),name(&edge.v1),edge.attribute))).collect::<Vec<_>>()}).collect()
}
