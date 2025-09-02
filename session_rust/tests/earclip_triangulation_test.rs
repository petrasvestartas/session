use session_rust::collections::Mesh;
use session_rust::primitives::Point;

// Test cases based on COMPAS reference: 
// https://github.com/compas-dev/compas/blob/0dfc019bcdc1dc97d7878b1ab450f00b4f2421c2/tests/compas/geometry/test_triangulation_earclip.py

#[test]
fn test_earclip_polygon_triangle() {
    // Triangle case from COMPAS reference
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![points], None);
    assert_eq!(mesh.number_of_vertices(), 3);
    assert_eq!(mesh.number_of_faces(), 1);
    // Expected triangulation: [[0, 1, 2]]
}

#[test]
fn test_earclip_polygon_square() {
    // Square case from COMPAS reference
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![points], None);
    assert_eq!(mesh.number_of_vertices(), 4);
    assert_eq!(mesh.number_of_faces(), 2);
    // Expected triangulation: [[3, 0, 1], [1, 2, 3]]
}

#[test]
fn test_earclip_polygon_when_reversed() {
    // Test case from COMPAS reference for winding order handling
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(5.0, 0.0, 0.0),
        Point::new(5.0, 5.0, 0.0),
        Point::new(10.0, 5.0, 0.0),
        Point::new(10.0, 15.0, 0.0),
        Point::new(0.0, 10.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![points], None);
    assert_eq!(mesh.number_of_vertices(), 6);
    assert_eq!(mesh.number_of_faces(), 4); // n-2 triangles for n vertices
    // Expected triangulation: [[5, 0, 1], [2, 3, 4], [5, 1, 2], [2, 4, 5]]
}

#[test]
fn test_earclip_polygon_concave() {
    // L-shaped polygon - concave case
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(2.0, 0.0, 0.0),
        Point::new(2.0, 1.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(1.0, 2.0, 0.0),
        Point::new(0.0, 2.0, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![points], None);
    assert_eq!(mesh.number_of_vertices(), 6);
    assert_eq!(mesh.number_of_faces(), 4); // n-2 triangles for n vertices
}

#[test]
fn test_earclip_polygon_pentagon() {
    // Regular pentagon
    let points = vec![
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.309, 0.951, 0.0),
        Point::new(-0.809, 0.588, 0.0),
        Point::new(-0.809, -0.588, 0.0),
        Point::new(0.309, -0.951, 0.0),
    ];
    
    let mesh = Mesh::from_polygons(vec![points], None);
    assert_eq!(mesh.number_of_vertices(), 5);
    assert_eq!(mesh.number_of_faces(), 3); // n-2 triangles for n vertices
}

#[test]
fn test_earclip_polygon_empty() {
    // Empty polygon should return empty mesh
    let empty = vec![];
    let mesh = Mesh::from_polygons(vec![empty], None);
    assert_eq!(mesh.number_of_vertices(), 0);
    assert_eq!(mesh.number_of_faces(), 0);
}

#[test]
fn test_earclip_polygon_degenerate() {
    // Degenerate cases
    let single = vec![Point::new(0.0, 0.0, 0.0)];
    let mesh = Mesh::from_polygons(vec![single], None);
    assert_eq!(mesh.number_of_vertices(), 0);
    assert_eq!(mesh.number_of_faces(), 0);
    
    let two = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    ];
    let mesh = Mesh::from_polygons(vec![two], None);
    assert_eq!(mesh.number_of_vertices(), 0);
    assert_eq!(mesh.number_of_faces(), 0);
}
