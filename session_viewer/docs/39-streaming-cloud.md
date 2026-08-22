# 39 Streaming cloud — HTTP Range in, GPU rows out

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

## Files we touch

| file | change |
|---|---|
| `Cargo.toml` | `web-sys` gains `"Headers"` |
| `src/app/persistence.rs` | `varint`, `walk_to_coords`, `fetch_range(_start/_finish)`, `positions_from`, `cloud_fields`, `cloud_colors` |
| `src/engine/gpu/mod.rs` | `cloud_begin` / `cloud_pos` / `cloud_col` / `grow_scene`; `set_scene` keeps a finite box |
| `src/app/scene.rs` | `CloudSlot`, `begin_cloud`, `grow_bounds`; `rebuild` preserves clouds |
| `src/lib.rs` | four `Msg` variants, the streaming branch, GPU up first |

---

## Step 1 — `Cargo.toml`

Find the `web-sys` feature list and add `"Headers",` after `"Response",`. Setting a `Range`
header needs it.

## Step 2 — the reader: `src/app/persistence.rs`

Two edits. First, the loader will yield between slices, so **find** `async fn next_tick()`
and make it `pub async fn next_tick()`.

Then **append at the bottom of the file** the whole streaming block:

```rust
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
    let headers = web_sys::Headers::new()?;
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
    let headers = web_sys::Headers::new()?;
    headers.set("Range", &format!("bytes={}-{}", start, start + len - 1))?;
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);
    opts.set_headers(&headers);
    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let resp: Response = JsFuture::from(window.fetch_with_request(&request)).await?.dyn_into()?;
    if resp.status() != 206 {
        return Err(JsValue::from_str("server ignored Range (no 206) - refusing to pull the whole body"));
    }
    let buf = JsFuture::from(resp.array_buffer()?).await?;
    Ok(js_sys::Uint8Array::new(&buf).to_vec())
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

`cloud_colors` reads the whole 27 MB run because packed `uint32` is **varint**, not
memcpy-able the way `coords` is. Values 0–255 encode as 1–2 bytes each, four per point;
taking the run in one piece buys complete freedom from split-varint handling for
noise-level memory.

## Step 3 — the GPU side: `src/engine/gpu/mod.rs`

Two cursor fields, four entry points, and one guard in `set_scene`.

**Find** in the `Gpu` struct:

```rust
    pub cloud_draws: Vec<CloudDraw>,
```

**Add below it:**

```rust
    cloud_pos_at: u32, // streaming cursors, in POINTS not bytes
    cloud_col_at: u32,
```

and in the struct literal at the end of `new()`, after `cloud_draws,`:

```rust
            cloud_pos_at: 0,
            cloud_col_at: 0,
