use crate::{editor::{Editor,Action},origin::Origin,reload_url::ReloadUrl};
use prost::Message;use session_rust::{proto,Geometry};use std::rc::Rc;
#[test]
fn all_editor_byte_actions_accept_full_elements_and_keep_one_undo_and_source_location(){
 let bytes=crate::interaction_example::bytes();let message=proto::Session::decode(bytes.as_slice()).unwrap();
 for (replace,located) in [(false,false),(true,false),(false,true),(true,true)]{
  let mut e=Editor::default();let before=e.scene.all_ids();let url=Rc::new(ReloadUrl::new("test:editor-full".into()));
  let action=match (replace,located){(false,false)=>Action::Import(bytes.clone()),(true,false)=>Action::Replace(bytes.clone()),(false,true)=>Action::ImportAt(bytes.clone(),url.clone()),(true,true)=>Action::ReplaceAt(bytes.clone(),url.clone())};
  e.apply(action).unwrap();let after=e.scene.all_ids();assert_ne!(after,before);assert_eq!(before.is_subset(&after),!replace);
  let guids=e.scene.objects().iter().filter_map(|row|match row.original.as_ref(){Some(Geometry::Element(element))=>Some(element.guid().to_owned()),_=>None}).collect::<std::collections::HashSet<_>>();for element in &message.objects.as_ref().unwrap().elements{assert!(guids.contains(&element.guid));}
  let origin=e.scene.forest.documents.last().unwrap().header.as_ref().unwrap();assert_eq!(origin.header.interactions,message.interactions);assert_eq!(origin.location.is_some(),located);if located{assert!(Rc::ptr_eq(origin.location.as_ref().unwrap(),&url));}
  e.apply(Action::Undo).unwrap();assert_eq!(e.scene.all_ids(),before);e.apply(Action::Redo).unwrap();assert_eq!(e.scene.all_ids(),after);
  assert!(e.apply(Action::Replace(vec![255])).is_err());assert_eq!(e.scene.all_ids(),after);e.apply(Action::Undo).unwrap();assert_eq!(e.scene.all_ids(),before);e.apply(Action::Redo).unwrap();assert_eq!(e.scene.all_ids(),after);
 }
}
#[test]
fn released_mesh_reload_accepts_its_full_mixed_original_file_and_rejects_changed_bytes(){
 let mut message=proto::Session::decode(crate::specimen::bytes().as_slice()).unwrap();let point=session_rust::Point::new(2.123456789,3.,4.);let _=point.guid();let original=point.to_proto();let guid=original.guid.clone();message.objects.as_mut().unwrap().points.push(original.clone());message.tree.as_mut().unwrap().root.as_mut().unwrap().children.push(proto::TreeNode{name:guid,..Default::default()});let bytes=message.encode_to_vec();let mut origin=Origin::new(&message,&bytes);origin.location=Some(Rc::new(ReloadUrl::new("test:mixed-reload".into())));
 let source=crate::document::restore(&bytes,&origin).unwrap();assert_eq!(source.objects.points.len(),1);assert_eq!(source.objects.points[0].to_proto(),original);assert_eq!(source.objects.meshes.len(),message.objects.as_ref().unwrap().meshes.len());for mesh in &source.objects.meshes {assert!(message.objects.as_ref().unwrap().meshes.contains(&mesh.to_proto()));}
 assert_eq!(crate::document::restore(&[255],&origin).err().unwrap(),"Reloaded file version changed");
}
