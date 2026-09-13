//! Typed coordinates: what a command line does with the text between the commas.
//!
//! Four forms, resolved against the last point the user gave and the plane they are drawing on:
//!
//! ```text
//!   12,4,0     absolute, world
//!   12,4       absolute, on the construction plane
//!   @3,0       relative to the previous point
//!   @5<90      polar: 5 along the plane, 90 degrees round from its first axis
//!   5          bare distance, along the direction already established
//! ```
//!
//! f64 throughout. A typed coordinate is the one input a user expects to be exact, and every
//! kernel `Point` is f64, so narrowing anywhere here would be a choice to lose what they typed.

use session_rust::{Point, Vector};

/// A parsed coordinate, before it knows where it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Typed {
    /// World coordinates. `z` is `None` when only two were given.
    Absolute { x: f64, y: f64, z: Option<f64> },
    /// An offset from the previous point. `z` is `None` when only two were given.
    Relative { x: f64, y: f64, z: Option<f64> },
    /// A distance and an angle in degrees, in the plane.
    Polar { distance: f64, degrees: f64 },
    /// A distance along a direction the caller already has.
    Distance(f64),
}

/// Parse one line of coordinate text. `None` when it is not a coordinate at all, which is how
/// a command line tells a coordinate from a verb.
pub fn parse(text: &str) -> Option<Typed> {
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
        1 if relative => None, // `@5` alone has no direction; use `@5<0`
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

/// Turn a parsed coordinate into a world point.
///
/// `origin` and the two axes are the construction plane; `previous` is the last point given,
/// and `along` the direction already established, both `None` at the start of a draft. A form
/// that needs something the caller does not have resolves to `None` rather than to a guess.
pub fn resolve(
    typed: Typed,
    origin: &Point,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    let on_plane = |u: f64, v: f64, base: &Point| {
        Point::new(
            base[0] + x_axis[0] * u + y_axis[0] * v,
            base[1] + x_axis[1] * u + y_axis[1] * v,
            base[2] + x_axis[2] * u + y_axis[2] * v,
        )
    };
    match typed {
        Typed::Absolute { x, y, z: Some(z) } => Some(Point::new(x, y, z)),
        Typed::Absolute { x, y, z: None } => Some(on_plane(x, y, origin)),
        Typed::Relative { x, y, z: Some(z) } => {
            let p = previous?;
            Some(Point::new(p[0] + x, p[1] + y, p[2] + z))
        }
        Typed::Relative { x, y, z: None } => Some(on_plane(x, y, previous?)),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.unwrap_or(origin);
            Some(on_plane(distance * r.cos(), distance * r.sin(), base))
        }
        Typed::Distance(d) => {
            let p = previous?;
            let v = along?;
            Some(Point::new(
                p[0] + v[0] * d,
                p[1] + v[1] * d,
                p[2] + v[2] * d,
            ))
        }
    }
}

/// A finite number, or nothing. Rejects the infinities and NaN that `parse::<f64>` accepts,
/// because every one of them reaches a bounding box and makes every later fit meaningless.
fn number(text: &str) -> Option<f64> {
    let value: f64 = text.trim().parse().ok()?;
    value.is_finite().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plane() -> (Point, Vector, Vector) {
        (
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    #[test]
    fn the_four_forms_parse() {
        assert_eq!(
            parse("12,4,2"),
            Some(Typed::Absolute {
                x: 12.0,
                y: 4.0,
                z: Some(2.0)
            })
        );
        assert_eq!(
            parse(" 12 , 4 "),
            Some(Typed::Absolute {
                x: 12.0,
                y: 4.0,
                z: None
            })
        );
        assert_eq!(
            parse("@3,0"),
            Some(Typed::Relative {
                x: 3.0,
                y: 0.0,
                z: None
            })
        );
        assert_eq!(
            parse("@5<90"),
            Some(Typed::Polar {
                distance: 5.0,
                degrees: 90.0
            })
        );
        assert_eq!(parse("7.5"), Some(Typed::Distance(7.5)));
    }

    /// A verb is not a coordinate, and neither is a number that would poison a bounding box.
    #[test]
    fn what_is_not_a_coordinate() {
        assert_eq!(parse("line"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("1,2,3,4"), None);
        assert_eq!(parse("@5"), None, "no direction");
        assert_eq!(parse("nan"), None);
        assert_eq!(parse("inf,0"), None);
    }

    /// Two numbers land on the plane, three land in the world, and the plane can be anywhere.
    #[test]
    fn two_numbers_are_on_the_plane_and_three_are_not() {
        let lifted = Point::new(0.0, 0.0, 9.0);
        let (_, x, y) = plane();
        let on = resolve(parse("2,3").unwrap(), &lifted, &x, &y, None, None).unwrap();
        assert_eq!([on[0], on[1], on[2]], [2.0, 3.0, 9.0]);
        let world = resolve(parse("2,3,0").unwrap(), &lifted, &x, &y, None, None).unwrap();
        assert_eq!([world[0], world[1], world[2]], [2.0, 3.0, 0.0]);
    }

    /// Relative and polar both measure from the previous point; polar falls back to the plane
    /// origin so the very first point of a draft can still be typed as an angle.
    #[test]
    fn relative_forms_measure_from_the_previous_point() {
        let (o, x, y) = plane();
        let prev = Point::new(10.0, 10.0, 0.0);
        let r = resolve(parse("@3,4").unwrap(), &o, &x, &y, Some(&prev), None).unwrap();
        assert_eq!([r[0], r[1]], [13.0, 14.0]);
        let p = resolve(parse("@5<90").unwrap(), &o, &x, &y, Some(&prev), None).unwrap();
        assert!((p[0] - 10.0).abs() < 1e-12 && (p[1] - 15.0).abs() < 1e-12);
        let first = resolve(parse("@5<0").unwrap(), &o, &x, &y, None, None).unwrap();
        assert_eq!([first[0], first[1]], [5.0, 0.0]);
    }

    /// A form that needs something the caller does not have resolves to nothing, not a guess.
    #[test]
    fn a_missing_reference_does_not_resolve() {
        let (o, x, y) = plane();
        assert!(resolve(parse("@3,4").unwrap(), &o, &x, &y, None, None).is_none());
        assert!(resolve(parse("5").unwrap(), &o, &x, &y, None, None).is_none());
        let prev = Point::new(1.0, 0.0, 0.0);
        assert!(resolve(parse("5").unwrap(), &o, &x, &y, Some(&prev), None).is_none());
    }

    /// A millimetre typed a kilometre out is kept exactly.
    #[test]
    fn a_typed_coordinate_is_exact() {
        let (o, x, y) = plane();
        let p = resolve(parse("1000000.001,0,0").unwrap(), &o, &x, &y, None, None).unwrap();
        assert_eq!(p[0], 1_000_000.001);
    }
}
