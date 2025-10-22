use serde::Deserialize;
use session_rust::{BoundingBox, Point, Vector, BVH};
use std::fs;

#[derive(Deserialize)]
struct BoxData {
    center: [f64; 3],
    half_size: [f64; 3],
}

fn main() {
    // Load test data from JSON file
    let data =
        fs::read_to_string("test_boxes_data.json").expect("Failed to read test_boxes_data.json");
    let boxes_data: Vec<BoxData> = serde_json::from_str(&data).expect("Failed to parse JSON");

    println!("Loaded {} boxes from test data", boxes_data.len());

    // Create boxes from data
    let mut boxes = Vec::new();
    for data in boxes_data {
        let center = Point::new(data.center[0], data.center[1], data.center[2]);
        let half_size = Vector::new(data.half_size[0], data.half_size[1], data.half_size[2]);
        let bbox = BoundingBox::new(
            center,
            Vector::new(1.0, 0.0, 0.0), // X axis
            Vector::new(0.0, 1.0, 0.0), // Y axis
            Vector::new(0.0, 0.0, 1.0), // Z axis
            half_size,
        );
        boxes.push(bbox);
    }

    // Print min/max corners using new API (first 5 boxes only)
    println!("\nFirst 5 boxes:");
    for (i, bbox) in boxes.iter().take(5).enumerate() {
        let min_corner = bbox.min_point();
        let max_corner = bbox.max_point();
        println!(
            "Box {} - Min: ({:.6}, {:.6}, {:.6}), Max: ({:.6}, {:.6}, {:.6})",
            i + 1,
            min_corner.x(),
            min_corner.y(),
            min_corner.z(),
            max_corner.x(),
            max_corner.y(),
            max_corner.z()
        );
    }

    // Use BVH for collision detection
    println!("\nBuilding BVH and checking collisions...");
    let bvh = BVH::from_boxes(&boxes, 100.0);
    let (collisions, colliding_indices, check_count) = bvh.check_all_collisions(&boxes);

    println!("\nNumber of collisions: {}", collisions.len());
    println!("Number of colliding objects: {}", colliding_indices.len());
    println!("Check count: {check_count}");

    // Print first 10 collisions
    println!("\nFirst 10 collisions:");
    for (i, (a, b)) in collisions.iter().take(10).enumerate() {
        println!("  {}. Box {} <-> Box {}", i + 1, a, b);
    }

    // Print first 20 colliding indices
    print!("\nFirst 20 colliding indices: [");
    for (i, idx) in colliding_indices.iter().take(20).enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{idx}");
    }
    println!("]");
}
