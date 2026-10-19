use session_rust::{Plane,Point,Xform};
/// The signed angle, radians, from `from` to `to` about `center` in the plane; None when either lies on the center.
pub fn angle(center: &Point, from: &Point, to: &Point, plane: &Plane) -> Option<f64> {
    let (x, y) = (plane.x_axis(), plane.y_axis());
    let (a, b) = (from - center, to - center);
    let (ax, ay, bx, by) = (a.dot(&x), a.dot(&y), b.dot(&x), b.dot(&y));

    if ax.hypot(ay) <= 1e-12 || bx.hypot(by) <= 1e-12 {
        return None;
    }

    Some((ax * by - ay * bx).atan2(ax * bx + ay * by))
}

/// A turn of `radians` about the plane normal through `center`.
pub fn rotation(center: &Point, radians: f64, plane: &Plane) -> Xform {
    let to = Xform::translation(center[0], center[1], center[2]);
    let back = Xform::translation(-center[0], -center[1], -center[2]);
    &(&to * &Xform::rotation(&plane.z_axis(), radians, false)) * &back
}

