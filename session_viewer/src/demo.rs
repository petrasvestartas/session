use session_rust::{
    BRep, Color, Session, TreeNode, Xform,
};
use session_rust::primitives::Primitives;

use crate::gpu_session::CylinderSegment;

const COL: f32 = 1_500.0;

pub fn make_demo_scene() -> (Session, Vec<CylinderSegment>) {
    let mut s = Session::new("demo");
    let demo_cones: Vec<CylinderSegment> = Vec::new();

    // Row 0 — BRep
    let g_brep = s.add_group("BRep");
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
    let g_ns = s.add_group("NurbsSurface");
    let ns_y = 1.0 * COL;
    {
        let mut ns = Primitives::cylinder_surface(0.0 * COL, ns_y, 0.0, 400.0, 800.0);
        ns.name = "ns_cylinder".to_string();
        ns.set_guid("ns_cylinder".to_string());
        ns.linecolors.push(Color::new(0.9, 0.5, 0.2, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_cylinder"), Some(&g_ns));
    }
    {
        let mut ns = Primitives::cone_surface(2.0 * COL, ns_y, 0.0, 400.0, 800.0);
        ns.name = "ns_cone".to_string();
        ns.set_guid("ns_cone".to_string());
        ns.linecolors.push(Color::new(0.3, 0.6, 1.0, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_cone"), Some(&g_ns));
    }
    {
        let mut ns = Primitives::sphere_surface(4.0 * COL, ns_y + 500.0, 0.0, 500.0);
        ns.name = "ns_sphere".to_string();
        ns.set_guid("ns_sphere".to_string());
        ns.linecolors.push(Color::new(0.35, 0.85, 0.45, 1.0));
        s.objects.nurbssurfaces.push(ns);
        s.tree.add(&TreeNode::new("ns_sphere"), Some(&g_ns));
    }

    (s, demo_cones)
}
