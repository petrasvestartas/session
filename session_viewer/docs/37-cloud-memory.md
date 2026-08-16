# 37 Cloud memory — the five copies, and the tab that died

## Goal

Load the same three LiDAR scans with **half the peak wasm heap** and **none** of the
stale upload buffers, by making four small changes that apply to every lane — not just
clouds. No new features. The scene looks identical; the tab stops being 3.5 GB.

[36](36-raw-cloud-lane.md) made the cloud draw. This one makes it load:
37 takes the peak from 839 MB to 518 MB and kills the stale upload buffers,
[38](38-sixteen-bytes.md) halves the GPU table, [39](39-streaming-cloud.md) makes the peak
constant.

## What happened

Loading `scenes/pointclouds3.json` — three Zagreb Cathedral scans, 10.6M points —
killed the browser. Not a wasm trap, not a wgpu validation error. The Linux OOM
killer:

```
oom-kill:constraint=CONSTRAINT_NONE ... global_oom, task=chrome, pid=1978223
Out of memory: Killed process 1978223 (chrome) anon-rss:1741484kB oom_score_adj:300
```

`global_oom` means the whole machine ran out — 234 MB free RAM and 228 kB of free
swap. Chrome tags every renderer `oom_score_adj=300` precisely so that a runaway tab
dies before your window manager does, and the viewer tab was both the designated
victim and the fattest renderer at 2.37 GB.

So the viewer did not have a bug that crashed it. It had an appetite. That is worth
separating, because the fix is not a bug fix — it is a budget.

## Where the memory goes

With the scene loaded, `/proc/<renderer>/smaps` accounts for every megabyte:

```
1589 MB  [anon:v8-sandbox]                   <- the wasm linear memory
 323 MB  /dev/shm/.com.google.Chrome.NffmD9  <- upload #3 (files 1+2+3)
 217 MB  /dev/shm/.com.google.Chrome.nPub2S  <- upload #2 (files 1+2)
 111 MB  /dev/shm/.com.google.Chrome.lows0D  <- upload #1 (file 1)
```

renderer 2490 MB + gpu-process 1034 MB = **~3.5 GB for a GPU payload of 323 MB**.
Two separate faults, and both are visible in that dump.

### Fault 1 — every point exists four times

```
fetched bytes ──▶ decoded proto ──▶ kernel PointCloud ──▶ CloudPoint rows ──▶ GPU
   109 MB          coords Vec<f64>   coords Vec<f64>        f32, 32 B/pt
                 + colors Vec<u32>  + colors Vec<i32>
                     139 MB             139 MB                111 MB
```

Every one of those is live at the same time, because `session_from_bytes_chunked`
*borrows* the bytes for the whole conversion, and the decoded proto lives until the
function returns. And the last two are never freed at all: `Scene` keeps the kernel
`Session`, and `Scene.tables.points` keeps the f32 mirror.

Measured natively over the real files, `VmHWM` per stage, file 3 of 3:

| stage | rss | peak |
|---|---|---|
| after fetch bytes | 599 MB | 599 MB |
| after prost decode | 733 MB | 733 MB |
| after `from_proto` | 733 MB | **785 MB** |
| after CloudPoint table | 839 MB | **839 MB** |

The 733 → 785 step is a copy nobody asked for. `PointCloud::from_proto` converts
colours with `proto.colors.iter().map(|&c| c as i32).collect()` — `.iter()` *borrows*,
so a second 53 MB vec is built beside the live one. Coordinates escape it by luck:
`Vec<f64>` → `Vec<f64>` collects in place.

### Fault 2 — every append re-uploads everything

Those three `/dev/shm` segments are `create_buffer_init` calls, one per appended file,
at exactly the cumulative table sizes: 111.4, 216.6, 323.5 MB. **All three are still
resident**, and all three are mapped into the GPU process too — that is why
gpu-process sits at 1 GB. 651 MB of shared memory for 323 MB of data, two-thirds of it
dead.

