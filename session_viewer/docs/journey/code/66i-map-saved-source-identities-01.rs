use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn aliases(scene:&Scene,namespace:&Namespace)->Result<HashMap<String,String>,String> {
    let mut names=HashMap::new();let mut used=HashSet::new();let reserved=namespace.ids.iter().filter_map(|id|scene.row_guid(*id).map(str::to_owned)).collect::<HashSet<_>>();
    if let Some(doc)=namespace.doc.as_ref() {for node in &doc.nodes {if let Some(id)=node.object.filter(|id|namespace.ids.contains(id)) {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?.to_owned();let mut saved=if scene.texts().iter().any(|row|row.id==id) {current.clone()} else {node.source.name.clone()};
        if saved!=current&&reserved.contains(&saved)||used.contains(&saved) {saved=current.clone();}if !used.insert(saved.clone()) {return Err("Saved source identities collide".into());}names.insert(current,saved);
    }}}
    Ok(names)
}
pub(crate) fn objects(objects:&mut proto::Objects,names:&HashMap<String,String>) {
    macro_rules! remap {($field:ident)=>{for object in &mut objects.$field {if let Some(guid)=names.get(&object.guid) {object.guid=guid.clone();}}};}
    remap!(meshes);remap!(lines);remap!(points);remap!(polylines);remap!(nurbscurves);remap!(nurbssurfaces);remap!(breps);remap!(pointclouds);
}
pub(crate) fn tree(node:&mut proto::TreeNode,names:&HashMap<String,String>) {
    if let Some(guid)=names.get(&node.name) {node.name=guid.clone();}for child in &mut node.children {tree(child,names);}
}
