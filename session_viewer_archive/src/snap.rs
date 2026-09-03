//! Object snapping (osnap): gather candidate world points from nearby objects,
//! project them to the screen, and pick the closest within a pixel aperture,
//! ranked by snap-kind priority then pixel distance. Pure: no egui/wgpu.

use std::collections::HashSet;
use session_rust::session::{Geometry, Session};
use session_rust::{Closest, Line, Point, Xform};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnapKind {
    Endpoint,
    Vertex,
    Intersection,
    Midpoint,
    Knot,
    Center,
    Near,
    Grid,
    Plane,
}

impl SnapKind {
    /// Lower wins. Discrete/derived snaps outrank the sliding "near"; the bare
    /// plane projection is the last resort.
    pub fn priority(self) -> u8 {
        match self {
            SnapKind::Endpoint | SnapKind::Vertex => 0,
            SnapKind::Intersection => 1,
            SnapKind::Midpoint => 2,
            SnapKind::Knot => 3,
            SnapKind::Center => 4,
            SnapKind::Near => 5,
            SnapKind::Grid | SnapKind::Plane => 6,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            SnapKind::Endpoint => "End",
            SnapKind::Vertex => "Vertex",
            SnapKind::Intersection => "Int",
            SnapKind::Midpoint => "Mid",
            SnapKind::Knot => "Knot",
            SnapKind::Center => "Center",
            SnapKind::Near => "Near",
            SnapKind::Grid => "Grid",
            SnapKind::Plane => "Plane",
        }
    }
}

#[derive(Clone, Copy)]
pub struct SnapModes(pub u32);

impl SnapModes {
    pub const ENDPOINT: u32 = 1 << 0;
    pub const MIDPOINT: u32 = 1 << 1;
    pub const VERTEX: u32 = 1 << 2;
    pub const NEAR: u32 = 1 << 3;
    pub const INTERSECTION: u32 = 1 << 4;
    pub const KNOT: u32 = 1 << 5;
    pub const CENTER: u32 = 1 << 6;

    pub fn has(self, m: u32) -> bool {
        self.0 & m != 0
    }

    pub fn default_on() -> Self {
        SnapModes(
            Self::ENDPOINT | Self::MIDPOINT | Self::VERTEX | Self::NEAR | Self::INTERSECTION | Self::KNOT,
        )
    }
}

#[derive(Clone)]
pub struct SnapResult {
    pub point: Point,
    pub kind: SnapKind,
}

/// Static snap data, built once per tool session: discrete points (End/Mid/Vertex) plus edge
/// polylines (world-space). Per frame the tool only PROJECTS these — no `point_at` on mouse-move.
/// Edge polylines drive the "Near" (along-edge) snap and the object-move ghost wireframe.
pub struct StaticSnaps {
    pub points: Vec<(Point, SnapKind, String)>,
    pub edges: Vec<(String, Vec<Point>)>,
}

