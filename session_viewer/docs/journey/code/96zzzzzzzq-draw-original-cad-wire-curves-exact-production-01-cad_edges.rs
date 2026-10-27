use session_rust::Mesh;
use session_rust::brep::{BRep,BRepOrientation};

fn sample_order(a: &f64, b: &f64) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

fn parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    sample_order(&a.0, &b.0).then(a.1.cmp(&b.1))
}

fn total_parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    a.0.total_cmp(&b.0).then(a.1.cmp(&b.1))
}

fn same_parameter(a: &mut (f64, usize), b: &mut (f64, usize)) -> bool {
    a.0 == b.0
}

fn parse_sample_index(value: &str) -> Option<usize> {
    value.parse().ok()
}

pub struct EdgeUse {
    pub edge: usize,
    pub face: usize,
    pub orientation: BRepOrientation,
}

fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);

    if ci < 0 {
        return None;
    }

    let c = b.m_curves_2d.get(ci as usize)?;

    if c.degree() != 1 || c.is_rational() || c.cv_count() != 2 {
        return None;
    }

    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

fn sample_values(fm: &Mesh, name: &str) -> Vec<f64> {
    let mut vals = Vec::new();

    for vertex in fm.vertex.values() {
        if let Some(value) = vertex.attributes.get(name) {
            vals.push(*value);
        }
    }

    vals.sort_unstable_by(sample_order);
    vals.dedup();
    vals
}

fn nearest_sample(vals: &[f64], target: f64, wrap: Option<(f64, f64)>) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;

    for &v in vals {
        let mut d = (v - target).abs();

        if let Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }

        if match best {
            Some((bd, _)) => d < bd,
            None => true,
        } {
            best = Some((d, v));
        }
    }

    Some(best?.1)
}

pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let e = b.m_edges.get(eu.edge)?;

    if e.degenerated {
        return None;
    }

    let (p0, p1) = pcurve_ends(b, eu)?;

    if !p0.into_iter().chain(p1).all(f64::is_finite) || (p0[0] != p1[0] && p0[1] != p1[1]) {
        return None;
    }

    let fixed = if (p1[0] - p0[0]).abs() <= (p1[1] - p0[1]).abs() {
        0
    } else {
        1
    };
    let (fixed_name, free_name) = if fixed == 0 { ("u", "v") } else { ("v", "u") };
    let vals = sample_values(fm, fixed_name);

    if vals.is_empty() {
        return None;
    }

    let face = b.m_faces.get(eu.face)?;
    let srf = b.m_surfaces.get(face.surface_index as usize)?;
    let wrap = if srf.is_closed(fixed) {
        srf.domain(fixed)
    } else {
        None
    };
    let at = nearest_sample(&vals, p0[fixed], wrap)?;
    let wrapped_start = match wrap {
        Some((start, end)) => p0[fixed] - (end - start) == at,
        None => false,
    };

    if at != p0[fixed] && !wrapped_start {
        return None;
    }

    let mut on_line: Vec<(f64, usize)> = Vec::new();

    for (&key, vd) in fm.vertex.iter() {
        let (Some(&f), Some(&t)) = (vd.attributes.get(fixed_name), vd.attributes.get(free_name))
        else {
            continue;
        };
        let lo = p0[1 - fixed].min(p1[1 - fixed]);
        let hi = p0[1 - fixed].max(p1[1 - fixed]);

        if f == at && t >= lo && t <= hi {
            on_line.push((t, key));
        }
    }

    if on_line.len() < 2 {
        return None;
    }

    on_line.sort_unstable_by(parameter_order);
    let mut keys = Vec::with_capacity(on_line.len());

    for (_, key) in on_line {
        keys.push(key);
    }

    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }

    Some(keys)
}

pub struct EdgeChain {
    pub edge: usize,
    pub face: usize,
    pub keys: Vec<usize>,
    pub other: Option<usize>,
}

