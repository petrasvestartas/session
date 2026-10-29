use session_rust::Mesh;
use session_rust::brep::{BRep, BRepOrientation};

use super::curves::turning_degrees;
use super::encode::{Pen, pack_facing};
use crate::app::knobs;
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;

/// Degrees of turning one display chord of a curved edge may span: how round a circle looks
/// depends only on its radius on screen, and 90 chords keep the sag of one under 0.3 px while
/// it fills a 1000 px screen.
const EDGE_DEGREES: f64 = 4.0;

/// Most display chords one edge gets.
const EDGE_CHORDS_MAX: usize = 256;

/// The sag of a display segment is reported this many times its measured rise off the chord,
/// so the neighbouring facet's plane, which tilts away from the chord, is covered too.
const SAG_MARGIN: f64 = 2.0;

/// Order two floats, NaN counts as equal.
fn sample_order(a: &f64, b: &f64) -> std::cmp::Ordering {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
}

/// Order by parameter, then by vertex key.
fn parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    sample_order(&a.0, &b.0).then(a.1.cmp(&b.1))
}

/// Order by parameter with a total float order, then by key.
fn total_parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    a.0.total_cmp(&b.0).then(a.1.cmp(&b.1))
}

/// True when two samples share a parameter.
fn same_parameter(a: &mut (f64, usize), b: &mut (f64, usize)) -> bool {
    a.0 == b.0
}

/// Parse a sample index, None when not a number.
fn parse_sample_index(value: &str) -> Option<usize> {
    value.parse().ok()
}

/// One use of an edge by a face.
pub struct EdgeUse {
    pub edge: usize,                  // edge index
    pub face: usize,                  // face index
    pub orientation: BRepOrientation, // which way the face runs it
}

/// The two UV ends of a straight pcurve.
fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);

    if ci < 0 {
        return None;
    }

    let c = b.m_curves_2d.get(ci as usize)?;

    if c.degree() != 1 || c.is_rational() || c.cv_count() != 2 {
        return None; // not a straight line
    }

    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

/// Sorted distinct values of one vertex attribute.
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

/// The sample value nearest `target`, also across a wrap.
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

/// Vertex keys along a grid edge, in parameter order.
pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let e = b.m_edges.get(eu.edge)?;

    if e.degenerated {
        return None;
    }

    let (p0, p1) = pcurve_ends(b, eu)?;

    if !p0.into_iter().chain(p1).all(f64::is_finite) || (p0[0] != p1[0] && p0[1] != p1[1]) {
        return None;
    }

    // the parameter that does not change along the edge
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
        srf.domain(fixed) // closed: end equals start
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

    let mut on_line: Vec<(f64, usize)> = Vec::new(); // (free parameter, key)

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
        keys.push(keys[0]); // closed edge
    }

    Some(keys)
}

/// The mesh vertices one BRep edge runs along.
pub struct EdgeChain {
    pub edge: usize,          // BRep edge index
    pub face: usize,          // face mesh the keys belong to
    pub keys: Vec<usize>,     // vertex keys along the edge
    pub other: Option<usize>, // the face on the other side
}

