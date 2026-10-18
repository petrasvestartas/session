use crate::{manifest::Manifest,scene::Scene};
use std::{collections::HashMap,rc::Rc};

// Repeated manifest entries share immutable sources and display buffers; placements and viewer identities remain independent for each instance.
pub fn prepare(previous:&Scene,manifest:&Manifest,urls:&[String],sources:&HashMap<String,Scene>)->Result<Scene,String> {
    if urls.len()!=manifest.items.len() {return Err("Manifest source inventory does not match".into());}
    let mut scene=previous.clone();scene.clear();
    for (index,(item,url)) in manifest.items.iter().zip(urls).enumerate() {
        let source=sources.get(url).ok_or_else(||format!("Missing manifest source: {url}"))?;
        scene.append_document(source,&item.name,&manifest.place(index,[0.,0.]),item.point_size as f32).map_err(str::to_owned)?;
    }
    for (index,item) in manifest.texts.iter().enumerate() {
        let placement=if item.camera_facing {crate::text::TextPlacement::WorldBillboard {world:item.at,world_height:item.height}} else {crate::text::TextPlacement::WorldPlane {world:item.at,right:item.right,up:item.up,world_height:item.height}};
        let mut source=crate::scene_label::example(placement);source.guid=format!("manifest-text-{index}");source.name=item.text.clone();source.label.text=item.text.clone();source.label.id=index as u32+1;
        scene.insert_text(crate::scene_label::Prepared::new(Rc::new(source)).map_err(str::to_owned)?).map_err(str::to_owned)?;
    }
    Ok(scene)
}
