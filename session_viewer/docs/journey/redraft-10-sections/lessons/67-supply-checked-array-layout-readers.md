# 12.2 · Locate original arrays without downloading their bodies

**Section 7: Out-of-Core HTTP Range Streaming, Shared Instances, and Drawing Sheets** · **Checkpoints:** `67-supply-checked-array-layout-readers` → `67si-validate-complete-octrees-before-range-streaming` (10 steps) · **Production source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs) · [app/fetch.rs](../../../../src/app/fetch.rs) · [app/loader.rs](../../../../src/app/loader.rs) · [app/live.rs](../../../../src/app/live.rs)

> **Goal:** Validate complete octrees before range streaming.

## 1. Visual WebGPU Schematic

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. Glyph Atlas (R8)</text>
  <rect x="52" y="60" width="166" height="96" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="64" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="80" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">A</text>
  <rect x="104" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="120" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">B</text>
  <rect x="144" y="70" width="32" height="36" rx="3" fill="url(#hatch)" stroke="#111111" stroke-width="1.25"/><text x="160" y="93" text-anchor="middle" font-family="monospace" font-size="14" font-weight="700" fill="#111111">7</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">1px Padded Shelves</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. Plane vs Billboard</text>
  <polygon points="324,136 364,74 416,74 376,136" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="370" y="110" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3D</text>
  <rect x="436" y="78" width="68" height="54" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="470" y="109" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2D UI</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">text_plane vs text</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. matrix.wgsl</text>
  <rect x="644" y="64" width="120" height="90" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <line x1="684" y1="64" x2="684" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="724" y1="64" x2="724" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="644" y1="94" x2="764" y2="94" stroke="#111111" stroke-width="1"/>
  <line x1="644" y1="124" x2="764" y2="124" stroke="#111111" stroke-width="1"/>
  <rect x="644" y="64" width="40" height="30" fill="url(#hatch)"/><text x="664" y="83" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">1.0</text>
  <rect x="684" y="94" width="40" height="30" fill="url(#hatch)"/><text x="704" y="113" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">2.4</text>
  <rect x="724" y="124" width="40" height="30" fill="url(#hatch)"/><text x="744" y="143" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">1.0</text>
  <text x="704" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">0-Cells Blank</text>
</svg>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **HTTP / Fetch** | `Range: bytes=start-end` -> `206 Partial Content` | Fetches only the exact byte slice of an octree node or sheet column needed for the current camera view. |
| **Protobuf Wire** | Varint tag + length prefix scanner | Skips over multi-megabyte coordinate arrays by reading their byte range offsets without allocating their contents. |
| **Rust (Residency)** | `budget.reserve(bytes)?` -> LRU eviction | Guarantees resident + in-flight pages never exceed the memory cap; coarse root pages stay pinned so the scene is never blank. |
| **GPU Instancing** | Shared definition buffer + per-instance record | Draws N placed copies of a complex CAD component using 1 shared vertex/index allocation and N compact instance records. |

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-67-supply-checked-array-layout-readers"></a>

### Step 1 of 10 · Bound scalar headers and byte addresses (`67-supply-checked-array-layout-readers`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs, src/lib.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Reuse the varint decoder from 66b. A protobuf scalar has a wire-dependent byte length; byte addresses stay u64 until a bounded slice requires usize. checked_add must precede the end comparison. This is the boundary between an external file layout and later GPU-ready arrays.

> **Check:** *Why check addition before comparing the message end?* — An overflowing byte address could wrap into a small, apparently valid offset.

### 1. `src/stream.rs`

Bound scalar headers and byte addresses.

