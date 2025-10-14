use crate::encoders::{json_dump, json_load};
use crate::{Plane, Point, Polyline, Vector};

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

    let json_string = polyline.jsondump().unwrap();
    assert!(json_string.contains("Polyline"));
    assert!(json_string.contains("points"));
}

#[test]
fn test_polyline_from_json_data() {
    let polyline = Polyline::new(vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)]);

    let json_string = polyline.jsondump().unwrap();
    let deserialized = Polyline::jsonload(&json_string).unwrap();

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
    json_dump(&polyline, filepath, true).unwrap();
    let loaded = json_load::<Polyline>(filepath).unwrap();

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

#[test]
fn test_polyline_shift() {
    let mut polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
    ]);

    polyline.shift(1);

    assert_eq!(polyline.points[0].x(), 1.0);
    assert_eq!(polyline.points[1].x(), 2.0);
    assert_eq!(polyline.points[2].x(), 0.0);
}

#[test]
fn test_polyline_length_squared() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ]);

    let length_sq = polyline.length_squared();
    assert!((length_sq - 2.0).abs() < 1e-5);
}

#[test]
fn test_polyline_point_at_parameter() {
    let start = Point::new(0.0, 0.0, 0.0);
    let end = Point::new(2.0, 0.0, 0.0);

    let mid = Polyline::point_at_parameter(&start, &end, 0.5);
    assert_eq!(mid.x(), 1.0);
    assert_eq!(mid.y(), 0.0);
    assert_eq!(mid.z(), 0.0);
}

#[test]
fn test_polyline_closest_point_to_line() {
    let line_start = Point::new(0.0, 0.0, 0.0);
    let line_end = Point::new(2.0, 0.0, 0.0);
    let test_point = Point::new(1.0, 1.0, 0.0);

    let t = Polyline::closest_point_to_line(&test_point, &line_start, &line_end);
    assert!((t - 0.5).abs() < 1e-5);
}

#[test]
fn test_polyline_line_line_overlap() {
    let line0_start = Point::new(0.0, 0.0, 0.0);
    let line0_end = Point::new(2.0, 0.0, 0.0);
    let line1_start = Point::new(1.0, 0.0, 0.0);
    let line1_end = Point::new(3.0, 0.0, 0.0);

    let overlap = Polyline::line_line_overlap(&line0_start, &line0_end, &line1_start, &line1_end);

    assert!(overlap.is_some());
    let (overlap_start, overlap_end) = overlap.unwrap();
    assert!((overlap_start.x() - 1.0).abs() < 1e-5);
    assert!((overlap_end.x() - 2.0).abs() < 1e-5);
}

#[test]
fn test_polyline_line_line_average() {
    let line0_start = Point::new(0.0, 0.0, 0.0);
    let line0_end = Point::new(2.0, 0.0, 0.0);
    let line1_start = Point::new(0.0, 2.0, 0.0);
    let line1_end = Point::new(2.0, 2.0, 0.0);

    let (avg_start, avg_end) =
        Polyline::line_line_average(&line0_start, &line0_end, &line1_start, &line1_end);

    assert!((avg_start.y() - 1.0).abs() < 1e-5);
    assert!((avg_end.y() - 1.0).abs() < 1e-5);
}

#[test]
fn test_polyline_line_line_overlap_average() {
    let line0_start = Point::new(0.0, 0.0, 0.0);
    let line0_end = Point::new(3.0, 0.0, 0.0);
    let line1_start = Point::new(1.0, 0.0, 0.0);
    let line1_end = Point::new(4.0, 0.0, 0.0);

    let (output_start, output_end) =
        Polyline::line_line_overlap_average(&line0_start, &line0_end, &line1_start, &line1_end);

    assert!(output_start.x() >= 0.0);
    assert!(output_end.x() <= 4.0);
}

#[test]
fn test_polyline_line_from_projected_points() {
    let line_start = Point::new(0.0, 0.0, 0.0);
    let line_end = Point::new(2.0, 0.0, 0.0);
    let points = vec![Point::new(0.5, 1.0, 0.0), Point::new(1.5, -1.0, 0.0)];

    let result = Polyline::line_from_projected_points(&line_start, &line_end, &points);

    assert!(result.is_some());
    let (output_start, output_end) = result.unwrap();
    assert!((output_start.x() - 0.5).abs() < 1e-5);
    assert!((output_end.x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_closest_distance_and_point() {
    let polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)]);
    let test_point = Point::new(1.0, 1.0, 0.0);

    let (distance, edge_id, closest_point) = polyline.closest_distance_and_point(&test_point);

    assert_eq!(edge_id, 0);
    assert!((closest_point.x() - 1.0).abs() < 1e-5);
    assert!((distance - 1.0).abs() < 1e-5);
}

#[test]
fn test_polyline_is_closed() {
    let open_polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ]);
    assert!(!open_polyline.is_closed());

    let closed_polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
    ]);
    assert!(closed_polyline.is_closed());
}

