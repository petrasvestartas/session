use crate::{clipping_geometry::{Mode,plane_from},tool::{Tool,Built}};
#[derive(Debug)]
pub struct Clipping{pub mode:Mode,pub half:f64}
impl Tool for Clipping{
    fn name(&self)->&'static str{"Clipping Plane"}
    fn command_prefix(&self)->String{format!("Clipping Plane {}",self.mode.word())}
    fn point_prompt(&self,index:usize)->Option<String>{Some(format!("Clipping Plane {} · {} · Esc cancels",self.mode.word(),self.mode.prompts()[index.min(self.count()-1)]))}
    fn count(&self)->usize{self.mode.points()}
    fn build(&self,points:&[[f64;3]])->Result<Built,&'static str>{
        if !self.half.is_finite()||self.half<=0.||points.iter().flatten().any(|v|!v.is_finite()||v.abs()>1e12){return Err("Invalid clipping plane coordinates or size")}
        let plane=plane_from(self.mode,points,self.half).map_err(|_|"Clipping plane points are equal, collinear or incomplete")?;
        Ok(Built::Geometry(session_rust::Geometry::Plane(std::rc::Rc::new(plane))))
    }
    fn outline(&self,points:&[[f64;3]],_:crate::cplane::CPlane)->Vec<[f64;3]>{
        if let Ok(plane)=plane_from(self.mode,points,self.half){if let Ok(p)=crate::plane_original::points(&plane){return [0,1,2,3,0].into_iter().map(|i|[p[i][0],p[i][1],p[i][2]]).collect()}}points.to_vec()
    }
    fn buttons(&self)->Vec<(String,String)>{Mode::ALL.into_iter().map(|m|(m.word().into(),format!("Clipping Plane {}",m.word()))).chain([("On".into(),"Clipping Plane On".into()),("Off".into(),"Clipping Plane Off".into()),("Cancel".into(),"Escape".into())]).collect()}
}
