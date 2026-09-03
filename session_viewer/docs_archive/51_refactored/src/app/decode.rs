//! The whole-file decode: prost decodes the proto in one short block, then the kernel objects
//! are converted CHUNK at a time with a macrotask between chunks, so a 250k-object parse no
//! longer freezes the page. Reads bytes; hands back a kernel `Session`.

use std::rc::Rc;
use prost::Message;
use session_rust::proto;
use session_rust::{Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, Polyline, PointCloud, BRep, Element, Session, Xform};
use session_rust::tree::{Tree, TreeNode};
use super::fetch::next_tick;

/// Objects converted per slice before the loader hands the browser one macrotask — the whole
/// point is that a frame can render BETWEEN slices, so a 250k-object parse stops freezing the UI.
const CHUNK: usize = 25_000;

/// The wire `Session` without its `tree` (4), `graph` (5) and `bvh_boxes` (6): prost skips a
/// field it is not asked for without allocating, and a display-only document never reads those
/// three - on a sheet they are 52% of the decoded session. Same tags as `proto::Session`.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanSession {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, optional, tag = "3")]
    pub objects: Option<proto::Objects>,
    #[prost(message, repeated, tag = "7")]
    pub xforms: Vec<proto::XformEntry>,
}

/// The wire session as `proto::Session`, whole or lean: a display-only document skips the
/// tree and the graph on the wire, not just in the kernel object.
fn decode_wire(bytes: &[u8], display_only: bool) -> Option<proto::Session> {
    if !display_only {
        return proto::Session::decode(bytes).ok();
    }
    let lean = LeanSession::decode(bytes).ok()?;

    Some(proto::Session { name: lean.name, guid: lean.guid, objects: lean.objects, tree: None, graph: None, bvh_boxes: Vec::new(), xforms: lean.xforms })
}

/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
/// fast), then convert objects CHUNK at a time. Same result, no multi-second freeze. `.json`
/// files stay on the synchronous path (they are small). The bytes are taken BY VALUE and dropped
/// the moment prost is done: the file, the proto and the kernel objects never coexist.
pub async fn session_from_bytes_chunked(url: &str, bytes: Vec<u8>, display_only: bool) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(&bytes));
    }
    let Some(p) = decode_wire(&bytes, display_only) else { return Session::default() };
    drop(bytes);
    let mut s = Session::new(&p.name);
    s.set_guid(p.guid.clone());

    let mut n = 0usize;
    // The same conversion loop for all 11 types, written once: proto -> object, stored, paused
    // every CHUNK so the browser can paint.
    macro_rules! chunk {
        ($vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x));
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
        // from_proto -> Result for the nested types; a bad object is skipped, not fatal
        (fallible $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let Ok(v) = $ty::from_proto(x) else { continue };
                let g = Rc::new(v);
                s.lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(&g)));
                s.objects.$slot.push(g);
                n += 1;
                if n % CHUNK == 0 { next_tick().await; }
            }
        };
    }

    if let Some(o) = p.objects {
        s.objects.set_guid(o.guid);
        s.objects.name = o.name;
        chunk!(o.points, Point, Point, points);
        chunk!(o.lines, Line, Line, lines);
        chunk!(o.planes, Plane, Plane, planes);
        chunk!(fallible o.bboxes, OBB, OBB, bboxes);
        chunk!(o.polylines, Polyline, Polyline, polylines);
        chunk!(o.pointclouds, PointCloud, PointCloud, pointclouds);
        chunk!(o.meshes, Mesh, Mesh, meshes);
        chunk!(o.nurbscurves, NurbsCurve, NurbsCurve, nurbscurves);
        chunk!(fallible o.nurbssurfaces, NurbsSurface, NurbsSurface, nurbssurfaces);
        chunk!(fallible o.breps, BRep, BRep, breps);
        chunk!(fallible o.elements, Element, Element, elements);
    }

    // Xforms first: they decide whether the tree is needed at all.
    for entry in &p.xforms {
        if let Some(xf) = &entry.xform {
            let mut xform = Xform::identity();
            xform.set_guid(xf.guid.clone());
            xform.name = xf.name.clone();
            for (i, val) in xf.matrix.iter().enumerate().take(16) {
                xform.m[i] = *val;
            }
            s.xforms.insert(entry.guid.clone(), xform);
        }
    }

    // The graph is real session data, not scratch: it was being decoded and dropped.
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

    // The tree comes from the same decode as everything else. It used to be skipped and then
    // re-decoded by a second mirror struct; a Session that loads its own tree is both simpler
    // and honest about what it holds.
    if let Some(tp) = &p.tree {
        s.tree = Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            /// One proto node and its children, recursively.
            fn build(proto: &proto::TreeNode) -> Rc<std::cell::RefCell<TreeNode>>{
                let node = TreeNode::new(&proto.name);
                for c in &proto.children {
                    let child = build(c);
                    node.borrow_mut().add(&child);
                }
                node
            }
            let root = build(rp);
            s.tree.add(&root, None);
        }
    }

    s
}
