use crate::{scene::Scene,scene_archive::Namespace};
use session_rust::proto;
use std::collections::{HashMap,HashSet};

pub(crate) fn prepare(scene:&Scene,namespace:&Namespace)->Result<(proto::Session,HashMap<String,String>,f32),String> {
    let names=crate::archive_identity::aliases(scene,namespace)?;let mut objects=crate::geometry_snapshot::objects(scene,&namespace.ids)?;crate::archive_identity::objects(&mut objects,&names);
    let mut xforms=Vec::new();let (mut roots,placed)=if let Some(doc)=namespace.doc.as_ref() {let mut forest=crate::hierarchy::Forest::default();forest.documents.push(doc.clone());forest.snapshot(scene,&mut xforms).map_err(str::to_owned)?} else {(vec![],HashSet::new())};
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.into());
        if !placed.contains(&id) {roots.push(proto::TreeNode {name:guid.clone(),..Default::default()});let model=scene.placement(id).ok_or("Saved source placement is missing")?;if model.m!=session_rust::Xform::identity().m {xforms.push(proto::XformEntry {guid,xform:Some(model.to_proto())});}}
    }
    for root in &mut roots {crate::archive_identity::tree(root,&names);}for entry in &mut xforms {if let Some(guid)=names.get(&entry.guid) {entry.guid=guid.clone();}}
    let mut source=namespace.doc.as_ref().and_then(|doc|doc.header.as_ref()).map_or_else(proto::Session::default,|header|header.header.clone());if source.name.is_empty() {source.name=namespace.name.clone();}
    let root=if namespace.doc.is_some()&&roots.len()==1 {roots.pop().unwrap()} else {proto::TreeNode {name:source.name.clone(),children:roots,..Default::default()}};
    source.objects=Some(objects);source.tree=Some(proto::Tree {root:Some(root),..Default::default()});source.xforms=xforms;source.bvh_boxes.clear();
    let mut clouds=scene.clouds().iter().filter(|row|namespace.ids.contains(&row.id));let point_px=clouds.next().map_or(0.,|row|row.point_px);
    if clouds.any(|row|row.point_px!=point_px) {return Err("Cloud placements in one saved document need the same point size override".into());}Ok((source,names,point_px))
}
