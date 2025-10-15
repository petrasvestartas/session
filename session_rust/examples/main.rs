use session_rust::{BoundingBox, Line, Point, Session, Vector};

fn main() {
    let mut session = Session::new("demo");

    // Add geometry with some overlaps, some separated
    session.add_point(Point::new(0.0, 0.0, 0.0)); // Point 1
    session.add_point(Point::new(0.0005, 0.0, 0.0)); // Point 2 - collides with Point 1
    session.add_line(Line::new(0.0, 0.0, 0.0, 0.1, 0.1, 0.1)); // Line 1 - collides with both points
    session.add_line(Line::new(5.0, 5.0, 5.0, 5.1, 5.1, 5.1)); // Line 2 - far away
    session.add_bbox(BoundingBox::new(
        Point::new(10.0, 10.0, 10.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.5, 0.5, 0.5), // Box - far away
    ));

    // Detect collisions
    let collisions = session.get_collisions();
    println!(
        "Objects: {}, Collisions: {}",
        session.lookup.len(),
        collisions.len()
    );

    // Print graph edges
    println!("\nGraph edges:");
    for (node, edges) in &session.graph.edges {
        for (neighbor, edge) in edges {
            println!(
                "  {}... -> {}... [{}]",
                &node[..8],
                &neighbor[..8],
                edge.attribute
            );
        }
    }
}
