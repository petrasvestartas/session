use session_rust::Mesh;
use session_rust::brep::{BRep, BRepOrientation};

use super::encode::{Pen, pack_facing};
use crate::engine::gpu::CylinderSegment;
use crate::engine::gpu::segments::SegRows;
use session_rust::AABB;

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
    pub edge: usize,
    pub face: usize,
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

    vals.sort_by(sample_order);
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

    on_line.sort_by(parameter_order);
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
    pub edge: usize, // BRep edge index
    pub face: usize, // face mesh the keys belong to
    pub keys: Vec<usize>, // vertex keys along the edge
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

        ordered.sort_by(total_parameter_order);
        ordered.dedup_by(same_parameter);
        let mut keys = Vec::with_capacity(ordered.len());

        for (_, key) in ordered {
            keys.push(key);
        }

        return Some(keys);
    }

    None
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

/// A triangle edge keyed by its two end positions.
type FacetEdge = [[u64; 3]; 2];

/// The triangles meeting at one edge.
#[derive(Default)]
struct FacetPair {
    normals: [Option<[f64; 3]>; 2], // first two triangle normals
    count: usize, // how many triangles in total
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

/// What every edge pipe of one BRep needs.
pub struct EdgePen<'a> {
    pub fms: &'a [Mesh],
    pub signs: &'a [f64], // +1 or -1 per face
    pub pen: Pen, // row, width, colour
    facets: Vec<std::collections::HashMap<FacetEdge, FacetPair>>,
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
        }
    }

    /// Facing word of one pipe; unknown when ambiguous.
    fn facing(&self, chain: &EdgeChain, a: [f64; 3], b: [f64; 3]) -> u32 {
        let key = facet_edge(a, b);
        let Some(owner) = self.facets[chain.face].get(&key) else {
            return pack_facing(None, None);
        };

        if owner.count == 0 || owner.count > 2 {
            return pack_facing(None, None);
        }

        let first = scaled_normal(owner.normals[0], self.signs[chain.face]);
        let second = if let Some(other) = chain.other {
            let Some(pair) = self.facets[other].get(&key) else {
                return pack_facing(None, None);
            };

            if pair.count != 1 || owner.count != 1 {
                return pack_facing(None, None);
            }

            scaled_normal(pair.normals[0], self.signs[other])
        } else if owner.count == 2 {
            scaled_normal(owner.normals[1], self.signs[chain.face])
        } else {
            first
        };
        pack_facing(first.as_ref(), second.as_ref())
    }
}

/// A normal turned outward by its face's sign.
fn scaled_normal(normal: Option<[f64; 3]>, sign: f64) -> Option<[f64; 3]> {
    let n = normal?;
    Some([n[0] * sign, n[1] * sign, n[2] * sign])
}

/// One pipe per chain segment; returns how many were pushed.
pub fn push_edge_pipes(
    seg: &mut SegRows,
    chain: &EdgeChain,
    ep: &EdgePen,
    bounds: &mut AABB,
) -> usize {
    let fm = &ep.fms[chain.face];

    seg.pipes.reserve(chain.keys.len().saturating_sub(1));
    let mut count = 0;

    for w in chain.keys.windows(2) {
        let (a, b) = (&fm.vertex[&w[0]], &fm.vertex[&w[1]]);
        let p0 = [a.x, a.y, a.z];
        let p1 = [b.x, b.y, b.z];
        let p0f = super::curves::render_position(p0);
        let p1f = super::curves::render_position(p1);

        // skip a segment that collapses in f32
        if p0f == p1f || !p0f.into_iter().chain(p1f).all(f32::is_finite) {
            continue;
        }

        bounds.union_with_point(p0f[0] as f64, p0f[1] as f64, p0f[2] as f64);
        bounds.union_with_point(p1f[0] as f64, p1f[1] as f64, p1f[2] as f64);
        seg.pipes.push(CylinderSegment {
            p0: p0f,
            radius: ep.pen.radius,
            p1: p1f,
            instance_id: ep.pen.row,
            color: ep.pen.color,
            facing: ep.facing(chain, p0, p1),
        });
        seg.pipe_ids
            .push(u32::try_from(chain.edge).unwrap_or(u32::MAX)); // edge id for picking
        count += 1;
    }

    count
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
        ));
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
        let circle = push_edge_pipes(&mut seg, chains[0].as_ref().unwrap(), &ep, &mut bounds);
        assert_eq!(circle, chains[0].as_ref().unwrap().keys.len() - 1);

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
        let seam = push_edge_pipes(&mut seg, chains[2].as_ref().unwrap(), &ep, &mut bounds);
        assert_eq!(seam, 1);
        let p = &seg.pipes[before];
        assert_ne!(p.facing & 0xffff, p.facing >> 16);
        assert!(seg.ribbons.is_empty());
        assert_eq!(seg.pipe_ids.len(), seg.pipes.len());
        assert!(seg.pipe_ids[..before].iter().all(|&id| id == 0));
        assert_eq!(seg.pipe_ids[before], 2);
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
        push_edge_pipes(
            &mut segments,
            chains[1].as_ref().unwrap(),
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
        let b = BRep::create_box(40.0, 30.0, 25.0);
        let mut fms = b.face_meshes_q(Some(QUALITY));
        let chains = edge_chains(&b, &fms);
        let chain = chains[0].as_ref().unwrap();

        for key in &chain.keys {
            let vertex = fms[chain.face].vertex.get_mut(key).unwrap();
            vertex.x = 1e20;
            vertex.y = 1e20;
            vertex.z = 1e20;
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
        assert_eq!(push_edge_pipes(&mut seg, chain, &ep, &mut AABB::empty()), 0);
        assert!(seg.pipes.is_empty());
        assert!(seg.pipe_ids.is_empty());
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
