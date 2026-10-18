    scene.cloud_controls(None);renderer.set_scene(&scene,None);renderer.set_pick_mode(Mode::Object);camera_pixels(&mut renderer,&camera);assert_eq!(renderer.cloud_usage()[0],64);assert_eq!(renderer.cloud_usage()[2],uploads);
    scene.clear();renderer.set_scene(&scene,None);assert_eq!(renderer.cloud_usage(),[0;5]);
}

#[test]
fn actual_streamed_pages_pick_original_ids_share_uploads_and_release_after_eviction() {
    use crate::pick_sources::{Mode,Part};
    let prepared=crate::range_example::prepared();let weak=Rc::downgrade(&prepared.source);
    let mut scene=Scene::empty();let owner=scene.insert_stream(prepared.clone(),session_rust::Xform::identity(),12.).unwrap();scene.cloud_controls(Some(owner));
    let mut renderer=renderer(&scene);renderer.set_pick_mode(Mode::Control);
    for (cursor,id) in [([-0.5,0.5],u32::MAX),([0.5,0.5],16_777_217),([-0.5,-0.5],10)] {
        renderer.request_pick(cursor,identity()).unwrap();let hit=finish(&mut renderer).expect("Actual streamed point coverage");let target=renderer.resolve_pick(&scene,hit).unwrap();assert_eq!(target.owner,owner);assert_eq!(target.part,Part::CloudPoint(id));
    }
    let pixels=colour_pixels(&mut renderer);assert!(pixels.chunks_exact(4).any(|p|p[2]>160&&p[0]<10));assert!(pixels.chunks_exact(4).any(|p|p[0]>160&&p[2]<10));
    let uploads=renderer.cloud_usage()[2];let copy=scene.copy_row(owner,false).unwrap();scene.translate(copy,[0.2,0.,0.]).unwrap();renderer.set_scene(&scene,None);colour_pixels(&mut renderer);assert_eq!(renderer.cloud_usage()[2],uploads,"Placed owners share original page uploads");
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],true);renderer.set_scene(&scene,None);renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none());
    Rc::make_mut(&mut scene.view).lock(&[owner,copy],false);let held=Rc::downgrade(&prepared.page(2).unwrap());prepared.pages.borrow_mut().clear();assert!(held.upgrade().is_some(),"GPU page owner retains precise pick identities");renderer.set_scene(&scene,None);assert!(held.upgrade().is_none());renderer.request_pick([-0.5,0.5],identity()).unwrap();assert!(finish(&mut renderer).is_none(),"Evicted detailed points are absent");
    renderer.request_pick([-0.5,-0.5],identity()).unwrap();let hit=finish(&mut renderer).unwrap();assert_eq!(renderer.resolve_pick(&scene,hit).unwrap().part,Part::CloudPoint(10));
    scene.clear();renderer.set_scene(&scene,None);drop(prepared);assert_eq!(renderer.cloud_usage(),[0;5]);assert!(weak.upgrade().is_none());
}
