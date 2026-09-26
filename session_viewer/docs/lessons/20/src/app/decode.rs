// --8<-- [start:decode-limits]
// A .pb file is protobuf: a run of fields, each a varint key (field number × 8 + wire type) then a value; wire type 2 = a length, then that many bytes.
use super::validate;
use super::validate::varint;
use prost::Message;
use session_rust::proto;
use session_rust::tree::{Tree, TreeNode};
use session_rust::{
    BRep, Element, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, OBB, Plane, Point, PointCloud,
    Polyline, Session, Xform,
};
use session_rust::{InstanceRef, Objects};
use std::cell::RefCell;
use std::rc::Rc;

// `next_tick` exists twice under opposite cfgs, so the decoder below calls it the same way in the browser and in native tests.
#[cfg(target_arch = "wasm32")]
use super::fetch::next_tick;

/// Nothing to wait for outside the browser.
#[cfg(not(target_arch = "wasm32"))]
async fn next_tick() {}

// The page redraws and answers input only between tasks, so a long decode pauses every 25,000 objects.
/// Objects converted before yielding to the browser.
const CHUNK: usize = 25_000;

/// Bytes of a JS body copied into wasm memory at once.
#[cfg(target_arch = "wasm32")]
const WINDOW: u64 = 1 << 20;

/// `Objects` field numbers that hold geometry the viewer reads.
const OBJECT_FIELDS: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 12, 13, 14, 15];

/// Largest file read whole.
const MAX_FILE: u64 = 512 * 1024 * 1024;

/// Why a file that ends inside a field is refused.
const TRUNCATED: &str = "invalid session protobuf: the file ends inside a field";

/// Why a file with a broken field header is refused.
const MALFORMED: &str = "invalid session protobuf: malformed field";
// --8<-- [end:decode-limits]

// --8<-- [start:decode-body]
/// A session file; a JS buffer stays outside wasm memory and is read a window at a time.
pub enum Body {
    Bytes(Vec<u8>), // bytes already in wasm memory
    #[cfg(target_arch = "wasm32")]
    Js(js_sys::Uint8Array), // a fetched body, left in JS
}

impl Body {
    /// Bytes in the file.
    pub fn size(&self) -> u64 {
        match self {
            Body::Bytes(bytes) => bytes.len() as u64,
            #[cfg(target_arch = "wasm32")]
            Body::Js(array) => u64::from(array.length()),
        }
    }
}
// --8<-- [end:decode-body]

// --8<-- [start:decode-reader]
/// One protobuf field: number, wire type and where its value lies.
#[derive(Clone, Copy)]
struct Field {
    number: u64, // field number
    wire: u64,   // wire type
    at: u64,     // first byte of a length-delimited value
    len: u64,    // its length; 0 for other wire types
    next: u64,   // first byte after the field
}

/// A body read through one bounded window.
// `cfg_attr` adds an attribute only under a condition: natively `at` and `window` go unused, so that warning is silenced there.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct Reader<'a> {
    body: &'a Body,  // the file
    size: u64,       // its length
    at: u64,         // body offset of the window's first byte
    window: Vec<u8>, // bytes of a JS body
}

impl<'a> Reader<'a> {
    /// A reader at the start of `body`.
    fn new(body: &'a Body) -> Self {
        Self {
            body,
            size: body.size(),
            at: 0,
            window: Vec::new(),
        }
    }

    /// Bytes `[at, at + n)` of the body.
    fn bytes(&mut self, at: u64, n: u64) -> Result<&[u8], String> {
        let end = match at.checked_add(n) {
            Some(end) if end <= self.size => end,
            _ => return Err(TRUNCATED.into()),
        };

        match self.body {
            Body::Bytes(bytes) => Ok(&bytes[at as usize..end as usize]),
            #[cfg(target_arch = "wasm32")]
            Body::Js(array) => {
                let held = self.at + self.window.len() as u64;

                if at < self.at || end > held { // not in the window: copy up to 1 MiB from the JS array
                    let to = end.max(at + WINDOW).min(self.size);
                    self.window.resize((to - at) as usize, 0);
                    array
                        .subarray(at as u32, to as u32)
                        .copy_to(&mut self.window);
                    self.at = at;
                }

                let from = (at - self.at) as usize;
                Ok(&self.window[from..from + n as usize])
            }
        }
    }

