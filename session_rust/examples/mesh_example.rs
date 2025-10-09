use session_rust::{Mesh, Point};

fn main() {
    println!("=== Mesh Example ===\n");

    // Create a simple triangle mesh
    let mut mesh = Mesh::new();

    // Add vertices (returns sparse keys like 1, 2, 3)
    let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), None);
    let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), None);
    let v2 = mesh.add_vertex(Point::new(0.5, 1.0, 0.0), None);
    let v3 = mesh.add_vertex(Point::new(0.5, 0.5, 1.0), None);

    println!("Added vertices with keys: {v0}, {v1}, {v2}, {v3}");

    // Add faces using the sparse vertex keys
    mesh.add_face(vec![v0, v1, v2], None); // Bottom triangle
    mesh.add_face(vec![v0, v1, v3], None); // Side 1
    mesh.add_face(vec![v1, v2, v3], None); // Side 2
    mesh.add_face(vec![v2, v0, v3], None); // Side 3

    println!(
        "Mesh has {} vertices and {} faces\n",
        mesh.number_of_vertices(),
        mesh.number_of_faces()
    );

    // Use COMPAS-style method to get sequential indices (0-3)
    let (vertices, faces) = mesh.to_vertices_and_faces();

    println!("Vertices (sequential indices 0-{}):", vertices.len() - 1);
    for (i, vertex) in vertices.iter().enumerate() {
        println!("  {}: {} {} {}", i, vertex.x(), vertex.y(), vertex.z());
    }

    println!(
        "\nFaces (with remapped sequential indices 0-{}):",
        vertices.len() - 1
    );
    for (i, face) in faces.iter().enumerate() {
        println!("  Face {}: {} {} {}", i, face[0], face[1], face[2]);
    }
}