And it is worse than it looks, because `create_buffer_init` means
`mapped_at_creation: true`, and on the WebGPU backend that is the most expensive
upload wgpu offers. From `wgpu-29.0.4/src/backend/webgpu.rs:1431`:

```rust
    actual_mapping: js_sys::Uint8Array,
    /// Copy of actual_mapping that lives in the Rust/Wasm heap instead of JS.
    temporary_mapping: OnceCell<Vec<u8>>,
```

```rust
    fn get_temporary_mapping(&self) -> &[u8] {
        self.temporary_mapping.get_or_init(|| self.actual_mapping.to_vec())
    }
```

Writing into a mapped range allocates a **full-size mirror in the wasm heap**, you
copy into that, and unmap copies it back out to JS. So uploading the 323 MB table
costs another 323 MB of wasm heap, transiently — a **fifth** copy of every point,
on top of the four above. That is the missing piece: it is why the wasm heap measures
1589 MB when the replay of the four copies only accounts for 839 MB.

`queue.write_buffer` has none of that. Its generated glue, in this project's own
`dist/*.js`:

```js
arg0.writeBuffer(arg1, arg2, getArrayU8FromWasm0(arg3, arg4), arg5, arg6);
// getArrayU8FromWasm0 -> getUint8ArrayMemory0().subarray(ptr, ptr + len)
```

A `subarray` **view** straight onto wasm linear memory. No mirror, no round trip. We
want that one, and we want it to write only the new rows.

### The rule that governs everything here

**Freeing memory in wasm does not give it back to the browser.** The linear memory
only ever grows. So that 1589 MB is the *high-water mark of transient peaks*, not the
live set — and "drop it when you're done" only helps because it lowers the *next*
peak. Design for a small peak, not a small live set. Everything in these three
lessons follows from that one sentence.

## Files we touch

| file | change |
|---|---|
| `src/app/persistence.rs` | take the bytes by value, drop them at the decode |
| `src/lib.rs` | hand the bytes over; call the new upload path |
| `session_rust/src/pointcloud.rs` | one word: `.iter()` → `.into_iter()` |
| `src/app/scene.rs` | `reserve_exact`; a `Scene::upload_to` that clears the mirror |
| `src/engine/gpu/mod.rs` | the point buffer appends instead of being rebuilt |
| `src/state.rs` | use the new upload path |

---

## Step 1 — the bytes die at the decode: `src/app/persistence.rs`

The signature borrows, so the 109 MB (411 MB on the full scan) stays alive through
the entire conversion for no reason.

**Find** (line 60):

```rust
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
```

**Replace with:**

```rust
pub async fn session_from_bytes_chunked(url: &str, bytes: Vec<u8>) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(&bytes));
    }
    let Ok(p) = proto::Session::decode(&bytes[..]) else { return Session::default() };
    // The decode is done and prost owns its own copy: from here the file bytes are
    // dead weight. 109 MB on a scan, 411 MB on the full one - and in wasm a peak you
    // never take is the only kind you ever get back.
    drop(bytes);
```

Taking `Vec<u8>` rather than `&[u8]` is the whole point: a borrow cannot be dropped,
only an owned value can. This one line is worth **104 MB** on this scene and 411 MB on
the 14M rung, and it helps every file type, not just clouds.

## Step 2 — the caller hands them over: `src/lib.rs`

The loader logs `bytes.len()` *after* the call, so capture it before the move.

**Find** (line 113):

```rust
                    let session = persistence::session_from_bytes_chunked(&item.file, &bytes).await;
```

**Replace with:**

```rust
                    let nbytes = bytes.len(); // read it before `bytes` moves into the parse
                    let session = persistence::session_from_bytes_chunked(&item.file, bytes).await;
```

**Find** (line 119) — the `bytes.len()` inside the log line:

