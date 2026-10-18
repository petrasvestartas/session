use std::rc::Rc;
fn source()->Rc<crate::mesh::Mesh>{Rc::new(crate::mesh::Mesh::new(vec![[0.,0.,0.,0.9,0.1,0.1],[1.,0.,0.,0.9,0.1,0.1],[0.,1.,0.,0.9,0.1,0.1]],vec![0,1,2]).unwrap().with_alphas(vec![1./255.;3]).unwrap())}
#[test]
fn ranges_share_identity_and_rebase_indices_without_changing_packed_positions(){
 let a=source();let b=source();let mut rows=crate::arena_rows::Rows::default();
 let first=rows.append(&a).unwrap();assert_eq!(first.vertices,0..3);assert_eq!(first.indices,0..3);
 assert_eq!(rows.append(&Rc::clone(&a)).unwrap(),first);assert_eq!(rows.source_count(),1);
 let second=rows.append(&b).unwrap();assert_eq!(second.vertices,3..6);assert_eq!(second.indices,3..6);assert_eq!(rows.source_count(),2);
 assert_eq!(rows.indices,[0,1,2,3,4,5]);assert_eq!(rows.vertices,[crate::packed_vertex::bytes(&a),crate::packed_vertex::bytes(&b)].concat());
 assert_eq!(rows.vertices.len(),6*20);assert_eq!(rows.vertices[19],1);assert_eq!(rows.index_bytes(),[0u32,1,2,3,4,5].into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>());
 assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);
}
#[test]
fn arena_records_do_not_pin_sources_and_reset_reuses_storage(){
 let mesh=source();let weak=Rc::downgrade(&mesh);let mut rows=crate::arena_rows::Rows::default();rows.append(&mesh).unwrap();
 let cap=[rows.vertices.capacity(),rows.indices.capacity()];drop(mesh);assert!(weak.upgrade().is_none());
 rows.reset();assert_eq!(rows.source_count(),0);assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],cap);
 let next=source();assert_eq!(rows.append(&next).unwrap().vertices,0..3);rows.release();assert_eq!([rows.vertices.capacity(),rows.indices.capacity()],[0,0]);
}

#[test]
fn retired_ranges_reuse_holes_without_pinning_or_moving_surviving_sources() {
 let a=source();let b=source();let history=Rc::clone(&a);let weak=Rc::downgrade(&a);let mut rows=crate::arena_rows::Rows::default();let first=rows.append(&a).unwrap();let survivor=rows.append(&b).unwrap();let held=rows.vertices[60..120].to_vec();
 assert!(!rows.retire(usize::MAX));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert!(weak.upgrade().is_some());assert_eq!(rows.source_count(),1);
 let c=Rc::new((*source()).clone().with_alphas(vec![0.5;3]).unwrap());let reused=rows.append(&c).unwrap();assert_eq!(reused,first);assert_eq!(rows.append(&b).unwrap(),survivor);assert_eq!(rows.vertices.len(),120);assert_eq!(&rows.vertices[..60],crate::packed_vertex::bytes(&c));assert_eq!(&rows.vertices[60..120],held);assert_eq!(rows.indices,[0,1,2,3,4,5]);
 let restored=rows.append(&history).unwrap();assert_eq!(restored.vertices,6..9);assert_eq!(restored.indices,6..9);assert_ne!(restored,reused);assert_eq!(a.indices(),&[0,1,2]);assert_eq!(b.indices(),&[0,1,2]);assert_eq!(c.indices(),&[0,1,2]);
 assert!(rows.retire(Rc::as_ptr(&c)as usize));assert!(rows.retire(Rc::as_ptr(&a)as usize));assert_eq!(rows.vertices.len(),120);assert_eq!(rows.append(&b).unwrap(),survivor);assert!(rows.retire(Rc::as_ptr(&b)as usize));assert!(rows.vertices.is_empty());assert!(rows.indices.is_empty());assert_eq!(rows.source_count(),0);assert_eq!(rows.append(&history).unwrap().vertices,0..3);
}

#[test]
fn empty_sources_release_after_earlier_tail_retirement(){
 for empty_first in [false,true]{let occupied=source();let empty=Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap());let mut rows=crate::arena_rows::Rows::default();let full=rows.append(&occupied).unwrap();let zero=rows.append(&empty).unwrap();assert_eq!(zero.vertices,3..3);assert_eq!(zero.indices,3..3);let keys=[Rc::as_ptr(&occupied)as usize,Rc::as_ptr(&empty)as usize];let order=if empty_first{[1,0]}else{[0,1]};for i in order{assert!(rows.retire(keys[i]));}assert!(rows.vertices.is_empty());assert!(rows.indices.is_empty());assert_eq!(rows.source_count(),0);assert_eq!(rows.append(&occupied).unwrap(),full);assert_eq!(occupied.indices(),&[0,1,2]);assert!(empty.vertices().is_empty());}
 let mut free=vec![];assert_eq!(crate::arena_spans::retire(&mut free,36..36,0),0);assert!(free.is_empty());assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||crate::arena_spans::retire(&mut free,2..1,0))).is_err());assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||crate::arena_spans::retire(&mut free,1..2,0))).is_err());assert!(free.is_empty());
}
