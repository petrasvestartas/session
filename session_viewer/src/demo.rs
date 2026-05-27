#![allow(dead_code)]
use session_rust::{
    BRep, Color, Mesh, NurbsCurve, NurbsSurface, NurbsSurfaceTrimmed, Point, Session, TreeNode, Xform,
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
                let mut m = m.clone();
                m.crease_angle_deg = 15.0;
                dst.add_mesh(m, Some(parent));
            }
            Geometry::Polyline(p) => { dst.add_polyline(p.clone(), Some(parent)); }
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

use crate::gpu_session::CylinderSegment;

const COL: f32 = 1_500.0;

// ── Demo A: Trimmed NurbsSurface CDT fix ─────────────────────────────────────
fn trimmed_cdt_demo(s: &mut Session) {
    let cdt_root = s.add_group("CDT");
    // Row 0 — BRep
    let g_brep = TreeNode::new("BRep");
    s.add(&g_brep, Some(&cdt_root));
    {
        let mut b = BRep::create_box(1_000.0, 800.0, 600.0);
        b.name = "brep_box".to_string();
        b.xform = Xform::translation(0.0 * COL, 0.0, 0.0);
        b.surfacecolor = Color::new(0.3, 0.6, 1.0, 1.0);
        s.add_brep(b, Some(&g_brep));
    }
    {
        let mut b = BRep::create_cylinder(400.0, 800.0);
        b.name = "brep_cylinder".to_string();
        b.xform = Xform::translation(2.0 * COL, 0.0, 0.0);
        b.surfacecolor = Color::new(0.9, 0.5, 0.2, 1.0);
        s.add_brep(b, Some(&g_brep));
    }
    {
        let mut b = BRep::create_sphere(500.0);
        b.name = "brep_sphere".to_string();
        b.xform = Xform::translation(4.0 * COL, 0.0, 0.0);
        b.surfacecolor = Color::new(0.35, 0.85, 0.45, 1.0);
        s.add_brep(b, Some(&g_brep));
    }

    // Row 1 — NurbsSurface
    let g_ns = TreeNode::new("NurbsSurface");
    s.add(&g_ns, Some(&cdt_root));
    let ns_y = 1.0 * COL;
    {
        let mut ns = Primitives::cylinder_surface(0.0 * COL, ns_y, 0.0, 400.0, 800.0);
        ns.name = "ns_cylinder".to_string();
        ns.set_guid("ns_cylinder".to_string());
        ns.facecolors.push(Color::new(0.9, 0.5, 0.2, 1.0));
        ns.linecolors.push(Color::new(1.0, 1.0, 1.0, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_cylinder"), Some(&g_ns));
    }
    {
        let mut ns = Primitives::cone_surface(2.0 * COL, ns_y, 0.0, 400.0, 800.0);
        ns.name = "ns_cone".to_string();
        ns.set_guid("ns_cone".to_string());
        ns.facecolors.push(Color::new(0.3, 0.6, 1.0, 1.0));
        ns.linecolors.push(Color::new(1.0, 1.0, 1.0, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_cone"), Some(&g_ns));
    }
    {
        let mut ns = Primitives::sphere_surface(4.0 * COL, ns_y + 500.0, 0.0, 500.0);
        ns.name = "ns_sphere".to_string();
        ns.set_guid("ns_sphere".to_string());
        ns.facecolors.push(Color::new(0.35, 0.85, 0.45, 1.0));
        ns.linecolors.push(Color::new(1.0, 1.0, 1.0, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_sphere"), Some(&g_ns));
    }

    // Row 2 — TrimmedNurbsSurface CDT fix
    // Flat bilinear surface + UV trim loop → CDT mesh via sweep-line (robust for circles)
    let g_ts = TreeNode::new("TrimmedNurbsSurface");
    s.add(&g_ts, Some(&cdt_root));
    let ts_y = 2.0 * COL;
    let r = 700.0f32;

    let flat_srf = |cx: f32, cy: f32| -> NurbsSurface {
        let mut srf = NurbsSurface::create_raw(3, false, 2, 2, 2, 2, false, false, 1.0, 1.0).unwrap();
        srf.set_cv(0, 0, &Point::new(cx - r, cy - r, 0.0));
        srf.set_cv(1, 0, &Point::new(cx + r, cy - r, 0.0));
        srf.set_cv(0, 1, &Point::new(cx - r, cy + r, 0.0));
        srf.set_cv(1, 1, &Point::new(cx + r, cy + r, 0.0));
        srf
    };

    let uv_ngon = |n: usize| -> NurbsCurve {
        let pts: Vec<Point> = (0..n).map(|i| {
            let a = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
            Point::new(0.5 + 0.5 * a.cos(), 0.5 + 0.5 * a.sin(), 0.0)
        }).collect();
        NurbsCurve::create(true, 1, &pts)
    };

    // Rational NURBS circle in UV space — the case that was broken with Bowyer-Watson CDT
    let uv_circle = || -> NurbsCurve {
        let cw = (2.0_f32).sqrt() / 2.0;
        let ccx = [1.0f32, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0];
        let ccy = [0.0f32, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0];
        let cwt = [1.0f32, cw, 1.0, cw, 1.0, cw, 1.0, cw, 1.0];
        let mut c = NurbsCurve::new(3, true, 3, 9);
        c.m_nurbsknot = vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0];
        for i in 0..9 {
            c.set_cv_4d(i, (0.5 + 0.5 * ccx[i]) * cwt[i], (0.5 + 0.5 * ccy[i]) * cwt[i], 0.0, cwt[i]);
        }
        c
    };

    {
        // Circle — rational NURBS degree 2, sampled to 36 pts, CDT via sweep-line
        let cx = 0.0 * COL;
        let g = TreeNode::new("Circle");
        s.tree.add(&g, Some(&g_ts));
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(cx, ts_y);
        ts.m_outer_loop = Some(uv_circle());
        let mut mesh = ts.mesh();
        mesh.name = "ts_circle".to_string();
        mesh.set_guid("ts_circle".to_string());
        mesh.set_objectcolor(Color::new(0.3, 0.6, 1.0, 1.0));
        s.add_mesh(mesh, Some(&g));
        let nc = Primitives::circle(cx, ts_y, 0.0, r);
        let nc = { let mut c = nc.duplicate(); c.name = "ts_circle_boundary".to_string(); c.set_guid("ts_circle_boundary".to_string()); c };
        s.objects.nurbscurves.push(nc);
        s.tree.add(&TreeNode::new("ts_circle_boundary"), Some(&g));
    }
    {
        // Hexagon
        let cx = 2.0 * COL;
        let g = TreeNode::new("Hexagon");
        s.tree.add(&g, Some(&g_ts));
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(cx, ts_y);
        ts.m_outer_loop = Some(uv_ngon(6));
        let mut mesh = ts.mesh();
        mesh.name = "ts_hexagon".to_string();
        mesh.set_guid("ts_hexagon".to_string());
        mesh.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));
        s.add_mesh(mesh, Some(&g));
        let pts: Vec<Point> = (0..6).map(|i| {
            let a = 2.0 * std::f32::consts::PI * i as f32 / 6.0;
            Point::new(cx + r * a.cos(), ts_y + r * a.sin(), 0.0)
        }).collect();
        let nc = { let mut c = NurbsCurve::create(true, 1, &pts); c.name = "ts_hexagon_boundary".to_string(); c.set_guid("ts_hexagon_boundary".to_string()); c };
        s.objects.nurbscurves.push(nc);
        s.tree.add(&TreeNode::new("ts_hexagon_boundary"), Some(&g));
    }
    {
        // Triangle
        let cx = 4.0 * COL;
        let g = TreeNode::new("Triangle");
        s.tree.add(&g, Some(&g_ts));
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(cx, ts_y);
        ts.m_outer_loop = Some(uv_ngon(3));
        let mut mesh = ts.mesh();
        mesh.name = "ts_triangle".to_string();
        mesh.set_guid("ts_triangle".to_string());
        mesh.set_objectcolor(Color::new(0.35, 0.85, 0.45, 1.0));
        s.add_mesh(mesh, Some(&g));
        let pts: Vec<Point> = (0..3).map(|i| {
            let a = 2.0 * std::f32::consts::PI * i as f32 / 3.0 - std::f32::consts::PI / 2.0;
            Point::new(cx + r * a.cos(), ts_y + r * a.sin(), 0.0)
        }).collect();
        let nc = { let mut c = NurbsCurve::create(true, 1, &pts); c.name = "ts_triangle_boundary".to_string(); c.set_guid("ts_triangle_boundary".to_string()); c };
        s.objects.nurbscurves.push(nc);
        s.tree.add(&TreeNode::new("ts_triangle_boundary"), Some(&g));
    }
    {
        // Octagon
        let cx = 6.0 * COL;
        let g = TreeNode::new("Octagon");
        s.tree.add(&g, Some(&g_ts));
        let mut ts = NurbsSurfaceTrimmed::new();
        ts.m_surface = flat_srf(cx, ts_y);
        ts.m_outer_loop = Some(uv_ngon(8));
        let mut mesh = ts.mesh();
        mesh.name = "ts_octagon".to_string();
        mesh.set_guid("ts_octagon".to_string());
        mesh.set_objectcolor(Color::new(0.8, 0.3, 0.7, 1.0));
        s.add_mesh(mesh, Some(&g));
        let pts: Vec<Point> = (0..8).map(|i| {
            let a = 2.0 * std::f32::consts::PI * i as f32 / 8.0;
            Point::new(cx + r * a.cos(), ts_y + r * a.sin(), 0.0)
        }).collect();
        let nc = { let mut c = NurbsCurve::create(true, 1, &pts); c.name = "ts_octagon_boundary".to_string(); c.set_guid("ts_octagon_boundary".to_string()); c };
        s.objects.nurbscurves.push(nc);
        s.tree.add(&TreeNode::new("ts_octagon_boundary"), Some(&g));
    }

    // Row 3 — Mesh (crease edges)
    let g_mc = TreeNode::new("MeshCrease");
    s.add(&g_mc, Some(&cdt_root));
    let mc_y = 3.0 * COL;
    {
        let cx = 0.0 * COL;
        let (hw, hd, h) = (400.0f32, 400.0f32, 800.0f32);
        let verts = vec![
            Point::new(cx - hw, mc_y - hd, 0.0),
            Point::new(cx + hw, mc_y - hd, 0.0),
            Point::new(cx + hw, mc_y + hd, 0.0),
            Point::new(cx - hw, mc_y + hd, 0.0),
            Point::new(cx - hw, mc_y - hd, h),
            Point::new(cx + hw, mc_y - hd, h),
            Point::new(cx + hw, mc_y + hd, h),
            Point::new(cx - hw, mc_y + hd, h),
        ];
        let faces = vec![
            vec![3, 2, 1, 0],
            vec![4, 5, 6, 7],
            vec![0, 1, 5, 4],
            vec![1, 2, 6, 5],
            vec![2, 3, 7, 6],
            vec![3, 0, 4, 7],
        ];
        let mut mesh = Mesh::from_vertices_and_faces(verts, faces);
        mesh.name = "mc_rect".to_string();
        mesh.set_guid("mc_rect".to_string());
        mesh.set_objectcolor(Color::new(0.3, 0.6, 1.0, 1.0));
        mesh.crease_angle_deg = 30.0;
        s.add_mesh(mesh, Some(&g_mc));
    }
    {
        let cx = 2.0 * COL;
        let (r, h) = (400.0f32, 800.0f32);
        let pi = std::f32::consts::PI;
        let bot: Vec<Point> = (0..8).map(|i| {
            let a = i as f32 * pi / 4.0;
            Point::new(cx + r * a.cos(), mc_y + r * a.sin(), 0.0)
        }).collect();
        let top: Vec<Point> = (0..8).map(|i| {
            let a = i as f32 * pi / 4.0;
            Point::new(cx + r * a.cos(), mc_y + r * a.sin(), h)
        }).collect();
        let mut verts: Vec<Point> = Vec::new();
        verts.extend(bot);
        verts.extend(top);
        let mut faces: Vec<Vec<usize>> = Vec::new();
        faces.push(vec![7, 6, 5, 4, 3, 2, 1, 0]);
        faces.push(vec![8, 9, 10, 11, 12, 13, 14, 15]);
        for i in 0..8 {
            faces.push(vec![i, (i + 1) % 8, 8 + (i + 1) % 8, 8 + i]);
        }
        let mut mesh = Mesh::from_vertices_and_faces(verts, faces);
        mesh.name = "mc_oct".to_string();
        mesh.set_guid("mc_oct".to_string());
        mesh.set_objectcolor(Color::new(0.9, 0.5, 0.2, 1.0));
        mesh.crease_angle_deg = 50.0;
        s.add_mesh(mesh, Some(&g_mc));
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