/// Build `StaticSnaps` for every object in the scene.
pub fn build_static_snaps(session: &Session, hidden: &HashSet<String>) -> StaticSnaps {
    const EDGE_SAMPLES: usize = 16;
    let mut points: Vec<(Point, SnapKind, String)> = Vec::new();
    let mut edges: Vec<(String, Vec<Point>)> = Vec::new();
    // ONE downward pass for every placement (BRep geometry is local; the pose is on the
    // session). `world_xform` per object rescans the whole tree and is quadratic.
    let world = session.world_xforms();
    for (guid, geom) in &session.lookup {
        if hidden.contains(guid) { continue; }
        match geom {
            Geometry::Point(p) => points.push(((**p).clone(), SnapKind::Vertex, guid.clone())),
            Geometry::Line(l) => {
                points.push((l.start(), SnapKind::Endpoint, guid.clone()));
                points.push((l.end(), SnapKind::Endpoint, guid.clone()));
                points.push((l.center(), SnapKind::Midpoint, guid.clone()));
                edges.push((guid.clone(), vec![l.start(), l.end()]));
            }
            Geometry::Polyline(pl) => {
                let pts = pl.get_points();
                for (i, v) in pts.iter().enumerate() {
                    let k = if i == 0 || i == pts.len()-1 { SnapKind::Endpoint } else { SnapKind::Vertex };
                    points.push((v.clone(), k, guid.clone()));
                }
                for seg in &pl.get_lines() {
                    points.push((seg.center(), SnapKind::Midpoint, guid.clone()));
                }
                edges.push((guid.clone(), pts));
            }
            Geometry::Mesh(m) => {
                for (i, vd) in m.vertex.values().enumerate() {
                    if i >= MAX_MESH_VERTS { break; }
                    points.push((vd.position(), SnapKind::Vertex, guid.clone()));
                }
            }
            Geometry::BRep(b) => {
                let c = world.get(guid).cloned().unwrap_or_else(Xform::identity).to_cols();
                let tp = |p: &Point| Point::new(
                    c[0][0]*p[0]+c[1][0]*p[1]+c[2][0]*p[2]+c[3][0],
                    c[0][1]*p[0]+c[1][1]*p[1]+c[2][1]*p[2]+c[3][1],
                    c[0][2]*p[0]+c[1][2]*p[1]+c[2][2]*p[2]+c[3][2],
                );
                for v in &b.m_vertices { points.push((tp(v), SnapKind::Vertex, guid.clone())); }
                for crv in &b.m_curves_3d {
                    let (t0, t1) = crv.domain();
                    points.push((tp(&crv.point_at(t0)), SnapKind::Endpoint, guid.clone()));
                    points.push((tp(&crv.point_at(t1)), SnapKind::Endpoint, guid.clone()));
                    points.push((tp(&crv.point_at((t0+t1)*0.5)), SnapKind::Midpoint, guid.clone()));
                    let poly: Vec<Point> = (0..=EDGE_SAMPLES)
                        .map(|k| tp(&crv.point_at(t0 + (t1-t0)*(k as f64/EDGE_SAMPLES as f64)))).collect();
                    edges.push((guid.clone(), poly));
                }
            }
            _ => {}
        }
    }
    for nc in &session.objects.nurbscurves {
        let g = nc.guid().to_string();
        if hidden.contains(&g) { continue; }
        let (t0, t1) = nc.domain();
        points.push((nc.point_at(t0), SnapKind::Endpoint, g.clone()));
        points.push((nc.point_at(t1), SnapKind::Endpoint, g.clone()));
        points.push((nc.point_at((t0+t1)*0.5), SnapKind::Midpoint, g.clone()));
        let poly: Vec<Point> = (0..=EDGE_SAMPLES)
            .map(|k| nc.point_at(t0 + (t1-t0)*(k as f64/EDGE_SAMPLES as f64))).collect();
        edges.push((g, poly));
    }
    StaticSnaps { points, edges }
}

/// Cap mesh-vertex projection per object so a dense mesh cannot stall a mouse-move.
const MAX_MESH_VERTS: usize = 8192;
/// Cap segments fed to the pairwise intersection test (O(n^2)).
const MAX_INT_SEGS: usize = 256;
/// Cap knot points emitted per NURBS object.
const MAX_KNOTS_PER_OBJ: usize = 512;

/// Unique interior knot values in (d0, d1), assuming `knots` is sorted ascending.
/// Ends are skipped (they coincide with Endpoint snaps); multiplicities collapse to one.
fn unique_interior_knots(knots: &[f64], d0: f64, d1: f64) -> Vec<f64> {
    let eps = (d1 - d0).abs() * 1e-7 + 1e-9;
    let mut out: Vec<f64> = Vec::new();
    for &k in knots {
        if k <= d0 + eps || k >= d1 - eps {
            continue;
        }
        if out.last().map_or(true, |&p| (k - p).abs() > eps) {
            out.push(k);
        }
    }
    out
}

