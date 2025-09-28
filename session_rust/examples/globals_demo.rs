#![allow(static_mut_refs)]

use session_rust::globals;

fn main() {
    println!("=== Simple Unsafe Mutable Globals Demo ===");

    // Show that ALL constants are now mutable (not just SCALE/TOLERANCE)
    unsafe {
        println!("Initial values:");
        println!("  PI: {}", globals::PI);
        println!("  TO_DEGREES: {}", globals::TO_DEGREES);
        println!("  SCALE: {}", globals::SCALE);
        println!("  MEGA: {}", globals::MEGA);
        println!("  EPSILON: {}", globals::EPSILON);
    }

    // Modify ALL globals (even mathematical constants!)
    unsafe {
        globals::PI = 3.0; // Heresy! But now possible
        globals::TO_DEGREES = 60.0; // Wrong, but mutable
        globals::SCALE = 2000.0;
        globals::MEGA = 2e6; // Change metric prefix
        globals::EPSILON = 1e-10; // Change precision
    }

    // Show modified values
    unsafe {
        println!("\nModified values:");
        println!("  PI: {} (was 3.14159...)", globals::PI);
        println!("  TO_DEGREES: {} (was ~57.29)", globals::TO_DEGREES);
        println!("  SCALE: {} (was 1e6)", globals::SCALE);
        println!("  MEGA: {} (was 1e6)", globals::MEGA);
        println!("  EPSILON: {} (was ~2.22e-16)", globals::EPSILON);
    }

    // Reset to defaults
    unsafe {
        globals::PI = std::f64::consts::PI;
        globals::TO_DEGREES = 180.0 / std::f64::consts::PI;
        globals::SCALE = 1e6;
        globals::MEGA = 1e6;
        globals::EPSILON = f64::EPSILON;
    }

    println!("\n✅ All globals reset to defaults");
    println!("🦀 Simple, direct global access like Python/C++!");
}
