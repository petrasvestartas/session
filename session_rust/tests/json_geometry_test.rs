use session_rust::collections::Mesh;

#[test]
fn test_load_cube_from_json() {
    // Write a temporary JSON file with the expected structure
    let json = r#"
    {
      "cube": {
        "polygons": [
          {"vertices": [{"x":0,"y":0,"z":0},{"x":1,"y":0,"z":0},{"x":1,"y":1,"z":0},{"x":0,"y":1,"z":0}]},
          {"vertices": [{"x":1,"y":0,"z":1},{"x":0,"y":0,"z":1},{"x":0,"y":1,"z":1},{"x":1,"y":1,"z":1}]},
          {"vertices": [{"x":0,"y":0,"z":1},{"x":0,"y":0,"z":0},{"x":0,"y":1,"z":0},{"x":0,"y":1,"z":1}]},
          {"vertices": [{"x":1,"y":0,"z":0},{"x":1,"y":0,"z":1},{"x":1,"y":1,"z":1},{"x":1,"y":1,"z":0}]},
          {"vertices": [{"x":0,"y":0,"z":1},{"x":1,"y":0,"z":1},{"x":1,"y":0,"z":0},{"x":0,"y":0,"z":0}]},
          {"vertices": [{"x":0,"y":1,"z":0},{"x":1,"y":1,"z":0},{"x":1,"y":1,"z":1},{"x":0,"y":1,"z":1}]}
        ]
      }
    }
    "#;
    let path = "temp_cube.json";
    std::fs::write(path, json).expect("Failed to write temp JSON");

    let mesh = Mesh::from_json_file(path).expect("Failed to load cube from JSON");
    // Clean up
    let _ = std::fs::remove_file(path);

    // Derived counts
    assert_eq!(mesh.number_of_vertices(), 8);
    assert_eq!(mesh.number_of_faces(), 12);

    // Verify vertex positions via the new Mesh API
    // All cube coordinates should be 0 or 1
    for (_fkey, vkeys) in mesh.get_face_data() {
        for &vk in vkeys {
            let p = mesh.vertex_position(vk).expect("vertex exists");
            assert!(
                (p.x == 0.0 || p.x == 1.0) &&
                (p.y == 0.0 || p.y == 1.0) &&
                (p.z == 0.0 || p.z == 1.0),
                "Vertex position should have coords 0.0 or 1.0, got ({},{},{})",
                p.x, p.y, p.z
            );
        }
    }
}

#[test]
fn test_json_file_not_found() {
    // Test handling of non-existent file
    let result = Mesh::from_json_file("nonexistent.json");
    assert!(result.is_err());
}

#[test]
fn test_invalid_json_format() {
    // Create a temporary invalid JSON file for testing
    use std::fs;
    
    let invalid_json = r#"{"invalid": "format"}"#;
    fs::write("temp_invalid.json", invalid_json).expect("Failed to write test file");
    
    let result = Mesh::from_json_file("temp_invalid.json");
    assert!(result.is_err());
    
    // Clean up
    let _ = fs::remove_file("temp_invalid.json");
}
