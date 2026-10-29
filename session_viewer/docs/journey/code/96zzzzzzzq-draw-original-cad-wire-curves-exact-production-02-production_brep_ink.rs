/// chain segment of the face mesh it lies on, for its facing, with how far it rises off that mesh.
pub struct DisplayChain {
    pub points: Vec<[f64; 3]>, // display points on the edge's exact curve, in chain order
    pub spans: Vec<usize>,     // per display segment: the chain segment (keys[k], keys[k+1]) it lies on
    pub sags: Vec<f32>,        // per display segment: its rise off the face mesh, mm; 0 on a straight edge
}

/// Position of one mesh vertex.
fn vertex_position(fm: &Mesh, key: usize) -> [f64; 3] {
    let v = &fm.vertex[&key];
    [v.x, v.y, v.z]
}

/// Distance from `p` to the segment `a`-`b`.
fn segment_distance(p: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let q = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
    let length = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    let t = if length > 0.0 {
        ((q[0] * d[0] + q[1] * d[1] + q[2] * d[2]) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let r = [q[0] - d[0] * t, q[1] - d[1] * t, q[2] - d[2] * t];
    (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()
}

/// Squared distance between two points.
fn distance2(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]) * (a[i] - b[i])).sum()
}

/// Index of the sample nearest `p`.
#[cfg(test)]
fn nearest_index(samples: &[[f64; 3]], p: [f64; 3]) -> usize {
    let mut best = 0;
    let mut best_d = f64::INFINITY;

    for (i, s) in samples.iter().enumerate() {
        let d = distance2(*s, p);

        if d < best_d {
            best_d = d;
            best = i;
        }
    }

    best
}

/// The chain as drawn straight from its mesh vertices: one display segment per chain segment; only
/// for an edge with no 3D curve to draw.
fn coarse_display(coarse: Vec<[f64; 3]>) -> DisplayChain {
    let segments = coarse.len().saturating_sub(1);
    DisplayChain {
        points: coarse,
        spans: (0..segments).collect(),
        sags: vec![0.0; segments],
    }
}

/// The 3D curve of the chain's edge, the curve its pipes are drawn on; None when it has none.
fn edge_curve<'a>(b: &'a BRep, chain: &EdgeChain) -> Option<&'a session_rust::NurbsCurve> {
    let edge = b.m_edges.get(chain.edge)?;

    if edge.degenerated || edge.curve_3d_index < 0 || chain.keys.len() < 2 {
        return None;
    }

    let curve = b.m_curves_3d.get(edge.curve_3d_index as usize)?;

    if curve.cv_count() < 2 {
        return None;
    }

    Some(curve)
}

/// Display chord count of a curved edge: one per `EDGE_DEGREES` of turning, never fewer than
/// the mesh chain has, at most `EDGE_CHORDS_MAX`. A full circle gets 90 wherever it lies: the
/// turning of a moved circle comes out a hair over 360°, which must not round up to 91.
fn display_chords(curve: &session_rust::NurbsCurve, chain_segments: usize) -> usize {
    let by_angle = (turning_degrees(curve) / EDGE_DEGREES - 1e-6).ceil() as usize;
    by_angle
        .max(curve.span_count())
        .max(chain_segments)
        .min(EDGE_CHORDS_MAX.max(chain_segments))
}

