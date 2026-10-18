use crate::{editor::{Editor,Action},commands::{Command,parse},clipping_geometry::{Mode,clip_plane},clipping_rows};
fn command(e:&mut Editor,line:&str)->Result<crate::editor::Change,&'static str>{let action=if let Some(result)=e.tool_line(line){result?}else{let Command::Editor(action)=parse(line).map_err(|_|"Invalid command")?else{return Err("Editor command")};action};e.apply(action)}
#[test]
fn original_plane_creation_flip_save_and_one_undo_keep_exact_identity(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();command(&mut e,"Clipping Plane XY 0,0,3").unwrap();let id=e.selected.unwrap();let source=e.scene.objects()[0].original.clone().unwrap();let session_rust::Geometry::Plane(source)=source else{panic!("original Plane")};let guid=source.guid().to_owned();let before=source.to_proto();assert_eq!(clipping_rows::current(&e.scene).unwrap()[0].plane.distance([0.,0.,2.]),1.);
 command(&mut e,"Clipping Plane Flip").unwrap();assert_eq!(clipping_rows::current(&e.scene).unwrap()[0].plane.distance([0.,0.,2.]),-1.);e.apply(Action::Undo).unwrap();assert_eq!(e.selected,Some(id));let Some(session_rust::Geometry::Plane(restored))=&e.scene.objects()[0].original else{panic!("original Plane")};assert_eq!(restored.guid(),guid);assert_eq!(restored.to_proto(),before);
 let saved=crate::scene_archive::save(&e.scene).unwrap();assert!(!saved.is_empty());assert_eq!(source.to_proto(),before);
}
#[test]
fn switches_fills_views_and_invalid_answers_preserve_pending_original_points(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();command(&mut e,"Clipping Plane Normal").unwrap();command(&mut e,"w0,0,0").unwrap();let points=e.draft.as_ref().unwrap().points.clone();for line in ["Clipping Plane Off","Clipping Plane Fill Hatch","View Top"]{command(&mut e,line).unwrap();assert_eq!(e.draft.as_ref().unwrap().points,points);}assert!(!e.clipping_enabled&&!e.clipping_solid);assert!(command(&mut e,"w0,0,0").is_err());assert_eq!(e.draft.as_ref().unwrap().points,points);command(&mut e,"w0,0,1").unwrap();assert_eq!(clipping_rows::owners(&e.scene).len(),1);assert!(e.draft.is_none());let plane=crate::clipping_geometry::plane_from(Mode::Normal,&[[0.,0.,0.],[0.,0.,1.]],500.).unwrap();assert_eq!(clipping_rows::current(&e.scene).unwrap()[0].plane,clip_plane(&plane,&session_rust::Xform::identity()).unwrap());
}
