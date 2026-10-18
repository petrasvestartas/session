use session_rust::{proto,Xform};
use prost::Message;

#[test]
fn component_json_and_unknown_interaction_bytes_survive_archives(){
    let mut message=proto::Session::decode(crate::comparison::specimen().as_slice()).unwrap();
    let component=proto::Component{type_name:"FloorBuilder".into(),guid:"component-original".into(),name:"guide".into(),json_data:"{ \"origin\": [1000000000.125, 0, 0], \"custom\": true }".into()};
    let interactions=vec![proto::InteractionEntry{guid:"original-edge".into(),interactions:vec![proto::Interaction{guid:"original-interaction".into(),name:"glue".into(),interaction_type:"custom-glue".into(),interaction_data:vec![0,255,1,2,3]}]}];
    message.objects.as_mut().unwrap().components.push(component.clone());message.tree.as_mut().unwrap().root.as_mut().unwrap().children.push(proto::TreeNode{name:component.guid.clone(),..Default::default()});message.interactions=interactions.clone();if let Ok(path)=std::env::var("VIEWER_COURSE_METADATA_OUTPUT"){std::fs::write(path,message.encode_to_vec()).unwrap();}
    let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&message.encode_to_vec(),"original",&Xform::identity(),0.).unwrap();assert_eq!(scene.objects().len(),4);
    for _ in 0..2{let bytes=crate::scene_archive::save(&scene).unwrap();let bundle=crate::archive::Bundle::decode(&bytes).unwrap();let saved=proto::Session::decode(bundle.documents[0].as_slice()).unwrap();assert_eq!(saved.objects.unwrap().components,vec![component.clone()]);assert_eq!(saved.interactions,interactions);crate::scene_document::open(&mut scene,&bytes).unwrap();}
}

#[test]
fn unused_original_definition_stays_original_without_a_display_row(){
    let mut message=proto::Session::decode(crate::comparison::specimen().as_slice()).unwrap();let mut original=session_rust::BRep::create_cylinder(25.,100.).to_proto();original.guid="unused-original".into();message.definitions=Some(proto::Objects{breps:vec![original.clone()],..Default::default()});
    let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&message.encode_to_vec(),"unused",&Xform::identity(),0.).unwrap();assert_eq!(scene.objects().len(),4);
    let saved=crate::archive::Bundle::decode(&crate::scene_archive::save(&scene).unwrap()).unwrap();let source=proto::Session::decode(saved.documents[0].as_slice()).unwrap();assert_eq!(source.definitions.unwrap().breps,vec![original]);
}

#[test]
fn raw_plane_nonfinite_and_short_frames_are_rejected_before_kernel_defaults(){
    let original=session_rust::Plane::xy_plane().to_proto();for index in 0..12{let mut p=original.clone();p.frame[index]=f64::NAN;let message=proto::Session{objects:Some(proto::Objects{planes:vec![p],..Default::default()}),..Default::default()};assert!(crate::document_source::prepare(&message.encode_to_vec(),&Xform::identity(),0.).is_err());}
    let mut p=original.clone();p.frame.truncate(3);assert!(crate::source_metadata::raw_plane(&p).is_err());crate::source_metadata::raw_plane(&original).unwrap();
}
#[test]
fn element_instances_keep_typed_definitions_and_instance_features(){
    use wood::WoodElement;
    let axis=session_rust::Polyline::new(vec![session_rust::Point::new(0.,0.,0.),session_rust::Point::new(1000.,0.,0.)]);
    let element=crate::element_beam::build(&[axis.clone()],100.,200.).unwrap()[0].to_element();let _=element.guid();let source=element.to_proto();
    let feature=proto::ElementFeature{guid:"instance-feature".into(),name:"cut".into(),feature_type:"cut".into(),face_index:-1,outlines:vec![axis.to_proto()],..Default::default()};
    let instance=proto::InstanceRef{guid:"element-instance".into(),name:"beam instance".into(),definition_guid:source.guid.clone(),features:vec![feature.clone()],..Default::default()};
    let message=proto::Session{objects:Some(proto::Objects{instances:vec![instance],..Default::default()}),definitions:Some(proto::Objects{elements:vec![source.clone()],..Default::default()}),..Default::default()};
    let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&message.encode_to_vec(),"element instance",&Xform::identity(),0.).unwrap();assert_eq!(scene.instances.len(),1);
    let saved=crate::archive::Bundle::decode(&crate::scene_archive::save(&scene).unwrap()).unwrap();let source_again=proto::Session::decode(saved.documents[0].as_slice()).unwrap();assert_eq!(source_again.definitions.unwrap().elements,vec![source]);assert_eq!(source_again.objects.unwrap().instances[0].features,vec![feature]);
}
#[test]
fn merging_original_definitions_rejects_conflicting_payloads(){
    let mut mesh=session_rust::Mesh::from_vertices_and_faces(vec![session_rust::Point::new(0.,0.,0.)],vec![]).to_proto();mesh.guid="same".into();let unused=proto::Objects{meshes:vec![mesh.clone()],..Default::default()};let mut other=mesh;other.name="different source".into();assert!(crate::definition_merge::merge(Some(&unused),Some(proto::Objects{meshes:vec![other],..Default::default()})).is_err());
}
