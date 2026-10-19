use crate::source_cut::{Cutter, planar, samples};
use session_rust::{
    BRep, BRepRef, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, Plane, Point, Polyline, Vector,
    intersection, split,
};
use std::collections::HashMap;
use std::rc::Rc;

pub const TOLERANCE: f64 = 1e-6; // local units, as Split

/// What a cut leaves of one object: the parts a click can remove.
pub enum Parts {
    Curve {
        curve: NurbsCurve, // the object as a curve
        cuts: Vec<f64>,    // sorted cut parameters inside its domain
    },
    Faces {
        brep: BRep,               // the object with every cut face split
        groups: Vec<Vec<usize>>,  // faces of each part
        meshes: Vec<Mesh>,        // one mesh per face, for clicks
        section: Vec<Vec<Point>>, // the cut edges
    },
    Mesh {
        sides: Vec<Mesh>,         // above and below the plane
        section: Vec<[Point; 2]>, // the cut segments
    },
}

/// One cutter and, for a straight line, the plane it stands for off the object.
pub struct Blade {
    pub cutter: Cutter,       // in the target's frame
    pub fence: Option<Plane>, // a line's vertical plane, in the target's frame
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 { d0 } else { cuts[index - 1] };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near).then(|| trimmed(a, d1)).flatten();
    let head = (b > d0 + near).then(|| trimmed(d0, b)).flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}

