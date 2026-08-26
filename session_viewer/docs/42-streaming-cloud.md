# 42 Streaming cloud — HTTP Range in, GPU rows out

> Direct-path chain (36-42); every step below is replay-verified against a clean
> end-of-35 checkout carried through lesson [41](41-cloud-scenes.md), and the whole
> pipeline was then run live against the real files - the numbers in **Expected state**
> are measured, not estimated.

## Goal

Load a 411 MB, 13.8-million-point scan with a **bounded** wasm heap — a few tens of MB,
independent of how big the file is and how many files there are. The peak stops being a
function of the data.

After [37](37-cloud-memory.md) the 14M scan still peaks near a gigabyte, and no amount of
dropping things earlier fixes it: the decoded proto and the GPU rows have to coexist,
because prost builds the whole message before you can look at any of it. The only way past
that is to stop decoding the whole message.

## The two facts that make this possible

Walking the wire format of a real scan (`assets/pb/lidar_scan000.pb`, 114.8 MB):

```
Session.3 (Objects)              LEN 114,807,751
  Objects.8 (pointclouds)        LEN 114,807,696
    PointCloud.1 guid            LEN 36
    PointCloud.2 name            LEN 13
    PointCloud.3 coords          LEN 87,570,576   packed double
    PointCloud.4 colors          LEN 27,237,048   packed uint32 (varint)
    PointCloud.6 point_size      fixed64
```

**One: every hop is wire type 2**, length-delimited, so all the headers we need sit in the
first ~170 bytes. Reaching `coords` is a loop over three length prefixes, skipping
everything else by its own length. No decoding.

**Two: `coords` is a packed `double`** — a fixed 8 bytes an element. So its length prefix
gives the exact point count *before a byte of payload is read*:

```
87,570,576 / 24 = 3,648,774 points
```

That second fact is the one that matters. Knowing the count up front removes **every**
reallocation: both GPU buffers are sized once, exactly, and every slice afterwards lands at
a known offset. There is no growth mid-cloud and no `reserve` to get wrong.

## Range requests, not a push stream

The obvious design is `fetch().body().getReader()` and a state machine over the chunks. Do
not build that. Its entire risk surface — chunks splitting an 8-byte double, splitting a
multi-byte varint, splitting a length header, and nested byte-budget bookkeeping across all
of it — exists *only because the data is pushed at you*.

Pull it instead:

```
  1. Range 0-8191          -> walk three headers in a contiguous buffer. No state machine.
  2. coords_len / 24       -> exact count -> size both GPU buffers, once.
  3. Range, 8 MB slices    -> aligned to 24 B, so a slice CANNOT split a point.
  4. Range, colours whole  -> 27 MB, decoded in one pass. No split varints, ever.
```

Each slice arrives complete, converts to f32, goes to the GPU, and dies. Peak wasm heap is
one slice plus the colour run.

**One hard prerequisite.** A server that does not implement `Range` **ignores the header
and returns `200` with the whole body** — which on a 411 MB scan is silent and
catastrophic. So the fetch refuses anything but `206`. `trunk serve` (axum +
`tower-http::ServeDir`) does ranges:

```
$ curl -s -D- -o /dev/null -H "Range: bytes=0-99" http://localhost:8770/pb/lidar_scan000.pb
HTTP/1.1 206 Partial Content
accept-ranges: bytes
content-range: bytes 0-99/114808149
```

`docs/serve.py` is `SimpleHTTPRequestHandler` and does **not**. If you ever serve the `.pb`
assets from it, this path must fail loudly rather than quietly download everything.

## Why the stream lane is its OWN lane

There is one design decision to make before any code: where do streamed rows LAND?

The walked lane cannot take them. `set_scene` rebuilds `point_buffer`/`point_col_buffer`/
`point_nrm_buffer` WHOLE from the Scene tables on every upload — that is lesson 36's
contract, and every later file append re-runs it. A streamed cloud has no rows in those
tables (that is the whole point), so the first document that loads after it would rebuild
the buffers without it and the cloud would vanish.

So streamed clouds get their own three buffers, their own draw list and their own record
table — and the two lanes MEET in the shared per-pixel depth/colour buffers, because
atomics compose across dispatches: both lanes' `cs_depth` passes contest the same
`atomicMax` race, and the resolve triangle never knows there were two. The cost is a
second bind-group pair and a second dispatch; the payoff is that nothing about lessons
36-41 changes underneath, and `rebuild` preserves streamed clouds for free because
`set_scene` never touches their buffers.

## Files we touch

