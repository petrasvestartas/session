use crate::marker::PreparedPoint;
use session_rust::{Point, Color};
use std::rc::Rc;

#[test]
fn point_preparation_preserves_original_coordinates_style_and_shared_owner() {
    let mut point = Point::new(0.123456789012345, 0.0, 0.0); point.width = 9.0;
    point.pointcolor = Color::new(0.2, 0.4, 0.6, 0.5);
    let source = Rc::new(point); let weak = Rc::downgrade(&source); let guid = source.guid().to_owned();
    let prepared = PreparedPoint::new(Rc::clone(&source)).unwrap();
    assert_eq!(prepared.source[0], 0.123456789012345); assert_eq!(prepared.source.guid(), guid);
    assert_ne!(prepared.display.center[0] as f64, source[0]); assert_eq!(prepared.display.diameter, 9.0);
    let bytes = prepared.display.bytes(); assert_eq!(bytes.len(), 32);
    assert_eq!(f32::from_ne_bytes(bytes[12..16].try_into().unwrap()), 9.0);
    let shared = prepared.clone(); assert!(Rc::ptr_eq(&prepared.source, &shared.source));
    drop(source); drop(prepared); assert!(weak.upgrade().is_some()); drop(shared); assert!(weak.upgrade().is_none());
}

#[test]
fn marker_default_width_and_display_refusal_match_their_source_policy() {
    for width in [0.0, 1.0, -1.0, f64::NAN, f64::INFINITY, 1e-100] {
        let mut point = Point::new(0.0, 0.0, 0.0); point.width = width;
        assert_eq!(PreparedPoint::new(Rc::new(point)).unwrap().display.diameter, 6.0);
    }
    for x in [f64::NAN, f64::INFINITY, 1e100] { assert!(PreparedPoint::new(Rc::new(Point::new(x, 0.0, 0.0))).is_err()); }
    let mut point = Point::new(0.0, 0.0, 0.0); point.width = 1e100;
    assert!(PreparedPoint::new(Rc::new(point)).is_err());
}
