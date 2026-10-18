use crate::{archive::Metadata,scene::{Scene,ObjectId}};
use std::{collections::HashMap,rc::Rc};

pub(crate) fn restore(scene:&mut Scene,metadata:&Metadata,keys:&[u32])->Result<(),String> {
    let mut owners=HashMap::new();for (index,key) in keys.iter().enumerate() {let doc=scene.forest.documents.iter().find(|doc|doc.key==*key).unwrap();for node in &doc.nodes {if let Some(id)=node.object {owners.insert((index,node.source.name.as_str()),id);}}}
    let owner=|doc:usize,guid:&str|owners.get(&(doc,guid)).copied().ok_or_else(||"Saved presentation owner is missing".to_string());
    let hidden=metadata.hidden.iter().map(|(doc,guid)|owner(*doc,guid)).collect::<Result<Vec<ObjectId>,_>>()?;
    let locked=metadata.locked.iter().map(|(doc,guid)|owner(*doc,guid)).collect::<Result<Vec<ObjectId>,_>>()?;
    let colours=metadata.colors.iter().map(|(doc,guid,rgb)|Ok((owner(*doc,guid)?,*rgb))).collect::<Result<Vec<_>,String>>()?;
    let edges=metadata.edge_colors.as_ref().unwrap_or(&metadata.colors).iter().map(|(doc,guid,rgb)|Ok((owner(*doc,guid)?,*rgb))).collect::<Result<Vec<_>,String>>()?;
    drop(owners);let view=Rc::make_mut(&mut scene.view);view.hide(&hidden,true);view.lock(&locked,true);view.faces.extend(colours);view.edges.extend(edges);view.current=None;
    if let Some((index,guid))=metadata.current_layer.as_ref() {let doc=scene.forest.documents.iter().find(|doc|doc.key==keys[*index]).unwrap();let node=doc.nodes.iter().find(|node|node.source.guid==*guid).ok_or("Saved current layer is missing")?;let key=format!("{}/{}",doc.key,node.source.guid);crate::layer_edit::locate(scene,&key).map_err(str::to_owned)?;Rc::make_mut(&mut scene.view).current=Some(key);}
    if let Some(index)=metadata.created_doc {scene.created=Some(keys[index]);}
    Ok(())
}
