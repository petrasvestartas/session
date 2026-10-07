use crate::app::cplane::CPlane;
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

/// Parse coordinate text, a leading `w` included; None when it is not a coordinate.
pub fn parse(text: &str) -> Option<Typed> {
    parse_world(text).map(|(typed, _)| typed)
}

/// Parse coordinate text and say whether a leading `w` asks for world coordinates, e.g. `w500,0,0` or `w@0,0,100`.
pub fn parse_world(text: &str) -> Option<(Typed, bool)> {
    let text = text.trim();

    match text.strip_prefix(['w', 'W']) {
        Some(rest) => Some((parse_plain(rest)?, true)),
        None => Some((parse_plain(text)?, false)),
    }
}

/// A typed absolute point in the world: measured in the fixed construction plane, in world coordinates in View mode or after `w`.
pub fn absolute(text: &str) -> Option<[f64; 3]> {
    let (Typed::Absolute { x, y, z }, world) = parse_world(text)? else {
        return None;
    };
    let uvw = [x, y, z.unwrap_or(0.0)];

    match plane().filter(|_| !world) {
        Some(plane) => {
            let p = plane.world(uvw);
            Some([p[0], p[1], p[2]])
        }
        None => Some(uvw),
    }
}

/// A typed offset in the world: along the fixed construction plane's axes, along the world axes in View mode or after `w`.
pub fn offset(uvw: [f64; 3], world: bool) -> [f64; 3] {
    match plane().filter(|_| !world) {
        Some(plane) => {
            let (o, p) = (plane.origin(), plane.world(uvw));
            [p[0] - o[0], p[1] - o[1], p[2] - o[2]]
        }
        None => uvw,
    }
}

/// A typed coordinate as a point: in the fixed construction plane's coordinates from its origin, else as `resolve` places it on the drawing plane.
pub fn place(
    typed: Typed,
    world: bool,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    let Some(plane) = plane().filter(|_| !world) else {
        // world coordinates; a fixed plane's axes do not apply after `w`
        let (x, y) = match world && plane().is_some() {
            true => CPlane::Xy.axes(),
            false => (x_axis.clone(), y_axis.clone()),
        };
        return resolve(typed, &Point::new(0.0, 0.0, 0.0), &x, &y, previous, along);
    };
    // `base` plus an offset in plane coordinates
    let moved = |base: &Point, uvw: [f64; 3]| {
        let d = offset(uvw, false);
        Point::new(base[0] + d[0], base[1] + d[1], base[2] + d[2])
    };

    match typed {
        Typed::Absolute { x, y, z } => Some(plane.world([x, y, z.unwrap_or(0.0)])),
        Typed::Relative { x, y, z } => Some(moved(previous?, [x, y, z.unwrap_or(0.0)])),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.cloned().unwrap_or_else(|| plane.origin());
            Some(moved(&base, [distance * r.cos(), distance * r.sin(), 0.0]))
        }
        Typed::Distance(_) => resolve(typed, &plane.origin(), x_axis, y_axis, previous, along),
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

/// A typed coordinate as a point on the plane.
pub fn resolve(
    typed: Typed,
    origin: &Point,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    // `base` plus u along x and v along y
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

/// A finite number, or None.
fn number(text: &str) -> Option<f64> {
    let value: f64 = text.trim().parse().ok()?;
    value.is_finite().then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The world XY plane.
    fn plane() -> (Point, Vector, Vector) {
        (
            Point::new(0.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
    }

    /// Absolute, relative, polar and distance all parse.
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

    /// Words, extra parts and NaN are not coordinates.
    #[test]
    fn what_is_not_a_coordinate() {
        assert_eq!(parse("line"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("1,2,3,4"), None);
        assert_eq!(parse("@5"), None, "no direction");
        assert_eq!(parse("nan"), None);
        assert_eq!(parse("inf,0"), None);
    }

    /// Two numbers land on the plane, three in the world.
    #[test]
    fn two_numbers_are_on_the_plane_and_three_are_not() {
        let lifted = Point::new(0.0, 0.0, 9.0);
        let (_, x, y) = plane();
        let on = resolve(parse("2,3").unwrap(), &lifted, &x, &y, None, None).unwrap();
        assert_eq!([on[0], on[1], on[2]], [2.0, 3.0, 9.0]);
        let world = resolve(parse("2,3,0").unwrap(), &lifted, &x, &y, None, None).unwrap();
        assert_eq!([world[0], world[1], world[2]], [2.0, 3.0, 0.0]);
    }

    /// Relative and polar measure from the previous point.
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

    /// A relative form without a previous point gives nothing.
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

    /// On a fixed tilted plane typed points are plane coordinates from its origin; `w` and View mode mean world.
    #[test]
    fn typed_points_follow_a_fixed_construction_plane() {
        let tilted = CPlane::from_3_points(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1000.0, 0.0, 1000.0),
            &Point::new(0.0, 1000.0, 0.0),
        )
        .unwrap();
        let near = |p: [f64; 3], q: [f64; 3]| (0..3).all(|k| (p[k] - q[k]).abs() < 1e-9);
        let h = std::f64::consts::FRAC_1_SQRT_2 * 1000.0;
        let (_, x, y) = plane();
        let at = |text: &str, previous: Option<&Point>| {
            let (typed, world) = parse_world(text).unwrap();
            let p = place(typed, world, &x, &y, previous, None).unwrap();
            [p[0], p[1], p[2]]
        };

        // View mode: world, as before
        set_plane(None);
        assert_eq!(absolute("1000,0,0"), Some([1000.0, 0.0, 0.0]));
        assert!(near(at("1000,0,0", None), [1000.0, 0.0, 0.0]));

        set_plane(Some(tilted));
        assert_eq!(hint(), "plane x,y,z (w for world)");
        assert!(
            near(absolute("1000,0,0").unwrap(), [h, 0.0, h]),
            "along the plane's x axis"
        );
        assert!(
            near(absolute("0,0,1000").unwrap(), [-h, 0.0, h]),
            "z is the plane's normal"
        );
        assert_eq!(
            absolute("w1000,0,0"),
            Some([1000.0, 0.0, 0.0]),
            "w is world"
        );
        assert!(near(at("1000,0,0", None), [h, 0.0, h]));
        assert!(near(at("w1000,0,0", None), [1000.0, 0.0, 0.0]));
        let start = Point::new(h, 0.0, h);
        assert!(
            near(at("@0,500", Some(&start)), [h, 500.0, h]),
            "relative along the plane's y"
        );
        assert!(
            near(at("1000<90", Some(&start)), [h, 1000.0, h]),
            "polar in the plane"
        );
        assert!(
            near(at("w@0,0,100", Some(&start)), [h, 0.0, h + 100.0]),
            "w relative is world"
        );
        assert!(
            near(offset([1000.0, 0.0, 0.0], false), [h, 0.0, h])
                && offset([1.0, 2.0, 3.0], true) == [1.0, 2.0, 3.0]
        );
        set_plane(None);
    }
}
