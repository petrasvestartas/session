//! Cross-language geometry library with Point, Color, and Vector types.
//! Supports JSON serialization for interoperability between Rust, Python, and C++.

// Module declarations - makes modules publicly accessible
// Usage: session_rust::point::Point
pub mod color;
pub mod graph;
pub mod objects;
pub mod point;
pub mod session;
pub mod tree;
pub mod vector;

// Test modules
pub mod color_test;
pub mod graph_test;
pub mod objects_test;
pub mod point_test;
pub mod session_test;
pub mod tree_test;
pub mod vector_test;

// Re-exports - creates convenient shortcuts at crate root
// Usage: session_rust::Point (instead of session_rust::point::Point)
pub use color::Color;
pub use graph::Edge;
pub use graph::Graph;
pub use graph::Vertex;
pub use objects::Objects;
pub use point::Point;
pub use session::Session;
pub use tree::Tree;
pub use tree::TreeNode;
pub use vector::Vector;
