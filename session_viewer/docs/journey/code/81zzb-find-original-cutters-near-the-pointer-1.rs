/// The plane through `line` that holds `normal`: a vertical cut when `normal` is the view's up.
pub fn fence_plane(line: &Line, normal: &Vector) -> Result<Plane, String> {
    let across = line.to_vector().cross(normal);

    if across.magnitude() <= 1e-12 * line.length().max(1.0) {
        return Err("The cutting line runs along the view direction".into());
    }

    Ok(Plane::from_point_normal(line.start(), across, None))
}

/// `plane` moved by `xform`, rebuilt from three moved points so a stretch keeps it true.
pub fn plane_moved(plane: &Plane, xform: &Xform) -> Option<Plane> {
    let origin = plane.origin();
    let a = origin.transformed(xform);
    let b = (&origin + &plane.x_axis()).transformed(xform);
    let c = (&origin + &plane.y_axis()).transformed(xform);
    let normal = (&b - &a).cross(&(&c - &a));
    (normal.magnitude() > 1e-14).then(|| Plane::from_point_normal(a, normal, None))
}

/// A ray moved by `xform` with a unit direction, so a scaled object's triangles are not taken for parallel.
pub fn unit_ray(origin: &Point, direction: &Vector, xform: &Xform) -> Line {
    let start = origin.transformed(xform);
    let along = (&(origin + direction).transformed(xform) - &start).normalized();
    Line::from_points(&start, &(&start + &along))
}

/// Points along a curve: its corners when straight, else `count` even steps.
pub fn samples(curve: &NurbsCurve, count: usize) -> Vec<Point> {
    if curve.degree() == 1 {
        return curve
            .get_span_vector()
            .iter()
            .map(|t| curve.point_at(*t))
            .collect();
    }

    let (t0, t1) = curve.domain();
    (0..=count)
        .map(|i| curve.point_at(t0 + (t1 - t0) * i as f64 / count as f64))
        .collect()
}

/// Pixel distance from `at` to the segment a-b.
pub fn segment_px(a: (f64, f64), b: (f64, f64), at: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let length = dx * dx + dy * dy;
    let t = if length > 0.0 {
        (((at.0 - a.0) * dx + (at.1 - a.1) * dy) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (a.0 + t * dx - at.0).hypot(a.1 + t * dy - at.1)
}

/// The polyline nearest `at` on screen within `aperture` pixels, and its distance.
pub fn nearest_px(polylines: &[Vec<(f64, f64)>], at: (f64, f64), aperture: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;

    for (index, polyline) in polylines.iter().enumerate() {
        let distance = match polyline.as_slice() {
            [single] => segment_px(*single, *single, at),
            _ => polyline
                .windows(2)
                .map(|pair| segment_px(pair[0], pair[1], at))
                .fold(f64::INFINITY, f64::min),
        };

        if distance <= aperture && best.is_none_or(|(_, near)| distance < near) {
            best = Some((index, distance));
        }
    }

    best.map(|(index, _)| index)
