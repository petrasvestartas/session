use std::rc::Rc;
#[test]
fn full_element_reader_measures_actual_phases_and_preserves_location(){
 let bytes=crate::interaction_example::bytes();let location=Rc::new(crate::reload_url::ReloadUrl::new("test:full-phases".into()));let mut time=0.;let mut phases=Vec::new();
 let source=crate::document_source::prepare_observed(&bytes,&session_rust::Xform::identity(),0.,Some(location.clone()),&mut ||{time+=1.;time},&mut |p|phases.push((p.name.to_owned(),p.duration_ms,p.elapsed_ms,p.bytes,p.succeeded))).unwrap();
 assert_eq!(phases.iter().map(|p|p.0.as_str()).collect::<Vec<_>>(),["decode","validation","kernel"]);for(i,p)in phases.iter().enumerate(){assert_eq!(p.1,1.);assert_eq!(p.2,(i*2+2)as f64);assert_eq!(p.3,bytes.len()as u64);assert!(p.4);}
 assert_eq!(source.document.objects.elements.len(),3);assert!(!source.document.interactions.is_empty());assert!(Rc::ptr_eq(source.origin.location.as_ref().unwrap(),&location));
}
#[test]
fn malformed_full_source_records_only_failed_decode(){
 let mut time=0.;let mut phases=Vec::new();let result=crate::document_source::prepare_observed(&[255],&session_rust::Xform::identity(),0.,None,&mut ||{time+=1.;time},&mut |p|phases.push((p.name.to_owned(),p.bytes,p.succeeded)));
 assert_eq!(result.err().unwrap(),"Invalid session protobuf");assert_eq!(phases,vec![("decode".into(),1,false)]);
}
