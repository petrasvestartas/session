//! Bytes to a kernel `Session`, chunked: prost decodes the proto in one short block, then the
//! objects are converted CHUNK at a time with a macrotask between chunks so a 250k-object
//! parse never freezes the page. The bytes are taken by value and dropped after the decode.

use std::rc::Rc;
use prost::Message;
use session_rust::proto;
use session_rust::tree::{Tree, TreeNode};
use session_rust::{BRep, Element, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, Plane, Point, PointCloud, Polyline, Session, Xform, OBB};
use super::fetch::next_tick;

/// Objects converted between two yields.
const CHUNK: usize = 25_000;

/// Counts conversions and yields every `CHUNK`.
struct Pacer {
    n: usize,
}

impl Pacer {
    /// One more object converted; true when the caller should yield now.
    fn tick(&mut self) -> bool {
        self.n += 1;
        self.n.is_multiple_of(CHUNK)
    }
}

/// The same conversion loop for every type: proto -> object, stored, paused every CHUNK.
macro_rules! convert {
    ($s:expr, $pacer:expr, $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
        for x in $vec {
            let g = Rc::new($ty::from_proto(x));
            $s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
            $s.objects.$slot.push(g);
            if $pacer.tick() {
                next_tick().await;
            }
        }
    };
    (fallible $s:expr, $pacer:expr, $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
        for x in $vec {
            let Ok(v) = $ty::from_proto(x) else { continue };
            let g = Rc::new(v);
            $s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
            $s.objects.$slot.push(g);
            if $pacer.tick() {
                next_tick().await;
            }
        }
    };
}

/// `Session::pb_loads`, unrolled with awaits. `.json` files take the synchronous path.
pub async fn session_from_bytes(url: &str, bytes: Vec<u8>) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(&bytes));
    }
    let Ok(p) = proto::Session::decode(bytes.as_slice()) else { return Session::default() };
    drop(bytes);
    let mut s = Session::new(&p.name);
    s.set_guid(p.guid.clone());
    let mut pacer = Pacer { n: 0 };

    if let Some(o) = p.objects {
        s.objects.set_guid(o.guid);
        s.objects.name = o.name;
        convert!(s, pacer, o.points, Point, Point, points);
        convert!(s, pacer, o.lines, Line, Line, lines);
        convert!(s, pacer, o.planes, Plane, Plane, planes);
        convert!(fallible s, pacer, o.bboxes, OBB, OBB, bboxes);
        convert!(s, pacer, o.polylines, Polyline, Polyline, polylines);
        convert!(s, pacer, o.pointclouds, PointCloud, PointCloud, pointclouds);
        convert!(s, pacer, o.meshes, Mesh, Mesh, meshes);
        convert!(s, pacer, o.nurbscurves, NurbsCurve, NurbsCurve, nurbscurves);
        convert!(fallible s, pacer, o.nurbssurfaces, NurbsSurface, NurbsSurface, nurbssurfaces);
        convert!(fallible s, pacer, o.breps, BRep, BRep, breps);
        convert!(fallible s, pacer, o.elements, Element, Element, elements);
    }

    for entry in &p.xforms {
        let Some(xf) = &entry.xform else { continue };
        let mut xform = Xform::identity();
        xform.set_guid(xf.guid.clone());
        xform.name = xf.name.clone();
        for (i, val) in xf.matrix.iter().enumerate().take(16) {
            xform.m[i] = *val;
        }
        s.xforms.insert(entry.guid.clone(), xform);
    }

    if let Some(gp) = &p.graph {
        s.graph = session_rust::Graph::new(&gp.name);
        s.graph.set_guid(gp.guid.clone());
        for (name, v) in &gp.vertices {
            s.graph.add_node(name, &v.attribute);
        }
        for e in &gp.edges {
            s.graph.add_edge(&e.v0, &e.v1, &e.attribute);
        }
    }

    if let Some(tp) = &p.tree {
        s.tree = Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            let root = build_tree(rp);
            s.tree.add(&root, None);
        }
    }
    s
}

/// One proto node and its children, recursively.
fn build_tree(proto: &proto::TreeNode) -> Rc<std::cell::RefCell<TreeNode>> {
    let node = TreeNode::new(&proto.name);
    for c in &proto.children {
        let child = build_tree(c);
        node.borrow_mut().add(&child);
    }
    node
}
