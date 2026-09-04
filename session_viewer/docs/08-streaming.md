# 08 Streaming a 431 MB cloud by Range

- A scan too big to decode opens anyway: `view_pointclouds` shows its first cloud in under 2 s and every file in about 6 s, at 264 MB of heap where decoding the same files whole took 1168 MB (`docs/_PERF.md`).
- The file is never fetched whole: `fetch_range` asks the bucket for byte ranges and refuses any answer but `206`, because a server that ignores `Range` sends the whole 431 MB with a `200`.
- `stream.rs` finds the packed `coords` and `colors` arrays and the octree's node table from a few KB of headers, because every hop of the wire format is length-delimited and `coords` is packed `double` - its length IS the point count.
- The first slice is a correct low-detail cloud, not a preview: the octree stores coarse levels first, so a 250 k-point prefix draws the whole scan sparsely and later slices only add detail.
- A cloud becomes a list of `Chunk`s in the lane because uploads are append-only and files interleave: a cloud's second slice lands far from its prefix, and `Chunk::row_of` maps a point index to a lane row.
- One budget covers the page (6 M points, `?points=` to change) and is split as a `share` over the scene's files, because a 14 M cloud resident at 16 B a point killed the GPU process.

<svg viewBox="0 0 720 360" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Lesson 8 on the two-halves map: fetch_range and stream.rs read a cloud by byte range; the loader posts a prefix and then slices; scene and walk turn each slice into a CloudDraw with a from; the engine keeps a chunk list per cloud that the octree walk and the splat records respect" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="s8a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#333"/></marker></defs>
  <rect x="14" y="10" width="190" height="30" fill="none" stroke="#333"/>
  <text x="109" y="29" fill="#222" text-anchor="middle">R2 bucket: scan.pb, 431 MB</text>
  <line x1="204" y1="25" x2="266" y2="25" stroke="#333" marker-end="url(#s8a)"/>
  <text x="235" y="19" fill="#666" font-size="9" text-anchor="middle">Range: bytes=a-b</text>
  <rect x="268" y="10" width="438" height="30" fill="none" stroke="#333"/>
  <text x="487" y="29" fill="#222" text-anchor="middle">app/fetch.rs  fetch_range(url, start, len) -> 206 or Err</text>
  <text x="14" y="62" fill="#222">app/</text>
  <text x="359" y="62" fill="#222" text-anchor="middle">Upload</text>
  <text x="706" y="62" fill="#222" text-anchor="end">engine/</text>
  <line x1="14" y1="67" x2="706" y2="67" stroke="#333"/>
  <line x1="309" y1="67" x2="309" y2="322" stroke="#999" stroke-dasharray="3 3"/>
  <line x1="409" y1="67" x2="409" y2="322" stroke="#999" stroke-dasharray="3 3"/>
  <rect x="14" y="76" width="290" height="52" fill="none" stroke="#333" stroke-width="1.3"/>
  <text x="22" y="91" fill="#222">stream.rs  (new file)</text>
  <text x="22" y="105" fill="#666" font-size="10">cloud_fields: 8 KB head -> coords_at, count</text>
  <text x="22" y="119" fill="#666" font-size="10">cloud_lod: node table off the tail; fetch_positions/colors</text>
  <line x1="159" y1="128" x2="159" y2="136" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="14" y="138" width="290" height="66" fill="none" stroke="#333"/>
  <text x="22" y="153" fill="#222">loader.rs</text>
  <text x="22" y="167" fill="#666" font-size="10">stream_prefix -> Msg::StreamedCloud (first slice + nodes)</text>
  <text x="22" y="181" fill="#666" font-size="10">stream_rest  -> Msg::CloudChunk, one slice per fetch</text>
  <text x="22" y="195" fill="#666" font-size="10">RESIDENT budget: max_points() split as share per file</text>
  <line x1="159" y1="204" x2="159" y2="212" stroke="#333" marker-end="url(#s8a)"/>
  <text x="166" y="211" fill="#666" font-size="9">via lib.rs Msg -> state.rs</text>
  <rect x="14" y="214" width="290" height="52" fill="none" stroke="#333"/>
  <text x="22" y="229" fill="#222">scene.rs</text>
  <text x="22" y="243" fill="#666" font-size="10">add_streamed_cloud(StreamedInit) -> slot idx</text>
  <text x="22" y="257" fill="#666" font-size="10">extend_streamed_cloud(idx, rows, to); streamed: Vec&lt;..&gt;</text>
  <line x1="159" y1="266" x2="159" y2="274" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="14" y="276" width="290" height="38" fill="none" stroke="#333"/>
  <text x="22" y="291" fill="#222">walk/cloud.rs</text>
  <text x="22" y="305" fill="#666" font-size="10">walk_stream_slice(StreamSlice) -> one CloudDraw</text>
  <line x1="304" y1="295" x2="318" y2="295" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="320" y="262" width="78" height="52" fill="none" stroke="#333" stroke-width="1.3"/>
  <text x="359" y="277" fill="#222" text-anchor="middle">CloudDraw</text>
  <text x="359" y="291" fill="#666" font-size="10" text-anchor="middle">{ from, count,</text>
  <text x="359" y="305" fill="#666" font-size="10" text-anchor="middle">first, nodes }</text>
  <line x1="398" y1="288" x2="414" y2="288" stroke="#333"/>
  <line x1="414" y1="288" x2="414" y2="112" stroke="#333"/>
  <line x1="414" y1="112" x2="422" y2="112" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="424" y="76" width="282" height="66" fill="none" stroke="#333" stroke-width="1.3"/>
  <text x="432" y="91" fill="#222">gpu/cloud.rs</text>
  <text x="432" y="105" fill="#666" font-size="10">Cloud { resident, chunks: Vec&lt;Chunk&gt; }</text>
  <text x="432" y="119" fill="#666" font-size="10">Chunk { from, to, row } . row_of(i)</text>
  <text x="432" y="133" fill="#666" font-size="10">append: from == 0 opens a cloud, else extend</text>
  <line x1="565" y1="142" x2="565" y2="150" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="424" y="152" width="282" height="38" fill="none" stroke="#333"/>
  <text x="432" y="167" fill="#222">gpu/lod.rs</text>
  <text x="432" y="181" fill="#666" font-size="10">select: node.count.min(resident - node.first)</text>
  <line x1="565" y1="190" x2="565" y2="198" stroke="#333" marker-end="url(#s8a)"/>
  <rect x="424" y="200" width="282" height="38" fill="none" stroke="#333"/>
  <text x="432" y="215" fill="#222">gpu/splat.rs</text>
  <text x="432" y="229" fill="#666" font-size="10">one record per (range n chunk); first = chunk.row_of(a)</text>
  <line x1="14" y1="322" x2="706" y2="322" stroke="#333"/>
  <text x="14" y="340" fill="#222">shell: lib.rs Msg::StreamedCloud / Msg::CloudChunk   state.rs add_streamed / extend_streamed</text>
  <text x="14" y="354" fill="#666" font-size="10">bold boxes = files this lesson creates or reshapes most; every other box gains a function</text>
