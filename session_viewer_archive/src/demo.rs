#![allow(dead_code)]
use session_rust::{
    BRep, Color, Mesh, NurbsCurve, NurbsSurface, NurbsSurfaceTrimmed, Point, Session, TreeNode, Vector, Xform,
};
use session_rust::primitives::Primitives;
use session_rust::session::Geometry;
use std::rc::Rc;
use std::cell::RefCell;

fn merge_tree_node(
    node: &Rc<RefCell<TreeNode>>,
    parent: &Rc<RefCell<TreeNode>>,
    root: &Rc<RefCell<TreeNode>>,
    promote: &[&str],
    dst: &mut Session,
    src: &Session,
) {
    let name = node.borrow().name.clone();
    let children = node.borrow().children();
    if let Some(geom) = src.lookup.get(&name) {
        match geom {
            Geometry::Mesh(m) => {
                let mut m = (**m).clone();
                m.crease_angle_deg = 15.0;
                dst.add_mesh(m, Some(parent));
            }
            Geometry::Polyline(p) => { dst.add_polyline((**p).clone(), Some(parent)); }
            _ => {
                dst.lookup.insert(name.clone(), geom.clone());
                dst.tree.add(&TreeNode::new(&name), Some(parent));
            }
        }
    } else {
        let target = if promote.contains(&name.as_str()) { root } else { parent };
        let group = TreeNode::new(&name);
        dst.tree.add(&group, Some(target));
        for child in &children {
            merge_tree_node(child, &group, root, promote, dst, src);
        }
    }
}

use crate::gpu_session::{CylinderSegment, GpuSession};
use crate::gpu_instance_groups::TemplateKey;
use crate::gpu_adapters::mesh_to_vertices;

const COL: f32 = 1_500.0;

// Add an untrimmed NurbsSurface (already positioned in world) at the tree root.
fn add_ns(s: &mut Session, mut ns: NurbsSurface, name: &str, face: Color) {
    ns.name = name.to_string();
    ns.set_guid(name.to_string());
    ns.facecolors.push(face);
    ns.linecolors.push(Color::new(0.0, 0.0, 0.0, 1.0));
    s.objects.nurbssurfaces.push(std::rc::Rc::new(ns));
    s.add(&TreeNode::new(name), None);
}

// A trimmed NurbsSurface becomes a SINGLE mesh object: mesh_q triangulates the trim region
// at the SAME fine quality as untrimmed NurbsSurfaces/BReps (mesh() defaults to a coarse 20°,
// which looked faceted next to them). Its naked edges (outer loop + hole rims) are the outline;
// a high crease angle keeps only those, so surface + boundary are one object that moves together.
fn add_trimmed(s: &mut Session, root: &Rc<RefCell<TreeNode>>, ts: &NurbsSurfaceTrimmed, name: &str, color: Color) {
    // Use the SAME tessellation quality as untrimmed NurbsSurfaces (the shared TESS_* constants),
    // not a hardcoded dense pair. 2.5°/0.0006 produced ~16-23× more triangles, which made the
    // gumball drag and every add/commit slow. The deflection-refined CDT is adaptive: flat spans
    // stay ~2 triangles, only curvature subdivides.
    let angle = crate::engine::gpu::geometry::TESS_MAX_ANGLE_DEG as f64;
    let chord = crate::engine::gpu::geometry::TESS_CHORD_FACTOR as f64;
    let mut mesh: Mesh = ts.mesh_q(angle, chord);
    mesh.name = name.to_string();
    mesh.set_guid(name.to_string());
    mesh.set_objectcolor(color);
    mesh.crease_angle_deg = 170.0;
    s.add_mesh(mesh, Some(root));
}

// A surface trimmed by a PLANE (q0, n): keeps the half where (S-q0).n <= 0. Uses the proper
// OCCT path-A mesher (mesh_by_plane): signed-distance contour, every cut-boundary point
// Newton-refined onto the plane, periodic seams welded watertight. Replaces the old column-scan
// loop (which put ~4 boundary points off the plane and produced open seams).
fn add_plane_trim(s: &mut Session, _root: &Rc<RefCell<TreeNode>>, srf: &NurbsSurface, q0: [f32;3], n: [f32;3], name: &str, color: Color) {
    // First-class NurbsSurfaceTrimmed (NOT baked to a Mesh): stored with its cut plane so the
    // viewer tessellates it, draws the trim boundary as selectable curves, and moves it
    // matrix-only like a BRep. Mirrors add_ns (push to objects vec + tree node).
    let mut ts = NurbsSurfaceTrimmed::new();
    ts.m_surface = srf.clone();
    ts.cut_q0 = Some(Point::new(q0[0] as f64, q0[1] as f64, q0[2] as f64));
    ts.cut_n = Some(Vector::new(n[0] as f64, n[1] as f64, n[2] as f64));
    ts.name = name.to_string();
    ts.set_guid(name.to_string());
    ts.surfacecolor = color;
    s.objects.nurbssurfacetrimmeds.push(std::rc::Rc::new(ts));
    s.add(&TreeNode::new(name), None);
}

