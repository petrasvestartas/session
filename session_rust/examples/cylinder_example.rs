use session_rust::{Cylinder, Line};

fn main() {
    println!("=== Cylinder/Pipe Generation Example ===\n");

    // Test 1: Vertical cylinder
    println!("=== Vertical Cylinder ===");
    let vertical_line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
    let vertical_cylinder = Cylinder::new(vertical_line, 1.0);
    let (_v_vertices, v_faces) = vertical_cylinder.mesh.to_vertices_and_faces();

    println!("Faces:");
    for (i, face) in v_faces.iter().enumerate() {
        println!("  {}: {} {} {}", i, face[0], face[1], face[2]);
    }
}
