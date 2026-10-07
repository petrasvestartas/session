use crate::{chain::PreparedChain, scene::Scene};
use session_rust::{NurbsCurve, Point, Xform};
use std::rc::Rc;

#[test]
fn placed_controls_share_the_original_curve_but_keep_distinct_document_parents() {
    let source = Rc::new(NurbsCurve::create(false, 2, &[Point::new(-0.6, -0.4, 0.0), Point::new(0.0, 0.6, 0.0), Point::new(0.6, -0.4, 0.0)]));
    let weak = Rc::downgrade(&source); let prepared = PreparedChain::curve(source).unwrap();
    let mut scene = Scene::demo(); scene.clear(); let first = scene.insert_path(prepared.clone()).unwrap(); let second = scene.insert_path(prepared).unwrap();
    assert!(scene.control_markers().is_empty()); scene.place_path(second, Xform::translation(2.0, 3.0, 4.0)).unwrap();
    scene.show_controls(true); let markers = scene.control_markers(); assert_eq!(markers.len(), 6);
    assert_eq!(markers[0].parent, first); assert_eq!(markers[3].parent, second);
    assert_eq!(markers[1].control.index, 1); assert_eq!(markers[4].control.index, 1);
    assert!(Rc::ptr_eq(&markers[1].control.source, &markers[4].control.source));
    assert_eq!(markers[1].display.center, [0.0, 0.6, 0.0]); assert_eq!(markers[4].display.center, [2.0, 3.6, 4.0]);
    drop(markers); scene.show_controls(false); assert!(scene.control_markers().is_empty());
    scene.clear(); assert!(weak.upgrade().is_none());
}