// SPLIT a surface into all the parts carved by `planes` (every sign-combination region). Each
// part is a first-class multi-plane NurbsSurfaceTrimmed (selectable / hideable / transformable).
// `explode` rigidly pushes each part outward from the surface center so the split reads clearly.
fn add_split(s: &mut Session, _root: &Rc<RefCell<TreeNode>>, srf: &NurbsSurface, planes: &[([f32;3],[f32;3])], base: &str, explode: f32) {
    let kp: Vec<(Point, Vector)> = planes.iter().map(|(q,n)|
        (Point::new(q[0] as f64, q[1] as f64, q[2] as f64), Vector::new(n[0] as f64, n[1] as f64, n[2] as f64))).collect();
    let palette = [
        Color::new(0.30, 0.60, 1.00, 1.0),
        Color::new(0.35, 0.85, 0.45, 1.0),
        Color::new(0.85, 0.35, 0.70, 1.0),
        Color::new(0.95, 0.65, 0.20, 1.0),
        Color::new(0.55, 0.45, 0.95, 1.0),
        Color::new(0.20, 0.80, 0.80, 1.0),
        Color::new(0.90, 0.30, 0.30, 1.0),
        Color::new(0.70, 0.70, 0.25, 1.0),
    ];
    // surface center = midpoint of the parametric domain (explode direction reference)
    let (u0, u1) = srf.domain(0).unwrap_or((0.0, 1.0));
    let (v0, v1) = srf.domain(1).unwrap_or((0.0, 1.0));
    let cp = srf.point_at((u0+u1)*0.5, (v0+v1)*0.5).unwrap_or(Point::new(0.0,0.0,0.0));
    let center = [cp[0] as f64, cp[1] as f64, cp[2] as f64];
    let parts = NurbsSurfaceTrimmed::split_by_planes(srf, &kp);
    for (idx, mut ts) in parts.into_iter().enumerate() {
        if explode > 0.0 {
            let m = ts.mesh_render(20.0, 0.01);
            let mut pc = [0.0f64; 3]; let mut cnt = 0.0f64;
            for vk in m.vertex.keys() { let p = m.vertex_point(*vk).unwrap(); for t in 0..3 { pc[t]+=p[t]; } cnt+=1.0; }
            if cnt > 0.0 {
                let d = [pc[0]/cnt-center[0], pc[1]/cnt-center[1], pc[2]/cnt-center[2]];
                let l = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt().max(1e-9);
                let e = explode as f64 / l;
                ts.transform(&Xform::translation(d[0]*e, d[1]*e, d[2]*e));
            }
        }
        let name = format!("{base}_part{idx}");
        ts.name = name.clone();
        ts.set_guid(name.clone());
        ts.surfacecolor = palette[idx % palette.len()].clone();
        s.objects.nurbssurfacetrimmeds.push(std::rc::Rc::new(ts));
        s.add(&TreeNode::new(&name), None);
    }
}

// A "split"/"cut" of a surface as a STRUCTURED grid sampled directly on the surface, with
// per-vertex analytic normals (smooth shading). This is just an open portion of the
// surface — far cleaner than a CDT trim for a sub-rectangle of a curved/periodic surface.
// `vs_of(i)` gives the v-params for column i (lets the cut height vary, e.g. a plane cut);
// wrap_u/wrap_v close the grid where the surface itself is closed (full tube / full ring).
fn grid_patch_mesh(
    srf: &NurbsSurface, us: &[f32], vs_of: impl Fn(usize) -> Vec<f32>, wrap_u: bool, wrap_v: bool,
) -> Mesh {
    let mut mesh = Mesh::new();
    let nu = us.len();
    let mut grid: Vec<Vec<usize>> = Vec::with_capacity(nu);
    let mut nrm: Vec<Vec<[f32; 3]>> = Vec::with_capacity(nu);
    for i in 0..nu {
        let vs = vs_of(i);
        let mut row = Vec::with_capacity(vs.len());
        let mut nrow = Vec::with_capacity(vs.len());
        for &v in &vs {
            let p = srf.point_at(us[i] as f64, v as f64).unwrap_or(Point::new(0.0, 0.0, 0.0));
            let vk = mesh.add_vertex(p, None);
            let n = srf.normal_at(us[i] as f64, v as f64);
            row.push(vk);
            nrow.push([n[0] as f32, n[1] as f32, n[2] as f32]);
        }
        grid.push(row);
        nrm.push(nrow);
    }
    // Orient the analytic normals to agree with the grid winding (one global check; the
    // surface orientation is consistent across the patch).
    let mut flip = false;
    if nu >= 2 && grid[0].len() >= 2 {
        let p00 = mesh.vertex[&grid[0][0]].position();
        let p10 = mesh.vertex[&grid[1][0]].position();
        let p01 = mesh.vertex[&grid[0][1]].position();
        let e1 = [p10[0]-p00[0], p10[1]-p00[1], p10[2]-p00[2]];
        let e2 = [p01[0]-p00[0], p01[1]-p00[1], p01[2]-p00[2]];
        let wn = [e1[1]*e2[2]-e1[2]*e2[1], e1[2]*e2[0]-e1[0]*e2[2], e1[0]*e2[1]-e1[1]*e2[0]];
        let an = nrm[0][0];
        if (wn[0]*an[0] as f64 + wn[1]*an[1] as f64 + wn[2]*an[2] as f64) < 0.0 { flip = true; }
    }
    for i in 0..nu {
        for (jj, &vk) in grid[i].iter().enumerate() {
            let n = nrm[i][jj];
            let n = if flip { [-n[0], -n[1], -n[2]] } else { n };
            if let Some(vd) = mesh.vertex.get_mut(&vk) { vd.set_normal(n[0] as f64, n[1] as f64, n[2] as f64); }
        }
    }
    let imax = if wrap_u { nu } else { nu.saturating_sub(1) };
    for i in 0..imax {
        let i1 = if wrap_u { (i + 1) % nu } else { i + 1 };
        let nv = grid[i].len().min(grid[i1].len());
        if nv < 2 { continue; }
        let jmax = if wrap_v { nv } else { nv - 1 };
        for j in 0..jmax {
            let j1 = if wrap_v { (j + 1) % nv } else { j + 1 };
            let (a, b, c, d) = (grid[i][j], grid[i1][j], grid[i1][j1], grid[i][j1]);
            if a == b || b == c || c == d || d == a { continue; }
            mesh.add_face(vec![a, b, c], None);
            mesh.add_face(vec![a, c, d], None);
        }
    }
    mesh
}

// Add a structured-grid surface patch (see grid_patch_mesh) to the session.
fn add_grid_patch(
    s: &mut Session, root: &Rc<RefCell<TreeNode>>, srf: &NurbsSurface,
    us: &[f32], vs_of: impl Fn(usize) -> Vec<f32>, wrap_u: bool, wrap_v: bool,
    name: &str, color: Color,
) {
    let mut mesh = grid_patch_mesh(srf, us, vs_of, wrap_u, wrap_v);
    mesh.name = name.to_string();
    mesh.set_guid(name.to_string());
    mesh.set_objectcolor(color);
    mesh.crease_angle_deg = 170.0;
    s.add_mesh(mesh, Some(root));
}

