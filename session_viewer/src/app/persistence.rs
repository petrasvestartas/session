// Session loading
// WASM32 has no filesystem, so the fetch API is the only way to reach .pb or .json files.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};
use session_rust::Session;

/// A request already IN FLIGHT: the browser's fetch() promise is eager, only the Rust await is
/// lazy - so starting the next file's fetch before parsing the current one overlaps network
/// with parse (State::new pipelines with a window of 2).
pub struct Fetch { fut: JsFuture }

pub fn fetch_start(url: &str) -> Result<Fetch, JsValue>{
    fetch_start_mode(url, RequestMode::Cors)
}

fn fetch_start_mode(url: &str, mode: RequestMode) -> Result<Fetch, JsValue>{
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(mode);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// What a cross-origin GET came back with. `bytes` is empty on a 304.
pub struct CorsReply { pub status: u16, pub etag: Option<String>, pub bytes: Vec<u8> }

/// GET a cross-origin `url` (GitHub API, raw.githubusercontent.com) past the browser's HTTP
/// cache (`no-store`: the API answers carry `max-age=60`, which would hide a moved branch for a
/// minute). `if_none_match` makes the request conditional. Any HTTP status is `Ok` - the caller
/// reads it; a network failure is `Err` with the browser's message.
pub async fn fetch_cors(url: &str, if_none_match: Option<&str>) -> Result<CorsReply, String>{
    let describe = |e: JsValue| e.as_string().unwrap_or_else(|| format!("{e:?}"));
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    opts.set_cache(web_sys::RequestCache::NoStore);
    if let Some(tag) = if_none_match {
        let headers = web_sys::Headers::new().map_err(describe)?;
        headers.set("If-None-Match", tag).map_err(describe)?;
        opts.set_headers(&headers);
    }
    let request = Request::new_with_str_and_init(url, &opts).map_err(describe)?;
    let window = web_sys::window().ok_or_else(|| "no window".to_string())?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|e| format!("network error: {}", describe(e)))?
        .dyn_into().map_err(describe)?;
    let etag = resp.headers().get("etag").ok().flatten();
    let buf = JsFuture::from(resp.array_buffer().map_err(describe)?).await.map_err(describe)?;
    Ok(CorsReply { status: resp.status(), etag, bytes: js_sys::Uint8Array::new(&buf).to_vec() })
}

/// `fetch_cors` for a file: a non-2xx status is an error carrying the status code, so the
/// caller can say what went wrong.
pub async fn fetch_bytes_cors(url: &str) -> Result<Vec<u8>, String>{
    let r = fetch_cors(url, None).await?;
    if !(200..300).contains(&r.status) {
        return Err(format!("HTTP {}", r.status));
    }
    Ok(r.bytes)
}

/// Resolve after `ms` milliseconds (setTimeout), yielding to the browser meanwhile.
pub async fn sleep_ms(ms: i32) {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let _ = JsFuture::from(p).await;
}

pub async fn fetch_finish(f: Fetch) -> Result<Vec<u8>, JsValue>{
    let resp: Response = f.fut.await?.dyn_into()?;
    // A 404 has a BODY - S3 answers one in XML, a web server in HTML - and handing those bytes
    // back as if they were the file makes the parser report the failure instead of the fetch:
    // a missing manifest came back as `TOML: invalid key (byte offset 0-1)`.
    if !(200..300).contains(&resp.status()) {
        return Err(JsValue::from_str(&format!("HTTP {} for {}", resp.status(), resp.url())));
    }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// GET 'url' - trunk-served, same origin as the page and return raw bytes.
pub async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue>{
    fetch_finish(fetch_start(url)?).await
}

/// Where the scene files live, as a prefix with a trailing slash. Empty means the page's own
/// origin - what `trunk serve` wants from a `dist/` it just built. A bucket base points every
/// manifest at that bucket instead, so a scene file goes on saying `pb/lion.pb` and never has to
/// learn a hostname: ONE place names the host, and moving the data is a one-line change.
pub const DATA_BASE: &str = "https://pub-dfd304db921140a09a9ad44c30e0aceb.r2.dev/";

/// `?data=` for this page load: another base, or `off` for the page's own origin. Anything else
/// is ignored with a warning and `DATA_BASE` is used - a query string is untrusted input, and
/// only https (or a localhost dev server) may name where geometry comes from.
pub fn data_base() -> String {
    let asked = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .and_then(|q| q.split(['?', '&']).find_map(|p| p.strip_prefix("data=").map(str::to_string)))
        .and_then(|v| js_sys::decode_uri_component(&v).ok())
        .and_then(|v| v.as_string());
    let base = match asked {
        None => DATA_BASE.to_string(),
        Some(v) if v == "off" || v.is_empty() => return String::new(),
        Some(v) if v.starts_with("https://") || v.starts_with("http://localhost") => v,
        Some(other) => {
            log::warn!("data: ignoring `?data={other}` - expected an https:// base, one on http://localhost, or `off`; using {DATA_BASE}");
            DATA_BASE.to_string()
        }
    };
    if base.ends_with('/') { base } else { base + "/" }
}

/// The URL a manifest entry actually resolves to. An entry that already names a host is used as
/// it stands; every other one hangs off `data_base()`, which with an empty base is byte for byte
/// the page-relative path the viewer has always fetched.
pub fn asset_url(file: &str) -> String {
    if file.starts_with("https://") || file.starts_with("http://") {
        return file.to_string();
    }
    format!("{}{}", data_base(), file.trim_start_matches("./"))
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
pub async fn next_tick() {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .unwrap();
    });
    let _ = JsFuture::from(p).await;
}