```rust
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), bytes.len(), f1 - f0, crate::engine::performance::now_ms() - f1);
```

**Replace with:**

```rust
                    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), nbytes, f1 - f0, crate::engine::performance::now_ms() - f1);
```

## Step 3 — one word in the kernel: `session_rust/src/pointcloud.rs`

**Find** (line 378, inside `from_proto`):

```rust
            proto.colors.iter().map(|&c| c as i32).collect(),
```

**Replace with:**

```rust
            proto.colors.into_iter().map(|c| c as i32).collect(),
```

`u32` and `i32` are the same size and the same alignment, so consuming the vec lets
Rust convert **in place** and reuse the allocation. Borrowing it forces a second one.
Worth 53 MB here, 212 MB on the 14M scan.

Two things to know about this line. It is the **only** load-side copy-instead-of-move
in the whole kernel — every other `from_proto` conversion changes element size (`u32`
→ `usize` in `mesh.rs`) and genuinely has to allocate, so this is a one-line fix, not
a sweep. And it changes no signature, no behaviour, and no test, so the C++/Python
parity rule is untouched: this is an allocation detail inside one Rust function.

## Step 4 — reserve exactly: `src/app/scene.rs`

**Find**, in `push_cloud`:

```rust
    out.reserve(n);
```

**Replace with:**

```rust
    out.reserve_exact(n);
```

`reserve` is allowed to over-allocate, and on a shared table that already holds
millions of rows it doubles. Measured capacity across the three files: 111.4 →
222.7 → **445.4 MiB** for 323.5 MiB of rows — and during each regrowth both the old
and the new buffer are live. `reserve_exact` asks for what we actually know we need,
and here we always know it exactly.

## Step 5 — the point buffer appends: `src/engine/gpu/mod.rs`

This is the one that kills the 651 MB. Three edits.

**5a — a capacity field.** Find the point fields in the `Gpu` struct (line 128):

```rust
    pub point_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub point_count: u32,
```

**Replace with:**

```rust
    pub point_buffer: wgpu::Buffer,
    pub point_bind_group: wgpu::BindGroup,
    pub point_count: u32,
    pub point_capacity: u64, // ROWS the buffer can hold; point_count is how many are filled
```

**5b — start with a real, growable buffer.** Find, in `Gpu::new` (line 438):

```rust
        // Point buffer + the cloud uniform
        let points: Vec<CloudPoint> = Vec::new();
        let point_count = points.len() as u32;

        // point storage buffer
        let point_buffer = storage_buffer(&device, "points.buffer", &points);
```

**Replace with:**

```rust
        // Point buffer + the cloud uniform
        let point_count = 0u32;
        let point_capacity = 1u64; // one zeroed row: wgpu cannot bind a 0-byte buffer

        // COPY_SRC so growth can copy the live prefix on the GPU - and so a later lesson
        // can read a pick result back without reallocating this buffer.
        let point_buffer = zeroed_buffer(
            &device,
            "points.buffer",
            point_capacity * std::mem::size_of::<CloudPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        );
```

Then add `point_capacity,` to the struct literal at the end of `new()`, next to
`point_count,` (line 528).

**5c — append instead of rebuild.** Find the block in `set_scene` (line 656):

```rust
        // Raw cloud lane: one row per scanned point, uploaded like any other table. Until now
        // this buffer was built empty in new() and never refilled - the machinery existed, the
        // rows never arrived.
        self.point_count = up.points.len() as u32;
        self.point_buffer = storage_buffer(&self.device, "points.buffer", &up.points);
        self.point_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("points.bind_group"),
                layout: &self.glyph_layout,
                entries: &[wgpu::BindGroupEntry{
                    binding: 0,
                    resource: self.point_buffer.as_entire_binding()
                }],
        });
```

**Replace with:**

