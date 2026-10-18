use crate::drawing_coords as coords;
use session_rust::{Geometry,Plane,Point,Vector,Xform};
use std::rc::Rc;
/// A plane word, or 3Point with none or three typed points.
pub fn parse(_verb: &str, rest: &[&str]) -> Result<Project, String> {
    let usage = "try Project To Plane XY, CPlane, YZ, ZX or 3Point 0,0,0 1,0,0 0,1,0";
    let word = rest
        .first()
        .map_or("cplane".into(), |word| word.to_ascii_lowercase());
    let plane = match word.as_str() {
        "cplane" => "CPlane",
        "xy" => "XY",
        "yz" => "YZ",
        "zx" | "xz" => "ZX",
        "3point" | "3points" => "3Point",
        _ => return Err(usage.into()),
    };
    let points = rest
        .iter()
        .skip(1)
        .map(|word| match coords::absolute(word) {
            Some(point) => Ok(point),
            None => Err(format!("`{word}` is not a point; use x,y,z")),
        })
        .collect::<Result<Vec<_>, _>>()?;

    match (plane, points.len()) {
        ("3Point", 0 | 3) => {}
        ("3Point", _) => return Err("3Point takes three x,y,z points, or none to pick them".into()),
        (_, 0) => {}
        _ => return Err(usage.into()),
    }

    Ok(Project { plane, points })
}

/// Project the selection onto a named plane, or onto three points.
#[derive(Debug)]
pub struct Project {
    pub plane: &'static str,   // CPlane, XY, YZ, ZX or 3Point
    pub points: Vec<[f64; 3]>, // the three typed points of 3Point
}

/// The plane through three points, x toward the second; refused when they are in a line.
pub fn plane_of(a: &Point, b: &Point, c: &Point) -> Result<Plane, String> {
    let (x, y) = (b - a, c - a);

    if x.cross(&y).magnitude() <= 1e-9 * x.magnitude() * y.magnitude() || x.magnitude() <= 1e-12 {
        return Err("The three points lie on one line; pick a point off the line".into());
    }

    Ok(Plane::from_points(vec![a.clone(), b.clone(), c.clone()]))
}

