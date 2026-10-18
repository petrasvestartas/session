use super::Scene;
use crate::cap_gpu_tests::{source,scene,matrix,renderer,wait,pixels};
use std::rc::Rc;
fn picked(r:&mut crate::renderer::Renderer,at:[f32;2])->Option<[u32;2]>{r.request_pick(at,matrix()).unwrap();wait(r);r.take_pick(matrix()).unwrap().unwrap()}
fn plane(scene:&mut Scene){let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],0.9).unwrap();crate::plane_original::insert(scene,Rc::new(plane)).unwrap();}
#[test]
fn actual_cap_picks_keep_full_original_u32_ids_before_any_color_frame(){let mut scene=Scene::empty();scene.next_id=16_777_217;let owner=scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();plane(&mut scene);let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some([owner.value(),0]));assert!(r.cap_usage()[0]>24);pixels(&r);assert_eq!(picked(&mut r,[0.3,0.2]),Some([owner.value(),0]));assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);}
#[test]
fn actual_cap_picks_name_the_nearest_nested_original_and_keep_locked_caps_unpickable(){let(mut scene,outer)=scene(false);let mut inner=source(false);for p in inner.vertex.values_mut(){p.x*=0.6;p.y*=0.6;p.z=0.5*p.z-0.25;}let inner=scene.insert(crate::prepared::PreparedMesh::new(inner).unwrap()).unwrap();let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some([inner.value(),0]),"nearest exit belongs to the inner original");assert_eq!(picked(&mut r,[0.5,0.2]),Some([outer.value(),0]));Rc::make_mut(&mut scene.view).lock(&[outer,inner],true);r.set_scene(&scene,None);assert_eq!(picked(&mut r,[0.3,0.2]),None);}
#[test]
fn actual_section_depth_hides_retained_lines_and_points_in_color_and_first_picks_even_when_locked(){
 let(mut scene,owner)=scene(false);let green=session_rust::Color::new(0.,1.,0.,1.);let mut line=session_rust::Line::new(-0.5,0.2,-0.7,0.5,0.2,-0.7);line.width=8.;line.linecolor=green.clone();scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();let mut point=session_rust::Point::new(-0.3,-0.2,-0.7);point.width=16.;point.pointcolor=green;scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(point)).unwrap()).unwrap();let mut r=renderer(&scene);for at in [[0.3,0.2],[-0.3,-0.2]]{assert_eq!(picked(&mut r,at),Some([owner.value(),0]));}let image=pixels(&r);for y in 70..160{for x in 100..240{let p=&image[(y*320+x)*4..][..4];assert!(!(p[1]>180&&p[0]<30&&p[2]<30),"actual ink behind a section is hidden at{x},{y}: {p:?}");}}
 Rc::make_mut(&mut scene.view).lock(&[owner],true);r.set_scene(&scene,None);for at in [[0.3,0.2],[-0.3,-0.2]]{assert_eq!(picked(&mut r,at),None,"the locked cap remains an occluder");}
}
#[test]
fn actual_coplanar_section_ink_retains_its_full_width_and_original_owner(){
 let(mut scene,_)=scene(false);let mut line=session_rust::Line::new(-0.5,0.2,-0.5,0.5,0.2,-0.5);line.width=10.;line.linecolor=session_rust::Color::new(0.,1.,0.,1.);let ink=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some(crate::pick_sources::Sources::new(&scene).unwrap().line_ids[0]));assert_eq!(scene.lines()[0].id,ink);let image=pixels(&r);for x in 180..220{let green=(88..104).filter(|y|{let p=&image[(*y*320+x)*4..][..4];p[1]>200&&p[0]<30&&p[2]<30}).count();assert!(green>=10,"full coplanar width at{x}: {green}");}
}

#[test]
fn actual_sloping_coplanar_section_keeps_full_ink_width(){
 let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::new(source(false)).unwrap()).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Normal,&[[0.,0.,-0.5],[0.2,0.,0.5]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut line=session_rust::Line::new(-0.5,0.2,-0.4,0.5,0.2,-0.6);line.width=10.;line.linecolor=session_rust::Color::new(0.,1.,0.,1.);scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap()).unwrap();let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some(crate::pick_sources::Sources::new(&scene).unwrap().line_ids[0]));let image=pixels(&r);for x in 180..220{let green=(88..104).filter(|y|{let p=&image[(*y*320+x)*4..][..4];p[1]>200&&p[0]<30&&p[2]<30}).count();assert!(green>=10,"sloping full width at{x}: {green}");}
}
#[test]
fn actual_mirrored_and_inward_original_sections_keep_pixels_and_original_pick_owners(){
 for inward in [false,true]{let mut mesh=source(false);if inward{for face in mesh.face.values_mut(){face.reverse();}}let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();if !inward{scene.place_owner(owner,session_rust::Xform::scale_xyz(-1.,1.,1.)).unwrap();}plane(&mut scene);let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some([owner.value(),0]));let image=pixels(&r);let p=crate::cap_gpu_tests::at(&image,208,96);assert!(p[..3].iter().all(|v|(150..=155).contains(v)),"inward={inward}: {p:?}");}
}
#[test]
fn actual_original_brep_section_preserves_its_bore_and_original_owner(){
 let mut scene=Scene::empty();let original=Rc::new(session_rust::BRep::create_block_with_hole(1.2,1.2,0.6,0.2));let _=original.guid();let before=original.to_proto();let owner=scene.insert_cad(original.clone()).unwrap();scene.place_owner(owner,session_rust::Xform::translation(0.,0.,-0.5)).unwrap();plane(&mut scene);let mut r=renderer(&scene);assert_eq!(picked(&mut r,[0.3,0.2]),Some([owner.value(),0]));assert_eq!(picked(&mut r,[0.08,0.08]),None,"a bore has no invented section owner");let image=pixels(&r);let hole=crate::cap_gpu_tests::at(&image,173,110);assert!(hole[..3].iter().all(|v|*v>240),"bore remains empty: {hole:?}");let solid=crate::cap_gpu_tests::at(&image,208,96);assert!(solid[..3].iter().all(|v|(150..=155).contains(v)),"actual closed BRep section: {solid:?}");assert_eq!(original.to_proto(),before);
}

#[test]
fn actual_two_original_planes_cap_their_shared_corner_and_remove_cut_owners(){
 let(mut scene,owner)=scene(false);let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Normal,&[[0.,0.,-0.5],[1.,0.,0.5]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut r=renderer(&scene);for point in [[-0.3,0.2],[0.2,0.2]]{assert_eq!(picked(&mut r,point),Some([owner.value(),0]));}assert_eq!(picked(&mut r,[0.5,0.2]),None);let image=pixels(&r);for x in [112,192]{let p=crate::cap_gpu_tests::at(&image,x,96);assert!(p[..3].iter().all(|v|(150..=155).contains(v)),"two actual caps at{x}: {p:?}");}let p=crate::cap_gpu_tests::at(&image,240,96);assert!(p[..3].iter().all(|v|*v>240),"cut-away corner: {p:?}");
}
