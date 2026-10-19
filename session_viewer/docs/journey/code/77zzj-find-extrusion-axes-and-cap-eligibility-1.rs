use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{Picked, checked, count, loop_normal, planar};
use session_rust::{
    BRep, BRepOrientation, BRepRef, Geometry, NurbsCurve, NurbsSurface, Point, Polyline,
    Primitives, Vector, Xform,
};
use std::rc::Rc;

const F: BRepOrientation = BRepOrientation::Forward;
const R: BRepOrientation = BRepOrientation::Reversed;

/// The first curve's start and its plane's normal, facing like the construction plane; else that plane's normal.
fn axis(input: &Input) -> (Point, Vector) {
    let Some(first) = input.curves.first().and_then(|curves| curves.first()) else {
        return (Point::new(0.0, 0.0, 0.0), input.normal.clone());
    };
    let outline = first.outline();
    let normal = loop_normal(&outline)
        .filter(|normal| planar(&outline, normal))
        .map(|normal| {
            if normal.dot(&input.normal) < 0.0 {
                -normal
            } else {
                normal
            }
        })
        .unwrap_or_else(|| input.normal.clone());
    (first.curve.point_at_start(), normal)
}

/// Why a cap was left off, if it was.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Skip {
    None,
    Open,
    Bent,
    Flat,
}

/// The normal of a closed planar outline, signed toward `offset`; why not when it cannot be capped.
fn cap_normal(outline: &[Point], closed: bool, offset: &Vector) -> Result<Vector, Skip> {
    if !closed {
        return Err(Skip::Open);
    }

    let normal = loop_normal(outline).ok_or(Skip::Bent)?;

    if !planar(outline, &normal) {
        return Err(Skip::Bent);
    }

    let along = normal.dot(offset);

    if along.abs() <= 1e-9 * offset.magnitude() {
        return Err(Skip::Flat);
    }

    Ok(if along < 0.0 { -normal } else { normal })
}

