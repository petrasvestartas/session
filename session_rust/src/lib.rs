//! Cross-language geometry library with Point, Color, and Vector types.
//! Supports JSON serialization for interoperability between Rust, Python, and C++.

// Module declarations - makes modules publicly accessible
// Usage: session_rust::point::Point
#![allow(static_mut_refs)]

pub mod arrow;
pub mod boundingbox;
pub mod bvh;
#[cfg(test)]
mod bvh_test;
pub mod color;
pub mod cylinder;
pub mod edge;
pub mod encoders;
pub mod graph;
pub mod intersection;
#[cfg(test)]
mod intersection_test;
pub mod line;
pub mod mesh;
pub mod obj;
pub mod objects;
pub mod plane;
pub mod point;
pub mod pointcloud;
pub mod polyline;
pub mod quaternion;
pub mod session;
pub mod tolerance;
pub mod tree;
pub mod treenode;
pub mod vector;
pub mod vertex;
pub mod xform;

pub use arrow::Arrow;
pub use boundingbox::BoundingBox;
pub use bvh::BVH;
pub use color::Color;
pub use cylinder::Cylinder;
pub use edge::Edge;
pub use graph::Graph;
pub use line::Line;
pub use mesh::Mesh;
pub use obj::{read_obj, write_obj};
pub use objects::Objects;
pub use plane::Plane;
pub use point::Point;
pub use pointcloud::PointCloud;
pub use polyline::Polyline;
pub use quaternion::Quaternion;
pub use session::{Geometry, Session};
pub use tolerance::Tolerance;
pub use tree::Tree;
pub use treenode::TreeNode;
pub use vector::Vector;
pub use vertex::Vertex;
pub use xform::Xform;