</svg>

## Step 1 - Read a byte range

- A server that ignores `Range` answers `200` with the whole body, so `get` reads a body only when the status is the one the request asked for: `206` for a range, 2xx otherwise.
- `web-sys` gates every DOM type behind a feature; `Headers` is the one this step needs.

_Type it._
**Find** in `Cargo.toml`:

```toml
    "Response",
```

**Add below it:**

```toml
    "Headers",
```

_Type it._
**Find** in `src/app/fetch.rs`:

```rust
//! The browser's network edge: cross-origin GETs and the two ways to hand the browser its
//! main thread back.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
```

**Replace with:**

```rust
//! The browser's network edge: cross-origin GETs, HTTP Range reads
//! that refuse anything but `206`, and the two ways to hand the browser its main thread back.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};
```

_Type it._
**Find** in `src/app/fetch.rs`:

```rust
/// A GET's options: bypass the HTTP cache, revalidate it (a cached copy is used only when
/// the server says it is still current).
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,
    pub revalidate: bool,
}
```

**Replace with:**

```rust
/// A GET's options: bypass the HTTP cache, revalidate it (a cached copy is used only when
/// the server says it is still current), or read a byte range.
#[derive(Default)]
pub struct GetOpts {
    pub no_store: bool,
    pub revalidate: bool,
    pub range: Option<(u64, u64)>,
}
```

- An empty range never leaves the page: `bytes=a-(a-1)` is not a valid byte range, and whatever a server answered to it would not be the empty slice asked for.

_Type it._
**Find** in `src/app/fetch.rs`:

```rust
    let request = Request::new_with_str_and_init(url, &init).map_err(describe)?;
```

**Add above it:**

```rust
    let headers = Headers::new().map_err(describe)?;
    if let Some((start, len)) = opts.range {
        if len == 0 {
            return Ok(Reply { status: 206, bytes: Vec::new() });
        }
        headers.set("Range", &format!("bytes={}-{}", start, start + len - 1)).map_err(describe)?;
    }
    init.set_headers(&headers);
```

_Type it._
**Find** in `src/app/fetch.rs`:

```rust
    // A body is read only when it is the one asked for: an error page is not the file.
    let wanted = (200..300).contains(&status);
```

**Replace with:**

```rust
    // A body is read only when it is the one asked for: a `Range` answered with `200` is the
    // WHOLE file, an error page is not the file.
    let wanted = if opts.range.is_some() { status == 206 } else { (200..300).contains(&status) };
```

- `fetch_range` is the only entry the rest of the lesson calls, and it turns a `200` into an `Err` that names the URL.

_Type it._
**Find** in `src/app/fetch.rs`:

```rust
    Ok(r.bytes)
}
```

**Add below it:**

```rust

/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 431 MB scan would be catastrophic and silent.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, String> {
    let r = get(url, &GetOpts { range: Some((start, len)), ..GetOpts::default() }).await?;
    if r.status != 206 {
        return Err(format!("server ignored Range (HTTP {}) for {url}", r.status));
    }
    Ok(r.bytes)
}
```

## Step 2 - Locate the cloud inside its file

- The file is never parsed as a `Session`: `walk_to_coords` follows `Session.3 -> Objects.8 -> PointCloud` through length prefixes alone, and `coords` (field 3) being packed `double` makes its length the point count.
- `cloud_lod` walks the fields after the colours one 64-byte header at a time and fetches only the seven LOD arrays, skipping normals and point ids by their length.
- The byte-level half is pure and carries two native tests; the fetching half is `wasm32` only.

_Paste it._
**Create `src/app/stream.rs`**