#[test]
fn test_polyline_center() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
        Point::new(2.0, 2.0, 0.0),
        Point::new(0.0, 2.0, 0.0),
    ]);

    let c = polyline.center();
    assert!((c.x() - 1.0).abs() < 1e-5);
    assert!((c.y() - 1.0).abs() < 1e-5);
    assert!((c.z() - 0.0).abs() < 1e-5);
}

#[test]
fn test_polyline_center_vec() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
        Point::new(2.0, 2.0, 0.0),
    ]);

    let c = polyline.center_vec();
    assert!((c.x() - 4.0 / 3.0).abs() < 1e-5);
    assert!((c.y() - 2.0 / 3.0).abs() < 1e-5);
}

#[test]
fn test_polyline_get_average_plane() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ]);

    let (_origin, _x_axis, _y_axis, z_axis) = polyline.get_average_plane();

    assert!((z_axis.z() - 1.0).abs() < 1e-5);
}

#[test]
fn test_polyline_get_fast_plane() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ]);

    let (origin, _plane) = polyline.get_fast_plane();

    assert_eq!(origin.x(), 0.0);
    assert_eq!(origin.y(), 0.0);
    assert_eq!(origin.z(), 0.0);
}

#[test]
fn test_polyline_get_middle_line() {
    let line0_start = Point::new(0.0, 0.0, 0.0);
    let line0_end = Point::new(2.0, 0.0, 0.0);
    let line1_start = Point::new(0.0, 2.0, 0.0);
    let line1_end = Point::new(2.0, 2.0, 0.0);

    let (output_start, output_end) =
        Polyline::get_middle_line(&line0_start, &line0_end, &line1_start, &line1_end);

    assert!((output_start.y() - 1.0).abs() < 1e-5);
    assert!((output_end.y() - 1.0).abs() < 1e-5);
}

#[test]
fn test_polyline_extend_line() {
    let mut start = Point::new(0.0, 0.0, 0.0);
    let mut end = Point::new(1.0, 0.0, 0.0);

    Polyline::extend_line(&mut start, &mut end, 0.5, 0.5);

    assert!((start.x() - (-0.5)).abs() < 1e-5);
    assert!((end.x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_scale_line() {
    let mut start = Point::new(0.0, 0.0, 0.0);
    let mut end = Point::new(2.0, 0.0, 0.0);

    Polyline::scale_line(&mut start, &mut end, 0.25);

    assert!((start.x() - 0.5).abs() < 1e-5);
    assert!((end.x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_extend_segment() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);

    polyline.extend_segment(0, 0.5, 0.5, 0.0, 0.0);

    assert!((polyline.points[0].x() - (-0.5)).abs() < 1e-5);
    assert!((polyline.points[1].x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_extend_segment_equally_static() {
    let mut start = Point::new(0.0, 0.0, 0.0);
    let mut end = Point::new(1.0, 0.0, 0.0);

    Polyline::extend_segment_equally_static(&mut start, &mut end, 0.5, 0.0);

    assert!((start.x() - (-0.5)).abs() < 1e-5);
    assert!((end.x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_extend_segment_equally() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);

    polyline.extend_segment_equally(0, 0.5, 0.0);

    assert!((polyline.points[0].x() - (-0.5)).abs() < 1e-5);
    assert!((polyline.points[1].x() - 1.5).abs() < 1e-5);
}

#[test]
fn test_polyline_move_by() {
    let mut polyline = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
    let translation = Vector::new(1.0, 1.0, 1.0);

    polyline.move_by(&translation);

    assert_eq!(polyline.points[0].x(), 1.0);
    assert_eq!(polyline.points[0].y(), 1.0);
    assert_eq!(polyline.points[0].z(), 1.0);
    assert_eq!(polyline.points[1].x(), 2.0);
    assert_eq!(polyline.points[1].y(), 1.0);
    assert_eq!(polyline.points[1].z(), 1.0);
}

#[test]
fn test_polyline_is_clockwise() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ]);
    let plane = Plane::default();

    let _clockwise = polyline.is_clockwise(&plane);
    // Just test it doesn't crash - the function returns a boolean value
}

#[test]
fn test_polyline_flip() {
    let mut polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
    ]);

    polyline.flip();

    assert_eq!(polyline.points[0].x(), 2.0);
    assert_eq!(polyline.points[1].x(), 1.0);
    assert_eq!(polyline.points[2].x(), 0.0);
}

#[test]
fn test_polyline_get_convex_corners() {
    let polyline = Polyline::new(vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ]);

    let convex_corners = polyline.get_convex_corners();

    assert_eq!(convex_corners.len(), 4);
}

#[test]
fn test_polyline_tween_two_polylines() {
    let polyline0 = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
    let polyline1 = Polyline::new(vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0)]);

    let result = Polyline::tween_two_polylines(&polyline0, &polyline1, 0.5);

    assert!((result.points[0].y() - 1.0).abs() < 1e-5);
    assert!((result.points[1].y() - 1.0).abs() < 1e-5);
}
