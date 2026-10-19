        }
        Handle::Scale(_) | Handle::ScaleUniform => Ok((value != 1.0).then_some(value)),
        Handle::Translate(_) | Handle::Rotate(_) => Ok((value != 0.0).then_some(value)),
    }
}

fn typed_factor(value: f64) -> f64 {
    if value <= 0.0 {
        return MIN_SCALE;
    }

    value
}

fn softened(ratio: f64) -> f64 {
    if !ratio.is_finite() || ratio <= 0.0 {
        return MIN_SCALE;
    }

    ratio.powf(SCALE_SOFTENING).max(MIN_SCALE)
}

fn nonzero(v: f64) -> f64 {
    if v.abs() < 1e-9 {
        1e-9_f64.copysign(if v < 0.0 { -1.0 } else { 1.0 })
    } else {
        v
    }
}

fn facing(dir: &Vector) -> Vector {
    let (x, y, z) = (dir[0].abs(), dir[1].abs(), dir[2].abs());

    if x >= y && x >= z {
        Axis::X.unit()
    } else if y >= z {
        Axis::Y.unit()
    } else {
        Axis::Z.unit()
    }
}

fn end_on(dir: &Vector, axis: Axis) -> bool {
    dir.dot(&axis.unit()).abs() > 0.97
}

fn closest_on_axis(from: &Point, dir: &Vector, origin: &Point, axis: &Vector) -> Option<Point> {
    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let line = Line::from_point_direction_length(origin, axis, 1.0);
    let (_, t) = line_line_parameters(&ray, &line, 0.0, false, false)?;

    Some(origin + &(axis * t))
}

fn plane_hit(from: &Point, dir: &Vector, origin: &Point, normal: &Vector) -> Option<Point> {
    if dir.dot(normal).abs() < 1e-9 {
        return None;
    }

    let ray = Line::from_point_direction_length(from, dir, 1.0);
    let plane = Plane::from_point_normal(origin.clone(), normal.clone(), Some(false));
    let hit = line_plane(&ray, &plane, false)?;
    let t = (&hit - from).dot(dir);

    (t > 0.0 && t.is_finite()).then_some(hit)
}

fn within(from: &Point, dir: &Vector, at: &Point, radius: f64) -> bool {
    let t = (at - from).dot(dir);

    if t < 0.0 {
        return false;
    }

    let closest = from + &(dir * t);
    (&closest - at).magnitude() <= radius
}

fn angle_in_plane(p: &Point, origin: &Point, axis: Axis) -> f64 {
    let (u, v) = axis.others();
    let d = p - origin;
    d.dot(&v).atan2(d.dot(&u))
}

fn rotation(axis: Axis, radians: f64) -> Xform {
    match axis {
        Axis::X => Xform::rotation_x(radians, false),
        Axis::Y => Xform::rotation_y(radians, false),
        Axis::Z => Xform::rotation_z(radians, false),
    }
}

fn about(pivot: &Point, m: Xform) -> Xform {
    let to = Xform::translation(pivot[0], pivot[1], pivot[2]);
    let back = Xform::translation(-pivot[0], -pivot[1], -pivot[2]);

    &(&to * &m) * &back
}

