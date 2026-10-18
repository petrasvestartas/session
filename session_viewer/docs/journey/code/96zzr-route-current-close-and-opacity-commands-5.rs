use crate::{editor::{Editor,Action},commands::{parse,Command}};
fn command(e:&mut Editor,line:&str)->Result<crate::editor::Change,&'static str>{let Command::Editor(a)=parse(line).unwrap()else{panic!("editor command")};e.apply(a)}
#[test]
fn close_is_one_curve_transaction_and_never_releases_the_document(){
 let mut e=Editor::default();let original=e.scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let ids=e.scene.all_ids();
 assert!(command(&mut e,"Close").is_err());assert_eq!(e.scene.all_ids(),ids);
 for shape in [crate::tool::Shape::Polyline,crate::tool::Shape::Curve]{
  e.apply(Action::BeginTool(shape)).unwrap();for p in [[0.,0.,0.],[1.,0.,0.],[1.,1.,0.]]{e.apply(Action::ToolPoint(p)).unwrap();}
  command(&mut e,"Close").unwrap();assert!(e.draft.is_none());assert_eq!(e.scene.all_ids().len(),ids.len()+1);
  let added=e.scene.all_ids().difference(&ids).copied().collect::<Vec<_>>();let geometry=crate::source_edit::original(&e.scene,added[0]).unwrap();assert!(match geometry {session_rust::Geometry::Polyline(p)=>p.is_closed(),session_rust::Geometry::NurbsCurve(c)=>c.is_closed(),_=>false});
  e.apply(Action::Undo).unwrap();assert_eq!(e.scene.all_ids(),ids);assert_eq!(e.scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),original);
 }
 e.apply(Action::BeginTool(crate::tool::Shape::Polyline)).unwrap();e.apply(Action::ToolPoint([0.,0.,0.])).unwrap();let points=e.draft.as_ref().unwrap().points.clone();assert!(command(&mut e,"Close").is_err());assert_eq!(e.draft.as_ref().unwrap().points,points);assert_eq!(e.scene.all_ids(),ids);
 e.apply(Action::Plane3).unwrap();assert!(command(&mut e,"Close").is_err());assert!(e.draft.as_ref().unwrap().plane_points.is_some());assert_eq!(e.scene.all_ids(),ids);
}
#[test]
fn opacity_reads_only_the_first_finite_bounded_double(){
 let mut e=Editor::default();e.apply(Action::BeginTool(crate::tool::Shape::Polyline)).unwrap();e.apply(Action::ToolPoint([0.,0.,0.])).unwrap();let ids=e.scene.all_ids();let epoch=e.interaction_epoch;
 for text in ["Opacity 0.5 extra","Opacity 0.5 NaN","Opacity 0.5 10"]{command(&mut e,text).unwrap();assert_eq!(e.opacity,0.5);assert_eq!(e.scene.all_ids(),ids);assert_eq!(e.draft.as_ref().unwrap().points,vec![[0.,0.,0.]]);assert_eq!(e.interaction_epoch,epoch);}
 for text in ["Opacity","Opacity NaN extra","Opacity -0.000000001 extra","Opacity 1.000000001 extra"]{assert!(parse(text).is_err());assert_eq!(e.opacity,0.5);}
}
