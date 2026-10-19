use crate::{scene::{ObjectId,Scene},source_edit::Owner,source_cut::{Cutter,as_curve,as_plane,plane_moved,fence_plane,nearest_px,unit_ray},trim_parts::{Parts,Blade}};
use session_rust::{Geometry,Point,Line,Vector,Xform,intersection};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Phase{Targets,Cutters,Parts}
pub struct Cut{pub owner:Owner,pub parts:Parts,pub removed:Vec<usize>,pub outlines:Vec<Vec<Point>>,pub marks:Vec<Point>,pub section:Vec<Vec<Point>>}
pub struct Draft{pub before:Vec<ObjectId>,pub targets:Vec<Owner>,pub cutters:Vec<Owner>,pub phase:Phase,pub cuts:Vec<Cut>,pub hover:Option<(usize,usize)>,pub preview:Option<Scene>,pub missed:usize}
impl Draft{
    pub fn new(scene:&Scene,before:Vec<ObjectId>)->Result<Self,String>{
        let mut draft=Self{before:before.clone(),targets:Vec::new(),cutters:Vec::new(),phase:Phase::Targets,cuts:Vec::new(),hover:None,preview:None,missed:0};
        for id in before{draft.pick(scene,Some(id))?;}
        if !draft.targets.is_empty(){draft.phase=Phase::Cutters;}Ok(draft)
    }
    pub fn prompt(&self)->String{match self.phase{Phase::Targets=>format!("Trim · pick objects, Enter ({} selected)",self.targets.len()),Phase::Cutters=>format!("Trim · pick cutters, Enter ({} selected)",self.cutters.len()),Phase::Parts=>format!("Trim · click parts to remove, Enter finishes · Esc cancels ({} not crossed)",self.missed)}}
    pub fn picks(&self)->bool{self.phase!=Phase::Parts}
    pub fn pick(&mut self,scene:&Scene,id:Option<ObjectId>)->Result<(),String>{
        let id=id.ok_or("Nothing there · pick an original")?;
        if !self.picks(){return Err("Trim now needs a part click".into())}
        if self.phase==Phase::Cutters&&self.targets.iter().any(|o|o.id==id){return Err("A target cannot cut itself".into())}
        let list=if self.phase==Phase::Targets{&mut self.targets}else{&mut self.cutters};
        if let Some(at)=list.iter().position(|owner|owner.id==id){list.remove(at);return Ok(())}
        let owner=Owner::capture(scene,id)?;
        if list.len()>=crate::interval_geometry::MAX_POINTS{return Err("Trim accepts at most4096 owners".into())}
        if self.phase==Phase::Targets{
            if !matches!(&owner.source,Geometry::Line(_)|Geometry::Polyline(_)|Geometry::NurbsCurve(_)|Geometry::NurbsSurface(_)|Geometry::BRep(_)|Geometry::Mesh(_)){return Err("Pick an original curve, surface, BRep or mesh".into())}
            let document=|id|scene.forest.documents.iter().find(|d|d.nodes.iter().any(|n|n.object==Some(id))).map(|d|d.key);
            if list.first().is_some_and(|first|document(first.id)!=document(id)){return Err("Trim edits one document at a time".into())}
        }else{
            if as_curve(&owner.source).is_none()&&as_plane(&owner.source).is_none(){return Err("Cut with a curve, Plane or planar surface".into())}
        }
        list.push(owner);Ok(())
    }
}
