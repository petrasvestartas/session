/// The cut parameters of a curve: crossings with the curves, passes through the planes.
fn curve_cuts(curve: &NurbsCurve, blades: &[Blade]) -> Result<Vec<f64>, String> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let closed = curve.is_closed();
    let curves: Vec<NurbsCurve> = blades
        .iter()
        .filter_map(|blade| match &blade.cutter {
            Cutter::Curve(curve) => Some(curve.clone()),
            Cutter::Plane(_) => None,
        })
        .collect();
    let mut cuts = Vec::new();

    // every piece end is a crossing
    if !curves.is_empty() {
        let pieces = split::split_curve_by_curves(curve, &curves, TOLERANCE)?;

        if pieces.len() > 1 {
            for piece in &pieces {
                for end in [piece.point_at_start(), piece.point_at_end()] {
                    cuts.push(intersection::curve_closest_point(curve, &end, d0, d1).0);
                }
            }
        }
    }

    for blade in blades {
        if let Cutter::Plane(plane) = &blade.cutter {
            cuts.extend(intersection::curve_plane(curve, plane, Some(TOLERANCE)));
        }
    }

    // a closed curve's seam is one point; an open curve's ends cut nothing
    let mut cuts: Vec<f64> = cuts
        .into_iter()
        .map(|t| if closed && t >= d1 - near { d0 } else { t })
        .filter(|t| closed || (*t > d0 + near && *t < d1 - near))
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() <= near.max(1e-12));
    Ok(cuts)
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 { d0 } else { cuts[index - 1] };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near).then(|| trimmed(a, d1)).flatten();
    let head = (b > d0 + near).then(|| trimmed(d0, b)).flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}

/// New indices for the used entries, -1 for the rest.
fn remap(used: &[bool]) -> Vec<i32> {
    let mut next = 0;
    used.iter()
        .map(|&kept| {
            if !kept {
                return -1;
            }

            next += 1;
            next - 1
        })
        .collect()
}

/// The entries of `list` marked in `used`.
fn kept_of<T: Clone>(list: &[T], used: &[bool]) -> Vec<T> {
    list.iter()
        .zip(used)
        .filter(|(_, kept)| **kept)
        .map(|(item, _)| item.clone())
        .collect()
