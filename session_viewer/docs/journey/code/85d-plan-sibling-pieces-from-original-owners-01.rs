use crate::{scene::{ObjectId,Scene},source_edit::Owner};
use session_rust::{Geometry,Xform};
pub struct Plan{targets:Vec<(Owner,Vec<Geometry>)>,count:usize}
impl Plan{
    pub fn prepare(scene:&Scene,ids:&[ObjectId])->Result<Self,String>{
        let mut targets=Vec::new();let mut count=0;let mut refused=None;
        for &id in ids{
            let owner=Owner::capture(scene,id)?;
            if scene.forest.documents.iter().any(|d|d.nodes.iter().enumerate().any(|(i,n)|n.object==Some(id)&&n.end>i+1)){return Err("Explode needs an object without child objects".into())}
            match crate::explode_geometry::pieces(&owner.source){Ok(pieces)=>{count+=pieces.len();if count>crate::interval_geometry::MAX_POINTS{return Err("Explode makes at most4096 objects at once".into())}targets.push((owner,pieces));},Err(error)=>refused=Some(error)}
        }
        if targets.is_empty(){return Err(refused.unwrap_or_else(||"Nothing to explode".into()))}Ok(Self{targets,count})
    }
    pub fn adopt(self,scene:&mut Scene)->Result<Vec<ObjectId>,&'static str>{
        let mut made=Vec::with_capacity(self.count);
        for(owner,pieces)in self.targets{owner.valid(scene)?;for piece in pieces{let copy=crate::layer_edit::copy_objects(scene,&[owner.id],&Xform::identity())?[0];scene.replace_original(copy,piece)?;made.push(copy);}scene.remove(owner.id);}
        Ok(made)
    }
}
