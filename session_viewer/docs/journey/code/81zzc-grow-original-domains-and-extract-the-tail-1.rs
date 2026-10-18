use crate::source_cut::{Cutter, unit_ray};
use session_rust::{
    Geometry, Line, Mesh, NurbsCurve, Point, Polyline, Vector, Xform, intersection, split,
};
use std::rc::Rc;

const NEAR: f64 = 1e-6; // a crossing this close to the end is the end itself

/// Which end of a curve grows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum End {
    Start,
    End,
}

/// How far an end grows, in the curve's own frame.
pub enum Reach {
    Distance(f64),
    Boundaries {
        cutters: Vec<Cutter>,           // curves and planes
        meshes: Vec<(Rc<Mesh>, Xform)>, // meshes, each with the map from the curve's frame into its own
    },
}

/// A grown curve and the piece it gained.
pub struct Grown {
    pub geometry: Geometry, // the whole curve
    pub tail: Vec<Point>,   // the piece added, from the old end
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}

/// Per-control colours only fit the control count they were made for.
fn fit_colors(curve: &mut NurbsCurve) {
    if curve.pointcolors.len() != curve.m_cv_count {
        curve.pointcolors.clear();
    }

    if curve.linecolors.len() + 1 != curve.m_cv_count {
        curve.linecolors.clear();
    }
}

