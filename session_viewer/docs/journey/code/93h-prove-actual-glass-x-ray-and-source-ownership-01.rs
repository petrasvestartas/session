use crate::{scene::Scene,cap_gpu_tests::{renderer,pixels,matrix,wait,at}};
use std::rc::Rc;
fn pick(r:&mut super::Renderer,at:[f32;2])->Option<[u32;2]>{r.request_pick(at,matrix()).unwrap();wait(r);r.take_pick(matrix()).expect("mapped original opacity pick").unwrap()}
#[test]
fn actual_closed_faces_blend_hidden_ink_fades_and_xray_picks_original_ink(){
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let mut source=session_rust::Line::new(-0.4,0.,-0.9,0.4,0.,-0.9);source.linecolor=session_rust::Color::new(0.,1.,0.,1.);source.width=8.;let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(source)).unwrap()).unwrap();let mut r=renderer(&scene);let solid=pixels(&r);let uploads=r.stats()[0];assert!(at(&solid,160,120)[0]>100&&at(&solid,160,120)[1]<20);assert_eq!(pick(&mut r,[0.,0.]).unwrap()[0],owner.value());
 r.set_opacity(0.5);let glass=pixels(&r);let center=at(&glass,160,120);assert!(center[1]>100,"hidden original line fades through glass: {center:?}");let face=at(&glass,160,90);assert!((120..=135).contains(&face[1])&&(120..=135).contains(&face[2]),"one blended face, without a second back face: {face:?}");assert_eq!(pick(&mut r,[0.,0.]).unwrap()[0],line.value());
 r.set_opacity(0.);let xray=pixels(&r);let center=at(&xray,160,120);assert!(center[1]>240&&center[0]<20,"xray original ink: {center:?}");assert_eq!(&at(&xray,160,90)[..3],&[255,255,255]);assert_eq!(pick(&mut r,[0.,0.]).unwrap()[0],line.value());assert_eq!(pick(&mut r,[0.,0.25]),None,"closed xray faces do not take an original face pick");
 r.set_opacity(1.);assert_eq!(pixels(&r),solid);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(r.stats()[0],uploads);
}
#[test]
fn actual_open_surfaces_remain_opaque_and_mirrored_closed_faces_keep_glass(){
 let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(true)).unwrap()).unwrap();let mut r=renderer(&scene);let solid=pixels(&r);r.set_opacity(0.5);assert_eq!(pixels(&r),solid);r.set_opacity(0.);assert_eq!(pixels(&r),solid,"open surfaces retain original opaque faces");
 let mut scene=Scene::empty();let owner=scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut r=renderer(&scene);r.set_opacity(0.5);let plain=pixels(&r);scene.place_owner(owner,session_rust::Xform::scale_xyz(-1.,1.,1.)).unwrap();r.set_scene(&scene,None);let mirror=pixels(&r);assert_eq!(at(&mirror,160,90),at(&plain,160,90),"mirrored original front face stays blended");
}
#[test]
fn actual_xray_survives_scene_refresh_without_hiding_open_originals(){
 let mut scene=Scene::empty();scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut r=renderer(&scene);r.set_opacity(0.);pixels(&r);r.set_scene(&scene,None);let xray=pixels(&r);assert_eq!(&at(&xray,160,90)[..3],&[255,255,255]);
 scene.clear();scene.insert(crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(true)).unwrap()).unwrap();r.set_scene(&scene,None);let open=pixels(&r);assert!(at(&open,160,90)[0]>100&&at(&open,160,90)[1]<20,"a new open original stays opaque under xray");
}
