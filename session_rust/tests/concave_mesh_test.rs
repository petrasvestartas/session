use session_rust::collections::Mesh;
use session_rust::primitives::Point;

#[test]
fn test_concave_polygon_earclip() {
    // Create a concave polygon that fits 1x1 screen
    let concave_polygon = vec![
        Point::new(0.0, 0.0, 0.0),    // Bottom-left corner
        Point::new(0.2, 0.0, 0.0),    // Bottom edge
        Point::new(0.3, 0.1, 0.0),    // First concave indentation
        Point::new(0.2, 0.2, 0.0),    // Inner point
        Point::new(0.1, 0.15, 0.0),   // Inner point
        Point::new(0.0, 0.25, 0.0),   // Left edge
        Point::new(0.1, 0.4, 0.0),    // Left edge
        Point::new(0.3, 0.5, 0.0),    // Top-left area
        Point::new(0.4, 0.6, 0.0),    // Second concave indentation
        Point::new(0.3, 0.7, 0.0),    // Inner point
        Point::new(0.2, 0.65, 0.0),   // Inner point
        Point::new(0.1, 0.75, 0.0),   // Left edge
        Point::new(0.3, 0.8, 0.0),    // Top edge
        Point::new(0.6, 0.85, 0.0),   // Top edge
        Point::new(0.8, 0.9, 0.0),    // Top edge
        Point::new(1.0, 0.8, 0.0),    // Top-right corner
        Point::new(1.0, 0.6, 0.0),    // Right edge
        Point::new(0.9, 0.5, 0.0),    // Third concave indentation
        Point::new(0.8, 0.6, 0.0),    // Inner point
        Point::new(0.7, 0.55, 0.0),   // Inner point
        Point::new(0.6, 0.4, 0.0),    // Right edge
        Point::new(0.8, 0.3, 0.0),    // Right edge
        Point::new(0.9, 0.2, 0.0),    // Right edge
        Point::new(1.0, 0.1, 0.0),    // Right edge
        Point::new(0.8, 0.0, 0.0),    // Bottom edge
        Point::new(0.6, 0.0, 0.0),    // Bottom edge
        Point::new(0.4, 0.0, 0.0),    // Bottom edge
    ];
    
    let mesh = Mesh::from_polygons(vec![concave_polygon], None);
    
    // Verify mesh properties
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 27);
    assert_eq!(mesh.number_of_faces(), 25);
    assert_eq!(mesh.number_of_edges(), 51);
    assert_eq!(mesh.euler(), 1); // V - E + F = 27 - 51 + 25 = 1
    
    // Verify all vertices are on boundary (as expected for a simple polygon)
    for vertex_key in mesh.vertex.keys() {
        assert!(mesh.is_vertex_on_boundary(*vertex_key));
    }
    
    // Verify all faces are triangles
    for (_, vertices) in mesh.get_face_data() {
        assert_eq!(vertices.len(), 3);
    }
}

#[test]
fn test_simple_triangle_earclip() {
    // Test with a simple triangle
    let triangle = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![triangle], None);
    
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 3);
    assert_eq!(mesh.number_of_faces(), 1);
    assert_eq!(mesh.number_of_edges(), 3);
    assert_eq!(mesh.euler(), 1);
}

#[test]
fn test_square_earclip() {
    // Test with a square (should triangulate into 2 triangles)
    let square = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![square], None);
    
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 4);
    assert_eq!(mesh.number_of_faces(), 2);
    assert_eq!(mesh.number_of_edges(), 5);
    assert_eq!(mesh.euler(), 1);
}

#[test]
fn test_pentagon_earclip() {
    // Test with a pentagon
    let pentagon = vec![
        Point::new(0.5, 0.0, 0.0),   // Bottom
        Point::new(1.0, 0.4, 0.0),   // Bottom-right
        Point::new(0.8, 1.0, 0.0),   // Top-right
        Point::new(0.2, 1.0, 0.0),   // Top-left
        Point::new(0.0, 0.4, 0.0),   // Bottom-left
    ];
    
    let mesh = Mesh::from_polygons(vec![pentagon], None);
    
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 5);
    assert_eq!(mesh.number_of_faces(), 3); // Pentagon should triangulate into 3 triangles
    assert_eq!(mesh.number_of_edges(), 7);
    assert_eq!(mesh.euler(), 1);
}

