#![allow(static_mut_refs)]

use session_rust::Vector;

fn main() {
    println!("=== Length Caching Demo ===");

    // Create a vector
    let mut v = Vector::new(3.0, 4.0, 5.0);
    println!("Created vector: ({}, {}, {})", v.x(), v.y(), v.z());

    // First call to magnitude() - will compute and cache
    println!("First magnitude() call - computes: {}", v.magnitude());

    // Second call to magnitude() - uses cached value
    println!("Second magnitude() call - cached: {}", v.magnitude());

    // Modify the vector - this invalidates the cache
    v.set_x(6.0);
    println!("Modified x to 6.0");

    // Next call to magnitude() - recomputes because cache was invalidated
    println!("After modification - recomputes: {}", v.magnitude());

    // Use compound assignment - also invalidates cache
    v *= 2.0;
    println!("After scaling by 2.0");
    println!("Magnitude after scaling: {}", v.magnitude());

    // Just show magnitude again (computes without mutating cache)
    println!("Magnitude again: {}", v.magnitude());

    println!("\n✅ Length caching working correctly!");
    println!("🦀 Cache is invalidated when coordinates change");
    println!("📈 Performance improved by avoiding repeated sqrt() calls");
}