/// The parameter of the curve's point nearest `p`: the nearest of `count` even samples, refined
/// by golden section between its two neighbours.
fn curve_parameter(curve: &session_rust::NurbsCurve, p: [f64; 3], count: usize) -> f64 {
    let (t0, t1) = curve.domain();
    let distance = |t: f64| {
        let q = curve.point_at(t);
        distance2([q[0], q[1], q[2]], p)
    };
    let step = (t1 - t0) / count as f64;
    let nearest = (0..=count).min_by(|&i, &j| distance(t0 + step * i as f64).total_cmp(&distance(t0 + step * j as f64))).unwrap_or(0);
    let (mut lo, mut hi) = ((t0 + step * (nearest as f64 - 1.0)).max(t0), (t0 + step * (nearest as f64 + 1.0)).min(t1));
    let ratio = 0.5 * (5.0f64.sqrt() - 1.0);

    for _ in 0..60 {
        let (m1, m2) = (hi - ratio * (hi - lo), lo + ratio * (hi - lo));

        if distance(m1) < distance(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }

    0.5 * (lo + hi)
}

/// The chain's edge drawn from its own exact curve, never from the mesh: every mesh vertex of the
/// chain at its own parameter on the curve, and the curve between two of them cut into chords, one
/// for a straight edge and one per few degrees of turning for a curved one, so every display
/// segment lies on one chain segment of the face mesh, which gives it its facing and its rise off
/// the mesh, the depth it may sink in.
pub fn display_chain(b: &BRep, chain: &EdgeChain, fm: &Mesh) -> DisplayChain {
    if crate::math_quality::current().0.is_finite() {
        match crate::math_cad::boundary(b,chain,fm) {
            Ok(value)=>return DisplayChain{points:value.points,spans:value.spans,sags:value.sags},
            Err(_)=>{crate::math_quality::fail();return coarse_display(Vec::new());}
        }
    }
    let coarse: Vec<[f64; 3]> = chain
        .keys
        .iter()
        .map(|&key| vertex_position(fm, key))
        .collect();
    let Some(curve) = edge_curve(b, chain) else {
        return coarse_display(coarse);
    };
    let segments = coarse.len() - 1;
    let straight = !(turning_degrees(curve) > EDGE_DEGREES) || knobs::coarse_edges();
    let chords = if straight { segments } else { display_chords(curve, segments) };
    let (t0, t1) = curve.domain();
    let period = t1 - t0;

    // every mesh vertex's parameter, unwrapped across the seam of a closed curve so the chain runs on
    let mut parameters: Vec<f64> = coarse.iter().map(|&p| curve_parameter(curve, p, 8 * chords)).collect();

    if curve.is_closed() {
        for k in 1..parameters.len() {
            while parameters[k] - parameters[k - 1] > 0.5 * period {
                parameters[k] -= period;
            }

            while parameters[k] - parameters[k - 1] < -0.5 * period {
                parameters[k] += period;
            }
        }

        // a closed chain ends where it starts, one turn on
        if chain.keys.first() == chain.keys.last() && segments > 1 {
            let turn = if parameters[segments - 1] > parameters[0] { period } else { -period };
            parameters[segments] = parameters[0] + turn;
        }
    }

    let wrap = |t: f64| if curve.is_closed() { t0 + (t - t0).rem_euclid(period) } else { t.clamp(t0, t1) };
    let total: f64 = parameters.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
    let mut points = vec![coarse[0]];
    let mut spans = Vec::with_capacity(chords);
    let mut sags = Vec::with_capacity(chords);

    for k in 0..segments {
        let (a, z) = (coarse[k], coarse[k + 1]);
        let share = if total > 0.0 { (parameters[k + 1] - parameters[k]).abs() / total } else { 0.0 };
        let steps = if straight { 1 } else { ((chords as f64 * share).round() as usize).max(1) };
        let mut previous = a;

        for j in 1..=steps {
            let p = if j == steps {
                let q = curve.point_at(wrap(parameters[k + 1]));
                [q[0], q[1], q[2]]
            } else {
                let q = curve.point_at(wrap(parameters[k] + (parameters[k + 1] - parameters[k]) * j as f64 / steps as f64));
                [q[0], q[1], q[2]]
            };
            let rise = if straight { 0.0 } else { segment_distance(previous, a, z).max(segment_distance(p, a, z)) };
            points.push(p);
            spans.push(k);
            sags.push((rise * SAG_MARGIN) as f32);
            previous = p;
        }
    }

    // the first point the curve's own too
    let q = curve.point_at(wrap(parameters[0]));
    points[0] = [q[0], q[1], q[2]];

    if !points.iter().flatten().all(|v| v.is_finite()) {
        return coarse_display(coarse);
    }

    DisplayChain { points, spans, sags }
