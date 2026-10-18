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

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn exact_ranges_merge_members_and_reject_unrelated_storage() {
  let renderer=crate::cap_gpu_tests::renderer(&crate::scene::Scene::empty());let ctx=crate::text_gpu_support::GpuCtx::new(&renderer.device,&renderer.queue);
  let mut scene=crate::scene::Scene::empty();scene.insert(crate::prepared::PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let source=scene.objects()[0].mesh.clone();let other=Rc::new((*source).clone());
  let arena=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let a=Rc::new(GpuGeometry::shared(&ctx,source.clone(),arena.clone()).unwrap());let b=Rc::new(GpuGeometry::shared(&ctx,other,arena).unwrap());
  let(members,count)=collect([(&a,vec![7,8]),(&b,vec![9]),(&a,vec![10])]).unwrap();assert_eq!(count,2);assert_eq!(members.len(),2);assert_eq!((members[0].key,members[1].key),(0,3));assert_eq!(members[0].rows,[7,8,10]);assert_eq!(members[1].rows,[9]);let definition=members[1].definition();assert_eq!(definition.faces,3..6);assert_eq!(definition.rows,[9]);
  let owned=Rc::new(GpuGeometry::upload(&ctx.device,source.clone()));assert!(collect([(&owned,vec![7])]).is_none());let different=Rc::new(std::cell::RefCell::new(crate::mesh_arena::Arena::new(&ctx)));let different=Rc::new(GpuGeometry::shared(&ctx,source,different).unwrap());assert!(collect([(&a,vec![7]),(&different,vec![8])]).is_none());let(empty,count)=collect(std::iter::empty()).unwrap();assert!(empty.is_empty());assert_eq!(count,0);
 }
}