/// `Session::pb_loads`, unrolled with awaits: decode the proto whole (one short block — prost is
/// fast), then convert objects CHUNK at a time. Same result, no multi-second freeze. `.json`
/// files stay on the synchronous path (they are small).
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
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


// ── streaming a point cloud: HTTP Range in, GPU rows out, nothing large in between ──
//
// The whole-file path above peaks at raw bytes + decoded proto + kernel object + GPU rows.
// This one never holds more than one slice. It is possible because of two facts about the
// wire format, both checked against a real scan (assets/pb/lidar_scan000.pb):
//
//   Session.3 (Objects) -> Objects.8 (pointclouds) -> PointCloud.3 coords / .4 colors
//
//   - every hop is wire type 2, length-delimited, so the headers sit in the first ~170 bytes
//   - `coords` is packed DOUBLE, a fixed 8 bytes an element, so its length prefix gives the
//     exact point count BEFORE a byte of payload is read: 87,570,576 / 24 = 3,648,774
//
// Knowing the count up front is what removes every reallocation: all three GPU buffers are
// sized once, exactly, and each slice is written at a known offset.

/// Where the two packed arrays live in the file, as absolute byte offsets.
pub struct CloudFields {
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
}

/// One protobuf varint. Returns the value and how many bytes it ate.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;
    loop {
        let byte = *b.get(i)?;
        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;
        if byte & 0x80 == 0 { return Some((v, i - start)) }
        shift += 7;
        if shift > 63 { return None }
    }
}

/// Walk `head` (the first few KB of the file) down Session.3 -> Objects.8 -> PointCloud, and
/// report where `coords` starts. Descends into exactly the three fields it cares about and
/// skips every other one by its length - no allocation, no decoding.
///
/// Returns `None` for anything that is not a single-cloud file, which is the signal to fall
/// back to the whole-file prost path.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let mut i = 0usize;
    let mut end = head.len();
    for want in [3u32, 8u32] {
        let mut found = false;
        while i < end {
            let (tag, n) = varint(head, i)?;
            i += n;
            let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
            if wire != 2 { return None } // every hop we care about is length-delimited
            let (len, n) = varint(head, i)?;
            i += n;
            if field == want { end = i + len as usize; found = true; break }
            i += len as usize; // skip this sub-message whole
        }
        if !found { return None }
    }
    // inside PointCloud now: find field 3 (coords)
    while i < end {
        let (tag, n) = varint(head, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
        if wire != 2 {
            // point_size is a fixed64, everything else we skip by wire type
            i += match wire { 0 => varint(head, i)?.1, 1 => 8, 5 => 4, _ => return None };
            continue;
        }
        let (len, n) = varint(head, i)?;
        i += n;
        if field == 3 { return Some((i as u64, len)) }
        if field == 4 { return None } // colours before coords - not a layout we can size from
        i += len as usize;
    }
    None
}

/// Start a range request WITHOUT awaiting it. `window.fetch()` is eager - the browser has the
/// request in flight the moment this returns - so the caller can keep slice n+1 travelling
/// while it converts slice n. That overlap is the difference between 11 sequential round trips
/// and 11 hidden ones; see the loader in lib.rs.
/// Decode a packed `int32` (varint) array in full - the LOD index arrays.
pub fn packed_i32(raw: &[u8]) -> Vec<i32> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < raw.len() {
        let Some((v, n)) = varint(raw, i) else { break };
        out.push(v as i32);
        i += n;
    }
    out
}

/// Decode a packed `double` array in full.
pub fn packed_f64(raw: &[u8]) -> Vec<f64> {
    raw.chunks_exact(8).map(|c| f64::from_le_bytes(c.try_into().unwrap())).collect()
}

/// One cloud's LOD node table, read from the file's header region without touching a point.
/// `first`/`count` index the cloud's own point rows, which are stored in octree order - so a
/// node is one contiguous byte range in `coords` and can be fetched with a single `Range`.
#[derive(Clone)]
pub struct CloudLod {
    pub min: Vec<f64>,      // 3 per node
    pub size: Vec<f64>,     // 1 per node
    pub spacing: Vec<f64>,  // 1 per node
    pub level: Vec<i32>,
    pub first: Vec<i32>,
    pub count: Vec<i32>,
    pub children: Vec<i32>, // 8 per node, -1 = unused
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize { self.size.len() }
    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool { self.size.is_empty() }
}

pub fn fetch_range_start(url: &str, start: u64, len: u64) -> Result<Fetch, JsValue> {
    let headers = Headers::new()?;
    headers.set("Range", &format!("bytes={}-{}", start, start + len - 1))?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    opts.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    Ok(Fetch { fut: JsFuture::from(window.fetch_with_request(&request)) })
}