    /// The field starting at `at` inside a message ending at `end`.
    fn field(&mut self, at: u64, end: u64) -> Result<Field, String> {
        let head = self.bytes(at, (end - at).min(20))?; // a key and a length take at most 10 bytes each
        let (key, used) = varint(head, 0).ok_or(MALFORMED)?;
        let (number, wire) = (key >> 3, key & 7);
        let mut value = used;
        let mut len = 0;

        // wire types: 0 = varint, 1 = eight bytes, 2 = length then bytes, 5 = four bytes
        match wire {
            0 => value += varint(head, value).ok_or(MALFORMED)?.1,
            1 => value += 8,
            2 => {
                let (length, size) = varint(head, value).ok_or(MALFORMED)?;
                value += size;
                len = length;
            }
            5 => value += 4,
            _ => return Err(MALFORMED.into()),
        }

        let at = at + value as u64;
        let next = at.checked_add(len).ok_or(TRUNCATED)?;

        if number == 0 || next > end {
            return Err(TRUNCATED.into());
        }

        Ok(Field {
            number,
            wire,
            at,
            len,
            next,
        })
    }

    /// A length-delimited field's value as text.
    fn string(&mut self, f: Field) -> Result<String, String> {
        expect(f)?;
        let bytes = self.bytes(f.at, f.len)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| "invalid session protobuf: text is not UTF-8".into())
    }

    // `M: Message + Default`: any prost message type; the caller names it, e.g. `r.message::<proto::Vertex>(field)`
    /// A length-delimited field's value as one message; a window grown past its size for a large
    /// object is freed at once. Decoding from a slice alone keeps one copy of prost's code.
    fn message<M: Message + Default>(&mut self, f: Field) -> Result<M, String> {
        expect(f)?;
        let decoded = M::decode(self.bytes(f.at, f.len)?).map_err(invalid);

        #[cfg(target_arch = "wasm32")]
        if self.window.len() as u64 > WINDOW {
            self.window = Vec::new();
        }

        decoded
    }
}

/// A length-delimited field, or an error.
fn expect(f: Field) -> Result<(), String> {
    if f.wire == 2 {
        Ok(())
    } else {
        Err(MALFORMED.into())
    }
}

/// A prost error as a message.
fn invalid(error: prost::DecodeError) -> String {
    format!("invalid session protobuf: {error}")
}
// --8<-- [end:decode-reader]

// --8<-- [start:decode-object]
/// Counts conversions.
struct Pacer {
    n: usize, // objects converted so far
}

impl Pacer {
    /// Count one; true every `CHUNK`.
    fn tick(&mut self) -> bool {
        self.n += 1;
        self.n.is_multiple_of(CHUNK)
    }
}

// `$proto:ty` takes a type, `$check:path` a function, `$slot:ident` a field name; the body is pasted once per object kind below.
/// Convert one object record: decode, check, add to the session.
macro_rules! object {
    ($s:expr, $r:expr, $f:expr, $proto:ty, $check:path, $ty:ident, $slot:ident) => {{
        let source: $proto = $r.message($f)?;
        $check(&source)?;
        let g = Rc::new($ty::from_proto(source));
        $s.lookup
            .insert(g.guid().to_string(), Geometry::$ty(Rc::clone(&g)));
        $s.objects.$slot.push(g);
    }};
    // a second form, chosen by the word `fallible`, for kinds whose `from_proto` can fail
    (fallible $s:expr, $r:expr, $f:expr, $proto:ty, $check:path, $ty:ident, $slot:ident) => {{
        let source: $proto = $r.message($f)?;
        $check(&source)?;
        let g = match $ty::from_proto(source) {
            Ok(value) => Rc::new(value),
            Err(error) => return Err(format!("invalid {}: {error}", stringify!($ty))),
        };
        $s.lookup
            .insert(g.guid().to_string(), Geometry::$ty(Rc::clone(&g)));
        $s.objects.$slot.push(g);
    }};
}
// --8<-- [end:decode-object]

// --8<-- [start:decode-session]
/// A session from file bytes, yielding while converting.
pub async fn session_from_bytes(url: &str, bytes: Vec<u8>) -> Result<Session, String> {
    session_from_body(url, Body::Bytes(bytes), true).await
}

