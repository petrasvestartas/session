#[cfg(test)]
mod tests {
    use crate::arrow::Arrow;
    use crate::encoders::{json_dump, json_load};
    use crate::line::Line;

    #[test]
    fn test_arrow_new() {
        let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
        let arrow = Arrow::new(line, 1.0);

        assert_eq!(arrow.radius, 1.0);
        assert_eq!(arrow.mesh.number_of_vertices(), 29);
        assert_eq!(arrow.mesh.number_of_faces(), 28);
        assert!(!arrow.guid.is_empty());
        assert_eq!(arrow.name, "my_arrow");
    }

    #[test]
    fn test_arrow_json_serialization() {
        let line = Line::new(0.0, 0.0, 0.0, 5.0, 0.0, 0.0);
        let arrow = Arrow::new(line, 2.0);

        let json = serde_json::to_string(&arrow).unwrap();
        let deserialized: Arrow = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.radius, 2.0);
        assert_eq!(deserialized.mesh.number_of_vertices(), 29);
        assert_eq!(deserialized.mesh.number_of_faces(), 28);
    }

    #[test]
    fn test_arrow_to_json_data() {
        let line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let arrow = Arrow::new(line, 1.5);

        let json_string = arrow.jsondump().unwrap();
        assert!(json_string.contains("Arrow"));
        assert!(json_string.contains("radius"));
        assert!(json_string.contains("1.5"));
    }

    #[test]
    fn test_arrow_from_json_data() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let arrow = Arrow::new(line, 0.5);

        let json_string = arrow.jsondump().unwrap();
        let deserialized = Arrow::jsonload(&json_string).unwrap();

        assert_eq!(deserialized.radius, 0.5);
        assert_eq!(deserialized.mesh.number_of_vertices(), 29);
        assert_eq!(deserialized.mesh.number_of_faces(), 28);
    }

    #[test]
    fn test_arrow_to_json_from_json() {
        let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 8.0);
        let arrow = Arrow::new(line, 1.0);

        let filepath = "test_arrow.json";
        json_dump(&arrow, filepath, true).unwrap();

        let loaded = json_load::<Arrow>(filepath).unwrap();
        assert_eq!(loaded.radius, 1.0);
        assert_eq!(loaded.mesh.number_of_vertices(), 29);
        assert_eq!(loaded.mesh.number_of_faces(), 28);
    }
}
