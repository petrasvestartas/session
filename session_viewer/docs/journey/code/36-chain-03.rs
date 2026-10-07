use crate::chain::{PreparedChain, Source};
use session_rust::{Point, Polyline};
use std::rc::Rc;

#[test]
fn exact_sources_shared_display_points_and_invalid_input() {
    let source = Rc::new(Polyline::new(vec![Point::new(0.1234567890123, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]));
    let weak = Rc::downgrade(&source); let prepared = PreparedChain::polyline(Rc::clone(&source)).unwrap();
    let Source::Polyline(owner) = &prepared.source else { panic!("Original polyline owner") };
    assert!(Rc::ptr_eq(owner, &source)); assert_eq!(owner.coords, source.coords);
    assert_ne!(prepared.points[0][0] as f64, source.coords[0]);
    let shared = prepared.clone(); assert!(Rc::ptr_eq(&prepared.points, &shared.points));
    drop(source); drop(prepared); assert!(weak.upgrade().is_some()); drop(shared); assert!(weak.upgrade().is_none());
    assert!(PreparedChain::polyline(Rc::new(Polyline::from_coords(vec![0.0; 7]))).is_err());
    assert!(PreparedChain::polyline(Rc::new(Polyline::from_coords(vec![f64::INFINITY; 6]))).is_err());
}
