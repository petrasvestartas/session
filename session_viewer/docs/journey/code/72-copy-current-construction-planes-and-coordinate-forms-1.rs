

    match text.strip_prefix(['w', 'W']) {
        Some(rest) => Some((parse_plain(rest)?, true)),
        None => Some((parse_plain(text)?, false)),
    }
}

/// A typed absolute point in the world: measured in the fixed construction plane, in world coordinates in View mode or after `w`.
pub fn absolute(text: &str) -> Option<[f64; 3]> {
    let (Typed::Absolute { x, y, z }, world) = parse_world(text)? else {
        return None;
    };
    let uvw = [x, y, z.unwrap_or(0.0)];

    match plane().filter(|_| !world) {
        Some(plane) => {
            let p = plane.world(uvw);
            Some([p[0], p[1], p[2]])
        }
        None => Some(uvw),
    }
}

/// A typed offset in the world: along the fixed construction plane's axes, along the world axes in View mode or after `w`.
pub fn offset(uvw: [f64; 3], world: bool) -> [f64; 3] {
    match plane().filter(|_| !world) {
        Some(plane) => {
            let (o, p) = (plane.origin(), plane.world(uvw));
            [p[0] - o[0], p[1] - o[1], p[2] - o[2]]
        }
        None => uvw,
    }
}

/// A typed coordinate as a point: in the fixed construction plane's coordinates from its origin, else as `resolve` places it on the drawing plane.
pub fn place(
    typed: Typed,
    world: bool,
    x_axis: &Vector,
    y_axis: &Vector,
    previous: Option<&Point>,
    along: Option<&Vector>,
) -> Option<Point> {
    let Some(plane) = plane().filter(|_| !world) else {
        // world coordinates; a fixed plane's axes do not apply after `w`
        let (x, y) = match world && plane().is_some() {
            true => CPlane::Xy.axes(),
            false => (x_axis.clone(), y_axis.clone()),
        };
        return resolve(typed, &Point::new(0.0, 0.0, 0.0), &x, &y, previous, along);
    };
    // `base` plus an offset in plane coordinates
    let moved = |base: &Point, uvw: [f64; 3]| {
        let d = offset(uvw, false);
        Point::new(base[0] + d[0], base[1] + d[1], base[2] + d[2])
    };

    match typed {
        Typed::Absolute { x, y, z } => Some(plane.world([x, y, z.unwrap_or(0.0)])),
        Typed::Relative { x, y, z } => Some(moved(previous?, [x, y, z.unwrap_or(0.0)])),
        Typed::Polar { distance, degrees } => {
            let r = degrees.to_radians();
            let base = previous.cloned().unwrap_or_else(|| plane.origin());
            Some(moved(&base, [distance * r.cos(), distance * r.sin(), 0.0]))
        }
        Typed::Distance(_) => resolve(typed, &plane.origin(), x_axis, y_axis, previous, along),
    }
}