/// Finish one, insisting on `206` - see `fetch_range`.
pub async fn fetch_range_finish(f: Fetch) -> Result<Vec<u8>, JsValue> {
    let resp: Response = f.fut.await?.dyn_into()?;
    if resp.status() != 206 {
        return Err(JsValue::from_str("server ignored Range (no 206) - refusing to pull the whole body"));
    }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
}

/// Convert one already-fetched coords slice to f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }
    out
}

/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 431 MB scan would be catastrophic and silent.
/// `trunk serve` (axum + tower-http) does support ranges; `docs/serve.py`
/// (SimpleHTTPRequestHandler) does NOT.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, JsValue> {
    fetch_range_finish(fetch_range_start(url, start, len)?).await
}

/// Locate both packed arrays with two small reads: one at the head for `coords`, then one at
/// the end of the coords payload, where the `colors` header must be.
/// Read a cloud's LOD node table without touching a point.
///
/// Protobuf writes fields in number order, so the seven LOD arrays (8-14) sit at the very END
/// of the message, after the coords and colours payloads. Their offset is not in the header -
/// but it is COMPUTABLE from it: coords and colours each announce their byte length, so the
/// node table starts at `colors_at + colors_len`. Three small reads, no bulk transfer:
///
///   1. 8 KB header      -> coords offset and length, hence the exact point count
///   2. 16 B at coords end -> the colours header, hence where the node table starts
///   3. the tail          -> the seven arrays, 0.6 MB on a 13.8 M cloud
///
/// Returns `None` when the file carries no octree - the signal to fall back to whole-file.
pub async fn cloud_lod(url: &str) -> Option<(CloudFields, CloudLod)> {
    let fields = cloud_fields(url).await?;
    let lod_at = fields.colors_at + fields.colors_len;
    let tail = fetch_range(url, lod_at, u32::MAX as u64).await.ok()?;

    // The tail is the continuation of the PointCloud message: a run of tag/length pairs.
    let mut arrays: [&[u8]; 15] = [&[]; 15];
    let mut i = 0usize;
    while i < tail.len() {
        let (tag, n) = varint(&tail, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as usize, (tag & 7) as u32);
        if wire != 2 {
            i += match wire { 0 => varint(&tail, i)?.1, 1 => 8, 5 => 4, _ => break };
            continue;
        }
        let (len, n) = varint(&tail, i)?;
        i += n;
        let end = (i + len as usize).min(tail.len());
        if field < 15 { arrays[field] = &tail[i..end] }
        i = end;
    }

    let lod = CloudLod {
        min: packed_f64(arrays[8]),
        size: packed_f64(arrays[9]),
        spacing: packed_f64(arrays[10]),
        level: packed_i32(arrays[11]),
        first: packed_i32(arrays[12]),
        count: packed_i32(arrays[13]),
        children: packed_i32(arrays[14]),
    };
    if lod.is_empty() { return None }
    Some((fields, lod))
}

pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
    let head = fetch_range(url, 0, 8192).await.ok()?;
    let (coords_at, coords_len) = walk_to_coords(&head)?;
    if coords_len == 0 || coords_len % 24 != 0 { return None }

    let hdr = fetch_range(url, coords_at + coords_len, 16).await.ok()?;
    let (tag, n) = varint(&hdr, 0)?;
    if (tag >> 3) != 4 || (tag & 7) != 2 { return None } // expected the colours field next
    let (colors_len, n2) = varint(&hdr, n)?;
    Some(CloudFields {
        coords_at,
        coords_len,
        colors_at: coords_at + coords_len + (n + n2) as u64,
        colors_len,
        count: (coords_len / 24) as u32,
    })
}

/// Read the whole `colors` run and pack it to RGBA8. Packed uint32 is VARINT on the wire - not
/// memcpy-able the way `coords` is - so this decodes sequentially. It is 27 MB against the
/// coords' 87 MB, and taking it in one piece buys complete freedom from split-varint handling.
pub async fn cloud_colors(url: &str, at: u64, len: u64, count: u32) -> Option<Vec<u32>> {
    Some(cloud_colors_from(url, at, len, count).await?.0)
}

/// The same read, also reporting the ABSOLUTE byte offset just past the last colour decoded.
///
/// A packed varint field can only be decoded from a boundary, so a chunked reader that started
/// over each time would re-fetch the whole field per chunk - quadratic traffic, 448 MB of
/// colours on a 14 M cloud fetched in sevenths. Feeding this offset back as the next `at` makes
/// each chunk read only its own bytes, because the end of one chunk IS a boundary.
pub async fn cloud_colors_from(url: &str, at: u64, len: u64, count: u32) -> Option<(Vec<u32>, u64)> {
    let raw = fetch_range(url, at, len).await.ok()?;
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;
    for _ in 0..count {
        let mut rgba = [255u8; 4];
        for k in 0..4 {
            let (v, n) = varint(&raw, i)?;
            i += n;
            rgba[k] = (v & 255) as u8;
        }
        out.push(u32::from_le_bytes(rgba));
    }
    Some((out, at + i as u64))
}