/// A session read one object at a time, never as one message tree; without `vertices` the graph
/// keeps only the vertices its edges name, enough for a document that is never saved.
pub async fn session_from_body(url: &str, body: Body, vertices: bool) -> Result<Session, String> {
    if body.size() > MAX_FILE {
        return Err(
            "geometry payload exceeds the 512 MiB whole-file limit; use cloud streaming"
                .to_string(),
        );
    }

    if url.ends_with(".json") {
        return json(body);
    }

    let mut r = Reader::new(&body);
    let (mut name, mut guid) = (String::new(), String::new());
    let mut objects = Vec::new(); // Objects messages
    let mut xforms = Vec::new(); // XformEntry messages
    let (mut tree, mut graph) = (None, None);
    let mut definitions = None; // the Objects message instances place
    let mut folds = Vec::new(); // (instance guid, stored placement), folded into xforms
    let mut at = 0;

    // first pass: only note where each top-level field lies; the objects are read one at a time below
    while at < r.size {
        let f = r.field(at, r.size)?;

        match f.number {
            1 => name = r.string(f)?,
            2 => guid = r.string(f)?,
            3 => objects.push(f),
            4 => tree = Some(f),
            5 => graph = Some(f),
            7 => xforms.push(f),
            8 => definitions = Some(f),
            _ => {}
        }

        at = f.next;
    }

    let mut s = Session::new(&name);
    s.set_guid(guid);
    let counts = count(&mut r, &objects)?;
    reserve(&mut s, &counts);
    let mut pacer = Pacer { n: 0 };

    for &list in &objects {
        let (mut objects_name, mut objects_guid) = (String::new(), String::new());
        let mut at = list.at;

        while at < list.next {
            let f = r.field(at, list.next)?;
            at = f.next;

            match f.number {
                1 => objects_name = r.string(f)?,
                2 => objects_guid = r.string(f)?,
                3 => object!(s, r, f, proto::Point, validate::point, Point, points),
                4 => object!(s, r, f, proto::Line, validate::line, Line, lines),
                5 => object!(s, r, f, proto::Plane, validate::any, Plane, planes),
                6 => object!(fallible s, r, f, proto::BoundingBox, validate::any, OBB, bboxes),
                7 => object!(
                    s,
                    r,
                    f,
                    proto::Polyline,
                    validate::polyline,
                    Polyline,
                    polylines
                ),
                8 => object!(
                    s,
                    r,
                    f,
                    proto::PointCloud,
                    validate::cloud,
                    PointCloud,
                    pointclouds
                ),
                9 => object!(s, r, f, proto::Mesh, validate::mesh, Mesh, meshes),
                12 => object!(
                    s,
                    r,
                    f,
                    proto::NurbsCurve,
                    validate::curve,
                    NurbsCurve,
                    nurbscurves
                ),
                13 => {
                    object!(fallible s, r, f, proto::NurbsSurface, validate::surface, NurbsSurface, nurbssurfaces)
                }
                14 => object!(fallible s, r, f, proto::BRep, validate::brep, BRep, breps),
                15 => {
                    object!(fallible s, r, f, proto::Element, validate::element, Element, elements)
                }
                18 => instance(&mut s, &mut r, f, &mut folds)?,
                _ => continue,
            }

            if pacer.tick() {
                next_tick().await; // give the browser a turn
            }
        }

        s.objects.set_guid(objects_guid);
        s.objects.name = objects_name;
    }

    for &f in &xforms {
        let entry: proto::XformEntry = r.message(f)?;
        validate::xform(&entry)?;
        let Some(xf) = &entry.xform else { continue };
        let mut xform = Xform::identity();
        xform.set_guid(xf.guid.clone());
        xform.name = xf.name.clone();

        for (i, val) in xf.matrix.iter().enumerate().take(16) {
            xform.m[i] = *val;
        }

        s.xforms.insert(entry.guid, xform);
    }

    fold(&mut s, folds);

    if let Some(f) = definitions {
        define(&mut s, &mut r, f)?;
    }

    if let Some(f) = graph {
        expect(f)?;
        s.graph = read_graph(&mut r, f, vertices)?;
    }

    if let Some(f) = tree {
        expect(f)?;
        s.tree = read_tree(&mut r, f)?;
    }

    // the tables were filled by hand above; `reindex` rebuilds the kernel's guid → slot and guid → tree-node indexes, or delete and undo by guid would miss these objects
    s.reindex();

    Ok(s)
}
// --8<-- [end:decode-session]

