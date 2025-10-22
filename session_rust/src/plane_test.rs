use crate::encoders::{json_dump, json_load};
use crate::{Plane, Point, Vector};
use std::f64::consts::PI;

#[test]
fn test_plane_default_constructor() {
    let plane = Plane::default();
    assert_eq!(plane.origin(), Point::new(0.0, 0.0, 0.0));
    let x = plane.x_axis();
    let y = plane.y_axis();
    let z = plane.z_axis();
    assert_eq!((x.x(), x.y(), x.z()), (1.0, 0.0, 0.0));
    assert_eq!((y.x(), y.y(), y.z()), (0.0, 1.0, 0.0));
    assert_eq!((z.x(), z.y(), z.z()), (0.0, 0.0, 1.0));
    assert_eq!(plane.a(), 0.0);
    assert_eq!(plane.b(), 0.0);
    assert_eq!(plane.c(), 1.0);
    assert_eq!(plane.d(), 0.0);
}

#[test]
fn test_plane_constructor_from_origin_and_axes() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let x = Vector::new(1.0, 0.0, 0.0);
    let y = Vector::new(0.0, 1.0, 0.0);
    let plane = Plane::with_name(origin.clone(), x, y, "test_plane".to_string());
    assert_eq!(plane.name, "test_plane");
    assert_eq!(plane.origin(), origin);
    assert_eq!(plane.c(), 1.0);
}

#[test]
fn test_plane_from_point_normal() {
    let p = Point::new(0.0, 0.0, 5.0);
    let n = Vector::new(0.0, 0.0, 1.0);
    let plane = Plane::from_point_normal(p.clone(), n);
    assert_eq!(plane.origin(), p);
    assert!((plane.z_axis().z() - 1.0).abs() < 1e-5);
    assert!((plane.d() + 5.0).abs() < 1e-5);
}

#[test]
fn test_plane_from_points() {
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    let plane = Plane::from_points(points);
    assert!((plane.c() - 1.0).abs() < 1e-5);
    assert!(plane.d().abs() < 1e-5);
}

#[test]
fn test_plane_from_two_points() {
    let p1 = Point::new(0.0, 0.0, 0.0);
    let p2 = Point::new(1.0, 0.0, 0.0);
    let plane = Plane::from_two_points(p1.clone(), p2);
    assert_eq!(plane.origin(), p1);
}

#[test]
fn test_plane_xy_plane() {
    let plane = Plane::xy_plane();
    assert_eq!(plane.name, "xy_plane");
    assert_eq!(plane.a(), 0.0);
    assert_eq!(plane.b(), 0.0);
    assert_eq!(plane.c(), 1.0);
    assert_eq!(plane.d(), 0.0);
}

#[test]
fn test_plane_yz_plane() {
    let plane = Plane::yz_plane();
    assert_eq!(plane.name, "yz_plane");
    assert_eq!(plane.a(), 1.0);
    assert_eq!(plane.b(), 0.0);
    assert_eq!(plane.c(), 0.0);
    assert_eq!(plane.d(), 0.0);
}

#[test]
fn test_plane_xz_plane() {
    let plane = Plane::xz_plane();
    assert_eq!(plane.name, "xz_plane");
    assert_eq!(plane.a(), 0.0);
    assert_eq!(plane.b(), 1.0);
    assert_eq!(plane.c(), 0.0);
    assert_eq!(plane.d(), 0.0);
}

#[test]
fn test_plane_to_string() {
    let plane = Plane::xy_plane();
    let str = format!("{plane}");
    assert!(str.contains("Plane"));
    assert!(str.contains("xy_plane"));
}

#[test]
fn test_plane_operator_index() {
    let plane = Plane::default();
    let x = &plane[0];
    let y = &plane[1];
    let z = &plane[2];
    assert_eq!((x.x(), x.y(), x.z()), (1.0, 0.0, 0.0));
    assert_eq!((y.x(), y.y(), y.z()), (0.0, 1.0, 0.0));
    assert_eq!((z.x(), z.y(), z.z()), (0.0, 0.0, 1.0));
}

#[test]
fn test_plane_operator_add_assign_translation() {
    let mut plane = Plane::xy_plane();
    let offset = Vector::new(1.0, 2.0, 3.0);
    plane += offset;
    assert_eq!(plane.origin().x(), 1.0);
    assert_eq!(plane.origin().y(), 2.0);
    assert_eq!(plane.origin().z(), 3.0);
    assert!((plane.d() + 3.0).abs() < 1e-5);
}

#[test]
fn test_plane_operator_sub_assign_translation() {
    let mut plane = Plane::xy_plane();
    let offset = Vector::new(1.0, 2.0, 3.0);
    plane -= offset;
    assert_eq!(plane.origin().x(), -1.0);
    assert_eq!(plane.origin().y(), -2.0);
    assert_eq!(plane.origin().z(), -3.0);
}

