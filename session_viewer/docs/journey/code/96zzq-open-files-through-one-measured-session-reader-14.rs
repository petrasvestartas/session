t(),1);assert_eq!(editor.camera.uniform(),camera);assert_eq!(editor.scene.clouds().len(),1);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.clouds().len(),1);
    }

}

#[test]
fn full_element_file_preparation_retains_originals_interactions_and_shared_reload_origin(){
 use session_rust::Geometry;use prost::Message;let scene=Scene::demo();let before=scene.all_ids();let bytes=crate::interaction_example::bytes();
 let location=Rc::new(crate::reload_url::ReloadUrl::new("test:full-file".into()));let ready=prepare(&scene,&bytes,true,Some(location.clone())).unwrap();
 let originals=ready.objects().iter().filter_map(|row|match row.original.as_ref(){Some(Geometry::Element(element))=>Some(element.guid().to_owned()),_=>None}).collect::<std::collections::HashSet<_>>();
 let message=session_rust::proto::Session::decode(bytes.as_slice()).unwrap();for element in &message.objects.as_ref().unwrap().elements {assert!(originals.contains(&element.guid));}
 let origin=ready.forest.documents[0].header.as_ref().unwrap();assert!(Rc::ptr_eq(origin.location.as_ref().unwrap(),&location));assert!(!origin.header.interactions.is_empty());
 assert_eq!(scene.all_ids(),before);assert_eq!(prepare(&scene,&[255],true,None).err().unwrap(),"Invalid session protobuf");assert_eq!(scene.all_ids(),before);
}
