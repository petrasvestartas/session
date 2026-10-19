use std::rc::Rc;
use crate::scene::Scene;

use crate::range_example::prepared;

#[test]
fn streamed_owners_share_sources_keep_original_ids_and_refuse_incomplete_save() {
    let prepared=prepared();let weak=Rc::downgrade(&prepared.source);let mut scene=Scene::empty();let first=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),4.).unwrap();let second=scene.insert_stream(prepared.clone(),session_rust::Xform::translation(2.,0.,0.),7.).unwrap();assert_ne!(first,second);assert_ne!(scene.row_guid(first),scene.row_guid(second));assert!(Rc::ptr_eq(&scene.streams()[0].prepared,&scene.streams()[1].prepared));assert_eq!(scene.streams()[1].point_px,7.);assert_eq!(scene.next(None),Some(first));assert_eq!(scene.next(Some(first)),Some(second));assert!(crate::scene_archive::save(&scene).unwrap_err().contains("complete source"));
    let mut editor=crate::editor::Editor::default();editor.scene=scene.clone();editor.apply(crate::editor::Action::SelectAll).unwrap();let mut selected=editor.selected_objects();selected.sort_by_key(|id|id.value());assert_eq!(selected,[first,second]);editor.apply(crate::editor::Action::Close).unwrap();
    scene.cloud_controls(Some(first));let selected=crate::cloud_pick::resolve(&scene,first,0).unwrap();assert_eq!(selected.part,crate::pick_sources::Part::CloudPoint(u32::MAX));assert_eq!(crate::cloud_pick::resolve(&scene,first,2).unwrap().part,crate::pick_sources::Part::CloudPoint(10));assert_eq!(crate::cloud_pick::position(&scene,first,u32::MAX).unwrap()[1],0.5);scene.translate(first,[0.25,0.,0.]).unwrap();assert_eq!(crate::cloud_pick::position(&scene,first,u32::MAX).unwrap()[0],-0.25);assert_eq!(prepared.position(u32::MAX).unwrap()[0],-0.5);
    scene.clear();drop(prepared);assert!(weak.upgrade().is_none());
}
#[test]
fn pinned_coarse_samples_survive_cache_eviction_and_source_close_releases_them() {
    let prepared=prepared();let mut scene=Scene::empty();let id=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),4.).unwrap();scene.cloud_controls(Some(id));prepared.pages.borrow_mut().clear();assert_eq!(prepared.pages().len(),1);assert_eq!(crate::cloud_pick::resolve(&scene,id,0).unwrap().part,crate::pick_sources::Part::CloudPoint(10));assert!(prepared.position(u32::MAX).is_none());assert_eq!(prepared.position(10).unwrap()[0],-0.5);let root=Rc::downgrade(prepared.root.borrow().as_ref().unwrap());scene.clear();drop(prepared);assert!(root.upgrade().is_none());
}

#[test]
fn shared_streamed_owners_report_original_and_retained_counts_separately() {
    let prepared=prepared();let mut scene=Scene::empty();let id=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),4.).unwrap();scene.copy_row(id,false).unwrap();let snapshot=scene.clone();let count=crate::memory::streams([&scene,&snapshot]);assert_eq!(&count[..3],&[4,1,4]);assert_eq!(count[4],4);assert_eq!(count[3],prepared.pages().iter().map(|page|page.bytes()).sum::<usize>());prepared.pages.borrow_mut().clear();let count=crate::memory::streams([&scene,&snapshot]);assert_eq!(count[2],4);assert_eq!(count[4],2);scene.clear();assert_eq!(crate::memory::streams([&scene]),[0;5]);
}
