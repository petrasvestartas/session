use crate::primitive_shape as shape;
use crate::primitive_shape::{ MESH_ONLY, Part, Shape};
use session_rust::{Geometry, Primitives};
use std::rc::Rc;

pub static SHAPE: Shape = Shape {
    name: "Quad Sphere",
    options: MESH_ONLY,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

const QUADS: usize = 8; // per patch side

/// Six kernel patches meshed and welded into one closed mesh.
fn build(part: &Part, _option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Quad Sphere needs a radius".into());
    };
    let patches: Vec<_> = Primitives::quad_sphere(0.0, 0.0, 0.0, radius)
        .iter()
        .map(|patch| Primitives::quad_mesh(patch, QUADS, QUADS))
        .collect();
    let mut mesh = shape::to_mesh(&patches);
    mesh.transform(&part.frame.to_xform());
    mesh.name = "quad sphere".into();
    Ok(Geometry::Mesh(Rc::new(mesh)))
}

