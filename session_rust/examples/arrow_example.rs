use session_rust::{Arrow, Line};

fn main() {
    let line = Line::new(0.0, 0.0, 0.0, 10.0, 0.0, 10.0);
    let arrow = Arrow::new(line, 1.0);

    println!("=== Cylinder/Pipe Generation Example ===\n");

    let (v_vertices, v_faces) = arrow.mesh.to_vertices_and_faces();

    for vertex in &v_vertices {
        println!("{} {} {}", vertex.x(), vertex.y(), vertex.z());
    }

    println!("Faces:");
    for face in &v_faces {
        println!("{} {} {}", face[0], face[1], face[2]);
    }
}
