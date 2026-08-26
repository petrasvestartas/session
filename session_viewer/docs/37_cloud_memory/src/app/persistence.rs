// Session loading
// WASM32 has no filesystem, so the fetch API is the only way to reach .pb or .json files.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use session_rust::Session;

/// A request already IN FLIGHT: the browser's fetch() promise is eager, only the Rust await is
/// lazy - so starting the next file's fetch before parsing the current one overlaps network
/// with parse (State::new pipelines with a window of 2).
pub struct Fetch { fut: JsFuture }

pub fn fetch_start(url: &str) -> Result<Fetch, JsValue>{
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

pub async fn fetch_finish(f: Fetch) -> Result<Vec<u8>, JsValue>{
    let resp: Response = f.fut.await?.dyn_into()?;
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET 'url' - trunk-served, same origin as the page and return raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue>{
    fetch_finish(fetch_start(url)?).await
}

// ── chunked parsing: convert the decoded proto in slices, yielding between them ──

use std::rc::Rc;
use prost::Message;
use session_rust::proto;
use session_rust::{Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, Polyline, PointCloud, BRep, Element, Xform};
use session_rust::tree::{Tree, TreeNode};

/// Objects converted per slice before the loader hands the browser one macrotask — the whole
/// point is that a frame can render BETWEEN slices, so a 250k-object parse stops freezing the UI.
const CHUNK: usize = 25_000;

/// One macrotask (setTimeout 0). A microtask (Promise.resolve) would NOT let the browser paint.
async fn next_tick() {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .unwrap();
    });
    let _ = JsFuture::from(p).await;
}

/// Wire-identical mirror of `proto::Mesh` with ONE field left out: `halfedges` (tag 5).
///
/// `Mesh::from_proto` discards that map - topology is rebuilt from faces on the first edit - but
/// prost still decoded it into a nested `HashMap<u64, HashMap<u64, ..>>` first: 208k entries on
/// the bunny, allocated and dropped. An unlisted tag is skipped with a length jump instead.
/// Every other field keeps `proto::Mesh`'s exact type, so `into_proto` below MOVES them - no
/// copy, no second hash, and the kernel's own `from_proto` stays the single source of truth for
/// what a mesh means.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanMesh {
    #[prost(string, tag = "1")]
    pub guid: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(map = "uint64, message", tag = "3")]
    pub vertices: std::collections::HashMap<u64, proto::VertexData>,
    #[prost(map = "uint64, message", tag = "4")]
    pub faces: std::collections::HashMap<u64, proto::FaceData>,
    // tag 5 (halfedges) intentionally absent - see the doc comment.
    #[prost(message, repeated, tag = "6")]
    pub edge_data: Vec<proto::EdgeData>,
    #[prost(btree_map = "string, double", tag = "7")]
    pub default_vertex_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "8")]
    pub default_face_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(btree_map = "string, double", tag = "9")]
    pub default_edge_attributes: std::collections::BTreeMap<String, f64>,
    #[prost(message, repeated, tag = "10")]
    pub pointcolors: Vec<proto::Color>,
    #[prost(message, repeated, tag = "11")]
    pub facecolors: Vec<proto::Color>,
    #[prost(message, repeated, tag = "12")]
    pub linecolors: Vec<proto::Color>,
    #[prost(double, repeated, tag = "13")]
    pub widths: Vec<f64>,
    #[prost(message, optional, tag = "15")]
    pub objectcolor: Option<proto::Color>,
    #[prost(int32, tag = "16")]
    pub color_mode: i32,
    #[prost(map = "uint64, message", tag = "17")]
    pub triangulation: std::collections::HashMap<u64, proto::TriList>,
}

impl LeanMesh {
    /// Hand the decoded fields to the kernel unchanged. `halfedges` is the empty map the kernel
    /// would have ignored anyway.
    pub fn into_proto_pub(self) -> proto::Mesh { self.into_proto() }

