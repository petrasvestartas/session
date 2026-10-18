    out.sort_unstable_by_key(|id|id.value());out.dedup();out
}
pub fn connect(scene:&mut Scene,ids:&[ObjectId])->Result<String,&'static str>{
    let [a,b]=ids else{return Err("Add Edge connects exactly two selected objects")};if a==b{return Err("Connect two different objects")}
    for id in ids{crate::source_edit::editable(scene,*id)?;}
    let docs=ids.iter().map(|id|scene.forest.documents.iter().find(|doc|doc.nodes.iter().any(|n|n.object==Some(*id))).ok_or("Only document objects can be connected")).collect::<Result<Vec<_>,_>>()?;
    if docs[0].key!=docs[1].key{return Err("Add Edge connects two objects of one document")}
    let key=docs[0].key;let names=ids.iter().map(|id|scene.row_guid(*id).unwrap().to_owned()).collect::<Vec<_>>();
    let mut graph=current(scene,docs[0]);
    if graph.has_edge((&names[0],&names[1])){return Err("These objects are already connected")}
    graph.add_edge(&names[0],&names[1],"");Rc::make_mut(&mut scene.graphs).insert(key,Rc::new(graph));Ok(format!("Connected {} and {}",names[0],names[1]))
}
pub fn saved(scene:&Scene,doc:&crate::hierarchy::Document,names:&std::collections::HashMap<String,String>)->Option<session_rust::proto::Graph>{
    let mut graph=scene.graphs.get(&doc.key).map(|g|g.to_proto()).or_else(||doc.header.as_ref()?.header.graph.clone())?;
    let remap=|s:&str|names.get(s).cloned().unwrap_or_else(||s.into());graph.vertices=graph.vertices.into_iter().map(|(key,mut vertex)|{vertex.name=remap(&vertex.name);(remap(&key),vertex)}).collect();for edge in &mut graph.edges{edge.v0=remap(&edge.v0);edge.v1=remap(&edge.v1)}Some(graph)
}

fn current(scene:&Scene,doc:&crate::hierarchy::Document)->session_rust::Graph{