/// Vertex keys along edge `edge` from the mesher's labels.
fn constrained_chain(fm: &Mesh, edge: usize) -> Option<Vec<usize>> {
    let prefix = format!("brep_edge/{edge}/");
    let mut uses =
        std::collections::BTreeMap::<usize, std::collections::BTreeMap<usize, usize>>::new(); // use id -> sample -> key

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
            *value = (*value).min(key); // smallest key wins
        }
    }

    for (&use_id, samples) in &uses {
        if samples.len() < 2 {
            continue;
        }

        let mut keys = Vec::with_capacity(samples.len());

        for (expected, (&sample, &key)) in samples.iter().enumerate() {
            if expected != sample {
                return None; // a sample is missing
            }

            keys.push(key);
        }

        // vertices inserted between samples carry a 0..1 position
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

/// Whether a chain runs between its edge's own two ends, in either direction: the mesh vertices
/// tagged for the edge may be wrong, and a chain that is not on its edge must not lend it facets.
fn chain_on_edge(b: &BRep, edge: usize, fm: &Mesh, keys: &[usize]) -> bool {
    let Some(curve) = b
        .m_edges
        .get(edge)
        .filter(|e| e.curve_3d_index >= 0)
        .and_then(|e| b.m_curves_3d.get(e.curve_3d_index as usize))
    else {
        return true;
    };
    let (Some(first), Some(last)) = (keys.first(), keys.last()) else {
        return false;
    };
    let start = curve.point_at_start();
    let end = curve.point_at_end();
    let (start, end) = ([start[0], start[1], start[2]], [end[0], end[1], end[2]]);
    let (a, z) = (vertex_position(fm, *first), vertex_position(fm, *last));
    let scale = start.iter().chain(end.iter()).fold(1.0f64, |m, v| m.max(v.abs()));
    let tolerance = 1e-6 * scale;
    let near = |p: [f64; 3], q: [f64; 3]| distance2(p, q).sqrt() <= tolerance;

    (near(a, start) && near(z, end)) || (near(a, end) && near(z, start))
}

/// One chain per BRep edge, None when no mesh carries it.
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

            if !chain_on_edge(b, ei, &fms[eu.face], &keys) {
                continue;
            }

            // the neighbouring face, if a different one
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

/// A BRep edge as it is drawn: points along its exact curve, every display segment mapped to the
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
fn curve_parameter(curve: &session_rust::NurbsCurve, p: [f64; 3], scouts: &[[f64; 3]]) -> f64 {
    let count = scouts.len() - 1;
    let (t0, t1) = curve.domain();
    let distance = |t: f64| {
        let q = curve.point_at(t);
        distance2([q[0], q[1], q[2]], p)
    };
    let step = (t1 - t0) / count as f64;
    let nearest = (0..=count).min_by(|&i, &j| distance2(scouts[i], p).total_cmp(&distance2(scouts[j], p))).unwrap_or(0);
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
    display_chain_quality(b, chain, fm, Default::default())
}

pub fn display_chain_quality(b: &BRep, chain: &EdgeChain, fm: &Mesh, quality: super::display::Quality) -> DisplayChain {
    let coarse: Vec<[f64; 3]> = chain
        .keys
        .iter()
        .map(|&key| vertex_position(fm, key))
        .collect();
    let Some(curve) = edge_curve(b, chain) else {
        return coarse_display(coarse);
    };
    let segments = coarse.len() - 1;
    let finite_quality = quality.tolerance.is_finite();
    let straight = if finite_quality {
        curve.degree() == 1 && curve.cv_count() == 2
    } else {
        !(turning_degrees(curve) > EDGE_DEGREES) || knobs::coarse_edges()
    };
    let chords = if straight { segments } else { display_chords(curve, segments) };
    let (t0, t1) = curve.domain();
    let period = t1 - t0;

    // every mesh vertex's parameter, unwrapped across the seam of a closed curve so the chain runs on
    let count = (8 * chords).clamp(8, super::display::CURVE_SEGMENTS * 8);
    let scouts: Vec<[f64; 3]> = (0..=count).map(|i| {
        let p = curve.point_at(t0 + period * i as f64 / count as f64); [p[0], p[1], p[2]]
    }).collect();
    let mut parameters: Vec<f64> = coarse.iter().map(|&p| curve_parameter(curve, p, &scouts)).collect();

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
    let sampled = (quality.tolerance.is_finite() && segments + 2 < super::display::CURVE_SEGMENTS)
        .then(|| super::display::sample(curve, quality, super::display::CURVE_SEGMENTS - segments - 2));
    let first = curve.point_at(wrap(parameters[0]));
    let mut points = vec![[first[0], first[1], first[2]]];
    let mut spans = Vec::with_capacity(chords);
    let mut sags = Vec::with_capacity(chords);

    for k in 0..segments {
        let (a, z) = (coarse[k], coarse[k + 1]);
        let share = if total > 0.0 { (parameters[k + 1] - parameters[k]).abs() / total } else { 0.0 };
        let steps = if straight || sampled.is_some() { 1 } else { ((chords as f64 * share).round() as usize).max(1) };
        let mut cuts: Vec<f64> = (1..=steps).map(|j| parameters[k] + (parameters[k + 1] - parameters[k]) * j as f64 / steps as f64).collect();
        if let Some(sampled) = &sampled {
            for &t in &sampled.parameters {
                // Match the unwrapped interval of a closed chain without adding seam copies.
                let t = if curve.is_closed() { t + ((parameters[k] - t) / period).round() * period } else { t };
                if t > parameters[k].min(parameters[k + 1]) && t < parameters[k].max(parameters[k + 1]) {
                    cuts.push(t);
                }
            }
            cuts.sort_by(f64::total_cmp);
            if parameters[k + 1] < parameters[k] { cuts.reverse(); }
            cuts.dedup_by(|a, b| (*a - *b).abs() < period.abs() * 1e-12);
        }
        let mut previous = *points.last().unwrap();

        for t in cuts {
            let q = curve.point_at(wrap(t));
            let p = [q[0], q[1], q[2]];
            let rise = if straight && !finite_quality { 0.0 } else { segment_distance(previous, a, z).max(segment_distance(p, a, z)) };
            points.push(p);
            spans.push(k);
            sags.push((rise * SAG_MARGIN) as f32);
            previous = p;
        }
    }

    if !points.iter().flatten().all(|v| v.is_finite()) {
        return coarse_display(coarse);
    }

    DisplayChain { points, spans, sags }
}

/// A triangle edge keyed by its two end positions.
type FacetEdge = [[u64; 3]; 2];

/// The triangles meeting at one edge.
#[derive(Default)]
struct FacetPair {
    normals: [Option<[f64; 3]>; 2], // first two triangle normals
    count: usize,                   // how many triangles in total
}

/// Position as bits, -0 same as 0.
fn position_bits(position: [f64; 3]) -> [u64; 3] {
    let mut bits = [0; 3];

    for axis in 0..3 {
        bits[axis] = if position[axis] == 0.0 {
            0
        } else {
            position[axis].to_bits()
        };
    }

    bits
}

/// Edge key for two positions, in either order.
fn facet_edge(a: [f64; 3], b: [f64; 3]) -> FacetEdge {
    let a = position_bits(a);
    let b = position_bits(b);

    if a <= b { [a, b] } else { [b, a] }
}

/// Triangle normals at every edge of one face mesh.
fn face_facets(mesh: &Mesh) -> std::collections::HashMap<FacetEdge, FacetPair> {
    let mut result = std::collections::HashMap::<FacetEdge, FacetPair>::new();
    let mut faces: Vec<_> = mesh.face.keys().copied().collect();
    faces.sort_unstable();

    for key in faces {
        let vertices = &mesh.face[&key];

        if vertices.len() != 3 {
            continue;
        }

        let mut p = [[0.0; 3]; 3];

        for corner in 0..3 {
            let v = &mesh.vertex[&vertices[corner]];
            p[corner] = [v.x, v.y, v.z];
        }

        let mut a = [0.0; 3];
        let mut b = [0.0; 3];

        for axis in 0..3 {
            a[axis] = p[1][axis] - p[0][axis];
            b[axis] = p[2][axis] - p[0][axis];
        }

        let mut normal = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

        if !length.is_finite() || length <= 0.0 {
            continue;
        }

        for component in &mut normal {
            *component /= length;
        }

        for edge in 0..3 {
            let pair = result
                .entry(facet_edge(p[edge], p[(edge + 1) % 3]))
                .or_default();

            if pair.count < 2 {
                pair.normals[pair.count] = Some(normal);
            }

            pair.count += 1;
        }
    }

    result
}

/// The outward normals of the two faces beside one pipe, before they are packed into its facing
/// word; None where the facing is unknown.
pub type FacingPair = (Option<[f64; 3]>, Option<[f64; 3]>);

/// What every edge pipe of one BRep needs.
pub struct EdgePen<'a> {
    pub fms: &'a [Mesh],                                          // face meshes
    pub signs: &'a [f64],                                         // +1 or -1 per face
    pub pen: Pen,                                                 // row, width, colour
    facets: Vec<std::collections::HashMap<FacetEdge, FacetPair>>, // per face: triangles at each edge
    normals: Option<std::cell::RefCell<Vec<FacingPair>>>,         // pipe normals, when recording
}

