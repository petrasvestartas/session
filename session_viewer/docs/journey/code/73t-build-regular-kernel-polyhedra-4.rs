use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use session_rust::{Geometry, Mesh, Point};
use std::sync::OnceLock;

pub static SHAPE: Shape = Shape {
    name: "Octahedron",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline,
    build,
};

static UNIT: OnceLock<Vec<Vec<[f64; 3]>>> = OnceLock::new(); // faces at radius 1

/// The kernel mesh of edge length `edge`.
fn make(edge: f64) -> Mesh {
    session_rust::Primitives::octahedron(edge)
}

/// The faces at the radius.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    shape::polyhedron_outline(part, shape::unit_faces(&UNIT, make))
}

/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "octahedron")
}

