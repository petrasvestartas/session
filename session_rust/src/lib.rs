//! # Session Rust Library
//!
//! A cross-language compatible geometry library providing Point and Color structures
//! with JSON serialization support for interoperability between Rust, Python, and C++.
//!
//! ## Features
//!
//! - Cross-platform 3D point representation
//! - RGBA color management
//! - JSON serialization/deserialization
//! - UUID support for unique identification
//! - Interoperability with Python and C++ implementations
//!
//! ## Example
//!
//! ```rust
//! use session_rust::{Point, Color};
//!
//! let mut point = Point::new(10.0, 20.0, 30.0);
//! point.name = "my_point".to_string();
//! point.pointcolor = Color::new(255, 0, 0, 255);
//!
//! // Save to JSON file
//! point.to_json("point.json").unwrap();
//! ```

/// Point module containing the Point struct and its implementations.
pub mod point;

/// Color module containing the Color struct and its implementations.
pub mod color;

/// A 3D point with visual properties.
///
/// Re-exported from the point module for convenience.
pub use point::Point;

/// An RGBA color representation.
///
/// Re-exported from the color module for convenience.
pub use color::Color;
