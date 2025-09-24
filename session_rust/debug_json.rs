use session_rust::{Session, Point};

fn main() {
    let mut session = Session::new("test_session");
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(4.0, 5.0, 6.0);
    let point1_guid = point1.guid.clone();
    let point2_guid = point2.guid.clone();

    session.add_point(point1);
    session.add_point(point2);
    session.add_edge(&point1_guid, &point2_guid, "test_connection");

    match session.to_json_data() {
        Ok(json) => {
            println!("JSON generated successfully");
            println!("Length: {}", json.len());
            println!("Last 100 characters:");
            let start = if json.len() > 100 { json.len() - 100 } else { 0 };
            println!("{}", &json[start..]);
        }
        Err(e) => {
            println!("Error generating JSON: {}", e);
        }
    }
}