#[test]
fn test_plane_operator_add_translation() {
    let plane = Plane::xy_plane();
    let offset = Vector::new(1.0, 2.0, 3.0);
    let moved = plane.clone() + offset;
    assert_eq!(moved.origin().z(), 3.0);
    assert_eq!(plane.origin().z(), 0.0);
}

#[test]
fn test_plane_operator_sub_translation() {
    let plane = Plane::xy_plane();
    let offset = Vector::new(1.0, 2.0, 3.0);
    let moved = plane - offset;
    assert_eq!(moved.origin().z(), -3.0);
}

#[test]
fn test_plane_json_serialization() {
    let plane = Plane::xy_plane();
    let json = plane.jsondump().unwrap();
    assert!(json.contains("Plane"));
    assert!(json.contains("xy_plane"));
}

#[test]
fn test_plane_json_deserialization() {
    let original = Plane::xy_plane();
    let json = original.jsondump().unwrap();
    let loaded = Plane::jsonload(&json).unwrap();
    assert_eq!(loaded.name, "xy_plane");
    assert_eq!(loaded.c(), 1.0);
}

#[test]
fn test_plane_json_file_round_trip() {
    let filepath = "test_plane.json";
    let original = Plane::xy_plane();
    json_dump(&original, filepath, true).unwrap();
    let loaded: Plane = json_load(filepath).unwrap();
    assert_eq!(loaded.name, original.name);
    assert_eq!(loaded.c(), original.c());
}

#[test]
fn test_plane_reverse() {
    let mut plane = Plane::xy_plane();
    let orig_x = plane.x_axis();
    let orig_y = plane.y_axis();
    plane.reverse();
    assert_eq!(plane.x_axis(), orig_y);
    assert_eq!(plane.y_axis(), orig_x);
    assert_eq!(plane.c(), -1.0);
}

#[test]
fn test_plane_rotate() {
    let mut plane = Plane::xy_plane();
    let angle = PI / 2.0;
    plane.rotate(angle);
    assert!((plane.x_axis().y() - 1.0).abs() < 1e-5);
}

#[test]
fn test_plane_is_same_direction_parallel() {
    let p1 = Plane::xy_plane();
    let p2 = Plane::xy_plane();
    assert!(Plane::is_same_direction(&p1, &p2, true));
}

#[test]
fn test_plane_is_same_direction_flipped() {
    let p1 = Plane::xy_plane();
    let mut p2 = Plane::xy_plane();
    p2.reverse();
    assert!(Plane::is_same_direction(&p1, &p2, true));
    assert!(!Plane::is_same_direction(&p1, &p2, false));
}

#[test]
fn test_plane_is_same_position() {
    let p1 = Plane::xy_plane();
    let mut p2 = Plane::xy_plane();
    assert!(Plane::is_same_position(&p1, &p2));
    p2 += Vector::new(0.0, 0.0, 1.0);
    assert!(!Plane::is_same_position(&p1, &p2));
}

#[test]
fn test_plane_is_coplanar() {
    let p1 = Plane::xy_plane();
    let mut p2 = Plane::xy_plane();
    assert!(Plane::is_coplanar(&p1, &p2, true));
    p2.reverse();
    assert!(Plane::is_coplanar(&p1, &p2, true));
    p2 += Vector::new(0.0, 0.0, 1.0);
    assert!(!Plane::is_coplanar(&p1, &p2, true));
}

#[test]
fn test_plane_is_right_hand() {
    let mut plane = Plane::xy_plane();
    assert!(plane.is_right_hand());
    plane = Plane::yz_plane();
    assert!(plane.is_right_hand());
    plane = Plane::xz_plane();
    assert!(plane.is_right_hand());
    plane = Plane::default();
    assert!(plane.is_right_hand());
    plane.reverse();
    assert!(plane.is_right_hand());
    plane.rotate(PI / 4.0);
    assert!(plane.is_right_hand());
}

#[test]
fn test_plane_translate_by_normal() {
    let plane = Plane::xy_plane();

    // Translate along positive normal (Z direction)
    let translated = plane.translate_by_normal(5.0);
    assert_eq!(translated.origin().x(), 0.0);
    assert_eq!(translated.origin().y(), 0.0);
    assert_eq!(translated.origin().z(), 5.0);

    // Normal should remain the same
    assert_eq!(translated.z_axis().x(), plane.z_axis().x());
    assert_eq!(translated.z_axis().y(), plane.z_axis().y());
    assert_eq!(translated.z_axis().z(), plane.z_axis().z());

    // Translate along negative normal
    let translated_neg = plane.translate_by_normal(-3.0);
    assert_eq!(translated_neg.origin().x(), 0.0);
    assert_eq!(translated_neg.origin().y(), 0.0);
    assert_eq!(translated_neg.origin().z(), -3.0);

    // Test with YZ plane
    let yz_plane = Plane::yz_plane();
    let yz_translated = yz_plane.translate_by_normal(2.0);
    assert_eq!(yz_translated.origin().x(), 2.0);
    assert_eq!(yz_translated.origin().y(), 0.0);
    assert_eq!(yz_translated.origin().z(), 0.0);
}
