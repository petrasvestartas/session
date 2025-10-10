use session_rust::{Arrow, Color, Cylinder, Line};

fn main() {
    let line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 10.0);
    let mut arrow = Arrow::new(line, 1.0);

    println!("Arrow mesh color collections:");
    println!("  Vertex colors: {}", arrow.mesh.pointcolors.len());
    println!("  Face colors: {}", arrow.mesh.facecolors.len());
    println!("  Edge colors: {}", arrow.mesh.linecolors.len());
    println!("  Edge widths: {}", arrow.mesh.widths.len());

    arrow.mesh.set_vertex_color(0, Color::new(255, 0, 0, 255));
    arrow.mesh.set_face_color(0, Color::new(0, 255, 0, 255));
    arrow.mesh.set_edge_color(0, Color::new(0, 0, 255, 255));
    arrow.mesh.set_edge_width(0, 2.5);

    println!("\nAfter setting colors:");
    println!("  First vertex color: {:?}", arrow.mesh.pointcolors[0]);
    println!("  First face color: {:?}", arrow.mesh.facecolors[0]);
    println!("  First edge color: {:?}", arrow.mesh.linecolors[0]);
    println!("  First edge width: {}", arrow.mesh.widths[0]);

    let cylinder_line = Line::new(0.0, 0.0, 0.0, 5.0, 0.0, 0.0);
    let cylinder = Cylinder::new(cylinder_line, 0.5);

    println!("\nCylinder mesh color collections:");
    println!("  Vertex colors: {}", cylinder.mesh.pointcolors.len());
    println!("  Face colors: {}", cylinder.mesh.facecolors.len());
    println!("  Edge colors: {}", cylinder.mesh.linecolors.len());
    println!("  Edge widths: {}", cylinder.mesh.widths.len());
}
