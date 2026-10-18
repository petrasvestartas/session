use crate::{editor::{Editor,Action},commands::{parse,Command}};
fn command(e:&mut Editor,line:&str)->Result<crate::editor::Change,&'static str>{let Command::Editor(a)=parse(line).map_err(|_|"Invalid command")?else{return Err("Editor command")};e.apply(a)}
#[test]
fn outline_and_arctic_strict_presentation_preserves_tool_history_and_originals(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();let id=e.scene.objects()[0].id;let original=e.scene.objects()[0].geometry().unwrap().to_proto();e.select_objects([id],false);e.apply(Action::Translate([1.,0.,0.])).unwrap();let moved=e.scene.placement(id).unwrap();e.apply(Action::Undo).unwrap();command(&mut e,"Line").unwrap();e.apply(Action::ToolPoint([0.,0.,0.])).unwrap();let points=e.draft.as_ref().unwrap().points.clone();let epoch=e.interaction_epoch;
 for line in ["vIeW oUtLiNe On","View Outline Off","View Outline","Arctic On","Arctic Off"]{command(&mut e,line).unwrap();assert_eq!(e.draft.as_ref().unwrap().points,points);assert_eq!(e.interaction_epoch,epoch);assert_eq!(e.scene.objects()[0].geometry().unwrap().to_proto(),original);}
 assert!(e.outlines,"Arctic Off retains the independent Outline setting");command(&mut e,"View Outline Off").unwrap();assert!(!e.outlines);command(&mut e,"Arctic On").unwrap();assert!(e.outlines);for line in ["View Outline No","View Outline On Off","View Outline On extra"]{assert!(command(&mut e,line).is_err());assert!(e.outlines);}
 e.apply(Action::CancelTool).unwrap();e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,moved.m);
}
