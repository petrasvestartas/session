/// The end point of a curve and the direction it leaves in.
pub fn tip(geometry: &Geometry, end: End) -> Option<(Point, Vector)> {
    let (tip, inward) = match geometry {
        Geometry::Line(line) => match end {
            End::Start => (line.start(), line.end()),
            End::End => (line.end(), line.start()),
        },
        Geometry::Polyline(polyline) => {
            let count = polyline.point_count();

            if count < 2 {
                return None;
            }

            match end {
                End::Start => (polyline.get_point(0)?, polyline.get_point(1)?),
                End::End => (
                    polyline.get_point(count - 1)?,
                    polyline.get_point(count - 2)?,
                ),
            }
        }
        Geometry::NurbsCurve(curve) => {
            let (d0, d1) = curve.domain();
            let (t, sign) = match end {
                End::Start => (d0, -1.0),
                End::End => (d1, 1.0),
            };
            let mut direction = &curve.tangent_at(t) * sign;
            direction.normalize_self().then_some(())?;
            return Some((curve.point_at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}

/// The curve with its domain grown by `s` at `end`.
fn grow(curve: &NurbsCurve, end: End, s: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let mut grown = curve.clone();
    let done = match end {
        End::Start => grown.extend(d0 - s, d1),
        End::End => grown.extend(d0, d1 + s),
    };
    done.then_some(grown)
}

/// The part of `grown` beyond the old domain.
fn beyond(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let (g0, g1) = grown.domain();
    let mut piece = grown.clone();
    let done = match end {
        End::Start => piece.trim(g0, d0),
        End::End => piece.trim(d1, g1),
    };
    done.then_some(piece)
}

/// The added piece as points, from the old end.
fn tail(curve: &NurbsCurve, grown: &NurbsCurve, end: End) -> Vec<Point> {
    let Some(piece) = beyond(curve, grown, end) else {
        return Vec::new();
    };
    let mut points = crate::source_cut::samples(&piece, 24);

    if end == End::Start {
        points.reverse();
    }

    points
}

/// A curve grown along its own shape until the added length is `distance`.
fn by_length(curve: &NurbsCurve, end: End, distance: f64) -> Result<NurbsCurve, String> {
    let (d0, d1) = curve.domain();
    let at = if end == End::Start { d0 } else { d1 };
    let speed = curve.evaluate(at, 1)[1].magnitude();

    if speed <= 1e-12 {
        return Err("This curve has no direction at its end".into());
    }

    // the added length for a growth of `s`
    let added = |s: f64| -> Result<f64, String> {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        Ok(beyond(curve, &grown, end)
            .ok_or("Cannot extend this curve")?
            .length(None))
    };
    let (mut s0, mut f0) = (0.0, -distance);
    let mut s1 = distance / speed;
    let mut f1 = added(s1)? - distance;

    // secant steps on the added length
    for _ in 0..8 {
        if f1.abs() <= 1e-9 * distance || (f1 - f0).abs() <= f64::MIN_POSITIVE {
            break;
        }

        let s2 = (s1 - f1 * (s1 - s0) / (f1 - f0)).max(s1 * 0.1);
        (s0, f0) = (s1, f1);
        s1 = s2;
        f1 = added(s1)? - distance;
    }

    grow(curve, end, s1).ok_or_else(|| "Cannot extend this curve".into())
