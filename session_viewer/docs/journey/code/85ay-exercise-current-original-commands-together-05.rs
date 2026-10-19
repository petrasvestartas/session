use crate::{editor::{Editor,Action},edit_intent::Intent,reload_reply::Reply};
use std::rc::Rc;
fn cold()->(Editor,Vec<u8>){let mut e=Editor::default();let bytes=crate::specimen::bytes();e.apply(Action::ReplaceAt(bytes.clone(),Rc::new(crate::reload_url::ReloadUrl::new("test:command-source".into())))).unwrap();e.apply(Action::SelectAll).unwrap();e.apply(Action::UnloadSources).unwrap();(e,bytes)}
#[test]
fn cold_multi_object_commands_load_each_origin_once_and_replay_after_orbit(){
    let(mut e,bytes)=cold();let ids=e.selected_objects();let action=Action::BoundingBox;let intent=Intent::capture_for(&action,&e).unwrap().unwrap();let keys=intent.keys(&e).unwrap();assert_eq!(keys.len(),1);assert!(e.scene.objects().iter().all(|r|r.geometry().is_none()));let n=e.scenes().count();e.apply(Action::CanvasCancel).unwrap();e.apply(Action::Touch(vec![Action::Orbit(0.2,0.1)])).unwrap();let camera=e.camera.uniform();
    let result=Reply::new(keys,Some(intent),Ok(vec![bytes])).complete(&mut e).unwrap().unwrap();assert_eq!(result,crate::edit_replay::Reply::Changed(crate::editor::Change::Scene));assert_eq!(e.camera.uniform(),camera);assert_eq!(e.scenes().count(),n+1);assert_eq!(e.scene.objects().len(),ids.len()+1);assert_eq!(e.scene.objects().iter().filter(|r|matches!(r.original,Some(session_rust::Geometry::OBB(_)))).count(),1);e.apply(Action::Undo).unwrap();assert_eq!(e.scene.objects().len(),ids.len());e.apply(Action::Redo).unwrap();assert_eq!(e.scene.objects().len(),ids.len()+1);
}
#[test]
fn source_backed_face_measurement_replays_its_part_without_history(){
    let(mut e,bytes)=cold();let id=e.scene.objects()[0].id;e.apply(Action::PickedTarget(Some(crate::pick_sources::Target{owner:id,part:crate::pick_sources::Part::Face(0)}))).unwrap();let n=e.scenes().count();let intent=Intent::capture_for(&Action::Measure(crate::measure_mark::Kind::Area),&e).unwrap().unwrap();let keys=intent.keys(&e).unwrap();Reply::new(keys,Some(intent),Ok(vec![bytes])).complete(&mut e).unwrap();assert_eq!(e.selected_part,Some(crate::pick_sources::Part::Face(0)));assert!(e.mark.as_ref().unwrap().label.ends_with("mm²"));assert_eq!(e.scenes().count(),n);
}
#[test]
fn a_later_command_changed_target_or_fetch_failure_cannot_replay_a_pending_edit(){
    for kind in 0..4{let(mut e,bytes)=cold();let id=e.scene.objects()[0].id;let intent=Intent::capture_for(&Action::AddGroup("loaded".into()),&e).unwrap().unwrap();let keys=intent.keys(&e).unwrap();let n=e.scenes().count();let groups=e.scene.groups.len();match kind{0=>{e.apply(Action::Picked(Some(id))).unwrap();},1=>{e.scene.objects_mut().iter_mut().find(|r|r.id==id).unwrap().model=session_rust::Xform::translation(1.,0.,0.);},2=>{e.apply(Action::ConstructionPlane(Some(crate::cplane::CPlane::Xz))).unwrap();},_=>{}}let response=if kind==3{Err("fetch failed".into())}else{Ok(vec![bytes])};let result=Reply::new(keys,Some(intent),response).complete(&mut e);assert!(result.is_err());assert_eq!(e.scenes().count(),n);assert_eq!(e.scene.groups.len(),groups);if kind==3{assert!(e.scene.objects().iter().all(|r|r.geometry().is_none()));}}
}