| file | change |
|---|---|
| `Cargo.toml` | `web-sys` gains `"Headers"` |
| `src/app/persistence.rs` | `varint`, `walk_to_coords`, `fetch_range(_start/_finish)`, `positions_from`, `cloud_fields`, `cloud_colors`; `next_tick` goes `pub` |
| `src/engine/gpu/mod.rs` | the stream lane: buffers, `stream_reserve`, `cloud_begin`/`cloud_pos`/`cloud_col`, `grow_scene`; `splat_records` factored out; two-lane dispatch; `set_scene` keeps a finite box |
| `src/app/scene.rs` | `Item.stream`, `CloudSlot`, `begin_cloud`, `grow_bounds`; `rebuild` preserves streamed clouds |
| `src/lib.rs` | four `Msg` variants, GPU-first boot, the streaming branch, four handler arms |
| `assets/scenes/*.json` | `"stream": true` on the scan items |

---

## Step 1 — `Cargo.toml`

Find the `web-sys` feature list and add `"Headers",` after `"ImageData",`. Setting a
`Range` header needs it.

## Step 2 — the reader: `src/app/persistence.rs`

Two edits. First, the loader will yield between slices, so **find** `async fn next_tick()`
and make it `pub async fn next_tick()`.

Then **append at the bottom of the file** the whole streaming block:

```rust
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
fn varint(b: &[u8], mut i: usize) -> Option<(u64, usize)> {
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
fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> {
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
pub fn fetch_range_start(url: &str, start: u64, len: u64) -> Result<Fetch, JsValue> {
    let headers = Headers::new()?;
    headers.set("Range", &format!("bytes={}-{}", start, start + len - 1))?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
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
/// with the WHOLE body, which for a 411 MB scan would be catastrophic and silent.
/// `trunk serve` (axum + tower-http) does support ranges; `docs/serve.py`
/// (SimpleHTTPRequestHandler) does NOT.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, JsValue> {
    fetch_range_finish(fetch_range_start(url, start, len)?).await
}

/// Locate both packed arrays with two small reads: one at the head for `coords`, then one at
/// the end of the coords payload, where the `colors` header must be.
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
    Some(out)
}
```

`cloud_fields` does it in **two small reads**: 8 KB at the head for `coords`, then 16 bytes
at `coords_at + coords_len`, which is exactly where the `colors` header must be.

`positions_from` converts `f64 → f32` straight out of the slice. Positions are packed
little-endian doubles, bit-identical to a `&[f64]` on any LE target, so there is no
per-element *decode* — only the narrowing cast.

`cloud_colors` reads the whole colour run in one piece because packed `uint32` is
**varint**, not memcpy-able the way `coords` is. Values 0-255 encode as 1-2 bytes each,
four per point; one contiguous read buys complete freedom from split-varint handling for
noise-level memory (26 MB against the coords' 84 MB).

## Step 3 — the GPU side: `src/engine/gpu/mod.rs`

**3a — fields.** Find `pub point_count: u32,` and add below it:

```rust
    // The STREAM lane: clouds whose points never existed on the CPU. Their own three buffers
    // and record table - the walked lane above is rebuilt whole by every set_scene, so a
    // streamed cloud cannot live in it. The two lanes meet in the shared per-pixel
    // depth/colour buffers: atomics compose across dispatches.
    stream_pos_buf: wgpu::Buffer,
    stream_col_buf: wgpu::Buffer,
    stream_nrm_buf: wgpu::Buffer,
    stream_capacity: u64, // rows
    stream_count: u32,
    stream_pos_at: u32,
    stream_col_at: u32,
    pub stream_draws: Vec<(u32, u32, u32, f32)>, // (first, count, instance, spacing)
    splat_stream_recs: wgpu::Buffer,
    splat_group0_stream: wgpu::BindGroup,
    splat_group1_stream: wgpu::BindGroup,
```

**3b — construction.** In `Gpu::new`, **find** the line that starts the NEXT group's
construction (anchoring past `mk_splat_group1`'s call, which you may have wrapped over several
lines):

```rust
        let splat_resolve_group = Self::mk_splat_resolve_group(
```

**Add above it:**

```rust
        // stream lane: same layouts, its own buffers; grown for real by stream_reserve
        let stream_usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stream_pos_buf = zeroed_buffer(&device, "stream.pos", 12, stream_usage);
        let stream_col_buf = zeroed_buffer(&device, "stream.col", 4, stream_usage);
        let stream_nrm_buf = zeroed_buffer(&device, "stream.nrm", 4, stream_usage);
        let splat_stream_recs = zeroed_buffer(&device, "splat.stream.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0_stream = Self::mk_splat_group0(&device, &splat_group0_layout, &mvp_buffer, &cloud_buffer, &instance_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&device, &splat_group1_layout, &stream_pos_buf, &stream_col_buf, &splat_depth_buf, &splat_color_buf, &stream_nrm_buf);
```

and the matching struct-literal lines after `splat_state: None,`:

```rust
            stream_pos_buf,
            stream_col_buf,
            stream_nrm_buf,
            stream_capacity: 1,
            stream_count: 0,
            stream_pos_at: 0,
            stream_col_at: 0,
            stream_draws: Vec::new(),
            splat_stream_recs,
            splat_group0_stream,
            splat_group1_stream,
```

**3c — group rebuilds.** `rebuild_splat_groups` recreates bind groups whenever a bound
buffer is replaced — the stream lane's groups now belong in it too (the pixel buffers they
bind are recreated on every resize). **Add** to its body, before the resolve-group line:

