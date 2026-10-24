# 12.2 · Locate original arrays without downloading their bodies

**Goal:** Validate complete octrees before range streaming. · **Steps:** 10 · **Source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Octree Cell Subdivision: 8 Child AABBs (L0 -&gt; L1 -&gt; L2)</text>
  <rect x="68" y="48" width="108" height="108" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <line x1="122" y1="48" x2="122" y2="156" stroke="#111111" stroke-width="1.5"/>
  <line x1="68" y1="102" x2="176" y2="102" stroke="#111111" stroke-width="1.5"/>
  <rect x="122" y="48" width="54" height="54" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="149" y1="48" x2="149" y2="102" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="122" y1="75" x2="176" y2="75" stroke="#2563eb" stroke-width="1.2"/>
  <rect x="149" y="48" width="27" height="27" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sse = d_cell / z</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="110" text-anchor="middle" font-family="monospace" font-size="10" fill="#16a34a">sse > τ_split</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Frustum Cull + HTTP Range: bytes=offset..offset+len</text>
  <circle cx="432" cy="104" r="5" fill="#111111"/>
  <polygon points="432,104 562,50 562,156" fill="#fef9c3" fill-opacity="0.4" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <rect x="504" y="68" width="38" height="34" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="504" y="110" width="38" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <rect x="592" y="64" width="102" height="78" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="600" y="74" width="86" height="16" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.2"/>
  <rect x="600" y="96" width="86" height="16" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Progressive Multi-Resolution Point Cloud</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="832" y="58" width="96" height="90" fill="none" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <circle cx="860" cy="84" r="6" fill="#94a3b8"/>
  <circle cx="898" cy="116" r="6" fill="#94a3b8"/>
  <rect x="936" y="58" width="96" height="90" fill="none" stroke="#16a34a" stroke-width="1.75"/>
  <circle cx="956" cy="76" r="3.5" fill="#16a34a"/><circle cx="972" cy="76" r="3.5" fill="#16a34a"/><circle cx="988" cy="76" r="3.5" fill="#16a34a"/><circle cx="1004" cy="76" r="3.5" fill="#16a34a"/>
  <circle cx="956" cy="96" r="3.5" fill="#2563eb"/><circle cx="972" cy="96" r="3.5" fill="#2563eb"/><circle cx="988" cy="96" r="3.5" fill="#2563eb"/><circle cx="1004" cy="96" r="3.5" fill="#2563eb"/>
  <circle cx="956" cy="116" r="3.5" fill="#dc2626"/><circle cx="972" cy="116" r="3.5" fill="#dc2626"/><circle cx="988" cy="116" r="3.5" fill="#dc2626"/><circle cx="1004" cy="116" r="3.5" fill="#dc2626"/>
</svg>
</div>

## <a id="checkpoint-67-supply-checked-array-layout-readers"></a>1. Bound scalar headers and byte addresses

### 1. `src/stream.rs`

Create the file and type:

```rust
use crate::source_validate::varint;

/// Byte length of a scalar field of wire type `wire`.
fn skip_scalar(b: &[u8], i: usize, wire: u32) -> Option<usize> {
    match wire {
        0 => Some(varint(b, i)?.1),
        1 => Some(8),
        5 => Some(4),
        _ => None,
    }
}

/// True when the value fits an f32.
fn finite_float(value: f64) -> bool {
    value.is_finite() && (value as f32).is_finite()
}

/// `at + length` when it stays within `end`.
fn body_end(at: u64, length: u64, end: u64) -> Option<u64> {
    let next = at.checked_add(length)?;

    if next <= end {
        Some(next)
    } else {
        None
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod browser_publication;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod browser_publication;

pub mod stream;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-67sa-locate-original-cloud-and-sheet-array-fields"></a>2. Locate cloud and sheet array fields

### 1. `src/stream.rs`

Locate original cloud and sheet array fields.

**Locate this existing block:**

```rust
        None
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        None
    }
}

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
    let (field, wire) = (
        u32::try_from(tag >> 3).ok()?,
        (tag & 7) as u32,
    );

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

## <a id="checkpoint-67sb-convert-bounded-doubles-and-cache-metadata-windows"></a>3. Convert bounded doubles and cache metadata windows

### 1. `src/stream.rs`

**Locate this existing block:**