```rust
//! Reading a cloud by HTTP Range, without decoding the file whole. Two facts about the wire
//! format make it possible: every hop `Session.3 -> Objects.8 -> PointCloud` is
//! length-delimited, so the headers sit in the first few KB; and `coords` is packed double,
//! so its length prefix gives the exact point count before a byte of payload is read. The
//! byte-level parsing here is pure and tested natively; the fetching half is wasm-only.

/// Where the two packed arrays live in the file, as absolute byte offsets.
#[derive(Clone, Copy, Debug)]
pub struct CloudFields {
    pub coords_at: u64,
    pub coords_len: u64,
    pub colors_at: u64,
    pub colors_len: u64,
    pub count: u32,
}

/// One cloud's LOD node table, read from the file's tail without touching a point.
/// `first`/`count` index the cloud's rows, which are stored in octree order.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,
    pub size: Vec<f64>,
    pub spacing: Vec<f64>,
    pub level: Vec<i32>,
    pub first: Vec<i32>,
    pub count: Vec<i32>,
    pub children: Vec<i32>,
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// Fill one of the seven arrays from its packed bytes; other fields are ignored.
    pub fn set_field(&mut self, field: usize, raw: &[u8]) {
        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => {}
        }
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}

/// One protobuf varint at `i`: the value and how many bytes it took.
pub fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let (mut v, mut shift) = (0u64, 0u32);
    let start = i;
    loop {
        let byte = *b.get(i)?;
        v |= ((byte & 0x7f) as u64) << shift;
        i += 1;
        if byte & 0x80 == 0 {
            return Some((v, i - start));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
}

/// Bytes a non-length-delimited field of wire type `wire` occupies at `i`.
fn skip_scalar(b: &[u8], i: usize, wire: u32) -> Option<usize> {
    match wire {
        0 => Some(varint(b, i)?.1),
        1 => Some(8),
        5 => Some(4),
        _ => None,
    }
}

/// A skipped field longer than this is geometry, not a name: the file holds more than one
/// cloud and is not streamed.
const NAME_BYTES: u64 = 256;

/// Walk `head` down `Session.3 -> Objects.8 -> PointCloud` and report where `coords` (field
/// 3) starts and how long it is. `None` for anything that is not a single-cloud file: a
/// geometry-sized field before the cloud, or bytes after it inside the objects message.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let mut i = 0usize;
    let mut end = head.len();
    let mut outer_end = end;
    for want in [3u32, 8u32] {
        let mut found = false;
        while i < end {
            let (tag, n) = varint(head, i)?;
            i += n;
            let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
            if wire != 2 {
                return None;
            }
            let (len, n) = varint(head, i)?;
            i += n;
            if field == want {
                outer_end = end;
                end = i + len as usize;
                found = true;
                break;
            }
            if len > NAME_BYTES {
                return None;
            }
            i += len as usize;
        }
        if !found {
            return None;
        }
    }
    if end != outer_end {
        return None;
    }
    while i < end {
        let (tag, n) = varint(head, i)?;
        i += n;
        let (field, wire) = ((tag >> 3) as u32, (tag & 7) as u32);
        if wire != 2 {
            i += skip_scalar(head, i, wire)?;
            continue;
        }
        let (len, n) = varint(head, i)?;
        i += n;
        if field == 3 {
            return Some((i as u64, len));
        }
        if field == 4 {
            return None;
        }
        i += len as usize;
    }
    None
}

/// A packed `int32` (varint) array in full.
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

/// A packed `double` array in full.
pub fn packed_f64(raw: &[u8]) -> Vec<f64> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()));
    }
    out
}

/// An already-fetched coords slice as f32 triples.
pub fn positions_from(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 8);
    for c in raw.chunks_exact(8) {
        out.push(f64::from_le_bytes(c.try_into().unwrap()) as f32);
    }
    out
}

/// `count` RGBA colours decoded from packed varints in `raw`, and the byte offset just past
/// the last one (so the next slice starts on a varint boundary).
pub fn colors_from(raw: &[u8], count: u32) -> Option<(Vec<u32>, usize)> {
    let mut out = Vec::with_capacity(count as usize);
    let mut i = 0usize;
    for _ in 0..count {
        let mut rgba = [255u8; 4];
        for k in &mut rgba {
            let (v, n) = varint(raw, i)?;
            i += n;
            *k = (v & 255) as u8;
        }
        out.push(u32::from_le_bytes(rgba));
    }
    Some((out, i))
}

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The fetching half: three small reads locate everything, then slices come down by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::app::fetch::fetch_range;

    /// Bytes per point of packed doubles.
    const POINT_BYTES: u64 = 24;

    /// Locate both packed arrays: one read at the head for `coords`, one at its end for the
    /// `colors` header.
    pub async fn cloud_fields(url: &str) -> Option<CloudFields> {
        let head = fetch_range(url, 0, 8192).await.ok()?;
        let (coords_at, coords_len) = walk_to_coords(&head)?;
        if coords_len == 0 || coords_len % POINT_BYTES != 0 {
            return None;
        }
        let after = coords_at + coords_len;
        let hdr = fetch_range(url, after, 16).await.ok()?;
        let (tag, n) = varint(&hdr, 0)?;
        let mut colors = (after, 0u64);
        if (tag >> 3) == 4 && (tag & 7) == 2 {
            let (len, n2) = varint(&hdr, n)?;
            colors = (after + (n + n2) as u64, len);
        }
        Some(CloudFields { coords_at, coords_len, colors_at: colors.0, colors_len: colors.1, count: (coords_len / POINT_BYTES) as u32 })
    }

    /// Bytes of one header read while walking the fields after the colours.
    const HEADER_READ: u64 = 64;

    /// The node table: walk the fields after the colours one tag/length header at a time,
    /// fetching ONLY the seven LOD arrays (8-14) and skipping normals (5) and point ids (15)
    /// by their length - on a 14 M cloud those two are 380 MB the table never needs.
    /// `None` = no octree.
    pub async fn cloud_lod(url: &str, f: &CloudFields) -> Option<CloudLod> {
        let mut at = f.colors_at + f.colors_len;
        let mut lod = CloudLod::default();
        let mut found = false;
        loop {
            // Past the end the store answers 416: the message is over.
            let Ok(head) = fetch_range(url, at, HEADER_READ).await else { break };
            if head.is_empty() {
                break;
            }
            let Some((tag, n)) = varint(&head, 0) else { break };
            let (field, wire) = ((tag >> 3) as usize, (tag & 7) as u32);
            if wire != 2 {
                let Some(skip) = skip_scalar(&head, n, wire) else { break };
                at += (n + skip) as u64;
                continue;
            }
            let Some((len, n2)) = varint(&head, n) else { break };
            let body_at = at + (n + n2) as u64;
            if (8..=14).contains(&field) {
                let raw = fetch_range(url, body_at, len).await.ok()?;
                lod.set_field(field, &raw);
                found = true;
            }
            if field >= 15 {
                break;
            }
            at = body_at + len;
        }
        if !found || lod.is_empty() { None } else { Some(lod) }
    }

    /// Points `[from, to)` of the coords run as f32 triples.
    pub async fn fetch_positions(url: &str, f: &CloudFields, from: u32, to: u32) -> Option<Vec<f32>> {
        let raw = fetch_range(url, f.coords_at + from as u64 * POINT_BYTES, (to - from) as u64 * POINT_BYTES).await.ok()?;
        Some(positions_from(&raw))
    }

    /// `count` colours starting at byte `at` of the colour run; returns them and where the
    /// run continues. 8 bytes per point is generous for four 0-255 varints.
    pub async fn fetch_colors(url: &str, f: &CloudFields, at: u64, count: u32) -> Option<(Vec<u32>, u64)> {
        let end = f.colors_at + f.colors_len;
        let want = (count as u64 * 8).min(end.saturating_sub(at));
        if want == 0 {
            return None;
        }
        let raw = fetch_range(url, at, want).await.ok()?;
        let (colors, used) = colors_from(&raw, count)?;
        Some((colors, at + used as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A varint round trip over the boundaries that matter.
    #[test]
    fn varint_reads_one_and_two_byte_values() {
        assert_eq!(varint(&[0x05], 0), Some((5, 1)));
        assert_eq!(varint(&[0xac, 0x02], 0), Some((300, 2)));
        assert_eq!(varint(&[0x80], 0), None);
    }

    /// Colours decode sequentially and report the boundary the next slice starts on.
    #[test]
    fn colors_decode_and_report_their_end() {
        let raw = [127u8, 0, 100, 127, 1, 2, 3, 4, 9, 9];
        let (c, used) = colors_from(&raw, 2).unwrap();
        assert_eq!(c, [0x7f64_007f, 0x0403_0201]);
        assert_eq!(used, 8);
    }
}
```

_Type it._
**Find** in `src/app/mod.rs`:

```rust
//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (route, fetch, decode, the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.

pub mod input;
pub mod knobs;
pub mod manifest;
pub mod scene;
```

**Replace with:**

```rust
//! The app layer: what a scene IS (manifest, documents, the walk into rows) and how it gets
//! here (route, fetch, decode, stream, the loader) and is driven (input,
//! touch). Above the engine, below the shell in lib.rs. Never names a wgpu type.

pub mod input;
pub mod knobs;
pub mod manifest;
pub mod scene;
pub mod stream;
```

## Step 3 - A cloud is a list of chunks

