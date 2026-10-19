use crate::{camera::Camera,scene::{Scene,ObjectId}};
use session_rust::{Point,Vector,Xform};

pub enum Answer {Follow,Move {add:bool,remove:bool},Rectangle,Select {add:bool,remove:bool}}
pub struct Drag {
    pub from:[f32;2],pub to:[f32;2],pub size:[f64;2],pub pointer:i32,
    pub released:Option<(bool,bool)>,pub modified:bool,pub missed:bool,
    pub preview:Option<crate::edit_preview::Preview>,plane:Option<([f64;3],Vector)>,start:Option<[f64;3]>,
}
impl Drag {
    pub fn new(from:[f32;2],to:[f32;2],size:[f64;2],modified:bool)->Option<Self> {
        if from.iter().chain(to.iter()).any(|v|!v.is_finite()||v.abs()>1.)||size.iter().any(|v|!v.is_finite()||*v<=0.) {return None;}
        Some(Self {from,to,size,pointer:0,released:None,modified,missed:modified,preview:None,plane:None,start:None})
    }
    pub fn answer(&self,hit:bool)->Answer {
        match (hit,self.released) {(true,None)=>Answer::Follow,(true,Some((add,remove)))=>Answer::Move {add,remove},(false,None)=>Answer::Rectangle,(false,Some((add,remove)))=>Answer::Select {add,remove}}
    }
    pub fn rectangle(&self)->Option<crate::rect_select::Rectangle> {
        let css=|p:[f32;2]|[(p[0] as f64+1.)*self.size[0]/2.,(1.-p[1] as f64)*self.size[1]/2.];crate::rect_select::Rectangle::new(css(self.from),css(self.to))
    }
    pub fn grab(&mut self,scene:&Scene,ids:Vec<ObjectId>,camera:&Camera)->Result<(),&'static str> {
        let origin=scene.bounds_of(&ids).ok_or("No drag bounds")?.centre();let view=camera.orientation.rotate_vector(Vector::y_axis());
        let normal=if view[2].abs()<0.34 {Vector::new(view[0],view[1],0.).normalized()} else {Vector::z_axis()};
        self.plane=Some((origin,normal));self.start=Some(self.point(camera,self.from).ok_or("Drag plane is parallel to the view")?);
        self.preview=Some(crate::edit_preview::Preview::new(scene,ids)?);self.update(camera)
    }
    fn point(&self,camera:&Camera,screen:[f32;2])->Option<[f64;3]> {
        let (origin,normal)=self.plane.as_ref()?;let ray=camera.ray_unbounded(screen)?;let denominator=ray.direction.dot(normal);if denominator.abs()<1e-9 {return None;}
        let distance=(&Point::new(origin[0],origin[1],origin[2])-&ray.origin).dot(normal)/denominator;
        if !distance.is_finite()||distance<0. {return None;}
        Some(std::array::from_fn(|i|ray.origin[i]+distance*ray.direction[i]))
    }
    pub fn update(&mut self,camera:&Camera)->Result<(),&'static str> {
        let Some(point)=self.point(camera,self.to) else {return Ok(());};let Some(start)=self.start else {return Ok(());};
        if let Some(preview)=self.preview.as_mut() {preview.set(Xform::translation(point[0]-start[0],point[1]-start[1],point[2]-start[2]))?;}Ok(())
    }
}
