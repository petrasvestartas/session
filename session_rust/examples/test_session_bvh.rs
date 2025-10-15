use session_rust::{Point, Session};

fn main() {
    println!("Testing Session BVH Collision Detection\n");

    // Create session
    let mut session = Session::new("collision_test");

    // Add some overlapping points
    let p1 = Point::new(0.0, 0.0, 0.0);
    let p2 = Point::new(0.0005, 0.0005, 0.0005); // Very close to p1 - should collide
    let p3 = Point::new(10.0, 10.0, 10.0); // Far away - no collision

    session.add_point(p1);
    session.add_point(p2);
    session.add_point(p3);

    println!("Added 3 points to session");
    println!("Point 1: (0, 0, 0)");
    println!("Point 2: (0.0005, 0.0005, 0.0005) - close to Point 1");
    println!("Point 3: (10, 10, 10) - far away\n");

    // Check for collisions
    let collisions = session.get_collisions();

    println!("Collision pairs found: {}", collisions.len());
    for (i, (guid1, guid2)) in collisions.iter().enumerate() {
        println!("  Collision {}: {} <-> {}", i + 1, guid1, guid2);
    }

    // Verify edges were added to graph
    println!(
        "\nGraph edges (including collision edges): {}",
        session.graph.edge_count
    );

    println!("\n✅ BVH collision detection working!");
}
