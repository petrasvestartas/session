use crate::{Line, Point, Tolerance};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_line_intersection() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let p = line_line(&l0, &l1, Tolerance::APPROXIMATION as f32)
            .expect("Should find intersection");

        assert!((p.x() - 500.0).abs() < 0.1);
        assert!((p.y() - 328.303).abs() < 0.1);
        assert!((p.z() - 468.866).abs() < 0.1);
    }

    #[test]
    fn test_line_line_parameters_with_tolerance() {
        let l0 = Line::new(500.000, -573.576, -819.152, 500.000, 573.576, 819.152);
        let l1 = Line::new(13.195, 234.832, 534.315, 986.805, 421.775, 403.416);

        let result = line_line_parameters(
            &l0,
            &l1,
            Tolerance::APPROXIMATION as f32,
            true,
            false,
        )
        .expect("Should find parameters");

        let (t0, t1) = result;
        assert!(t0 >= 0.0 && t0 <= 1.0);
        assert!(t1 >= 0.0 && t1 <= 1.0);
    }
}
