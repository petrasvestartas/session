use crate::primitive_shape as shape;
use crate::primitive_shape::{ BREP_MESH, Part, Shape};
use session_rust::{BRep, Geometry};

pub static SHAPE: Shape = Shape {
    name: "Sphere",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

/// A kernel sphere at the center, poles along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Sphere needs a radius".into());
    };
    Ok(shape::solid(
        BRep::create_sphere(radius),
        option,
        &part.frame.to_xform(),
        "sphere",
    ))
}