// --8<-- [start:decode-count]
/// Records per `Objects` field number, checked against the scene cap.
fn count(r: &mut Reader, objects: &[Field]) -> Result<[usize; 16], String> {
    let mut counts = [0usize; 16];

    for &list in objects {
        expect(list)?;
        let mut at = list.at;

        while at < list.next {
            let f = r.field(at, list.next)?;

            if let Some(n) = counts.get_mut(f.number as usize) {
                *n += 1;
            }

            // instances, field 18, are past the table: counted in slot 0, which no field uses
            if f.number == 18 {
                counts[0] += 1;
            }

            at = f.next;
        }
    }

    // planes and boxes do not count, as before
    let capped: usize = OBJECT_FIELDS
        .iter()
        .filter(|field| !matches!(field, 5 | 6))
        .map(|field| counts[*field])
        .sum();

    if capped > validate::MAX_OBJECTS {
        return Err(validate::TOO_MANY.into());
    }

    if capped + counts[0] > validate::MAX_OBJECTS {
        return Err(validate::TOO_MANY.into());
    }

    Ok(counts)
}

/// Room for every object, so no table grows by doubling while both copies are alive.
fn reserve(s: &mut Session, counts: &[usize; 16]) {
    let o = &mut s.objects;
    o.points.reserve_exact(counts[3]);
    o.lines.reserve_exact(counts[4]);
    o.planes.reserve_exact(counts[5]);
    o.bboxes.reserve_exact(counts[6]);
    o.polylines.reserve_exact(counts[7]);
    o.pointclouds.reserve_exact(counts[8]);
    o.meshes.reserve_exact(counts[9]);
    o.nurbscurves.reserve_exact(counts[12]);
    o.nurbssurfaces.reserve_exact(counts[13]);
    o.breps.reserve_exact(counts[14]);
    o.elements.reserve_exact(counts[15]);
    s.lookup
        .reserve(OBJECT_FIELDS.iter().map(|field| counts[*field]).sum());
    s.objects.instances.reserve_exact(counts[0]);
    s.instance_lookup.reserve(counts[0]);
}
// --8<-- [end:decode-count]

// --8<-- [start:decode-graph]
/// The graph message `f`; without `vertices` only the edges and the vertices they name.
fn read_graph(r: &mut Reader, f: Field, vertices: bool) -> Result<session_rust::Graph, String> {
    let (mut name, mut guid) = (String::new(), String::new());
    let mut at = f.at;

    // the name first: the graph is made with it
    while at < f.next {
        let field = r.field(at, f.next)?;

        match field.number {
            1 => name = r.string(field)?,
            2 => guid = r.string(field)?,
            _ => {}
        }

        at = field.next;
    }

    let mut graph = session_rust::Graph::new(&name);
    graph.set_guid(guid);
    at = f.at;

    while at < f.next {
        let field = r.field(at, f.next)?;
        at = field.next;

        match field.number {
            3 if vertices => {
                let entry = read_vertex(r, field)?;
                graph.add_node(&entry.0, &entry.1);
            }
            4 => {
                let edge: proto::Edge = r.message(field)?;
                graph.add_edge(&edge.v0, &edge.v1, &edge.attribute);
            }
            _ => {}
        }
    }

    Ok(graph)
}

/// One vertex map entry: its key and its attribute text.
fn read_vertex(r: &mut Reader, f: Field) -> Result<(String, String), String> {
    expect(f)?;
    let (mut key, mut attribute) = (String::new(), String::new());
    let mut at = f.at;

    while at < f.next {
        let field = r.field(at, f.next)?;

        match field.number {
            1 => key = r.string(field)?,
            2 => attribute = r.message::<proto::Vertex>(field)?.attribute,
            _ => {}
        }

        at = field.next;
    }

    Ok((key, attribute))
}

/// The tree message `f`.
fn read_tree(r: &mut Reader, f: Field) -> Result<Tree, String> {
    let (mut name, mut guid, mut root) = (String::new(), String::new(), None);
    let mut at = f.at;

    while at < f.next {
        let field = r.field(at, f.next)?;

        match field.number {
            1 => guid = r.string(field)?,
            2 => name = r.string(field)?,
            3 => root = Some(read_node(r, field)?),
            _ => {}
        }

        at = field.next;
    }

    let mut tree = Tree::new(&name);
    tree.set_guid(guid);

    if let Some(root) = root {
        tree.add(&root, None);
    }

    Ok(tree)
}

