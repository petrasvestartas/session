use crate::{Line, Point};

pub fn line_line_parameters(
    line0: &Line,
    line1: &Line,
    tolerance: f32,
    intersect_segments: bool,
    near_parallel_as_closest: bool,
) -> Option<(f32, f32)> {
    let p0_start = line0.start();
    let p0_end = line0.end();
    let p1_start = line1.start();
    let p1_end = line1.end();

    if p0_start == p1_start {
        return Some((0.0, 0.0));
    }
    if p0_start == p1_end {
        return Some((0.0, 1.0));
    }
    if p0_end == p1_start {
        return Some((1.0, 0.0));
    }
    if p0_end == p1_end {
        return Some((1.0, 1.0));
    }

    let a = line0.to_vector();
    let b = line1.to_vector();
    let c = p1_start - p0_start;

    let aa = a.dot(&a);
    let bb = b.dot(&b);
    let ab = a.dot(&b);
    let ac = a.dot(&c);
    let bc = b.dot(&c);

    let det = aa * bb - ab * ab;

    let zero_tol = aa.max(bb) * f32::EPSILON;
    if det.abs() < zero_tol {
        if !near_parallel_as_closest {
            return None;
        }
        let mut t0 = if aa > 0.0 { ac / aa } else { 0.0 };
        let mut t1 = if bb > 0.0 { (bc + t0 * ab) / bb } else { 0.0 };

        if intersect_segments {
            t0 = t0.clamp(0.0, 1.0);
            t1 = t1.clamp(0.0, 1.0);
        }

        if tolerance > 0.0 {
            let pt0 = line0.point_at(t0);
            let pt1 = line1.point_at(t1);
            if pt0.distance(&pt1) > tolerance {
                return None;
            }
        }
        return Some((t0, t1));
    }

    let inv_det = 1.0 / det;
    let mut t0 = (bb * ac - ab * bc) * inv_det;
    let mut t1 = (ab * ac - aa * bc) * inv_det;

    if intersect_segments {
        t0 = t0.clamp(0.0, 1.0);
        t1 = t1.clamp(0.0, 1.0);
    }

    if tolerance > 0.0 {
        let pt0 = line0.point_at(t0);
        let pt1 = line1.point_at(t1);
        if pt0.distance(&pt1) > tolerance {
            return None;
        }
    }

    Some((t0, t1))
}

pub fn line_line(line0: &Line, line1: &Line, tolerance: f32) -> Option<Point> {
    let result = line_line_parameters(line0, line1, tolerance, true, false)?;

    let (t0, t1) = result;
    let p0 = line0.point_at(t0);
    let p1 = line1.point_at(t1);

    Some(Point::new(
        (p0.x() + p1.x()) * 0.5,
        (p0.y() + p1.y()) * 0.5,
        (p0.z() + p1.z()) * 0.5,
    ))
}

pub fn plane_plane(plane0: &crate::Plane, plane1: &crate::Plane) -> Option<Line> {
    let d = plane1.z_axis().cross(&plane0.z_axis());

    let origin0 = plane0.origin();
    let origin1 = plane1.origin();
    let p = Point::new(
        (origin0.x() + origin1.x()) * 0.5,
        (origin0.y() + origin1.y()) * 0.5,
        (origin0.z() + origin1.z()) * 0.5,
    );

    let plane2 = crate::Plane::from_point_normal(p, d.clone());

    let output_p = plane_plane_plane(plane0, plane1, &plane2)?;

    Some(Line::new(
        output_p.x(),
        output_p.y(),
        output_p.z(),
        output_p.x() + d.x(),
        output_p.y() + d.y(),
        output_p.z() + d.z(),
    ))
}

