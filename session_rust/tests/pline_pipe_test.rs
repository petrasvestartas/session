use session_rust::primitives::{Point, Pline};

#[test]
fn test_pline_to_pipe_meshes_basic() {
    // Create a simple L-shaped polyline with 3 points
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
    ];
    let pline = Pline::new(points);
    
    // Convert to pipe meshes
    let pipe_meshes = pline.to_pipe_meshes(Some(0.1), Some(8));
    
    // Should have 2 segments (3 points = 2 segments)
    assert_eq!(pipe_meshes.len(), 2);
    
    // Each pipe mesh should have the expected vertex and face counts (8-sided unit cylinder)
    for mesh in &pipe_meshes {
        // 8 vertices per ring, 2 rings + 2 cap centers -> 18 vertices total
        assert_eq!(mesh.number_of_vertices(), 18);
        
        // Faces: caps 2*8=16 + sides 8*2=16 -> 32
        assert_eq!(mesh.number_of_faces(), 32);
    }
}

#[test]
fn test_pline_to_pipe_meshes_with_color() {
    // Create a polyline with color data
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
    ];
    let mut pline = Pline::new(points);
    
    // Set color and width directly
    pline.linecolor = [1.0, 0.0, 0.0, 1.0]; // Red
    pline.width = 0.2;
    
    // Convert to pipe meshes using data width
    let pipe_meshes = pline.to_pipe_meshes(None, None);
    
    // Should have 1 segment
    assert_eq!(pipe_meshes.len(), 1);
    
    // Check that color was applied
    let mesh = &pipe_meshes[0];
    let rgb = [mesh.facecolors[0], mesh.facecolors[1], mesh.facecolors[2]];
    assert_eq!(rgb, [1.0, 0.0, 0.0]); // Colors are in 0-1 range
}

#[test]
fn test_pline_to_pipe_meshes_empty() {
    // Test with empty polyline
    let pline = Pline::new(vec![]);
    let pipe_meshes = pline.to_pipe_meshes(Some(0.1), None);
    assert_eq!(pipe_meshes.len(), 0);
}

#[test]
fn test_pline_to_pipe_meshes_single_point() {
    // Test with single point (no segments)
    let points = vec![Point::new(0.0, 0.0, 0.0)];
    let pline = Pline::new(points);
    let pipe_meshes = pline.to_pipe_meshes(Some(0.1), None);
    assert_eq!(pipe_meshes.len(), 0);
}

#[test]
fn test_pline_to_pipe_meshes_complex() {
    // Create a more complex polyline with 5 points
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 1.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        Point::new(0.0, 0.0, 1.0),
    ];
    let pline = Pline::new(points);
    
    // Convert to pipe meshes with custom parameters
    let pipe_meshes = pline.to_pipe_meshes(Some(0.05), Some(6));
    
    // Should have 4 segments (5 points = 4 segments)
    assert_eq!(pipe_meshes.len(), 4);
    
    // Each pipe mesh should have the expected vertex and face counts (8-sided, sides parameter ignored)
    for mesh in &pipe_meshes {
        assert_eq!(mesh.number_of_vertices(), 18);
        assert_eq!(mesh.number_of_faces(), 32);
    }
}