```

**Add** to `impl Gpu`, right after `cloud_reserve` (the helper 38 factored out is
exactly what `cloud_begin` needs):

```rust
    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so both buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        self.cloud_reserve(self.point_count as u64 + count as u64);
        self.cloud_draws.push(CloudDraw { base: self.point_count, count, instance });
        self.cloud_pos_at = self.point_count;
        self.cloud_col_at = self.point_count;
        self.point_count += count;
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes a
    /// subarray VIEW of wasm memory - the slice is the only copy that exists.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.queue.write_buffer(&self.point_pos_buffer, self.cloud_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.cloud_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.queue.submit([]);
    }

    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.queue.write_buffer(&self.point_col_buffer, self.cloud_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.cloud_col_at += col.len() as u32;
        self.queue.submit([]);
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, min: [f32; 3], max: [f32; 3]) {
        if !min[0].is_finite() { return }
        // set_scene collapses an empty upload to a zero box; the first cloud replaces it.
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

That empty `queue.submit([])` is not a no-op. It ticks Dawn's serial and lets the staging
ring recycle; without it the GPU process accumulates the whole upload a second time.

Last, the guard. A streamed cloud's instance row arrives through `set_scene` in an upload
whose walk tables are empty — so `up.min` is still infinite, and blindly assigning it
would wipe the box of every cloud already loaded (that is the press-F bug described at the
end of this lesson). **Find** in `set_scene`:

```rust
        self.scene_min = up.min;
        self.scene_max = up.max;
```

**Replace with:**

```rust
        // ONLY the walk tables know a box here, and a STREAMED cloud has none: its points never
        // pass through `add_file`, so `up.min` stays infinite and the cloud reports its box later
        // through `grow_scene`. Overwriting unconditionally therefore wiped the box of every cloud
        // already loaded - which is why F framed the LAST scan instead of all three.
        if up.min[0].is_finite() {
            self.scene_min = up.min;
            self.scene_max = up.max;
        }
```

## Step 4 — the document row: `src/app/scene.rs`

A streamed cloud has no `Session` and no `Doc`. **Add above `pub struct Scene`:**

```rust
/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This struct is the ENTIRE CPU-side footprint of a 13.8M-point
/// scan - a name, a placement, a count, and the instance row it draws with.
pub struct CloudSlot {
    pub name: String,
    pub place: Xform,
    pub count: u32,
    pub instance: u32,
}
```

Give `Scene` the list — after `pub docs: Vec<Doc>,` add

```rust
    pub clouds: Vec<CloudSlot>, // streamed clouds - no Doc, no Session, no points on the CPU
```

with the matching `clouds: Vec::new(),` in `Scene::new()`.

**Add** to `impl Scene`, above `rebuild`:

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
    pub fn begin_cloud(&mut self, name: String, place: Xform, count: u32) -> u32 {
        let row = self.tables.objects.len() as u32;
        self.tables.objects.push((place.clone(), [1.0; 4], 0));
        self.tables.object_bounds.push(None);
        self.tables.object_spacing.push(0.0);
        // Keep the row bookkeeping aligned - `order` is indexed by row everywhere else.
        let guid = format!("cloud:{name}");
        self.guid_to_row.insert(guid.clone(), row);
        self.order.push(guid);
        self.clouds.push(CloudSlot { name, place, count, instance: row });
        row
    }
```

The `object_bounds`/`object_spacing` pushes are the same row-alignment rule every walk arm
follows — `set_scene` zips those tables against `objects` and a missing row would shift
every later object's bound.

And `rebuild` learns the cloud rule: streamed points exist only on the GPU, so they cannot
be re-walked — the slots survive and their instance rows are re-issued. In `rebuild`,
**find** `let docs = std::mem::take(&mut self.docs);` and add below it
`let clouds = std::mem::take(&mut self.clouds);`. Then **find** the reset lines 37/38
added:

```rust
        gpu.point_count = 0; // clouds are delta-appended; the re-walk regenerates every row
        gpu.cloud_draws.clear();
```

**Delete them** — they were right while every cloud row came from a re-walkable kernel
object, and wrong the moment rows exist that nothing can regenerate. Finally, after the
`for d in docs { … }` loop, **add:**

```rust
        // Clouds keep their GPU rows; only the instance they draw against is re-issued, and the
        // Gpu's draw list is patched to match. Order is preserved on both sides, so index i
        // here is index i there.
        for (i, c) in clouds.into_iter().enumerate() {
            let row = self.begin_cloud(c.name, c.place, c.count);
            if let Some(d) = gpu.cloud_draws.get_mut(i) {
                d.instance = row;
            }
        }
```

(A kernel-path cloud — one under `CLOUD_RAW_MIN`, or one living inside a mixed file —
still re-walks through `push_cloud` and re-appends; that pairing is only exact while
rebuild stays uncalled, which it is until the editing lessons. Noted so it does not bite
silently.)

## Step 5 — the loader: `src/lib.rs`

Four new `Msg` variants, a restructured loader, and four handler arms.

**5a — the variants.** Find `File(String, session_rust::Session, session_rust::Xform),`
in the `Msg` enum and **add below it:**

```rust
    /// A cloud is about to stream in: name, placement, and the EXACT point count - known from
    /// the file's packed-double length prefix before a single point has been fetched.
    CloudBegin(String, session_rust::Xform, u32),
    /// One slice of positions / the colour run, on their way to GPU memory. These Vecs are the
    /// only copy of that data that ever exists on the CPU, and they die in the handler.
    CloudPos(Vec<f32>),
    CloudCol(Vec<u32>),
    /// Done, with the cloud's local-space AABB for the camera fit.
    CloudEnd([f32; 3], [f32; 3]),
```

**5b — the loader.** One structural change: **the GPU and the canvas come up FIRST,
empty.** A streamed cloud writes into GPU buffers, so the GPU has to exist before the
first byte of geometry is fetched — and as a bonus the viewport is live immediately
instead of after a parse. This *replaces* 35's first-file-builds-the-State pattern, and it
also retires 35's fetch-ahead of the next item — deliberately, because the next item might
be a cloud, and eagerly fetching a cloud whole is exactly what this lesson exists to
prevent.

**Replace** the body of the `spawn_local` — everything from the `// Manifest, then the
files` comment down to the end of the `for` loop over `manifest.items` — with:

```rust
                // Manifest, then the files.
                //
                // The canvas and the GPU come up FIRST, empty. A streamed cloud writes into GPU
                // buffers, so the GPU has to exist before the first byte of geometry is fetched -
                // and as a bonus the viewport is live immediately instead of after a parse.
                // (35's fetch-ahead is gone on purpose: the next item might be a cloud, and
                // eagerly fetching a cloud WHOLE is exactly what this loader exists to prevent.)
                let t0 = crate::engine::performance::now_ms();
                let manifest_bytes = persistence::fetch_bytes(DEMO_SCENE_URL).await.unwrap_or_default();
                let manifest = Manifest::parse(&manifest_bytes).unwrap_or_else(|| panic!("cannot read the scene manifest at {DEMO_SCENE_URL}"));
                log::info!("scene '{}': {} items", manifest.name, manifest.items.len());
                let count = manifest.items.len();

                let state = State::new(window.clone(), Scene::new()).await.expect("State init failed");
                log::info!("canvas live {:.0}ms after manifest fetch", crate::engine::performance::now_ms() - t0);
                let _ = proxy.send_event(Msg::Ready(Box::new(state)));

                for (i, item) in manifest.items.iter().enumerate() {
                    let f0 = crate::engine::performance::now_ms();
                    let place = item.placement().unwrap_or_else(|| auto_grid(i, count, [0.0, 0.0]));
                    let named = if item.name.is_empty() { item.file.clone() } else { item.name.clone() };

                    // ── STREAMING PATH ──────────────────────────────────────────────────────
                    // A cloud-only .pb never becomes a kernel object and never exists whole in
                    // wasm memory. Two small Range reads find the packed arrays; the coords run
                    // then arrives in 8 MB slices, each converted, handed to the GPU and dropped.
                    if let Some(f) = persistence::cloud_fields(&item.file).await {
                        log::info!("streaming '{}': {} points | coords {:.0} MB + colours {:.0} MB",
                            named, f.count, f.coords_len as f64 / 1048576.0, f.colors_len as f64 / 1048576.0);
                        let _ = proxy.send_event(Msg::CloudBegin(named.clone(), place, f.count));

                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
                        let (mut at, mut left) = (f.coords_at, f.coords_len);
                        let (mut lo, mut hi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);

                        // PIPELINED, and this is the whole performance story of the loader.
                        // `fetch_range(..).await` is itself a yield: the promise resolves off
                        // network I/O, so it cannot resume until the current FRAME is done.
                        // Keeping slice n+1 in flight while slice n converts hides the round
                        // trip AND the frame behind work we had to do anyway.
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

                    // ── WHOLE-FILE PATH ─────────────────────────────────────────────────────
                    // Everything that is not a lone point cloud still goes through prost.
                    let bytes = match persistence::fetch_start(&item.file) {
                        Ok(f) => persistence::fetch_finish(f).await.unwrap_or_default(),
                        _ => Vec::new(),
                    };
                    let f1 = crate::engine::performance::now_ms();
                    let nbytes = bytes.len(); // read it before `bytes` moves into the parse
                    let session = persistence::session_from_bytes_chunked(&item.file, bytes).await;
                    let name = if item.name.is_empty() { session.name.clone() } else { item.name.clone() };
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), nbytes, f1 - f0, crate::engine::performance::now_ms() - f1);
                    if session.lookup.is_empty() {
                        continue; // failed fetch - skipped file
                    }
                    let _ = proxy.send_event(Msg::File(name, session, place));
                }
