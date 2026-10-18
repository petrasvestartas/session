use crate::gather_input::{Input, Made, Recipe, Step};
use crate::surfacing::{checked, count, curves, diagonal};
use session_rust::nurbsknot::{CurveInterpStyle, CurveNurbsKnotStyle};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

const DENSITY: usize = 6; // samples per span between crossings
const MOST: usize = 64; // samples per direction at most

/// Where two curves cross: the parameter on each, when they come within `tolerance`.
fn crossing(
    a: &NurbsCurve,
    b: &NurbsCurve,
    boxes: (&[[f64; 3]; 2], &[[f64; 3]; 2]),
    tolerance: f64,
) -> Option<(f64, f64)> {
    let (one, two) = boxes;

    if (0..3).any(|k| one[0][k] > two[1][k] + tolerance || two[0][k] > one[1][k] + tolerance) {
        return None;
    }

    let (s, t) = a.closest_parameters_curve(b);
    (a.point_at(s).distance(&b.point_at(t), None) <= tolerance).then_some((s, t))
}

/// The low and high corners of a curve's control points.
fn bounds(curve: &NurbsCurve) -> [[f64; 3]; 2] {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in (0..curve.cv_count()).filter_map(|i| curve.get_cv(i)) {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }

    [low, high]
}

/// The curve through `points` at even parameters.
fn interpolated(points: &[Point]) -> NurbsCurve {
    NurbsCurve::create_interpolated(
        points,
        CurveNurbsKnotStyle::Uniform,
        CurveInterpStyle::Rhino,
    )
}

/// `curve` at `x` of its domain.
fn at(curve: &NurbsCurve, x: f64) -> Point {
    let (t0, t1) = curve.domain();
    curve.point_at(t0 + x * (t1 - t0))
}

/// The piecewise-linear map from even stations 0..1 to `params`, at `x`.
fn station(params: &[f64], x: f64) -> f64 {
    let last = params.len() - 1;
    let at = (x * last as f64).clamp(0.0, last as f64);
    let low = (at.floor() as usize).min(last - 1);
    let s = at - low as f64;
    params[low] + (params[low + 1] - params[low]) * s
}

/// Samples between crossings: `DENSITY` per span, at most `MOST` in all, crossings included.
fn sample_count(crossings: usize) -> usize {
    let spans = crossings - 1;
    DENSITY.min((MOST - 1) / spans).max(1) * spans + 1
}