Create the file and type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-01.rs"
```

### 2. `src/lib.rs`

Register the source component.

After that block, type:

```rust
--8<-- "journey/code/67-supply-checked-array-layout-readers-02.rs" skip=60
```

**Result after `67-supply-checked-array-layout-readers`:** The cumulative browser application compiles. This parser preparation preserves the current drawing; actual range requests and streamed pages connect in the following lessons.

---

<a id="checkpoint-67sa-locate-original-cloud-and-sheet-array-fields"></a>

### Step 2 of 10 · Locate original cloud and sheet array fields (`67sa-locate-original-cloud-and-sheet-array-fields`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

CloudFields and SheetFields retain offsets into the original file rather than owned coordinate arrays.

Field records the parsed tag, body start and next address. Parse scalar fields with the helper from the preceding lesson; length-delimited bodies require a second checked address addition. ETags remain attached to the original layout so later ranges can reject mixed revisions.

> **Check:** *Does a field body offset point at its length prefix?* — No. The body begins after both the tag and the length varint.

### 1. `src/stream.rs`

Locate original cloud and sheet array fields.

After that block, type:

```rust

/// Byte positions of a cloud's arrays in its file.
#[derive(Clone, Debug)]
pub struct CloudFields {
    pub end: u64,                 // end of the cloud message
    pub coords_at: u64,           // start of the positions
    pub coords_len: u64,          // their length
    pub colors_at: u64,           // start of the colours
    pub colors_len: u64,          // their length
    pub normals_at: u64,          // start of the normals, 0 = none
    pub normals_len: u64,         // their length
    pub count: u32,               // points in the cloud
    pub ids_at: u64,              // start of the original ids, 0 = none
    pub ids_len: u64,             // their length
    pub revision: Option<String>, // file ETag every read must match
}

/// Byte positions of a sheet's arrays in its file; length 0 = absent.
#[derive(Clone, Debug, Default)]
pub struct SheetFields {
    pub end: u64,                 // end of the sheet message
    pub coords_at: u64,           // start of the segment ends
    pub coords_len: u64,          // their length
    pub colors_at: u64,           // start of the colours
    pub colors_len: u64,          // their length
    pub widths_at: u64,           // start of the pen widths
    pub widths_len: u64,          // their length
    pub ids_at: u64,              // start of the entity ids
    pub ids_len: u64,             // their length
    pub count: u32,               // segments in the sheet
    pub entities: u32,            // records in the side table
    pub meta: String,             // side table file name, empty = none
    pub revision: Option<String>, // file ETag every read must match
}

/// One protobuf field header.
#[derive(Clone, Copy)]
pub struct Field {
    pub field: u32, // field number
    pub wire: u32,  // wire type
    pub value: u64, // the varint value, or the body length
    pub body: u64,  // where the body starts
    pub next: u64,  // where the next field starts
}

/// Parse the field header at `at`; None when it runs past `end`.
pub fn field_at(header: &[u8], at: u64, end: u64) -> Option<Field> {
    let (tag, used) = varint(header, 0)?;
    let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);

    if field == 0 {
        return None;
    }

    let body = body_end(at, used as u64, end)?;

    if wire != 2 {
        let (value, _) = if wire == 0 {
            varint(header, used)?
        } else {
            (0, 0)
        };
        let skip = skip_scalar(header, used, wire)?;
        return Some(Field {
            field,
            wire,
            value,
            body,
            next: body_end(body, skip as u64, end)?,
        });
    }

    let (length, extra) = varint(header, used)?;
    let body = body_end(body, extra as u64, end)?;
    Some(Field {
        field,
        wire,
        value: length,
        body,
        next: body_end(body, length, end)?,
    })
}
```

---

<a id="checkpoint-67sb-convert-bounded-doubles-and-cache-metadata-windows"></a>

### Step 3 of 10 · Convert bounded doubles and cache metadata windows (`67sb-convert-bounded-doubles-and-cache-metadata-windows`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Keep original source precision separate from the f32 upload representation.

Require the exact declared byte count before converting doubles, then reject non-finite values in either representation. MetadataWindow serves a borrowed slice only when the complete range is cached. Its read length batches nearby small fields within 64 KiB and the enclosing message; it does not allocate a geometry-sized header.

> **Check:** *Why reject a finite f64 that becomes infinite as f32?* — The source value is valid as f64 but cannot be represented in the GPU coordinate format.

### 1. `src/stream.rs`

Convert bounded doubles and cache metadata windows.

After that block, type:

```rust