- A `CloudDraw` now says WHICH points of its cloud it carries: `from == 0` opens the cloud, a later `from` extends the one already open on the same object row.
- `Cloud` stops being one contiguous range: `resident` is how many points are on the GPU, `chunks` is where each run of them landed, and `row_of` translates a point index through them.

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
//! The cloud lane's tables: positions, colours, optional normals, the octree nodes, and one
//! `Cloud` record per cloud. `CloudRows` is one upload's delta; `CloudLane` is the GPU side.

use super::buffers::{GpuCtx, GrowBuf, ROWS};
```

**Replace with:**

```rust
//! The cloud lane's tables: positions, colours, optional normals, the octree nodes, and one
//! `Cloud` record per cloud. A cloud's points arrive in CHUNKS (a whole file is one chunk; a
//! streamed file is a prefix and then slices, interleaved with other files' rows), so a
//! cloud maps its own point index to lane rows through its chunk list. `CloudRows` is one
//! upload's delta; `CloudLane` is the GPU side.

use super::buffers::{GpuCtx, GrowBuf, ROWS};
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
/// One cloud in an upload: `count` points landing at upload-local row `first`, with its node
/// table and spacing.
pub struct CloudDraw {
    pub instance: u32,
```

**Replace with:**

```rust
/// One upload's contribution to a cloud: points `[from, from + count)` of the cloud, landing
/// at upload-local row `first`. `from == 0` opens a cloud (and carries its node table and
/// spacing); a later `from` extends the cloud already open on the same object row.
pub struct CloudDraw {
    pub instance: u32,
    pub from: u32,
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
/// One cloud as the lane knows it: its object row, its node slice, and its rows
/// `[first, first + count)` in the lane.
pub struct Cloud {
    pub instance: u32,
    pub spacing: f32,
    pub node_first: u32,
    pub node_count: u32,
    pub nrm_first: u32,
    pub first: u32,
    pub count: u32,
}
```

**Replace with:**

```rust
/// Points `[from, to)` of a cloud, resident at lane rows starting at `row`.
#[derive(Clone, Copy)]
pub struct Chunk {
    pub from: u32,
    pub to: u32,
    pub row: u32,
}

impl Chunk {
    /// The lane row of cloud point `i`, which must lie in the chunk.
    pub fn row_of(&self, i: u32) -> u32 {
        self.row + (i - self.from)
    }
}

/// One cloud as the lane knows it: its object row, its node slice, and the chunks resident
/// so far. `resident` is the point count the chunks cover, contiguous from 0.
pub struct Cloud {
    pub instance: u32,
    pub spacing: f32,
    pub node_first: u32,
    pub node_count: u32,
    pub nrm_first: u32,
    pub resident: u32,
    pub chunks: Vec<Chunk>,
}

impl Cloud {
    /// The lane row of cloud point `i`, or `None` past the resident prefix.
    pub fn row_of(&self, i: u32) -> Option<u32> {
        let c = self.chunks.iter().find(|c| i >= c.from && i < c.to)?;
        Some(c.row_of(i))
    }
}
```

## Step 4 - Open a cloud or extend it

- `append` builds one `Chunk` per draw from the upload-local `first` and the lane's `point_base`; a draw with `from > 0` goes to `extend` instead of opening a second cloud.
- A chunk that does not continue the resident prefix is dropped with a warning, because the octree walk can only address a prefix.

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
    /// Append one upload: rows to the tables, nodes to the node table, one cloud per draw.
    /// Returns whether a buffer moved (the point lane must rebind).
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
```

**Replace with:**

```rust
    /// Append one upload: rows to the tables, nodes to the node table, and each draw either
    /// opens a cloud or adds a chunk to the one on its object row. Returns whether a buffer
    /// moved (the point lane must rebind).
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
        for d in &up.draws {
```

**Add below it:**

```rust
            let chunk = Chunk { from: d.from, to: d.from + d.count, row: point_base + d.first };
            if d.from > 0 {
                self.extend(d.instance, chunk);
                continue;
            }
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
                first: point_base + d.first,
                count: d.count,
```

**Replace with:**

```rust
                resident: chunk.to,
                chunks: vec![chunk],
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
        moved
    }
```

**Add below it:**

```rust

    /// Add a chunk to the cloud on object row `instance`; a chunk that does not continue the
    /// resident prefix is dropped with a warning (the walk cannot address it).
    fn extend(&mut self, instance: u32, chunk: Chunk) {
        let Some(c) = self.clouds.iter_mut().find(|c| c.instance == instance) else {
            log::warn!("cloud chunk for row {instance} arrived before its cloud; dropped");
            return;
        };
        if chunk.from != c.resident {
            log::warn!("cloud chunk [{}, {}) does not continue the {} resident points; dropped", chunk.from, chunk.to, c.resident);
            return;
        }
        c.resident = chunk.to;
        c.chunks.push(chunk);
    }
```

- A pick's global row now has to search every chunk, and the point index it reports is `k.from` plus the offset into that chunk.

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
            if row >= c.first && row < c.first + c.count {
                return Some((c.instance, row - c.first));
            }
```

**Replace with:**

```rust
            for k in &c.chunks {
                if row >= k.row && row < k.row + (k.to - k.from) {
                    return Some((c.instance, k.from + (row - k.row)));
                }
            }
```

_Type it._
**Find** in `src/engine/gpu/cloud.rs`:

```rust
        PointBufs { pos: &self.pos.buf, col: &self.col.buf, nrm: &self.nrm.buf }
    }
```

**Add below it:**

```rust

