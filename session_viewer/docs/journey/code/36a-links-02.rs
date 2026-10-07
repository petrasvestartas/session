use crate::chain::PreparedChain;
use session_rust::{Arrowhead, Line, Point, Polyline};
use std::rc::Rc;

#[test]
fn linked_repeated_points_closed_neighbours_and_exact_source_are_distinct() {
    let p = |x, y| Point::new(x, y, 0.0);
    let mut line = Polyline::new(vec![p(0.1234567890123, 0.0), p(1.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(0.1234567890123, 0.0)]);
    line.arrowhead = Arrowhead::BOTH;
    let source = Rc::new(line); let weak = Rc::downgrade(&source); let original = source.coords.clone();
    let prepared = PreparedChain::polyline(Rc::clone(&source)).unwrap();
    let segments = prepared.segments(); assert_eq!(segments.len(), 3);
    assert_eq!(segments[0].previous, segments[2].stroke.start);
    assert_eq!(segments[2].next, segments[0].stroke.end);
    assert!(segments.iter().all(|s| s.heads == 0 && s.bytes().len() == 72));
    assert_eq!(source.coords, original); assert_ne!(prepared.points[0][0] as f64, original[0]);
    let shared = prepared.clone(); assert!(Rc::ptr_eq(&prepared.points, &shared.points));
    drop(source); drop(prepared); assert!(weak.upgrade().is_some()); drop(shared); assert!(weak.upgrade().is_none());
}

#[test]
fn heads_use_only_nonzero_free_spans_and_keep_original_line_identity() {
    for (heads, flags) in [(Arrowhead::NONE, 0), (Arrowhead::START, 2), (Arrowhead::END, 1), (Arrowhead::BOTH, 3)] {
        let mut source = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0); source.arrowhead = heads;
        let source = Rc::new(source); let guid = source.guid().to_owned();
        let chain = PreparedChain::line(Rc::clone(&source)).unwrap(); let segment = chain.segments()[0];
        assert_eq!(segment.heads, flags); assert_eq!(u32::from_ne_bytes(segment.bytes()[68..72].try_into().unwrap()), flags);
        assert_eq!(source.guid(), guid);
    }
    assert!(PreparedChain::line(Rc::new(Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0))).unwrap().segments().is_empty());
    assert!(PreparedChain::polyline(Rc::new(Polyline::from_coords(vec![0.0; 7]))).is_err());
    assert!(PreparedChain::polyline(Rc::new(Polyline::from_coords(vec![f64::INFINITY; 6]))).is_err());
}