/// `count` xyz triples as f32; None when short or not finite.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_positions(raw: &[u8], count: u32) -> Option<Vec<f32>> {
    checked_doubles(raw, u64::from(count).checked_mul(3)?)
}

/// Exactly `n` doubles as f32; None when short or not finite.
#[cfg(any(target_arch = "wasm32", test))]
fn checked_doubles(raw: &[u8], n: u64) -> Option<Vec<f32>> {
    if raw.len() as u64 != n.checked_mul(8)? {
        return None;
    }

    let mut out = Vec::with_capacity(raw.len() / 8);

    for bytes in raw.chunks_exact(8) {
        let value = f64::from_le_bytes(bytes.try_into().ok()?);

        if !finite_float(value) {
            return None;
        }

        out.push(value as f32);
    }

    Some(out)
}

/// True when a range is small enough to read.
#[cfg(any(target_arch = "wasm32", test))]
fn bounded_range(at: u64, length: u64) -> bool {
    length <= 64 * 1024 * 1024 && at.checked_add(length).is_some()
}

/// A cached slice of the file's header bytes.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct MetadataWindow {
    at: u64,        // file position of `bytes[0]`
    bytes: Vec<u8>, // the cached bytes
}

#[cfg(any(target_arch = "wasm32", test))]
impl MetadataWindow {
    /// The cached bytes at `at`, if all present.
    fn slice(&self, at: u64, length: u64) -> Option<&[u8]> {
        let start = usize::try_from(at.checked_sub(self.at)?).ok()?;
        let length = usize::try_from(length).ok()?;
        self.bytes.get(start..start.checked_add(length)?)
    }

    /// How much to read at once: at least `length`, up to 64 KiB.
    fn read_length(at: u64, length: u64, end: u64) -> Option<u64> {
        if !bounded_range(at, length) {
            return None;
        }

        body_end(at, length, end)?;
        Some(length.max(64 * 1024).min(end - at))
    }
}
```

---

<a id="checkpoint-67sc-decode-packed-arrays-for-existing-gpu-owners"></a>

### Step 4 of 10 · Decode packed arrays for existing GPU owners (`67sc-decode-packed-arrays-for-existing-gpu-owners`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Read fixed-width arrays with explicit little-endian conversion and varints with the shared decoder.

These low-level readers are used after layout and length validation; they are not independent whole-file validators. Streamed positions become f32 upload triples while original bytes remain in the source. Normals reuse cloud_normals::oct16, and colours pack four byte channels into one u32.

> **Check:** *Does decoding normals require another normal encoding algorithm?* — No. Reuse the octahedral encoder already written for the cloud owner.

### 1. `src/stream.rs`

Decode packed arrays for existing GPU owners.

After that block, type:

```rust

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

/// A packed `fixed32` array in full.
pub fn packed_u32(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(raw.len() / 4);

    for c in raw.chunks_exact(4) {
        out.push(u32::from_le_bytes(c.try_into().unwrap()));
    }

    out
}