```rust
        // Raw cloud lane. READ THIS BEFORE THE CODE: unlike every other table in this
        // function, `up.points` is a DELTA - only the rows the newest file added. The caller
        // clears the mirror after each upload (see Scene::upload_to), because once the GPU has
        // a scanned point the CPU has no further use for it.
        //
        // The old code recreated the whole buffer per appended file. Three files meant three
        // create_buffer_init calls of 111, 217 and 323 MB, each one a full-size MAPPED buffer
        // in shared memory - and all three stayed resident, in this process AND in the GPU
        // process. 651 MB of shm for 323 MB of data. write_buffer instead views wasm memory
        // directly and writes only the new rows.
        if !up.points.is_empty() {
            let stride = std::mem::size_of::<CloudPoint>() as u64;
            let need = self.point_count as u64 + up.points.len() as u64;

            if need > self.point_capacity {
                // EXACT, not doubling. Doubling is the reflex here and it is wrong: appends are
                // FEW (one per manifest file) and HUGE, so the capacity it buys is pure waste -
                // 445 MB of buffer for 323 MB of rows on this scene, 122 MB thrown away - and it
                // does not even win on the transient, because the reallocation it fails to avoid
                // is the big one (668 MB of old+new live at once, against 540 MB exact). What
                // doubling saves is a GPU-side copy, and a GPU-side copy is the one thing here
                // that costs nothing: it never touches wasm memory.
                let cap = need;
                let bigger = zeroed_buffer(
                    &self.device,
                    "points.buffer",
                    cap * stride,
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
                );
                if self.point_count > 0 {
                    let mut enc = self.device.create_command_encoder(&Default::default());
                    enc.copy_buffer_to_buffer(&self.point_buffer, 0, &bigger, 0, self.point_count as u64 * stride);
                    self.queue.submit([enc.finish()]);
                }
                self.point_buffer = bigger;
                self.point_capacity = cap;
                self.point_bind_group = self.device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some("points.bind_group"),
                        layout: &self.glyph_layout,
                        entries: &[wgpu::BindGroupEntry{
                            binding: 0,
                            resource: self.point_buffer.as_entire_binding()
                        }],
                });
            }

            self.queue.write_buffer(
                &self.point_buffer,
                self.point_count as u64 * stride,
                bytemuck::cast_slice(&up.points),
            );
            self.point_count += up.points.len() as u32;
        }
```

The bind group is rebuilt **only** when the buffer is reallocated — that is the whole
reason `point_capacity` exists as a separate number from `point_count`.

## Step 6 — clear the mirror: `src/app/scene.rs`, `src/state.rs`, `src/lib.rs`

`set_scene` takes `&ArenaUpload`, so it cannot clear the table itself. Give `Scene`
one method that does both, so the two call sites cannot drift apart.

**Add** to `impl Scene` in `src/app/scene.rs`, next to `add_file`:

```rust
    /// Upload, then FORGET the cloud rows. The GPU is now the only holder of those points,
    /// which is the point: 323 MB of f32 mirror for the three scans, retained for nothing.
    /// Only `points` is cleared - the other lanes are still uploaded cumulatively, because
    /// only the point lane has an append path (Gpu::set_scene, step 5c).
    pub fn upload_to(&mut self, gpu: &mut crate::engine::gpu::Gpu) {
        gpu.set_scene(&self.tables);
        self.tables.points.clear();
        self.tables.points.shrink_to_fit();
    }
```

`shrink_to_fit` matters here in a way it usually does not: without it the `Vec` keeps
its 445 MB allocation, and a cleared-but-still-huge allocation is exactly the kind of
thing wasm never hands back.

**In `src/state.rs`**, find (line 25):

```rust
    pub async fn new(window: Arc<Window>, scene: Scene) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        gpu.set_scene(&scene.tables);
```

**Replace with:**

```rust
    pub async fn new(window: Arc<Window>, mut scene: Scene) -> anyhow::Result<Self>{
        let t0 = now_ms();
        let mut gpu = Gpu::new(window.clone()).await?;
        scene.upload_to(&mut gpu);
```

