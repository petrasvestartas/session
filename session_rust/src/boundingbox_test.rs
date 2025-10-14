#[cfg(test)]
mod tests {
    use crate::boundingbox::BoundingBox;
    use crate::encoders::{json_dump, json_load};
    use crate::plane::Plane;
    use crate::point::Point;
    use crate::vector::Vector;

    #[test]
    fn test_box_default() {
        let b = BoundingBox::default();
        assert_eq!(b.center.x(), 0.0);
        assert_eq!(b.center.y(), 0.0);
        assert_eq!(b.center.z(), 0.0);
        assert_eq!(b.x_axis.x(), 1.0);
        assert_eq!(b.y_axis.y(), 1.0);
        assert_eq!(b.z_axis.z(), 1.0);
        assert_eq!(b.half_size.x(), 0.5);
        assert_eq!(b.half_size.y(), 0.5);
        assert_eq!(b.half_size.z(), 0.5);
        assert!(!b.guid.is_empty());
    }

    #[test]
    fn test_box_new() {
        let center = Point::new(1.0, 2.0, 3.0);
        let x_axis = Vector::new(1.0, 0.0, 0.0);
        let y_axis = Vector::new(0.0, 1.0, 0.0);
        let z_axis = Vector::new(0.0, 0.0, 1.0);
        let half_size = Vector::new(2.0, 3.0, 4.0);

        let b = BoundingBox::new(center, x_axis, y_axis, z_axis, half_size);

        assert_eq!(b.center.x(), 1.0);
        assert_eq!(b.center.y(), 2.0);
        assert_eq!(b.center.z(), 3.0);
        assert_eq!(b.half_size.x(), 2.0);
        assert_eq!(b.half_size.y(), 3.0);
        assert_eq!(b.half_size.z(), 4.0);
    }

    #[test]
    fn test_box_from_plane() {
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let b = BoundingBox::from_plane(&plane, 4.0, 6.0, 8.0);

        assert_eq!(b.center.x(), 0.0);
        assert_eq!(b.half_size.x(), 2.0);
        assert_eq!(b.half_size.y(), 3.0);
        assert_eq!(b.half_size.z(), 4.0);
    }

    #[test]
    fn test_box_corners() {
        let center = Point::new(0.0, 0.0, 0.0);
        let x_axis = Vector::new(1.0, 0.0, 0.0);
        let y_axis = Vector::new(0.0, 1.0, 0.0);
        let z_axis = Vector::new(0.0, 0.0, 1.0);
        let half_size = Vector::new(1.0, 1.0, 1.0);

        let b = BoundingBox::new(center, x_axis, y_axis, z_axis, half_size);
        let corners = b.corners();

        assert_eq!(corners.len(), 8);
        assert_eq!(corners[0].x(), 1.0);
        assert_eq!(corners[0].y(), 1.0);
        assert_eq!(corners[0].z(), -1.0);
    }

    #[test]
    fn test_box_two_rectangles() {
        let center = Point::new(0.0, 0.0, 0.0);
        let x_axis = Vector::new(1.0, 0.0, 0.0);
        let y_axis = Vector::new(0.0, 1.0, 0.0);
        let z_axis = Vector::new(0.0, 0.0, 1.0);
        let half_size = Vector::new(1.0, 1.0, 1.0);

        let b = BoundingBox::new(center, x_axis, y_axis, z_axis, half_size);
        let rects = b.two_rectangles();

        assert_eq!(rects.len(), 10);
        assert_eq!(rects[0].x(), rects[4].x());
        assert_eq!(rects[0].y(), rects[4].y());
        assert_eq!(rects[0].z(), rects[4].z());
    }

    #[test]
    fn test_box_collision_overlapping() {
        let box1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        let box2 = BoundingBox::new(
            Point::new(0.5, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );

        assert!(box1.collides_with(&box2));
    }

    #[test]
    fn test_box_collision_separated() {
        let box1 = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );
        let box2 = BoundingBox::new(
            Point::new(5.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 1.0, 1.0),
        );

        assert!(!box1.collides_with(&box2));
    }

    #[test]
    fn test_box_json_serialization() {
        let b = BoundingBox::new(
            Point::new(1.0, 2.0, 3.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(2.0, 3.0, 4.0),
        );

        let data = b.jsondump().unwrap();
        assert!(data.contains("\"type\""));
        assert!(data.contains("\"BoundingBox\""));
        assert!(data.contains("\"center\""));
        assert!(data.contains("\"x_axis\""));
        assert!(data.contains("\"y_axis\""));
        assert!(data.contains("\"z_axis\""));
        assert!(data.contains("\"half_size\""));
        assert!(data.contains("\"guid\""));
        assert!(data.contains("\"name\""));
    }

    #[test]
    fn test_box_json_round_trip() {
        let mut original = BoundingBox::new(
            Point::new(1.0, 2.0, 3.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(2.0, 3.0, 4.0),
        );
        original.name = "test_box".to_string();

        let data = original.jsondump().unwrap();
        let loaded = BoundingBox::jsonload(&data).unwrap();

        assert_eq!(loaded.center.x(), original.center.x());
        assert_eq!(loaded.center.y(), original.center.y());
        assert_eq!(loaded.center.z(), original.center.z());
        assert_eq!(loaded.half_size.x(), original.half_size.x());
        assert_eq!(loaded.half_size.y(), original.half_size.y());
        assert_eq!(loaded.half_size.z(), original.half_size.z());
        assert_eq!(loaded.name, original.name);
    }

    #[test]
    fn test_box_inflate() {
        let mut b = BoundingBox::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 2.0, 3.0),
        );

        b.inflate(0.5);

        assert_eq!(b.half_size.x(), 1.5);
        assert_eq!(b.half_size.y(), 2.5);
        assert_eq!(b.half_size.z(), 3.5);
    }

