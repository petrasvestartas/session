#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{Color, Point, Vector};

    #[test]
    fn test_point_constructor() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point.name, "my_point");
        assert!(!point.guid.to_string().is_empty());
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
        assert_eq!(point.width, 1.0);
        assert_eq!(point.pointcolor.r, Color::white().r);
        assert_eq!(point.pointcolor.g, Color::white().g);
        assert_eq!(point.pointcolor.b, Color::white().b);
        assert_eq!(point.pointcolor.a, Color::white().a);
    }

    #[test]
    fn test_point_equality() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1, p2);
        assert!(!(p1 != p2));

        let p3 = Point::new(1.0, 2.0, 3.0);
        let p4 = Point::new(1.1, 2.0, 3.0);
        assert!(!(p3 == p4));
        assert_ne!(p3, p4);
    }

    #[test]
    fn test_point_to_json_data() {
        let mut point = Point::new(15.5, 25.7, 35.9);
        point.name = "survey_point_A".to_string();
        point.width = 2.5;
        point.pointcolor = Color::new(255, 128, 64, 255);

        let json_string = point.jsondump().unwrap();
        let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(data["type"], "Point");
        assert_eq!(data["name"], "survey_point_A");
        assert_eq!(data["x"], 15.5);
        assert_eq!(data["y"], 25.7);
        assert_eq!(data["z"], 35.9);
        assert_eq!(data["width"], 2.5);
        assert_eq!(data["pointcolor"]["r"], 255);
        assert_eq!(data["pointcolor"]["g"], 128);
        assert_eq!(data["pointcolor"]["b"], 64);
        assert_eq!(data["pointcolor"]["a"], 255);
        assert!(data["guid"].is_string());
    }

    #[test]
    fn test_point_from_json_data() {
        let mut original_point = Point::new(42.1, 84.2, 126.3);
        original_point.name = "control_point_B".to_string();
        original_point.width = 3.0;
        original_point.pointcolor = Color::new(200, 100, 50, 255);

        let json_string = original_point.jsondump().unwrap();
        let restored_point = Point::jsonload(&json_string).unwrap();

        assert_eq!(restored_point.x(), 42.1);
        assert_eq!(restored_point.y(), 84.2);
        assert_eq!(restored_point.z(), 126.3);
        assert_eq!(restored_point.name, "control_point_B");
        assert_eq!(restored_point.width, 3.0);
        assert_eq!(restored_point.pointcolor.r, 200);
        assert_eq!(restored_point.pointcolor.g, 100);
        assert_eq!(restored_point.pointcolor.b, 50);
        assert_eq!(restored_point.pointcolor.a, 255);
        assert_eq!(restored_point.guid, original_point.guid);
    }

    #[test]
    fn test_point_to_json_from_json() {
        let mut original = Point::new(123.45, 678.90, 999.11);
        original.name = "file_test_point".to_string();
        original.width = 4.5;
        original.pointcolor = Color::new(0, 255, 128, 255);
        let filename = "test_point.json";

        json_dump(&original, filename, true).unwrap();
        let loaded = json_load::<Point>(filename).unwrap();

        assert_eq!(loaded.x(), original.x());
        assert_eq!(loaded.y(), original.y());
        assert_eq!(loaded.z(), original.z());
        assert_eq!(loaded.name, original.name);
        assert_eq!(loaded.width, original.width);
        assert_eq!(loaded.pointcolor.r, original.pointcolor.r);
        assert_eq!(loaded.pointcolor.g, original.pointcolor.g);
        assert_eq!(loaded.pointcolor.b, original.pointcolor.b);
        assert_eq!(loaded.pointcolor.a, original.pointcolor.a);
        assert_eq!(loaded.guid, original.guid);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // No-copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_point_getitem() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point[0], 1.0);
        assert_eq!(point[1], 2.0);
        assert_eq!(point[2], 3.0);
    }

    #[test]
    fn test_point_setitem() {
        let mut point = Point::new(1.0, 2.0, 3.0);
        point[0] = 4.0;
        point[1] = 5.0;
        point[2] = 6.0;
        assert_eq!(point.x(), 4.0);
        assert_eq!(point.y(), 5.0);
        assert_eq!(point.z(), 6.0);
    }

    #[test]
    fn test_point_imul() {
        let mut point = Point::new(1.0, 2.0, 3.0);
        point *= 2.0;
        assert_eq!(point.x(), 2.0);
        assert_eq!(point.y(), 4.0);
        assert_eq!(point.z(), 6.0);
    }

    #[test]
    fn test_point_itruediv() {
        let mut point = Point::new(2.0, 4.0, 6.0);
        point /= 2.0;
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
    }

    #[test]
    fn test_point_iadd() {
        let mut point = Point::new(1.0, 2.0, 3.0);
        point += Vector::new(4.0, 5.0, 6.0);
        assert_eq!(point.x(), 5.0);
        assert_eq!(point.y(), 7.0);
        assert_eq!(point.z(), 9.0);
    }

    #[test]
    fn test_point_isub() {
        let mut point = Point::new(5.0, 7.0, 9.0);
        point -= Vector::new(4.0, 5.0, 6.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
        assert_eq!(point.z(), 3.0);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Copy Operators
    ///////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_point_mul() {
        let point = Point::new(1.0, 2.0, 3.0);
        let result = point * 2.0;
        assert_eq!(result.x(), 2.0);
        assert_eq!(result.y(), 4.0);
        assert_eq!(result.z(), 6.0);
    }

    #[test]
    fn test_point_truediv() {
        let point = Point::new(2.0, 4.0, 6.0);
        let result = point / 2.0;
        assert_eq!(result.x(), 1.0);
        assert_eq!(result.y(), 2.0);
        assert_eq!(result.z(), 3.0);
    }

    #[test]
    fn test_point_add() {
        let point = Point::new(1.0, 2.0, 3.0);
        let result = point + Vector::new(4.0, 5.0, 6.0);
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 7.0);
        assert_eq!(result.z(), 9.0);
    }

    #[test]
    fn test_point_sub() {
        let point = Point::new(5.0, 7.0, 9.0);
        let result = point - Point::new(4.0, 5.0, 6.0);
        assert_eq!(result.x(), 1.0);
        assert_eq!(result.y(), 2.0);
        assert_eq!(result.z(), 3.0);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    // Details
    ///////////////////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_point_ccw() {
        let a = Point::new(0.0, 0.0, 0.0);
        let b = Point::new(1.0, 0.0, 0.0);
        let c = Point::new(0.0, 1.0, 0.0);
        assert!(Point::ccw(&a, &b, &c));
        assert!(!Point::ccw(&b, &a, &c));
    }

    #[test]
    fn test_point_mid_point() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let mid = p1.mid_point(&p2);
        assert_eq!((mid.x() * 1000000.0).round() / 1000000.0, 0.5);
        assert_eq!((mid.y() * 1000000.0).round() / 1000000.0, 0.0);
        assert_eq!((mid.z() * 1000000.0).round() / 1000000.0, 0.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        assert_eq!((p1.distance(&p2) * 1000000.0).round() / 1000000.0, 1.0);
    }

    #[test]
    fn test_point_area() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        assert_eq!(Point::area(&points), 0.5);
    }

    #[test]
    fn test_point_centroid_quad() {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let centroid = Point::centroid_quad(&vertices).unwrap();
        assert_eq!((centroid.x() * 1000000.0).round() / 1000000.0, 0.5);
        assert_eq!((centroid.y() * 1000000.0).round() / 1000000.0, 0.5);
        assert_eq!((centroid.z() * 1000000.0).round() / 1000000.0, 0.0);
    }
}
