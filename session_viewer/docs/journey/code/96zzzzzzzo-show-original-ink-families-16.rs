use super::*;
use crate::{cap_gpu_tests::{renderer,pixels,wait,matrix},ink_ranges::Visibility,pick_sources::{Mode,Part,Target}};
use session_rust::{Mesh,Point,Vector,Xform,OBB};use std::rc::Rc;
fn query(r:&mut Renderer,scene:&Scene,mode:Mode)->Option<Target>{r.set_pick_mode(mode);r.request_pick([0.,0.2],matrix()).unwrap();wait(r);r.take_pick(matrix()).unwrap().unwrap().and_then(|v|r.resolve_pick(scene,v))}
fn original_documents(scene:&Scene)->Vec<Vec<u8>>{crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap()).unwrap().documents}
#[test]
fn actual_original_frames_keep_object_only_queries_and_reveal_the_real_edge_behind(){
 for kind in ["plane","obb"] {
  let mut scene=Scene::empty();let mut mesh=Mesh::from_vertices_and_faces(vec![Point::new(-0.6,0.2,-0.8),Point::new(0.6,0.2,-0.8),Point::new(0.,-0.6,-0.8)],vec![vec![0,1,2]]);mesh.set_linecolors(vec![],vec![6.]);let solid=scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();
  let frame=if kind=="plane" {let id=crate::plane_original::insert(&mut scene,Rc::new(session_rust::Plane::xy_plane())).unwrap();scene.place(id,&Xform::translation(0.,0.,-0.3)*&Xform::scale_xyz(0.2,0.2,0.2)).unwrap();id}else{crate::bounds_original::insert(&mut scene,Rc::new(OBB::new(Point::new(0.,0.,-0.35),Vector::x_axis(),Vector::y_axis(),Vector::z_axis(),Vector::new(0.2,0.2,0.05)))).unwrap()};
  let original=original_documents(&scene);let prepared=scene.objects().iter().map(|r|r.mesh.clone()).collect::<Vec<_>>();let mut r=renderer(&scene);r.set_opacity(1.);r.set_lighting(false,false);assert!(!r.sources.edges.iter().any(|t|t.owner==frame),"An original frame must not invent proxy MeshEdge subparts");
  for samples in [1,4] {r.set_samples(samples);r.set_ink_visibility(Visibility::default());let all=pixels(&r);let usage=(r.stats()[0],r.uploaded_bytes(),r.part_usage(),r.point_usage(),r.stroke_usage());assert_eq!(query(&mut r,&scene,Mode::Object),Some(Target{owner:frame,part:Part::Object}));assert_eq!(query(&mut r,&scene,Mode::Edge),Some(Target{owner:solid,part:Part::MeshEdge(0,1)}),"The real source edge behind an object-only frame must remain queryable");
   r.set_ink_visibility(Visibility{edges:false,..Default::default()});pixels(&r);assert_eq!(query(&mut r,&scene,Mode::Edge),None);assert_eq!(query(&mut r,&scene,Mode::Object),Some(Target{owner:frame,part:Part::Object}));
   r.set_ink_visibility(Visibility::default());r.set_pick_mode(Mode::Object);r.request_pick([0.,0.2],matrix()).unwrap();r.set_ink_visibility(Visibility{lines:false,..Default::default()});wait(&r);assert!(r.take_pick(matrix()).is_none(),"A visibility change cancels the pending old frame hit");let hidden=pixels(&r);assert!(hidden!=all);assert_eq!(query(&mut r,&scene,Mode::Object),Some(Target{owner:solid,part:Part::Object}));assert_eq!(query(&mut r,&scene,Mode::Edge),Some(Target{owner:solid,part:Part::MeshEdge(0,1)}));
   r.set_ink_visibility(Visibility::default());assert!(pixels(&r)==all);assert_eq!((r.stats()[0],r.uploaded_bytes(),r.part_usage(),r.point_usage(),r.stroke_usage()),usage,"Family changes retain original geometry, IDs, buffers and source uploads");assert_eq!(original_documents(&scene),original);for(before,row)in prepared.iter().zip(scene.objects()){assert!(Rc::ptr_eq(before,&row.mesh));}
  }
 }
}
