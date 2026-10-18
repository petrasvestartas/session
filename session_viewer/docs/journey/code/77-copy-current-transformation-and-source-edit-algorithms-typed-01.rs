use session_rust::{Plane,Point,Vector,Xform};
/// How far `p` reaches from `origin`: in space, in the plane, or along origin→`from` for 1D.
pub fn reach(mode: u8, origin: &Point, from: &Point, p: &Point, plane: &Plane) -> Option<f64> {
    let d = p - origin;

    match mode {
        1 => {
            let along = from - origin;
            let length = along.magnitude();
            (length > 1e-12).then(|| d.dot(&along) / length)
        }
        2 => Some(d.dot(&plane.x_axis()).hypot(d.dot(&plane.y_axis()))),
        _ => Some(d.magnitude()),
    }
}

/// The factor that takes the first reference point's reach to `to`'s.
pub fn factor(mode: u8, origin: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let base = reach(mode, origin, from, from, plane)?;
    let k = reach(mode, origin, from, to, plane)? / base;
    (base > 1e-12 && k.is_finite() && k > 1e-12).then_some(k)
}

/// A scale by `k` about `origin`: uniform, in the plane, or along origin→`from`.
pub fn scaling(mode: u8, origin: &Point, from: &Point, k: f64, plane: &Plane) -> Xform {
    match mode {
        1 => stretch(origin, &[(from - origin).normalized()], k),
        2 => stretch(origin, &[plane.x_axis(), plane.y_axis()], k),
        _ => Xform::scale_uniform(origin, k),
    }
}

/// Scale by `k` along the orthonormal `axes` through `origin`, keeping every other direction.
fn stretch(origin: &Point, axes: &[Vector], k: f64) -> Xform {
    let mut xform = Xform::identity();

    for a in axes {
        for col in 0..3 {
            for row in 0..3 {
                xform.m[col * 4 + row] += (k - 1.0) * a[row] * a[col];
            }
        }
    }

    // the origin stays where it is
    for row in 0..3 {
        let moved: f64 = (0..3).map(|col| xform.m[col * 4 + row] * origin[col]).sum();
        xform.m[12 + row] = origin[row] - moved;
    }

    xform
}

