use crate::cplane::CPlane;
use session_rust::{Point, Vector};
use std::cell::Cell;

thread_local! {
    /// The fixed construction plane typed coordinates are measured in; None in View mode, where they are world coordinates.
    static PLANE: Cell<Option<CPlane>> = const { Cell::new(None) };
}

/// Fix the plane typed coordinates are measured in, None for world coordinates.
pub fn set_plane(plane: Option<CPlane>) {
    PLANE.with(|cell| cell.set(plane));
}

/// The fixed construction plane typed coordinates are measured in, if any.
pub fn plane() -> Option<CPlane> {
    PLANE.with(Cell::get)
}

/// A typed coordinate, not yet placed in the world.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Typed {
    Absolute { x: f64, y: f64, z: Option<f64> }, // `1,2,3` or `1,2` on the plane
    Relative { x: f64, y: f64, z: Option<f64> }, // `@1,2` from the previous point
    Polar { distance: f64, degrees: f64 },       // `5<45` in the plane
    Distance(f64),                               // `5` along the current direction
}

/// How the prompt names typed points: `x,y,z`, or plane coordinates while a construction plane is fixed.
pub fn hint() -> &'static str {
    match plane() {
        Some(_) => "plane x,y,z (w for world)",
        None => "x,y,z",
    }
}

/// Parse coordinate text without the world prefix.
fn parse_plain(text: &str) -> Option<Typed> {
    let text = text.trim();

    if text.is_empty() {
        return None;
    }

    let (body, relative) = match text.strip_prefix('@') {
        Some(rest) => (rest.trim(), true),
        None => (text, false),
    };

    if let Some((d, a)) = body.split_once('<') {
        let distance = number(d)?;
        let degrees = number(a)?;
        return Some(Typed::Polar { distance, degrees });
    }

    let parts: Vec<&str> = body.split(',').map(str::trim).collect();

    match parts.len() {
        1 if relative => None, // `@5` has no direction
        1 => Some(Typed::Distance(number(parts[0])?)),
        2 | 3 => {
            let x = number(parts[0])?;
            let y = number(parts[1])?;
            let z = match parts.get(2) {
                Some(v) => Some(number(v)?),
                None => None,
            };
            Some(if relative {
                Typed::Relative { x, y, z }
            } else {
                Typed::Absolute { x, y, z }
            })
        }
        _ => None,
    }
}

/// A finite number, or None.
fn number(text: &str) -> Option<f64> {
    let value: f64 = text.trim().parse().ok()?;
    value.is_finite().then_some(value)
}