fn constrained_chain(fm: &Mesh, edge: usize) -> Option<Vec<usize>> {
    let prefix = format!("brep_edge/{edge}/");
    let mut uses =
        std::collections::BTreeMap::<usize, std::collections::BTreeMap<usize, usize>>::new();

    for (&key, vertex) in &fm.vertex {
        for name in vertex.attributes.keys() {
            let Some(suffix) = name.strip_prefix(&prefix) else {
                continue;
            };
            let Some((use_id, sample)) = suffix.split_once('/') else {
                continue;
            };
            let (Ok(use_id), Ok(sample)) = (use_id.parse::<usize>(), sample.parse::<usize>())
            else {
                continue;
            };
            let value = uses.entry(use_id).or_default().entry(sample).or_insert(key);
            *value = (*value).min(key);
        }
    }

    for (&use_id, samples) in &uses {
        if samples.len() < 2 {
            continue;
        }

        let mut keys = Vec::with_capacity(samples.len());

        for (expected, (&sample, &key)) in samples.iter().enumerate() {
            if expected != sample {
                return None;
            }

            keys.push(key);
        }

        let interval_prefix = format!("brep_edge_interval/{edge}/{use_id}/");
        let mut ordered = Vec::with_capacity(keys.len());

        for (sample, key) in keys.into_iter().enumerate() {
            ordered.push((sample as f64, key));
        }

        for (&key, vertex) in &fm.vertex {
            for name in vertex.attributes.keys() {
                if let Some(sample) = name
                    .strip_prefix(&interval_prefix)
                    .and_then(parse_sample_index)
                    && let Some(&t) = vertex.attributes.get(name)
                    && sample + 1 < samples.len()
                    && t.is_finite()
                    && t > 0.0
                    && t < 1.0
                {
                    ordered.push((sample as f64 + t, key));
                }
            }
        }

        ordered.sort_unstable_by(total_parameter_order);
        ordered.dedup_by(same_parameter);
        let mut keys = Vec::with_capacity(ordered.len());

        for (_, key) in ordered {
            keys.push(key);
        }

        return Some(keys);
    }

    None
}

fn chain_on_edge(b: &BRep, edge: usize, fm: &Mesh, keys: &[usize]) -> bool {
    let Some(curve) = b.m_edges.get(edge).filter(|e| e.curve_3d_index >= 0)
        .and_then(|e| b.m_curves_3d.get(e.curve_3d_index as usize)) else { return true; };
    let (Some(first), Some(last)) = (keys.first(), keys.last()) else { return false; };
    let start = curve.point_at_start(); let end = curve.point_at_end();
    let (start, end) = ([start[0], start[1], start[2]], [end[0], end[1], end[2]]);
    let position = |key: usize| { let v = &fm.vertex[&key]; [v.x, v.y, v.z] };
    let (a, z) = (position(*first), position(*last));
    let scale = start.iter().chain(end.iter()).fold(1.0f64, |m, v| m.max(v.abs()));
    let tolerance = 1e-6 * scale;
    let near = |p: [f64; 3], q: [f64; 3]| (0..3).map(|i| (p[i]-q[i])*(p[i]-q[i])).sum::<f64>().sqrt() <= tolerance;
    (near(a, start) && near(z, end)) || (near(a, end) && near(z, start))
}

pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>> {
    let mut out = Vec::with_capacity(b.m_edges.len());

    for (ei, e) in b.m_edges.iter().enumerate() {
        if e.degenerated {
            out.push(None);
            continue;
        }

        let uses = b.edge_faces(ei);
        let mut found: Option<EdgeChain> = None;

        for (k, u) in uses.iter().enumerate() {
            let eu = EdgeUse {
                edge: ei,
                face: u.index as usize,
                orientation: u.orientation,
            };
            let keys = match constrained_chain(&fms[eu.face], ei) {
                Some(keys) => keys,
                None => match iso_chain(b, &fms[eu.face], &eu) {
                    Some(keys) => keys,
                    None => continue,
                },
            };
            if !chain_on_edge(b, ei, &fms[eu.face], &keys) { continue; }
            let mut other = None;

            for (j, candidate) in uses.iter().enumerate() {
                if j != k && candidate.index as usize != eu.face {
                    other = Some(candidate.index as usize);
                    break;
                }
            }

            found = Some(EdgeChain {
                edge: ei,
                face: eu.face,
                keys,
                other,
            });
            break;
        }

        out.push(found);
    }

    out
}