// Cut a surface mesh by an arbitrary plane (point q0, normal n): keep the f<=0 half-space,
// splitting triangles exactly on the plane for a clean cut edge, with interpolated normals.
// Works for ANY plane orientation — including ones that slice a torus into a complex shape.
fn clip_mesh_by_field(src: &Mesh, f: impl Fn([f32; 3]) -> f32) -> Mesh {
    let mut out = Mesh::new();
    let lerp = |a: [f32; 3], b: [f32; 3], t: f32| [a[0]+(b[0]-a[0])*t, a[1]+(b[1]-a[1])*t, a[2]+(b[2]-a[2])*t];
    let nrm_of = |m: &Mesh, v: usize| -> [f32; 3] {
        let vd = &m.vertex[&v];
        [vd.attributes.get("nx").copied().unwrap_or(0.0) as f32,
         vd.attributes.get("ny").copied().unwrap_or(0.0) as f32,
         vd.attributes.get("nz").copied().unwrap_or(0.0) as f32]
    };
    for fverts in src.face.values() {
        if fverts.len() < 3 { continue; }
        for t in 1..fverts.len() - 1 {
            let tri = [fverts[0], fverts[t], fverts[t + 1]];
            let mut p = [[0.0f32; 3]; 3];
            let mut nr = [[0.0f32; 3]; 3];
            let mut d = [0.0f32; 3];
            for k in 0..3 {
                let vd = &src.vertex[&tri[k]];
                p[k] = [vd.x as f32, vd.y as f32, vd.z as f32];
                nr[k] = nrm_of(src, tri[k]);
                d[k] = f(p[k]);
            }
            // Sutherland-Hodgman clip of the triangle against the half-space d<=0.
            let mut pp: Vec<[f32; 3]> = Vec::new();
            let mut pn: Vec<[f32; 3]> = Vec::new();
            for i in 0..3 {
                let j = (i + 1) % 3;
                if d[i] <= 0.0 { pp.push(p[i]); pn.push(nr[i]); }
                if (d[i] <= 0.0) != (d[j] <= 0.0) {
                    let tt = d[i] / (d[i] - d[j]);
                    pp.push(lerp(p[i], p[j], tt));
                    let mut nn = lerp(nr[i], nr[j], tt);
                    let l = (nn[0]*nn[0] + nn[1]*nn[1] + nn[2]*nn[2]).sqrt();
                    if l > 1e-9 { nn = [nn[0]/l, nn[1]/l, nn[2]/l]; }
                    pn.push(nn);
                }
            }
            if pp.len() >= 3 {
                let vks: Vec<usize> = (0..pp.len()).map(|k| {
                    let vk = out.add_vertex(Point::new(pp[k][0] as f64, pp[k][1] as f64, pp[k][2] as f64), None);
                    if let Some(vd) = out.vertex.get_mut(&vk) { vd.set_normal(pn[k][0] as f64, pn[k][1] as f64, pn[k][2] as f64); }
                    vk
                }).collect();
                for k in 1..vks.len() - 1 { out.add_face(vec![vks[0], vks[k], vks[k + 1]], None); }
            }
        }
    }
    out
}

// Build the FULL surface as a structured grid (wrapping where the surface is closed) and
// CUT it by an arbitrary scalar field f, keeping the f<=0 part — a genuine surface cut, not
// a constructed/lofted patch. f = (p-q0)·n is a plane; any closure works (curved cuts, or a
// wedge as the max of two half-space fields).
fn add_cut(
    s: &mut Session, root: &Rc<RefCell<TreeNode>>, srf: &NurbsSurface,
    nu: usize, nv: usize, wrap_u: bool, wrap_v: bool,
    f: impl Fn([f32; 3]) -> f32, name: &str, color: Color,
) {
    let (u0, u1) = match srf.domain(0) { Some(d) => d, None => return };
    let (v0, v1) = match srf.domain(1) { Some(d) => d, None => return };
    let nu_pts = if wrap_u { nu } else { nu + 1 };
    let us: Vec<f32> = (0..nu_pts).map(|i| (u0 + (u1 - u0) * i as f64 / nu as f64) as f32).collect();
    let vfun = |_i: usize| -> Vec<f32> {
        let m = if wrap_v { nv } else { nv + 1 };
        (0..m).map(|j| (v0 + (v1 - v0) * j as f64 / nv as f64) as f32).collect()
    };
    let full = grid_patch_mesh(srf, &us, vfun, wrap_u, wrap_v);
    let mut cut = clip_mesh_by_field(&full, f);
    cut.name = name.to_string();
    cut.set_guid(name.to_string());
    cut.set_objectcolor(color);
    cut.crease_angle_deg = 170.0;
    s.add_mesh(cut, Some(root));
}

// Plane field (point q0, normal n): the signed distance, >0 on the +n side.
fn plane_field(q0: [f32; 3], n: [f32; 3]) -> impl Fn([f32; 3]) -> f32 {
    move |p| (p[0]-q0[0])*n[0] + (p[1]-q0[1])*n[1] + (p[2]-q0[2])*n[2]
}

// g(u,v) = (S(u,v) - q0)·n in surface parameter space.
fn surf_g(srf: &NurbsSurface, u: f32, v: f32, q0: [f32; 3], n: [f32; 3]) -> f32 {
    let p = srf.point_at(u as f64, v as f64).unwrap_or(Point::new(0.0, 0.0, 0.0));
    (p[0] as f32 - q0[0]) * n[0] + (p[1] as f32 - q0[1]) * n[1] + (p[2] as f32 - q0[2]) * n[2]
}

// First v in [v0,v1] where the column at u crosses the plane (q0,n), by bisection. Returns v1
// if the whole column is on the keep side (no crossing).
#[allow(dead_code)]
fn plane_v(srf: &NurbsSurface, u: f32, v0: f32, v1: f32, q0: [f32; 3], n: [f32; 3]) -> f32 {
    let f = |v: f32| surf_g(srf, u, v, q0, n);
    let (f0, f1) = (f(v0), f(v1));
    if (f0 <= 0.0) == (f1 <= 0.0) { return if f0 <= 0.0 { v1 } else { v0 }; }
    let (mut a, mut b, mut fa) = (v0, v1, f0);
    for _ in 0..30 { let m = (a + b) * 0.5; let fm = f(m); if (fm <= 0.0) == (fa <= 0.0) { a = m; fa = fm; } else { b = m; } }
    (a + b) * 0.5
}

// All v in [v0,v1] where the column at u crosses the plane (no periodic wrap), via march + bisect.
#[allow(dead_code)]
fn v_crossings(srf: &NurbsSurface, u: f32, v0: f32, v1: f32, q0: [f32; 3], n: [f32; 3]) -> Vec<f32> {
    let m = 96usize;
    let mut out = Vec::new();
    let (mut pv, mut pf) = (v0, surf_g(srf, u, v0, q0, n));
    for k in 1..=m {
        let cv = v0 + (v1 - v0) * k as f32 / m as f32;
        let cf = surf_g(srf, u, cv, q0, n);
        if (pf <= 0.0) != (cf <= 0.0) {
            let (mut a, mut b, mut fa) = (pv, cv, pf);
            for _ in 0..30 { let mm = (a + b) * 0.5; let fm = surf_g(srf, u, mm, q0, n); if (fm <= 0.0) == (fa <= 0.0) { a = mm; fa = fm; } else { b = mm; } }
            out.push((a + b) * 0.5);
        }
        pv = cv; pf = cf;
    }
    out
}

