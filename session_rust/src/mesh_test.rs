#[cfg(test)]
mod tests {
    use crate::encoders::{json_dump, json_load};
    use crate::mesh::Mesh;
    use crate::point::Point;

    #[test]
    fn test_mesh_constructor() {
        let mesh = Mesh::new();
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert!(mesh.is_empty());
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        let _vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
        assert_eq!(mesh.number_of_vertices(), 1);
        assert!(!mesh.is_empty());
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        assert_eq!(mesh.number_of_faces(), 1);
    }

    #[test]
    fn test_mesh_json_roundtrip() {
        let mut original = Mesh::new();
        let v0 = original.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = original.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = original.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let _face_key = original.add_face(vec![v0, v1, v2], None).unwrap();

        let json_data = original.jsondump();
        let loaded = Mesh::jsonload(&json_data).unwrap();

        assert_eq!(loaded.number_of_vertices(), original.number_of_vertices());
        assert_eq!(loaded.number_of_faces(), original.number_of_faces());

        // File I/O test
        json_dump(&original, "test_mesh.json", true).unwrap();
        let from_file: Mesh = json_load("test_mesh.json").unwrap();
        assert_eq!(
            from_file.number_of_vertices(),
            original.number_of_vertices()
        );
    }
}
