use crate::scene::Scene;
use std::rc::Rc;

pub fn prepare(scene:&Scene,bytes:&[u8],replace:bool,location:Option<Rc<crate::reload_url::ReloadUrl>>)->Result<Scene,String> {
    let mut candidate=scene.clone();
    if bytes.starts_with(crate::archive::MAGIC) {crate::scene_document::open(&mut candidate,bytes)?;return Ok(candidate);}
    if replace {candidate.clear();}
    match crate::document::load_at(bytes,location) {
        Ok(loaded)=>candidate.import(loaded).map_err(str::to_owned)?,
        Err(_)=>{crate::scene_document::import(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.)?;}
    }
    Ok(candidate)
}