**In `src/lib.rs`**, in the `Msg::File` arm, find:

```rust
                state.gpu.set_scene(&state.scene.tables);
```

**Replace with:**

```rust
                state.scene.upload_to(&mut state.gpu);
```

(`scene` and `gpu` are different fields of `State`, so borrowing both at once is fine.)

### The thing that will look wrong later

`Scene::add_file` starts with `let point0 = self.tables.points.len();` and its bounds
loops then do `t.points.iter().skip(point0)`. After step 6, `point0` is always **0**,
because the table was cleared by the previous upload. That is correct — `skip(0)`
walks exactly the rows this file just added — but it reads like a bug six months from
now, so leave the `skip(point0)` in place rather than "simplifying" it to nothing. It
is what keeps `add_file` honest if the clearing ever goes away.

`instance_id` still indexes `t.objects`, which is **not** cleared and stays cumulative.
That asymmetry is deliberate and is why the comment in step 5c is shouty.

## Step 7 — while we are here: `storage_buffer` itself

Step 5 took the point lane off `create_buffer_init`, but the helper is still used for
the instance table (`mod.rs:284` and `mod.rs:563`), and it will be used by every table
someone adds later. Fix the helper, not just the one caller.

**Find** (line 1171):

```rust
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, label: &str, data: &[T]) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    let one = [T::zeroed()];
    let contents: &[u8] = if data.is_empty() { bytemuck::cast_slice(&one) } else { bytemuck::cast_slice(data) };
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    })
}
```

**Replace with:**

```rust
/// A read-only storage buffer that is never zero-sized (wgpu can't bind a 0-byte buffer).
///
/// Deliberately NOT `create_buffer_init`: `mapped_at_creation` on the WebGPU backend
/// allocates a full-size mirror of the contents in the wasm heap (wgpu's
/// `temporary_mapping`), copies into it, then copies it back out to JS. `write_buffer`
/// passes a subarray VIEW of wasm memory instead - same result, one fewer copy of the
/// whole table, and the copy it removes is the one that lands in the wasm heap and
/// never comes back.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer {
    let one = [T::zeroed()];
    let contents: &[u8] = if data.is_empty() { bytemuck::cast_slice(&one) } else { bytemuck::cast_slice(data) };
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: contents.len() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    queue.write_buffer(&buffer, 0, contents);
    buffer
}
```

Then pass the queue at both remaining call sites — `storage_buffer(&device, &queue, "instance.buffer", &instances)` at line 284, and `storage_buffer(&self.device, &self.queue, "instance.buffer", &self.instances)` at line 563.

The three `arena_*` buffers (`mod.rs:580`, `586`, `592`) still call `create_buffer_init`
directly. They are the drawings lane, not the cloud lane, and they deserve the same
treatment — but measure that separately so you can attribute the win.

---

## What this bought

Same replay, same files, two agreeing passes:

| | 3 scans (10.6M) | 14M scan |
|---|---|---|
| peak before | 839 MB | 1365 MB |
| **peak after** | **518 MB** | **953 MB** |
| retained before | 735 MB | 954 MB |
| **retained after** | **411 MB** | **533 MB** |
| stale shm upload buffers | 651 MB | — |
| **after** | **0** | — |

Roughly: a third of the peak and all of the shared memory, for a handful of small
edits and one word in the kernel.

**Read that table carefully — it undercounts.** Those are *native* numbers, and a
native replay models the four CPU copies but not the fifth: wgpu's `temporary_mapping`
mirror only exists on the WebGPU backend. Steps 5 and 7 delete a further ~323 MB of
wasm heap that this table cannot see. That saving shows up only in the in-browser gate
below, which is the number that actually killed the tab.

What it does **not** fix: the 14M scan still peaks near a gigabyte, because the
decoded proto and the row table have to coexist. And 411 MB is still retained for
three scans that draw from 323 MB of GPU data. Those are 38 and 39.

