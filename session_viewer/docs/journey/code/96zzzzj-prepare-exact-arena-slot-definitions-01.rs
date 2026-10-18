use std::{collections::BTreeMap,ops::Range,rc::Rc};
use crate::gpu_geometry::GpuGeometry;
pub struct Members {pub key:u32,pub faces:Range<u32>,pub rows:Vec<u32>}
impl Members {
 pub fn definition(&self)->crate::slot_space::Definition<'_> {
  crate::slot_space::Definition {key:self.key,rows:&self.rows,faces:self.faces.clone(),pipes:0..0,ribbons:0..0}
 }
}
pub fn collect<'a>(sources:impl IntoIterator<Item=(&'a Rc<GpuGeometry>,Vec<u32>)>)->Option<(Vec<Members>,u32)> {
 let mut arena=None;let mut count=0;let mut members=BTreeMap::<u32,Members>::new();
 for(geometry,rows)in sources {
  let current=geometry.arena()?;if arena.is_some_and(|first|!Rc::ptr_eq(first,current)){return None;}arena=Some(current);
  count=u32::try_from(current.borrow().rows.indices.len()/3).ok()?;
  let first=geometry.index_start;let end=first.checked_add(geometry.index_count)?;
  if first%3!=0||geometry.index_count%3!=0{return None;}
  let member=members.entry(first).or_insert_with(||Members {key:first,faces:first..end,rows:Vec::new()});
  if member.faces.end!=end{return None;}member.rows.extend(rows);
 }
 Some((members.into_values().collect(),count))
}