```rust
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    })
}

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
        self.bytes
            .get(start..start.checked_add(length)?)
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

## <a id="checkpoint-67sc-decode-packed-arrays-for-existing-gpu-owners"></a>4. Decode packed arrays for existing GPU owners

### 1. `src/stream.rs`

**Locate this existing block:**

```rust
        Some(length.max(64 * 1024).min(end - at))
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Some(length.max(64 * 1024).min(end - at))
    }
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
        out.push(if v.is_finite() {
            v
        } else {
            0.0
        });
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

## <a id="checkpoint-67sd-descend-bounded-original-protobuf-messages"></a>5. Descend bounded protobuf messages

### 1. `src/stream.rs`

Descend bounded original protobuf messages.

**Locate this existing block:**

```rust
    Some((out, i))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Some((out, i))
}

/// A skipped field longer than this is geometry, not a name.
const NAME_BYTES: u64 = 256;

/// Enter field `want` of the message at `at`; returns its end.
fn descend_message(
    head: &[u8],
    at: &mut usize,
    parent_end: Option<u64>,
    want: u32,
) -> Option<u64> {
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

## <a id="checkpoint-67se-find-coordinate-arrays-without-downloading-their-bodies"></a>6. Find coordinate arrays without downloading their bodies

### 1. `src/stream.rs`

**Locate this existing block:**

```rust
        *at = usize::try_from(next).ok()?;
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        *at = usize::try_from(next).ok()?;
    }
}

/// The `coords` field, which only small names may precede.
fn first_array(head: &[u8], mut at: usize, end: u64) -> Option<(u64, u64, u64)> {
    while (at as u64) < end {
        let (tag, used) = varint(head, at)?;
        at = at.checked_add(used)?;
        let (field, wire) = (
            u32::try_from(tag >> 3).ok()?,
            (tag & 7) as u32,
        );

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

## <a id="checkpoint-67sf-validate-sheet-side-array-lengths"></a>7. Validate sheet side-array lengths

### 1. `src/stream.rs`

**Locate this existing block:**

```rust
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    cloud_layout(head).is_none() && sheet_layout(head).is_none()
}

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

## <a id="checkpoint-67sg-collect-original-octree-node-columns"></a>8. Collect octree node columns

### 1. `src/stream.rs`

Collect original octree node columns.

**Locate this existing block:**

```rust
        true
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        true
    }
}

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

## <a id="checkpoint-67sh-check-octree-bounds-and-unique-child-ownership"></a>9. Check octree bounds and unique child ownership

### 1. `src/stream.rs`

**Locate this existing block:**

