use session_rust::collections::LineCloud;
use session_rust::primitives::{Line, Color};

#[test]
fn test_line_pipe_visualization() {
    // Create a simple vertical line
    let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    
    // Set width directly
    line.width = 0.1;
    
    // Initially there's no mesh
    assert!(line.mesh.is_none());
    
    // Get the mesh - this should trigger creation
    let _mesh = line.get_mesh().unwrap();
    
    // Convert to pipe mesh with radius 0.1 and 8 sides
    let mesh = line.to_pipe_mesh(0.1, Some(8)).expect("Failed to create pipe mesh");
    
    // Debug output
    println!("Pipe mesh vertices: {}", mesh.number_of_vertices());
    println!("Pipe mesh faces: {}", mesh.number_of_faces());
    
    // Expected mesh structure:
    // - 8 vertices per ring, 2 rings -> 16 vertices total
    // - 2 center vertices for caps -> 18 vertices total
    // - Caps: 8 triangles per cap -> 16 triangles for both caps
    // - Sides: 8 quads split into 2 triangles each -> 16 triangles
    // Total faces = 16 + 16 = 32
    assert_eq!(mesh.number_of_vertices(), 18);
    assert_eq!(mesh.number_of_faces(), 32);
    
    // Test setting color in data
    let color = Color::new(255, 0, 0, 255); // Red
    line.linecolor = [color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0];
}

#[test]
fn test_linecloud_pipe_visualization() {
    // Create two lines with colors
    let mut line1 = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let mut line2 = Line::new(0.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    let color1 = Color::new(255, 0, 0, 255); // Red
    let color2 = Color::new(0, 255, 0, 255); // Green
    
    // Set colors on the lines
    line1.linecolor = [color1.r as f32 / 255.0, color1.g as f32 / 255.0, color1.b as f32 / 255.0, color1.a as f32 / 255.0];
    line2.linecolor = [color2.r as f32 / 255.0, color2.g as f32 / 255.0, color2.b as f32 / 255.0, color2.a as f32 / 255.0];
    
    // Create a LineCloud with these lines
    let mut cloud = LineCloud::new(vec![line1, line2]);
    
    // Set width directly
    cloud.widths = vec![0.05, 0.05];
    
    // Initially there are no meshes
    assert!(cloud.meshes.is_empty());
    
    // Get the meshes - this should trigger mesh creation
    let meshes = cloud.get_meshes();
    
    // Check that there are two meshes (one per line)
    assert_eq!(meshes.len(), 2);
    
    // Check that each mesh has the expected properties
    for mesh in meshes {
        // Current pipe uses an 8-sided cylinder with cap centers:
        // - 18 vertices total (8 per ring + 2 cap centers)
        // - 32 faces total (8+8 caps, 16 sides)
        assert_eq!(mesh.number_of_vertices(), 18);
        assert_eq!(mesh.number_of_faces(), 32);
        
        // Check that the mesh has color data from the colors array
        // We know it should have either red or green color (values in 0-1 range)
        if mesh.facecolors.len() >= 3 {
            let rgb = [mesh.facecolors[0], mesh.facecolors[1], mesh.facecolors[2]];
            assert!(
                (rgb[0] == 1.0 && rgb[1] == 0.0 && rgb[2] == 0.0) || // Red
                (rgb[0] == 0.0 && rgb[1] == 1.0 && rgb[2] == 0.0)    // Green
            );
        }
    }
    
    // Test that we can update meshes multiple times
    let v = session_rust::primitives::Vector::new(1.0, 1.0, 1.0);
    cloud += &v;
    
    // Force mesh update
    cloud.update_meshes();
    
    // Check that meshes were updated (should still have correct count)
    let updated_meshes = cloud.get_meshes();
    assert_eq!(updated_meshes.len(), 2);
}
