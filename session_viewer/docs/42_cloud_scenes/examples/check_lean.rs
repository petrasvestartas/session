// Equivalence check for the viewer's lean decode: build the SAME file through
// `Session::pb_loads` (the kernel's full path) and through the viewer's lean path, walk both
// into GPU tables, and compare the tables byte for byte. A skipped proto field that mattered
// shows up here as a table mismatch.
use prost::Message;
use std::rc::Rc;
use session_rust::{Session, Geometry, Xform, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, Polyline, PointCloud, BRep, Element};
use session_viewer::app::persistence::LeanSessionProbe;
use session_viewer::app::scene::Scene;

fn lean_session(bytes: &[u8]) -> Session {
    let p = LeanSessionProbe::decode(bytes).expect("lean decode");
    let mut s = Session::new(&p.name);
    s.set_guid(p.guid.clone());
    macro_rules! put { ($g:expr, $v:ident, $slot:ident) => {{
        let g = Rc::new($g);
        s.lookup.insert(g.guid().to_string(), Geometry::$v(Rc::clone(&g)));
        s.objects.$slot.push(g);
    }}; }
    if let Some(o) = p.objects {
        s.objects.set_guid(o.guid); s.objects.name = o.name;
        for x in o.points { put!(Point::from_proto(x), Point, points) }
        for x in o.lines { put!(Line::from_proto(x), Line, lines) }
        for x in o.planes { put!(Plane::from_proto(x), Plane, planes) }
        for x in o.bboxes { if let Ok(v) = OBB::from_proto(x) { put!(v, OBB, bboxes) } }
        for x in o.polylines { put!(Polyline::from_proto(x), Polyline, polylines) }
        for x in o.pointclouds { put!(PointCloud::from_proto(x), PointCloud, pointclouds) }
        for x in o.meshes { put!(Mesh::from_proto(x.into_proto_pub()), Mesh, meshes) }
        for x in o.nurbscurves { put!(NurbsCurve::from_proto(x), NurbsCurve, nurbscurves) }
        for x in o.nurbssurfaces { if let Ok(v) = NurbsSurface::from_proto(x) { put!(v, NurbsSurface, nurbssurfaces) } }
        for x in o.breps { if let Ok(v) = BRep::from_proto(x) { put!(v, BRep, breps) } }
        for x in o.elements { if let Ok(v) = Element::from_proto(x) { put!(v, Element, elements) } }
    }
    // …and the transform/tree tail, exactly as the loader does it.
    for entry in &p.xforms {
        if let Some(xf) = &entry.xform {
            let mut xform = Xform::identity();
            xform.set_guid(xf.guid.clone());
            xform.name = xf.name.clone();
            for (i, val) in xf.matrix.iter().enumerate().take(16) { xform.m[i] = *val; }
            s.xforms.insert(entry.guid.clone(), xform);
        }
    }
    if s.xforms.is_empty() { return s }
    let t = session_viewer::app::persistence::TreeOnlyProbe::decode(bytes).expect("tree decode");
    if let Some(tp) = &t.tree {
        s.tree = session_rust::tree::Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            fn build(pr: &session_rust::proto::TreeNode) -> Rc<std::cell::RefCell<session_rust::tree::TreeNode>> {
                let node = session_rust::tree::TreeNode::new(&pr.name);
                for c in &pr.children { let child = build(c); node.borrow_mut().add(&child); }
                node
            }
            let root = build(rp);
            s.tree.add(&root, None);
        }
    }
    s
}

fn tables_of(session: Session) -> Scene {
    let mut sc = Scene::new();
    sc.add_file("check".into(), session, Xform::identity(), 0.0, false);
    sc
}

fn main() {
    let mut bad = 0;
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let full = tables_of(Session::pb_loads(&bytes).expect("pb_loads"));
        let lean = if std::env::var("SELF_CHECK").is_ok() {
            tables_of(Session::pb_loads(&bytes).expect("pb_loads"))
        } else {
            tables_of(lean_session(&bytes))
        };
        let (a, b) = (&full.tables, &lean.tables);
        let mut ok = true;
        macro_rules! same { ($f:ident) => {
            if bytemuck::cast_slice::<_, u8>(&a.$f) != bytemuck::cast_slice::<_, u8>(&b.$f) {
                println!("  MISMATCH {}: {} vs {}", stringify!($f), a.$f.len(), b.$f.len()); ok = false;
            }
        }; }
        same!(verts); same!(idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
        if std::env::var("DETAIL").is_ok() {
            for (i, (x, y)) in a.pipes.iter().zip(&b.pipes).enumerate() {
                if bytemuck::bytes_of(x) != bytemuck::bytes_of(y) {
                    println!("  pipe[{i}] p0 {:?}/{:?} r {}/{} p1 {:?}/{:?} inst {}/{} col {:08x}/{:08x} facing {:08x}/{:08x}",
                        x.p0, y.p0, x.radius, y.radius, x.p1, y.p1, x.instance_id, y.instance_id, x.color, y.color, x.facing, y.facing);
                    break;
                }
            }
            for (i, (x, y)) in a.spheres.iter().zip(&b.spheres).enumerate() {
                if bytemuck::bytes_of(x) != bytemuck::bytes_of(y) {
                    println!("  sphere[{i}] c {:?}/{:?} inst {}/{} col {:?}/{:?}",
                        x.center, y.center, x.instance_id, y.instance_id, x.color, y.color);
                    break;
                }
            }
        }
        same!(cloud_pos); same!(cloud_col); same!(cloud_nrm);
        if a.objects.len() != b.objects.len() { println!("  MISMATCH objects rows"); ok = false; }
        if a.min != b.min || a.max != b.max { println!("  MISMATCH bounds {:?}{:?} vs {:?}{:?}", a.min, a.max, b.min, b.max); ok = false; }
        for (x, y) in a.objects.iter().zip(&b.objects) {
            if x.0 != y.0 || x.1 != y.1 || x.2 != y.2 { println!("  MISMATCH object row"); ok = false; break }
        }
        println!("{}: {}", path.rsplit('/').next().unwrap_or(&path), if ok { "IDENTICAL" } else { bad += 1; "DIFFERS" });
    }
    if bad > 0 { std::process::exit(1) }
}