// Push a UV trim loop's domain-edge segments just OUTSIDE the surface domain, so tessellation
// grid points that sit exactly on u0/u1 (the periodic seam) or v0/v1 land strictly inside the
// loop. Otherwise even-odd point-in-polygon classifies those boundary columns/rows ambiguously
// and drops them → missing corner points and seam gaps. Interior segments (the cut curve / cap
// arcs) are left untouched.
#[allow(dead_code)]
fn nudge_loop_to_domain(pts: &mut [Point], u0: f32, u1: f32, v0: f32, v1: f32) {
    let du = (u1 - u0) * 0.004;
    let dv = (v1 - v0) * 0.004;
    let eu = (u1 - u0) * 1e-4;
    let ev = (v1 - v0) * 1e-4;
    for p in pts.iter_mut() {
        let mut u = p[0];
        let mut v = p[1];
        if (u - u0 as f64).abs() < eu as f64 { u = (u0 - du) as f64; } else if (u - u1 as f64).abs() < eu as f64 { u = (u1 + du) as f64; }
        if (v - v0 as f64).abs() < ev as f64 { v = (v0 - dv) as f64; } else if (v - v1 as f64).abs() < ev as f64 { v = (v1 + dv) as f64; }
        *p = Point::new(u, v, 0.0);
    }
}

// A tube cut (cylinder) as a REAL NurbsSurfaceTrimmed band: keep v in [vlo(u), vhi(u)], the
// surface kept intact + a UV trim loop (OCCT p-curve model). The loop spans the full periodic u,
// so the u-seam becomes the loop's left/right edges. Rendered (trimmed) by the deflection-refined CDT.
#[allow(dead_code)]
fn add_cyl_band(s: &mut Session, root: &Rc<RefCell<TreeNode>>, srf: &NurbsSurface, vlo_of: impl Fn(f32) -> f32, vhi_of: impl Fn(f32) -> f32, name: &str, color: Color) {
    let (u0, u1) = match srf.domain(0) { Some(d) => d, None => return };
    let (v0, v1) = match srf.domain(1) { Some(d) => d, None => return };
    let nu = 96;
    let mut lp: Vec<Point> = Vec::with_capacity(2 * nu + 2);
    for i in 0..=nu { let u = u0 + (u1 - u0) * i as f64 / nu as f64; lp.push(Point::new(u, vlo_of(u as f32) as f64, 0.0)); }
    for i in (0..=nu).rev() { let u = u0 + (u1 - u0) * i as f64 / nu as f64; lp.push(Point::new(u, vhi_of(u as f32) as f64, 0.0)); }
    // No domain nudge: the loop must reach the EXACT u-seam (u0 and u1), where the periodic
    // surface evaluates to identical 3D points so the mesher welds the band into a closed tube.
    // (The old nudge pushed the seam ends to u0-du / u1+du, ~18 units apart, splitting the seam.)
    let _ = (v0, v1);
    let mut ts = NurbsSurfaceTrimmed::new();
    ts.m_surface = srf.clone();
    ts.m_outer_loop = Some(NurbsCurve::create(true, 1, &lp));
    add_trimmed(s, root, &ts, name, color);
}

// A torus plane "bite" as a REAL NurbsSurfaceTrimmed. The plane is offset along its normal so it
// removes only a localized cap. That cap sits on the outer-equator v-seam, so the removed region
// straddles the seam; built the OCCT seam way as a SINGLE outer loop = the domain rect with the
// cap strip removed from top (vb dip) and bottom (va hump). Use a horizontal normal (nz≈0) so the
// straddle is consistent all along the cap (tilted normals would need a full contour tracer).
fn add_torus_bite(s: &mut Session, root: &Rc<RefCell<TreeNode>>, tor: &NurbsSurface, center: [f32; 3], n: [f32; 3], bite_depth: f32, name: &str, color: Color) {
    let (u0, u1) = match tor.domain(0) { Some(d) => d, None => return };
    let (v0, v1) = match tor.domain(1) { Some(d) => d, None => return };
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len < 1e-9 { return; }
    let nh = [n[0] / len, n[1] / len, n[2] / len];
    let mut fmax = -1e30f32;
    for it in 0..=32 {
        let u = u0 + (u1 - u0) * it as f64 / 32.0;
        for jt in 0..=32 {
            let vv = v0 + (v1 - v0) * jt as f64 / 32.0;
            let fv = surf_g(tor, u as f32, vv as f32, center, nh);
            if fv > fmax { fmax = fv; }
        }
    }
    let dist = fmax - bite_depth;
    let q0 = [center[0] + nh[0] * dist, center[1] + nh[1] * dist, center[2] + nh[2] * dist];
    let _ = (u0, u1, v0, v1);
    // Proper trim: keep (S-q0).nh <= 0 (torus minus the outer cap). The contour mesher traces the
    // cut curve onto the plane and welds the periodic u- AND v-seams watertight — no column scan.
    add_plane_trim(s, root, tor, q0, nh, name, color.clone());
    add_cut_plane(s, root, q0, nh, 700.0, &format!("{name}_plane"), color);
}


