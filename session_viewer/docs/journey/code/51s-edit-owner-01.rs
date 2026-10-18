use crate::{scene::{Scene,ObjectId},gizmo::{Gizmo,Handle},pick_sources::Target};
use session_rust::Xform;

pub enum Preview { Whole(crate::edit_preview::Preview),Part(crate::part_preview::Preview) }
impl Preview {
    pub fn new(scene:&Scene,ids:Vec<ObjectId>,part:Option<Target>) -> Result<Self,&'static str> {
        match part {Some(target)=>crate::part_preview::Preview::new(scene,target).map(Self::Part),None=>crate::edit_preview::Preview::new(scene,ids).map(Self::Whole)}
    }
    pub fn scene(&self) -> &Scene { match self {Self::Whole(p)=>&p.scene,Self::Part(p)=>&p.scene} }
    pub fn scenes(&self)->[&Scene;2] {match self {Self::Whole(p)=>p.scenes(),Self::Part(p)=>p.scenes()}}
    pub fn matrix(&self)->&[f64;16] {match self {Self::Whole(p)=>p.matrix(),Self::Part(p)=>p.matrix()}}
    pub fn set(&mut self,transform:Xform) -> Result<(), &'static str> {match self {Self::Whole(p)=>p.set(transform),Self::Part(p)=>p.set(transform)}}
    pub fn commit(&self,scene:&mut Scene,history:&mut crate::history::History) -> Result<bool,&'static str> {match self {Self::Whole(p)=>p.commit(scene,history),Self::Part(p)=>p.commit(scene,history)}}
}