/// A tree node and its children, depth first in file order, 64 levels at most.
fn read_node(r: &mut Reader, f: Field) -> Result<Rc<RefCell<TreeNode>>, String> {
    expect(f)?;
    let root = TreeNode::new("");
    let mut stack = vec![(Rc::clone(&root), f.at, f.next)]; // (node, next field, end), one per level

    while let Some((node, at, end)) = stack.pop() {
        if at >= end {
            continue;
        }

        let field = r.field(at, end)?;
        stack.push((Rc::clone(&node), field.next, end));

        match field.number {
            2 => node.borrow_mut().name = r.string(field)?,
            4 => {
                expect(field)?;

                if stack.len() > validate::MAX_DEPTH {
                    return Err(validate::TOO_DEEP.into());
                }

                let child = TreeNode::new("");
                node.borrow_mut().add(&child);
                stack.push((child, field.at, field.next));
            }
            _ => {}
        }
    }

    Ok(root)
}
// --8<-- [end:decode-graph]

// --8<-- [start:decode-instances]
// An instance places a shared definition: one bolt stored once and drawn 400 times, each copy with its own placement.
/// One instance: added with its lookup; a stored placement waits in `folds` for the xforms.
fn instance(
    s: &mut Session,
    r: &mut Reader,
    f: Field,
    folds: &mut Vec<(String, Xform)>,
) -> Result<(), String> {
    let mut source: proto::InstanceRef = r.message(f)?;

    if let Some(xform) = source.xform.take() {
        let entry = proto::XformEntry {
            guid: source.guid.clone(),
            xform: Some(xform),
        };
        validate::xform(&entry)?;
        folds.push((
            entry.guid,
            Xform::from_proto(entry.xform.unwrap_or_default()),
        ));
    }

    let instance = Rc::new(InstanceRef::from_proto(source));
    s.instance_lookup
        .insert(instance.guid().to_string(), Rc::clone(&instance));
    s.objects.instances.push(instance);
    Ok(())
}

/// Stored instance placements moved into `xforms`, as the kernel loaders fold them.
fn fold(s: &mut Session, folds: Vec<(String, Xform)>) {
    for (guid, xform) in folds {
        if !xform.is_identity() && !guid.is_empty() {
            let folded = &s.xform(&guid) * &xform;
            s.xforms.insert(guid, folded);
        }
    }
}

/// The definitions message `f`, checked, with its lookup.
fn define(s: &mut Session, r: &mut Reader, f: Field) -> Result<(), String> {
    let source: proto::Objects = r.message(f)?;
    validate::definitions(&source, s.objects.instances.len())?;
    s.definitions = Objects::from_proto(source).map_err(|e| format!("invalid definitions: {e}"))?;
    let d = &s.definitions;
    let lookup = &mut s.definition_lookup;
    macro_rules! index {
        ($($list:ident => $variant:ident),*) => {
            $(for g in &d.$list {
                lookup.insert(g.guid().to_string(), Geometry::$variant(Rc::clone(g)));
            })*
        };
    }

    index!(
        points => Point,
        lines => Line,
        planes => Plane,
        bboxes => OBB,
        polylines => Polyline,
        pointclouds => PointCloud,
        meshes => Mesh,
        nurbscurves => NurbsCurve,
        nurbssurfaces => NurbsSurface,
        breps => BRep,
        elements => Element
    );
    Ok(())
}
// --8<-- [end:decode-instances]

// --8<-- [start:decode-json]
/// A session from JSON text.
#[cfg(feature = "json-sessions")]
fn json(body: Body) -> Result<Session, String> {
    let bytes = match body {
        Body::Bytes(bytes) => bytes,
        #[cfg(target_arch = "wasm32")]
        Body::Js(array) => array.to_vec(),
    };
    let text = match std::str::from_utf8(&bytes) {
        Ok(text) => text,
        Err(error) => return Err(format!("invalid JSON encoding: {error}")),
    };
    validate::json(text)?;
    let session = match Session::jsonload(text) {
        Ok(session) => session,
        Err(error) => return Err(format!("invalid session JSON: {error}")),
    };
    validate::retained(&session)?;
    Ok(session)
}

