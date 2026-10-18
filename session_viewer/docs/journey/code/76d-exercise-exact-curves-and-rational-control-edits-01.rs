use crate::{editor::{Action,Editor},chain::Source};
use session_rust::Xform;
use std::rc::Rc;
fn editor()->Editor {let mut e=Editor::default();e.apply(Action::Close).unwrap();e}
fn command(e:&mut Editor,line:&str)->Result<(),String> {let action=if let Some(a)=e.tool_line(line){a.map_err(str::to_owned)?}else{let crate::commands::Command::Editor(a)=crate::commands::parse(line)? else{return Err("Editor command".into())};a};e.apply(action).map_err(|message|if message=="Shape input invalid"{e.tool_status.clone()}else{message.into()})?;Ok(())}
fn curve(e:&Editor,index:usize)->Rc<session_rust::NurbsCurve> {let Source::Curve(source,_)=&e.scene.paths()[index].prepared.source else{panic!("Original curve")};Rc::clone(source)}
#[test]
fn current_curve_families_preserve_original_controls_knots_weights_and_archive_identity() {
    let mut e=editor();for line in ["Nurbs Curve Circle w0,0,0 1","Nurbs Curve Ellipse w3,0,0 2 1","Nurbs Curve Arc Center w6,0,0 1 270","Nurbs Curve Arc 3 Points w8,0,0 w10,0,0 w9,1,0","Nurbs Curve Parabola w12,0,0 2 1","Curve w15,0,0 w16,1,0 w17,0,0"] {let count=e.scene.paths().len();command(&mut e,line).unwrap();assert!(e.draft.is_none());command(&mut e,"Undo").unwrap();assert_eq!(e.scene.paths().len(),count);command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),count+1);}
    let originals=(0..6).map(|i|curve(&e,i)).collect::<Vec<_>>();let archive=crate::scene_archive::save(&e.scene).unwrap();e.apply(Action::Close).unwrap();crate::scene_document::open(&mut e.scene,&archive).unwrap();for source in originals {let saved=e.scene.paths().iter().find_map(|row|match &row.prepared.source{Source::Curve(c,_) if c.guid()==source.guid()=>Some(c),_=>None}).unwrap();assert_eq!(saved.m_cv,source.m_cv);assert_eq!(saved.m_nurbsknot,source.m_nurbsknot);assert_eq!(saved.m_is_rat,source.m_is_rat);}
}
#[test]
fn closed_curve_replays_world_points_once_and_keeps_an_original_smooth_curve() {
    let mut e=editor();e.apply(Action::ConstructionPlane(Some(crate::cplane::CPlane::Xz))).unwrap();command(&mut e,"Curve").unwrap();command(&mut e,"0,0,0 2,0,0 2,2,0 0,2,0").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Close").unwrap();let source=curve(&e,0);assert!(source.is_closed());for i in 0..source.cv_count(){assert!(source.get_cv(i).unwrap()[1].abs()<1e-12);}assert!(source.point_at_start().distance(&source.point_at_end(),None)<1e-9);command(&mut e,"Undo").unwrap();assert!(e.scene.paths().is_empty());command(&mut e,"Redo").unwrap();assert!(Rc::ptr_eq(&curve(&e,0),&source));
}
#[test]
fn failed_exact_curve_input_keeps_answers_previews_and_redo_without_scene_mutation() {
    let mut e=editor();command(&mut e,"Curve w0,0,0 w1,1,0 w2,0,0").unwrap();command(&mut e,"Undo").unwrap();command(&mut e,"Nurbs Curve Arc Center").unwrap();command(&mut e,"w0,0,0 1").unwrap();assert!(command(&mut e,"360").is_err());assert_eq!(e.draft.as_ref().unwrap().primitive.as_ref().unwrap().answers.len(),2);assert_eq!(e.draft.as_ref().unwrap().outlines().len(),1);assert!(e.scene.paths().is_empty());command(&mut e,"Escape").unwrap();command(&mut e,"Redo").unwrap();assert_eq!(e.scene.paths().len(),1);
    command(&mut e,"Curve").unwrap();command(&mut e,"w0,0,0").unwrap();assert!(command(&mut e,"Nurbs Curve Parabola w0,0,0 2 0").is_err());assert_eq!(e.draft.as_ref().unwrap().tool.name(),"Curve");assert_eq!(e.draft.as_ref().unwrap().points.len(),1);
}
#[test]
fn editing_a_created_rational_arc_preserves_weights_knots_other_controls_and_undo() {
    let mut e=editor();command(&mut e,"Nurbs Curve Arc Center w0,0,0 1 90").unwrap();let source=curve(&e,0);assert!(source.m_is_rat);e.apply(Action::Controls(true)).unwrap();let id=e.scene.paths()[0].id;let target=crate::pick_sources::Target{owner:id,part:crate::pick_sources::Part::Control(1)};let mut preview=crate::part_preview::Preview::new(&e.scene,target).unwrap();preview.set(Xform::translation(0.125,0.25,0.)).unwrap();let Source::Curve(edited,_)=&preview.scene.paths()[0].prepared.source else{panic!("Original curve")};assert_eq!(edited.m_nurbsknot,source.m_nurbsknot);for i in 0..source.cv_count(){assert_eq!(edited.get_cv_4d(i).unwrap().3,source.get_cv_4d(i).unwrap().3);let expected=if i==1{&source.get_cv(i).unwrap()+&session_rust::Vector::new(0.125,0.25,0.)}else{source.get_cv(i).unwrap()};assert!(edited.get_cv(i).unwrap().distance(&expected,None)<1e-12);}assert!(Rc::ptr_eq(&curve(&e,0),&source));let mut history=crate::history::History::default();assert!(preview.commit(&mut e.scene,&mut history).unwrap());assert!(history.undo(&mut e.scene));assert!(Rc::ptr_eq(&curve(&e,0),&source));assert!(history.redo(&mut e.scene));assert!(!Rc::ptr_eq(&curve(&e,0),&source));
}