/// A packed `float` array in full; a non-finite value becomes 0.
pub fn packed_f32(raw: &[u8]) -> Vec<f32> {
    let mut out = Vec::with_capacity(raw.len() / 4);

    for c in raw.chunks_exact(4) {
        let v = f32::from_le_bytes(c.try_into().unwrap());
        out.push(if v.is_finite() { v } else { 0.0 });
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

/// Packed doubles, three per point, as octahedral normals; a degenerate one becomes 0.
pub fn normals_from(raw: &[u8]) -> Vec<u32> {
    let mut out = Vec::with_capacity(raw.len() / 24);

    for c in raw.chunks_exact(24) {
        let n = [
            f64::from_le_bytes(c[0..8].try_into().unwrap()),
            f64::from_le_bytes(c[8..16].try_into().unwrap()),
            f64::from_le_bytes(c[16..24].try_into().unwrap()),
        ];
        out.push(crate::cloud_normals::oct16(&n).unwrap_or(0));
    }

    out
}

/// `count` colours from packed varints, and where they ended.
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
```

---

<a id="checkpoint-67sd-descend-bounded-original-protobuf-messages"></a>

### Step 5 of 10 · Descend bounded original protobuf messages (`67sd-descend-bounded-original-protobuf-messages`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Walk nested length-delimited messages using parent-end constraints.

Small names can precede the target; a larger skipped body stops this prefix-only fast path. Reaching a target nested message requires its end to agree with the declared parent end. Failure means this specialized layout is unavailable, not that arbitrary source data can be accepted.

> **Check:** *Why refuse a large field before the expected geometry message?* — A bounded prefix reader cannot skip an arbitrary geometry body while claiming it has inspected the layout.

### 1. `src/stream.rs`

Descend bounded original protobuf messages.

After that block, type:

```rust

/// A skipped field longer than this is geometry, not a name.
const NAME_BYTES: u64 = 256;

/// Enter field `want` of the message at `at`; returns its end.
fn descend_message(head: &[u8], at: &mut usize, parent_end: Option<u64>, want: u32) -> Option<u64> {
    loop {
        let (tag, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, tag & 7);

        if field == 0 || wire != 2 {
            return None;
        }

        let (length, used) = varint(head, *at)?;
        *at = (*at).checked_add(used)?;
        let next = (*at as u64).checked_add(length)?;

        if let Some(end) = parent_end
            && next > end
        {
            return None;
        }

        if field == want {
            if let Some(end) = parent_end
                && end != next
            {
                return None;
            }

            return Some(next);
        }

        if length > NAME_BYTES {
            return None;
        }

        *at = usize::try_from(next).ok()?;
    }
}
```

---

<a id="checkpoint-67se-find-coordinate-arrays-without-downloading-their-bodies"></a>

### Step 6 of 10 · Find coordinate arrays without downloading their bodies (`67se-find-coordinate-arrays-without-downloading-their-bodies`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

After entering the cloud or sheet message, scan only the permitted leading fields.

The coordinates field supplies its source offset and length without reading the entire body. cloud_layout and sheet_layout use the same scanner; plain chooses the whole-file path when neither bounded specialization matches. A truncated varint cannot produce a guessed offset.

> **Check:** *How can a short prefix locate a much larger coordinate array?* — The field header declares the array length; its body does not need to be present in the prefix.

### 1. `src/stream.rs`

Find coordinate arrays without downloading their bodies.

After that block, type:

```rust

/// The `coords` field, which only small names may precede.
fn first_array(head: &[u8], mut at: usize, end: u64) -> Option<(u64, u64, u64)> {
    while (at as u64) < end {
        let (tag, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let (field, wire) = (u32::try_from(tag >> 3).ok()?, (tag & 7) as u32);

        if field == 0 {
            return None;
        }

        if wire != 2 {
            at = at.checked_add(skip_scalar(head, at, wire)?)?;

            if at as u64 > end {
                return None;
            }

            continue;
        }

        let (length, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let next = (at as u64).checked_add(length)?;

        if next > end {
            return None;
        }

        if field == 3 {
            return Some((at as u64, length, end));
        }

        if field > 3 || length > NAME_BYTES {
            return None;
        }

        at = usize::try_from(next).ok()?;
    }

    None
}

/// Start and length of a single cloud's `coords`; None otherwise.
pub fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
    let (at, length, _) = cloud_layout(head)?;
    Some((at, length))
}

/// Start, length of `coords` and end of the cloud message.
fn cloud_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 8)?;
    first_array(head, at, end)
}

/// Start, length of `coords` and end of the sheet message.
pub fn sheet_layout(head: &[u8]) -> Option<(u64, u64, u64)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 17)?;
    first_array(head, at, end)
}