// The cutting plane itself, as a translucent two-sided quad centred at `q0` with normal `n`,
// sized `half` in each in-plane direction. Lets the cut read at a glance: you see the plane
// slicing through the torus and which side was kept. Alpha-blended (mesh shader passes color.a).
fn add_cut_plane(s: &mut Session, root: &Rc<RefCell<TreeNode>>, q0: [f32; 3], n: [f32; 3], half: f32, name: &str, color: Color) {
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len < 1e-9 { return; }
    let nz = [n[0] / len, n[1] / len, n[2] / len];
    let r = if nz[0].abs() < 0.9 { [1.0f32, 0.0, 0.0] } else { [0.0f32, 1.0, 0.0] };
    let mut u = [r[1] * nz[2] - r[2] * nz[1], r[2] * nz[0] - r[0] * nz[2], r[0] * nz[1] - r[1] * nz[0]];
    let ul = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
    u = [u[0] / ul, u[1] / ul, u[2] / ul];
    let v = [nz[1] * u[2] - nz[2] * u[1], nz[2] * u[0] - nz[0] * u[2], nz[0] * u[1] - nz[1] * u[0]];
    let corner = |su: f32, sv: f32| Point::new(
        (q0[0] + su * half * u[0] + sv * half * v[0]) as f64,
        (q0[1] + su * half * u[1] + sv * half * v[1]) as f64,
        (q0[2] + su * half * u[2] + sv * half * v[2]) as f64,
    );
    let mut mesh = Mesh::new();
    let a = mesh.add_vertex(corner(-1.0, -1.0), None);
    let b = mesh.add_vertex(corner(1.0, -1.0), None);
    let c = mesh.add_vertex(corner(1.0, 1.0), None);
    let d = mesh.add_vertex(corner(-1.0, 1.0), None);
    mesh.add_face(vec![a, b, c, d], None);
    mesh.name = name.to_string();
    mesh.set_guid(name.to_string());
    mesh.set_objectcolor(Color::new(color.r, color.g, color.b, 0.22));
    mesh.crease_angle_deg = 170.0;
    s.add_mesh(mesh, Some(root));
}

