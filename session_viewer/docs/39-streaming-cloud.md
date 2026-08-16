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
| `src/app/persistence.rs` | `varint`, `walk_to_coords`, `fetch_range`, `cloud_fields`, `cloud_positions`, `cloud_colors` |
| `src/engine/gpu/mod.rs` | `cloud_begin` / `cloud_pos` / `cloud_col` / `grow_scene` |
| `src/app/scene.rs` | `CloudSlot`, `Scene::begin_cloud` |
| `src/lib.rs` | four `Msg` variants, the streaming branch, GPU up first |

---

## Step 1 — `Cargo.toml`

Find the `web-sys` feature list and add `"Headers",` after `"Response",`. Setting a `Range`
header needs it.

## Step 2 — the reader: `src/app/persistence.rs`

Append the streaming block: a `varint` helper, `walk_to_coords`, `fetch_range`,
`cloud_fields`, `cloud_positions`, `cloud_colors`. The two worth reading closely:

```rust
/// Walk `head` (the first few KB of the file) down Session.3 -> Objects.8 -> PointCloud, and
/// report where `coords` starts. Descends into exactly the three fields it cares about and
/// skips every other one by its length - no allocation, no decoding.
///
/// Returns `None` for anything that is not a single-cloud file, which is the signal to fall
/// back to the whole-file prost path.
fn walk_to_coords(head: &[u8]) -> Option<(u64, u64)> { … }
```

```rust
/// GET a byte range. Refuses anything but `206`: a server that ignores `Range` answers `200`
/// with the WHOLE body, which for a 411 MB scan would be catastrophic and silent.
pub async fn fetch_range(url: &str, start: u64, len: u64) -> Result<Vec<u8>, JsValue> { … }
```

`cloud_fields` does it in **two small reads**: 8 KB at the head for `coords`, then 16 bytes
at `coords_at + coords_len`, which is exactly where the `colors` header must be.

`cloud_positions` converts `f64 → f32` straight out of the slice. Positions are packed
little-endian doubles, bit-identical to a `&[f64]` on any LE target, so there is no
per-element *decode* — only the narrowing cast.

`cloud_colors` reads the whole 27 MB run because packed `uint32` is **varint**, not
memcpy-able the way `coords` is. Values 0–255 encode as 1–2 bytes each, four per point;
taking the run in one piece buys complete freedom from split-varint handling for
noise-level memory.

## Step 3 — the GPU side: `src/engine/gpu/mod.rs`

Three entry points plus the reserve helper 38 already factored out:

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

    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.queue.write_buffer(&self.point_pos_buffer, self.cloud_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.cloud_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.queue.submit([]);
    }
```

`cloud_col` is the same shape at stride 4. Add the `cloud_pos_at` / `cloud_col_at` cursor
fields (in **points**, not bytes) and a `grow_scene` that widens the scene box so the camera
can fit a cloud that no `ArenaUpload` ever described.

That empty `queue.submit([])` is not a no-op. It ticks Dawn's serial and lets the staging
ring recycle; without it the GPU process accumulates the whole upload a second time.

## Step 4 — the document row: `src/app/scene.rs`

A streamed cloud has no `Session` and no `Doc`. What it has is:

```rust
/// A cloud whose points never became kernel objects: the loader streamed them from the file
/// straight into GPU memory. This struct is the ENTIRE CPU-side footprint of a 13.8M-point
/// scan - a name, a count, and the instance row it draws with.
pub struct CloudSlot {
    pub name: String,
    pub count: u32,
    pub instance: u32,
}
```

`Scene::begin_cloud(name, place, count) -> u32` pushes the instance row into
`tables.objects`, keeps `order` / `guid_to_row` aligned with a synthetic `cloud:<name>`
guid, records the slot, and returns the row.

## Step 5 — the loader: `src/lib.rs`

Four new `Msg` variants — `CloudBegin`, `CloudPos`, `CloudCol`, `CloudEnd` — and one
structural change:

**The GPU and the canvas come up FIRST, empty.** A streamed cloud writes into GPU buffers,
so the GPU has to exist before the first byte of geometry is fetched. `State::new` now takes
an empty `Scene` and `Msg::Ready` is sent before the item loop, which as a bonus makes the
viewport live immediately instead of after a parse.

Then, per manifest item, `cloud_fields` decides the path:

```rust
                    if let Some(f) = persistence::cloud_fields(&item.file).await {
                        let _ = proxy.send_event(Msg::CloudBegin(named.clone(), place, f.count));

                        // 8 MB, rounded DOWN to a whole number of points: a slice boundary can
                        // then never fall inside a point, let alone inside one of its doubles.
                        const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
                        …
                            let _ = proxy.send_event(Msg::CloudPos(pos));
                            at += n;
                            left -= n;
                            // A real macrotask between slices. With a warm cache the fetch
                            // promises resolve as MICROtasks, which never let the browser paint -
                            // the same freeze the sliced prost parse exists to avoid.
                            persistence::next_tick().await;
                        …
                        continue;
                    }
                    // ── WHOLE-FILE PATH ── everything that is not a lone point cloud
```

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
is the one that does: three architectural sheets, a 3D model, and two LiDAR scans, with the
manifest order deliberately **interleaved** —

```
sheet · scan · sheet · sheet · scan · model
```

— so the two load paths take turns and neither is allowed to depend on running first or last.

They are genuinely different kinds of data taking genuinely different routes:

| | sheets + model | scans |
|---|---|---|
| content | meshes (fills) + polylines (linework) | 3.5M points each |
| path | whole-file prost → `add_file` → shared tables | Range slices → GPU buffers |
| kernel objects | yes, retained in `Doc.session` | **none** — a `CloudSlot` and nothing else |
| lane | triangles + ribbons + glyphs | the raw cloud lane |

Point `DEMO_SCENE_URL` at it:

```rust
const DEMO_SCENE_URL: &str = "scenes/mixed.json";
```

Three things this scene checks that a clouds-only scene cannot:

**The MSAA flip survives.** `msaa_for` returns 4 the moment a sheet's mesh vertices arrive,
which rebuilds every pipeline — including the point pipeline — *while clouds are already
resident in their buffers*. Pipelines are rebuilt; buffers are not. If the cloud vanished on
the first sheet, that is where to look.

**The draw order holds with both kinds present.** The cloud is opaque and writes depth, the
sheets' linework is blended and does not. Sheet ink in front of a scan must composite over
it; ink behind must be rejected. That is the whole reason the cloud draw sits up with the
solids rather than at the end of the pass — see [36](36-raw-cloud-lane.md).

**Press F.** The fit has to frame sheets *and* scans together, which is precisely what broke
when `set_scene` overwrote the scene box on every call: a streamed cloud contributes its box
through `grow_scene` after the fact, so an unconditional overwrite threw away every cloud
already loaded and F framed the last scan alone.

Sheets are 2–5 m across and sit in a row at `y = -30000`; each scan spans about 72 m and they
are 90 m apart. So a fitted view is mostly scan, with the sheets a small cluster below —
zoom in on them and the pen weights and hatching are all still there.

**One honest regression.** Lesson 35's loader kept file *n+1*'s fetch in flight while file *n*
parsed. That prefetch is gone here, and deliberately: the next item might be a cloud, and
eagerly fetching a cloud whole is exactly what this lesson exists to stop doing. Sheets
therefore load one after another. Restoring it means probing the next item with a cheap
8 KB range read before deciding — worth doing, not worth doing inside this lesson.

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