```rust
        self.splat_group0_stream = Self::mk_splat_group0(&self.device, &self.splat_group0_layout, &self.mvp_buffer, &self.cloud_buffer, &self.instance_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.device, &self.splat_group1_layout, &self.stream_pos_buf, &self.stream_col_buf, &self.splat_depth_buf, &self.splat_color_buf, &self.stream_nrm_buf);
```

**3d — the grow API.** Add to `impl Gpu`, right after `rebuild_splat_groups`:

```rust
    /// Make room for `need` stream rows total, copying the live prefix GPU-side.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste over a
    /// hundred MB on a multi-scan scene AND worsen the worst transient (old+new live at once).
    /// What doubling avoids is a GPU-side copy - the one thing here that never touches wasm.
    fn stream_reserve(&mut self, need: u64) {
        if need <= self.stream_capacity { return }
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&self.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&self.device, "stream.nrm", cap * 4, usage);
        if self.stream_count > 0 {
            let mut enc = self.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.stream_pos_buf, 0, &pos, 0, self.stream_count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.stream_col_buf, 0, &col, 0, self.stream_count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.stream_nrm_buf, 0, &nrm, 0, self.stream_count as u64 * 4);
            self.queue.submit([enc.finish()]);
        }
        // The wire has no normals, and a zeroed buffer is NOT "no normal" - oct code 0 decodes
        // to a real direction. Fill the new region with the sentinel, in 1M-row slabs so the
        // staging cost stays bounded.
        let fill = vec![u32::MAX; 1 << 20];
        let mut at = self.stream_count as u64;
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            self.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            self.queue.submit([]);
            at += n as u64;
        }
        self.stream_pos_buf = pos;
        self.stream_col_buf = col;
        self.stream_nrm_buf = nrm;
        self.stream_capacity = cap;
        self.rebuild_splat_groups();
        self.splat_state = None;
    }

    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so all three buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        self.stream_reserve(self.stream_count as u64 + count as u64);
        self.stream_draws.push((self.stream_count, count, instance, 0.0));
        self.stream_pos_at = self.stream_count;
        self.stream_col_at = self.stream_count;
        self.stream_count += count;
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 40's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        if let Some(d) = self.stream_draws.last_mut() {
            if d.3 == 0.0 && self.stream_pos_at == d.0 && pos.len() >= 6 {
                let n = (pos.len() / 3).min(2048);
                let mut gaps: Vec<f32> = (1..n).map(|i| {
                    let (a, b) = ((i - 1) * 3, i * 3);
                    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
                }).filter(|g| *g > 0.0).collect();
                if !gaps.is_empty() {
                    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    d.3 = gaps[gaps.len() / 2];
                }
            }
        }
        self.queue.write_buffer(&self.stream_pos_buf, self.stream_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.stream_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.queue.submit([]);
        self.splat_state = None; // new points - the splat buffers are stale
    }

    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.queue.write_buffer(&self.stream_col_buf, self.stream_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.stream_col_at += col.len() as u32;
        self.queue.submit([]);
        self.splat_state = None;
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, min: [f32; 3], max: [f32; 3]) {
        if !min[0].is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.scene_min[0] >= self.scene_max[0] {
            self.scene_min = min;
            self.scene_max = max;
            return;
        }
        for k in 0..3 {
            self.scene_min[k] = self.scene_min[k].min(min[k]);
            self.scene_max[k] = self.scene_max[k].max(max[k]);
        }
    }
```