```

**5c — the handlers.** In `user_event`, **add above the `Msg::File` arm:**

```rust
            // A cloud, streaming. Nothing here holds points: begin_cloud reserves the GPU
            // range from a count that is already known, each slice is written and dropped, and
            // the CPU keeps a name, a count and one instance row.
            Msg::CloudBegin(name, place, count) => {
                let Some(state) = &mut self.state else { return };
                let row = state.scene.begin_cloud(name, place, count);
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
                    if let Some((xf, _, _)) = state.scene.tables.objects.get(slot.instance as usize) {
                        let (mut wlo, mut whi) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
                        for c in 0..8u32 {
                            let corner = [
                                if c & 1 == 0 { lo[0] } else { hi[0] },
                                if c & 2 == 0 { lo[1] } else { hi[1] },
                                if c & 4 == 0 { lo[2] } else { hi[2] },
                            ];
                            let w = crate::app::scene::xform_point(xf, corner);
                            for k in 0..3 { wlo[k] = wlo[k].min(w[k]); whi[k] = whi[k].max(w[k]); }
                        }
                        state.gpu.grow_scene(wlo, whi);
                        state.scene.grow_bounds(wlo, whi);
                    }
                }
                let s = state.window.inner_size();
                let aspect = s.width.max(1) as f64 / s.height.max(1) as f64;
                state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);
                state.window.request_redraw();
            }
