use crate::{Line, Point};

pub fn line_line_parameters(
    line0: &Line,
    line1: &Line,
    tolerance: f64,
    intersect_segments: bool,
    near_parallel_as_closest: bool,
) -> Option<(f64, f64)> {
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

    let zero_tol = aa.max(bb) * f64::EPSILON;
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

/// Find intersection point between two 3D lines.
///
/// # Arguments
/// * `line0` - First line
/// * `line1` - Second line
/// * `tolerance` - Maximum distance between lines to consider them intersecting
///
/// # Returns
/// * `Some(Point)` - Intersection point (midpoint of closest approach for skew lines)
/// * `None` - If lines don't intersect within tolerance
pub fn line_line(line0: &Line, line1: &Line, tolerance: f64) -> Option<Point> {
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

/// Find intersection line between two planes.
///
/// # Arguments
/// * `plane0` - First plane
/// * `plane1` - Second plane
///
/// # Returns
/// * `Some(Line)` - Intersection line (infinite) if planes intersect
/// * `None` - If planes are parallel
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

fn plane_value_at(plane: &crate::Plane, point: &Point) -> f64 {
    plane.a() * point.x() + plane.b() * point.y() + plane.c() * point.z() + plane.d()
}

/// Find intersection point between a line and a plane.
///
/// # Arguments
/// * `line` - Line to intersect
/// * `plane` - Plane to intersect
/// * `is_finite` - If true, treat line as finite segment; if false, treat as infinite
///
/// # Returns
/// * `Some(Point)` - Intersection point if exists
/// * `None` - If line is parallel to plane or intersection is outside segment bounds
pub fn line_plane(line: &Line, plane: &crate::Plane, is_finite: bool) -> Option<Point> {
    let pt0 = line.start();
    let pt1 = line.end();

    let a = plane_value_at(plane, &pt0);
    let b = plane_value_at(plane, &pt1);
    let d = a - b;

    let (t, rc) = if d == 0.0 {
        let t = if a.abs() < b.abs() {
            0.0
        } else if b.abs() < a.abs() {
            1.0
        } else {
            0.5
        };
        (t, false)
    } else {
        let d_inv = 1.0 / d;
        let fd = d_inv.abs();
        if fd > 1.0 && (a.abs() >= f64::MAX / fd || b.abs() >= f64::MAX / fd) {
            (0.5, false)
        } else {
            (a / (a - b), true)
        }
    };

    let s = 1.0 - t;

    let output = Point::new(
        if line.x0() == line.x1() {
            line.x0()
        } else {
            s * line.x0() + t * line.x1()
        },
        if line.y0() == line.y1() {
            line.y0()
        } else {
            s * line.y0() + t * line.y1()
        },
        if line.z0() == line.z1() {
            line.z0()
        } else {
            s * line.z0() + t * line.z1()
        },
    );

    if is_finite && !(0.0..=1.0).contains(&t) {
        return None;
    }

    if rc {
        Some(output)
    } else {
        None
    }
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

/// Find intersection points between a line and an axis-aligned bounding box.
///
/// # Arguments
/// * `line` - Line to intersect
/// * `box_` - Axis-aligned bounding box
/// * `t0` - Minimum parameter value to consider (e.g., 0.0 for ray origin)
/// * `t1` - Maximum parameter value to consider (e.g., 1000.0 for max distance)
///
/// # Returns
/// * `Some(Vec<Point>)` - Vector of 2 points [entry, exit] if intersection exists
/// * `None` - If no intersection within [t0, t1] range
///
/// # Note
/// Points are sorted from line start (entry first, exit second)
pub fn ray_box(line: &Line, box_: &crate::BoundingBox, t0: f64, t1: f64) -> Option<Vec<Point>> {
    let origin = line.start();
    let direction = line.to_vector();

    let box_min = box_.min_point();
    let box_max = box_.max_point();

    // Calculate inverse direction (avoid division by zero)
    let inv_dir_x = if direction.x() != 0.0 {
        1.0 / direction.x()
    } else {
        f64::INFINITY
    };
    let inv_dir_y = if direction.y() != 0.0 {
        1.0 / direction.y()
    } else {
        f64::INFINITY
    };
    let inv_dir_z = if direction.z() != 0.0 {
        1.0 / direction.z()
    } else {
        f64::INFINITY
    };

    // Calculate intersections with X slabs
    let tx1 = (box_min.x() - origin.x()) * inv_dir_x;
    let tx2 = (box_max.x() - origin.x()) * inv_dir_x;

    let mut tmin = tx1.min(tx2);
    let mut tmax = tx1.max(tx2);

    // Calculate intersections with Y slabs
    let ty1 = (box_min.y() - origin.y()) * inv_dir_y;
    let ty2 = (box_max.y() - origin.y()) * inv_dir_y;

    tmin = tmin.max(ty1.min(ty2));
    tmax = tmax.min(ty1.max(ty2));

    // Calculate intersections with Z slabs
    let tz1 = (box_min.z() - origin.z()) * inv_dir_z;
    let tz2 = (box_max.z() - origin.z()) * inv_dir_z;

    tmin = tmin.max(tz1.min(tz2));
    tmax = tmax.min(tz1.max(tz2));

    // Clip to valid range
    tmin = tmin.max(t0);
    tmax = tmax.min(t1);

    // Check if intersection exists
    if tmax < tmin {
        return None;
    }

    // Calculate actual intersection points
    let entry = Point::new(
        origin.x() + direction.x() * tmin,
        origin.y() + direction.y() * tmin,
        origin.z() + direction.z() * tmin,
    );

    let exit = Point::new(
        origin.x() + direction.x() * tmax,
        origin.y() + direction.y() * tmax,
        origin.z() + direction.z() * tmax,
    );

    Some(vec![entry, exit])
}

/// Find intersection points between a line and a sphere.
///
/// # Arguments
/// * `line` - Line to intersect
/// * `center` - Sphere center point
/// * `radius` - Sphere radius
///
/// # Returns
/// * `Some(Vec<Point>)` - Vector of 1 point (tangent) or 2 points (entry/exit)
/// * `None` - If no intersection
///
/// # Note
/// Points are sorted from line start
pub fn ray_sphere(line: &Line, center: &Point, radius: f64) -> Option<Vec<Point>> {
    let origin = line.start();
    let direction = line.to_vector();

    // Vector from origin to center
    let o_x = origin.x() - center.x();
    let o_y = origin.y() - center.y();
    let o_z = origin.z() - center.z();

    // Quadratic equation coefficients
    let a = direction.x() * direction.x()
        + direction.y() * direction.y()
        + direction.z() * direction.z();
    let b = 2.0 * (direction.x() * o_x + direction.y() * o_y + direction.z() * o_z);
    let c = o_x * o_x + o_y * o_y + o_z * o_z - radius * radius;

    // Discriminant
    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        return None;
    }

    // Calculate intersection parameters
    let dist_sqrt = disc.sqrt();
    let q = if b < 0.0 {
        (-b - dist_sqrt) / 2.0
    } else {
        (-b + dist_sqrt) / 2.0
    };

    let mut t0 = q / a;
    let mut t1 = c / q;

    // Sort parameters
    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }

    // Calculate intersection points
    let mut points = Vec::new();

    // First intersection
    let p0 = Point::new(
        origin.x() + direction.x() * t0,
        origin.y() + direction.y() * t0,
        origin.z() + direction.z() * t0,
    );
    points.push(p0);

    // Second intersection (if different from first)
    if (t1 - t0).abs() > 1e-10 {
        let p1 = Point::new(
            origin.x() + direction.x() * t1,
            origin.y() + direction.y() * t1,
            origin.z() + direction.z() * t1,
        );
        points.push(p1);
    }

    Some(points)
}

/// Find intersection point between a line and a triangle.
///
/// # Arguments
/// * `line` - Line to intersect (start point used as origin, direction computed internally)
/// * `v0` - First vertex of triangle
/// * `v1` - Second vertex of triangle
/// * `v2` - Third vertex of triangle
/// * `epsilon` - Tolerance for parallel detection
///
/// # Returns
/// * `Some(Point)` - Intersection point if exists
/// * `None` - If no intersection (parallel or outside triangle)
pub fn ray_triangle(
    line: &Line,
    v0: &Point,
    v1: &Point,
    v2: &Point,
    epsilon: f64,
) -> Option<Point> {
    let origin = line.start();
    let direction = line.to_vector();

    // Möller-Trumbore algorithm
    let edge1_x = v1.x() - v0.x();
    let edge1_y = v1.y() - v0.y();
    let edge1_z = v1.z() - v0.z();

    let edge2_x = v2.x() - v0.x();
    let edge2_y = v2.y() - v0.y();
    let edge2_z = v2.z() - v0.z();

    // pvec = direction.cross(edge2)
    let pvec_x = direction.y() * edge2_z - direction.z() * edge2_y;
    let pvec_y = direction.z() * edge2_x - direction.x() * edge2_z;
    let pvec_z = direction.x() * edge2_y - direction.y() * edge2_x;

    // det = edge1.dot(pvec)
    let det = edge1_x * pvec_x + edge1_y * pvec_y + edge1_z * pvec_z;

    if det > -epsilon && det < epsilon {
        return None; // Parallel
    }

    let inv_det = 1.0 / det;

    // tvec = origin - v0
    let tvec_x = origin.x() - v0.x();
    let tvec_y = origin.y() - v0.y();
    let tvec_z = origin.z() - v0.z();

    // u = tvec.dot(pvec) * inv_det
    let u = (tvec_x * pvec_x + tvec_y * pvec_y + tvec_z * pvec_z) * inv_det;

    if u < -epsilon || u > 1.0 + epsilon {
        return None;
    }

    // qvec = tvec.cross(edge1)
    let qvec_x = tvec_y * edge1_z - tvec_z * edge1_y;
    let qvec_y = tvec_z * edge1_x - tvec_x * edge1_z;
    let qvec_z = tvec_x * edge1_y - tvec_y * edge1_x;

    // v = direction.dot(qvec) * inv_det
    let v = (direction.x() * qvec_x + direction.y() * qvec_y + direction.z() * qvec_z) * inv_det;

    if v < -epsilon || u + v > 1.0 + epsilon {
        return None;
    }

    // t = edge2.dot(qvec) * inv_det
    let t = (edge2_x * qvec_x + edge2_y * qvec_y + edge2_z * qvec_z) * inv_det;

    // Calculate intersection point: origin + t * direction
    Some(Point::new(
        origin.x() + t * direction.x(),
        origin.y() + t * direction.y(),
        origin.z() + t * direction.z(),
    ))
}
