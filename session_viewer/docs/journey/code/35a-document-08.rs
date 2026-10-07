use crate::{document, history::History, scene::Scene, stroke::PreparedLine};
use session_rust::{Line, Xform};
use std::rc::Rc;

#[test]
fn line_rows_keep_distinct_ids_original_sources_and_placed_bounds() {
    let mut scene = Scene::demo();
    let source = Rc::new(Line::new(2.123456789, 0.0, 0.0, 4.0, 1.0, 2.0));
    let id = scene.insert_line(PreparedLine::new(Rc::clone(&source)).unwrap()).unwrap();
    assert!(scene.objects().iter().all(|row| row.id != id));
    scene.place_line(id, Xform::translation(10.0, 0.0, 0.0)).unwrap();
    assert!((scene.lines()[0].endpoints()[0][0] - 12.123456789).abs() < 1e-12);
    assert_eq!(scene.bounds().unwrap().max[0], 14.0);
    assert_eq!(source.start()[0], 2.123456789);
    assert!(document::snapshot(&scene).is_err());
}

#[test]
fn history_shares_line_sources_and_close_releases_every_owner() {
    let mut scene = Scene::demo(); let mut history = History::default();
    let source = Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)); let weak = Rc::downgrade(&source);
    let id = scene.insert_line(PreparedLine::new(source).unwrap()).unwrap();
    history.try_edit(&mut scene, |scene| scene.place_line(id, Xform::translation(3.0, 0.0, 0.0))).unwrap();
    assert!(Rc::ptr_eq(&scene.lines()[0].prepared.source, &history.scenes().next().unwrap().lines()[0].prepared.source));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    assert!(history.undo(&mut scene)); assert_eq!(scene.lines()[0].endpoints()[0][0], 0.0);
    assert!(history.redo(&mut scene)); assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    scene.clear(); assert!(weak.upgrade().is_some()); history.clear(); assert!(weak.upgrade().is_none());
    assert!(scene.lines().is_empty()); assert!(scene.objects().is_empty()); assert!(scene.bounds().is_none());
}