```

(`xform_point` in `scene.rs` is already `pub`; the handler reaches it as
`crate::app::scene::xform_point`.)

Those `Vec`s in the messages are the only copy of the data that ever exists on the CPU, and
they die in the handler. `Msg::CloudPos` writes and drops; `Msg::CloudEnd` carries the
cloud's local AABB, which the handler transforms by the instance matrix and hands to
`grow_scene` before fitting the camera.

`next_tick()` matters more than it looks. With a warm cache the range requests resolve as
**microtasks**, and a microtask never lets the browser paint — the exact freeze that the
sliced prost parse in 35 exists to avoid, reintroduced by a different route.

## Verify

```bash
cargo check --target wasm32-unknown-unknown
trunk build
```

Then, with a cloud manifest loaded, the console should read:

```
canvas live NNNms after manifest fetch
streaming 'scan000  (3.65M pts)': 3648774 points | coords 84 MB + colours 26 MB
streamed 'scan000  (3.65M pts)' in NNNNms
```

The count in that line is the acceptance test all by itself: it came from a length prefix
in the first 175 bytes of the file, and if the walk is wrong it will be wrong loudly rather
than subtly.

Then measure, which is the whole point:

```bash
awk '/^[0-9a-f]/{n=$6} /^Rss:/{if($2>50000) printf "%7.0f MB  %s\n", $2/1024, n}' \
    /proc/<renderer-pid>/smaps | sort -rn
