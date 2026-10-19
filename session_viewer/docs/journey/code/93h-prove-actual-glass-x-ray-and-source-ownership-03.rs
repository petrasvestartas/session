use crate::{editor::{Editor,Action},commands::{parse,Command}};
#[test]
fn opacity_strict_presentation_keeps_original_pending_points_and_redo(){
 let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();let id=e.scene.objects()[0].id;e.select_objects([id],false);let before=e.scene.objects()[0].geometry().unwrap().to_proto();e.apply(Action::Translate([1.,0.,0.])).unwrap();let moved=e.scene.placement(id).unwrap();e.apply(Action::Undo).unwrap();e.apply(Action::BeginTool(crate::tool::Shape::Line)).unwrap();e.apply(Action::ToolPoint([0.,0.,0.])).unwrap();let points=e.draft.as_ref().unwrap().points.clone();let epoch=e.interaction_epoch;
 for value in [0.,0.4,0.7,1.]{let Command::Editor(a)=parse(&format!("Opacity {value}")).unwrap()else{unreachable!()};e.apply(a).unwrap();assert_eq!(e.opacity,value);assert_eq!(e.draft.as_ref().unwrap().points,points);assert_eq!(e.interaction_epoch,epoch);assert_eq!(e.scene.objects()[0].geometry().unwrap().to_proto(),before);}
 for text in ["Opacity","Opacity -1","Opacity 1.1","Opacity NaN","Opacity 0.5 extra"]{assert!(parse(text).is_err());}assert!(e.apply(Action::Opacity(f32::NAN)).is_err());e.apply(Action::CancelTool).unwrap();e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,moved.m);
}