## Verify

```bash
trunk serve --release          # judge memory on release wasm, not the dev build
```

Load `scenes/pointclouds3.json`, then from a terminal:

```bash
# the viewer tab's renderer
ps -eo pid,rss,args --no-headers | grep -- --type=renderer | sort -k2 -rn | head -3

# where its memory actually is
awk '/^[0-9a-f]/{n=$6} /^Rss:/{if($2>50000) printf "%7.0f MB  %s\n", $2/1024, (n==""?"[anon]":n)}' \
    /proc/<pid>/smaps | sort -rn | head
```

Gates:

- **No `/dev/shm/.com.google.Chrome.*` mapping above a few MB.** Before this lesson
  there were three, totalling 651 MB. This is the sharpest signal that step 5 landed.
- `[anon:v8-sandbox]` clearly below the 1589 MB it was.
- The console's `appended: walk … · upload …ms` line — the **upload figure must stop
  growing** with each file. It was 491 → 675 ms because each append re-sent the whole
  table.
- The scene is pixel-identical and still runs at ~110 fps with 10.6M points on screen.

## What is deliberately NOT here

- **The 32 B row.** `CloudPoint` still carries a per-point `instance_id` and four f32
  colour channels. → [38](38-sixteen-bytes.md)
- **The kernel `PointCloud`.** Still built, still retained, still 139 MB a scan —
  and, as it happens, `Doc.session` currently has no readers at all. → [39](39-streaming-cloud.md)
- **The decoded proto.** Still materialised whole by prost, which is why the 14M peak
  barely moved. → [39](39-streaming-cloud.md)
- **`PointCloud::_colors` is `Vec<i32>`** — four bytes to hold a 0-255 channel, 221 MB
  per 14M-point cloud where `u8` would be 55 MB. Fixing it is a three-language API
  change with matching minitests, and it stops mattering to the viewer once 39 stops
  building kernel clouds. Worth doing; not here.

## Recap

```
Ch 35:  the document came back - Scene keeps every parsed Session, set_scene is the ONE
        upload path, and the parse yields so the UI survives.
Ch 37: it also came back with FIVE copies of every point and a fresh 300 MB mapped
        buffer per appended file. Fix them: bytes dropped at the decode (Vec, not
        &[u8] - you cannot drop a borrow), colours converted in place (into_iter, one
        word, one line in the kernel), reserve_exact instead of a doubling reserve,
        the point buffer APPENDS via write_buffer with GPU-side growth instead of
        being rebuilt, and storage_buffer stops using create_buffer_init - because
        mapped_at_creation mirrors the WHOLE table into the wasm heap first, which is
        the fifth copy and the one the native replay cannot see. Scene::upload_to then
        forgets the rows, because the GPU is now their only holder.
        839 -> 518 MB native peak, 651 -> 0 MB shm, and ~323 MB of wasm heap that only
        the browser gate reveals. The rule underneath all of it: wasm memory never
        shrinks, so the number that matters is the PEAK, not the live set.
```

Edited: `app/persistence.rs` (bytes by value + `drop`), `lib.rs` (`nbytes`, `upload_to`),
`app/scene.rs` (`reserve_exact`, `upload_to`), `engine/gpu/mod.rs` (`point_capacity`,
growable point buffer, append path, `storage_buffer` off `create_buffer_init`),
`state.rs` (`upload_to`), `session_rust/src/pointcloud.rs` (`into_iter`).

## Next

[`38-sixteen-bytes.md`](38-sixteen-bytes.md) — a scanned point does not need a
per-point object id and it does not need four floats of colour. Split the row into a
positions buffer and a colours buffer, 12 B + 4 B, one draw call per cloud: the GPU
table halves, and the split turns out to be exactly what 39 needs to stream a file
straight from the socket into GPU memory.