/// Resolve a snap. `near_world` is the ray-plane fallback and the test point for
/// continuous "near" snaps; returned as a `Plane` snap if nothing else qualifies.
/// `world_tol` is the 3D distance (mm) within which two segments count as intersecting.
pub fn snap(
    session: &Session,
    candidate_guids: &[String],
    hidden: &HashSet<String>,
    cursor_px: (f32, f32),
    aperture_px: f32,
    world_tol: f64,
    near_world: Point,
    modes: SnapModes,
    project: &dyn Fn(&Point) -> Option<(f32, f32)>,
) -> SnapResult {
    let cand: HashSet<&str> = candidate_guids.iter().map(|s| s.as_str()).collect();
    let mut world: Option<std::collections::HashMap<String, session_rust::Xform>> = None;
    let mut cands: Vec<(Point, SnapKind)> = Vec::new();
    let mut segs: Vec<Line> = Vec::new(); // for pairwise Intersection

    for guid in &cand {
        if hidden.contains(*guid) {
            continue;
        }
        let Some(geom) = session.lookup.get(*guid) else { continue };
        match geom {
            Geometry::Point(p) => {
                if modes.has(SnapModes::VERTEX) || modes.has(SnapModes::ENDPOINT) {
                    cands.push(((**p).clone(), SnapKind::Vertex));
                }
            }
            Geometry::Line(l) => {
                if modes.has(SnapModes::ENDPOINT) {
                    cands.push((l.start(), SnapKind::Endpoint));
                    cands.push((l.end(), SnapKind::Endpoint));
                }
                if modes.has(SnapModes::MIDPOINT) {
                    cands.push((l.center(), SnapKind::Midpoint));
                }
                if modes.has(SnapModes::NEAR) {
                    let (cp, _, _) = Closest::line_point(l, &near_world);
                    cands.push((cp, SnapKind::Near));
                }
                if modes.has(SnapModes::INTERSECTION) && segs.len() < MAX_INT_SEGS {
                    segs.push((**l).clone());
                }
            }
            Geometry::Polyline(pl) => {
                let pts = pl.get_points();
                if !pts.is_empty() {
                    if modes.has(SnapModes::ENDPOINT) {
                        cands.push((pts[0].clone(), SnapKind::Endpoint));
                        cands.push((pts[pts.len() - 1].clone(), SnapKind::Endpoint));
                    }
                    if modes.has(SnapModes::VERTEX) && pts.len() > 2 {
                        for v in &pts[1..pts.len() - 1] {
                            cands.push((v.clone(), SnapKind::Vertex));
                        }
                    }
                }
                let lines = pl.get_lines();
                if modes.has(SnapModes::MIDPOINT) {
                    for seg in &lines {
                        cands.push((seg.center(), SnapKind::Midpoint));
                    }
                }
                if modes.has(SnapModes::INTERSECTION) {
                    for seg in lines.iter() {
                        if segs.len() >= MAX_INT_SEGS {
                            break;
                        }
                        segs.push(seg.clone());
                    }
                }
                if modes.has(SnapModes::NEAR) {
                    let (cp, _, _) = Closest::polyline_point(pl, &near_world);
                    cands.push((cp, SnapKind::Near));
                }
            }
            Geometry::Mesh(m) => {
                if modes.has(SnapModes::VERTEX) {
                    for (i, vd) in m.vertex.values().enumerate() {
                        if i >= MAX_MESH_VERTS {
                            break;
                        }
                        cands.push((vd.position(), SnapKind::Vertex));
                    }
                }
            }
            Geometry::BRep(b) => {
                // BRep geometry is local; the visible pose is the session placement. Transform
                // every snap point through it so markers land on the rendered edges/corners.
                // The placement map is built lazily and reused: one tree pass, only if a BRep
                // is actually among the (few) candidates.
                let world = world.get_or_insert_with(|| session.world_xforms());
                let c = world.get(*guid).cloned().unwrap_or_else(Xform::identity).to_cols();
                let tp = |p: &Point| Point::new(
                    c[0][0]*p[0] + c[1][0]*p[1] + c[2][0]*p[2] + c[3][0],
                    c[0][1]*p[0] + c[1][1]*p[1] + c[2][1]*p[2] + c[3][1],
                    c[0][2]*p[0] + c[1][2]*p[1] + c[2][2]*p[2] + c[3][2],
                );
                if modes.has(SnapModes::VERTEX) || modes.has(SnapModes::ENDPOINT) {
                    for v in &b.m_vertices {
                        cands.push((tp(v), SnapKind::Vertex));
                    }
                }
                for crv in &b.m_curves_3d {
                    let (t0, t1) = crv.domain();
                    if modes.has(SnapModes::ENDPOINT) {
                        cands.push((tp(&crv.point_at(t0)), SnapKind::Endpoint));
                        cands.push((tp(&crv.point_at(t1)), SnapKind::Endpoint));
                    }
                    if modes.has(SnapModes::MIDPOINT) {
                        cands.push((tp(&crv.point_at((t0 + t1) * 0.5)), SnapKind::Midpoint));
                    }
                    if modes.has(SnapModes::NEAR) {
                        // Closest of a coarse sampling of this edge to the cursor ray-plane point.
                        let mut best = (f64::MAX, near_world.clone());
                        for k in 0..=24 {
                            let t = t0 + (t1 - t0) * (k as f64 / 24.0);
                            let w = tp(&crv.point_at(t));
                            let d = (w[0]-near_world[0]).powi(2)+(w[1]-near_world[1]).powi(2)+(w[2]-near_world[2]).powi(2);
                            if d < best.0 { best = (d, w); }
                        }
                        cands.push((best.1, SnapKind::Near));
                    }
                }
            }
            _ => {}
        }
    }

    // Pairwise segment intersections (segment-bounded, within world_tol).
    if modes.has(SnapModes::INTERSECTION) && segs.len() >= 2 {
        let n = segs.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if let Some(p) = session_rust::intersection::line_line(&segs[i], &segs[j], world_tol) {
                    cands.push((p, SnapKind::Intersection));
                }
            }
        }
    }

    // NURBS curves / surfaces live in session.objects, not lookup. Gate by candidates.
    for nc in &session.objects.nurbscurves {
        let g = nc.guid().to_string();
        if !cand.contains(g.as_str()) || hidden.contains(&g) {
            continue;
        }
        if modes.has(SnapModes::ENDPOINT) {
            cands.push((nc.point_at_start(), SnapKind::Endpoint));
            cands.push((nc.point_at_end(), SnapKind::Endpoint));
        }
        if modes.has(SnapModes::KNOT) {
            let (d0, d1) = nc.domain();
            for t in unique_interior_knots(&nc.get_nurbsknots(), d0, d1) {
                cands.push((nc.point_at(t), SnapKind::Knot));
            }
        }
        if modes.has(SnapModes::NEAR) {
            let (t, _) = Closest::curve_point(nc, &near_world, 0.0, 0.0);
            cands.push((nc.point_at(t), SnapKind::Near));
        }
    }
    for ns in &session.objects.nurbssurfaces {
        let g = ns.guid().to_string();
        if !cand.contains(g.as_str()) || hidden.contains(&g) {
            continue;
        }
        if modes.has(SnapModes::KNOT) {
            if let (Some((u0, u1)), Some((v0, v1))) = (ns.domain(0), ns.domain(1)) {
                let us = unique_interior_knots(&ns.get_nurbsknots(0), u0, u1);
                let vs = unique_interior_knots(&ns.get_nurbsknots(1), v0, v1);
                let mut emitted = 0usize;
                'grid: for &u in &us {
                    for &v in &vs {
                        if emitted >= MAX_KNOTS_PER_OBJ {
                            break 'grid;
                        }
                        if let Some(p) = ns.point_at(u, v) {
                            cands.push((p, SnapKind::Knot));
                            emitted += 1;
                        }
                    }
                }
            }
        }
        if modes.has(SnapModes::NEAR) {
            let (u, v, _) = Closest::surface_point(ns, &near_world, 0.0, 0.0, 0.0, 0.0);
            if let Some(p) = ns.point_at(u, v) {
                cands.push((p, SnapKind::Near));
            }
        }
    }

    // Project each candidate to the screen and keep the best within the aperture.
    let mut best: Option<(u8, f32, Point, SnapKind)> = None;
    for (p, kind) in cands {
        let Some((sx, sy)) = project(&p) else { continue };
        let dx = sx - cursor_px.0;
        let dy = sy - cursor_px.1;
        let d = (dx * dx + dy * dy).sqrt();
        if d > aperture_px {
            continue;
        }
        let prio = kind.priority();
        let take = match &best {
            None => true,
            Some((bp, bd, ..)) => prio < *bp || (prio == *bp && d < *bd),
        };
        if take {
            best = Some((prio, d, p, kind));
        }
    }

    match best {
        Some((_, _, p, kind)) => SnapResult { point: p, kind },
        None => SnapResult { point: near_world, kind: SnapKind::Plane },
    }
}
