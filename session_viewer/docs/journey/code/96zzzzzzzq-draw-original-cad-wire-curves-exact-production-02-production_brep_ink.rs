use session_rust::{Mesh,BRep,AABB};
use crate::cad_edges::EdgeChain;
use crate::production_curve_walk::{Pen,CylinderSegment,turning_degrees};
use crate::production_mesh_ink::{SegRows,SILHOUETTE_SAG,COPLANAR_DOT,pack_facing};
const EDGE_DEGREES:f64=4.0;
const EDGE_CHORDS_MAX:usize=256;
const SAG_MARGIN:f64=2.0;
mod knobs {pub fn coarse_edges()->bool{false}}
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

    /// Every inner edge of a curved face, between two of its triangles that are not coplanar, as a
    /// pipe drawn only where it is the face's silhouette: one triangle beside it faces the camera
    /// and the other faces away. Planar faces and the face boundaries add none.
    pub fn push_silhouette_pipes(&self, seg: &mut SegRows, bounds: &mut AABB) {
        seg.pipe_ids.resize(seg.pipes.len(), u32::MAX);
        seg.pipe_sags.resize(seg.pipes.len(), 0.0);

        for (face, facets) in self.facets.iter().enumerate() {
            let mut edges: Vec<_> = facets.iter().collect();
            edges.sort_unstable_by_key(|(edge, _)| **edge);

            for (edge, pair) in edges {
                let (Some(n0), Some(n1)) = (pair.normals[0], pair.normals[1]) else {
                    continue;
                };

                if pair.count != 2 || n0[0] * n1[0] + n0[1] * n1[1] + n0[2] * n1[2] >= COPLANAR_DOT {
                    continue;
                }

                let p0 = crate::production_curve_walk::render_position(edge[0].map(f64::from_bits));
                let p1 = crate::production_curve_walk::render_position(edge[1].map(f64::from_bits));

                if p0 == p1 || !p0.into_iter().chain(p1).all(f32::is_finite) {
                    continue;
                }

                bounds.union_with_point(p0[0] as f64, p0[1] as f64, p0[2] as f64);
                bounds.union_with_point(p1[0] as f64, p1[1] as f64, p1[2] as f64);
                let pair = (scaled_normal(Some(n0), self.signs[face]), scaled_normal(Some(n1), self.signs[face]));
                seg.pipes.push(CylinderSegment {
                    p0,
                    radius: self.pen.radius,
                    p1,
                    instance_id: self.pen.row,
                    color: self.pen.color,
                    facing: self.facing_word(pair),
                });
                seg.pipe_ids.push(u32::MAX);
                seg.pipe_sags.push(SILHOUETTE_SAG);
            }
        }
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
        let p0f = crate::production_curve_walk::render_position(w[0]);
        let p1f = crate::production_curve_walk::render_position(w[1]);

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