// ── Demo A: BReps, NurbsSurfaces and Trimmed NurbsSurfaces (flat layout) ──────
fn trimmed_cdt_demo(s: &mut Session) {
    let root = s.tree.root().expect("session root");
    let pi = std::f32::consts::PI;
    let r = 700.0f32;

    // ── Row 0 — BReps (box, cylinder, sphere, block with hole) ────────────────
    let brep_y = 0.0 * COL;
    {
        let mut b = BRep::create_box(1_000.0, 800.0, 600.0);
        b.name = "brep_box".to_string();
        b.xform = Xform::translation((0.0 * COL) as f64, brep_y as f64, 0.0);
        b.surfacecolor = Color::new(0.3, 0.6, 1.0, 1.0);
        s.add_brep(b, Some(&root));
    }
    {
        let mut b = BRep::create_cylinder(400.0, 800.0);
        b.name = "brep_cylinder".to_string();
        b.xform = Xform::translation((1.0 * COL) as f64, brep_y as f64, 0.0);
        b.surfacecolor = Color::new(0.9, 0.5, 0.2, 1.0);
        s.add_brep(b, Some(&root));
    }
    {
        let mut b = BRep::create_sphere(500.0);
        b.name = "brep_sphere".to_string();
        b.xform = Xform::translation((2.0 * COL) as f64, brep_y as f64, 0.0);
        b.surfacecolor = Color::new(0.35, 0.85, 0.45, 1.0);
        s.add_brep(b, Some(&root));
    }
    {
        let mut b = BRep::create_block_with_hole(1_000.0, 1_000.0, 600.0, 300.0);
        b.name = "brep_block_with_hole".to_string();
        b.xform = Xform::translation((3.0 * COL) as f64, brep_y as f64, 0.0);
        b.surfacecolor = Color::new(0.8, 0.3, 0.7, 1.0);
        s.add_brep(b, Some(&root));
    }

    // ── Row 1 — NurbsSurfaces (untrimmed: cylinder, cone, sphere, torus, wave) ─
    let ns_y = 1.0 * COL;
    add_ns(s, Primitives::cylinder_surface((0.0 * COL) as f64, ns_y as f64, 0.0, 400.0, 800.0), "ns_cylinder", Color::new(0.9, 0.5, 0.2, 1.0));
    add_ns(s, Primitives::cone_surface((1.0 * COL) as f64, ns_y as f64, 0.0, 400.0, 800.0), "ns_cone", Color::new(0.3, 0.6, 1.0, 1.0));
    add_ns(s, Primitives::sphere_surface((2.0 * COL) as f64, ns_y as f64, 500.0, 500.0), "ns_sphere", Color::new(0.35, 0.85, 0.45, 1.0));
    add_ns(s, Primitives::torus_surface((3.0 * COL) as f64, ns_y as f64, 0.0, 450.0, 180.0), "ns_torus", Color::new(0.8, 0.3, 0.7, 1.0));
    {
        // Coarse wave (7×7 CVs) for easy control-point/edit-point handling — fewer than the
        // kernel's 13×13 wave_surface.
        let (size, amp, n) = (1_400.0_f64, 250.0_f64, 7usize);
        let pi2 = 2.0 * std::f64::consts::PI;
        let mut pts = Vec::new();
        for i in 0..n {
            let u = i as f64 / (n - 1) as f64;
            for j in 0..n {
                let v = j as f64 / (n - 1) as f64;
                let z = amp * (pi2 * u).sin() * (pi2 * v).sin();
                pts.push(session_rust::Point::new(size * u, size * v, z));
            }
        }
        let mut wv = session_rust::NurbsSurface::create(false, false, 3, 3, n, n, &pts).unwrap_or_default();
        wv.transform(&Xform::translation((4.0 * COL - 700.0) as f64, (ns_y - 700.0) as f64, 0.0));
        add_ns(s, wv, "ns_wave", Color::new(0.2, 0.7, 0.85, 1.0));
    }

    // ── Trim-loop helpers (all in UV space, surface domain [0,1]x[0,1]) ────────
    // Flat bilinear surface: UV (u,v) maps to world (cx-r+2r*u, cy-r+2r*v, 0),
    // so a UV ngon/circle of centre (0.5,0.5) maps to a world ngon/circle about (cx,cy).
    let flat_srf = |cx: f32, cy: f32| -> NurbsSurface {
        let mut srf = NurbsSurface::create_raw(3, false, 2, 2, 2, 2, false, false, 1.0, 1.0).unwrap();
        srf.set_cv(0, 0, &Point::new((cx - r) as f64, (cy - r) as f64, 0.0));
        srf.set_cv(1, 0, &Point::new((cx + r) as f64, (cy - r) as f64, 0.0));
        srf.set_cv(0, 1, &Point::new((cx - r) as f64, (cy + r) as f64, 0.0));
        srf.set_cv(1, 1, &Point::new((cx + r) as f64, (cy + r) as f64, 0.0));
        srf
    };
    // Regular N-gon in UV, vertex k at angle 2*pi*k/n CCW from +U, radius 0.5.
    let uv_ngon = |n: usize| -> NurbsCurve {
        let pts: Vec<Point> = (0..n).map(|i| {
            let a = 2.0 * pi * i as f32 / n as f32;
            Point::new((0.5 + 0.5 * a.cos()) as f64, (0.5 + 0.5 * a.sin()) as f64, 0.0)
        }).collect();
        NurbsCurve::create(true, 1, &pts)
    };
    // Polygonal circle in UV: centre (cu,cv), radius rad, n segments.
    let uv_poly_circle = |cu: f32, cv: f32, rad: f32, n: usize| -> NurbsCurve {
        let pts: Vec<Point> = (0..n).map(|i| {
            let a = 2.0 * pi * i as f32 / n as f32;
            Point::new((cu + rad * a.cos()) as f64, (cv + rad * a.sin()) as f64, 0.0)
        }).collect();
        NurbsCurve::create(true, 1, &pts)
    };
    // Axis-aligned rectangle in UV.
    let uv_rect = |u0: f32, v0: f32, u1: f32, v1: f32| -> NurbsCurve {
        NurbsCurve::create(true, 1, &[
            Point::new(u0 as f64, v0 as f64, 0.0),
            Point::new(u1 as f64, v0 as f64, 0.0),
            Point::new(u1 as f64, v1 as f64, 0.0),
            Point::new(u0 as f64, v1 as f64, 0.0),
        ])
    };
    // Canonical rational 9-CV degree-2 circle in UV (centre 0.5,0.5 radius 0.5).
    let uv_circle = || -> NurbsCurve {
        let cw = (2.0_f32).sqrt() / 2.0;
        let ccx = [1.0f32, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0];
        let ccy = [0.0f32, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0];
        let cwt = [1.0f32, cw, 1.0, cw, 1.0, cw, 1.0, cw, 1.0];
        let mut c = NurbsCurve::new(3, true, 3, 9);
        c.m_nurbsknot = vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0];
        for i in 0..9 {
            c.set_cv_4d(i, ((0.5 + 0.5 * ccx[i]) * cwt[i]) as f64, ((0.5 + 0.5 * ccy[i]) * cwt[i]) as f64, 0.0, cwt[i] as f64);
        }
        c
    };
    // Wavy degree-3 surface centred at (cx,cy), spanning 2r x 2r in world.
    let wavy_srf = |cx: f32, cy: f32| -> NurbsSurface {
        let mut srf = Primitives::wave_surface((2.0 * r) as f64, 220.0);
        srf.transform(&Xform::translation((cx - r) as f64, (cy - r) as f64, 0.0));
        srf
    };

    // ── Row 2 — Flat trimmed surfaces (circle, hexagon, triangle, octagon) ─────
    let ft_y = 2.0 * COL;
    {
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(0.0 * COL, ft_y);
        ts.m_outer_loop = Some(uv_circle());
        add_trimmed(s, &root, &ts, "ts_circle", Color::new(0.3, 0.6, 1.0, 1.0));
    }
    {
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(1.0 * COL, ft_y);
        ts.m_outer_loop = Some(uv_ngon(6));
        add_trimmed(s, &root, &ts, "ts_hexagon", Color::new(0.9, 0.5, 0.2, 1.0));
    }
    {
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(2.0 * COL, ft_y);
        ts.m_outer_loop = Some(uv_ngon(3));
        add_trimmed(s, &root, &ts, "ts_triangle", Color::new(0.35, 0.85, 0.45, 1.0));
    }
    {
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(3.0 * COL, ft_y);
        ts.m_outer_loop = Some(uv_ngon(8));
        add_trimmed(s, &root, &ts, "ts_octagon", Color::new(0.8, 0.3, 0.7, 1.0));
    }

    // ── Row 3 — Curved trimmed + holes ─────────────────────────────────────────
    let ct_y = 3.0 * COL;
    {
        // Wavy surface cut by a top-projected circle → curved circular patch.
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = wavy_srf(0.0 * COL, ct_y);
        ts.m_outer_loop = Some(uv_poly_circle(0.5, 0.5, 0.48, 48));
        add_trimmed(s, &root, &ts, "ts_wavy_circle", Color::new(0.3, 0.6, 1.0, 1.0));
    }
    {
        // Wavy circular patch with a circular hole.
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = wavy_srf(1.0 * COL, ct_y);
        ts.m_outer_loop = Some(uv_poly_circle(0.5, 0.5, 0.48, 48));
        ts.add_inner_loop(uv_poly_circle(0.5, 0.5, 0.18, 32));
        add_trimmed(s, &root, &ts, "ts_wavy_hole", Color::new(0.9, 0.5, 0.2, 1.0));
    }
    {
        // Flat rectangle with a square hole.
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(2.0 * COL, ct_y);
        ts.m_outer_loop = Some(uv_rect(0.05, 0.05, 0.95, 0.95));
        ts.add_inner_loop(uv_rect(0.35, 0.35, 0.65, 0.65));
        add_trimmed(s, &root, &ts, "ts_rect_hole", Color::new(0.35, 0.85, 0.45, 1.0));
    }
    {
        // Flat circle with a circular hole (annulus).
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(3.0 * COL, ct_y);
        ts.m_outer_loop = Some(uv_circle());
        ts.add_inner_loop(uv_poly_circle(0.5, 0.5, 0.2, 32));
        add_trimmed(s, &root, &ts, "ts_circle_hole", Color::new(0.8, 0.3, 0.7, 1.0));
    }

    // ── Row 4 — swept & revolved primitives ───────────────────────────────────
    // Built at the origin (validated geometry) then translated into the column.
    let p4_y = 4.0 * COL;
    let zaxis = Vector::new(0.0, 0.0, 1.0);
    let place = |mut srf: NurbsSurface, cx: f32| -> NurbsSurface {
        srf.transform(&Xform::translation(cx as f64, p4_y as f64, 0.0));
        srf
    };
    {
        // Cone as TWO surfaces (like a real B-rep cone): the conic side + a flat circular
        // cap at the base. cone_surface has its base circle at z=0 and apex at z=height.
        let cx = 0.0 * COL;
        let cr = 400.0f32;
        add_ns(s, Primitives::cone_surface(cx as f64, p4_y as f64, 0.0, cr as f64, 800.0), "cone_side", Color::new(0.9, 0.5, 0.2, 1.0));
        let mut cap_srf = NurbsSurface::create_raw(3, false, 2, 2, 2, 2, false, false, 1.0, 1.0).unwrap();
        cap_srf.set_cv(0, 0, &Point::new((cx - cr) as f64, (p4_y - cr) as f64, 0.0));
        cap_srf.set_cv(1, 0, &Point::new((cx + cr) as f64, (p4_y - cr) as f64, 0.0));
        cap_srf.set_cv(0, 1, &Point::new((cx - cr) as f64, (p4_y + cr) as f64, 0.0));
        cap_srf.set_cv(1, 1, &Point::new((cx + cr) as f64, (p4_y + cr) as f64, 0.0));
        let mut cap = NurbsSurfaceTrimmed::new();
        cap.m_surface = cap_srf;
        cap.m_outer_loop = Some(uv_circle());
        add_trimmed(s, &root, &cap, "cone_cap", Color::new(0.9, 0.5, 0.2, 1.0));
    }
    {
        // Ellipsoid: a sphere scaled non-uniformly.
        let mut e = Primitives::sphere_surface(0.0, 0.0, 0.0, 450.0);
        e.transform(&Xform::scale_xyz(1.0, 0.65, 1.35));
        add_ns(s, place(e, 1.0 * COL), "ellipsoid", Color::new(0.3, 0.6, 1.0, 1.0));
    }
    {
        // Capsule: revolve a stadium meridian about Z.
        let (cr, hl) = (260.0f32, 320.0f32);
        let arc_n = 12;
        let mut prof: Vec<Point> = Vec::new();
        for i in 0..=arc_n {
            let a = (pi / 2.0) * (i as f32 / arc_n as f32);
            prof.push(Point::new((cr * a.sin()) as f64, 0.0, (hl + cr * a.cos()) as f64));
        }
        for i in 0..=arc_n {
            let a = (pi / 2.0) * (i as f32 / arc_n as f32);
            prof.push(Point::new((cr * a.cos()) as f64, 0.0, (-hl - cr * a.sin()) as f64));
        }
        // Revolve winds outward only when the profile runs bottom→top; our profile is
        // built top→bottom, so reverse it (otherwise the capsule shades inside-out).
        prof.reverse();
        let prof = NurbsCurve::create(false, 1, &prof);
        let cap = Primitives::create_revolve(&prof, &Point::new(0.0, 0.0, 0.0), &zaxis, (2.0 * pi) as f64);
        add_ns(s, place(cap, 2.0 * COL), "capsule", Color::new(0.35, 0.85, 0.45, 1.0));
    }
    {
        // Helix pipe: sweep a circle profile along a helix rail.
        let (turns, rad, pitch, n) = (2.0f32, 280.0f32, 240.0f32, 48usize);
        let helix_pts: Vec<Point> = (0..=n).map(|i| {
            let t = i as f32 / n as f32;
            let a = turns * 2.0 * pi * t;
            Point::new((rad * a.cos()) as f64, (rad * a.sin()) as f64, (pitch * turns * t) as f64)
        }).collect();
        let helix = NurbsCurve::create(false, 3, &helix_pts);
        let profile = Primitives::circle(0.0, 0.0, 0.0, 55.0);
        let pipe = Primitives::create_sweep1(&helix, &profile);
        add_ns(s, place(pipe, 3.0 * COL), "helix_pipe", Color::new(0.8, 0.3, 0.7, 1.0));
    }
    {
        // Polysolid: sweep a square section along a polyline path.
        let path = NurbsCurve::create(false, 1, &[
            Point::new(-500.0, -300.0, 0.0),
            Point::new(300.0, -300.0, 0.0),
            Point::new(300.0, 300.0, 200.0),
            Point::new(-200.0, 300.0, 500.0),
        ]);
        let h = 80.0;
        let square = NurbsCurve::create(true, 1, &[
            Point::new(-h, -h, 0.0),
            Point::new(h, -h, 0.0),
            Point::new(h, h, 0.0),
            Point::new(-h, h, 0.0),
        ]);
        let poly = Primitives::create_sweep1(&path, &square);
        add_ns(s, place(poly, 4.0 * COL), "polysolid", Color::new(0.2, 0.7, 0.85, 1.0));
    }
    {
        // Gasket: a flat ring with a centre bore and a bolt-hole pattern.
        let cx = 5.0 * COL;
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(cx, p4_y);
        ts.m_outer_loop = Some(uv_poly_circle(0.5, 0.5, 0.48, 64));
        ts.add_inner_loop(uv_poly_circle(0.5, 0.5, 0.18, 32));
        for k in 0..8 {
            let a = 2.0 * pi * k as f32 / 8.0;
            ts.add_inner_loop(uv_poly_circle(0.5 + 0.33 * a.cos(), 0.5 + 0.33 * a.sin(), 0.05, 16));
        }
        add_trimmed(s, &root, &ts, "gasket", Color::new(0.6, 0.6, 0.65, 1.0));
    }

    // ── Row 5 — CYLINDER cut by a plane (proper plane-cut mesher) ──────────────
    // mesh_by_plane traces the cut curve onto the plane and welds the u-seam watertight; the
    // kept half is (S-q0).n <= 0 (below the plane, down to the bottom rim). Edit q0/n to try configs.
    let p5_y = 5.0 * COL;
    {
        let cx = 0.0 * COL;
        let cyl = Primitives::cylinder_surface(cx as f64, p5_y as f64, 0.0, 400.0, 900.0);
        let (q0, n) = ([cx, p5_y, 450.0], [0.8, 0.0, 1.0]);
        add_plane_trim(s, &root, &cyl, q0, n, "cyl_cut_plane_x", Color::new(0.3, 0.6, 1.0, 1.0));
        add_cut_plane(s, &root, q0, n, 480.0, "cyl_cut_plane_x_plane", Color::new(0.3, 0.6, 1.0, 1.0));
    }

    // ── Row 6 — TORUS split into wedges as REAL rational sub-surfaces ───────────
    // NurbsSurface::trim (now rational-correct, OCCT RectangularTrimmedSurface style)
    // restricts the torus to an angular sub-range → a true rational NURBS surface, rendered
    // smooth via the normal NurbsSurface path (iso-curve boundaries), not a mesh/CDT.
    let p6_y = 6.0 * COL;
    {
        let tor = Primitives::torus_surface(0.0, p6_y as f64, 0.0, 450.0, 160.0);
        let (tu0, tu1) = tor.domain(0).unwrap();
        let span = tu1 - tu0;
        let parts = 4;
        let cols = [
            Color::new(0.3, 0.6, 1.0, 1.0),
            Color::new(0.35, 0.85, 0.45, 1.0),
            Color::new(0.8, 0.3, 0.7, 1.0),
            Color::new(0.9, 0.6, 0.2, 1.0),
        ];
        for p in 0..parts {
            let g = span * 0.03; // angular gap so the wedges read as separate
            let ua = tu0 + span * p as f64 / parts as f64 + g;
            let ub = tu0 + span * (p + 1) as f64 / parts as f64 - g;
            let mut wedge = tor.clone();
            if wedge.trim(0, (ua, ub)) {
                add_ns(s, wedge, &format!("torus_wedge_{p}"), cols[p as usize].clone());
            }
        }
    }

    // ── Row 7 — TORUS plane "bites" as REAL NurbsSurfaceTrimmed (surface + UV trim loop) ─
    // The plane is offset along its normal so it removes only a localized cap. That cap sits on
    // the outer-equator v-seam, so the removed region straddles the seam (the OCCT seam case):
    // the kept region is built as a single outer loop = domain rect minus the cap strip, top &
    // bottom (see add_torus_bite). Vertical normals (nz=0) keep the straddle consistent along the
    // cap; the directions vary so the bite lands on different sides of the ring (away from the
    // u-seam at +x). Tilted-plane torus cuts would need a full seam-weaving contour tracer.
    let p7_y = 7.0 * COL;
    {
        // ONE torus cut by a plane (edit n / bite_depth to try configurations).
        let (n, bite_depth) = ([-1.0f32, 0.0, 0.0], 170.0f32);
        let cx = 0.0 * COL;
        let tor = Primitives::torus_surface(cx as f64, p7_y as f64, 0.0, 450.0, 160.0);
        add_torus_bite(s, &root, &tor, [cx, p7_y, 0.0], n, bite_depth, "torus_bite", Color::new(0.35, 0.85, 0.45, 1.0));
    }

    // ── Row 8 — SPLIT: surfaces cut by one or more planes into ALL their parts ──────────
    // split_by_planes keeps every region (2^K sign combinations), each a first-class multi-plane
    // NurbsSurfaceTrimmed — selectable, hideable, transformable. Parts are exploded outward so the
    // partition reads clearly. This is "split", not "trim": nothing is discarded.
    let p8_y = 8.0 * COL;
    {
        // Sphere split by ONE horizontal plane → 2 caps.
        let sph = Primitives::sphere_surface((0.0 * COL) as f64, p8_y as f64, 0.0, 360.0);
        add_split(s, &root, &sph, &[([0.0, p8_y, 40.0], [0.0, 0.0, 1.0])], "split_sphere_1", 120.0);

        // Cylinder split by TWO planes (slanted X + horizontal) → up to 4 parts.
        let cyl = Primitives::cylinder_surface((1.0 * COL) as f64, p8_y as f64, 0.0, 360.0, 900.0);
        add_split(s, &root, &cyl, &[
            ([1.0 * COL, p8_y, 0.0], [1.0, 0.0, 0.4]),
            ([1.0 * COL, p8_y, 0.0], [0.0, 1.0, 0.0]),
        ], "split_cyl_2", 220.0);

        // Cone split by THREE axis-aligned planes → up to 8 parts.
        let cone = Primitives::cone_surface((2.0 * COL) as f64, p8_y as f64, 0.0, 380.0, 820.0);
        add_split(s, &root, &cone, &[
            ([2.0 * COL, p8_y, 0.0], [1.0, 0.0, 0.0]),
            ([2.0 * COL, p8_y, 0.0], [0.0, 1.0, 0.0]),
            ([2.0 * COL, p8_y, 410.0], [0.0, 0.0, 1.0]),
        ], "split_cone_3", 240.0);

        // Torus split by TWO vertical planes → ring quarters (4 parts).
        let tor = Primitives::torus_surface((3.0 * COL) as f64, p8_y as f64, 0.0, 450.0, 160.0);
        add_split(s, &root, &tor, &[
            ([3.0 * COL, p8_y, 0.0], [1.0, 0.0, 0.0]),
            ([3.0 * COL, p8_y, 0.0], [0.0, 1.0, 0.0]),
        ], "split_torus_2", 200.0);
    }
}

