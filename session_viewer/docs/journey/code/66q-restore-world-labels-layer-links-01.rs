use crate::{archive::Metadata,scene::Scene};
use std::{collections::{HashMap,HashSet},rc::Rc};

pub(crate) fn restore(scene:&mut Scene,metadata:&Metadata)->Result<(),String> {
    let forest=scene.forest.clone();let mut texts=HashMap::new();
    for (key,label,active) in &metadata.texts {
        let mut label=label.clone();label.object=None;
        let source=crate::scene_label::Source {guid:key.clone(),name:label.text.clone(),visible:*active,locked:false,label};
        let id=scene.insert_text(crate::scene_label::Prepared::new(Rc::new(source)).map_err(str::to_owned)?).map_err(str::to_owned)?;texts.insert(key.clone(),id);
    }
    scene.forest=forest;scene.created=None;
    let mut linked=HashSet::new();
    for index in 0..scene.forest.documents.len() {
        let mut doc=(*scene.forest.documents[index]).clone();
        for node in &mut doc.nodes {node.portable_guid=node.source.guid.clone();}
        for node in &mut doc.nodes {if node.object.is_none() {if let Some(id)=texts.get(&node.source.name).copied() {if !linked.insert(id) {return Err("Text occurs in two saved layers".into());}node.object=Some(id);node.label=scene.texts().iter().find(|row|row.id==id).unwrap().prepared.source.name.clone();}}}
        scene.forest.replace(index,doc);
    }
    for id in texts.values().copied().filter(|id|!linked.contains(id)).collect::<Vec<_>>() {let row=scene.texts().iter().find(|row|row.id==id).unwrap();let guid=row.guid.clone();let label=row.prepared.source.name.clone();crate::layer_edit::attach_generated(scene,id,guid,label).map_err(str::to_owned)?;}
    Ok(())
}
