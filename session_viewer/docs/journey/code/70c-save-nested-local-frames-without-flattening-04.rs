use crate::{scene::Scene,scene_document};
use session_rust::{proto,Xform};
use prost::Message;
use std::rc::Rc;

#[test]
fn nested_rotated_mirrored_and_legacy_placements_apply_once_and_roundtrip_precise_originals() {
    let source=crate::nested_example::message();let original=source.definitions.as_ref().unwrap().meshes[0].clone();let mut scene=Scene::empty();let document_place=Xform::translation(0.125,0.,0.);scene_document::import(&mut scene,&source.encode_to_vec(),"",&document_place,0.).unwrap();let a=&scene.objects()[0];let b=&scene.objects()[1];assert!(Rc::ptr_eq(&a.mesh,&b.mesh));assert_eq!(a.model.m[12],1_000_000_000.25);assert!((a.model.m[13]-2.25).abs()<1e-12);assert_eq!(b.model.m[12],999_999_999.25);assert_eq!(b.model.m[0],-1.);assert_eq!(a.geometry().unwrap().to_proto(),original);
    let bytes=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&bytes).unwrap();let saved=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();assert_eq!(saved.xforms.iter().find(|entry|entry.guid=="assembly").unwrap().xform.as_ref().unwrap().matrix[12],1_000_000_000.25);assert_eq!(saved.xforms.iter().find(|entry|entry.guid=="branch").unwrap().xform.as_ref().unwrap().matrix,Xform::rotation_z(90.,true).m);let mut restored=Scene::empty();scene_document::open(&mut restored,&bytes).unwrap();assert_eq!(restored.objects().len(),2);for before in scene.objects() {let guid=scene.instances[&before.id].instance.guid();let after=restored.objects().iter().find(|row|restored.instances[&row.id].instance.guid()==guid).unwrap();for (a,b) in before.model.m.iter().zip(&after.model.m) {assert!((a-b).abs()<1e-12,"Canonical placement for {guid}: {a} {b}");}assert_eq!(after.geometry().unwrap().to_proto(),original);}
}
#[test]
fn invalid_composed_placements_are_rejected_before_adoption_and_fallback_uses_document_model_once() {
    let mut source=crate::nested_example::message();source.xforms[0].xform=Some(Xform::scale_xyz(1e30,1e30,1e30).to_proto());source.xforms[1].xform=Some(Xform::scale_xyz(1e30,1e30,1e30).to_proto());let mut scene=Scene::demo();let id=scene.objects()[0].id;assert!(scene_document::import(&mut scene,&source.encode_to_vec(),"",&Xform::identity(),0.).unwrap_err().contains("Nested source placement"));assert_eq!(scene.objects()[0].id,id);
    let point=session_rust::Point::new(0.123456789012345,0.,0.);let _=point.guid();let source=proto::Session {objects:Some(proto::Objects {points:vec![point.to_proto()],..Default::default()}),..Default::default()};let mut scene=Scene::empty();scene_document::import(&mut scene,&source.encode_to_vec(),"",&Xform::translation(0.25,0.,0.),0.).unwrap();assert_eq!(scene.points()[0].model.m[12],0.25);
}
#[test]
fn shared_definition_nested_parent_move_groups_one_undo_and_close_releases_originals() {
    use crate::editor::{Action,Editor};let mut scene=Scene::empty();scene_document::import(&mut scene,&crate::nested_example::message().encode_to_vec(),"",&Xform::identity(),0.).unwrap();let original=scene.objects()[0].model.m;let weak=Rc::downgrade(scene.objects()[0].geometry().unwrap());let root=crate::hierarchy::rows(&scene)[0].ids.clone();let mut editor=Editor::default();editor.apply(Action::Close).unwrap();editor.apply(Action::Adopt {scene:Box::new(scene),replace:false}).unwrap();editor.apply(Action::SelectRows {ids:root,add:false,remove:false}).unwrap();editor.apply(Action::Translate([0.25,0.,0.])).unwrap();assert_eq!(editor.scene.objects()[0].model.m[12],original[12]+0.25);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.objects()[0].model.m,original);editor.apply(Action::Redo).unwrap();assert_eq!(editor.scene.objects()[0].model.m[12],original[12]+0.25);editor.apply(Action::Close).unwrap();assert!(weak.upgrade().is_none());
}