/// True for a file whose first bytes show neither a cloud nor a sheet: it loads whole.
pub fn plain(head: &[u8]) -> bool {
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}
```

---

<a id="checkpoint-67sf-validate-sheet-side-array-lengths"></a>

### Step 7 of 10 · Validate sheet side-array lengths (`67sf-validate-sheet-side-array-lengths`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

SheetFields::set records each optional side array once.

Colours, widths and IDs must match the declared segment count, while metadata names remain bounded UTF-8. Reject duplicate known fields and wrong wire types; skip unknown fields to preserve forward compatibility. Actual sheet GPU drawing is connected at 71, using these original entity IDs.

> **Check:** *Why must a colour field match the segment count?* — Its fixed32 values index the same original segments as the coordinate array.

### 1. `src/stream.rs`

Validate sheet side-array lengths.

After that block, type:

```rust

impl SheetFields {
    /// Bytes per segment: six doubles.
    pub const SEGMENT_BYTES: u64 = 48;

    /// Record one field found after `coords`; false when it is wrong.
    pub fn set(&mut self, f: &Field, body: &[u8]) -> bool {
        let per_segment = u64::from(self.count) * 4;

        match (f.field, f.wire) {
            (4, 2) if self.colors_len == 0 && f.value == per_segment => {
                (self.colors_at, self.colors_len) = (f.body, f.value)
            }
            (5, 2) if self.widths_len == 0 && f.value == per_segment => {
                (self.widths_at, self.widths_len) = (f.body, f.value)
            }
            (15, 2) if self.ids_len == 0 && f.value == per_segment => {
                (self.ids_at, self.ids_len) = (f.body, f.value)
            }
            (6, 0) => return f.value == u64::from(self.count),
            (7, 0) => match u32::try_from(f.value) {
                Ok(entities) => self.entities = entities,
                Err(_) => return false,
            },
            (8, 2) if f.value <= NAME_BYTES => match std::str::from_utf8(body) {
                Ok(meta) => self.meta = meta.to_string(),
                Err(_) => return false,
            },
            (1..=8, _) | (15, _) => return false,
            _ => {}
        }

        true
    }
}
```

---

<a id="checkpoint-67sg-collect-original-octree-node-columns"></a>

### Step 8 of 10 · Collect original octree node columns (`67sg-collect-original-octree-node-columns`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Store the original octree as parallel columns.

Validate fixed-width alignment and every varint before adopting a field, then use the already-written packed readers. set_field only fills columns; it does not establish topology. The next two lessons verify node bounds and connected parent relationships before streamed level selection uses them.

> **Check:** *Why validate a packed varint column before decoding it?* — The permissive packed reader can stop at malformed input; the column must first be known complete.

### 1. `src/stream.rs`

Collect original octree node columns.

After that block, type:

```rust

/// A cloud's octree node table.
#[derive(Clone, Default)]
pub struct CloudLod {
    pub min: Vec<f64>,      // cube corner, three per node
    pub size: Vec<f64>,     // cube size per node
    pub spacing: Vec<f64>,  // point spacing per node
    pub level: Vec<i32>,    // depth per node
    pub first: Vec<i32>,    // first point per node
    pub count: Vec<i32>,    // points per node
    pub children: Vec<i32>, // eight child indices per node, -1 = none
}

impl CloudLod {
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.size.len()
    }

    /// Fill one array from a packed field; false when unknown.
    pub fn set_field(&mut self, field: usize, raw: &[u8]) -> bool {
        if (8..=10).contains(&field) && !raw.len().is_multiple_of(8) {
            return false;
        }

        if (11..=14).contains(&field) {
            let mut at = 0;

            while at < raw.len() {
                let Some((_, bytes)) = varint(raw, at) else {
                    return false;
                };
                at += bytes;
            }
        }

        match field {
            8 => self.min = packed_f64(raw),
            9 => self.size = packed_f64(raw),
            10 => self.spacing = packed_f64(raw),
            11 => self.level = packed_i32(raw),
            12 => self.first = packed_i32(raw),
            13 => self.count = packed_i32(raw),
            14 => self.children = packed_i32(raw),
            _ => return false,
        }

        true
    }
}
```

---

<a id="checkpoint-67sh-check-octree-bounds-and-unique-child-ownership"></a>

### Step 9 of 10 · Check octree bounds and unique child ownership (`67sh-check-octree-bounds-and-unique-child-ownership`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

A node must have representable finite bounds, nonnegative size and spacing, and a point interval inside the original cloud.

Child indices must be valid, unclaimed and exactly one level below their parent. Strictly increasing levels prevent cycles; the full-table check in the next lesson also rejects detached nodes. These checks protect later GPU page selection without rewriting original topology.

> **Check:** *Can two octree parents claim the same child?* — No. The shared parent bitmap rejects a second claim and the root cannot be a child.

### 1. `src/stream.rs`

Check octree bounds and unique child ownership.

After that block, type:

```rust