```rust
            _ => return false,
        }

        true
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            _ => return false,
        }

        true
    }
}

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

## <a id="checkpoint-67si-validate-complete-octrees-before-range-streaming"></a>10. Validate complete octrees before range streaming

### 1. `src/stream.rs`

Validate complete octrees before range streaming. Supply independent malformed-layout and original-source value fixtures.

**Locate this existing block:**

```rust
        self.size.is_empty()
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.size.is_empty()
    }
}

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
            if !self.valid_node(node, total) || !self.valid_children(node, &mut parents)
            {
                return false;
            }
        }

        !parents[1..].contains(&false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Normals pack per point and a zero normal stays 0.
    #[test]
    fn normals_pack_three_doubles_per_point() {
        let mut raw = Vec::new();

        for v in [0.0f64, 0.0, 1.0, 0.0, 0.0, 0.0] {
            raw.extend_from_slice(&v.to_le_bytes());
        }

        let packed = normals_from(&raw);
        assert_eq!(packed.len(), 2);
        assert_eq!(
            packed[0],
            crate::cloud_normals::oct16(&[0.0, 0.0, 1.0]).unwrap()
        );
        assert_eq!(packed[1], 0);
    }

    /// The window serves nearby reads from cache.
    #[test]
    fn metadata_window_checks_ranges_without_copying_or_following_large_skips() {
        let window = MetadataWindow {
            at: 100,
            bytes: vec![1, 2, 3, 4],
        };
        assert_eq!(window.slice(101, 2), Some(&[2, 3][..]));
        assert_eq!(window.slice(104, 0), Some(&[][..]));
        assert!(window.slice(99, 1).is_none());
        assert!(window.slice(103, 2).is_none());
        assert!(window.slice(u64::MAX, 2).is_none());
        assert_eq!(
            MetadataWindow::read_length(100, 64, 1_000_000_000),
            Some(65_536)
        );
        assert_eq!(
            MetadataWindow::read_length(100, 64, 170),
            Some(70)
        );
        assert_eq!(
            MetadataWindow::read_length(100, 70_000, 1_000_000),
            Some(70_000)
        );
        assert_eq!(
            MetadataWindow::read_length(100, 64, 150),
            None
        );
        assert_eq!(
            MetadataWindow::read_length(0, 64 * 1024 * 1024 + 1, u64::MAX),
            None
        );
        assert_eq!(
            MetadataWindow::read_length(u64::MAX - 1, 2, u64::MAX),
            None
        );
    }

    /// A two-node table.
    fn valid_lod() -> CloudLod {
        CloudLod {
            min: vec![0.0; 6],
            size: vec![10.0, 1.0],
            spacing: vec![1.0, 0.5],
            level: vec![0, 1],
            first: vec![0, 1],
            count: vec![1, 1],
            children: [vec![1], vec![-1; 15]].concat(),
        }
    }

    /// Bad node tables are refused.
    #[test]
    fn lod_parallel_arrays_bounds_ranges_and_child_graph_are_validated() {
        assert!(valid_lod().valid(2));
        let mut lod = valid_lod();
        lod.min.pop();
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.spacing[0] = f64::NAN;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.size[0] = f64::MAX;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.min[0] = f32::MAX as f64;
        lod.size[0] = f32::MAX as f64;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.first[1] = 2;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.count[1] = -1;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[0] = 2;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[8] = 0;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[0] = -1;
        assert!(!lod.valid(2));
        let mut lod = valid_lod();
        lod.children[1] = 1;
        assert!(!lod.valid(2));
    }

    /// Short or non-finite arrays are refused.
    #[test]
    fn packed_metadata_and_positions_reject_partial_or_nonfinite_values() {
        let mut lod = CloudLod::default();
        assert!(!lod.set_field(8, &[0; 7]));
        assert!(!lod.set_field(14, &[0x80]));
        let raw: Vec<_> = [1.0f64, 2.0, 3.0]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        assert_eq!(
            checked_positions(&raw, 1).unwrap(),
            [1.0, 2.0, 3.0]
        );
        assert!(checked_positions(&raw[..23], 1).is_none());
        assert!(checked_positions(&raw, 2).is_none());

        for bad in [f64::NAN, f64::INFINITY, f64::MAX] {
            let raw: Vec<_> = [bad, 0.0, 0.0]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .collect();
            assert!(checked_positions(&raw, 1).is_none());
        }
    }

    /// Ranges never overflow or leave the message.
    #[test]
    fn source_offsets_do_not_wrap_or_cross_the_enclosing_message() {
        assert!(bounded_range(0, 64 * 1024 * 1024));
        assert!(!bounded_range(0, 64 * 1024 * 1024 + 1));
        assert!(!bounded_range(u64::MAX - 1, 3));
        assert_eq!(body_end(u64::MAX - 1, 3, u64::MAX), None);
        assert_eq!(body_end(50, 51, 100), None);
        assert_eq!(body_end(50, 50, 100), Some(100));
        // a coords field longer than its message
        assert_eq!(
            walk_to_coords(&[0x1a, 4, 0x42, 2, 0x1a, 24]),
            None
        );
    }

    /// Varints of one and two bytes read back.
    #[test]
    fn varint_reads_one_and_two_byte_values() {
        assert_eq!(varint(&[0x05], 0), Some((5, 1)));
        assert_eq!(varint(&[0xac, 0x02], 0), Some((300, 2)));
        assert_eq!(varint(&[0x80], 0), None);
        assert_eq!(
            varint(
                &[
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1
                ],
                0
            ),
            Some((u64::MAX, 10))
        );
        assert_eq!(
            varint(
                &[
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2
                ],
                0
            ),
            None
        );
    }

    /// One protobuf varint.
    fn uvarint(mut v: u64) -> Vec<u8> {
        let mut out = Vec::new();

        loop {
            let byte = (v & 0x7f) as u8;
            v >>= 7;

            if v == 0 {
                out.push(byte);
                return out;
            }

            out.push(byte | 0x80);
        }
    }

    /// One length-delimited field: tag, length, body.
    fn bytes_field(field: u32, body: &[u8]) -> Vec<u8> {
        let mut out = uvarint(u64::from(field << 3 | 2));
        out.extend(uvarint(body.len() as u64));
        out.extend_from_slice(body);
        out
    }

    /// One varint field.
    fn uint_field(field: u32, value: u32) -> Vec<u8> {
        let mut out = uvarint(u64::from(field << 3));
        out.extend(uvarint(u64::from(value)));
        out
    }

    /// A one-segment sheet file.
    fn sheet_file() -> Vec<u8> {
        let coords: Vec<u8> = [0.0f64, 1.0, 2.0, 3.0, 4.0, 5.0]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .collect();
        let mut sheet = bytes_field(1, b"guid");
        sheet.extend(bytes_field(2, b"plan"));
        sheet.extend(bytes_field(3, &coords));
        sheet.extend(bytes_field(4, &0xff00_00ffu32.to_le_bytes()));
        sheet.extend(bytes_field(5, &0.35f32.to_le_bytes()));
        sheet.extend(uint_field(6, 1));
        sheet.extend(uint_field(7, 9));
        sheet.extend(bytes_field(8, b"plan.shm"));
        sheet.extend(bytes_field(15, &7u32.to_le_bytes()));
        let objects = bytes_field(17, &sheet);
        bytes_field(3, &objects)
    }

    /// Scan the fields after `coords` from memory.
    fn scan_tail(file: &[u8], fields: &mut SheetFields, mut at: u64) -> bool {
        while at < fields.end {
            let Some(f) = field_at(&file[at as usize..], at, fields.end) else {
                return false;
            };
            let body = if f.wire == 2 {
                &file[f.body as usize..(f.body + f.value.min(NAME_BYTES)) as usize]
            } else {
                &[]
            };

            if !fields.set(&f, body) {
                return false;
            }

            at = f.next;
        }

        true
    }

    /// The sheet walk finds every array.
    #[test]
    fn sheet_layout_walks_objects_field_17_and_locates_every_array() {
        let file = sheet_file();
        let (coords_at, coords_len, end) = sheet_layout(&file).unwrap();
        assert_eq!(coords_len, 48);
        assert_eq!(end, file.len() as u64);
        assert_eq!(
            packed_f64(&file[coords_at as usize..(coords_at + 48) as usize])[5],
            5.0
        );
        let mut fields = SheetFields {
            end,
            coords_at,
            coords_len,
            count: 1,
            ..Default::default()
        };
        assert!(scan_tail(
            &file,
            &mut fields,
            coords_at + coords_len
        ));
        assert_eq!(fields.colors_len, 4);
        assert_eq!(fields.widths_len, 4);
        assert_eq!(fields.ids_len, 4);
        assert_eq!(fields.entities, 9);
        assert_eq!(fields.meta, "plan.shm");
        assert_eq!(
            packed_u32(&file[fields.colors_at as usize..][..4]),
            [0xff00_00ff]
        );
        assert_eq!(
            packed_f32(&file[fields.widths_at as usize..][..4]),
            [0.35]
        );
        assert_eq!(
            packed_u32(&file[fields.ids_at as usize..][..4]),
            [7]
        );
        assert_eq!(fields.ids_at + 4, end);
        // a sheet is not a cloud, nor a file that loads whole; two objects are not one file
        assert!(walk_to_coords(&file).is_none());
        assert!(!plain(&file));
        assert!(!plain(&bytes_field(
            3,
            &bytes_field(8, &bytes_field(3, &[0; 24]))
        )));
        assert!(plain(&bytes_field(
            3,
            &bytes_field(1, b"mesh")
        )));
        let mut two = bytes_field(17, &file[4..]);
        two.extend(bytes_field(17, b""));
        assert!(sheet_layout(&bytes_field(3, &two)).is_none());
        // a wrong segment count is refused
        let mut wrong = fields.clone();
        wrong.count = 2;
        assert!(!scan_tail(
            &file,
            &mut wrong,
            coords_at + coords_len
        ));
    }

    /// Colours decode and report where they end.
    #[test]
    fn colors_decode_and_report_their_end() {
        let raw = [127u8, 0, 100, 127, 1, 2, 3, 4, 9, 9];
        let (c, used) = colors_from(&raw, 2).unwrap();
        assert_eq!(c, [0x7f64_007f, 0x0403_0201]);
        assert_eq!(used, 8);
    }
}
```

## Build & Run

![Recorded visual output for Lesson 12.2 (67si-validate-complete-octrees-before-range-streaming)](../../../screenshots/journey/67si-validate-complete-octrees-before-range-streaming.png)

```sh
npm --prefix ../session_tests run course -- dependencies 12.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.2
npm --prefix ../session_tests run course -- save 12.2
```

Continue with [12.3 · Bounded scheduling, exact replies and cancellation](../../consolidation/66za-own-original-range-read-permits.md).
