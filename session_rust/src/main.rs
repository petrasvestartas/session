use session_rust::Point;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust Point JSON Demo ===");
    
    // Create a point
    let point = Point::new(1.5, 2.5, 3.5);
    println!("Created point: {}", point);
    
    // Show JSON serialization output
    let json_data = point.to_json_data()?;
    println!("\nSerialized JSON:");
    println!("{}", json_data);
    
    // Test deserialization from JSON string
    let loaded_point = Point::from_json_data(&json_data)?;
    println!("\nDeserialized point: {}", loaded_point);
    
    // Also save to file
    let filename = "point_rust.json";
    point.to_json(filename)?;
    println!("\nAlso saved to file: {}", filename);
    
    Ok(())
}