impl CloudLod {
    /// True when one node's values are sound.
    fn valid_node(&self, node: usize, total: u32) -> bool {
        let size = self.size[node];
        let spacing = self.spacing[node];

        if !finite_float(size)
            || size < 0.0
            || !finite_float(spacing)
            || spacing < 0.0
            || self.level[node] < 0
        {
            return false;
        }

        for &min in &self.min[node * 3..node * 3 + 3] {
            if !finite_float(min) || !finite_float(min + size) {
                return false;
            }
        }

        let (Ok(first), Ok(count)) = (
            u32::try_from(self.first[node]),
            u32::try_from(self.count[node]),
        ) else {
            return false;
        };
        let Some(end) = first.checked_add(count) else {
            return false;
        };
        end <= total
    }

    /// True when a node's children are one level down and unclaimed.
    fn valid_children(&self, node: usize, parents: &mut [bool]) -> bool {
        for &child in &self.children[node * 8..node * 8 + 8] {
            if child == -1 {
                continue;
            }

            let Ok(child) = usize::try_from(child) else {
                return false;
            };

            if child == 0
                || child >= parents.len()
                || parents[child]
                || self.level[node].checked_add(1) != Some(self.level[child])
            {
                return false;
            }

            parents[child] = true;
        }

        true
    }

    /// True when the file carried no octree.
    pub fn is_empty(&self) -> bool {
        self.size.is_empty()
    }
}
```

---

<a id="checkpoint-67si-validate-complete-octrees-before-range-streaming"></a>

### Step 10 of 10 · Validate complete octrees before range streaming (`67si-validate-complete-octrees-before-range-streaming`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/stream.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">existing GPU array owne…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">later range streaming</text>
</svg>

Check column lengths with checked multiplication before indexing.

Then validate each node and its children with one shared parent bitmap. A final parent check rejects disconnected nodes. The supplied fixtures exercise malformed headers, finite conversion, original IDs, sheet fields and hostile topology; browser range requests connect in the following lessons.

> **Check:** *Do valid individual nodes prove the table is connected?* — No. Every non-root node must also have exactly one valid parent.

### 1. `src/stream.rs`

Validate complete octrees before range streaming.

After that block, type:

```rust

impl CloudLod {
    /// True when every node and child index is sound.
    pub fn valid(&self, total: u32) -> bool {
        let n = self.len();
        let Some(triples) = n.checked_mul(3) else {
            return false;
        };
        let Some(children) = n.checked_mul(8) else {
            return false;
        };

        if n == 0
            || self.min.len() != triples
            || self.spacing.len() != n
            || self.level.len() != n
            || self.first.len() != n
            || self.count.len() != n
            || self.children.len() != children
            || self.level[0] != 0
        {
            return false;
        }

        let mut parents = vec![false; n];

        for node in 0..n {
            if !self.valid_node(node, total) || !self.valid_children(node, &mut parents) {
                return false;
            }
        }

        !parents[1..].contains(&false)
    }
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/stream.rs`

Supply independent malformed-layout and original-source value fixtures.

After that block, append this supplied code:

```text
--8<-- "journey/code/67si-validate-complete-octrees-before-range-streaming-02.rs" skip=500
```

---

## 3. Build, Verify & Save

```sh
npm --prefix ../session_tests run course -- dependencies 12.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.2
npm --prefix ../session_tests run course -- save 12.2
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../../recovery.md).

Continue with [12.3 · Bounded scheduling, exact replies and cancellation](../../consolidation/66za-own-original-range-read-permits.md).
