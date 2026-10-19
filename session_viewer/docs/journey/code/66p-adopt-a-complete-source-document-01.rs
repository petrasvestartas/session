use crate::scene::Scene;
use session_rust::Xform;
use std::rc::Rc;
#[cfg(test)]
use {session_rust::proto,prost::Message,std::collections::HashSet};

// Prepare every original source in a temporary scene; one successful adoption prevents partially imported files from replacing the document.
pub fn import(scene:&mut Scene,bytes:&[u8],name:&str,place:&Xform,point_px:f32)->Result<u32,String> {
    let source=crate::document_source::prepare(bytes,place,point_px)?;
    let mut candidate=scene.clone();let forest=candidate.forest.clone();let created=candidate.created;let current=candidate.view.current.clone();
    candidate.forest=Default::default();candidate.created=None;Rc::make_mut(&mut candidate.view).current=None;
    let key=candidate.reserve_document_key().map_err(str::to_owned)?;let owners=crate::document_geometry::append(&mut candidate,&source,place,point_px)?;
    candidate.forest=forest;candidate.created=created;Rc::make_mut(&mut candidate.view).current=current;
    candidate.forest.add(key,if name.is_empty() {source.document.name.clone()} else {name.into()},source.tree,&owners);
    Rc::make_mut(candidate.forest.documents.last_mut().unwrap()).header=Some(source.origin);
    let colours=candidate.forest.colours(key);Rc::make_mut(&mut candidate.view).faces.extend(colours);
    scene.restore(candidate);Ok(key)
}

