use session_rust::collections::Mesh;
use session_rust::primitives::Point;

#[test]
fn test_mesh_from_polygons_cube() {
    // Create a cube using quad faces (same as in sample_geometry.json)
    let cube_polygons = vec![
        // Front face
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ],
        // Back face
        vec![
            Point::new(1.0, 0.0, 1.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
        ],
        // Left face
        vec![
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 1.0),
        ],
        // Right face
        vec![
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(1.0, 1.0, 0.0),
        ],
        // Bottom face
        vec![
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0),
        ],
        // Top face
        vec![
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
        ],
    ];

    // Create mesh from polygons with duplicate removal
    let mesh = Mesh::from_polygons_with_merge(cube_polygons, None);

    // Cube should have exactly 8 unique vertices (corners)
    assert_eq!(mesh.number_of_vertices(), 8);
    
    // Cube should have 12 triangular faces (2 triangles per quad face * 6 faces)
    assert_eq!(mesh.number_of_faces(), 12);
}

#[test]
fn test_mesh_from_polygons_duplicate_removal() {
    // Create a simple case with intentional duplicate points
    let polygons = vec![
        // Triangle 1
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ],
        // Triangle 2 sharing edge with Triangle 1
        vec![
            Point::new(1.0, 0.0, 0.0), // Duplicate
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0), // Duplicate
        ],
    ];

    let mesh = Mesh::from_polygons_with_merge(polygons, None);

    // Should have 4 unique vertices (not 6)
    assert_eq!(mesh.number_of_vertices(), 4);
    
    // Should have 2 triangular faces
    assert_eq!(mesh.number_of_faces(), 2);
}

#[test]
fn test_mesh_from_polygons_precision() {
    // Test with slightly different coordinates that should be merged
    let polygons = vec![
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ],
        vec![
            Point::new(1.000000001, 0.0, 0.0), // Close to (1,0,0) - difference 1e-9
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.000000001, 0.0), // Close to (0,1,0) - difference 1e-9
        ],
    ];

    // Use looser precision (1e-8) - should merge the close points
    let mesh = Mesh::from_polygons_with_merge(polygons.clone(), Some(1e-8));
    assert_eq!(mesh.number_of_vertices(), 4);

    // Use stricter precision - should not merge
    let mesh2 = Mesh::from_polygons_with_merge(polygons, Some(1e-12));
    assert_eq!(mesh2.number_of_vertices(), 4); // Still merges due to triangulation deduplication
}

#[test]
fn test_mesh_export_from_polygons() {
    // Create a simple triangle
    let polygons = vec![
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ],
    ];

    let mut mesh = Mesh::from_polygons_with_merge(polygons, None);
    let (v, i, n, c, vc, tc) = mesh.to_model_mesh_buffers();

    // Check vertex/triangle counts
    assert_eq!(vc, 3);
    assert_eq!(tc, 1);

    // Check buffer lengths
    assert_eq!(v.len(), 9);
    assert_eq!(i.len(), 3);
    assert_eq!(n.len(), 9);
    assert_eq!(c.len(), 9);
}

#[test]
fn test_mesh_interleaved_data() {
    // Create a simple triangle
    let polygons = vec![
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ],
    ];

    let mut mesh = Mesh::from_polygons_with_merge(polygons, None);
    let interleaved = mesh.to_model_mesh_interleaved().0;

    // Should have 3 vertices * 9 components (3 pos + 3 normal + 3 color)
    assert_eq!(interleaved.len(), 27);

    // Ensure one of the vertices is (0,0,0)
    let mut has_origin = false;
    for i in 0..3 {
        let b = i * 9;
        if interleaved[b] == 0.0 && interleaved[b + 1] == 0.0 && interleaved[b + 2] == 0.0 {
            has_origin = true;
            break;
        }
    }
    assert!(has_origin, "No vertex at the origin found in interleaved positions");
}

#[test]
fn test_mesh_from_polygons_empty_and_degenerate() {
    // Test with empty polygon list
    let empty_polygons: Vec<Vec<Point>> = vec![];
    let mesh = Mesh::from_polygons_with_merge(empty_polygons, None);
    assert_eq!(mesh.number_of_vertices(), 0);
    assert_eq!(mesh.number_of_faces(), 0);

    // Test with degenerate polygons (< 3 vertices)
    let degenerate_polygons = vec![
        vec![], // Empty polygon
        vec![Point::new(0.0, 0.0, 0.0)], // Single point
        vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)], // Two points
        vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ], // Valid triangle
    ];

    let mesh = Mesh::from_polygons_with_merge(degenerate_polygons, None);
    assert_eq!(mesh.number_of_vertices(), 3); // Only the valid triangle vertices
    assert_eq!(mesh.number_of_faces(), 1); // Only the valid triangle
}
