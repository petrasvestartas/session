ax(s1 * 0.1);
        (s0, f0) = (s1, f1);
        s1 = s2;
        f1 = added(s1)? - distance;
    }

    grow(curve, end, s1).ok_or_else(|| "Cannot extend this curve".into())
}

/// A curve grown along its own shape to the first boundary curve or plane it meets.
fn to_boundary(curve: &NurbsCurve, end: End, cutters: &[Cutter]) -> Result<NurbsCurve, String> {
    if cutters.is_empty() {
        return Err(
            "Meshes and BReps stop lines and polylines; pick a curve or plane for a curve".into(),
        );
    }

    let (d0, d1) = curve.domain();
    let at = if end == End::Start { d0 } else { d1 };
    let from = curve.point_at(at);
    let speed = curve.evaluate(at, 1)[1].magnitude().max(1e-12);
    // how far the nearest boundary is, as a first guess
    let away = cutters
        .iter()
        .map(|cutter| match cutter {
            Cutter::Plane(plane) => (&from - &plane.origin()).dot(&plane.z_axis()).abs(),
            Cutter::Curve(boundary) => boundary.closest_point(&from).distance(&from, None),
        })
        .fold(f64::INFINITY, f64::min)
        .max(curve.length(None) * 0.05);
    let mut s = away / speed;

    for _ in 0..7 {
        let grown = grow(curve, end, s).ok_or("Cannot extend this curve")?;
        let piece = beyond(curve, &grown, end).ok_or("Cannot extend this curve")?;
        let (p0, p1) = piece.domain();
        let near = (p1 - p0) * 1e-9;
        let mut crossings = Vec::new();

        for cutter in cutters {
            match cutter {
                Cutter::Plane(plane) => {
                    crossings.extend(intersection::curve_plane(&piece, plane, Some(NEAR)))
                }
                Cutter::Curve(boundary) => {
                    let pieces = split::split_curve_by_curves(
                        &piece,
                        std::slice::from_ref(boundary),
                        NEAR,
                    )
                    .unwrap_or_default();

                    for part in pieces.iter().take(pieces.len().saturating_sub(1)) {
                        crossings.push(
                            intersection::curve_closest_point(&piece, &part.point_at_end(), p0, p1)
                                .0,
                        );
                    }
                }
            }
        }

        // the crossing nearest the old end
        let first = match end {
            End::End => crossings
                .into_iter()
                .filter(|t| *t > p0 + near)
                .min_by(f64::total_cmp),
            End::Start => crossings
                .into_iter()
                .filter(|t| *t < p1 - near)
                .max_by(f64::total_cmp),
        };

        if let Some(t) = first {
            let (g0, g1) = grown.domain();
            let mut cut = grown;
            let done = match end {
                End::End => cut.trim(g0, t),
                End::Start => cut.trim(t, g1),
            };
            return done
                .then_some(cut)
                .ok_or_else(|| "Cannot cut the curve at the boundary".into());
        }

        s *= 2.0;
    }

    Err("does not reach the boundary".into())
}

