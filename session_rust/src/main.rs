use session_rust::{Point, Session};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust Session JSON Debug ===");

    let mut session = Session::new("test_session");
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(4.0, 5.0, 6.0);

    session.add_point(point1.clone());
    session.add_point(point2.clone());
    session.add_relationship(&point1.guid, &point2.guid, "test_connection");

    let json = session.to_json_data()?;
    println!("Session JSON:\n{json}");

    Ok(())
}
