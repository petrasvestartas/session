use session_rust::{Geometry, NurbsCurve, NurbsSurface, Point, Primitives, Vector};
use std::rc::Rc;

pub use crate::gather_input::SAMPLES;

pub use crate::gather_input::{Picked, count, picked};

/// The unit normal of a loop by Newell's method; None when it encloses no area.
pub fn loop_normal(points: &[Point]) -> Option<Vector> {
    let mut sum = [0.0; 3];

    for (index, a) in points.iter().enumerate() {
        let b = &points[(index + 1) % points.len()];
        sum[0] += (a[1] - b[1]) * (a[2] + b[2]);
        sum[1] += (a[2] - b[2]) * (a[0] + b[0]);
        sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }

    let length = (sum[0] * sum[0] + sum[1] * sum[1] + sum[2] * sum[2]).sqrt();
    let size = diagonal(points);

    if length <= 1e-12 * size * size {
        return None;
    }

    Some(Vector::new(
        sum[0] / length,
        sum[1] / length,
        sum[2] / length,
    ))
}

/// The diagonal of the points' box, at least 1.
pub fn diagonal(points: &[Point]) -> f64 {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in points {
        for i in 0..3 {
            low[i] = low[i].min(p[i]);
            high[i] = high[i].max(p[i]);
        }
    }

    let size = (0..3)
        .map(|i| (high[i] - low[i]).powi(2))
        .sum::<f64>()
        .sqrt();
    size.max(1.0)
}

/// True when every point lies on the plane through the first with `normal`, within a millionth of the size.
pub fn planar(points: &[Point], normal: &Vector) -> bool {
    let Some(first) = points.first() else {
        return false;
    };
    let tolerance = 1e-6 * diagonal(points);
    points
        .iter()
        .all(|p| (p - first).dot(normal).abs() <= tolerance)
}

/// The surface as a named geometry, when valid with finite control points.
pub fn checked(mut surface: NurbsSurface, name: &str) -> Result<Geometry, String> {
    let finite = (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface
                .get_cv(i, j)
                .is_some_and(|p| (0..3).all(|k| p[k].is_finite()))
        })
    });

    if !surface.is_valid() || !finite {
        return Err(format!("The {name} surface could not be built"));
    }

    surface.name = name.into();
    Ok(Geometry::NurbsSurface(Rc::new(surface)))
}

/// The world curves of one step.
pub fn curves(picked: &[Picked]) -> Vec<NurbsCurve> {
    picked.iter().map(|picked| picked.curve.clone()).collect()
}