impl<'a> EdgePen<'a> {
    /// Index the triangles of every face once.
    pub fn new(fms: &'a [Mesh], signs: &'a [f64], pen: Pen) -> Self {
        let mut facets = Vec::with_capacity(fms.len());

        for mesh in fms {
            facets.push(face_facets(mesh));
        }

        Self {
            fms,
            signs,
            pen,
            facets,
            normals: None,
        }
    }

    /// Also keep the exact normals of every pipe pushed, so a congruent copy can turn them.
    pub fn recording(mut self) -> Self {
        self.normals = Some(std::cell::RefCell::new(Vec::new()));
        self
    }

    /// The normals kept since `recording`, one pair per pipe pushed.
    pub fn take_normals(&self) -> Vec<FacingPair> {
        self.normals
            .as_ref()
            .map(|normals| normals.take())
            .unwrap_or_default()
    }

    /// Facing word of one pipe; unknown when ambiguous.
    fn facing(&self, chain: &EdgeChain, a: [f64; 3], b: [f64; 3]) -> u32 {
        self.facing_word(self.facing_pair(chain, a, b))
    }

    /// Facing word of two normals, kept when recording.
    fn facing_word(&self, (first, second): FacingPair) -> u32 {
        if let Some(normals) = &self.normals {
            normals.borrow_mut().push((first, second));
        }

        pack_facing(first.as_ref(), second.as_ref())
    }

    /// Outward normals of the faces beside one pipe; none when ambiguous.
    fn facing_pair(&self, chain: &EdgeChain, a: [f64; 3], b: [f64; 3]) -> FacingPair {
        let key = facet_edge(a, b);
        let Some(owner) = self.facets[chain.face].get(&key) else {
            return (None, None);
        };

        if owner.count == 0 || owner.count > 2 {
            return (None, None);
        }

        let first = scaled_normal(owner.normals[0], self.signs[chain.face]);
        let second = if let Some(other) = chain.other {
            let Some(pair) = self.facets[other].get(&key) else {
                return (None, None);
            };

            if pair.count != 1 || owner.count != 1 {
                return (None, None);
            }

            scaled_normal(pair.normals[0], self.signs[other])
        } else if owner.count == 2 {
            scaled_normal(owner.normals[1], self.signs[chain.face])
        } else {
            first
        };

        (first, second)
    }
}

/// A normal turned outward by its face's sign.
fn scaled_normal(normal: Option<[f64; 3]>, sign: f64) -> Option<[f64; 3]> {
    let n = normal?;
    Some([n[0] * sign, n[1] * sign, n[2] * sign])
}

