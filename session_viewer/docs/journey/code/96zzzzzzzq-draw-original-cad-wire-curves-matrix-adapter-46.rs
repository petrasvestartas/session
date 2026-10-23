}
pub(crate) fn objects(objects:&mut proto::Objects,names:&HashMap<String,String>) {
    macro_rules! remap {($field:ident)=>{for object in &mut objects.$field {if let Some(guid)=names.get(&object.guid) {object.guid=guid.clone();}}};}
    remap!(elements);remap!(planes);remap!(bboxes);remap!(meshes);remap!(lines);remap!(points);remap!(polylines);remap!(nurbscurves);remap!(nurbssurfaces);remap!(breps);remap!(pointclouds);remap!(instances);remap!(sheets);remap!(components);
}
pub(crate) fn tree(node:&mut proto::TreeNode,names:&HashMap<String,String>) {
    if let Some(guid)=names.get(&node.name) {node.name=guid.clone();}for child in &mut node.children {tree(child,names);}
