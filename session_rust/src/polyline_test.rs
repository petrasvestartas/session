use crate::{Point, Polyline, Vector};

#[test]
fn test_polyline_new() {
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    let polyline = Polyline::new(points);
    assert_eq!(polyline.len(), 3);
    assert_eq!(polyline.segment_count(), 2);
}

#[test]
fn test_polyline_default() {
    let polyline = Polyline::default();
    assert_eq!(polyline.len(), 0);
    assert!(polyline.is_empty());
    assert_eq!(polyline.segment_count(), 0);
}

#[test]
fn test_polyline_length() {
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ];
    let polyline = Polyline::new(points);
    let length = polyline.length();
    assert!((length - 2.0).abs() < 1e-5);
}

#[test]
fn test_polyline_add_point() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
    assert_eq!(polyline.len(), 2);

    polyline.add_point(Point::new(1.0, 1.0, 0.0));
    assert_eq!(polyline.len(), 3);
    assert_eq!(polyline.segment_count(), 2);
}

#[test]
fn test_polyline_insert_point() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)]);

    polyline.insert_point(1, Point::new(1.0, 0.0, 0.0));
    assert_eq!(polyline.len(), 3);
    assert_eq!(polyline.points[1].x(), 1.0);
}

#[test]
fn test_polyline_remove_point() {
    let mut polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
    ]);

    let removed = polyline.remove_point(1);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().x(), 1.0);
    assert_eq!(polyline.len(), 2);
}

#[test]
fn test_polyline_reverse() {
    let mut polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
    ]);

    polyline.reverse();
    assert_eq!(polyline.points[0].x(), 2.0);
    assert_eq!(polyline.points[1].x(), 1.0);
    assert_eq!(polyline.points[2].x(), 0.0);
}

#[test]
fn test_polyline_reversed() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
    ]);

    let reversed = polyline.reversed();
    assert_eq!(reversed.points[0].x(), 2.0);
    assert_eq!(reversed.points[1].x(), 1.0);
    assert_eq!(reversed.points[2].x(), 0.0);

    // Original should be unchanged
    assert_eq!(polyline.points[0].x(), 0.0);
}

#[test]
fn test_polyline_add_assign_vector() {
    let mut polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    let v = Vector::new(4.0, 5.0, 6.0);
    polyline += &v;

    assert_eq!(polyline.points[0].x(), 5.0);
    assert_eq!(polyline.points[0].y(), 7.0);
    assert_eq!(polyline.points[0].z(), 9.0);
    assert_eq!(polyline.points[1].x(), 8.0);
    assert_eq!(polyline.points[1].y(), 10.0);
    assert_eq!(polyline.points[1].z(), 12.0);
}

#[test]
fn test_polyline_add_vector() {
    let polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    let v = Vector::new(4.0, 5.0, 6.0);
    let polyline2 = polyline + &v;

    assert_eq!(polyline2.points[0].x(), 5.0);
    assert_eq!(polyline2.points[0].y(), 7.0);
    assert_eq!(polyline2.points[0].z(), 9.0);
}

#[test]
fn test_polyline_sub_assign_vector() {
    let mut polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    let v = Vector::new(4.0, 5.0, 6.0);
    polyline -= &v;

    assert_eq!(polyline.points[0].x(), -3.0);
    assert_eq!(polyline.points[0].y(), -3.0);
    assert_eq!(polyline.points[0].z(), -3.0);
    assert_eq!(polyline.points[1].x(), 0.0);
    assert_eq!(polyline.points[1].y(), 0.0);
    assert_eq!(polyline.points[1].z(), 0.0);
}

#[test]
fn test_polyline_sub_vector() {
    let polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);
    let v = Vector::new(4.0, 5.0, 6.0);
    let polyline2 = polyline - &v;

    assert_eq!(polyline2.points[0].x(), -3.0);
    assert_eq!(polyline2.points[0].y(), -3.0);
    assert_eq!(polyline2.points[0].z(), -3.0);
    assert_eq!(polyline2.points[1].x(), 0.0);
    assert_eq!(polyline2.points[1].y(), 0.0);
    assert_eq!(polyline2.points[1].z(), 0.0);
}

#[test]
fn test_polyline_display() {
    let polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
    let display_str = format!("{polyline}");
    assert!(display_str.contains("Polyline"));
    assert!(display_str.contains("points=2"));
}

#[test]
fn test_polyline_json_serialization() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ]);

    let json = serde_json::to_string(&polyline).unwrap();
    let deserialized: Polyline = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.len(), 3);
    assert_eq!(deserialized.points[0].x(), 0.0);
    assert_eq!(deserialized.points[1].x(), 1.0);
    assert_eq!(deserialized.points[2].y(), 1.0);
}

#[test]
fn test_polyline_to_json_data() {
    let polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);

    let json_string = polyline.to_json_data().unwrap();
    assert!(json_string.contains("Polyline"));
    assert!(json_string.contains("points"));
}

#[test]
fn test_polyline_from_json_data() {
    let polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);

    let json_string = polyline.to_json_data().unwrap();
    let deserialized = Polyline::from_json_data(&json_string).unwrap();

    assert_eq!(deserialized.len(), 2);
    assert_eq!(deserialized.points[0].x(), 1.0);
    assert_eq!(deserialized.points[1].x(), 4.0);
}

#[test]
fn test_polyline_to_json_from_json() {
    let polyline = Polyline::new(vec![
        Point::new(1.0, 2.0, 3.0),
        Point::new(4.0, 5.0, 6.0),
        Point::new(7.0, 8.0, 9.0),
    ]);

    let filepath = "test_polyline.json";
    polyline.to_json(filepath).unwrap();
    let loaded = Polyline::from_json(filepath).unwrap();

    assert_eq!(loaded.len(), 3);
    assert_eq!(loaded.points[0].x(), 1.0);
    assert_eq!(loaded.points[1].y(), 5.0);
    assert_eq!(loaded.points[2].z(), 9.0);
}

#[test]
fn test_polyline_get_point() {
    let polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0)]);

    let point = polyline.get_point(1);
    assert!(point.is_some());
    assert_eq!(point.unwrap().x(), 1.0);

    let invalid = polyline.get_point(10);
    assert!(invalid.is_none());
}

#[test]
fn test_polyline_get_point_mut() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0)]);

    if let Some(point) = polyline.get_point_mut(1) {
        *point = Point::new(5.0, 6.0, 7.0);
    }

    assert_eq!(polyline.points[1].x(), 5.0);
    assert_eq!(polyline.points[1].y(), 6.0);
    assert_eq!(polyline.points[1].z(), 7.0);
}
