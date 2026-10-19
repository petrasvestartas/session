use std::{collections::HashMap,rc::{Rc,Weak}};
#[derive(Default)]
pub struct Bounds{entries:HashMap<usize,(Weak<crate::mesh::Mesh>,[[f64;3];2])>}
impl Bounds{
 pub fn placed(&mut self,source:&Rc<crate::mesh::Mesh>,model:&session_rust::Xform)->Option<session_rust::AABB>{
  let key=Rc::as_ptr(source)as usize;
  let local=if let Some((_,bounds))=self.entries.get(&key).filter(|(weak,_)|weak.strong_count()>0){*bounds}else{
   let first=source.vertices().first()?;let mut bounds=[[f64::from(first[0]),f64::from(first[1]),f64::from(first[2])];2];for vertex in source.vertices(){for i in 0..3{bounds[0][i]=bounds[0][i].min(f64::from(vertex[i]));bounds[1][i]=bounds[1][i].max(f64::from(vertex[i]));}}self.entries.insert(key,(Rc::downgrade(source),bounds));bounds
  };
  let mut world=session_rust::AABB::empty();for corner in 0..8{let p=[0,1,2].map(|i|local[(corner>>i)&1][i]);let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));world.union_with_point(p[0],p[1],p[2]);}Some(world)
 }
 pub fn prune(&mut self){self.entries.retain(|_,(weak,_)|weak.strong_count()>0);if self.entries.is_empty(){self.entries.shrink_to_fit();}}
}
