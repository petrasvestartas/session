//! Greville "edit points" and the ΔE → ΔP refit map.
//!
//! An edit point is the curve evaluated at a Greville abscissa `Eᵢ = C(ξᵢ)`; it lies
//! on the geometry, there are exactly as many as control points, and the map
//! edit-points ↔ control-points is the square (rational) collocation matrix `R`.
//! Dragging edit point `k` by world Δ refits the control points by `ΔPᵢ = (R⁻¹)ᵢₖ·Δ`
//! with weights kept, so rational geometry (circles) stays rational.
//!
//! Pure NURBS math over the existing kernel hooks (`nurbsknot::{find_span, eval_basis}`,
//! `Matrix::solve`, `NurbsCurve::{point_at, get_cv_4d}`); viewer-local to avoid a
//! 3-language kernel port.

use session_rust::nurbsknot;
use session_rust::{Matrix, NurbsCurve, Point};

/// Greville abscissae `ξᵢ = (knot[i] + … + knot[i+p-1]) / p`, one per control point.
///
/// Uses the OpenNURBS knot layout (length `order + cv_count - 2`, end knots not
/// duplicated): the average of `p = order-1` consecutive knots starting at `i`. For a
/// clamped curve `ξ₀ = u_min` and `ξₙ = u_max`, so the end edit points equal the end CVs.
pub fn greville_params(order: usize, knots: &[f64], cv_count: usize) -> Vec<f64> {
    let p = order.saturating_sub(1);
    if p == 0 || knots.len() < cv_count + p - 1 {
        // Degenerate (degree 0 / malformed knots): fall back to a uniform spread.
        return (0..cv_count).map(|i| i as f64).collect();
    }
    let mut g = Vec::with_capacity(cv_count);
    for i in 0..cv_count {
        let mut s = 0.0;
        for j in 0..p {
            s += knots[i + j];
        }
        g.push(s / p as f64);
    }
    g
}

/// Edit points `Eᵢ = C(ξᵢ)` (on the curve), one per control point.
pub fn edit_points(curve: &NurbsCurve) -> Vec<Point> {
    let order = curve.order();
    let cv_count = curve.cv_count();
    greville_params(order, &curve.m_nurbsknot, cv_count)
        .into_iter()
        .map(|t| curve.point_at(t))
        .collect()
}

/// The square rational collocation matrix `Rⱼᵢ = wᵢ Nᵢ(ξⱼ) / Σₖ wₖ Nₖ(ξⱼ)`.
/// Non-rational falls out when all weights are 1. Banded (bandwidth `p`), stored dense.
fn collocation_matrix(curve: &NurbsCurve) -> Matrix {
    let order = curve.order();
    let n = curve.cv_count();
    let knots = &curve.m_nurbsknot;
    let xi = greville_params(order, knots, n);
    let weight = |i: usize| curve.get_cv_4d(i).map_or(1.0, |(_, _, _, w)| w);
    let mut mat = Matrix::zeros(n, n);
    for j in 0..n {
        let t = xi[j];
        let span = nurbsknot::find_span(order, n, knots, t);
        let basis = nurbsknot::eval_basis(order, knots, span, t); // `order` values for CVs span..span+order-1
        let mut denom = 0.0;
        for r in 0..order.min(basis.len()) {
            denom += basis[r] * weight(span + r);
        }
        if denom == 0.0 {
            denom = 1.0;
        }
        for r in 0..order.min(basis.len()) {
            let i = span + r;
            if i < n {
                mat[(j, i)] = basis[r] * weight(i) / denom;
            }
        }
    }
    mat
}

/// Column `k` of `R⁻¹`: the CV displacement field for a unit drag of edit point `k`.
/// Precompute once at drag start, then `ΔPᵢ = col[i]·Δ` per mouse-move.
/// Returns `None` if the system is singular (degenerate curve).
pub fn inverse_collocation_column(curve: &NurbsCurve, k: usize) -> Option<Vec<f64>> {
    let n = curve.cv_count();
    if k >= n {
        return None;
    }
    let mat = collocation_matrix(curve);
    let mut e = Matrix::zeros(n, 1);
    e[(k, 0)] = 1.0;
    let x = mat.solve(&e)?;
    Some((0..n).map(|i| x[(i, 0)]).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Point;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    // A degree-3 open curve through a few points: edit points must lie on the curve,
    // the end edit points must equal the end CVs, and ξ must be monotone.
    #[test]
    fn edit_points_on_curve_and_monotone() {
        let pts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, -1.0, 0.0),
            Point::new(5.0, 1.0, 0.0),
            Point::new(7.0, 0.0, 0.0),
        ];
        let c = NurbsCurve::create(false, 3, &pts);
        let order = c.order();
        let n = c.cv_count();
        let xi = greville_params(order, &c.m_nurbsknot, n);
        for w in xi.windows(2) {
            assert!(w[1] >= w[0] - 1e-12, "Greville params not monotone: {:?}", xi);
        }
        let eps = edit_points(&c);
        for (i, e) in eps.iter().enumerate() {
            let on = c.point_at(xi[i]);
            assert!(approx(e[0], on[0], 1e-9) && approx(e[1], on[1], 1e-9));
        }
    }

    // Round-trip: ΔPᵢ = (R⁻¹)ᵢₖ·Δ moves edit point k by Δ and leaves it interpolated.
    #[test]
    fn drag_keeps_moved_edit_point_interpolated() {
        let pts = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, -1.0, 0.0),
            Point::new(5.0, 1.0, 0.0),
            Point::new(7.0, 0.0, 0.0),
        ];
        let mut c = NurbsCurve::create(false, 3, &pts);
        let n = c.cv_count();
        let order = c.order();
        let xi = greville_params(order, &c.m_nurbsknot, n);
        let k = 2;
        let e0 = c.point_at(xi[k]);
        let delta = [0.0_f64, 1.5, 0.0];
        let col = inverse_collocation_column(&c, k).expect("non-singular");
        for i in 0..n {
            let (x, y, z, w) = c.get_cv_4d(i).unwrap_or((0.0, 0.0, 0.0, 1.0));
            // CVs are stored homogeneous (x·w …); apply the Euclidean displacement col[i]·Δ.
            let dx = col[i] * delta[0];
            let dy = col[i] * delta[1];
            let dz = col[i] * delta[2];
            c.set_cv_4d(i, (x / w + dx) * w, (y / w + dy) * w, (z / w + dz) * w, w);
        }
        let e1 = c.point_at(xi[k]);
        assert!(approx(e1[0], e0[0] + delta[0], 1e-7), "x: {} vs {}", e1[0], e0[0] + delta[0]);
        assert!(approx(e1[1], e0[1] + delta[1], 1e-7), "y: {} vs {}", e1[1], e0[1] + delta[1]);
    }
}
