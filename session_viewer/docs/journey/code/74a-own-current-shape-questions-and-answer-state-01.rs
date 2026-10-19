use crate::{cplane::CPlane,primitive_shape::{Answer,Ask,Kind,Part,Shape,frame_of}};
use session_rust::{Geometry,Plane,Point,Vector};

#[derive(Clone)]
pub struct Draft {
    pub shape:&'static Shape,
    pub option:usize,
    pub answers:Vec<Answer>,
    pub part:Part,
    pub plane:CPlane,
}

impl Draft {
    pub fn new(shape:&'static Shape,plane:CPlane,option:usize)->Result<Self,String> {
        if option>=shape.choices().count().max(1) {return Err("Unknown shape option".into());}
        let (x,y)=plane.axes();let part=Part::new(frame_of(&Plane::new(plane.origin(),x,y),&[]));
        Ok(Self {shape,option,answers:Vec::new(),part,plane})
    }
    pub fn choice(&self)->&'static str {self.shape.choices().nth(self.option).unwrap_or("")}
    pub fn next(&self)->Option<Ask> {(self.shape.ask)(&self.answers,&self.part,self.choice())}
    pub fn prompt(&self)->String {
        let Some(ask)=self.next() else{return String::new()};
        let default=ask.default.map_or(String::new(),|value|format!(" · Enter uses {value}"));
        format!("{} · {}{default} · Esc cancels",self.shape.name,ask.prompt)
    }
}
