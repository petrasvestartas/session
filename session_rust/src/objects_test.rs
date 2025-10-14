#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::{Objects, Point};

    #[test]
    fn test_objects_constructor() {
        let objects = Objects::new();
        assert_eq!(objects.name, "my_objects");
        assert!(!objects.guid.is_empty());
        assert_eq!(objects.points.len(), 0);
    }

    #[test]
    fn test_objects_to_json_data() {
        let mut objects = Objects::new();
        let point1 = Point::new(1.0, 2.0, 3.0);
        let point2 = Point::new(4.0, 5.0, 6.0);
        let point3 = Point::new(7.0, 8.0, 9.0);
        objects.points = vec![point1, point2, point3];

        let json_result = objects.jsondump();
        assert!(json_result.is_ok());

        let json_data = json_result.unwrap();
        assert!(json_data.contains("\"type\": \"Objects\""));
        assert!(json_data.contains("\"name\": \"my_objects\""));
        assert!(json_data.contains("\"guid\""));
        assert!(json_data.contains("\"points\""));
        // Check that we have 3 points in the JSON
        let point_count = json_data.matches("\"type\": \"Point\"").count();
        assert_eq!(point_count, 3);
    }

    #[test]
    fn test_objects_from_json_data() {
        let mut objects = Objects::new();
        let point1 = Point::new(10.0, 20.0, 30.0);
        let point2 = Point::new(40.0, 50.0, 60.0);
        objects.points = vec![point1, point2];

        let json_data = objects.jsondump().unwrap();
        let objects2_result = Objects::jsonload(&json_data);
        assert!(objects2_result.is_ok());

        let objects2 = objects2_result.unwrap();
        assert_eq!(objects2.name, "my_objects");
        assert_eq!(objects2.points.len(), 2);
        assert_eq!(objects2.points[0].x(), 10.0);
        assert_eq!(objects2.points[1].z(), 60.0);
    }

    #[test]
    fn test_objects_to_json_from_json() {
        let mut objects = Objects::new();
        let point1 = Point::new(100.0, 200.0, 300.0);
        let point2 = Point::new(400.0, 500.0, 600.0);
        let point3 = Point::new(700.0, 800.0, 900.0);
        objects.points = vec![point1, point2, point3];
        let filename = "test_objects.json";

        // Save to file
        let save_result = json_dump(&objects, filename, true);
        assert!(save_result.is_ok());

        // Load from file
        let loaded_result = json_load::<Objects>(filename);
        assert!(loaded_result.is_ok());

        let loaded_objects = loaded_result.unwrap();
        assert_eq!(loaded_objects.name, objects.name);
        assert_eq!(loaded_objects.points.len(), objects.points.len());
        assert_eq!(loaded_objects.points[0].x(), objects.points[0].x());
        assert_eq!(loaded_objects.points[2].z(), objects.points[2].z());
    }
}
