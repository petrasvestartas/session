//! Cross-language geometry library with Point, Color, and Vector types.
//! Supports JSON serialization for interoperability between Rust, Python, and C++.

// Module declarations - makes modules publicly accessible
// Usage: session_rust::point::Point
#![allow(static_mut_refs)]

pub mod color;
pub mod graph;
pub mod line;
pub mod objects;
pub mod plane;
pub mod point;
pub mod polyline;
pub mod quaternion;
pub mod session;
pub mod tolerance;
pub mod tree;
pub mod vector;
pub mod xform;

#[cfg(test)]
#[path = "polyline_test.rs"]
mod polyline_test;

pub use color::Color;
pub use graph::Graph;
pub use graph::Vertex;
pub use line::Line;
pub use objects::Objects;
pub use plane::Plane;
pub use point::Point;
pub use polyline::Polyline;
pub use quaternion::Quaternion;
pub use session::Session;
pub use tolerance::Tolerance;
pub use tree::Tree;
pub use tree::TreeNode;
pub use vector::Vector;
pub use xform::Xform;
