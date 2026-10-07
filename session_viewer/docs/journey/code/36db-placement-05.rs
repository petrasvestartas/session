use crate::{chain::PreparedChain, scene::Scene, history::History, memory};
use session_rust::{Polyline, Point, Xform};
use std::rc::Rc;

#[test]
fn connected_rows_keep_exact_source_placement_shared_history_and_complete_release() {
    let mut scene = Scene::demo(); let mut history = History::default();
    let source = Rc::new(Polyline::new(vec![Point::new(2.123456789, 0.0, 0.0), Point::new(4.0, 1.0, 2.0)]));
    let weak = Rc::downgrade(&source); let prepared = PreparedChain::polyline(Rc::clone(&source)).unwrap();
    let first = scene.insert_path(prepared.clone()).unwrap(); let second = scene.insert_path(prepared).unwrap();
    assert_ne!(first, second); assert_ne!(scene.paths()[0].guid, scene.paths()[1].guid);
    history.try_edit(&mut scene, |scene| scene.place_path(first, Xform::translation(10.0, 0.0, 0.0))).unwrap();
    assert!((scene.paths()[0].points()[0][0] - 12.123456789).abs() < 1e-12);
    assert_eq!(scene.bounds().unwrap().max[0], 14.0); assert_eq!(source.coords[0], 2.123456789);
    let usage = memory::paths(std::iter::once(&scene).chain(history.scenes()));
    assert_eq!(&usage[..3], &[4, 1, 1]); assert!(usage[3] > 0); assert_eq!(usage[4], scene.paths()[0].prepared.points.capacity() * 12);
    let model = scene.paths()[0].model.clone(); let mut invalid = Xform::identity(); invalid.m[0] = f64::NAN;
    assert!(scene.place_path(first, invalid).is_err()); assert_eq!(scene.paths()[0].model.m, model.m);
    assert!(history.undo(&mut scene)); assert_eq!(scene.paths()[0].points()[0][0], 2.123456789);
    assert!(history.redo(&mut scene)); assert_eq!(scene.paths()[0].points()[0][0], 12.123456789);
    assert!(crate::document::snapshot(&scene).is_err());
    drop(source); scene.clear(); assert!(weak.upgrade().is_some()); history.clear(); assert!(weak.upgrade().is_none());
    assert_eq!(memory::paths([&scene]), [0; 5]); assert!(scene.bounds().is_none());
}
