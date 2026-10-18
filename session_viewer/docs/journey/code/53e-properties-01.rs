use crate::{scene::Scene,hierarchy};
use std::rc::Rc;

#[derive(Clone,Copy)]
pub enum Property {Hidden(bool),Locked(bool),Colour {edge:bool,rgb:Option<[u8;3]>}}
pub fn apply(scene:&mut Scene,key:&str,property:Property)->Result<(), &'static str> {
    let rows=hierarchy::rows(scene);let index=rows.iter().position(|row|row.key==key).ok_or("This layer row is gone")?;let row=&rows[index];
    if matches!(property,Property::Hidden(true)) && scene.view.current.as_ref().is_some_and(|key|rows[index..row.end].iter().any(|row|&row.key==key)) {return Err("The current layer cannot be hidden");}
    let view=Rc::make_mut(&mut scene.view);
    match property {
        Property::Hidden(on)=>{
            let group=scene.forest.documents.iter().any(|doc|doc.nodes.iter().any(|node|node.object.is_none()&&format!("{}/{}",doc.key,node.source.guid)==key));
            if group {if on {view.lamps.insert(key.to_owned());}else{view.lamps.remove(key);}} else {view.hide(&row.ids,on);}
            view.refresh(&scene.forest);
        },
        Property::Locked(on)=>view.lock(&row.ids,on),
        Property::Colour {edge,rgb}=>{let colours=if edge {&mut view.edges} else {&mut view.faces};for id in &row.ids {if let Some(rgb)=rgb {colours.insert(*id,rgb);} else {colours.remove(id);}}},
    }
    Ok(())
}
