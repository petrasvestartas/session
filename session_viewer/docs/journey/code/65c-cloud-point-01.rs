use crate::{cloud::Prepared, editor::{Action,Editor}, pick_sources::{Part,Target}, scene::Scene};
use session_rust::{PointCloud,Point,Xform};
use std::rc::Rc;

fn source()->Rc<PointCloud> {
    let mut source=PointCloud::new(vec![Point::new(-0.4,0.,0.2),Point::new(0.4,0.,0.2)],vec![],vec![]);
    source.point_size=16.;let mut proto=source.to_proto();proto.point_ids=vec![u32::MAX,16_777_217];
    Rc::new(PointCloud::from_proto(proto))
}

#[test]
fn cloud_point_identity_uses_original_u32_ids_and_f64_instance_positions() {
    let source=source();let _=source.guid();let before=source.to_proto();let mut scene=Scene::demo();scene.clear();
    let owner=scene.insert_cloud(Prepared::new(source.clone()).unwrap()).unwrap();scene.cloud_controls(Some(owner));
    let target=crate::cloud_pick::resolve(&scene,owner,0).unwrap();assert_eq!(target,Target {owner,part:Part::CloudPoint(u32::MAX)});
    assert!(target.valid(&scene));assert!(target.describe().contains("4294967295"));
    assert!(crate::cloud_pick::resolve(&scene,owner,2).is_none());
    scene.place_cloud(owner,Xform::translation(1_000_000_000.125,0.,0.)).unwrap();
    assert_eq!(crate::cloud_pick::position(&scene,owner,u32::MAX).unwrap()[0],1_000_000_000.125-0.4);
    assert_eq!(crate::part_preview::origin(&scene,target).unwrap()[0],1_000_000_000.125-0.4);
    assert!(crate::part_preview::Preview::new(&scene,target).is_err());assert_eq!(source.to_proto(),before);
    Rc::make_mut(&mut scene.view).lock(&[owner],true);assert!(!target.valid(&scene));
    Rc::make_mut(&mut scene.view).lock(&[owner],false);Rc::make_mut(&mut scene.view).hide(&[owner],true);assert!(!target.valid(&scene));
    Rc::make_mut(&mut scene.view).hide(&[owner],false);scene.cloud_controls(None);assert!(!target.valid(&scene));
}

#[test]
fn cloud_controls_bind_one_selected_parent_without_a_per_point_cpu_target_table() {
    let mut editor=Editor::default();editor.apply(Action::Close).unwrap();let source=source();
    let owner=editor.scene.insert_cloud(Prepared::new(source.clone()).unwrap()).unwrap();let copy=editor.scene.copy_row(owner,false).unwrap();
    editor.apply(Action::Picked(Some(owner))).unwrap();let history=editor.scenes().count();editor.apply(Action::Controls(true)).unwrap();
    assert!(editor.scene.clouds()[0].controls_visible);assert!(!editor.scene.clouds()[1].controls_visible);
    let targets=crate::pick_sources::Sources::new(&editor.scene).unwrap();assert!(targets.controls.is_empty());
    let tag=crate::sub_id::encode(crate::sub_id::Kind::Control,1).unwrap();
    let target=targets.resolve(&editor.scene,[owner.value(),tag],crate::pick_sources::Mode::Control).unwrap();assert_eq!(target.part,Part::CloudPoint(16_777_217));
    assert!(targets.resolve(&editor.scene,[copy.value(),tag],crate::pick_sources::Mode::Control).is_none());
    editor.apply(Action::PickedTarget(Some(target))).unwrap();assert_eq!(editor.selection(),Some(target));assert!(editor.gizmo_origin().is_none());editor.apply(Action::Controls(true)).unwrap();assert_eq!(editor.selection(),Some(target));
    assert_eq!(editor.scenes().count(),history);editor.apply(Action::Controls(false)).unwrap();assert!(!target.valid(&editor.scene));
    assert!(editor.scene.clouds().iter().all(|r|!r.controls_visible));editor.apply(Action::Close).unwrap();assert!(editor.scene.clouds().is_empty());
}