    /// Points resident across every cloud.
    pub fn resident(&self) -> u32 {
        self.clouds.iter().map(|c| c.resident).sum()
    }
```

## Step 5 - Clip the octree walk to what is resident

- A node whose points have not arrived is skipped, and a node that is only partly resident is drawn up to `resident`: the walk never names a row the lane does not hold.
- The no-octree fallback draws the resident prefix, not the cloud's total.

_Type it._
**Find** in `src/engine/gpu/lod.rs`:

```rust
    /// The ranges one cloud contributes: the whole cloud, or the octree nodes whose
    /// spacing still projects wider than `lod_px` pixels (each node OWNS its subsample, so
    /// descending only adds detail), every node sized by the finest spacing selected beneath
    /// it.
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();
        if c.node_count == 0 || p.lod_px <= 0.0 || c.count < LOD_MIN_POINTS {
            self.ranges.push(Range { first: 0, count: c.count, spacing: c.spacing, tile: false });
```

**Replace with:**

```rust
    /// The ranges one cloud contributes: the whole resident prefix, or the octree nodes whose
    /// spacing still projects wider than `lod_px` pixels (each node OWNS its subsample, so
    /// descending only adds detail), every node sized by the finest spacing selected beneath
    /// it and clipped to the points resident so far.
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();
        if c.node_count == 0 || p.lod_px <= 0.0 || c.resident < LOD_MIN_POINTS {
            self.ranges.push(Range { first: 0, count: c.resident, spacing: c.spacing, tile: false });
```

_Type it._
**Find** in `src/engine/gpu/lod.rs`:

```rust
            let count = node.count;
```

**Replace with:**

```rust
            if node.first >= c.resident {
                continue;
            }
            let count = node.count.min(c.resident - node.first);
```

## Step 6 - One record per chunk

- The point pass draws lane rows, and a range of cloud points can straddle two chunks that sit far apart in the lane, so every range is cut against every chunk and each non-empty piece becomes its own record.
- The pass key counts resident points, so a slice landing changes the key and the records are rebuilt.

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
            point_count += c.count;
```

**Replace with:**

```rust
            point_count += c.resident;
```

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
    /// One record per visible cloud, or per selected octree node when the LOD walk is on.
    fn build_records(&mut self, cx: &RecordCx) {
```

**Replace with:**

```rust
    /// One record per visible cloud, or per selected octree node when the LOD walk is on;
    /// a range that straddles two chunks of a streamed cloud becomes two records.
    fn build_records(&mut self, cx: &RecordCx) {
```

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
            for r in &self.walk.ranges {
                if self.records.len() >= MAX_RECORDS {
                    break;
                }
                let k = radius_factor(r, px, scale, cx.ortho_h);
                let nrm_first = if c.nrm_first == NO_NORMALS { NO_NORMALS } else { c.nrm_first + r.first };
                self.records.push(SplatRecord {
                    mvp_model: m,
                    tint,
                    first: c.first + r.first,
                    count: r.count,
                    cum,
                    k,
                    rot,
                    nrm_first,
                    instance: c.instance,
                    flags: row.flags,
                    _pad: 0,
                });
                cum += r.count;
            }
```

**Replace with:**

```rust
            for r in &self.walk.ranges {
                let k = radius_factor(r, px, scale, cx.ortho_h);
                for chunk in &c.chunks {
                    let a = r.first.max(chunk.from);
                    let b = (r.first + r.count).min(chunk.to);
                    if a >= b || self.records.len() >= MAX_RECORDS {
                        continue;
                    }
                    let nrm_first = if c.nrm_first == NO_NORMALS { NO_NORMALS } else { c.nrm_first + a };
                    self.records.push(SplatRecord {
                        mvp_model: m,
                        tint,
                        first: chunk.row_of(a),
                        count: b - a,
                        cum,
                        k,
                        rot,
                        nrm_first,
                        instance: c.instance,
                        flags: row.flags,
                        _pad: 0,
                    });
                    cum += b - a;
                }
            }
```

## Step 7 - Walk a streamed slice into the lane

- A slice is raw rows off the wire, already `f32` and RGBA words, plus a borrowed `CloudLod`; the first slice (`from == 0`) pushes the cloud's WHOLE node table so later slices need none.
- `walk_cloud` keeps its one draw, which now says `from: 0`.

_Type it._
**Find** in `src/app/walk/cloud.rs`:

```rust
//! Point clouds into the cloud lane: a walked kernel `PointCloud` (points, optional normals,
//! the octree it carries, one draw).

use session_rust::PointCloud;
```

**Replace with:**

```rust
//! Point clouds into the cloud lane: a walked kernel `PointCloud` (points, optional normals,
//! the octree it carries, one draw), and the streamed form - a prefix or chunk of raw rows
//! that never became a kernel object, with the nodes those rows complete.

use session_rust::PointCloud;
use crate::app::stream::CloudLod;
```

_Type it._
**Find** in `src/app/walk/cloud.rs`:

```rust
        instance: cx.row,
```

**Add below it:**

```rust
        from: 0,
```

_Type it._
**Find** in `src/app/walk/cloud.rs`:

```rust
    (area / n as f64).sqrt() as f32
}
```

**Add below it:**

```rust

/// A streamed slice: raw rows off the wire, already converted.
pub struct StreamRows {
    pub positions: Vec<f32>,
    pub colors: Vec<u32>,
}

/// One streamed slice into the lane: rows `[from, to)` of the cloud. The first slice
/// (`from == 0`) carries the cloud's WHOLE node table, so the walk can descend to nodes
/// whose points arrive later and clip them to what is resident.
pub struct StreamSlice<'a> {
    pub rows: StreamRows,
    pub lod: &'a CloudLod,
    pub from: u32,
    pub to: u32,
    pub row: u32,
    pub point_px: f32,
}
```

- The slice's spacing is the finest node spacing among the nodes COMPLETE within the resident prefix, so the splat radius shrinks as detail arrives.
- A colour run shorter than the positions is padded with opaque black: the colour table stays one word a point.

_Type it._
**Find** in `src/app/walk/cloud.rs`:

```rust
    pub point_px: f32,
}
```

**Add below it:**

```rust

/// Append one streamed slice; returns the slice's local box (the first slice's box is the
/// prefix's, which spreads over the whole cloud since the octree stores coarse levels first).
/// A short colour run is padded with opaque black so the colour table stays one word a point.
pub fn walk_stream_slice(c: &mut CloudRows, s: &StreamSlice) -> Aabb {
    let first = c.point_count();
    let node_first = c.nodes.len() as u32;
    let mut node_count = 0u32;
    if s.from == 0 {
        for k in 0..s.lod.len() {
            c.nodes.push(lod_node(s.lod, k));
        }
        node_count = s.lod.len() as u32;
    }

    let mut bounds = Aabb::empty();
    for p in s.rows.positions.chunks_exact(3) {
        bounds.grow([p[0], p[1], p[2]]);
    }
    let count = (s.rows.positions.len() / 3) as u32;
    let colors = &s.rows.colors[..s.rows.colors.len().min(count as usize)];
    c.col.extend_from_slice(colors);
    c.col.resize(first as usize + count as usize, 0xff00_0000);
    c.pos.extend_from_slice(&s.rows.positions);
    c.draws.push(CloudDraw {
        instance: s.row,
        from: s.from,
        count,
        first,
        spacing: resident_spacing(s.lod, s.to).unwrap_or(s.point_px.max(DEFAULT_SPACING)),
        node_first,
        node_count,
        nrm_first: NO_NORMALS,
    });
    bounds
}

/// The finest node spacing among the nodes complete within the first `to` points.
fn resident_spacing(lod: &CloudLod, to: u32) -> Option<f32> {
    let mut spacing = f64::INFINITY;
    for k in 0..lod.len() {
        let (f, n) = (lod.first[k], lod.count[k]);
        if f >= 0 && n >= 0 && (f + n) as u32 <= to {
            spacing = spacing.min(lod.spacing[k]);
        }
    }
    spacing.is_finite().then_some(spacing as f32)
}

/// One node of a streamed cloud's LOD table.
fn lod_node(lod: &CloudLod, k: usize) -> LodNode {
    let mut children = [-1i32; 8];
    for (slot, v) in lod.children[k * 8..k * 8 + 8].iter().enumerate() {
        children[slot] = *v;
    }
    let half = lod.size[k] as f32 * 0.5;
    LodNode {
        center: [lod.min[k * 3] as f32 + half, lod.min[k * 3 + 1] as f32 + half, lod.min[k * 3 + 2] as f32 + half],
        size: lod.size[k] as f32,
        spacing: lod.spacing[k] as f32,
        first: lod.first[k] as u32,
        count: lod.count[k] as u32,
        children,
    }
}
```

## Step 8 - A streamed cloud in the scene

- `StreamedInit` is the loader's first delivery: the first slice, the node table, the file layout and where the colour run continues; `StreamedCloud` is the slot the scene keeps so later slices know their object row and how far the cloud is resident.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
//! The document side: `Scene` owns WHAT is loaded - every kernel `Session` with its placement,
//! the `Upload` tables and the row bookkeeping. `add_file` walks one
//! session into the tables; rows are appended, never rebuilt. This file never names a
//! `Geometry` variant - the producers live in `walk/`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use session_rust::{Session, Xform};
use crate::app::knobs;
use crate::app::walk::bounds::{file_extent, is_planar, mark_sheet, Baselines};
```

**Replace with:**

```rust
//! The document side: `Scene` owns WHAT is loaded - every kernel `Session` with its placement,
//! the `Upload` tables, the row bookkeeping and the streamed-cloud slots. `add_file` walks one
//! session into the tables; rows are appended, never rebuilt. This file never names a
//! `Geometry` variant - the producers live in `walk/`.

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use session_rust::{Session, Xform};
use crate::app::knobs;
use crate::app::stream::{CloudFields, CloudLod};
use crate::app::walk::bounds::{file_extent, is_planar, mark_sheet, Baselines};
use crate::app::walk::cloud::{walk_stream_slice, StreamRows, StreamSlice};
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    pub point_px: f32,
    pub display_only: bool,
}
```

**Add below it:**

```rust

/// A streamed cloud's first slice and what later slices need: its file's node table, how
/// many points are resident, and the total.
pub struct StreamedInit {
    pub name: String,
    pub url: String,
    pub place: Xform,
    pub rows: StreamRows,
    pub lod: CloudLod,
    /// Where the packed arrays sit in the file, so the next slices need no second probe.
    pub fields: CloudFields,
    pub resident: u32,
    pub point_px: f32,
    /// Where the colour run continues for the next slice.
    pub col_at: u64,
}

/// A cloud still arriving off the wire.
pub struct StreamedCloud {
    pub name: String,
    pub url: String,
    pub row: u32,
    pub lod: CloudLod,
    pub done_to: u32,
    pub total: u32,
    pub point_px: f32,
}
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    pub tables: Upload,
```

**Add below it:**

```rust
    pub streamed: Vec<StreamedCloud>,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
            tables: Upload::default(),
```

**Add below it:**

```rust
            streamed: Vec::new(),
```

- A streamed cloud has no kernel object, so `clear` forgets it and `rebuild` cannot bring it back.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.docs.clear();
        self.tables = Upload::default();
```

**Add below it:**

```rust
        self.streamed.clear();
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    /// path an edit commit takes.
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        self.tables = Upload::default();
```

**Replace with:**

```rust
    /// path an edit commit takes. Streamed clouds cannot come back (no kernel object).
    pub fn rebuild(&mut self, gpu: &mut Gpu) {
        let docs = std::mem::take(&mut self.docs);
        self.tables = Upload::default();
        self.streamed.clear();
```

- The first slice is uploaded AT ONCE, so the slot records the absolute lane row point 0 landed on; the document it pushes is a `display_only` shell around an empty `Session`.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.docs.push(Doc { name, place, session, point_px, display_only });
    }
```

**Add below it:**

```rust

    /// Add a streamed cloud from its first slice and upload it at once, so the slot knows the
    /// absolute row its point 0 landed on. Returns the slot index later slices address.
    pub fn add_streamed_cloud(&mut self, init: StreamedInit, gpu: &mut Gpu) -> usize {
        let StreamedInit { name, url, place, rows, lod, fields, resident, point_px, col_at: _ } = init;
        let total = fields.count;
        let row = self.push_row(&format!("stream:{url}"), place.m, 0);
        let slice = StreamSlice { rows, lod: &lod, from: 0, to: resident, row, point_px };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        o.spacing = point_px;
        o.thickness = bounds.thinnest();
        self.tables.bounds.union(&bounds.placed(&place.m));
        self.upload_to(gpu);

        self.docs.push(Doc { name: name.clone(), place, session: Rc::new(Session::new(&name)), point_px, display_only: true });
        self.streamed.push(StreamedCloud { name, url, row, lod, done_to: resident, total, point_px });
        self.streamed.len() - 1
    }
```

- A later slice walks with `from = done_to`, so its draw extends the cloud instead of opening one; the scene's bounds grow by the slice's placed box.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.streamed.len() - 1
    }
```

**Add below it:**

```rust

    /// Append the next slice `[done_to, to)` of streamed cloud `idx` and upload it.
    pub fn extend_streamed_cloud(&mut self, idx: usize, rows: StreamRows, to: u32, gpu: &mut Gpu) {
        let Some(sc) = self.streamed.get(idx) else { return };
        if to <= sc.done_to {
            return;
        }
        let place = self.docs.iter().find(|d| d.name == sc.name).map(|d| d.place.m).unwrap_or(Xform::identity().m);
        let slice = StreamSlice { rows, lod: &sc.lod, from: sc.done_to, to, row: sc.row, point_px: sc.point_px };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        self.tables.bounds.union(&bounds.placed(&place));
        self.streamed[idx].done_to = to;
        self.upload_to(gpu);
    }
```

## Step 9 - Two entry points on State

- Both grow the camera's extent and ask for a frame, exactly as `append` does; neither touches the GPU directly.

_Type it._
**Find** in `src/state.rs`:

```rust
use crate::app::scene::{FileDoc, Scene};
```

**Replace with:**

```rust
use crate::app::scene::{FileDoc, Scene, StreamedInit};
use crate::app::walk::cloud::StreamRows;
```

_Type it._
**Find** in `src/state.rs`:

```rust
        log::info!("appended: walk {:.0} ms, upload {:.0} ms | {} docs", t1 - t0, now_ms() - t1, self.scene.docs.len());
        self.needs_frame = true;
    }
```

**Add below it:**

```rust

    /// A streamed cloud's first slice; returns the slot later slices address.
    pub fn add_streamed(&mut self, init: StreamedInit) -> usize {
        let idx = self.scene.add_streamed_cloud(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.needs_frame = true;
        idx
    }

    /// One more slice of streamed cloud `idx`.
    pub fn extend_streamed(&mut self, idx: usize, rows: StreamRows, to: u32) {
        self.scene.extend_streamed_cloud(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!("cloud slice: {to} points resident");
        self.needs_frame = true;
    }
```

## Step 10 - Two messages

- `StreamedCloud` carries the first slice and is boxed because the enum would otherwise be the size of a `StreamedInit`; `CloudChunk` carries every later slice and the point the cloud is then resident up to.
- The handler for `StreamedCloud` is the one place the rest of the cloud is started: only once the scene has handed back the slot index can slices be addressed.

_Type it._
**Find** in `src/lib.rs`:

```rust
use crate::app::scene::FileDoc;
```

**Replace with:**

```rust
use crate::app::scene::{FileDoc, StreamedInit};
use crate::app::walk::cloud::StreamRows;

/// One more slice of streamed cloud `idx`: its rows and the point the cloud is resident up to.
pub struct CloudChunk {
    pub idx: usize,
    pub rows: StreamRows,
    pub to: u32,
}
```

_Type it._
**Find** in `src/lib.rs`:

```rust
    Fit,
```

**Add below it:**

```rust
    StreamedCloud(Box<StreamedInit>),
    CloudChunk(CloudChunk),
```

_Type it._
**Find** in `src/lib.rs`:

```rust
            Msg::File(doc) => state.append(doc),
```

**Add below it:**

```rust
            Msg::StreamedCloud(init) => {
                let (url, fields, from, col_at) = (init.url.clone(), init.fields, init.resident, init.col_at);
                let idx = state.add_streamed(*init);
                loader::spawn_stream_rest(loader::StreamCursor { idx, url, fields, from, col_at });
            }
            Msg::CloudChunk(c) => state.extend_streamed(c.idx, c.rows, c.to),
```

## Step 11 - The page's point budget

- The ceiling is a PAGE budget, not a per-file one: `RESIDENT` counts every streamed point on screen, and `budget_left` is what the next slice may take; `?points=` overrides the 6 M default.
- `GENERATION` is what a slice loop compares against before it posts: a task from a scene that was replaced stops instead of landing rows in the next scene (the bump on `Clear` comes with the live scene in lesson 9).

_Type it._
**Find** in `src/app/loader.rs`:

```rust
//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`. Touches no GPU.

use std::rc::Rc;
use std::cell::RefCell;
```

**Replace with:**

```rust
//! The async loader (wasm): bring the canvas up EMPTY, then post every document to the
//! event loop as a `Msg` - whole files through `decode`, big clouds a slice at a time
//! through `stream`. Touches no GPU.

use std::rc::Rc;
use std::cell::{Cell, RefCell};
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
use winit::window::Window;
```

**Add below it:**

```rust
use session_rust::Xform;
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
use crate::{Msg, State};
```

**Replace with:**

```rust
use crate::{CloudChunk, Msg, State};
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
use super::route::{join, scene_route, SceneRoute};
use super::scene::{FileDoc, Scene};
```

**Replace with:**

```rust
use super::route::{join, knob_u32, scene_route, SceneRoute};
use super::scene::{FileDoc, Scene, StreamedInit};
use super::stream::{cloud_fields, cloud_lod, fetch_colors, fetch_positions, CloudFields};
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
use super::route::AUTO_GRID;
```

**Add below it:**

```rust
use super::walk::cloud::StreamRows;

/// Points a streamed cloud brings down before it is on screen: the octree's coarse levels,
/// so the file opens at a correct low detail whatever its size.
const STREAM_PREFIX_POINTS: u32 = 2_000_000;

/// Points per follow-up slice.
const STREAM_CHUNK_POINTS: u32 = 2_000_000;

/// Hard ceiling on resident streamed points across the whole page (`?points=` to change):
/// 16 B a point on the GPU plus the growth slack, and a 14 M cloud killed the GPU process.
const STREAM_MAX_POINTS: u32 = 6_000_000;

/// Files at least this large open by range even without a count-based reason.
const STREAM_MIN_BYTES: u64 = 64 * 1024 * 1024;

/// The smallest prefix a streamed cloud gets even past the ceiling: the coarsest octree levels,
/// so the cloud is on screen and correct, just sparse.
const STREAM_MIN_PREFIX: u32 = 250_000;
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
thread_local! {
    /// The start-up proxy, kept so the loader can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
}
```

**Replace with:**

```rust
thread_local! {
    /// The start-up proxy, kept so the stream tasks can post messages.
    static PROXY: RefCell<Option<EventLoopProxy<Msg>>> = const { RefCell::new(None) };
    /// Points resident across every streamed cloud on the page: the ceiling is a scene budget.
    static RESIDENT: Cell<u32> = const { Cell::new(0) };
    /// Bumped on every `Clear`: a stream task from an older scene stops at its next slice.
    static GENERATION: Cell<u32> = const { Cell::new(0) };
}
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
    PROXY.with(|p| p.borrow().as_ref().map(|proxy| proxy.send_event(msg).is_ok())).unwrap_or(false)
}
```

**Add below it:**

```rust

/// The resident ceiling for this page load.
fn max_points() -> u32 {
    knob_u32("points").unwrap_or(STREAM_MAX_POINTS)
}

/// Points the scene may still make resident.
fn budget_left() -> u32 {
    RESIDENT.with(|r| max_points().saturating_sub(r.get()))
}

/// Book `n` points against the budget.
fn budget_spend(n: u32) {
    RESIDENT.with(|r| r.set(r.get().saturating_add(n)));
}
```

## Step 12 - Probe every .pb before decoding it whole

- Every `.pb` item is offered to `stream_prefix` first; `None` (no octree, or small enough) falls through to the whole-file path unchanged.
- `share` is the budget divided by the scene's `.pb` count, so the first cloud in manifest order cannot take it all.

_Type it._
**Find** in `src/app/loader.rs`:

```rust
    for (i, item) in manifest.items.iter().enumerate() {
```

**Add above it:**

```rust
    let files = manifest.items.iter().filter(|i| i.file.ends_with(".pb")).count().max(1) as u32;
    let share = (max_points() / files).max(STREAM_MIN_PREFIX);
```

_Type it._
**Find** in `src/app/loader.rs`:

```rust
        let point_px = item.point_size as f32;
```

**Add below it:**

```rust
        if url.ends_with(".pb") {
            let slot = Placement { name: manifest.name_of(i, &item.file), place: place.clone(), point_px };
            if let Some(init) = stream_prefix(&url, &slot, share).await {
                post(Msg::StreamedCloud(Box::new(init)));
                continue;
            }
        }
```

## Step 13 - The prefix

- Three small reads locate everything (`cloud_fields`, then `cloud_lod`), one read brings the prefix: at most `STREAM_PREFIX_POINTS`, never more than `share` or what is left, never fewer than `STREAM_MIN_PREFIX`.
- A failed prefix read still posts the cloud with zero rows, so the scene has its slot and the log says what a whole decode would have cost.

_Type it._
**Find** in `src/app/loader.rs`:

```rust
    log::info!("scene posted {:.0} ms after the manifest fetch", now_ms() - t0);
}
```

**Add below it:**

```rust

/// Where a streamed cloud goes: its document name, placement and point size.
struct Placement {
    name: String,
    place: Xform,
    point_px: f32,
}

/// Try to open a cloud by RANGE: `None` means the file carries no octree or is small enough
/// to decode whole. Three small reads locate everything, one read brings the prefix - at most
/// `share` points (the budget split over the scene's files, so the first cloud cannot take it
/// all), never fewer than `STREAM_MIN_PREFIX`.
async fn stream_prefix(url: &str, slot: &Placement, share: u32) -> Option<StreamedInit> {
    let (name, place, point_px) = (slot.name.as_str(), slot.place.clone(), slot.point_px);
    let fields = cloud_fields(url).await?;
    if fields.count <= STREAM_PREFIX_POINTS && fields.coords_len < STREAM_MIN_BYTES {
        return None;
    }
    let lod = cloud_lod(url, &fields).await?;
    let resident = STREAM_PREFIX_POINTS.min(share).min(fields.count).min(budget_left().max(STREAM_MIN_PREFIX));
    let Some(positions) = fetch_positions(url, &fields, 0, resident).await else {
        log::warn!("'{name}': the prefix range read failed - the cloud stays off screen (a whole decode would take {:.0} MB)", fields.coords_len as f64 / 1.048576e6);
        return Some(StreamedInit { name: name.to_string(), url: url.to_string(), place, rows: StreamRows { positions: Vec::new(), colors: Vec::new() }, lod, fields, resident: 0, point_px, col_at: fields.colors_at });
    };
    let (colors, col_at) = fetch_colors(url, &fields, fields.colors_at, resident).await.unwrap_or((Vec::new(), fields.colors_at));
    budget_spend(resident);
    log::info!("streamed '{name}': {resident} of {} points on screen, {} nodes", fields.count, lod.len());
    Some(StreamedInit { name: name.to_string(), url: url.to_string(), place, rows: StreamRows { positions, colors }, lod, fields, resident, point_px, col_at })
}
```

## Step 14 - The rest, one slice at a time

- `StreamCursor` is what `lib.rs` hands back once the scene has a slot: the slot index, the file layout, the next point and where the colour run continues.
- Each turn books its slice against the budget BEFORE fetching, checks the generation before and after the await, and stops at the ceiling with a log line that names `?points=`.

_Type it._
**Find** in `src/app/loader.rs`:

```rust
    Some(StreamedInit { name: name.to_string(), url: url.to_string(), place, rows: StreamRows { positions, colors }, lod, fields, resident, point_px, col_at })
}
```

**Add below it:**

```rust

/// Where a streamed cloud continues: its slot, its file layout, the next point and where the
/// colour run continues.
pub struct StreamCursor {
    pub idx: usize,
    pub url: String,
    pub fields: CloudFields,
    pub from: u32,
    pub col_at: u64,
}

/// Fetch the rest of a streamed cloud, a slice at a time, posting each one; spawned once the
/// scene has given the cloud its slot. Stops when the scene it belongs to is cleared.
pub fn spawn_stream_rest(cursor: StreamCursor) {
    wasm_bindgen_futures::spawn_local(stream_rest(cursor));
}

/// The slice loop behind `spawn_stream_rest`.
async fn stream_rest(c: StreamCursor) {
    let (url, idx, fields) = (c.url, c.idx, c.fields);
    let generation = GENERATION.with(|g| g.get());
    let mut col_at = c.col_at;
    let mut at = c.from;
    while at < fields.count {
        if GENERATION.with(|g| g.get()) != generation {
            return;
        }
        let left = budget_left();
        if left == 0 {
            log::info!("'{url}': {at} of {} points resident - at the page's point ceiling (?points= to raise it)", fields.count);
            return;
        }
        let to = (at + STREAM_CHUNK_POINTS.min(left)).min(fields.count);
        budget_spend(to - at);
        let Some(positions) = fetch_positions(&url, &fields, at, to).await else { return };
        let (colors, next) = fetch_colors(&url, &fields, col_at, to - at).await.unwrap_or((Vec::new(), col_at));
        col_at = next;
        if GENERATION.with(|g| g.get()) != generation {
            return;
        }
        if !post(Msg::CloudChunk(CloudChunk { idx, rows: StreamRows { positions, colors }, to })) {
            return;
        }
        at = to;
    }
}

```

## Run

```bash
trunk serve
```

- Open `http://127.0.0.1:8770/?scene=view_pointclouds`: the first scan is on screen as a sparse but complete cloud, then densifies slice by slice while the next files arrive.
- The console prints `streamed '<name>': N of M points on screen, K nodes` per cloud, `cloud slice: N points resident` per slice, and a `... - at the page's point ceiling (?points= to raise it)` line from each cloud that stops at the 6 M budget.

## Why

- A 431 MB `.pb` decoded whole is a kernel `PointCloud` plus a copy in the walk tables: 1168 MB of heap on this scene, and the 14 M cloud killed the GPU process; by range the same scene sits at 264 MB.
- `206`-only is the safety catch: the browser's `fetch` happily returns a `200` for a `Range` request when the server ignores the header, and that `200` is the whole file.
- The prefix is correct because the kernel's octree order stores coarse levels first; `resident_spacing` and the lod clamp make the lane draw exactly the nodes that are complete, so nothing is ever drawn from rows that have not arrived.
- Chunks exist because the lane is append-only and files interleave: a later slice lands after other files' rows, so a cloud maps point index to lane row through its chunk list, and a splat record never spans two chunks.
- The budget is per page and shared out per file because the scene, not the file, is what has to fit in GPU memory; `?points=` is the knob, `STREAM_MIN_PREFIX` the floor so a cloud past the ceiling is still on screen.
- The colour run is varint-packed and cannot be indexed by point, so the colour cursor `col_at` rides along from slice to slice, and a short run is padded rather than misaligned.
- A streamed cloud keeps no kernel object (`display_only`, an empty `Session` shell): picking and editing would need the file decoded, which is exactly what streaming avoids.