#[test]
fn test_concave_polygon_with_holes() {
    // Test with a more complex concave polygon
    let complex_polygon = vec![
        Point::new(0.0, 0.0, 0.0),    // Bottom-left
        Point::new(0.3, 0.0, 0.0),    // Bottom edge
        Point::new(0.4, 0.2, 0.0),    // Concave indentation
        Point::new(0.2, 0.3, 0.0),    // Inner point
        Point::new(0.1, 0.2, 0.0),    // Inner point
        Point::new(0.0, 0.4, 0.0),    // Left edge
        Point::new(0.2, 0.6, 0.0),    // Left edge
        Point::new(0.5, 0.7, 0.0),    // Top edge
        Point::new(0.8, 0.8, 0.0),    // Top edge
        Point::new(1.0, 0.6, 0.0),    // Top-right corner
        Point::new(1.0, 0.3, 0.0),    // Right edge
        Point::new(0.8, 0.2, 0.0),    // Right edge
        Point::new(0.7, 0.0, 0.0),    // Bottom edge
        Point::new(0.5, 0.0, 0.0),    // Bottom edge
    ];
    
    let mesh = Mesh::from_polygons(vec![complex_polygon], None);
    
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 14);
    assert_eq!(mesh.number_of_faces(), 12); // 14-vertex polygon should triangulate into 12 triangles
    assert_eq!(mesh.number_of_edges(), 25);
    assert_eq!(mesh.euler(), 1);
    
    // Verify all faces are triangles
    for (_, vertices) in mesh.get_face_data() {
        assert_eq!(vertices.len(), 3);
    }
}

#[test]
fn test_cross_concave_polygon_earclip() {
    // Test with a cross-like concave polygon
    let cross_polygon = vec![
        Point::new(0.3, 0.0, 0.0),    // Bottom-left
        Point::new(0.7, 0.0, 0.0),    // Bottom-right
        Point::new(0.7, 0.3, 0.0),    // Bottom-right inner
        Point::new(0.8, 0.3, 0.0),    // Right outer
        Point::new(0.8, 0.7, 0.0),    // Right outer
        Point::new(0.7, 0.7, 0.0),    // Right inner
        Point::new(0.7, 1.0, 0.0),    // Top-right
        Point::new(0.3, 1.0, 0.0),    // Top-left
        Point::new(0.3, 0.7, 0.0),    // Top-left inner
        Point::new(0.2, 0.7, 0.0),    // Left outer
        Point::new(0.2, 0.3, 0.0),    // Left outer
        Point::new(0.3, 0.3, 0.0),    // Left inner
    ];
    
    let mesh = Mesh::from_polygons(vec![cross_polygon], None);
    
    // Verify mesh properties
    assert!(!mesh.is_empty());
    assert_eq!(mesh.number_of_vertices(), 12);
    assert_eq!(mesh.number_of_faces(), 10);
    assert_eq!(mesh.number_of_edges(), 21);
    assert_eq!(mesh.euler(), 1); // V - E + F = 12 - 21 + 10 = 1
    
    // Verify all vertices are on boundary (as expected for a simple polygon)
    for vertex_key in mesh.vertex.keys() {
        assert!(mesh.is_vertex_on_boundary(*vertex_key));
    }
    
    // Verify all faces are triangles
    for (_, vertices) in mesh.get_face_data() {
        assert_eq!(vertices.len(), 3);
    }
}

#[test]
fn test_edge_extraction() {
    // Test edge extraction from the triangulated mesh
    let concave_polygon = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.5, 0.5, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![concave_polygon], None);
    
    // Extract edges as lines
    let edges = mesh.extract_edges_as_lines();
    
    // Should have 5 unique edges (4 boundary + 1 diagonal)
    assert_eq!(edges.len(), 5);
    
    // Extract edges as pipe meshes
    let pipe_meshes = mesh.extract_edges_as_pipes(0.05, Some(8));
    assert_eq!(pipe_meshes.len(), 5);
}

#[test]
fn test_mesh_properties() {
    let concave_polygon = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.3, 0.0, 0.0),
        Point::new(0.4, 0.2, 0.0),
        Point::new(0.2, 0.3, 0.0),
        Point::new(0.1, 0.2, 0.0),
        Point::new(0.0, 0.4, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![concave_polygon], None);
    
    // Test vertex positions
    for (vertex_key, vertex_data) in &mesh.vertex {
        let position = mesh.vertex_position(*vertex_key).unwrap();
        assert_eq!(position.x, vertex_data.position.x);
        assert_eq!(position.y, vertex_data.position.y);
        assert_eq!(position.z, vertex_data.position.z);
    }
    
    // Test face normals
    let face_normals = mesh.face_normals();
    assert_eq!(face_normals.len(), mesh.number_of_faces());
    
    // Test vertex normals
    let vertex_normals = mesh.vertex_normals();
    assert_eq!(vertex_normals.len(), mesh.number_of_vertices());
    
    // Test face areas
    for (face_key, _) in mesh.get_face_data() {
        let area = mesh.face_area(face_key);
        assert!(area.is_some());
        assert!(area.unwrap() > 0.0);
    }
} 