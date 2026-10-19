at(t), direction));
        }
        _ => return None,
    };
    let mut direction = &tip - &inward;
    direction.normalize_self().then_some(())?;
    Some((tip, direction))
}

/// Where a straight end lands: a distance on, or the first boundary ahead.
fn straight(from: &Point, direction: &Vector, reach: &Reach) -> Result<Point, String> {
    let (cutters, meshes) = match reach {
        Reach::Distance(distance) => return Ok(from + &(direction * *distance)),
        Reach::Boundaries { cutters, meshes } => (cutters, meshes),
    };
    let mut best: Option<f64> = None; // distance ahead of the nearest boundary
    let mut take = |t: f64| {
        if t > NEAR && best.is_none_or(|near| t < near) {
            best = Some(t);
        }
    };

    for cutter in cutters {
        match cutter {
            Cutter::Plane(plane) => {
                let ray = Line::from_points(from, &(from + direction));

                if let Some(hit) = intersection::line_plane(&ray, plane, false) {
                    take((&hit - from).dot(direction));
                }
            }
            Cutter::Curve(curve) => {
                // a probe reaching past every control point of the boundary
                let reach = (0..curve.m_cv_count)
                    .filter_map(|i| curve.get_cv(i))
                    .map(|p| p.distance(from, None))
                    .fold(1.0, f64::max)
                    * 2.0;
                let probe =
                    NurbsCurve::create(false, 1, &[from.clone(), from + &(direction * reach)]);

                if let Ok(pieces) =
                    split::split_curve_by_curves(&probe, std::slice::from_ref(curve), NEAR)
                {
                    for piece in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        take((&piece.point_at_end() - from).dot(direction));
                    }
                }
            }
        }
    }

    for (mesh, into) in meshes {
        let Some(back) = into.inverse() else {
            continue;
        };
        let ray = unit_ray(from, direction, into);

        for hit in intersection::ray_mesh(&ray, mesh, 1e-12, true).unwrap_or_default() {
            take((&hit.transformed(&back) - from).dot(direction));
        }
    }

    best.map(|t| from + &(direction * t))
        .ok_or_else(|| "no boundary ahead".into())
}

