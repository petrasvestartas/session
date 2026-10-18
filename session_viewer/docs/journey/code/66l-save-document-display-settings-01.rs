use crate::{archive::Metadata,scene::Scene,scene_archive::Namespace};
use std::collections::HashMap;

pub(crate) fn append(metadata:&mut Metadata,index:usize,namespace:&Namespace,scene:&Scene,names:&HashMap<String,String>)->Result<(),String> {
    for id in namespace.ordered() {
        let current=scene.row_guid(id).ok_or("Saved source owner is missing")?;let guid=names.get(current).cloned().unwrap_or_else(||current.to_owned());
        if !scene.visible(id) {metadata.hidden.push((index,guid.clone()));}if scene.view.locked.contains(&id) {metadata.locked.push((index,guid.clone()));}
        if let Some(rgb)=scene.view.faces.get(&id) {metadata.colors.push((index,guid.clone(),*rgb));}if let Some(rgb)=scene.view.edges.get(&id) {metadata.edge_colors.as_mut().unwrap().push((index,guid,*rgb));}
    }
    if let Some(doc)=namespace.doc.as_ref() {
        if scene.created==Some(doc.key) {metadata.created_doc=Some(index);}
        if let Some(current)=scene.view.current.as_ref() {if let Some(guid)=current.strip_prefix(&format!("{}/",doc.key)) {if let Some(node)=doc.nodes.iter().find(|node|node.source.guid==guid) {metadata.current_layer=Some((index,node.portable_guid.clone()));}}}
    }Ok(())
}
