use crate::{editor::{Action,Editor},cplane::CPlane};
use session_rust::Point;
fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,line:&str)->Result<(),String> {let action=if let Some(a)=e.tool_line(line) {a.map_err(str::to_owned)?}else{let crate::commands::Command::Editor(a)=crate::commands::parse(line)? else{return Err("Editor command".into())};a};e.apply(action).map_err(|message|if message=="Shape input invalid"{e.tool_status.clone()}else{message.into()})?;Ok(())}
#[test]
fn all_round_families_create_current_originals_and_persist_with_one_transaction() {
    let mut e=editor();for (line,cad) in [("Cylinder 0,0,0 0.4 1",true),("Cone 2,0,0 0.4 1",true),("Sphere 4,0,0 0.5",true),("Torus 6,0,0 0.6 0.2",true),("Capsule 8,0,0 0.3 1.5",false),("Quad Sphere 10,0,0 0.5",false)] {let count=e.scene.objects().len();command(&mut e,line).unwrap();assert!(e.draft.is_none());assert_eq!(e.scene.objects().last().unwrap().cad.is_some(),cad);command(&mut e,"Undo").unwrap();assert_eq!(e.scene.objects().len(),count);command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),count+1);}
    let snapshot=crate::scene_archive::save(&e.scene).unwrap();let guids=e.scene.objects().iter().map(|row|e.scene.row_guid(row.id).unwrap().to_owned()).collect::<Vec<_>>();e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&snapshot).unwrap();assert_eq!(e.scene.objects().len(),6);for guid in guids {assert!(e.scene.objects().iter().any(|row|e.scene.row_guid(row.id).as_deref()==Some(guid.as_str())));}
}
#[test]
fn round_options_reject_invalid_dimensions_without_losing_the_active_tool_or_redo() {
    let mut e=editor();command(&mut e,"Sphere 0,0,0 1").unwrap();command(&mut e,"Undo").unwrap();command(&mut e,"Torus").unwrap();command(&mut e,"w0,0,0 2").unwrap();assert!(command(&mut e,"3").is_err());assert_eq!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.len(),2);command(&mut e,"0.5").unwrap();assert_eq!(e.scene.objects().len(),1);command(&mut e,"Undo").unwrap();
    for line in ["Cylinder 0,0,0 1 0","Cone 0,0,0 -1 2","Capsule 0,0,0 1 2","Sphere 0,0,0 0","Quad Sphere Brep 0,0,0 1","Capsule Brep 0,0,0 1 3"] {assert!(command(&mut e,line).is_err(),"{line}");assert!(e.draft.is_none());assert!(e.scene.objects().is_empty());}command(&mut e,"Redo").unwrap();assert_eq!(e.scene.objects().len(),1);
}
#[test]
fn tilted_round_frames_and_negative_heights_keep_original_geometry_and_temporary_wires() {
    let mut e=editor();let plane=CPlane::from_3_points(&Point::new(4.,5.,6.),&Point::new(5.,5.,7.),&Point::new(4.,6.,6.)).unwrap();e.apply(Action::ConstructionPlane(Some(plane))).unwrap();command(&mut e,"Cylinder").unwrap();command(&mut e,"0,0,0 0.5").unwrap();assert_eq!(e.draft.as_ref().unwrap().outlines().len(),1);assert!(e.scene.objects().is_empty());e.apply(Action::ToolHover(Some([4.,5.,7.]))).unwrap();assert!(e.draft.as_ref().unwrap().outlines().len()>=1);command(&mut e,"-2").unwrap();let brep=&e.scene.objects()[0].cad.as_ref().unwrap().source;let points=brep.vertex_points();let heights=points.iter().map(|p|plane.local(p)[2]).collect::<Vec<_>>();assert!(heights.iter().any(|v|v.abs()<1e-12));assert!(heights.iter().any(|v|(*v+2.).abs()<1e-12));
}