/// JSON sessions are left out of this build.
#[cfg(not(feature = "json-sessions"))]
fn json(_body: Body) -> Result<Session, String> {
    Err("JSON sessions are not read by this build (feature json-sessions); publish the .pb".into())
}
// --8<-- [end:decode-json]

// --8<-- [start:decode-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Vector;
    use session_rust::element::ElementFeature;

    /// A point.
    fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// One of each kind, nested groups, placements, an edge and a vertex no edge names.
    fn source() -> Session {
        let mut s = Session::new("mixed");
        let walls = s.add_group("walls");
        s.set_xform("walls", Xform::translation(0.0, 0.0, 3.0));
        let point = s.add_point(p(1.0, 2.0, 3.0), Some(&walls));
        let a = point.borrow().name.clone();
        let line = s.add_line(Line::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0), Some(&point));
        let b = line.borrow().name.clone();
        s.add_plane(
            Plane::new(
                p(0.0, 0.0, 0.0),
                Vector::new(1.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            ),
            None,
        );
        s.add_polyline(
            Polyline::new(vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0)]),
            Some(&walls),
        );
        let arc: Vec<Point> = (0..6).map(|i| p(i as f64, (i * i) as f64, 0.0)).collect();
        s.add_nurbscurve(NurbsCurve::create(false, 3, &arc), None);
        s.add_mesh(Mesh::create_box(2.0, 2.0, 2.0), Some(&walls));
        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(10.0, 10.0, 10.0));
        let axis = Polyline::new(vec![p(0.0, 0.0, 0.0), p(100.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        s.add_element(element, None);
        s.add_pointcloud(
            PointCloud::new(vec![p(0.0, 0.0, 0.0), p(1.0, 1.0, 1.0)], vec![], vec![]),
            None,
        );
        s.add_edge(&a, &b, "touches");
        s.graph.add_node("lonely", "kept");
        s
    }

    /// The decoded session, from bytes.
    fn decoded(bytes: Vec<u8>, vertices: bool) -> Result<Session, String> {
        pollster::block_on(session_from_body("a.pb", Body::Bytes(bytes), vertices))
    }

    /// Every object's own record, per kind, in order; maps compare as maps.
    fn same_objects(a: &Session, b: &Session) {
        macro_rules! slot {
            ($slot:ident) => {
                let x: Vec<_> = a.objects.$slot.iter().map(|g| g.to_proto()).collect();
                let y: Vec<_> = b.objects.$slot.iter().map(|g| g.to_proto()).collect();
                assert_eq!(x, y, stringify!($slot));
            };
        }

        slot!(points);
        slot!(lines);
        slot!(planes);
        slot!(polylines);
        slot!(pointclouds);
        slot!(meshes);
        slot!(nurbscurves);
        slot!(nurbssurfaces);
        assert_eq!(a.objects.elements.len(), b.objects.elements.len());

        for (x, y) in a.objects.elements.iter().zip(&b.objects.elements) {
            let (x, y) = (x.to_proto(), y.to_proto());
            assert_eq!((&x.guid, &x.name), (&y.guid, &y.name));
            let mesh = |data: &[u8]| proto::Mesh::decode(data).unwrap();
            assert_eq!(mesh(&x.geometry_data), mesh(&y.geometry_data));
        }
    }

    /// The tree as (depth, name), depth first.
    fn outline(s: &Session) -> Vec<(usize, String)> {
        let mut out = Vec::new();
        let mut stack: Vec<_> = s.tree.root().into_iter().map(|n| (n, 0)).collect();

        while let Some((node, depth)) = stack.pop() {
            out.push((depth, node.borrow().name.clone()));

            for child in node.borrow().children().into_iter().rev() {
                stack.push((child, depth + 1));
            }
        }

        out
    }

    /// Objects, tree, placements and graph come back as written.
    #[test]
    fn window_decode_matches_the_source() {
        let mut s = source();
        s.guid();
        let bytes = s.pb_dumps();
        let d = decoded(bytes, true).unwrap();
        assert_eq!(d.name, s.name);
        assert_eq!(d.guid(), s.guid());
        assert_eq!(d.lookup.len(), s.lookup.len());
        same_objects(&d, &s);
        assert_eq!(outline(&d), outline(&s));
        assert_eq!(d.xforms.len(), s.xforms.len());

        for (guid, xform) in &s.xforms {
            assert_eq!(d.xforms[guid].m, xform.m);
        }

        assert_eq!(d.graph.edges.len(), s.graph.edges.len());
        assert!(d.graph.has_node("lonely"));
        assert_eq!(d.world_xforms().len(), s.world_xforms().len());
    }

    /// A display document keeps the vertices its edges name, not the others.
    #[test]
    fn display_documents_skip_isolated_vertices() {
        let mut s = source();
        let d = decoded(s.pb_dumps(), false).unwrap();
        assert!(!d.graph.has_node("lonely"));
        assert_eq!(d.graph.edges.len(), s.graph.edges.len());
        same_objects(&d, &s);
    }

    /// Cut, garbled or hostile files are refused, never half read.
    #[test]
    fn broken_files_are_refused() {
        let bytes = source().pb_dumps();

        for cut in [1, bytes.len() / 3, bytes.len() / 2, bytes.len() - 1] {
            assert!(
                decoded(bytes[..cut].to_vec(), true).is_err(),
                "cut at {cut}"
            );
        }

        assert!(
            decoded(
                vec![
                    0x1a, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
                ],
                true
            )
            .is_err()
        );
        assert!(decoded(vec![0x0f], true).is_err(), "wire type 7");
        assert!(decoded(vec![0x23, 0x00], true).is_err(), "a group");

        let mut s = Session::new("bad");
        s.add_point(p(f64::NAN, 0.0, 0.0), None);
        assert!(decoded(s.pb_dumps(), true).is_err());

        let mut s = Session::new("deep");
        let mut parent = s.add_group("0");

        for level in 1..70 {
            let node = TreeNode::new(&level.to_string());
            s.add(&node, Some(&parent));
            parent = node;
        }

        assert_eq!(
            decoded(s.pb_dumps(), true).err().as_deref(),
            Some(validate::TOO_DEEP)
        );
    }

    /// A box defined once and placed three times, the last under a moved group.
    fn placed() -> Session {
        let mut s = Session::new("placed");
        let definition = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(1.0, 1.0, 1.0))));
        let group = s.add_group("row");
        s.set_xform("row", Xform::translation(0.0, 10.0, 0.0));

        for i in 0..3 {
            let name = format!("box_{i}");
            let place = Xform::translation(i as f64 * 3.0, 0.0, 0.0);
            let instance =
                session_rust::InstanceRef::with_name(&name, &definition, Xform::identity());
            s.add_instance(instance, place, (i == 2).then_some(&group));
        }

        s
    }

    /// Definitions and instances come back with their lookups and world placements.
    #[test]
    fn instances_survive_the_window_decode() {
        let mut source = placed();
        let d = decoded(source.pb_dumps(), true).unwrap();
        assert_eq!(d.objects.instances.len(), 3);
        assert_eq!(d.instance_lookup.len(), 3);
        assert_eq!(d.definitions.meshes.len(), 1);
        assert_eq!(d.definition_lookup.len(), 1);

        for instance in &source.objects.instances {
            let guid = instance.guid();
            assert_eq!(d.world_xform(guid).m, source.world_xform(guid).m, "{guid}");
            assert!(d.definition_of(guid).is_some());
        }

        let mut bytes = source.pb_dumps();
        // a definitions field whose length runs past the end
        bytes.extend_from_slice(&[0x42, 0x7f, 0x00]);
        assert!(decoded(bytes, true).is_err());
    }

    /// The same session the whole-message path read, for every local file.
    #[test]
    #[ignore = "reads the local scene files"]
    fn local_files_match_the_kernel_reader() {
        for name in ["boxes", "bunny", "sheet_querschnitt"] {
            let bytes = std::fs::read(format!("assets/pb/view_local_{name}.pb")).unwrap();
            let whole = Session::pb_loads(&bytes).unwrap();
            let d = decoded(bytes, true).unwrap();
            assert_eq!(d.lookup.len(), whole.lookup.len(), "{name}");
            same_objects(&d, &whole);
            assert_eq!(outline(&d), outline(&whole), "{name}");
        }
    }
}
// --8<-- [end:decode-tests]
