use crate::{chain::PreparedChain, scene::Scene, history::History};
use session_rust::{Polyline, Point};
use std::rc::Rc;

#[test]
fn connected_identity_and_history_preserve_original_sources() {
    let mut scene = Scene::demo(); let mut history = History::default();
    let source = Rc::new(Polyline::new(vec![Point::new(2.123456789, 0.0, 0.0), Point::new(4.0, 1.0, 2.0)]));
    let weak = Rc::downgrade(&source); let prepared = PreparedChain::polyline(source).unwrap();
    let first = scene.insert_path(prepared.clone()).unwrap(); let second = scene.insert_path(prepared).unwrap();
    assert_ne!(first, second); assert_ne!(scene.paths()[0].guid, scene.paths()[1].guid);
    assert_eq!(scene.paths()[0].points()[0][0], 2.123456789);
    history.try_edit(&mut scene, |scene| scene.add_box().map(|_| ())).unwrap();
    assert!(history.undo(&mut scene)); assert!(history.redo(&mut scene));
    assert_eq!(scene.paths()[0].id, first); assert!(crate::document::snapshot(&scene).is_err());
    scene.clear(); assert!(weak.upgrade().is_some()); history.clear(); assert!(weak.upgrade().is_none());
    assert!(scene.paths().is_empty()); assert!(scene.bounds().is_none());
}