pub fn plane_plane_plane(
    plane0: &crate::Plane,
    plane1: &crate::Plane,
    plane2: &crate::Plane,
) -> Option<Point> {
    let n0 = plane0.z_axis();
    let n1 = plane1.z_axis();
    let n2 = plane2.z_axis();

    let det = n0.dot(&n1.cross(&n2));

    if det.abs() < 1e-10 {
        return None;
    }

    let d0 = plane0.d();
    let d1 = plane1.d();
    let d2 = plane2.d();

    let inv_det = 1.0 / det;
    let p = (n1.cross(&n2) * (-d0) + n2.cross(&n0) * (-d1) + n0.cross(&n1) * (-d2)) * inv_det;

    Some(Point::new(p.x(), p.y(), p.z()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tolerance;

    #[test]
    fn test_line_line_intersection() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let p = line_line(&l0, &l1, Tolerance::APPROXIMATION).expect("Should find intersection");

        assert!((p.x() - 500.0).abs() < 0.1);
        assert!((p.y() - 328.303).abs() < 0.1);
        assert!((p.z() - 468.866).abs() < 0.1);
    }

    #[test]
    fn test_line_line_parameters_with_tolerance() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let result = line_line_parameters(&l0, &l1, Tolerance::APPROXIMATION, true, false)
            .expect("Should find parameters");

        let (t0, t1) = result;
        assert!((0.0..=1.0).contains(&t0));
        assert!((0.0..=1.0).contains(&t1));
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_plane_plane_intersection() {
        use crate::{Plane, Point, Vector};

        let plane_origin_0 = Point::new(213.787107, 513.797811, -24.743845);
        let plane_xaxis_0 = Vector::new(0.907673, -0.258819, 0.330366);
        let plane_yaxis_0 = Vector::new(0.272094, 0.96225, 0.006285);
        let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

        let plane_origin_1 = Point::new(247.17924, 499.115486, 59.619568);
        let plane_xaxis_1 = Vector::new(0.552465, 0.816035, 0.16991);
        let plane_yaxis_1 = Vector::new(0.172987, 0.087156, -0.98106);
        let pl1 = Plane::new(plane_origin_1, plane_xaxis_1, plane_yaxis_1);

        let intersection_line = plane_plane(&pl0, &pl1).expect("Should find intersection");

        let start = intersection_line.start();
        let end = intersection_line.end();

        assert!((start.x() - 252.4632).abs() < 0.01);
        assert!((start.y() - 495.32248).abs() < 0.01);
        assert!((start.z() - (-10.002656)).abs() < 0.01);

        assert!((end.x() - 253.01033).abs() < 0.01);
        assert!((end.y() - 496.1218).abs() < 0.01);
        assert!((end.z() - (-9.888727)).abs() < 0.01);
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn test_plane_plane_plane_intersection() {
        use crate::{Plane, Point, Vector};

        let plane_origin_0 = Point::new(213.787107, 513.797811, -24.743845);
        let plane_xaxis_0 = Vector::new(0.907673, -0.258819, 0.330366);
        let plane_yaxis_0 = Vector::new(0.272094, 0.96225, 0.006285);
        let pl0 = Plane::new(plane_origin_0, plane_xaxis_0, plane_yaxis_0);

        let plane_origin_1 = Point::new(247.17924, 499.115486, 59.619568);
        let plane_xaxis_1 = Vector::new(0.552465, 0.816035, 0.16991);
        let plane_yaxis_1 = Vector::new(0.172987, 0.087156, -0.98106);
        let pl1 = Plane::new(plane_origin_1, plane_xaxis_1, plane_yaxis_1);

        let plane_origin_2 = Point::new(221.399816, 605.893667, -54.000116);
        let plane_xaxis_2 = Vector::new(0.903451, -0.360516, -0.231957);
        let plane_yaxis_2 = Vector::new(0.172742, -0.189057, 0.966653);
        let pl2 = Plane::new(plane_origin_2, plane_xaxis_2, plane_yaxis_2);

        let ppp = plane_plane_plane(&pl0, &pl1, &pl2).expect("Should find intersection");

        assert!((ppp.x() - 300.5).abs() < 0.1);
        assert!((ppp.y() - 565.5).abs() < 0.1);
        assert!((ppp.z() - 0.0).abs() < 0.1);
    }
}
