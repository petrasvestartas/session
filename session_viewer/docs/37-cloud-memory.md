# 37 Cloud memory — kill the upload mirror

> Direct-path chain (36–41); every step below is replay-verified against a clean
> end-of-35 checkout.

## Goal

A 13.8-million-point cloud is 276 MB of tables in a wasm heap that practically ends
around 2 GB. Lesson [36](36-cloud-tables.md) already made the WALK frugal (straight-write
`push_cloud`, no collect-then-extend, flat kernel accessors). This lesson kills the last
hidden copy: the upload itself.

## Where the copies hide

Loading one cloud file touches, in order: the fetch buffer (the .pb bytes), the protobuf
intermediate (prost's decoded message), the kernel `PointCloud` (f64 arrays), the GPU
tables (`cloud_pos/col/nrm`), and — the one people forget — **the upload staging copy**.
`wgpu::util::create_buffer_init` maps the whole buffer at creation, and on the web
backend that materialises a FULL-SIZE mirror of the contents in the wasm heap: a 127 MB
position table briefly costs 254 MB, and `set_scene` runs once per appended file, so a
three-cloud scene pays it three times.

## Step 1 — `storage_buffer` stages through the queue

The old doc comment goes with the old body: its "we still allocate one zeroed element" claim
becomes false, because the new code allocates nothing and lets WebGPU's zero-initialization
guarantee do the work. So the anchor starts at the `///`.

**Find** in `src/engine/gpu/mod.rs`, near the bottom:

```rust
/// A read-only storage buffer that is never zero-sized (wgpu can't bind a 0-byte buffer).
/// When `data` is empty we still allocate one zeroed element; the real element count is
/// tracked separately, so the draw call issues 0 instances and nothing renders.
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
/// A storage buffer filled by `write_buffer`, NOT `create_buffer_init`: init maps the whole
/// buffer at creation, and on wgpu's web backend that allocates a FULL-SIZE mirror of the
/// contents in the wasm heap - a 127 MB cloud table briefly costs 254 MB, three times per
/// scene load. `write_buffer` stages through the queue instead; an empty `data` leaves the
/// minimum-size buffer zero-initialized (a WebGPU guarantee).
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer {
    let size = (data.len() * std::mem::size_of::<T>()).max(std::mem::size_of::<T>()).max(4) as u64;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !data.is_empty() {
        queue.write_buffer(&buf, 0, bytemuck::cast_slice(data));
    }
    buf
}
```

## Step 2 — nothing to thread

The signature grew a `&wgpu::Queue`, so every caller has to pass one. **There are no callers.**

`cargo check` will tell you so:

```
warning: function `storage_buffer` is never used
```

That is correct, and it is worth understanding rather than papering over. On 2026-08-26 the
eight call sites this step used to list — the instance table, the three cloud tables, and the
lane tables either side of them — were rewritten to GROW instead of being rebuilt. They now go
through `append_rows`, which does what step 1 just taught (`create_buffer` with
`mapped_at_creation: false`, filled by `queue.write_buffer`) and then keeps doing it for every
appended file. You type that function in lesson [38](38-append-dont-rebuild.md); nothing to type
here — this is an excerpt of the tail of its body, where the staging write happens:

```rust
    queue.write_buffer(buf, *count as u64 * stride, bytemuck::cast_slice(data));
    *count += data.len() as u32;
```

So the mirror this lesson set out to kill is already dead on those tables, and killed harder:
rebuilding per file also re-sent every EARLIER file's rows, and needed the CPU-side table kept
alive to re-send them from. Appending drops both. Measured on the ten-sheet `drawings` scene,
2311 MB resident -> 881 MB; on a 13.8 M-point scan the CPU mirror alone was 263 MB.

**`storage_buffer` is not dead weight** — it is the right tool for a different job, a small table
of KNOWN maximum size reserved once so the per-frame path is a plain `write_buffer` and never a
reallocation. It gets used again in lesson [80](80-gumball-geometry.md) (`gumball.segments`,
512 rows), [86](86-draw-tools-2.md) (`preview.segments`, 4096) and [87](87-snapping.md)
(`snap.marker`). Leave the warning; it goes away there.

> Typing this chain against a clean end-of-35 checkout instead of the live tree? Then the eight
> call sites DO still exist, and each one takes `&device, &queue,` (or `&self.device,
> &self.queue,`) in place of `&device,` / `&self.device,`. Lessons 74, 80 and 81 still spell the
> old three-argument form and need the same edit.

## What this does NOT fix

The four copies BEFORE the tables are the loader's problem: fetch buffer and prost
intermediate die with scope, and the kernel Session stays alive because things read geometry
back out of it — picking, editing, saving, `Scene::rebuild`. A document that does NONE of those
is pure cost, which is what the manifest's `display_only` flag is for: it releases the Session
the moment the walk is done, and on ten drawing sheets that is most of the gigabyte. Streaming
past the kernel entirely — never building a `Session` for a scan at all — is lesson
[43](43-streaming-cloud.md)'s subject.

## The finished tree

`docs/37_cloud_memory/` is this lesson's end state as a complete crate — the same snapshot
convention as [`35_scene_struct/`](35_scene_struct/). Diff against it if a step did not take.

## Expected state

- `cargo check --target wasm32-unknown-unknown --lib`: one warning, `function
  `storage_buffer` is never used`, and nothing else. See step 2 — it is expected, and lesson
  67 is where it goes away.
- The lesson-36 lion render is BYTE-IDENTICAL — this lesson moves bytes, not pixels:

```
VIEWER_W=1200 VIEWER_H=800 VIEWER_ZOOM=6 VIEWER_ORBIT="25,-10" \
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- \
    out.ppm assets/scenes/lion.toml
# => non-background pixels: 189148 (19.7%)
```

- In the browser, the memory panel no longer spikes per appended file: the upload's
  full-size staging mirror is gone.

## Next

Lesson [38](38-append-dont-rebuild.md) — **Append, don't rebuild: Mat4 rows and growable lanes.** Adding a file should cost the file, not the scene.