```

| | before this series | after |
|---|---|---|
| wasm heap (`[anon:v8-sandbox]`) | 1589 MB | tens of MB |
| stale `/dev/shm/.com.google.Chrome.*` | 651 MB in 3 buffers | none |
| GPU buffers, 14M scan | 421 MB | 221 MB |
| CPU retained | 954 MB | a name and a count |

And the acceptance test the whole series was for: **`pb/lidar_14m.pb` loads**, which it
could not before.

## The closing scene — everything at once

A lesson that only ever loads point clouds has not proved much. `assets/scenes/mixed.json`
is the one that does: it is **everything lesson 35 proved, plus the scans**, with the
manifest order deliberately **interleaved** —

```
sheet · scan · sheet · sheet · scan · sheet · boxes · model
```

— so the two load paths take turns and neither is allowed to depend on running first or last.

What is in it, and why each piece is there:

| item | what it tests |
|---|---|
| 4 sheets: flat / standing 90° / tilted 45° / spun 30° | 35's planar-lane torture test — paper-space pens under arbitrary orientation |
| `colors_widths` — 3 boxes, 1 polyline, 1 point | per-face colours, mesh edges, pen widths, the glyph lane's dot |
| `floor_model` — 201 meshes, 290 polylines | a real 3D model: solid lane and flat lane together |
| 2 LiDAR scans, 3.5M points each | the raw cloud lane, streamed |

and the two routes they take:

| | everything above the scans | the scans |
|---|---|---|
| path | whole-file prost → `add_file` → shared tables | Range slices → GPU buffers |
| kernel objects | yes, retained in `Doc.session` | **none** — a `CloudSlot` and nothing else |
| lanes | triangles, cylinders, ribbons, glyphs | the raw cloud lane |

Point `DEMO_SCENE_URL` at it:

```rust
const DEMO_SCENE_URL: &str = "scenes/mixed.json";
```

Four things this scene checks that a clouds-only scene cannot:

**The MSAA flip survives.** `msaa_for` returns 4 the moment the first sheet's mesh vertices
arrive, which rebuilds every pipeline — the point pipeline included — *while clouds are
already resident in their buffers*. Pipelines are rebuilt; buffers are not. If a cloud
vanished when a sheet landed, that is where to look.

**The draw order holds with both kinds present.** The cloud is opaque and writes depth; the
sheets' linework is blended and does not. Sheet ink in front of a scan must composite over
it, ink behind must be rejected. That is the whole reason the cloud draw sits up with the
solids rather than at the end of the pass — see [36](36-raw-cloud-lane.md).

**Pens stay px-constant next to 13.8M points.** The three `colors_widths` boxes and the
rotated sheets are the same test 35 ended on; nothing in 36–39 was allowed to change them.

**Press F.** The fit has to frame sheets *and* scans together, which is precisely what broke
when `set_scene` overwrote the scene box on every call: a streamed cloud contributes its box
through `grow_scene` after the fact, so an unconditional overwrite threw away every cloud
already loaded and F framed the last scan alone.

Sheets are 2–5 m across and sit in a row at `y = -30000`; each scan spans about 72 m and they
are 90 m apart. So a fitted view is mostly scan, with the sheets a small cluster below —
zoom in and the hatching, pen weights and box edges are all still there.

**Two honest notes.** The sheets are **not** streamed: 130 MB of them still go through the
whole-file prost path, so this scene's peak is dominated by the sheets, not the scans. The
streaming win of 37–39 applies to clouds only. And lesson 35's fetch-ahead is gone —
deliberately, because the next manifest item might be a cloud and eagerly fetching a cloud
whole is exactly what this lesson exists to prevent. Restoring it means probing the next
item with a cheap 8 KB range read first: worth doing, not worth doing inside this lesson.

## What is deliberately not here

- **A re-fetch fallback.** Chrome will not keep a 411 MB body in its disk cache, so
  "fall back and re-download" is a second 411 MB. Files that are not cloud-only take the
  whole-file prost path from the start; the single-pass hybrid (copy non-cloud sub-messages
  aside as raw bytes and `prost::decode` the concatenation at the end — protobuf
  concatenation semantics make that valid) is the honest fix if mixed files ever appear.
- **Unpacked encodings.** proto3 packs repeated scalars by default and prost never emits
  the unpacked form, but a decoder that *assumes* it is a decoder with a silent failure
  mode. `walk_to_coords` returns `None` on anything unexpected and the caller falls back.
- **`coords` before `colours` is assumed.** True because prost emits in field-number order,
  not guaranteed by the wire format. If colours came first the count would be unknown when
  they arrive; the walk returns `None` for that layout rather than guessing.
- **A per-cloud f64 origin.** `as f32` is fine for these scanner-local millimetres — the ULP
  at 73 m is 0.0078 mm — but a projected CRS (UTM easting ~500000) quantises to ~0.03 m, and
  camera-relative rebasing cannot recover precision destroyed at load. Subtract the first
  point (available in the first 24 bytes of `coords`) and fold it into the instance matrix.

## Recap

```
Ch 38:  16 B a point on the GPU, but the FILE still existed whole in wasm memory first -
        fetched bytes, then a prost-decoded proto - so the 14M scan still peaked near a GB.
Ch 39:  the file stops existing whole. Two facts do it: every hop to the payload is
        length-delimited so the headers are in the first ~170 bytes, and coords is a packed
        DOUBLE so its length prefix gives the exact point count before any payload is read -
        which sizes both GPU buffers once, exactly, with no growth and no reserve.
        RANGE requests, not a ReadableStream: split doubles, split varints and split headers
        are risks that only exist when data is PUSHED. 8 MB slices rounded down to whole
        points cannot split anything; colours come in one piece because packed uint32 is
        varint. 206 or refuse - a server that ignores Range returns the whole 411 MB body.
        GPU up first (a streamed cloud writes into buffers that must already exist),
        next_tick between slices (warm-cache promises resolve as microtasks and never paint),
        empty submit after each write (Dawn recycles staging only on a completed serial).
        CloudSlot - a name, a count, an instance row - is the entire CPU footprint of a
        13.8M-point scan.
```

## Next

[`40-scene-bvh.md`](40-scene-bvh.md) — back to the main line, with a scene that no longer
flirts with the heap ceiling. Per-point picking and lasso crop get their own lesson later;
this series deliberately left the door open for them: `vertex_index` is already the point
index, group(1) on the point pipeline is free, and a 1-bit-per-point selection mask is
1.64 MB for the full scan.
