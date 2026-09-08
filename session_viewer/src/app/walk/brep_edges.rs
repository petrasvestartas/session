//! A BRep's edges as ink, taken from the tessellation itself. The kernel's grid mesher puts
//! every boundary of a grid-meshed face on an iso-parametric line and tags each vertex with
//! the exact `u`/`v` it was sampled at, so the chain of vertices along an edge IS the facet
//! boundary - no resampling, no tolerance. One chain per BRep edge, from the first face that
//! can supply one; the other adjacent face lends the facing cull its normal.

use session_rust::brep::{BRep, BRepOrientation};
use session_rust::Mesh;

/// One use of an edge by a face: which edge, which face, and the orientation of that use
/// (`BRep::edge_faces` composes it), which selects the pcurve on a seam.
pub struct EdgeUse {
    pub edge: usize,
    pub face: usize,
    pub orientation: BRepOrientation,
}

/// A pcurve's two ends in UV, from its first and last control point. The primitives' pcurves
/// are straight iso lines, which is also the condition under which the kernel grid-meshes a
/// face; a curved pcurve never reaches here because its face has no `u`/`v` attributes.
fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);
    if ci < 0 {
        return None;
    }
    let c = &b.m_curves_2d[ci as usize];
    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

/// The distinct values of attribute `name` over the face mesh, sorted: the mesher's own
/// sample array, recovered exactly (every vertex carries one of its entries).
fn sample_values(fm: &Mesh, name: &str) -> Vec<f64> {
    let mut vals: Vec<f64> = fm.vertex.values().filter_map(|vd| vd.attributes.get(name).copied()).collect();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals.dedup();
    vals
}

/// Which sample value the pcurve's constant parameter `target` names. A closed direction has
/// no sample at its domain end (the mesher welds the wrap), so a pcurve sitting at the end -
/// the reversed use of a seam - is measured against the start as well; the nearest wins,
/// which needs no tolerance.
fn nearest_sample(vals: &[f64], target: f64, wrap: Option<(f64, f64)>) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;
    for &v in vals {
        let mut d = (v - target).abs();
        if let Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }
        if best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, v));
        }
    }
    best.map(|(_, v)| v)
}

/// The face-mesh vertex keys along edge use `eu`, ordered along the parameter that varies,
/// closed (first key repeated last) when the edge starts and ends at the same vertex.
pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let (p0, p1) = pcurve_ends(b, eu)?;
    // The constant parameter is the one that moves least between the pcurve's ends: u for a
    // meridian, v for a circle of latitude.
    let fixed = if (p1[0] - p0[0]).abs() <= (p1[1] - p0[1]).abs() { 0 } else { 1 };
    let (fixed_name, free_name) = if fixed == 0 { ("u", "v") } else { ("v", "u") };
    let vals = sample_values(fm, fixed_name);
    if vals.is_empty() {
        return None;
    }
    let srf = &b.m_surfaces[b.m_faces[eu.face].surface_index as usize];
    let wrap = if srf.is_closed(fixed) { srf.domain(fixed) } else { None };
    let at = nearest_sample(&vals, p0[fixed], wrap)?;

    let mut on_line: Vec<(f64, usize)> = Vec::new();
    for (&key, vd) in fm.vertex.iter() {
        let (Some(&f), Some(&t)) = (vd.attributes.get(fixed_name), vd.attributes.get(free_name)) else { continue };
        if f == at {
            on_line.push((t, key));
        }
    }
    if on_line.len() < 2 {
        return None;
    }
    // By parameter, then by key: the map's order must never reach the chain.
    on_line.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
    let mut keys: Vec<usize> = on_line.into_iter().map(|(_, k)| k).collect();
    let e = &b.m_edges[eu.edge];
    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }
    Some(keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::brep::QUALITY;

    /// The first use of every edge of `b`, with its face mesh index.
    fn first_uses(b: &BRep) -> Vec<EdgeUse> {
        let mut out = Vec::new();
        for ei in 0..b.m_edges.len() {
            let uses = b.edge_faces(ei);
            let Some(u) = uses.first() else { continue };
            out.push(EdgeUse { edge: ei, face: u.index as usize, orientation: u.orientation });
        }
        out
    }

    /// Chain `keys` of face mesh `fm` starts (and, when `closed`, ends) on BRep vertex `vi`
    /// bit for bit: the mesher sampled the corner, not a point near it.
    fn ends_on(b: &BRep, fm: &Mesh, keys: &[usize], vi: usize) -> bool {
        let p = fm.vertex[&keys[0]].position();
        let v = &b.m_vertices[vi].point;
        p[0] == v[0] && p[1] == v[1] && p[2] == v[2]
    }

    /// A cylinder's three edges: two closed circles of the same length that start on their own
    /// vertex, and an open seam of two keys (the side grid has two rows) from vertex 0 to 1.
    #[test]
    fn cylinder_chains_follow_the_grid() {
        let b = BRep::create_cylinder(150.0, 400.0);
        let fms = b.face_meshes_q(Some(QUALITY));
        let uses = first_uses(&b);
        let chains: Vec<Vec<usize>> = uses.iter().map(|u| iso_chain(&b, &fms[u.face], u).expect("grid use")).collect();
        assert_eq!(chains.len(), 3);
        for k in 0..2 {
            assert_eq!(chains[k].first(), chains[k].last());
            assert!(chains[k].len() > 4);
            assert!(ends_on(&b, &fms[uses[k].face], &chains[k], b.m_edges[k].start_vertex as usize));
        }
        assert_eq!(chains[0].len(), chains[1].len());
        assert_eq!(chains[2].len(), 2);
        assert!(ends_on(&b, &fms[uses[2].face], &chains[2], 0));
        let top = fms[uses[2].face].vertex[chains[2].last().unwrap()].position();
        assert_eq!([top[0], top[1], top[2]], [b.m_vertices[1].point[0], b.m_vertices[1].point[1], b.m_vertices[1].point[2]]);
    }

    /// A sphere's seam runs pole to pole through the welded grid: one open chain whose ends
    /// are the two BRep vertices; its two degenerated edges yield nothing.
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

    /// A torus has two closed seams on one welded face, one along each parameter, both
    /// through the single BRep vertex; a CDT face (the cylinder's cap) has no iso chain.
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
        let eu = EdgeUse { edge: 0, face: cap.index as usize, orientation: cap.orientation };
        assert!(iso_chain(&cyl, &cfm[eu.face], &eu).is_none());
    }
}
