#[cfg(test)]
mod tests {
    use crate::mesh::{Mesh, NormalWeighting};
    use crate::point::Point;

    #[test]
    fn test_mesh_new() {
        let mesh = Mesh::new();
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
        assert!(mesh.is_empty());
        assert_eq!(mesh.euler(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(1.0, 2.0, 3.0), None);
        assert_eq!(mesh.number_of_vertices(), 1);
        assert!(!mesh.is_empty());

        let pos = mesh.vertex_position(vertex_key).unwrap();
        assert_eq!(pos.x(), 1.0);
        assert_eq!(pos.y(), 2.0);
        assert_eq!(pos.z(), 3.0);
    }

    #[test]
    fn test_add_vertex_with_key() {
        let mut mesh = Mesh::new();
        let vertex_key = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Some(42));
        assert_eq!(vertex_key, 42);
        assert_eq!(mesh.number_of_vertices(), 1);
    }

    #[test]
    fn test_add_face() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _face_key = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
        assert_eq!(mesh.euler(), 1);
    }

    #[test]
    fn test_add_face_invalid() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);

        assert!(mesh.add_face(vec![v0, v1], None).is_none());
        assert!(mesh.add_face(vec![v0, v1, 999], None).is_none());
        assert!(mesh.add_face(vec![v0, v1, v0], None).is_none());
    }

    #[test]
    fn test_face_vertices() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let vertices = mesh.face_vertices(f).unwrap();
        assert_eq!(vertices, &vec![v0, v1, v2]);
    }

    #[test]
    fn test_vertex_neighbors() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        mesh.add_face(vec![v0, v1, v2], None);

        let neighbors = mesh.vertex_neighbors(v0);
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&v1));
        assert!(neighbors.contains(&v2));
    }

    #[test]
    fn test_vertex_faces() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);

        let f1 = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let f2 = mesh.add_face(vec![v1, v3, v2], None).unwrap();

        let faces = mesh.vertex_faces(v1);
        assert_eq!(faces.len(), 2);
        assert!(faces.contains(&f1));
        assert!(faces.contains(&f2));
    }

    #[test]
    fn test_is_vertex_on_boundary() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        mesh.add_face(vec![v0, v1, v2], None);

        assert!(mesh.is_vertex_on_boundary(v0));
        assert!(mesh.is_vertex_on_boundary(v1));
        assert!(mesh.is_vertex_on_boundary(v2));
    }

    #[test]
    fn test_face_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.face_normal(f).unwrap();

        assert!((normal.z() - 1.0).abs() < 1e-10);
        assert!(normal.x().abs() < 1e-10);
        assert!(normal.y().abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normal() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh.vertex_normal(v0).unwrap();

        assert!((normal.z() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_face_area() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let area = mesh.face_area(f).unwrap();

        assert!((area - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_face_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.face_normals();

        assert_eq!(normals.len(), 1);
        assert!(normals.contains_key(&f));
        let normal = normals.get(&f).unwrap();
        assert!((normal.z() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normals() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.vertex_normals();

        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v0));
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));
    }

    #[test]
    fn test_vertex_normal_weighted_area() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Area)
            .unwrap();

        let normal_default = mesh.vertex_normal(v0).unwrap();
        assert!((normal.x() - normal_default.x()).abs() < 1e-10);
        assert!((normal.y() - normal_default.y()).abs() < 1e-10);
        assert!((normal.z() - normal_default.z()).abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normal_weighted_angle() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Angle)
            .unwrap();

        assert!((normal.z() - 1.0).abs() < 1e-10);
        assert!(normal.x().abs() < 1e-10);
        assert!(normal.y().abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normal_weighted_uniform() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normal = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Uniform)
            .unwrap();

        assert!((normal.z() - 1.0).abs() < 1e-10);
        assert!(normal.x().abs() < 1e-10);
        assert!(normal.y().abs() < 1e-10);
    }

    #[test]
    fn test_vertex_normals_weighted() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let _f = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let normals = mesh.vertex_normals_weighted(NormalWeighting::Angle);

        assert_eq!(normals.len(), 3);
        assert!(normals.contains_key(&v0));
        assert!(normals.contains_key(&v1));
        assert!(normals.contains_key(&v2));

        let normal_v0 = normals.get(&v0).unwrap();
        assert!((normal_v0.z() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vertex_angle_in_face() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();

        let angle = mesh.vertex_angle_in_face(v0, f).unwrap();
        assert!((angle - std::f32::consts::PI / 2.0).abs() < 1e-6);

        let angle1 = mesh.vertex_angle_in_face(v1, f).unwrap();
        let angle2 = mesh.vertex_angle_in_face(v2, f).unwrap();
        assert!((angle1 - std::f32::consts::PI / 4.0).abs() < 1e-6);
        assert!((angle2 - std::f32::consts::PI / 4.0).abs() < 1e-6);

        let total_angle = angle + angle1 + angle2;
        assert!((total_angle - std::f32::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_from_polygons_simple() {
        let triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];

        let mesh = Mesh::from_polygons(vec![triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(mesh.number_of_faces(), 1);
        assert_eq!(mesh.number_of_edges(), 3);
    }

    #[test]
    fn test_from_polygons_vertex_merging() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];

        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], None);
        assert_eq!(mesh.number_of_vertices(), 4);
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_precision() {
        let triangle1 = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let triangle2 = vec![
            Point::new(1.0000001, 0.0, 0.0),
            Point::new(0.0, 1.0000001, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];

        let mesh = Mesh::from_polygons(vec![triangle1, triangle2], Some(1e-6));
        assert_eq!(mesh.number_of_vertices(), 4);
        assert_eq!(mesh.number_of_faces(), 2);
    }

    #[test]
    fn test_from_polygons_invalid_polygons() {
        let invalid_polygon = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)];
        let valid_triangle = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];

        let mesh = Mesh::from_polygons(vec![invalid_polygon, valid_triangle], None);
        assert_eq!(mesh.number_of_vertices(), 3);
        assert_eq!(mesh.number_of_faces(), 1);
    }

    #[test]
    fn test_from_polygons_cube() {
        let faces = vec![
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
            ],
            vec![
                Point::new(0.0, 0.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
            ],
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 0.0, 0.0),
            ],
            vec![
                Point::new(0.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(0.0, 1.0, 1.0),
            ],
            vec![
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 1.0, 0.0),
                Point::new(0.0, 1.0, 1.0),
                Point::new(0.0, 0.0, 1.0),
            ],
            vec![
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 1.0),
                Point::new(1.0, 1.0, 1.0),
                Point::new(1.0, 1.0, 0.0),
            ],
        ];

        let mesh = Mesh::from_polygons(faces, None);
        assert_eq!(mesh.number_of_vertices(), 8);
        assert_eq!(mesh.number_of_faces(), 6);
        assert_eq!(mesh.number_of_edges(), 12);
        assert_eq!(mesh.euler(), 2);
    }

    #[test]
    fn test_clear() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);

        assert!(!mesh.is_empty());
        mesh.clear();
        assert!(mesh.is_empty());
        assert_eq!(mesh.number_of_vertices(), 0);
        assert_eq!(mesh.number_of_faces(), 0);
    }

    #[test]
    fn test_vertex_data_color() {
        let mut mesh = Mesh::new();
        let v = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);

        mesh.vertex.get_mut(&v).unwrap().set_color(1.0, 0.5, 0.0);
        let color = mesh.vertex.get(&v).unwrap().color();
        assert_eq!(color, [1.0, 0.5, 0.0]);
    }

    #[test]
    fn test_vertex_data_normal() {
        let mut mesh = Mesh::new();
        let v = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);

        mesh.vertex.get_mut(&v).unwrap().set_normal(0.0, 0.0, 1.0);
        let normal = mesh.vertex.get(&v).unwrap().normal().unwrap();
        assert_eq!(normal, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_json_serialization() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);

        let data = mesh.to_json_data();
        let restored = Mesh::from_json_data(&data).unwrap();

        assert_eq!(restored.number_of_vertices(), 3);
        assert_eq!(restored.number_of_faces(), 1);
        assert_eq!(restored.number_of_edges(), 3);
    }

    #[test]
    fn test_json_file_io() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);

        let filename = "test_mesh.json";
        mesh.to_json(filename).unwrap();
        let loaded = Mesh::from_json(filename).unwrap();

        assert_eq!(loaded.number_of_vertices(), 3);
        assert_eq!(loaded.number_of_faces(), 1);
    }

    // Additional tests based on COMPAS test patterns

    #[test]
    fn test_constructor_with_coordinates() {
        let mut mesh = Mesh::new();
        let a = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let b = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let c = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);
        let d = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![a, b, c, d], None);

        let pos_a = mesh.vertex_position(a).unwrap();
        let pos_b = mesh.vertex_position(b).unwrap();
        let pos_c = mesh.vertex_position(c).unwrap();
        let pos_d = mesh.vertex_position(d).unwrap();

        assert_eq!(pos_a.x(), 0.0);
        assert_eq!(pos_a.y(), 0.0);
        assert_eq!(pos_a.z(), 0.0);
        assert_eq!(pos_b.x(), 1.0);
        assert_eq!(pos_b.y(), 0.0);
        assert_eq!(pos_b.z(), 0.0);
        assert_eq!(pos_c.x(), 1.0);
        assert_eq!(pos_c.y(), 1.0);
        assert_eq!(pos_c.z(), 0.0);
        assert_eq!(pos_d.x(), 0.0);
        assert_eq!(pos_d.y(), 1.0);
        assert_eq!(pos_d.z(), 0.0);
    }

    #[test]
    fn test_from_polygons_multiple() {
        let polygon1 = vec![
            Point::new(1.0, 0.0, 3.0),
            Point::new(1.0, 1.25, 0.0),
            Point::new(1.5, 0.5, 0.0),
        ];
        let polygon2 = vec![
            Point::new(1.0, 0.0, 3.0),
            Point::new(1.0, 5.25, 0.0),
            Point::new(1.5, 0.5, 0.0),
        ];

        let mesh = Mesh::from_polygons(vec![polygon1, polygon2], None);
        assert_eq!(mesh.number_of_faces(), 2);
        assert_eq!(mesh.number_of_vertices(), 4);
        assert_eq!(mesh.number_of_edges(), 5);
    }

    #[test]
    fn test_mesh_copy() {
        let mut mesh1 = Mesh::new();
        let v0 = mesh1.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh1.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh1.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh1.add_face(vec![v0, v1, v2], None);

        let mesh2 = mesh1.clone();
        assert_eq!(mesh1.number_of_faces(), mesh2.number_of_faces());
        assert_eq!(mesh1.number_of_vertices(), mesh2.number_of_vertices());
        assert_eq!(mesh1.number_of_edges(), mesh2.number_of_edges());
    }

    #[test]
    fn test_add_vertex_with_coordinates() {
        let mut mesh = Mesh::new();
        let v = mesh.number_of_vertices();
        let key = mesh.add_vertex(Point::new(0.0, 1.0, 2.0), None);

        let pos = mesh.vertex_position(key).unwrap();
        assert_eq!(pos.x(), 0.0);
        assert_eq!(pos.y(), 1.0);
        assert_eq!(pos.z(), 2.0);
        assert_eq!(mesh.number_of_vertices(), v + 1);
    }

    #[test]
    fn test_add_face_with_vertices() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        let f = mesh.number_of_faces();
        let key = mesh.add_face(vec![v0, v1, v2], None).unwrap();

        let vertices = mesh.face_vertices(key).unwrap();
        assert_eq!(vertices, &vec![v0, v1, v2]);
        assert_eq!(mesh.number_of_faces(), f + 1);
    }

    #[test]
    fn test_is_empty_mesh() {
        let mut mesh = Mesh::new();
        assert!(mesh.is_empty());

        mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        assert!(!mesh.is_empty());
    }

    #[test]
    fn test_mesh_euler_characteristic() {
        // Test Euler characteristic V - E + F = 2 for a closed surface
        let mut mesh = Mesh::new();

        // Create a tetrahedron (4 vertices, 6 edges, 4 faces)
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.5, 1.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(0.5, 0.5, 1.0), None);

        mesh.add_face(vec![v0, v1, v2], None);
        mesh.add_face(vec![v0, v1, v3], None);
        mesh.add_face(vec![v1, v2, v3], None);
        mesh.add_face(vec![v0, v2, v3], None);

        assert_eq!(mesh.number_of_vertices(), 4);
        assert_eq!(mesh.number_of_edges(), 6);
        assert_eq!(mesh.number_of_faces(), 4);
        assert_eq!(mesh.euler(), 2); // V - E + F = 4 - 6 + 4 = 2
    }

    #[test]
    fn test_mesh_boundary_detection() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        mesh.add_face(vec![v0, v1, v2], None);

        // All vertices should be on boundary for a single triangle
        assert!(mesh.is_vertex_on_boundary(v0));
        assert!(mesh.is_vertex_on_boundary(v1));
        assert!(mesh.is_vertex_on_boundary(v2));
    }

    #[test]
    fn test_vertex_data_attributes() {
        let mut mesh = Mesh::new();
        let v = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);

        // Test color attributes
        mesh.vertex.get_mut(&v).unwrap().set_color(0.8, 0.2, 0.6);
        let color = mesh.vertex.get(&v).unwrap().color();
        assert_eq!(color, [0.8, 0.2, 0.6]);

        // Test normal attributes
        mesh.vertex.get_mut(&v).unwrap().set_normal(0.0, 1.0, 0.0);
        let normal = mesh.vertex.get(&v).unwrap().normal().unwrap();
        assert_eq!(normal, [0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_mesh_geometric_properties() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(2.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 2.0, 0.0), None);

        let f = mesh.add_face(vec![v0, v1, v2], None).unwrap();

        // Test face area (should be 2.0 for this right triangle)
        let area = mesh.face_area(f).unwrap();
        assert!((area - 2.0).abs() < 1e-10);

        // Test face normal (should point in +Z direction)
        let normal = mesh.face_normal(f).unwrap();
        assert!((normal.z() - 1.0).abs() < 1e-10);
        assert!(normal.x().abs() < 1e-10);
        assert!(normal.y().abs() < 1e-10);
    }

    #[test]
    fn test_mesh_connectivity_queries() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        let v3 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0), None);

        let f1 = mesh.add_face(vec![v0, v1, v2], None).unwrap();
        let f2 = mesh.add_face(vec![v1, v3, v2], None).unwrap();

        // Test vertex neighbors
        let neighbors_v1 = mesh.vertex_neighbors(v1);
        assert_eq!(neighbors_v1.len(), 3); // v0, v2, v3
        assert!(neighbors_v1.contains(&v0));
        assert!(neighbors_v1.contains(&v2));
        assert!(neighbors_v1.contains(&v3));

        // Test vertex faces
        let faces_v1 = mesh.vertex_faces(v1);
        assert_eq!(faces_v1.len(), 2);
        assert!(faces_v1.contains(&f1));
        assert!(faces_v1.contains(&f2));

        // Test shared vertex v2
        let faces_v2 = mesh.vertex_faces(v2);
        assert_eq!(faces_v2.len(), 2);
        assert!(faces_v2.contains(&f1));
        assert!(faces_v2.contains(&f2));
    }

    #[test]
    fn test_mesh_normal_computation_consistency() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);

        mesh.add_face(vec![v0, v1, v2], None);

        // Test different normal weighting schemes produce consistent results
        let normal_area = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Area)
            .unwrap();
        let normal_angle = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Angle)
            .unwrap();
        let normal_uniform = mesh
            .vertex_normal_weighted(v0, NormalWeighting::Uniform)
            .unwrap();

        // For a single triangle, all weighting schemes should give same result
        assert!((normal_area.x() - normal_angle.x()).abs() < 1e-10);
        assert!((normal_area.y() - normal_angle.y()).abs() < 1e-10);
        assert!((normal_area.z() - normal_angle.z()).abs() < 1e-10);

        assert!((normal_area.x() - normal_uniform.x()).abs() < 1e-10);
        assert!((normal_area.y() - normal_uniform.y()).abs() < 1e-10);
        assert!((normal_area.z() - normal_uniform.z()).abs() < 1e-10);
    }

    #[test]
    fn test_mesh_data_integrity() {
        let mut mesh = Mesh::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0), None);
        mesh.add_face(vec![v0, v1, v2], None);

        // Test JSON round-trip preserves data
        let data = mesh.to_json_data();
        let restored = Mesh::from_json_data(&data).unwrap();

        assert_eq!(mesh.number_of_vertices(), restored.number_of_vertices());
        assert_eq!(mesh.number_of_faces(), restored.number_of_faces());
        assert_eq!(mesh.number_of_edges(), restored.number_of_edges());

        // Test vertex positions are preserved
        let original_pos = mesh.vertex_position(v0).unwrap();
        let restored_pos = restored.vertex_position(v0).unwrap();
        assert_eq!(original_pos.x(), restored_pos.x());
        assert_eq!(original_pos.y(), restored_pos.y());
        assert_eq!(original_pos.z(), restored_pos.z());
    }
}
