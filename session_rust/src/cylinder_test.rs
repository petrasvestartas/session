#[cfg(test)]
mod tests {
    use crate::cylinder::Cylinder;
    use crate::line::Line;

    #[test]
    fn test_pipe_new() {
        let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
        let pipe = Cylinder::new(line, 1.0);

        assert_eq!(pipe.radius, 1.0);
        assert_eq!(pipe.mesh.number_of_vertices(), 20);
        assert_eq!(pipe.mesh.number_of_faces(), 20);
        assert!(!pipe.guid.is_empty());
        assert_eq!(pipe.name, "my_cylinder");
    }

    #[test]
    fn test_pipe_json_serialization() {
        let line = Line::new(0.0, 0.0, 0.0, 5.0, 0.0, 0.0);
        let pipe = Cylinder::new(line, 2.0);

        let json = serde_json::to_string(&pipe).unwrap();
        let deserialized: Cylinder = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.radius, 2.0);
        assert_eq!(deserialized.mesh.number_of_vertices(), 20);
        assert_eq!(deserialized.mesh.number_of_faces(), 20);
    }

    #[test]
    fn test_pipe_to_json_data() {
        let line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let pipe = Cylinder::new(line, 1.5);

        let json_string = pipe.to_json_data().unwrap();
        assert!(json_string.contains("Cylinder"));
        assert!(json_string.contains("radius"));
        assert!(json_string.contains("1.5"));
    }

    #[test]
    fn test_pipe_from_json_data() {
        let line = Line::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let pipe = Cylinder::new(line, 0.5);

        let json_string = pipe.to_json_data().unwrap();
        let deserialized = Cylinder::from_json_data(&json_string).unwrap();

        assert_eq!(deserialized.radius, 0.5);
        assert_eq!(deserialized.mesh.number_of_vertices(), 20);
        assert_eq!(deserialized.mesh.number_of_faces(), 20);
    }

    #[test]
    fn test_pipe_to_json_from_json() {
        let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 8.0);
        let pipe = Cylinder::new(line, 1.0);

        let filepath = "test_cylinder.json";
        pipe.to_json(filepath).unwrap();

        let loaded = Cylinder::from_json(filepath).unwrap();
        assert_eq!(loaded.radius, 1.0);
        assert_eq!(loaded.mesh.number_of_vertices(), 20);
        assert_eq!(loaded.mesh.number_of_faces(), 20);
    }
}
