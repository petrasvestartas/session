use std::{collections::HashMap,rc::{Rc,Weak}};
use crate::solid_orientation::{solid_orientation,solid_flags,CLOSED};
enum Source{Mesh(Weak<session_rust::Mesh>),Cad(Weak<session_rust::BRep>)}
impl Source{fn alive(&self)->bool{match self{Self::Mesh(source)=>source.strong_count()>0,Self::Cad(source)=>source.strong_count()>0}}}
struct Entry{source:Source,flags:u32}
#[derive(Default)]
pub struct Solids{entries:HashMap<(u8,usize),Entry>,pub checks:u64}
impl Solids{
 pub fn flags(&mut self,object:&crate::scene::Object)->u32{
  if object.mesh.indices().is_empty(){return 0;}
  let cad=match &object.original{Some(session_rust::Geometry::BRep(source))=>Some(source),_=>object.cad.as_ref().map(|cad|&cad.source)};
  if let Some(source)=cad{
   let key=(1,Rc::as_ptr(source)as usize);
   if let Some(entry)=self.entries.get(&key).filter(|e|e.source.alive()){return entry.flags;}
   let flags=if source.is_solid(){CLOSED}else{0};self.entries.insert(key,Entry{source:Source::Cad(Rc::downgrade(source)),flags});self.checks+=1;return flags;
  }
  let mesh=match &object.original{Some(session_rust::Geometry::Mesh(source))=>Some(source),_=>object.geometry()};
  let Some(source)=mesh else{return 0};let key=(0,Rc::as_ptr(source)as usize);
  if let Some(entry)=self.entries.get(&key).filter(|e|e.source.alive()){return entry.flags;}
  let flags=solid_flags(solid_orientation(source));self.entries.insert(key,Entry{source:Source::Mesh(Rc::downgrade(source)),flags});self.checks+=1;flags
 }
 pub fn prune(&mut self){self.entries.retain(|_,entry|entry.source.alive());if self.entries.is_empty(){self.entries.shrink_to_fit();}}
 pub fn count(&self)->usize{self.entries.len()}
}
