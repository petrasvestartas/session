use crate::stroke::PreparedLine;
use session_rust::{Color, Line};
use std::rc::Rc;

#[test]
fn display_packing_preserves_the_original_source_owner_and_precision() {
    let mut line = Line::new(0.123456789012345, 0.0, 0.0, 1.0, 2.0, 3.0);
    line.linecolor = Color::new(0.2, 0.4, 0.6, 1.0); line.width = 3.0;
    let source = Rc::new(line); let weak = Rc::downgrade(&source);
    let first = PreparedLine::new(Rc::clone(&source)).unwrap();
    let second = PreparedLine::new(Rc::clone(&source)).unwrap();
    assert!(Rc::ptr_eq(&first.source, &second.source));
    assert_eq!(first.source.start()[0], 0.123456789012345);
    assert_ne!(first.display.start[0] as f64, first.source.start()[0]);
    assert_eq!(first.display.colour, [0.2, 0.4, 0.6, 1.0]);
    assert_eq!(first.display.width, 3.0); assert_eq!(first.display.bytes().len(), 44);
    assert_eq!(f32::from_ne_bytes(first.display.bytes()[40..44].try_into().unwrap()), 3.0);
    let guid = first.source.guid().to_owned(); drop(first); drop(source);
    assert_eq!(second.source.guid(), guid); assert!(weak.upgrade().is_some());
    drop(second); assert!(weak.upgrade().is_none());
}

#[test]
fn default_widths_and_invalid_display_coordinates_follow_their_separate_rules() {
    for width in [0.0, -2.0, f64::NAN, f64::INFINITY, 1e-100] {
        let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0); line.width = width;
        assert_eq!(PreparedLine::new(Rc::new(line)).unwrap().display.width, 1.0);
    }
    for value in [f64::NAN, f64::INFINITY, 1e100] {
        assert!(PreparedLine::new(Rc::new(Line::new(value, 0.0, 0.0, 1.0, 0.0, 0.0))).is_err());
    }
    let mut wide = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0); wide.width = 1e100;
    assert!(PreparedLine::new(Rc::new(wide)).is_err());
}
