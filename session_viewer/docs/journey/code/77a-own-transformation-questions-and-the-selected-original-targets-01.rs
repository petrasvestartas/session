use crate::{cplane::CPlane,primitive_shape::Answer};
use session_rust::{Plane,Point,Xform};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Kind {Copy,Move,Scale,Rotate}
impl Kind {pub fn name(self)->&'static str {match self {Self::Copy=>"Copy",Self::Move=>"Move",Self::Scale=>"Scale",Self::Rotate=>"Rotate"}}}
#[derive(Clone)]
pub struct Draft {pub kind:Kind,pub mode:u8,pub points:Vec<[f64;3]>,pub frame:CPlane,pub owners:Vec<crate::scene::ObjectId>,pub target:Option<crate::pick_sources::Target>}
pub struct Outcome {pub transforms:Vec<Xform>,pub finished:bool}
impl Draft {
    pub fn new(kind:Kind,frame:CPlane)->Self {Self{kind,mode:3,points:Vec::new(),frame,owners:Vec::new(),target:None}}
    fn plane(&self)->Plane {let (x,y)=self.frame.axes();Plane::new(self.frame.origin(),x,y)}
    fn p(point:[f64;3])->Point {Point::new(point[0],point[1],point[2])}
    pub fn prompt(&self)->String {
        let question=match(self.kind,self.points.len(),self.mode) {
            (Kind::Copy|Kind::Move,0,_)=>"Base point",(Kind::Copy,_,_)=>"Target point · Enter finishes",(Kind::Move,_,_)=>"Target point or distance along the band",
            (Kind::Scale,0,_)=>"Origin point",(Kind::Scale,1,1)=>"First reference point sets the direction",(Kind::Scale,1,_)=>"Scale factor or first reference point",(Kind::Scale,_,_)=>"Second reference point or new length",
            (Kind::Rotate,0,_)=>"Center of rotation",(Kind::Rotate,1,_)=>"Angle or first reference point",(Kind::Rotate,_,_)=>"Second reference point or angle",
        };format!("{} · {question} · Esc cancels{}",self.kind.name(),if self.kind==Kind::Scale{format!(" · {}D",self.mode)}else{String::new()})
    }
}