// ── Demo B: compas_tf floor model ────────────────────────────────────────────
fn compas_tf_demo(s: &mut Session) {
    const PB: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../session_data/floor_model.pb"));
    const PROMOTE: &[&str] = &[];
    let floor = Session::pb_loads(PB).unwrap_or_default();
    let Some(floor_root) = floor.tree.root() else { return };
    let floor_group = s.add_group("FloorModel");
    for child in &floor_root.borrow().children() {
        merge_tree_node(child, &floor_group, &floor_group, PROMOTE, s, &floor);
    }
    for (u, v) in floor.graph.get_edges() {
        let attr = floor.graph.edges.get(&u)
            .and_then(|nb| nb.get(&v))
            .map(|e| e.attribute.as_str())
            .unwrap_or("");
        s.add_edge(&u, &v, attr);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Switch demo here — comment/uncomment one line:
pub fn active_scene() -> (Session, Vec<CylinderSegment>) {
    make_floor_scene()
    // make_cdt_scene()
}

fn make_cdt_scene() -> (Session, Vec<CylinderSegment>) {
    let mut s = Session::new("trimmed_cdt");
    trimmed_cdt_demo(&mut s);
    (s, Vec::new())
}

fn make_floor_scene() -> (Session, Vec<CylinderSegment>) {
    let mut s = Session::new("floor");
    compas_tf_demo(&mut s);
    (s, Vec::new())
}

// ── Demo C: GPU instancing — 100 boxes, 1 draw call ──────────────────────────
// Called from State::new() after gpu_session.rebuild_from(), not from active_scene().
pub fn instancing_demo(gpu_session: &mut GpuSession, device: &wgpu::Device, queue: &wgpu::Queue) {
    let (verts, inds) = mesh_to_vertices(&BRep::create_box(100.0, 100.0, 100.0).mesh());
    let key = TemplateKey::new("demo_box");
    gpu_session.register_template_mesh(&key, &verts, &inds, device, queue);
    for row in 0..10u32 {
        for col in 0..10u32 {
            let tx = col as f32 * 200.0;
            let ty = row as f32 * 200.0;
            let model = [[1.,0.,0.,0.],[0.,1.,0.,0.],[0.,0.,1.,0.],[tx,ty,0.,1.]];
            let color = [col as f32 / 9.0, row as f32 / 9.0, 0.5, 1.0];
            gpu_session.add_instance(&key, &format!("box_{row}_{col}"), model, color, 0, device, queue);
        }
    }
}
