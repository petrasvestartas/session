use crate::scene::{ObjectId,Scene};
use std::rc::Rc;
pub fn expand(scene:&Scene,ids:impl IntoIterator<Item=ObjectId>)->Vec<ObjectId>{
    let ids=ids.into_iter().collect::<Vec<_>>();let mut out=ids.clone();
    for doc in &scene.forest.documents{for (index,node) in doc.nodes.iter().enumerate(){if !scene.groups.contains(&(doc.key,node.source.guid.clone())){continue}let members=doc.nodes[index+1..node.end].iter().filter_map(|n|n.object).collect::<Vec<_>>();if members.iter().any(|id|ids.contains(id)){out.extend(members)}}}
    out.sort_unstable_by_key(|id|id.value());out.dedup();out
}



fn current(scene:&Scene,doc:&crate::hierarchy::Document)->session_rust::Graph{
    scene.graphs.get(&doc.key).map_or_else(||{
        let mut source=doc.header.as_ref().and_then(|header|header.header.graph.clone()).unwrap_or_default();
        let aliases=doc.nodes.iter().filter_map(|node|Some((node.source.name.clone(),scene.row_guid(node.object?)?.to_owned()))).collect::<std::collections::HashMap<_,_>>();
        let remap=|s:&str|aliases.get(s).cloned().unwrap_or_else(||s.into());source.vertices=source.vertices.into_iter().map(|(key,mut vertex)|{vertex.name=remap(&vertex.name);(remap(&key),vertex)}).collect();for edge in &mut source.edges{edge.v0=remap(&edge.v0);edge.v1=remap(&edge.v1)}session_rust::Graph::from_proto(source)
    },|g|(**g).clone())

}






