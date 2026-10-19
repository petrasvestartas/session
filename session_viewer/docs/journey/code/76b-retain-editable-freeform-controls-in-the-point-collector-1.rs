use session_rust::{NurbsCurve,Point};
/// A curve through control points; ending on the start closes it smoothly.
pub fn curve(points: &[Point]) -> NurbsCurve {
    let count = points.len();
    let closed = count >= 4 && points[0].distance(&points[count - 1], None) <= 1e-12;

    if closed {
        return NurbsCurve::create(true, (count - 2).min(3), &points[..count - 1]);
    }

    NurbsCurve::create(false, (count - 1).min(3), points)
}

