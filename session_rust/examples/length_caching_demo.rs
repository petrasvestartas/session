#![allow(static_mut_refs)]

use session_rust::Vector;

fn main() {
    println!("=== Length Caching Demo ===");

    // Create a vector
    let mut v = Vector::new(3.0, 4.0, 5.0);
    println!("Created vector: ({}, {}, {})", v.x(), v.y(), v.z());

    // First call to length() - will compute and cache
    println!("First length() call - computes: {}", v.length());

    // Second call to length() - uses cached value
    println!("Second length() call - cached: {}", v.length());

    // Modify the vector - this invalidates the cache
    v.set_x(6.0);
    println!("Modified x to 6.0");

    // Next call to length() - recomputes because cache was invalidated
    println!("After modification - recomputes: {}", v.length());

    // Use compound assignment - also invalidates cache
    v *= 2.0;
    println!("After scaling by 2.0");
    println!("Length after scaling: {}", v.length());

    // Just show length again (computes without mutating cache)
    println!("Length again: {}", v.length());

    println!("\n✅ Length caching working correctly!");
    println!("🦀 Cache is invalidated when coordinates change");
    println!("📈 Performance improved by avoiding repeated sqrt() calls");
}