/// One pipe per display segment, each facing the facets of the chain segment it lies on;
/// returns that chain segment for every pipe pushed, in pipe order.
pub fn push_edge_pipes(
    seg: &mut SegRows,
    chain: &EdgeChain,
    display: &DisplayChain,
    ep: &EdgePen,
    bounds: &mut AABB,
) -> Vec<usize> {
    let fm = &ep.fms[chain.face];
    let first = seg.pipes.len() as u32;
    seg.pipes.reserve(display.spans.len());
    seg.pipe_ids.resize(seg.pipes.len(), u32::MAX);
    seg.pipe_sags.resize(seg.pipes.len(), 0.0);
    let mut spans = Vec::with_capacity(display.spans.len());

    for (j, w) in display.points.windows(2).enumerate() {
        let p0f = super::curves::render_position(w[0]);
        let p1f = super::curves::render_position(w[1]);

        // skip a segment that collapses in f32
        if p0f == p1f || !p0f.into_iter().chain(p1f).all(f32::is_finite) {
            continue;
        }

        let span = display.spans[j];
        let a = vertex_position(fm, chain.keys[span]);
        let b = vertex_position(fm, chain.keys[span + 1]);
        bounds.union_with_point(p0f[0] as f64, p0f[1] as f64, p0f[2] as f64);
        bounds.union_with_point(p1f[0] as f64, p1f[1] as f64, p1f[2] as f64);
        seg.pipes.push(CylinderSegment {
            p0: p0f,
            radius: ep.pen.radius,
            p1: p1f,
            instance_id: ep.pen.row,
            color: ep.pen.color,
            facing: ep.facing(chain, a, b),
        });
        seg.pipe_ids
            .push(u32::try_from(chain.edge).unwrap_or(u32::MAX)); // edge id for picking
        seg.pipe_sags.push(display.sags[j]);
        spans.push(span);
    }

    seg.pipe_chains.push(first..seg.pipes.len() as u32); // one joined stroke
    spans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::brep::QUALITY;

    /// Labelled samples and inserted nodes form one ordered chain.
    #[test]
    fn constrained_chain_keeps_inserted_crease_nodes_and_source_identity() {
        let mut mesh = Mesh::new();

        for (key, name, value) in [
            (10, "brep_edge/7/2/0", 1.0),
            (20, "brep_edge_interval/7/2/0", 0.25),
            (21, "brep_edge_interval/7/2/0", 0.25),
            (30, "brep_edge/7/2/1", 1.0),
        ] {
            mesh.add_vertex(session_rust::Point::new(key as f64, 0.0, 0.0), Some(key));
            mesh.vertex
                .get_mut(&key)
                .unwrap()
                .attributes
                .insert(name.to_string(), value);
        }

        assert_eq!(constrained_chain(&mesh, 7), Some(vec![10, 20, 30]));
        assert_eq!(constrained_chain(&mesh, 8), None);
        mesh.vertex
            .get_mut(&30)
            .unwrap()
            .attributes
            .remove("brep_edge/7/2/1");
        assert_eq!(constrained_chain(&mesh, 7), None);
    }

    /// First use of every edge.
    fn first_uses(b: &BRep) -> Vec<EdgeUse> {
        let mut out = Vec::new();

        for ei in 0..b.m_edges.len() {
            let uses = b.edge_faces(ei);
            let Some(u) = uses.first() else { continue };
            out.push(EdgeUse {
                edge: ei,
                face: u.index as usize,
                orientation: u.orientation,
            });
        }

        out
    }

    /// The chain starts exactly on BRep vertex `vi`.
    fn ends_on(b: &BRep, fm: &Mesh, keys: &[usize], vi: usize) -> bool {
        let p = fm.vertex[&keys[0]].position();
        let v = &b.m_vertices[vi].point;
        p[0] == v[0] && p[1] == v[1] && p[2] == v[2]
    }

    /// A cylinder: two closed circles and a two-key seam.
    #[test]
    fn cylinder_chains_follow_the_grid() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let uses = first_uses(&b);
        let chains: Vec<Vec<usize>> = uses
            .iter()
            .map(|u| iso_chain(&b, &fms[u.face], u).expect("grid use"))
            .collect();
        assert_eq!(chains.len(), 3);

        for k in 0..2 {
            assert_eq!(chains[k].first(), chains[k].last());
            assert!(chains[k].len() > 4);
            assert!(ends_on(
                &b,
                &fms[uses[k].face],
                &chains[k],
                b.m_edges[k].start_vertex as usize
            ));
        }

        assert_eq!(chains[0].len(), chains[1].len());
        assert_eq!(chains[2].len(), 2);
        assert!(ends_on(&b, &fms[uses[2].face], &chains[2], 0));
        let top = fms[uses[2].face].vertex[chains[2].last().unwrap()].position();
        assert_eq!(
            [top[0], top[1], top[2]],
            [
                b.m_vertices[1].point[0],
                b.m_vertices[1].point[1],
                b.m_vertices[1].point[2]
            ]
        );
    }

    /// A sphere's seam runs from pole to pole.
    #[test]
    fn sphere_seam_reaches_both_poles() {
        let b = BRep::create_sphere(180.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let uses = first_uses(&b);
        let seam = iso_chain(&b, &fms[0], &uses[0]).expect("seam");
        assert!(seam.len() > 10);
        assert_ne!(seam.first(), seam.last());
        assert!(ends_on(&b, &fms[0], &seam, 0));
        let north = fms[0].vertex[seam.last().unwrap()].position();
        assert_eq!(north[2], b.m_vertices[1].point[2]);
        assert!(iso_chain(&b, &fms[0], &uses[1]).is_none());
        assert!(iso_chain(&b, &fms[0], &uses[2]).is_none());
    }

    /// A torus has two closed seams; a cap has no iso chain.
    #[test]
    fn torus_seams_close_and_cdt_faces_decline() {
        let b = BRep::create_torus(220.0, 70.0);
        let fms = b.face_meshes_q(Some(QUALITY));

        for u in first_uses(&b) {
            let c = iso_chain(&b, &fms[u.face], &u).expect("seam");
            assert_eq!(c.first(), c.last());
            assert!(ends_on(&b, &fms[u.face], &c, 0));
        }

        let cyl = BRep::create_cylinder(150.0, 400.0);
        let cfm = cyl.face_meshes_q(Some(QUALITY));
        let cap = cyl.edge_faces(0)[1];
        let eu = EdgeUse {
            edge: 0,
            face: cap.index as usize,
            orientation: cap.orientation,
        };
        assert!(iso_chain(&cyl, &cfm[eu.face], &eu).is_none());
    }

    /// Every real edge of the mixed scene gets a chain.
    #[test]
    fn every_solid_edge_has_a_chain() {
        let solids = [
            BRep::create_box(400.0, 300.0, 250.0),
            BRep::create_cylinder(150.0, 400.0),
            BRep::create_cone(150.0, 400.0),
            BRep::create_sphere(180.0),
            BRep::create_torus(220.0, 70.0),
            BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0),
            BRep::create_pyramid(400.0, 350.0),
        ];

        for b in &solids {
            let fms = b.face_meshes_q(Some(QUALITY));
            let chains = edge_chains(b, &fms);
            assert_eq!(chains.len(), b.m_edges.len());

            for (ei, e) in b.m_edges.iter().enumerate() {
                let uses = b.edge_faces(ei);

                match &chains[ei] {
                    None => assert!(e.degenerated, "{} edge {ei}", b.name),
                    Some(c) => {
                        assert!(
                            uses.iter().any(|u| u.index as usize == c.face),
                            "{} edge {ei}",
                            b.name
                        );
                        let other = uses
                            .iter()
                            .find(|u| u.index as usize != c.face)
                            .map(|u| u.index as usize);
                        assert_eq!(c.other, other, "{} edge {ei}", b.name);
                    }
                }
            }
        }
    }

    /// The teapot's front meridian stays visible.
    #[test]
    fn teapot_front_meridian_is_not_self_occluded() {
        use session_rust::{Line, Point, Session};
        let scene = Session::pb_load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/pb/view_mixed_teapot.pb"
        ))
        .unwrap();
        let brep = &scene.objects.breps[0];
        let meshes = brep.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(brep, &meshes);
        let chain = chains[14].as_ref().expect("authored front meridian");
        let owner = &meshes[chain.face];
        let mut triangles = Vec::new();

        for mesh in &meshes {
            assert!(!mesh.face.is_empty(), "every authored patch remains meshed");

            for keys in mesh.face.values() {
                triangles.push(
                    keys.iter()
                        .map(|key| mesh.vertex[key].position())
                        .collect::<Vec<_>>(),
                );
            }
        }

        let eye = Point::new(-222.278644, -422.587076, 439.224717);
        let mut checked = 0;

        for pair in chain.keys.windows(2) {
            let a = owner.vertex[&pair[0]].position();
            let b = owner.vertex[&pair[1]].position();
            let at = Point::new(
                (a[0] + b[0]) * 0.5,
                (a[1] + b[1]) * 0.5,
                (a[2] + b[2]) * 0.5,
            );
            assert!(at[0].abs() < 1e-9 && at[1] < -140.0 && at[2] >= 90.0 && at[2] <= 240.0);
            let ray = Line::new(eye[0], eye[1], eye[2], at[0], at[1], at[2]);
            let distance = eye.distance(&at, None);

            for triangle in &triangles {
                if let Some(hit) = session_rust::intersection::ray_triangle(
                    &ray,
                    &triangle[0],
                    &triangle[1],
                    &triangle[2],
                    1e-12,
                ) {
                    assert!(
                        eye.distance(&hit, None) >= distance - 1e-6,
                        "front meridian at {:?} buried by {:?}",
                        at,
                        triangle
                    );
                }
            }

            checked += 1;
        }

        assert!(checked >= 8);
    }

    /// Cylinder pipes carry both neighbouring face normals.
    #[test]
    fn pipes_face_both_adjacent_faces() {
        use crate::app::walk::encode::{FACING_UNKNOWN, Pen, encode_width, pack_rgba};
        use crate::engine::gpu::segments::SegRows;
        use session_rust::AABB;
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let signs = vec![1.0; fms.len()];
        let ep = EdgePen::new(
            &fms,
            &signs,
            Pen {
                row: 3,
                radius: encode_width(1.0),
                color: pack_rgba([0.0, 0.0, 0.0, 1.0]),
            },
        );
        let mut seg = SegRows::default();
        let mut bounds = AABB::empty();
        let chain = chains[0].as_ref().unwrap();
        let circle = push_edge_pipes(&mut seg, chain, &display_chain(&b, chain, &fms[chain.face]), &ep, &mut bounds);
        assert!(circle.len() >= chain.keys.len() - 1);
        assert_eq!(circle.first(), Some(&0));
        assert_eq!(circle.last(), Some(&(chain.keys.len() - 2)));
        assert!(circle.windows(2).all(|w| w[1] == w[0] || w[1] == w[0] + 1), "pipes follow the chain in order");

        for p in &seg.pipes {
            assert_ne!(
                p.facing, FACING_UNKNOWN,
                "missing incident facet for {:?} → {:?}",
                p.p0, p.p1
            );
            assert_ne!(p.facing & 0xffff, p.facing >> 16);
            assert_eq!(p.instance_id, 3);
        }

        let before = seg.pipes.len();
        let chain = chains[2].as_ref().unwrap();
        let seam = push_edge_pipes(&mut seg, chain, &display_chain(&b, chain, &fms[chain.face]), &ep, &mut bounds);
        assert_eq!(seam, vec![0]);
        let p = &seg.pipes[before];
        assert_ne!(p.facing & 0xffff, p.facing >> 16);
        assert!(seg.ribbons.is_empty());
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        assert_eq!(seg.pipe_sags.len(), seg.pipes.len());
        assert!(seg.pipe_ids[..before].iter().all(|&id| id == 0));
        assert_eq!(seg.pipe_ids[before], 2);
        assert_eq!(seg.pipe_sags[before], 0.0, "a straight seam lies on its faces");
        assert!(seg.pipe_sags[..before].iter().all(|&sag| sag >= 0.0));
    }

    /// The cone seam faces by its triangles, not the apex fan.
    #[test]
    fn cone_seam_facing_uses_both_incident_facets() {
        let cone = BRep::create_cone(150.0, 400.0);
        let meshes = cone.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&cone, &meshes);
        let signs = vec![1.0; meshes.len()];
        let pen = EdgePen::new(
            &meshes,
            &signs,
            Pen {
                row: 4,
                radius: 1.0,
                color: 0,
            },
        );
        let mut segments = SegRows::default();
        let chain = chains[1].as_ref().unwrap();
        push_edge_pipes(
            &mut segments,
            chain,
            &display_chain(&cone, chain, &meshes[chain.face]),
            &pen,
            &mut AABB::empty(),
        );
        assert_eq!(segments.pipes.len(), 1);
        assert_eq!(segments.pipe_ids, vec![1]);
        let facing = segments.pipes[0].facing;
        assert_ne!(facing & 0xffff, facing >> 16);

        for code in [facing & 0xffff, facing >> 16] {
            // Both cone facet normals occupy the positive-Z octahedron hemisphere.
            let x = (code as u8 as i8) as f64 / 127.0;
            let y = ((code >> 8) as u8 as i8) as f64 / 127.0;
            let z = 1.0 - x.abs() - y.abs();
            let length = (x * x + y * y + z * z).sqrt();
            assert!(
                (z / length - 150.0f64 / (150.0f64.powi(2) + 400.0f64.powi(2)).sqrt()).abs() < 0.02,
                "physical cone facet normal must retain its slope; apex shading average gives z=0.822"
            );
        }
    }

    /// A segment that collapses in f32 pushes no pipe.
    #[test]
    fn collapsed_display_segments_do_not_create_pick_targets() {
        let mut b = BRep::create_box(40.0, 30.0, 25.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let chain = chains[0].as_ref().unwrap();
        let curve = &mut b.m_curves_3d[b.m_edges[chain.edge].curve_3d_index as usize];

        for index in 0..curve.cv_count() {
            curve.set_cv(index, &session_rust::Point::new(1e20, 1e20, 1e20));
        }

        let signs = vec![1.0; fms.len()];
        let ep = EdgePen::new(
            &fms,
            &signs,
            Pen {
                row: 0,
                radius: 1.0,
                color: 0,
            },
        );
        let mut seg = SegRows::default();
        let display = display_chain(&b, chain, &fms[chain.face]);
        assert!(push_edge_pipes(&mut seg, chain, &display, &ep, &mut AABB::empty()).is_empty());
        assert!(seg.pipes.is_empty());
        assert!(seg.pipe_ids.is_empty());
    }

    #[test]
    fn zoomed_shallow_curves_keep_their_measured_depth_envelope() {
        use session_rust::{NurbsCurve, Point};
        for offset in [0.01, 1e-8] {
            let mut brep = BRep::create_box(40.0, 30.0, 25.0);
            let meshes = brep.face_meshes_q(Some(QUALITY));
            let chains = edge_chains(&brep, &meshes);
            let chain = chains[0].as_ref().unwrap();
            let curve_index = brep.m_edges[chain.edge].curve_3d_index as usize;
            let curve = &brep.m_curves_3d[curve_index];
            let (start, end) = curve.domain();
            let (a, b) = (curve.point_at(start), curve.point_at(end));
            let mut middle = Point::new((a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5);
            let axis = if (a[2] - b[2]).abs() < 1e-12 { 2 } else { 0 };
            middle[axis] += offset;
            brep.m_curves_3d[curve_index] = NurbsCurve::create(false, 2, &[a, middle, b]);
            assert!(turning_degrees(&brep.m_curves_3d[curve_index]) < EDGE_DEGREES);
            let legacy = display_chain(&brep, chain, &meshes[chain.face]);
            assert!(legacy.sags.iter().all(|sag| *sag == 0.0));
            let display = display_chain_quality(&brep, chain, &meshes[chain.face], super::super::display::Quality { tolerance: offset / 100.0 });
            assert!(display.sags.iter().any(|sag| *sag > 0.0));
            for (i, ends) in display.points.windows(2).enumerate() {
                let span = display.spans[i];
                let a = vertex_position(&meshes[chain.face], chain.keys[span]);
                let b = vertex_position(&meshes[chain.face], chain.keys[span + 1]);
                for point in ends {
                    assert!(segment_distance(*point, a, b) <= f64::from(display.sags[i]) + offset * 1e-5);
                }
            }
        }
    }

    /// A circle edge is drawn from its curve, at least 64 chords along its 32-step chain, each
    /// chord facing the facets of the chain segment it lies on and rising off them by less than
    /// the chain's sag; a straight edge is drawn at its chain's steps, on its faces.
    #[test]
    fn curved_edges_draw_denser_than_their_face_mesh() {
        use crate::app::walk::encode::{FACING_UNKNOWN, Pen};
        use crate::engine::gpu::segments::SegRows;
        // a 10 mm hole through a thin block: the kernel's share rule leaves its wall 32 steps
        let b = BRep::create_block_with_hole(500.0, 300.0, 60.0, 10.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let signs = vec![1.0; fms.len()];
        let ep = EdgePen::new(
            &fms,
            &signs,
            Pen {
                row: 0,
                radius: 0.0,
                color: 0,
            },
        );
        let mut circles = 0;
        let mut straight = 0;

        for chain in chains.iter().flatten() {
            let edge = &b.m_edges[chain.edge];
            let curve = &b.m_curves_3d[edge.curve_3d_index as usize];
            let display = display_chain(&b, chain, &fms[chain.face]);
            let mut seg = SegRows::default();
            let spans = push_edge_pipes(&mut seg, chain, &display, &ep, &mut AABB::empty());
            assert_eq!(seg.pipes.len(), display.points.len() - 1);
            assert_eq!(seg.pipe_sags.len(), seg.pipes.len());
            assert_eq!(spans.len(), seg.pipes.len());

            if curve.degree() == 1 {
                assert_eq!(display.points.len(), chain.keys.len());
                assert!(display.sags.iter().all(|&sag| sag == 0.0));
                straight += 1;
                continue;
            }

            circles += 1;
            // the mesh keeps its 32 steps; the edge gets its chords
            assert_eq!(chain.keys.len(), 33, "edge {}", chain.edge);
            assert!(
                display.points.len() - 1 >= 64,
                "edge {} has {} chords",
                chain.edge,
                display.points.len() - 1
            );
            let radius = 10.0;

            for (j, w) in display.points.windows(2).enumerate() {
                let span = display.spans[j];
                let a = vertex_position(&fms[chain.face], chain.keys[span]);
                let z = vertex_position(&fms[chain.face], chain.keys[span + 1]);
                // this chord's own sag: the mesh steps are uniform in parameter, not in angle
                let middle = [(a[0] + z[0]) * 0.5, (a[1] + z[1]) * 0.5];
                let sag = radius - (middle[0] * middle[0] + middle[1] * middle[1]).sqrt();

                for p in w {
                    // on the circle, within the chain segment's reach
                    let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
                    assert!((r - radius).abs() < 1e-6, "edge {} point {p:?}", chain.edge);
                    assert!(
                        segment_distance(*p, a, z) <= sag * 1.001,
                        "edge {} segment {j} span {span}: {p:?} is {} off its chord",
                        chain.edge,
                        segment_distance(*p, a, z)
                    );
                }

                assert!(display.sags[j] >= 0.0);
                assert!(f64::from(display.sags[j]) <= sag * SAG_MARGIN * 1.001);
            }

            // every mesh vertex of the chain lies on the drawn line, within a chord's sag
            for &key in &chain.keys {
                let v = vertex_position(&fms[chain.face], key);
                let nearest = display
                    .points
                    .windows(2)
                    .map(|w| segment_distance(v, w[0], w[1]))
                    .fold(f64::INFINITY, f64::min);
                assert!(nearest < 0.05, "edge {} mesh vertex {v:?} is {nearest} off the drawn line", chain.edge);
            }

            assert!(display.sags.iter().any(|&sag| sag > 0.0));
            assert!(seg.pipes.iter().all(|p| p.facing != FACING_UNKNOWN), "edge {} facets", chain.edge);
            assert_eq!(spans.first(), Some(&0));
            assert_eq!(spans.last(), Some(&31));
        }

        assert_eq!(circles, 2);
        assert_eq!(straight, 13, "twelve box edges and the hole wall's seam");
    }

    /// A chain that runs its curve backwards still gets the samples in chain order.
    #[test]
    fn display_chain_follows_the_chain_direction() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);

        for chain in chains.iter().flatten().take(2) {
            let display = display_chain(&b, chain, &fms[chain.face]);
            let fm = &fms[chain.face];
            assert!(distance2(display.points[0], vertex_position(fm, chain.keys[0])).sqrt() < 1e-9);
            assert!(distance2(*display.points.last().unwrap(), vertex_position(fm, *chain.keys.last().unwrap())).sqrt() < 1e-9);
            let second = vertex_position(fm, chain.keys[1]);
            let at = nearest_index(&display.points, second);
            assert!(at > 0 && at < display.points.len() / 2, "the second mesh vertex comes early, not at the wrap");
            assert!(display.spans.windows(2).all(|w| w[1] >= w[0]));
        }
    }

    /// A chain counts for its edge only when it runs between the edge's own ends, either way round.
    #[test]
    fn chains_must_run_between_their_edges_ends() {
        let b = BRep::create_box(400.0, 300.0, 250.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let chain = chains[0].as_ref().unwrap();
        let fm = &fms[chain.face];
        let mut reversed = chain.keys.clone();
        reversed.reverse();
        assert!(chain_on_edge(&b, chain.edge, fm, &chain.keys));
        assert!(chain_on_edge(&b, chain.edge, fm, &reversed));

        // the vertices of another edge of the same face, as a mis-tagged mesh would hand them
        let other = chains
            .iter()
            .flatten()
            .find(|c| c.face == chain.face && c.edge != chain.edge && c.keys.first() != chain.keys.first() && c.keys.last() != chain.keys.last())
            .unwrap();
        assert!(!chain_on_edge(&b, chain.edge, fm, &other.keys));
    }

    /// Pipe ids stay the BRep edge index after remeshing.
    #[test]
    fn remeshing_keeps_source_edge_identity() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let coarse = b.face_meshes_q(Some((20.0, 0.005)));
        let fine = b.face_meshes_q(Some(QUALITY));
        let coarse = edge_chains(&b, &coarse);
        let fine = edge_chains(&b, &fine);

        for (edge, (coarse, fine)) in coarse.iter().zip(&fine).enumerate() {
            assert_eq!(coarse.as_ref().unwrap().edge, edge);
            assert_eq!(fine.as_ref().unwrap().edge, edge);
        }

        assert!(fine[0].as_ref().unwrap().keys.len() > coarse[0].as_ref().unwrap().keys.len());
    }

    /// A hole rim meshed by CDT still gets its edge chains.
    #[test]
    fn cdt_only_hole_boundaries_keep_real_edges() {
        let mut b = BRep::create_block_with_hole(500.0, 300.0, 200.0, 80.0);
        b.m_faces = vec![b.m_faces[5].clone()];
        b.m_shells.clear();
        b.m_solids.clear();
        let fms = b.face_meshes_q(Some(QUALITY));
        assert!(!fms[0].face.is_empty());
        let chains = edge_chains(&b, &fms);
        let mut attached = 0;

        for (edge, chain) in chains.iter().enumerate() {
            if b.edge_faces(edge).is_empty() {
                continue;
            }

            let chain = chain.as_ref().expect("trim constraint keeps its CAD edge");
            assert_eq!(chain.edge, edge);
            assert_eq!(chain.face, 0);
            attached += 1;
        }

        assert_eq!(attached, 5);
    }

    /// A curved trim edge runs on the face's own vertices.
    #[test]
    fn curved_trim_edges_use_exact_face_samples() {
        use session_rust::brep::BRepRef;
        use session_rust::{NurbsCurve, Point, Primitives};
        let surface = Primitives::wave_surface(1.0, 0.5);
        let mut b = BRep::new();
        let surface_index = b.add_surface(&surface);
        let corners = [[0.1, 0.1], [0.9, 0.1], [0.9, 0.9], [0.1, 0.9]];

        for uv in corners {
            b.add_vertex(&surface.point_at(uv[0], uv[1]).unwrap(), 0.0);
        }

        let mut edges = Vec::new();

        for side in 0..4 {
            let a = corners[side];
            let z = corners[(side + 1) % 4];
            let direction = if a[0] != z[0] { 0 } else { 1 };
            let mut curve = surface.iso_curve(direction, a[1 - direction]).unwrap();
            assert!(curve.trim(0.1, 0.9));

            if a[direction] > z[direction] {
                curve.reverse();
            }

            let curve_index = b.add_curve_3d(&curve);
            let edge = b.add_edge(curve_index as i32, side as i32, ((side + 1) % 4) as i32);
            let pcurve = NurbsCurve::create(
                false,
                1,
                &[Point::new(a[0], a[1], 0.0), Point::new(z[0], z[1], 0.0)],
            );
            let pcurve_index = b.add_curve_2d(&pcurve);
            b.add_pcurve(edge, surface_index, pcurve_index as i32, -1);
            edges.push(BRepRef::new(edge as i32, BRepOrientation::Forward));
        }

        let wire = b.add_wire(&edges);
        b.add_face(
            surface_index as i32,
            &[BRepRef::new(wire as i32, BRepOrientation::Forward)],
            0.0,
        );
        let fms = b.face_meshes_q(Some(QUALITY));
        assert!(!fms[0].face.is_empty());
        assert!(b.face_meshes_q_bounded(QUALITY, 1).is_none());
        let bounded = b.face_meshes_q_bounded(QUALITY, 1_000_000).expect("bounded generic curved trim");
        let (before, after) = (fms[0].to_render(), bounded[0].to_render());
        assert_eq!(before.indices, after.indices);
        assert_eq!(bytemuck::cast_slice::<_, u8>(&before.vertices), bytemuck::cast_slice::<_, u8>(&after.vertices));

        let absolute_chord = QUALITY.1 * super::super::display::brep_diagonal(&b);
        let remote = surface.transformed(&session_rust::Xform::translation(100.0, 0.0, 0.0));
        let mut wider = b.clone();
        wider.add_surface(&remote);
        let quality = (QUALITY.0, absolute_chord / super::super::display::brep_diagonal(&wider));
        let wider_meshes = wider.face_meshes_q_bounded(quality, 1_000_000).unwrap();
        let wider_render = wider_meshes[0].to_render();
        assert_eq!(after.indices, wider_render.indices);
        assert_eq!(bytemuck::cast_slice::<_, u8>(&after.vertices), bytemuck::cast_slice::<_, u8>(&wider_render.vertices));

        let chains = edge_chains(&b, &fms);

        for (edge, chain) in chains.iter().enumerate() {
            let chain = chain.as_ref().expect("curved trimmed face edge");
            assert_eq!(chain.edge, edge);
            assert!(chain.keys.len() > 4);

            for &key in &chain.keys {
                let vertex = &fms[0].vertex[&key];
                let u = *vertex.attributes.get("u").unwrap();
                let v = *vertex.attributes.get("v").unwrap();
                let p = surface.point_at(u, v).unwrap();
                assert_eq!([vertex.x, vertex.y, vertex.z], [p[0], p[1], p[2]]);
                let n = vertex.normal().unwrap();
                assert!((n[0] * n[0] + n[1] * n[1] + n[2] * n[2] - 1.0).abs() < 1e-10);
            }
        }
    }
}
