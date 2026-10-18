
pub fn rows(scene:&Scene)->Vec<(String,String,String,String)>{
    let mut rows=Vec::new();
    for doc in &scene.forest.documents{
        let mut labels=std::collections::HashMap::new();
        for node in &doc.nodes{
            labels.entry(node.source.name.as_str()).or_insert(node.label.as_str());
            if let Some(guid)=node.object.and_then(|id|scene.row_guid(id)){labels.entry(guid).or_insert(node.label.as_str());}
        }
        let mut push=|from:&str,to:&str,attribute:&str|rows.push((doc.label.clone(),labels.get(from).copied().unwrap_or(from).to_owned(),labels.get(to).copied().unwrap_or(to).to_owned(),attribute.to_owned()));
        if let Some(graph)=scene.graphs.get(&doc.key){
            for(u,neighbors)in &graph.edges{for(v,edge)in neighbors{if u<=v{push(&edge.v0,&edge.v1,&edge.attribute);}}}
        }else if let Some(graph)=doc.header.as_ref().and_then(|h|h.header.graph.as_ref()){
            for edge in &graph.edges{push(&edge.v0,&edge.v1,&edge.attribute);}
        }
    }
    rows
}