    fn into_proto(self) -> proto::Mesh {
        proto::Mesh {
            guid: self.guid,
            name: self.name,
            vertices: self.vertices,
            faces: self.faces,
            halfedges: Default::default(),
            edge_data: self.edge_data,
            default_vertex_attributes: self.default_vertex_attributes,
            default_face_attributes: self.default_face_attributes,
            default_edge_attributes: self.default_edge_attributes,
            pointcolors: self.pointcolors,
            facecolors: self.facecolors,
            linecolors: self.linecolors,
            widths: self.widths,
            objectcolor: self.objectcolor,
            color_mode: self.color_mode,
            triangulation: self.triangulation,
        }
    }
}

/// `proto::Objects` with the mesh lane swapped for [`LeanMesh`]; every other lane keeps the
/// generated type, so nothing else about the decode changes.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanObjects {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, repeated, tag = "3")]
    pub points: Vec<proto::Point>,
    #[prost(message, repeated, tag = "4")]
    pub lines: Vec<proto::Line>,
    #[prost(message, repeated, tag = "5")]
    pub planes: Vec<proto::Plane>,
    #[prost(message, repeated, tag = "6")]
    pub bboxes: Vec<proto::BoundingBox>,
    #[prost(message, repeated, tag = "7")]
    pub polylines: Vec<proto::Polyline>,
    #[prost(message, repeated, tag = "8")]
    pub pointclouds: Vec<proto::PointCloud>,
    #[prost(message, repeated, tag = "9")]
    pub meshes: Vec<LeanMesh>,
    #[prost(message, repeated, tag = "12")]
    pub nurbscurves: Vec<proto::NurbsCurve>,
    #[prost(message, repeated, tag = "13")]
    pub nurbssurfaces: Vec<proto::NurbsSurface>,
    #[prost(message, repeated, tag = "14")]
    pub breps: Vec<proto::BRep>,
    #[prost(message, repeated, tag = "15")]
    pub elements: Vec<proto::Element>,
}

/// The `Session` fields the viewer actually READS - same wire tags as `proto::Session`, so this
/// decodes the same bytes, but prost skips an unlisted field with a cheap length-delimited jump
/// instead of allocating it.
///
/// `tree` (tag 4) and `graph` (tag 5) are 21.7 MB of the 52 MB Treppenhaus sheet - 42% of the
/// file - and NOTHING in the viewer reads either one: the walk orders objects by
/// `Session::order()`, which is built from the object vectors, and `world_xforms()` consults the
/// tree only when `xforms` is non-empty. `TreeOnly` below covers exactly that case, and skipping
/// `objects` in turn makes it cheap.
/// Same shape, public, so the native `bench_load` harness can time this decode against the full
/// one. The loader below uses the private alias.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanSessionProbe {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, optional, tag = "3")]
    pub objects: Option<LeanObjects>,
    #[prost(message, repeated, tag = "7")]
    pub xforms: Vec<proto::XformEntry>,
}

/// Second pass for the rare file that carries local transforms: the tree, with the 30 MB of
/// objects skipped rather than decoded twice.
#[derive(Clone, PartialEq, prost::Message)]
pub struct TreeOnlyProbe {
    #[prost(message, optional, tag = "4")]
    pub tree: Option<proto::Tree>,
}

/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
/// fast), then convert objects CHUNK at a time. Same result, no multi-second freeze. `.json`
/// files stay on the synchronous path (they are small).
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = LeanSessionProbe::decode(bytes) else { return Session::default() };
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
        // the mesh lane arrives as LeanMesh (halfedges skipped); the kernel's from_proto still
        // does the building
        (lean $vec:expr, $ty:ident, $variant:ident, $slot:ident) => {
            for x in $vec {
                let g = Rc::new($ty::from_proto(x.into_proto()));
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
        chunk!(lean o.meshes, Mesh, Mesh, meshes);
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

    // The tree is rebuilt ONLY to compose those transforms down the hierarchy - see
    // `Session::world_xforms`, which returns an empty map on the same test. A flat sheet or a
    // mesh file lands here with nothing to compose and pays neither the decode nor the 90k
    // Rc<RefCell<TreeNode>> allocations.
    if s.xforms.is_empty() {
        return s;
    }
    let p = match TreeOnlyProbe::decode(bytes) { Ok(t) => t, Err(_) => return s };
    if let Some(tp) = &p.tree{
        s.tree = Tree::new(&tp.name);
        s.tree.set_guid(tp.guid.clone());
        if let Some(rp) = &tp.root {
            
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