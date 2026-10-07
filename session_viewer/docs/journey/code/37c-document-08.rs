use crate::{marker::PreparedPoint, scene::Scene, history::History};
use session_rust::{Point, Xform};
use std::rc::Rc;

#[test]
fn original_point_rows_keep_precise_placement_shared_history_and_release() {
    let mut scene = Scene::demo(); let mut history = History::default();
    let source = Rc::new(Point::new(2.123456789, 0.0, 0.0)); let weak = Rc::downgrade(&source);
    let prepared = PreparedPoint::new(Rc::clone(&source)).unwrap();
    let first = scene.insert_point(prepared.clone()).unwrap(); let second = scene.insert_point(prepared).unwrap();
    assert_ne!(first, second); assert_ne!(scene.points()[0].guid, scene.points()[1].guid);
    history.try_edit(&mut scene, |scene| scene.place_point(first, Xform::translation(10.0, 0.0, 0.0))).unwrap();
    assert!((scene.points()[0].point()[0] - 12.123456789).abs() < 1e-12); assert_eq!(source[0], 2.123456789);
    assert_eq!(scene.bounds().unwrap().max[0], 12.123456789);
    let model = scene.points()[0].model.clone(); let mut invalid = Xform::identity(); invalid.m[0] = f64::NAN;
    assert!(scene.place_point(first, invalid).is_err()); assert_eq!(scene.points()[0].model.m, model.m);
    assert!(history.undo(&mut scene)); assert_eq!(scene.points()[0].point()[0], source[0]);
    assert!(history.redo(&mut scene)); assert_eq!(scene.points()[0].id, first);
    assert!(Rc::ptr_eq(&scene.points()[0].prepared.source, &history.scenes().next().unwrap().points()[0].prepared.source));
    assert!(crate::document::snapshot(&scene).is_err());
    drop(source); scene.clear(); assert!(weak.upgrade().is_some()); history.clear(); assert!(weak.upgrade().is_none());
    assert!(scene.points().is_empty()); assert!(scene.bounds().is_none());
}