Three details earn their comments. The **exact** (not doubling) reserve: appends here are
few and huge, so doubling wastes buffer AND worsens the old+new transient. The
**`u32::MAX` normal fill**: a zeroed buffer is NOT "no normal" — oct code 0 decodes to a
real direction and lesson [40](40-potree-look.md)'s lambert would shade the scan with it.
And the empty **`queue.submit([])`** after every slice write: it ticks Dawn's serial so
the upload staging ring recycles; without it the GPU process accumulates the whole 165 MB
upload a second time.

`cloud_pos` also does the one thing a streamed cloud cannot get from the kernel walk: it
measures the point spacing (lesson 40's attenuation input) from the FIRST slice — median
consecutive distance, scan order being surface order, exactly `cloud_spacing`'s trick.

**3e — two-lane dispatch.** The record builder moves out of `encode_frame` so both draw
lists can use it. **Cut** the whole record loop (from `let mut header = [0u32; 4];` down
to `header[1] = cum;`) plus the attenuation comment above it, and paste it as a method
before `encode_frame`, generalised over `draws`:

```rust
    /// Build the record table for one cloud lane. A record folds the cloud's whole per-frame
    /// state: mvp x rebased model as ONE matrix, the tint, the attenuation constant and the
    /// model rotation - so a thread does one mat-vec, no instance fetch.
    /// Attenuated (world-sized) dots, Potree-style: the record carries k such that the
    /// shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its own
    /// world-space footprint, so near surfaces close up gap-free and far points shrink.
    /// The manifest px is a size FACTOR on the measured spacing.
    fn splat_records(&self, draws: &[(u32, u32, u32, f32)]) -> ([u32; 4], Vec<u8>, u32) {
        let mut header = [0u32; 4];
        let mut recs: Vec<u8> = Vec::new();
        let mut cum = 0u32;
        let ortho_h = self.last_ortho_h as f64;
        for &(first, count, inst, spacing) in draws {
            let Some(row) = self.instances.get(inst as usize) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 { continue; }
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.cloud_size;
            if px > 0.0 && header[0] < 256 {
                // column-major 4x4: combined = mvp x model
                let (a, b) = (&self.mvp_f32, &row.model);
                let mut m = [0.0f32; 16];
                for col in 0..4 {
                    for r in 0..4 {
                        m[col * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[col * 4 + k]).sum();
                    }
                }
                recs.extend_from_slice(bytemuck::cast_slice(&m));
                // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
                // floor, attenuation turns distant clouds to dust - Potree avoids that with
                // octree LOD (far nodes have bigger spacing); we keep the user's px instead.
                let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
                recs.extend_from_slice(bytemuck::cast_slice(&tint));
                // world radius = spacing x (px/6): manifest 6 ~ a full spacing of radius,
                // 3 ~ half. k folds the projection so the shader only divides by clip.w:
                //   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
                //   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
                // spacing was measured in the cloud's LOCAL units; the model may scale -
                // col0's length is that scale, so the footprint reaches world units first.
                let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();
                let world_r = (spacing as f64).max(1.0e-9) * mscale * 0.001 * (px as f64) / 6.0; // metres
                let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h) }
                        else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
                recs.extend_from_slice(bytemuck::cast_slice(&[first, count, cum, (k as f32).to_bits()]));
                // the MODEL rotation columns (translation-free), so a cloud with
                // normals can rotate them into world space for the lambert term
                let b = &row.model;
                recs.extend_from_slice(bytemuck::cast_slice(&[
                    b[0], b[1], b[2], 0.0f32,
                    b[4], b[5], b[6], 0.0,
                    b[8], b[9], b[10], 0.0,
                ]));
                header[0] += 1;
                cum += count;
            }
        }
        header[1] = cum;
        (header, recs, cum)
    }
```

Then the compute prelude at the top of `encode_frame` becomes:

```rust
        // Splat the clouds by COMPUTE before the render pass. One thread per point,
        // twice (depth race, then colour claim); the render pass composites the result
        // with one fullscreen triangle. TWO record sets - the walked lane and the stream
        // lane bind different point buffers - but one pixel buffer pair: atomics compose
        // across dispatches, so both lanes contest the same per-pixel depth race.
        {
            let (header, recs, cum) = self.splat_records(&self.cloud_draws);
            let (header_s, recs_s, cum_s) = self.splat_records(&self.stream_draws);
            self.splat_total = cum + cum_s;
            // Static skip: camera still, same scale, nothing rebuilt - the buffers already
            // hold this exact frame's splats, so the whole compute prelude is free.
            let state = (self.mvp_f32, self.cloud_size);
            if self.splat_total > 0 && self.splat_state != Some(state) {
                self.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.queue.write_buffer(&self.splat_recs, 16, &recs);
                self.queue.write_buffer(&self.splat_stream_recs, 0, bytemuck::bytes_of(&header_s));
                self.queue.write_buffer(&self.splat_stream_recs, 16, &recs_s);
                encoder.clear_buffer(&self.splat_depth_buf, 0, None); // 0 bits = reverse-Z far = empty
                encoder.clear_buffer(&self.splat_color_buf, 0, None);
                // 2D grid: a 1D dispatch caps at 65535 workgroups (~4.2M threads) and an
                // oversized dispatch invalidates the WHOLE command buffer - the frame
                // silently never draws. 4096-wide rows cover any point count.
                let grid = |n: u32| { let g = n.div_ceil(64); (g.min(4096), g.div_ceil(4096)) };
                let ((gx, gy), (sx, sy)) = (grid(cum), grid(cum_s));
                let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                // BOTH lanes' depth races must settle before EITHER lane claims colours -
                // dispatches in one pass are ordered, so lane order inside each phase is free.
                cp.set_pipeline(&self.splat_depth_pipeline);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat_group0, &[]);
                    cp.set_bind_group(1, &self.splat_group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat_group0_stream, &[]);
                    cp.set_bind_group(1, &self.splat_group1_stream, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                cp.set_pipeline(&self.splat_color_pipeline);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat_group0, &[]);
                    cp.set_bind_group(1, &self.splat_group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat_group0_stream, &[]);
                    cp.set_bind_group(1, &self.splat_group1_stream, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                self.splat_state = Some(state);
            }
        }
```

The ordering comment is the correctness core: `cs_color` claims a pixel by comparing its
own depth against the stored winner, so BOTH lanes' depth dispatches must land before
EITHER lane's colour dispatch — otherwise a walked point could claim a pixel that a
streamed point is about to win. Dispatches in one compute pass are ordered with memory
visibility, so the depth-depth-colour-colour sequence is all it takes.

**3f — the box guard.** A streamed cloud's instance row arrives through `set_scene` in an
upload whose walk tables are empty — `up.min` still infinite — and the State now boots
before any file at all. **Find** in `set_scene`:

```rust
        self.scene_min = up.min;
        self.scene_max = up.max;
```

**Replace with:**

```rust
        if up.min[0].is_finite() { // an empty upload (the State boots before any file) knows no box
            self.scene_min = up.min;
            self.scene_max = up.max;
        }
```

## Step 4 — the document row: `src/app/scene.rs`

**4a — the manifest flag.** Streaming is OPT-IN per item, not sniffed from the file: the
lion's .pb is also a single-cloud file, and the wire walk would happily stream it — right
past its NORMALS, which only the prost path decodes. **Find** in `Item`:

```rust
    pub point_size: f64,              // raw-cloud px for this file; 0 = keep the pb'own
```

**Add below it** (the field only — do NOT close the struct here; `Item` has more fields
after it):

```rust
    #[serde(default)]
    pub stream: bool,                 // Range-stream this file's cloud instead of parsing it
```

**4b — the slot.** A streamed cloud has no `Session` and no `Doc`. **Add above
`pub struct Scene`:**

```rust
/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This struct is the ENTIRE CPU-side footprint of a 13.8M-point
/// scan - a name, a placement, a count, a point size and the instance row it draws with.
pub struct CloudSlot {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub px: f32,
    pub instance: u32,
}
```

Give `Scene` the list — after `pub docs: Vec<Doc>,` add
`pub clouds: Vec<CloudSlot>,` with the matching `clouds: Vec::new(),` in `Scene::new()`.

**4c — the row.** Add to `impl Scene`, above `rebuild`:

```rust
    /// Widen the shared walk box by a streamed cloud's world AABB. Without this the box lives
    /// only in `Gpu` and the next `set_scene` from a real walk would replace it.
    pub fn grow_bounds(&mut self, min: [f32; 3], max: [f32; 3]) {
        for k in 0..3 {
            self.tables.min[k] = self.tables.min[k].min(min[k]);
            self.tables.max[k] = self.tables.max[k].max(max[k]);
        }
    }

    /// Reserve the document row for a cloud that is about to stream in. Called before a single
    /// point has been fetched: the count comes from the file's packed-double length prefix.
    /// Returns the instance row the streamed points will draw against.
    pub fn begin_cloud(&mut self, name: String, place: Xform, count: u32, px: f32) -> u32 {
        let row = self.tables.objects.len() as u32;
        self.tables.objects.push((place.clone(), [1.0; 4], 0));
        self.tables.object_bounds.push(None);
        self.tables.object_spacing.push(px); // the manifest px rides the spacing row, like the walk's clouds
        // Keep the row bookkeeping aligned - `order` is indexed by row everywhere else.
        let guid = format!("cloud:{name}");
        self.guid_to_row.insert(guid.clone(), row);
        self.order.push(guid);
        self.clouds.push(CloudSlot { name, place, count, px, instance: row });
        row
    }
```

The `object_bounds`/`object_spacing` pushes are the same row-alignment rule every walk arm
follows, and the spacing row carries the manifest px exactly like the walked clouds — the
record builder reads `row.spacing` for both lanes without knowing which is which.

**4d — rebuild preserves clouds.** In `rebuild`, **find**
`let docs = std::mem::take(&mut self.docs);` and add below it
`let clouds = std::mem::take(&mut self.clouds);`. Then after the `for d in docs { … }`
loop, **add:**

```rust
        // Clouds keep their GPU rows; only the instance they draw against is re-issued, and
        // the Gpu's stream draw list is patched to match. Order is preserved on both sides,
        // so index i here is index i there.
        for (i, c) in clouds.into_iter().enumerate() {
            let row = self.begin_cloud(c.name, c.place, c.count, c.px);
            if let Some(d) = gpu.stream_draws.get_mut(i) {
                d.2 = row;
            }
        }
```

## Step 5 — the loader: `src/lib.rs`

**5a — the variants.** The `Msg` enum becomes:

```rust
pub enum Msg {
    Ready(Box<State>),
    File(String, session_rust::Session, session_rust::Xform, f32),
    CloudBegin(String, session_rust::Xform, u32, f32),
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    CloudEnd([f32; 3], [f32; 3]),
}
```

**5b — GPU first, empty.** One structural change: **the GPU and the canvas come up FIRST,
empty.** A streamed cloud writes into GPU buffers, so the GPU has to exist before the
first byte of geometry is fetched — and as a bonus the viewport is live immediately
instead of after the first parse. In `resumed`'s `spawn_local`, after the manifest parse
and `let count = ...`, **replace** the `let mut next = ...` / `let mut sent_ready` boot
with:

```rust
                // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into
                // GPU buffers, so the GPU has to exist before the first byte of geometry is
                // fetched - and as a bonus the viewport is live immediately, not after a parse.
                let state = State::new(window.clone(), Scene::new()).await.expect("State init failed");
                log::info!("canvas live {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                let _ = proxy.send_event(Msg::Ready(Box::new(state)));

                // whole-file prefetch skips `stream` items - starting a plain GET on a 431 MB
                // scan would pull the entire body
                let prefetch = |it: &crate::app::scene::Item| (!it.stream).then(|| persistence::fetch_start(&it.file));
                let mut next = manifest.items.first().and_then(prefetch);
```

and inside the loop `next = manifest.items.get(i + 1).and_then(prefetch);` — the window-2
fetch-ahead SURVIVES, it just skips `stream` items: eagerly starting a plain GET on a
431 MB scan would pull the entire body, which is exactly what this lesson exists to
prevent. The old `if !sent_ready { ... }` branch collapses to a plain
`Msg::File(...)` send — `Ready` went out before the loop.

**5c — the streaming branch**, at the top of the loop body (before the whole-file fetch):

```rust
                    // ── STREAMING PATH ─────────────────────────────────────────────────
                    // A `stream` cloud never becomes a kernel object and never exists whole
                    // in wasm memory. Two small Range reads find the packed arrays; the
                    // coords run then arrives in 8 MB slices, each converted, handed to the
                    // GPU and dropped.
                    if item.stream {
                        let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
                        let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };
                        let Some(f) = persistence::cloud_fields(&item.file).await else {
                            log::warn!("'{}': stream requested but no Range-addressable cloud found - skipped", named);
                            continue;
                        };
                        log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
                            named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
                        let _ = proxy.send_event(Msg::CloudBegin(named.clone(), place, f.count, item.point_size as f32));

                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
                        let (mut at, mut left) = (f.coords_at, f.coords_len);
                        let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);

                        // PIPELINED, and this is the whole performance story of the loader:
                        // `fetch_range(..).await` resolves off network I/O, so it cannot
                        // resume until the current FRAME is done - a sequential loop pays one
                        // frame per slice. Keeping slice n+1 in flight while slice n converts
                        // hides the round trip AND the frame behind work we had to do anyway.
                        let mut inflight = if left > 0 {
                            persistence::fetch_range_start(&item.file, at, SLICE.min(left)).ok()
                        } else {
                            None
                        };
                        while let Some(f_in) = inflight.take() {
                            let n = SLICE.min(left);
                            at += n;
                            left -= n;
                            // next one on the wire BEFORE we spend time on this one
                            inflight = if left > 0 {
                                persistence::fetch_range_start(&item.file, at, SLICE.min(left)).ok()
                            } else {
                                None
                            };
                            let Ok(raw) = persistence::fetch_range_finish(f_in).await else { break };
                            let pos = persistence::positions_from(&raw);
                            drop(raw);
                            for q in pos.chunks_exact(3) {
                                for k in 0..3 { lo[k] = lo[k].min(q[k]); hi[k] = hi[k].max(q[k]); }
                            }
                            let _ = proxy.send_event(Msg::CloudPos(pos));
                            // A real macrotask between slices. With a warm cache the fetch
                            // promises resolve as MICROtasks, which never let the browser paint -
                            // the same freeze the sliced prost parse exists to avoid.
                            persistence::next_tick().await;
                        }
                        if let Some(col) = persistence::cloud_colors(&item.file, f.colors_at, f.colors_len, f.count).await {
                            let _ = proxy.send_event(Msg::CloudCol(col));
                        }
                        let _ = proxy.send_event(Msg::CloudEnd(lo, hi));
                        log::info!("streamed '{}' in {:.0}ms", named, crate::engine::performance::now_ms() - f0);
                        continue;
                    }
```

`next_tick()` matters more than it looks. With a warm cache the range requests resolve as
**microtasks**, and a microtask never lets the browser paint — the exact freeze that the
sliced prost parse exists to avoid, reintroduced by a different route.

**5d — the fit flag.** `App` gains a field — after `ctrl: bool,` add

```rust
    fitted: bool, // first geometry fits the camera; everything later only grows the extent
```

(init `fitted: false,` in the constructor). The `Ready` arm no longer fits — the scene is
empty at boot. **Find** in it:

```rust
                state.resize(w, h);
                let aspect = w as f64 / h as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
```

**Replace with:**

```rust
                state.resize(w, h);
                state.window.request_redraw(); // the scene is still empty - the first file fits
```

And in the `Msg::File` arm, **find** the
`state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);` line
(lesson [38](38-big-scenes.md)) and **replace with:**

```rust
                if self.fitted {
                    state.camera.grow_extent(state.gpu.scene_min, state.gpu.scene_max);
                } else {
                    let s = state.window.inner_size();
                    let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                    state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                    self.fitted = true;
                }
```

**5e — the handlers.** Four new arms, after the `Msg::File` arm:

```rust
            // A cloud, streaming. Nothing here holds points: begin_cloud reserves the GPU
            // range from a count that is already known, each slice is written and dropped,
            // and the CPU keeps a name, a count and one instance row.
            Msg::CloudBegin(name, place, count, px) => {
                let Some(state) = &mut self.state else { return };
                let row = state.scene.begin_cloud(name, place, count, px);
                state.scene.upload_to(&mut state.gpu); // pushes the instance row
                state.gpu.cloud_begin(count, row);
            }
            Msg::CloudPos(pos) => {
                let Some(state) = &mut self.state else { return };
                state.gpu.cloud_pos(&pos);
                state.window.request_redraw(); // the cloud grows on screen as it arrives
            }
            Msg::CloudCol(col) => {
                let Some(state) = &mut self.state else { return };
                state.gpu.cloud_col(&col);
                state.window.request_redraw();
            }
            Msg::CloudEnd(lo, hi) => {
                let Some(state) = &mut self.state else { return };
                // lo/hi are the cloud's LOCAL box; place it before it can fit the camera.
                if let Some(slot) = state.scene.clouds.last() {
                    let (mut wlo, mut whi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
                    for c in 0..8u32 {
                        let corner = [
                            if c & 1 == 0 { lo[0] } else { hi[0] },
                            if c & 2 == 0 { lo[1] } else { hi[1] },
                            if c & 4 == 0 { lo[2] } else { hi[2] },
                        ];
                        let w = crate::app::scene::xform_point(&slot.place, corner);
                        for k in 0..3 { wlo[k] = wlo[k].min(w[k]); whi[k] = whi[k].max(w[k]); }
                    }
                    state.gpu.grow_scene(wlo, whi);
                    state.scene.grow_bounds(wlo, whi);
                }
                // a finished scan is the dominant geometry - refit around everything so far
                let s = state.window.inner_size();
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                self.fitted = true;
                state.window.request_redraw();
            }
```

Those `Vec`s in the messages are the only copy of the data that ever exists on the CPU,
and they die in the handler: `Msg::CloudPos` writes and drops. `Msg::CloudEnd` carries
the cloud's local AABB, which the handler transforms by the slot's placement and hands to
`grow_scene` before refitting — a finished scan is the dominant geometry on screen.

## Step 6 — the scenes

Mark the scans in `assets/scenes/cloud_mix.toml`: each `lidar_scan*` item gains
`"stream": true`. The lion keeps parsing whole — its normals live only on the prost path.

And a dedicated stress scene, `assets/scenes/lidar14.toml`:

```json
{
  "name": "lidar 14M streamed",
  "items": [
    { "file": "pb/lidar_14m.pb", "name": "lidar 14M", "at": [0, 0, 0], "point_size": 1, "stream": true }
  ]
}
```

## Expected state

- `cargo check --target wasm32-unknown-unknown --lib`: clean; native `--examples` build.
- The native goldens are UNTOUCHED — the selftest path parses whole files, so
  `lion.json` still renders `325369 (33.9%)` and `cloud_mix.json` `12143 (1.1%)`. The
  new scene renders on both runs:

```
VIEWER_W=1200 VIEWER_H=800 VIEWER_ZOOM=8 \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- \
    out.ppm assets/scenes/lidar14.toml
# => non-background pixels: 18784 (2.0%)
```

![the 14M scan, streamed](img/42-lidar14.png)

- In the browser (`trunk serve --release`, which answers `206`), `cloud_mix` logs — with
  the sheets and bunny landing through prost BETWEEN the streams:

```
streaming 'scan000 (3.65M pts, 1 px)': 3648774 points | coords 84 MB + colours 26 MB
streamed 'scan000 (3.65M pts, 1 px)' in 1865ms
scene: 210891 objects ... 341989 cloud points          <- the lion, walked lane
streaming 'scan006 (3.50M pts, 6 px)': 3501943 points | coords 80 MB + colours 24 MB
streamed 'scan006 (3.50M pts, 6 px)' in 1072ms
```

- And the headline, with `DEMO_SCENE_URL` pointed at `scenes/lidar14.toml`:

```
streaming 'lidar 14M': 13793783 points | coords 316 MB + colours 96 MB
streamed 'lidar 14M' in 3402ms
```

A **431 MB** file on screen in **3.4 seconds**, growing visibly slice by slice, with the
JS heap around **130 MB** — one slice, one colour run, and the wasm runtime; never the
file. The whole-file path would have peaked near a gigabyte.

## What is deliberately not here

- **No push-stream state machine.** Range pulls make chunk-boundary bugs structurally
  impossible; see the design section above.
- **No streamed normals.** The scans have none; the fill is `u32::MAX` = unlit, and a
  dataset WITH normals (the lion) simply keeps the prost path.
- **No octree / LOD.** Potree streams by visibility cell; this lesson streams by byte
  offset. The splat lane absorbs the difference at these sizes — 13.8 M points splat in
  single-digit milliseconds — and LOD only starts paying at the 100 M+ scale.
- **No `Doc` for streamed clouds.** Picking, undo and save walk kernel sessions; a
  `CloudSlot` is not one. Streamed clouds are display objects until a lesson needs more.

## Recap

- A protobuf file is Range-addressable storage: three length prefixes reach `coords`, and
  packed-double means its length IS the point count — buffers sized once, exactly.
- Insist on `206`. A server that ignores `Range` sends the whole body, silently.
- Streamed rows land in their OWN lane; the two lanes meet in the per-pixel atomics.
  Nothing about the walked lane changes, and rebuild preservation is free.
- Depth dispatches for ALL lanes land before any colour dispatch.
- `queue.submit([])` after each slice write, or staging doubles the upload.
- The GPU boots before the first byte; slices convert, upload, and die.

## Next

[43 — Lane structs](43-lane-structs.md): before any new geometry, the flat `Gpu` and
`ArenaUpload` earn their split — named rows and per-lane sub-structs, behavior-identical.
Every lesson after it speaks the new field paths. Then
[44 — Cloud octree](44-cloud-octree.md) gives the cloud lane Potree's LOD, and
[45 — NurbsCurve](45-nurbscurve.md): the geometry block continues — every curved kernel type
gets on screen (curves, surfaces, iso lines, BReps, trims) before any interaction machinery exists.