    #[test]
    fn test_box_from_point() {
        let pt = Point::new(1.0, 2.0, 3.0);
        let b = BoundingBox::from_point(pt, 0.0);

        assert_eq!(b.center.x(), 1.0);
        assert_eq!(b.center.y(), 2.0);
        assert_eq!(b.center.z(), 3.0);
        assert_eq!(b.half_size.x(), 0.0);
        assert_eq!(b.half_size.y(), 0.0);
        assert_eq!(b.half_size.z(), 0.0);
    }

    #[test]
    fn test_box_from_point_with_inflate() {
        let pt = Point::new(1.0, 2.0, 3.0);
        let b = BoundingBox::from_point(pt, 0.5);

        assert_eq!(b.center.x(), 1.0);
        assert_eq!(b.center.y(), 2.0);
        assert_eq!(b.center.z(), 3.0);
        assert_eq!(b.half_size.x(), 0.5);
        assert_eq!(b.half_size.y(), 0.5);
        assert_eq!(b.half_size.z(), 0.5);
    }

    #[test]
    fn test_box_from_points() {
        let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(2.0, 4.0, 6.0)];
        let b = BoundingBox::from_points(&points, 0.0);

        assert_eq!(b.center.x(), 1.0);
        assert_eq!(b.center.y(), 2.0);
        assert_eq!(b.center.z(), 3.0);
        assert_eq!(b.half_size.x(), 1.0);
        assert_eq!(b.half_size.y(), 2.0);
        assert_eq!(b.half_size.z(), 3.0);
    }

    #[test]
    fn test_box_from_points_with_inflate() {
        let points = vec![Point::new(0.0, 0.0, 0.0), Point::new(2.0, 4.0, 6.0)];
        let b = BoundingBox::from_points(&points, 0.5);

        assert_eq!(b.center.x(), 1.0);
        assert_eq!(b.center.y(), 2.0);
        assert_eq!(b.center.z(), 3.0);
        assert_eq!(b.half_size.x(), 1.5);
        assert_eq!(b.half_size.y(), 2.5);
        assert_eq!(b.half_size.z(), 3.5);
    }

    #[test]
    fn test_box_from_line() {
        let line = crate::line::Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let b = BoundingBox::from_line(&line, 0.0);

        assert_eq!(b.center.x(), 5.0);
        assert_eq!(b.center.y(), 0.0);
        assert_eq!(b.center.z(), 0.0);
        assert_eq!(b.half_size.x(), 5.0);
    }

    #[test]
    fn test_box_from_line_with_inflate() {
        let line = crate::line::Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let b = BoundingBox::from_line(&line, 1.0);

        assert_eq!(b.center.x(), 5.0);
        assert_eq!(b.center.y(), 0.0);
        assert_eq!(b.center.z(), 0.0);
        assert_eq!(b.half_size.x(), 6.0);
        assert_eq!(b.half_size.y(), 1.0);
        assert_eq!(b.half_size.z(), 1.0);
    }

    #[test]
    fn test_box_from_polyline() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let polyline = crate::polyline::Polyline::new(points);
        let b = BoundingBox::from_polyline(&polyline, 0.0);

        assert_eq!(b.center.x(), 0.5);
        assert_eq!(b.center.y(), 0.5);
        assert_eq!(b.center.z(), 0.0);
        assert_eq!(b.half_size.x(), 0.5);
        assert_eq!(b.half_size.y(), 0.5);
        assert_eq!(b.half_size.z(), 0.0);
    }

    #[test]
    fn test_box_from_polyline_with_inflate() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let polyline = crate::polyline::Polyline::new(points);
        let b = BoundingBox::from_polyline(&polyline, 0.5);

        assert_eq!(b.center.x(), 0.5);
        assert_eq!(b.center.y(), 0.5);
        assert_eq!(b.center.z(), 0.0);
        assert_eq!(b.half_size.x(), 1.0);
        assert_eq!(b.half_size.y(), 1.0);
        assert_eq!(b.half_size.z(), 0.5);
    }

    #[test]
    fn test_box_to_json_from_json() {
        let original = BoundingBox::new(
            Point::new(1.0, 2.0, 3.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(2.0, 3.0, 4.0),
        );
        let filename = "test_boundingbox.json";

        json_dump(&original, filename, true).unwrap();
        let loaded = json_load::<BoundingBox>(filename).unwrap();

        assert_eq!(loaded.center.x(), original.center.x());
        assert_eq!(loaded.center.y(), original.center.y());
        assert_eq!(loaded.center.z(), original.center.z());
        assert_eq!(loaded.half_size.x(), original.half_size.x());
        assert_eq!(loaded.half_size.y(), original.half_size.y());
        assert_eq!(loaded.half_size.z(), original.half_size.z());
        assert_eq!(loaded.name, original.name);
        assert_eq!(loaded.guid, original.guid);
    }
}
