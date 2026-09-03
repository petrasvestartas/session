# 50 Performance and memory — measured, one item at a time

> Last refactor lesson. Start from the end of lesson 49. Unlike 45-49 this lesson CHANGES
> behaviour on purpose, one item per section, each with the numbers before and after. Every
> number is the median of two `--release` runs on an Intel iGPU with the load average recorded;
> a laptop of yours will differ in size, not in direction.

<svg viewBox="0 0 720 320" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="before: render requests the next frame forever; after: events set needs_frame and one frame is drawn; plus three measured memory fixes as bars" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="ob" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#888"/></marker><marker id="og" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#7ed37e"/></marker></defs>
  <text x="130" y="20" fill="#888" font-size="11" text-anchor="middle">before — 60 fps forever</text>
  <rect x="60" y="40" width="140" height="28" fill="none" stroke="#3a3a3a"/>
  <text x="130" y="58" fill="#d7dae0" font-size="10" text-anchor="middle">render()</text>
  <path d="M200,54 C250,54 250,110 130,110 C10,110 10,54 58,54" fill="none" stroke="#888" marker-end="url(#ob)"/>
  <text x="130" y="128" fill="#888" font-size="9" text-anchor="middle">request_redraw() at the end of every frame</text>
  <text x="130" y="140" fill="#888" font-size="9" text-anchor="middle">the camera is still, the GPU is not</text>
  <line x1="300" y1="10" x2="300" y2="150" stroke="#3a3a3a"/>
  <text x="510" y="20" fill="#7ed37e" font-size="11" text-anchor="middle">after — render on demand (lesson 50)</text>
  <rect x="320" y="40" width="120" height="40" fill="none" stroke="#f0b35c"/>
  <text x="380" y="56" fill="#d7dae0" font-size="10" text-anchor="middle">input · Msg</text>
  <text x="380" y="70" fill="#d7dae0" font-size="10" text-anchor="middle">stream · resize</text>
  <line x1="440" y1="60" x2="468" y2="60" stroke="#7ed37e" marker-end="url(#og)"/>
  <rect x="470" y="40" width="130" height="40" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="535" y="56" fill="#d7dae0" font-size="10" text-anchor="middle">State.needs_frame</text>
  <text x="535" y="70" fill="#d7dae0" font-size="10" text-anchor="middle">= true → redraw</text>
  <line x1="600" y1="60" x2="628" y2="60" stroke="#7ed37e" marker-end="url(#og)"/>
  <rect x="630" y="40" width="76" height="40" fill="none" stroke="#3a3a3a"/>
  <text x="668" y="56" fill="#d7dae0" font-size="10" text-anchor="middle">render()</text>
  <text x="668" y="70" fill="#888" font-size="9" text-anchor="middle">once, then stop</text>
  <text x="510" y="104" fill="#888" font-size="9" text-anchor="middle">render() never requests the next frame · ?perf=1 keeps requesting</text>
  <text x="510" y="118" fill="#888" font-size="9" text-anchor="middle">every handler sets needs_frame (lib.rs: request_if_needed)</text>
  <line x1="14" y1="160" x2="706" y2="160" stroke="#3a3a3a"/>
  <text x="14" y="182" fill="#888" font-size="10">three memory fixes, measured twice before and after (items 8-10)</text>
  <g fill="#d7dae0" font-size="10">
    <text x="14" y="212">per-object rows</text><text x="14" y="252">MSAA target</text><text x="14" y="292">instance re-anchor</text>
  </g>
  <g stroke="#0d0f12">
    <rect x="180" y="200" width="258" height="8" fill="#3a3a3a"/><rect x="180" y="211" width="78" height="8" fill="#2b4a2b"/>
    <rect x="180" y="240" width="258" height="8" fill="#3a3a3a"/><rect x="180" y="251" width="100" height="8" fill="#2b4a2b"/>
    <rect x="180" y="280" width="238" height="8" fill="#3a3a3a"/><rect x="180" y="291" width="42" height="8" fill="#2b4a2b"/>
  </g>
  <g fill="#888" font-size="9">
    <text x="444" y="207">997 B — objects_base, base_f32, bounds, inside</text>
    <text x="444" y="247">4x always, msaa texture at every size</text>
    <text x="424" y="287">68 MiB — every row rewritten (96 B) on re-anchor</text>
  </g>
  <g fill="#7ed37e" font-size="9">
    <text x="244" y="218">303 B — one owner: InstanceTable { rows, translation, bounded }</text>
    <text x="286" y="258">4x only for solids and w·h ≤ 4.2 Mpx, else 1x and no texture</text>
    <text x="228" y="298">11.4 MiB — translations buffer, 16 B/row; inside flips write only flipped rows</text>
  </g>
</svg>

## Goal

The viewer stops rendering when nothing changes, holds ~300 bytes per object instead of ~1,000,
uses 4x MSAA only where solids exist and the canvas is small enough, rewrites 16 bytes per row on
a re-anchor instead of 96, drops the file bytes as soon as they are decoded, streams colours in
slices like it streams positions, and grows every GPU table under one policy. Twelve items;
only item 5 moves golden rows, and only the `VIEWER_REBUILD=1` ones.

## Why

The audit in `docs/_PERF_AUDIT_45.md` measured where a weaker machine pays: a still scene drawn at
vsync forever, five CPU mirrors of every object row, 4x targets on pure sheets and on high-DPI
canvases, a 68 MB instance upload on every re-anchor. None of these is visible on a fast desktop
and all of them are visible on a laptop. The refactor made each one a change in one file.

## Files

Every item names its files. One example is new, `examples/probe_objects.rs` (bytes per object
through a counting allocator); `selftest.rs` gains the `VIEWER_GPU_REPORT` / `VIEWER_CLEAR`
switches. A listing that ends without its closing brace reuses the `}` already below the anchor.

## Item 1 — the splat prelude: test before you build

`splat_prelude` built both lanes' record tables every frame and then threw them away whenever the
camera was still. Now `is_current` is tested first, the record table is reused in place, and the
compute grid is sized to the point count instead of rounding up to a 4096-wide row (`splat.wgsl`
reads the row width from `num_workgroups`). Pixel-identical.

| lion (342k points) | before | after |
|---|---|---|
| still frame, encode | 0.03 ms | 0.01-0.02 ms |
| threads dispatched | 524,288 | 342,016 |

### `src/engine/gpu/render.rs`

**Find** in `src/engine/gpu/render.rs`:

```rust
use super::splat::{records, RecordCx};
```

**Replace with:**

```rust
use super::splat::RecordCx;
```

**Find** in `src/engine/gpu/render.rs`:

```rust
    fn splat_prelude(&mut self, encoder: &mut wgpu::CommandEncoder) {
```

**Add below it:**

```rust
        // Static skip FIRST: camera still, same scale, nothing rebuilt - the buffers already
        // hold this exact frame's splats, so not even the records are built.
        let (mvp, cloud_size) = (self.frame.mvp_f32, self.view.cloud_size);
        if self.splat.is_current(&mvp, cloud_size) {
            return;
        }

```

**Find** in `src/engine/gpu/render.rs`:

```rust
            cloud_size: self.view.cloud_size,
```

**Replace with:**

```rust
            cloud_size,
```

**Find** in `src/engine/gpu/render.rs`:

```rust
        let walked = records(&cx, &self.cloud.draws, &self.cloud.nodes);
        let streamed = records(&cx, &self.stream.draws, &[]);
        self.splat.walked.total = walked.total;
        self.splat.streamed.total = streamed.total;

        // Static skip: camera still, same scale, nothing rebuilt - the buffers already
        // hold this exact frame's splats, so the whole compute prelude is free.
        let (mvp, cloud_size) = (self.frame.mvp_f32, self.view.cloud_size);
        if self.splat.total() == 0 || self.splat.is_current(&mvp, cloud_size) {
            return;
        }
        self.splat.walked.write(&self.ctx, &walked);
        self.splat.streamed.write(&self.ctx, &streamed);
```

**Replace with:**

```rust
        self.splat.walked.build(&cx, &self.cloud.draws, &self.cloud.nodes);
        self.splat.streamed.build(&cx, &self.stream.draws, &[]);
        if self.splat.total() == 0 {
            return;
        }
        self.splat.walked.write(&self.ctx);
        self.splat.streamed.write(&self.ctx);
```

### `src/engine/gpu/splat.rs`

**Find** in `src/engine/gpu/splat.rs`:

```rust
/// One frame's records for one lane: the 4-word header {n, total, 0, 0}, the records, the threads.
```

**Add below it:**

```rust
/// Kept between frames and refilled in place: the table and the LOD stack keep their capacity, so
/// a rebuilt frame allocates nothing.
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
}

/// One lane's slot: its record table, group 0 (frame + records), group 1 (points + pixels), threads.
```

**Replace with:**

```rust
    stack: Vec<usize>,
}

impl Records {
    /// Empty the table for the next frame; capacity stays.
    fn clear(&mut self) {
        self.header = [0; 4];
        self.recs.clear();
        self.total = 0;
    }
}

/// One lane's slot: its record table on both sides, group 0 (frame + records), group 1 (points + pixels).
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    group1: wgpu::BindGroup,
```

**Add below it:**

```rust
    cpu: Records,
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        Self { recs, group0, group1, total: 0 }
```

**Replace with:**

```rust
        Self { recs, group0, group1, cpu: Records::default(), total: 0 }
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    /// Upload this frame's records: the header at 0, the records at 16.
    pub fn write(&self, ctx: &GpuCtx, r: &Records) {
        ctx.queue.write_buffer(&self.recs, 0, bytemuck::bytes_of(&r.header));
        ctx.queue.write_buffer(&self.recs, 16, bytemuck::cast_slice(&r.recs));
```

**Replace with:**

```rust
    /// Rebuild this lane's records for the frame in `cx`, in place; `total` follows.
    pub fn build(&mut self, cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode]) {
        records(cx, draws, nodes, &mut self.cpu);
        self.total = self.cpu.total;
    }

    /// Upload the records just built: the header at 0, the records at 16.
    pub fn write(&self, ctx: &GpuCtx) {
        ctx.queue.write_buffer(&self.recs, 0, bytemuck::bytes_of(&self.cpu.header));
        ctx.queue.write_buffer(&self.recs, 16, bytemuck::cast_slice(&self.cpu.recs));
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
/// and an oversized dispatch silently invalidates the WHOLE command buffer; 4096-wide rows cover any count.
fn dispatch_grid(n: u32) -> (u32, u32) {
    let g = n.div_ceil(64);
    (g.min(4096), g.div_ceil(4096))
```

**Replace with:**

```rust
/// and an oversized dispatch silently invalidates the WHOLE command buffer. The rows are as
/// narrow as the count allows: a full 4096-wide last row ran 53% idle threads on the lion.
fn dispatch_grid(n: u32) -> (u32, u32) {
    let g = n.div_ceil(64);
    let gy = g.div_ceil(4096);
    (g.div_ceil(gy), gy)
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
pub fn records(cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode]) -> Records {
    let mut out = Records::default();
```

**Replace with:**

```rust
pub fn records(cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode], out: &mut Records) {
    out.clear();
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
            walk_nodes(cx, &cloud, slice, &mut out);
        } else {
            push_record(&mut out, cloud.record(cx, d.first, d.count, d.spacing));
```

**Replace with:**

```rust
            walk_nodes(cx, &cloud, slice, out);
        } else {
            push_record(out, cloud.record(cx, d.first, d.count, d.spacing));
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    out.header[1] = out.total;
    out
```

**Replace with:**

```rust
    out.header[1] = out.total;
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
    let mut stack: Vec<usize> = vec![0];
    while let Some(ni) = stack.pop() {
```

**Replace with:**

```rust
    out.stack.clear();
    out.stack.push(0);
    while let Some(ni) = out.stack.pop() {
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
                if ch >= 0 { stack.push(ch as usize); }
```

**Replace with:**

```rust
                if ch >= 0 { out.stack.push(ch as usize); }
```

### `src/shaders/splat.wgsl`

**Find** in `src/shaders/splat.wgsl`:

```wgsl
// DIspatched a a 2D grid: 4096 workgroups wide, as many rows as needed - a 1D dispatch
// caps at 65535 workgroups (4.2M threads), well under a 7M-point frame, and an oversized
// dispatch invalidates the whole command buffer: the frame silently never draws.
const STRIDE: u32 = 4096u * 64u; // threads per grid row
```

**Replace with:**

```wgsl
// Dispatched as a 2D grid, at most 4096 workgroups wide and as narrow as the count allows
// (splat.rs `dispatch_grid`) - a 1D dispatch caps at 65535 workgroups (4.2M threads), well
// under a 7M-point frame, and an oversized dispatch invalidates the whole command buffer: the
// frame silently never draws. The row width comes from the dispatch itself.
const WG: u32 = 64u; // threads per workgroup
```

**Find** in `src/shaders/splat.wgsl`:

```wgsl
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_index) lane: u32){
    rasterize(prepare(g.y * STRIDE + g.x, lane), lane, false);
```

**Replace with:**

```wgsl
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) nw: vec3<u32>, @builtin(local_invocation_index) lane: u32){
    rasterize(prepare(g.y * nw.x * WG + g.x, lane), lane, false);
```

**Find** in `src/shaders/splat.wgsl`:

```wgsl
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    rasterize(prepare(g.y * STRIDE + g.x, lane), lane, true);
```

**Replace with:**

```wgsl
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) nw: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    rasterize(prepare(g.y * nw.x * WG + g.x, lane), lane, true);
```

## Item 2 — hygiene

The misspellings the moved comments carried, the draw-order and backdrop docstrings corrected
to what the code does, one timestamp per frame (`FrameInput.now_ms`), `Gpu::release` from
`Scene::clear`, `Line` ends read by index, and a `bench_load` leg that times that read.

| | before | after |
|---|---|---|
| Line ends, 947k lines | 5 ms | 0 ms |
| GPU after Clear, drawings_rotated | 132 MiB kept | 26 MiB |

### `examples/bench_load.rs`

**Find** in `examples/bench_load.rs`:

```rust
    println!("  length() only  {:>7.0} ms  (acc {acc2:.0})", t.elapsed().as_secs_f64()*1e3);
```

**Add below it:**

```rust
    // The walk's way since lesson 50: six floats by index, no Point, no String.
    let t = Instant::now();
    let mut acc3 = 0.0f64;
    for l in &s.objects.lines { acc3 += l[0] as f32 as f64 + l[3] as f32 as f64; }
    println!("  l[0]..l[5]     {:>7.0} ms  (acc {acc3:.0})", t.elapsed().as_secs_f64()*1e3);
```

### `src/app/scene.rs`

**Find** in `src/app/scene.rs`:

```rust
        self.hidden.clear();
        self.vert_base = 0;
        self.cloud_base = 0;
        gpu.reset_arena();
```

**Replace with:**

```rust
        self.hidden.clear();
        self.vert_base = 0;
        self.cloud_base = 0;
        gpu.release();
```

### `src/app/walk/curves.rs`

**Find** in `src/app/walk/curves.rs`:

```rust
/// One ribbon segment.
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    seg.ribbons.push(CylinderSegment {
        p0: l.start().to_f32(),
        radius: encode_width(l.width),
        p1: l.end().to_f32(),
```

**Replace with:**

```rust
/// One ribbon segment. The ends are read by index: `start()`/`end()` build a kernel `Point`
/// each (two Strings apiece), 947k allocations on one sheet for six floats.
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    seg.ribbons.push(CylinderSegment {
        p0: [l[0] as f32, l[1] as f32, l[2] as f32],
        radius: encode_width(l.width),
        p1: [l[3] as f32, l[4] as f32, l[5] as f32],
```

### `src/app/walk/mod.rs`

**Find** in `src/app/walk/mod.rs`:

```rust
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
    // 3D geometry takes the SOLID lane (edges are cylinders, vertices spheres); free
    // linework and points the FLAT lane; every cloud the splat lane. FLAG_OPEN for
    // `Mesh` objects only - an Element's mesh never raised it.
```

**Replace with:**

```rust
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

### `src/camera.rs`

**Find** in `src/camera.rs`:

```rust
        // iso start: yaw 45 deg about T, pitch -30 deg about the tileted right axis
```

**Replace with:**

```rust
        // iso start: yaw 45 deg about T, pitch -30 deg about the tilted right axis
```

**Find** in `src/camera.rs`:

```rust
            self.target[i] += cursor_off * (1.0 - k); // keeps the curson's world point fixed
```

**Replace with:**

```rust
            self.target[i] += cursor_off * (1.0 - k); // keeps the cursor's world point fixed
```

**Find** in `src/camera.rs`:

```rust
    /// panning theb costs 1x uniform instead of an instance-table rebuild.
```

**Replace with:**

```rust
    /// panning then costs 1x uniform instead of an instance-table rebuild.
```

### `src/engine/gpu/arena.rs`

**Find** in `src/engine/gpu/arena.rs`:

```rust
        self.text.reset();
    }

```

**Add below it:**

```rust
    /// Hand every buffer back: five one-row tables again, as `new` made them.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.verts.release(ctx);
        self.vids.release(ctx);
        self.faces.release(ctx);
        self.print.release(ctx);
        self.text.release(ctx);
    }

```

### `src/engine/gpu/backdrop.rs`

**Find** in `src/engine/gpu/backdrop.rs`:

```rust
/// Grid first as the depth writes are off, all objects paints over it; the line block carries the anchor.
```

**Replace with:**

```rust
/// The grid draws first: its depth writes are off, so every object paints over it. The line
/// block carries the anchor it has to subtract. Always 1 draw.
```

### `src/engine/gpu/buffers.rs`

**Find** in `src/engine/gpu/buffers.rs`:

```rust
        self.len = 0;
    }

```

**Add below it:**

```rust
    /// Forget the rows AND the buffer: back to the one zeroed row `new` made, so a cleared
    /// scene holds no GPU memory. The caller rebuilds any bind group over it.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.buf = zeroed_buffer(&ctx.device, self.label, self.stride, self.usage);
        self.len = 0;
        self.cap = 1;
    }

```

### `src/engine/gpu/cloud.rs`

**Find** in `src/engine/gpu/cloud.rs`:

```rust
        self.nodes.clear();
    }

```

**Add below it:**

```rust
    /// Hand every buffer and both lists back; the caller rebinds the splat groups.
    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.pos.release(ctx);
        self.col.release(ctx);
        self.nrm.release(ctx);
        self.draws.shrink_to_fit();
        self.nodes.shrink_to_fit();
    }

```

### `src/engine/gpu/frame.rs`

**Find** in `src/engine/gpu/frame.rs`:

```rust
/// What one frame needs from the camera, computed once per frame by the caller.
```

**Replace with:**

```rust
/// What one frame needs from the caller: the camera, the clear colour and the frame's ONE
/// timestamp (ms) - the re-anchor throttle and the fps counter both read it, neither reads a clock.
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
    pub clear: wgpu::Color,
```

**Add below it:**

```rust
    pub now_ms: f64,
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
```

**Replace with:**

```rust
    thickness: f32, // on-screen width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half-height x unit scale
```

### `src/engine/gpu/glyphs.rs`

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    pub instance_id: u32, // 4 B - row insntaces
```

**Replace with:**

```rust
    pub instance_id: u32, // 4 B - row in instances[]
```

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
        self.dots.reset();
    }

```

**Add below it:**

```rust
    /// Hand both buffers back (one-row tables again) and re-point the groups at them.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.spheres.release(ctx);
        self.dots.release(ctx);
        self.sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &self.spheres.buf);
        self.dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &self.dots.buf);
    }

```

### `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// (progressive loading calls it once per appended file), One code path, not two.
```

**Replace with:**

```rust
    /// (progressive loading calls it once per appended file). One code path, not two.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// A rebase moves every instance model, so the splats are stale.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point {
        let (anchor, moved) = self.objects.rebase_anchor(&self.ctx, origin, view_dist);
```

**Replace with:**

```rust
    /// A rebase moves every instance model, so the splats are stale. `now` is the frame's one
    /// timestamp (ms), read once by the caller.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Point {
        let (anchor, moved) = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.cloud.reset();
    }

```

**Add below it:**

```rust
    /// Forget every family's rows AND hand their memory back, CPU mirrors and GPU buffers alike:
    /// one-row placeholders again, as `build` made them. `reset_arena` keeps capacity for a
    /// rebuild; a cleared scene has nothing to rebuild and must not stay resident.
    pub fn release(&mut self) {
        self.objects.release(&self.ctx, &self.layouts);
        self.arena.release(&self.ctx);
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
        self.cloud.release(&self.ctx);
        self.rebind_splat();
        self.splat.invalidate();
    }

```

### `src/engine/gpu/objects.rs`

**Find** in `src/engine/gpu/objects.rs`:

```rust
//! reads: their f64 mirrors, the re-anchor, the inside test, the buffer and its bind group.

use crate::engine::performance::now_ms;
```

**Replace with:**

```rust
//! reads: their f64 mirrors, the re-anchor, the inside test, the buffer and its bind group.

```

**Find** in `src/engine/gpu/objects.rs`:

```rust
/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoon only changes the view matrix.
/// f32 error at 1e5 mm from the achor = 6e-3 mm - far below a pixel.
/// Re-anchor threshold, WORLD units (mm): a quarter of the current view distance, so a zoomed-out
/// pan does not rebuild constantly while a zoomed-IN pan re-anchors early enough that world
/// coordinates never regain the magnitude that eats f32 precision. Clamped to a sane band.
```

**Replace with:**

```rust
/// Re-anchor threshold band, WORLD units (mm): the table is rebased once the camera target
/// drifts a quarter of the view distance from the anchor, clamped to [MIN, MAX] - a zoomed-out
/// pan does not rebuild constantly, a zoomed-in pan re-anchors before f32 precision goes.
/// Within the band only the view matrix changes; f32 error at 1e5 mm from the anchor = 6e-3 mm.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        // instead of per re-achor: at 210k objects that turns a 20+ msCPU loop into a copy
```

**Replace with:**

```rust
        // instead of per re-anchor: at 210k objects that turns a 20+ ms CPU loop into a copy
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs
    /// only when the camera target strays REANCHOR_DIST from the current anchor - orbit newer moves the target.
    /// And pan/zoom within the budget just changes the view matrix
```

**Replace with:**

```rust
    /// The anchor the instance table is rebased about. A full rebuild runs only when the camera
    /// target strays past the `REANCHOR_MIN`/`REANCHOR_MAX` band from the current anchor - orbit
    /// never moves the target, and pan/zoom within the band just changes the view matrix.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64) -> (Point, bool) {
```

**Replace with:**

```rust
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64, now: f64) -> (Point, bool) {
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        // Throttled: during a wheel-zoom gesture the target moves every tick,
        // and an every-frame rebuild is the motion jank the rule forbids.
        // Between rebuulds the old achor stays valid - it is just farther from the eye than the threshold likes, which costs f32 precision
        // only past the threshold distance, never a wrong image.
        let now = now_ms();
```

**Replace with:**

```rust
        // Throttled: during a wheel-zoom gesture the target moves every tick, and an every-frame
        // rebuild is the motion jank the rule forbids. Between rebuilds the old anchor stays
        // valid - farther from the eye than the band likes costs f32 precision, never a wrong image.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'objects_base'
    /// Then cast to f32.
    /// 'instances', what GPU actually sees, never holds a coordinate bigger than the camera's distnace from 'origin',
    /// no matter how fas the scene fists from world (0,0,0).
```

**Replace with:**

```rust
    /// Rebase every instance's translation around `origin`: an f64 subtract against the TRUE
    /// world transform in `objects_base`, then the cast to f32. What the GPU sees never holds a
    /// coordinate bigger than the camera's distance from `origin`, however far the scene sits from (0,0,0).
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            let mut m = self.base_f32[i]; // rotation / scale casr once at set_scene
```

**Replace with:**

```rust
            let mut m = self.base_f32[i]; // rotation / scale cast once at set_scene
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.instances));
        }
```

**Add below it:**

```rust
    }

    /// Forget every row AND hand the memory back, both sides: the one-row placeholder again,
    /// so a cleared scene holds nothing (`reset` keeps capacity for a rebuild).
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.objects_base.shrink_to_fit();
        self.base_f32.shrink_to_fit();
        self.object_bounds_world.shrink_to_fit();
        self.inside.shrink_to_fit();
        self.bounded_rows.shrink_to_fit();
        self.instances.shrink_to_fit();
        self.instances.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.group = rows_group(ctx, &l.instance, "instances.bind_group", &self.buffer.buf);
```

### `src/engine/gpu/present.rs`

**Find** in `src/engine/gpu/present.rs`:

```rust
use session_rust::Xform;
```

**Add below it:**

```rust
use crate::engine::performance::now_ms;
```

**Find** in `src/engine/gpu/present.rs`:

```rust
        self.performance.frame(draws, objects);
```

**Replace with:**

```rust
        self.performance.frame(draws, objects, input.now_ms);
```

**Find** in `src/engine/gpu/present.rs`:

```rust
        let input = FrameInput { view_proj: view_proj.clone(), clear };
```

**Replace with:**

```rust
        let input = FrameInput { view_proj: view_proj.clone(), clear, now_ms: now_ms() };
```

### `src/engine/gpu/render.rs`

**Find** in `src/engine/gpu/render.rs`:

```rust
    /// background -> grid -> triangles -> sphere markers -> cylinders -> CLOUD -> ink
    /// prepass -> ribbon -> glyph. Everything that WRITES depth comes first (the cloud
```

**Replace with:**

```rust
    /// background -> grid -> faces -> print -> pipes -> CLOUD -> sphere markers -> ink
    /// prepass -> ribbons -> text -> dots. Everything that WRITES depth comes first (the cloud
```

### `src/engine/gpu/segments.rs`

**Find** in `src/engine/gpu/segments.rs`:

```rust
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
```

**Replace with:**

```rust
    pub radius: f32,    // 4 B - 0.0 = screen-constant px (default); > 0 = world mm override
```

**Find** in `src/engine/gpu/segments.rs`:

```rust
        self.ribbons.reset();
    }

```

**Add below it:**

```rust
    /// Hand both buffers back (one-row tables again) and re-point the groups at them.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.pipes.release(ctx);
        self.ribbons.release(ctx);
        self.pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &self.pipes.buf);
        self.ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &self.ribbons.buf);
    }

```

### `src/engine/performance.rs`

**Find** in `src/engine/performance.rs`:

```rust
    last_log: f64, // ms timestamp of the last ocnsole line
```

**Replace with:**

```rust
    last_log: f64, // ms timestamp of the last console line
```

**Find** in `src/engine/performance.rs`:

```rust
    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32){
        let t = now_ms();
```

**Replace with:**

```rust
    /// Call once at the end of every frame with the counts gathered during it and the frame's
    /// timestamp `t` (ms) - the one the caller read once for the whole frame.
    pub fn frame(&mut self, draws: u32, objects: u32, t: f64){
```

**Find** in `src/engine/performance.rs`:

```rust
        // exponential moving average - one raw frame is too jiterry to show as fps
```

**Replace with:**

```rust
        // exponential moving average - one raw frame is too jittery to show as fps
```

### `src/selftest.rs`

**Find** in `src/selftest.rs`:

```rust
use crate::engine::gpu::{FrameInput, Gpu};
```

**Add below it:**

```rust
use crate::engine::performance::now_ms;
```

**Find** in `src/selftest.rs`:

```rust
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
    let view_proj = camera.view_proj_anchored(w as f64 / h as f64, &anchor);
    let input = FrameInput { view_proj, clear: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 } };
```

**Replace with:**

```rust
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms());
    let view_proj = camera.view_proj_anchored(w as f64 / h as f64, &anchor);
    let input = FrameInput { view_proj, clear: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, now_ms: now_ms() };
```

**Find** in `src/selftest.rs`:

```rust
        let origin = camera.origin();
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
```

**Replace with:**

```rust
        let origin = camera.origin();
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms());
```

**Find** in `src/selftest.rs`:

```rust
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world());
            let vp = camera.view_proj_anchored(aspect, &anchor);

            let input = FrameInput { view_proj: vp, clear };
```

**Replace with:**

```rust
            let now = now_ms();
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now);
            let vp = camera.view_proj_anchored(aspect, &anchor);

            let input = FrameInput { view_proj: vp, clear, now_ms: now };
```

### `src/state.rs`

**Find** in `src/state.rs`:

```rust
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world());
```

**Replace with:**

```rust
        let now_ms = now_ms();
        let aspect = self.gpu.config.width as f64 / self.gpu.config.height as f64;
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world(), now_ms);
```

**Find** in `src/state.rs`:

```rust
        self.gpu.clear(&FrameInput { view_proj, clear })
```

**Replace with:**

```rust
        self.gpu.clear(&FrameInput { view_proj, clear, now_ms })
```

## Item 3 — the harness tells the truth

`bench_scene` printed zeros because the tables were already dropped; it prints the GPU counts now.
`VIEWER_GPU_REPORT=1` and `VIEWER_CLEAR=1` report wgpu's allocations per label, and a `rebase:`
leg at the end of `bench_frame` forces ten re-anchors.

### `src/selftest.rs`

**Find** in `src/selftest.rs`:

```rust
use session_rust::{Session, Xform};
```

**Replace with:**

```rust
use session_rust::{Point, Session, Xform};
```

**Find** in `src/selftest.rs`:

```rust
fn rss_mb() -> f64 { 0.0 }
```

**Add below it:**

```rust

/// `VIEWER_GPU_REPORT=1`: wgpu's allocator report, one line per buffer/texture LABEL (bytes
/// summed over its allocations, largest first) and the totals - what the GPU actually holds,
/// which no CPU-side count can tell. `None` (a backend without the report) says so.
fn gpu_report(gpu: &Gpu) {
    if std::env::var("VIEWER_GPU_REPORT").is_err() {
        return;
    }
    let Some(report) = gpu.ctx.device.generate_allocator_report() else {
        println!("gpu report: unavailable on this backend");
        return;
    };
    let mut by_label: std::collections::BTreeMap<String, (u64, usize)> = std::collections::BTreeMap::new();
    for a in &report.allocations {
        let e = by_label.entry(a.name.clone()).or_insert((0, 0));
        e.0 += a.size;
        e.1 += 1;
    }
    let mut rows: Vec<(String, u64, usize)> = by_label.into_iter().map(|(k, (b, n))| (k, b, n)).collect();
    rows.sort_by(|a, b| b.1.cmp(&a.1));
    println!("gpu report: {} allocations, {:.1} MiB allocated, {:.1} MiB reserved in {} blocks",
        report.allocations.len(), mb(report.total_allocated_bytes as usize), mb(report.total_reserved_bytes as usize), report.blocks.len());
    for (label, bytes, n) in rows {
        println!("  {label:<28} {:>9.2} MiB  x{n}", mb(bytes as usize));
    }
}
```

**Find** in `src/selftest.rs`:

```rust
    // so the wall clock includes the gpu actually finishing and reports the median
```

**Add below it:**

```rust
    // The camera is STILL across these frames, so the splat static skip applies: a cloud scene
    // measures its resolve, not its compute. `bench_frame` has the moving leg.
```

**Find** in `src/selftest.rs`:

```rust
        println!("frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1} | cloud scale x{}",
```

**Replace with:**

```rust
        println!("frames (still camera): n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1} | cloud scale x{}",
```

**Find** in `src/selftest.rs`:

```rust
    write_ppm(out, &rgba, w, h).expect("write ppm");
```

**Add below it:**

```rust
    gpu_report(&gpu);

    // VIEWER_CLEAR=1 clears the scene after the frame and reports again: what a cleared scene
    // still holds, on both sides, is what `Gpu::release` exists to hand back.
    if std::env::var("VIEWER_CLEAR").is_ok() {
        scene.clear(&mut gpu);
        let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        println!("after clear: RSS {:.1} MB", rss_mb() - rss0);
        gpu_report(&gpu);
    }
```

**Find** in `src/selftest.rs`:

```rust
    {
        let t = &scene.tables;
        println!("scene: {} edges ({:.1} MB segments), {} markers, {} verts",
            t.seg.pipes.len(), t.seg.pipes.len() as f64 * 40.0 / 1.048576e6, t.glyph.spheres.len(), t.arena.verts.len());
    }
```

**Replace with:**

```rust
    // GPU-side counts: the CPU tables are dropped by the upload, so they would all print 0.
    println!("scene: {} edges ({:.1} MB segments), {} markers, {} verts | {} ribbons, {} dots, {} cloud points",
        gpu.segments.pipe_count(), gpu.segments.pipe_count() as f64 * 40.0 / 1.048576e6, gpu.glyphs.sphere_count(), gpu.arena.vert_count(),
        gpu.segments.ribbon_count(), gpu.glyphs.dot_count(), gpu.cloud.point_count);
```

**Find** in `src/selftest.rs`:

```rust
            u + e + g, 1000.0 / (u + e + g)));
    }
```

**Add below it:**

```rust
    out.push_str(&rebase_profile(&mut gpu, &camera));
    gpu_report(&gpu);
```

**Find** in `src/selftest.rs`:

```rust
    gpu_report(&gpu);
    out
}

```

**Add below it:**

```rust
/// What one forced re-anchor costs: the CPU loop over every object row plus the write
/// (`rebase`), then the submit + poll that lands it (`gpu`). Ten of them, the origin thrown past
/// the threshold band and the clock past the 200 ms throttle each time, medians reported.
fn rebase_profile(gpu: &mut Gpu, camera: &Camera) -> String {
    let base = camera.origin();
    let (mut cpu, mut gpu_ms) = (Vec::new(), Vec::new());
    for i in 0..10 {
        let far = if i % 2 == 0 { 1.0e6 } else { 0.0 };
        let origin = Point::new(base[0] + far, base[1], base[2]);
        let t0 = std::time::Instant::now();
        let _ = gpu.rebase_anchor(&origin, camera.distance_world(), 1.0e5 * (i + 1) as f64);
        let t1 = std::time::Instant::now();
        gpu.ctx.queue.submit([]);
        let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        cpu.push((t1 - t0).as_secs_f64() * 1000.0);
        gpu_ms.push(t1.elapsed().as_secs_f64() * 1000.0);
    }
    format!("rebase: {} rows | cpu+write {:6.2} ms | gpu {:6.2} ms\n", gpu.objects.len(), median(&mut cpu), median(&mut gpu_ms))
}

```

## Item 4 — the load peak

The loader hands the fetched bytes to `decode` by value, and `decode` drops them the moment prost
is done: the peak is proto + kernel, not file + proto + kernel. The walk reserves its tables from
counts it already knows.

| drawings_rotated, 5 files | before | after |
|---|---|---|
| resident after the walks | 684 MB | 656 MB |
| four sheet walks | 82+191+132+99 ms | 71+166+127+67 ms |

### `src/app/decode.rs`

**Find** in `src/app/decode.rs`:

```rust
/// files stay on the synchronous path (they are small).
pub async fn session_from_bytes_chunked(url: &str, bytes: &[u8]) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(bytes));
    }
    let Ok(p) = proto::Session::decode(bytes) else { return Session::default() };
```

**Replace with:**

```rust
/// files stay on the synchronous path (they are small). The bytes are taken BY VALUE and dropped
/// the moment prost is done: the file, the proto and the kernel objects never coexist.
pub async fn session_from_bytes_chunked(url: &str, bytes: Vec<u8>) -> Session {
    if url.ends_with(".json") {
        return Session::file_json_loads(&String::from_utf8_lossy(&bytes));
    }
    let Ok(p) = proto::Session::decode(&bytes[..]) else { return Session::default() };
    drop(bytes);
```

### `src/app/loader.rs`

**Find** in `src/app/loader.rs`:

```rust
        let session = session_from_bytes_chunked(&item.file, &bytes).await;
```

**Replace with:**

```rust
        let session = session_from_bytes_chunked(&item.file, bytes).await;
```

**Find** in `src/app/loader.rs`:

```rust
    let session = session_from_bytes_chunked(&item.file, &bytes).await;
```

**Replace with:**

```rust
    let nbytes = bytes.len();
    let session = session_from_bytes_chunked(&item.file, bytes).await;
```

**Find** in `src/app/loader.rs`:

```rust
    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), bytes.len(), f1 - f0, now_ms() - f1);
```

**Replace with:**

```rust
    log::info!("loaded '{}': {} objects, {} bytes | fetch {:.0}ms · parse {:.0}ms", name, session.lookup.len(), nbytes, f1 - f0, now_ms() - f1);
```

### `src/app/scene.rs`

**Find** in `src/app/scene.rs`:

```rust
        let t = &mut self.tables;
```

**Add below it:**

```rust
        let count = session.lookup.len();
        t.obj.rows.reserve(count);
        t.obj.bounds.reserve(count);
        t.obj.spacing.reserve(count);
        self.order.reserve(count);
        self.guid_to_row.reserve(count);
```

### `src/app/walk/mesh.rs`

**Find** in `src/app/walk/mesh.rs`:

```rust
    let mut hi = [f32::NEG_INFINITY; 3];
```

**Add below it:**

```rust
    arena.verts.reserve(rm.vertices.len());
    arena.vids.reserve(rm.vertices.len());
```

**Find** in `src/app/walk/mesh.rs`:

```rust
    let idx = index_run(arena, m, o.sheet_lanes && print);
```

**Add below it:**

```rust
    idx.reserve(rm.indices.len());
```

### `src/app/walk/mesh_ink.rs`

**Find** in `src/app/walk/mesh_ink.rs`:

```rust
    let black_wire = edges.len() >= WIREFRAME_BLACK_MIN;

```

**Add below it:**

```rust
    // Upper bounds from the topology: a segment per edge, a marker per vertex.
    ink.seg.pipes.reserve(edges.len());
    ink.glyph.spheres.reserve(cx.vpos.len());
```

## Item 5 — release what nothing reads

A sheet or a cloud is looked at, never picked or saved, so its kernel `Session` is dead weight
after the walk. Every shipped sheet and cloud item gets `display_only = true`, and `decode` skips
the tree, the graph and the BVH on the wire for such files (`LeanSession`).

| | before | after |
|---|---|---|
| sheet decode | 194 ms | 125 ms |
| drawings_rotated resident, 5 files | 656 MB | 285-377 MB |

**Find** in `assets/scenes/bunny_cloud.toml`:

```toml
point_size = 6
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/lion.toml`:

```toml
point_size = 4
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/lidar14.toml`:

```toml
stream = true
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/bunny_drawings.toml`:

```toml
point_size = 6
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/bunny_drawings.toml`:

```toml
point_size = 4
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/drawings_rotated.toml`:

```toml
at = [0, 0, 0]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/drawings_rotated.toml`:

```toml
xform = [1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 3400, 0, 0, 1]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/drawings_rotated.toml`:

```toml
xform = [1, 0, 0, 0, 0, 0.7071068, 0.7071068, 0, 0, -0.7071068, 0.7071068, 0, 7200, 0, 0, 1]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/drawings_rotated.toml`:

```toml
xform = [0.8660254, 0.5, 0, 0, -0.5, 0.8660254, 0, 0, 0, 0, 1, 0, 10000, 0, 0, 1]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
at = [0, -26000, 0]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
at = [3400, -26000, 0]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
at = [6200, -26000, 0]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
at = [10000, -26000, 0]
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
point_size = 1
stream = true
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
point_size = 3
```

**Add below it:**

```toml
display_only = true
```

**Find** in `assets/scenes/cloud_mix.toml`:

```toml
point_size = 6
stream = true
```

**Add below it:**

```toml
display_only = true
```

### `examples/bench_load.rs`

**Find** in `examples/bench_load.rs`:

```rust
    let t = Instant::now();
    let lean = session_rust::proto::Session::decode(&bytes[..]).unwrap();
```

**Replace with:**

```rust
    // What a display_only document decodes since lesson 50: no tree, no graph, no bvh boxes.
    let t = Instant::now();
    let lean = session_viewer::app::decode::LeanSession::decode(&bytes[..]).unwrap();
```

### `src/app/decode.rs`

**Find** in `src/app/decode.rs`:

```rust
const CHUNK: usize = 25_000;

```

**Add below it:**

```rust
/// The wire `Session` without its `tree` (4), `graph` (5) and `bvh_boxes` (6): prost skips a
/// field it is not asked for without allocating, and a display-only document never reads those
/// three - on a sheet they are 52% of the decoded session. Same tags as `proto::Session`.
#[derive(Clone, PartialEq, prost::Message)]
pub struct LeanSession {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub guid: String,
    #[prost(message, optional, tag = "3")]
    pub objects: Option<proto::Objects>,
    #[prost(message, repeated, tag = "7")]
    pub xforms: Vec<proto::XformEntry>,
}

/// The wire session as `proto::Session`, whole or lean: a display-only document skips the
/// tree and the graph on the wire, not just in the kernel object.
fn decode_wire(bytes: &[u8], display_only: bool) -> Option<proto::Session> {
    if !display_only {
        return proto::Session::decode(bytes).ok();
    }
    let lean = LeanSession::decode(bytes).ok()?;

    Some(proto::Session { name: lean.name, guid: lean.guid, objects: lean.objects, tree: None, graph: None, bvh_boxes: Vec::new(), xforms: lean.xforms })
}

```

**Find** in `src/app/decode.rs`:

```rust
pub async fn session_from_bytes_chunked(url: &str, bytes: Vec<u8>) -> Session {
```

**Replace with:**

```rust
pub async fn session_from_bytes_chunked(url: &str, bytes: Vec<u8>, display_only: bool) -> Session {
```

**Find** in `src/app/decode.rs`:

```rust
    let Ok(p) = proto::Session::decode(&bytes[..]) else { return Session::default() };
```

**Replace with:**

```rust
    let Some(p) = decode_wire(&bytes, display_only) else { return Session::default() };
```

### `src/app/loader.rs`

**Find** in `src/app/loader.rs`:

```rust
        let session = session_from_bytes_chunked(&item.file, bytes).await;
```

**Replace with:**

```rust
        let session = session_from_bytes_chunked(&item.file, bytes, item.display_only).await;
```

**Find** in `src/app/loader.rs`:

```rust
    let session = session_from_bytes_chunked(&item.file, bytes).await;
```

**Replace with:**

```rust
    let session = session_from_bytes_chunked(&item.file, bytes, item.display_only).await;
```

## Item 6 — one growth policy

`GrowBuf` grows to `max(need, cap * 3 / 2)` everywhere, the arena included; the exact-fit variant
goes.

| incremental upload, sum | before | after |
|---|---|---|
| drawings (10 sheets) | 1883 ms, 389 MiB | 1424 ms, 383 MiB |
| drawings_rotated (4 sheets) | 358 ms | 417 ms |

### `src/engine/gpu/arena.rs`

**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The arena on the GPU. `verts`/`vids`/`faces` grow EXACT-fit: this is the biggest table in
/// the viewer (64 MB of vertices on a six-file scene) and it grows once per file, so doubling
/// would hold up to 2x the geometry for nothing. The two sheet runs double like every lane.
```

**Replace with:**

```rust
/// The arena on the GPU: five `GrowBuf`s under the one growth policy (`max(need, cap * 3/2)`).
/// This is the biggest table in the viewer (64 MB of vertices on a six-file scene); it used to
/// grow exact-fit, and every appended file then copied the whole table.
```

**Find** in `src/engine/gpu/arena.rs`:

```rust
            verts: GrowBuf::new_exact(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: GrowBuf::new_exact(ctx, "arena.vids", 4, vu),
            faces: GrowBuf::new_exact(ctx, "arena.ibo", 4, iu),
```

**Replace with:**

```rust
            verts: GrowBuf::new(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: GrowBuf::new(ctx, "arena.vids", 4, vu),
            faces: GrowBuf::new(ctx, "arena.ibo", 4, iu),
```

### `src/engine/gpu/buffers.rs`

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// A growable GPU table: capacity doubles when it runs out (or grows exact-fit, see `new_exact`),
/// the live prefix is copied GPU-side and only the new rows are written. Appending is what lets the CPU copy go after upload -
/// a lane that rebuilt its whole buffer per file had to keep every row twice.
```

**Replace with:**

```rust
/// A growable GPU table: ONE growth policy for every lane - capacity grows to
/// `max(need, cap * 3/2)`, the live prefix is copied GPU-side and only the new rows are written.
/// Appending is what lets the CPU copy go after upload. The arena used to grow exact-fit, which
/// copied the whole table per file (drawings: 65-525 ms per upload); 3/2 bounds the slack to 50%.
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
    label: &'static str,
    exact: bool,
```

**Replace with:**

```rust
    label: &'static str,
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
        Self { buf, len: 0, cap: 1, stride, usage, label, exact: false }
    }

    /// The same table with EXACT-fit growth: capacity becomes what is needed, never more. For
    /// a table that grows once per file and dwarfs every other (the mesh arena), doubling would
    /// hold up to 2x the geometry for nothing; the price is one GPU-side copy per append.
    pub fn new_exact(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        Self { exact: true, ..Self::new(ctx, label, stride, usage) }
```

**Replace with:**

```rust
        Self { buf, len: 0, cap: 1, stride, usage, label }
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
            let new_cap = if self.exact { need } else { need.max(self.cap * 2) };
```

**Replace with:**

```rust
            let new_cap = need.max(self.cap * 3 / 2);
```

## Item 7 — streamed colours in slices

Positions already arrived in 8 MiB slices; the colour run was fetched whole. `ColorRun` carries
the varint a slice boundary splits, and each slice becomes one `Msg::CloudCol`.

| lidar_14m colours | before | after |
|---|---|---|
| transient | 148 MB | 8 MiB raw + 8 MiB carry + one slice |

### `src/app/loader.rs`

**Find** in `src/app/loader.rs`:

```rust
use super::stream::{cloud_colors, cloud_fields, positions_from, CloudFields};
```

**Replace with:**

```rust
use super::stream::{cloud_fields, positions_from, CloudFields, ColorRun, SLICE_BYTES};
```

**Find** in `src/app/loader.rs`:

```rust
/// small Range reads find the packed arrays, the coords run streams, the colours follow whole.
```

**Replace with:**

```rust
/// small Range reads find the packed arrays, then the coords run and the colours run stream.
```

**Find** in `src/app/loader.rs`:

```rust
    if let Some(col) = cloud_colors(&item.file, f.colors_at, f.colors_len, f.count).await {
        let _ = proxy.send_event(Msg::CloudCol(col));
    }
```

**Replace with:**

```rust
    stream_colors(proxy, &item.file, &f).await;
```

**Find** in `src/app/loader.rs`:

```rust
    // 8 MB, rounded DOWN to a whole number of points: a slice boundary can then never fall
    // inside a point, let alone inside one of its doubles.
    const SLICE: u64 = (8 * 1024 * 1024 / 24) * 24;
```

**Replace with:**

```rust
    // Rounded DOWN to a whole number of points: a slice boundary can then never fall inside a
    // point, let alone inside one of its doubles.
    const SLICE: u64 = (SLICE_BYTES / 24) * 24;
```

**Find** in `src/app/loader.rs`:

```rust
    Aabb { min: lo, max: hi }
}

```

**Add below it:**

```rust
/// The colours run in the same 8 MiB slices, the same pipelining; one `Msg::CloudCol` per
/// slice, the split varint at each boundary carried by `ColorRun`. The GPU rows fill in behind
/// the positions as each slice lands, at the offset `StreamLane::push_col` keeps.
async fn stream_colors(proxy: &EventLoopProxy<Msg>, url: &str, f: &CloudFields) {
    let (mut at, mut left) = (f.colors_at, f.colors_len);
    let mut run = ColorRun::new(f.count);
    let mut inflight = if left > 0 {
        fetch_range_start(url, at, SLICE_BYTES.min(left)).ok()
    } else {
        None
    };
    while let Some(f_in) = inflight.take() {
        let n = SLICE_BYTES.min(left);
        at += n;
        left -= n;
        inflight = if left > 0 {
            fetch_range_start(url, at, SLICE_BYTES.min(left)).ok()
        } else {
            None
        };
        let Ok(raw) = fetch_range_finish(f_in).await else { break };
        let col = run.decode(&raw);
        drop(raw);
        if !col.is_empty() {
            let _ = proxy.send_event(Msg::CloudCol(col));
        }
        next_tick().await;
    }
}

```

### `src/app/stream.rs`

**Find** in `src/app/stream.rs`:

```rust

use super::fetch::fetch_range;
```

**Replace with:**

```rust
//! Colours are packed VARINTS, so their slices carry a split varint's tail across (`ColorRun`).

use super::fetch::fetch_range;

/// One Range read: 8 MiB. The coords loop rounds it down to whole points; the colour loop
/// takes it as is, since `ColorRun` carries a split varint over the boundary.
pub const SLICE_BYTES: u64 = 8 * 1024 * 1024;
```

**Find** in `src/app/stream.rs`:

```rust
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

**Replace with:**

```rust
/// The `colors` run decoded slice by slice. Packed uint32 is VARINT on the wire - not
/// memcpy-able the way `coords` is - so a slice boundary can fall inside a point's four
/// varints: the undecoded tail of every slice is carried into the next. Fetching the run
/// whole cost 148 MB of transient on a 14M-point scan; a slice costs 8 MiB.
pub struct ColorRun {
    carry: Vec<u8>,
    left: u32,
}

impl ColorRun {
    /// `count` points still to decode, nothing carried.
    pub fn new(count: u32) -> Self {
        Self { carry: Vec::new(), left: count }
    }

    /// Every WHOLE point in the carried tail + `raw`, packed to RGBA8; what is left over (at
    /// most one point's bytes) waits for the next slice. Empty once `count` points are out.
    pub fn decode(&mut self, raw: &[u8]) -> Vec<u32> {
        self.carry.extend_from_slice(raw);
        let buf = std::mem::take(&mut self.carry);
        let mut out = Vec::with_capacity((buf.len() / 4).min(self.left as usize));
        let mut i = 0usize;
        while self.left > 0 {
            let Some((rgba, n)) = point_rgba(&buf, i) else { break };
            out.push(rgba);
            i += n;
            self.left -= 1;
        }
        self.carry = buf[i..].to_vec();
        out
    }
}

/// One point's four varints at `i`, packed RGBA8, and the bytes they took; `None` when the
/// buffer ends inside them - the caller carries the tail.
fn point_rgba(b: &[u8], mut i: usize) -> Option<(u32, usize)> {
    let start = i;
    let mut rgba = [255u8; 4];
    for k in 0..4 {
        let (v, n) = varint(b, i)?;
        i += n;
        rgba[k] = (v & 255) as u8;
    }
    Some((u32::from_le_bytes(rgba), i - start))
}

```

## Item 8 — one owner per object row

`InstanceTable` keeps the row, a `[f64; 3]` translation and a sparse list of bounded rows;
`objects_base`, `base_f32`, the dense world boxes and the `inside` vector go. `ObjectRows` is a
per-upload delta, so `vert_base`/`cloud_base` become `Bases { vert, cloud, obj }` and
`Baselines::placement` maps a global id back into this upload's delta.

| drawings_rotated, 155,465 rows | before | after |
|---|---|---|
| bytes per object | 997 (table 556 + scene 441) | 303 (181 + 122) |

### `examples/probe_objects.rs`

**Create `examples/probe_objects.rs`**

```rust
//! EXACT live-heap bytes per OBJECT ROW of the viewer's own bookkeeping, via the counting
//! allocator of probe_mem.rs; the loading lives in `selftest::object_bytes`. The audit's ~1,034 B.
//!
//! cargo run --release --target x86_64-unknown-linux-gnu --example probe_objects -- assets/scenes/<scene>.toml
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static LIVE: AtomicUsize = AtomicUsize::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        LIVE.fetch_add(l.size(), Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new: usize) -> *mut u8 {
        LIVE.fetch_add(new, Relaxed);
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.realloc(p, l, new) }
    }
}
#[global_allocator]
static A: Counting = Counting;

/// Live heap, MB.
fn live() -> f64 { LIVE.load(Relaxed) as f64 / 1.048576e6 }

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let files = session_viewer::selftest::SceneFile::from_args(&a);
    print!("{}", session_viewer::selftest::object_bytes(&files, live));
}
```

### `src/app/scene.rs`

**Find** in `src/app/scene.rs`:

```rust
use std::collections::{HashMap, HashSet};
```

**Add below it:**

```rust
use std::rc::Rc;
```

**Find** in `src/app/scene.rs`:

```rust
/// guid -> row, hidden) lives here, never in the kernel type three languages share.
```

**Replace with:**

```rust
/// guid -> row, hidden) lives here, never in the kernel type three languages share. Both
/// directions of the guid map share ONE `Rc<str>` per object - picking and selection will
/// need both, and two Strings per guid were 24% of the per-object cost.
```

**Find** in `src/app/scene.rs`:

```rust
    vert_base: u32,             // arena rows already uploaded - walk_mesh bases its indices on this
    cloud_base: u32,            // cloud points already uploaded - a draw record's `first` counts from here
    pub tables: Upload,
    order: Vec<String>, // renderable guids, global row order across docs
    pub guid_to_row: HashMap<String, u32>,
    pub hidden: HashSet<String>,
```

**Replace with:**

```rust
    pub tables: Upload,
    order: Vec<Rc<str>>, // renderable guids, global row order across docs
    pub guid_to_row: HashMap<Rc<str>, u32>,
    pub hidden: HashSet<String>,
    bases: Bases,
}

/// Rows already uploaded, per table that keeps global numbering: the next walk counts from here.
#[derive(Default)]
struct Bases {
    vert: u32,  // arena rows - walk_mesh bases its indices on this
    cloud: u32, // cloud points - a draw record's `first` counts from here
    obj: u32,   // object rows - every `instance_id` counts from here
```

**Find** in `src/app/scene.rs`:

```rust
            clouds: Vec::new(),
            vert_base: 0,
            cloud_base: 0,
```

**Replace with:**

```rust
            clouds: Vec::new(),
```

**Find** in `src/app/scene.rs`:

```rust
            hidden: HashSet::new(),
```

**Add below it:**

```rust
            bases: Bases::default(),
```

**Find** in `src/app/scene.rs`:

```rust
        self.hidden.clear();
        self.vert_base = 0;
        self.cloud_base = 0;
```

**Replace with:**

```rust
        self.hidden.clear();
        self.bases = Bases::default();
```

**Find** in `src/app/scene.rs`:

```rust
        let row = self.tables.obj.rows.len() as u32;
```

**Replace with:**

```rust
        let row = self.bases.obj + self.tables.obj.rows.len() as u32;
```

**Find** in `src/app/scene.rs`:

```rust
        let guid = format!("cloud:{name}");
        self.guid_to_row.insert(guid.clone(), row);
```

**Replace with:**

```rust
        let guid: Rc<str> = Rc::from(format!("cloud:{name}"));
        self.guid_to_row.insert(Rc::clone(&guid), row);
```

**Find** in `src/app/scene.rs`:

```rust
        self.vert_base = 0;
        self.cloud_base = 0;
```

**Replace with:**

```rust
        self.bases = Bases::default();
```

**Find** in `src/app/scene.rs`:

```rust
        self.vert_base += self.tables.arena.verts.len() as u32;
        self.cloud_base += (self.tables.cloud.pos.len() / 3) as u32;
```

**Replace with:**

```rust
        self.bases.vert += self.tables.arena.verts.len() as u32;
        self.bases.cloud += (self.tables.cloud.pos.len() / 3) as u32;
        self.bases.obj += self.tables.obj.rows.len() as u32;
```

**Find** in `src/app/scene.rs`:

```rust
        let from = Baselines::capture(&self.tables, self.cloud_base);
        let (vb, cb) = (self.vert_base, self.cloud_base); // read before `t` borrows self.tables
```

**Replace with:**

```rust
        let from = Baselines::capture(&self.tables, self.bases.cloud, self.bases.obj);
        let (vb, cb, ob) = (self.bases.vert, self.bases.cloud, self.bases.obj); // read before `t` borrows self.tables
```

**Find** in `src/app/scene.rs`:

```rust
            let ri = t.obj.rows.len() as u32;
```

**Replace with:**

```rust
            let ri = ob + t.obj.rows.len() as u32;
```

**Find** in `src/app/scene.rs`:

```rust
            self.guid_to_row.insert(guid.clone(), ri);
```

**Replace with:**

```rust
            let guid: Rc<str> = Rc::from(guid);
            self.guid_to_row.insert(Rc::clone(&guid), ri);
```

### `src/app/walk/bounds.rs`

**Find** in `src/app/walk/bounds.rs`:

```rust
use crate::math::{grow_bounds, xform_point, Aabb};
```

**Replace with:**

```rust
use crate::math::{grow_bounds, xform_point, Aabb, Mat4};
```

**Find** in `src/app/walk/bounds.rs`:

```rust
/// them real. `cloud_base` is what a draw record's absolute `first` counts from.
```

**Replace with:**

```rust
/// them real. `cloud_base` is what a draw record's absolute `first` counts from; `obj_base`
/// is what a row's global `instance_id` counts from - the object columns are this upload's only.
```

**Find** in `src/app/walk/bounds.rs`:

```rust
    pub cloud_base: u32,
```

**Add below it:**

```rust
    pub obj_base: u32,
```

**Find** in `src/app/walk/bounds.rs`:

```rust
    /// Every table's length now.
    pub fn capture(t: &Upload, cloud_base: u32) -> Self {
```

**Replace with:**

```rust
    /// Every table's length now, and the two bases the global ids count from.
    pub fn capture(t: &Upload, cloud_base: u32, obj_base: u32) -> Self {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
            cloud_base,
        }
```

**Replace with:**

```rust
            cloud_base,
            obj_base,
        }
    }

    /// This upload's object row for a global instance id.
    fn placement<'a>(&self, t: &'a Upload, id: u32) -> Option<&'a Mat4> {
        t.obj.rows.get(id.wrapping_sub(self.obj_base) as usize).map(|(xf, _, _)| xf)
```

**Find** in `src/app/walk/bounds.rs`:

```rust
        if let Some(&ri) = t.arena.vids.get(i) {
            if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
```

**Replace with:**

```rust
        if let Some(&ri) = t.arena.vids.get(i) {
            if let Some(xf) = from.placement(t, ri) {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
    }

    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Replace with:**

```rust
    }

    for s in t.seg.pipes.iter().skip(from.pipe).chain(t.seg.ribbons.iter().skip(from.seg)){
        if let Some(xf) = from.placement(t, s.instance_id) {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
    for s in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Replace with:**

```rust
    for s in t.glyph.spheres.iter().skip(from.sphere).chain(t.glyph.dots.iter().skip(from.glyph)){
        if let Some(xf) = from.placement(t, s.instance_id) {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
        let Some((xf, _, _)) = t.obj.rows.get(inst as usize) else { continue };
```

**Replace with:**

```rust
        let Some(xf) = from.placement(t, inst) else { continue };
```

**Find** in `src/app/walk/bounds.rs`:

```rust
            if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
```

**Replace with:**

```rust
            if let Some(xf) = from.placement(t, ri) {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
        if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Replace with:**

```rust
        if let Some(xf) = from.placement(t, s.instance_id) {
```

**Find** in `src/app/walk/bounds.rs`:

```rust
        if let Some((xf, _, _)) = t.obj.rows.get(g.instance_id as usize) {
```

**Replace with:**

```rust
        if let Some(xf) = from.placement(t, g.instance_id) {
```

### `src/engine/gpu/cloud.rs`

**Find** in `src/engine/gpu/cloud.rs`:

```rust
        // `Scene::cloud_base` already does for the point rows.
```

**Replace with:**

```rust
        // `Scene.bases.cloud` already does for the point rows.
```

### `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// The object rows: instances, their f64 mirrors, the re-anchor and the inside test.
```

**Replace with:**

```rust
    /// The object rows: instances, their f64 translations, the bounded rows, the re-anchor, the inside test.
```

### `src/engine/gpu/objects.rs`

**Find** in `src/engine/gpu/objects.rs`:

```rust
//! `ObjectRows` - the per-object columns a walk fills (true placement, tint, flags, local
//! AABB, vertex spacing) - and `InstanceTable`, the one owner of the instance rows the GPU
//! reads: their f64 mirrors, the re-anchor, the inside test, the buffer and its bind group.
```

**Replace with:**

```rust
//! `ObjectRows` - the per-object columns ONE upload carries (true placement, tint, flags, local
//! AABB, vertex spacing; a delta, dropped after upload) - and `InstanceTable`, the ONE owner of
//! the object rows the GPU reads: the rows themselves, their f64 translations, the sparse list
//! of bounded rows, the re-anchor, the inside test, the buffer and its bind group.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
/// The object columns of one upload, aligned by row. `rows` is the ONE table the walk keeps
/// cumulative (the bounds sweep and the sheet pass index it by global row); `InstanceTable::append`
/// takes only the rows past what it already holds.
```

**Replace with:**

```rust
/// The object columns of one upload, aligned by row - THIS upload's rows only. The walk numbers
/// them from `Scene.bases.obj`, so a row index is global while the columns are a delta.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
/// The instance rows and everything that rewrites them: the true f64 transforms they are
/// rebased from, the world AABBs the inside test walks, and the GPU table itself.
pub struct InstanceTable {
    instances: Vec<Instance>,
    last_origin: Option<Point>, // rebuild skips when the camera target did not move
    objects_base: Vec<(Mat4, [f32; 4], u32)>, // TRUE world model+color; instances[] is rebased from this
    base_f32: Vec<[f32; 16]>, // model.to_f32() cached once - rebase only re-patches 3 slots
    bounded_rows: Vec<u32>, // rows with Some(world AABB) - the only ones the inside test walks
    /// Per-object WORLD AABB (`ObjectRows::bounds` through the true transform), aligned with
    /// `instances`. Drives FLAG_INSIDE - see `update_inside`.
    object_bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection
```

**Replace with:**

```rust
/// A row that carries a world AABB - a mesh that drew ink. The inside test walks these and
/// never the whole table: 3 of 744,040 rows on the ten-sheet scene.
pub struct BoundedRow {
    pub row: u32,
    pub lo: [f64; 3],
    pub hi: [f64; 3],
}

/// The object rows as the GPU sees them, and the two things the CPU keeps to rewrite them: the
/// TRUE f64 translation per row (the rotation/scale is cast once into the row) and the sparse
/// bounded rows. 96 + 24 B per object, plus 32 B per bounded row.
pub struct InstanceTable {
    rows: Vec<Instance>,
    translation: Vec<[f64; 3]>,
    bounded: Vec<BoundedRow>,
    last_origin: Option<Point>, // rebuild skips when the camera target did not move
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let instances: Vec<Instance> = vec![Instance::placeholder()];
```

**Replace with:**

```rust
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            instances,
            last_origin: None,
            objects_base: Vec::new(),
            base_f32: Vec::new(),
            bounded_rows: Vec::new(),
            object_bounds_world: Vec::new(),
            inside: Vec::new(),
```

**Replace with:**

```rust
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            bounded: Vec::new(),
            last_origin: None,
```

**Remove** `src/engine/gpu/objects.rs` `    /// Take the rows past what the table already holds, mirror them, and send only those.` **through** `        let fresh = &self.instances[self.buffer.len() as usize..];`

**Find** in `src/engine/gpu/objects.rs`:

```rust
            group,
        }
    }

```

**Add below it:**

```rust
    /// Append one upload's rows: cast once, keep the f64 translation, note the bounded ones,
    /// send only the new rows. The next frame rebases the whole table (`last_origin` cleared).
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        debug_assert_eq!(up.rows.len(), up.bounds.len());
        if self.translation.is_empty() {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.rows.clear();
            self.buffer.reset();
        }
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        for (i, (m, color, flags)) in up.rows.iter().enumerate() {
            // The diagonal, not an axis, is the extent: a flat sheet has a zero-thickness axis
            // and would clamp its ink lift to nothing.
            let world = up.bounds[i].map(|(lo, hi)| world_aabb(m, lo, hi));
            let extent = world.map_or(0.0, |(lo, hi)| diagonal(lo, hi));
            if let Some((lo, hi)) = world {
                self.bounded.push(BoundedRow { row: base + i as u32, lo, hi });
            }
            self.translation.push([m[12], m[13], m[14]]);
            self.rows.push(Instance {
                model: mat_to_f32(m),
                color: *color,
                flags: *flags,
                extent,
                spacing: up.spacing.get(i).copied().unwrap_or(0.0),
                _pad: 0,
            });
        }

        if self.rows.is_empty() {
            self.rows.push(Instance::placeholder());
        }
        let fresh = &self.rows[self.buffer.len() as usize..];
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// Rebase every instance's translation around `origin`: an f64 subtract against the TRUE
    /// world transform in `objects_base`, then the cast to f32. What the GPU sees never holds a
    /// coordinate bigger than the camera's distance from `origin`, however far the scene sits from (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        for (i, (model, _, _)) in self.objects_base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation / scale cast once at set_scene
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
            self.instances[i].model = m;
        }
        ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.instances));
```

**Replace with:**

```rust
    /// Rebase every row's translation around `origin`: an f64 subtract against the TRUE world
    /// translation, then the cast to f32. What the GPU sees never holds a coordinate bigger than
    /// the camera's distance from `origin`, however far the scene sits from (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        for (row, t) in self.rows.iter_mut().zip(&self.translation) {
            row.model[12] = (t[0] - origin[0]) as f32;
            row.model[13] = (t[1] - origin[1]) as f32;
            row.model[14] = (t[2] - origin[2]) as f32;
        }
        ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.rows));
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// difference is global - so the CPU answers it per object from the world AABBs, and the answer
    /// rides the instance row. One containment test per object per frame; the instance buffer is
    /// rewritten only when some answer flips, which orbit/zoom almost never does.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded_rows.is_empty() {
```

**Replace with:**

```rust
    /// difference is global - so the CPU answers it per BOUNDED row from the world AABBs, and the
    /// answer rides the instance row. The buffer is rewritten only when some answer flips, which
    /// orbit/zoom almost never does; the row's own flag bit is the change detector.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded.is_empty() {
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        for &row in &self.bounded_rows {
            let i = row as usize;
            let b = &self.object_bounds_world[i];
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
            if self.inside.get(i).copied().unwrap_or(false) == inside {
                continue;
            }
            if let Some(row) = self.instances.get_mut(i) {
                row.flags = if inside { row.flags | Instance::FLAG_INSIDE } else { row.flags & !Instance::FLAG_INSIDE };
            }
            if i < self.inside.len() { self.inside[i] = inside; }
```

**Replace with:**

```rust
        for b in &self.bounded {
            let inside = in_scene && (0..3).all(|k| ew[k] >= b.lo[k] && ew[k] <= b.hi[k]);
            let Some(row) = self.rows.get_mut(b.row as usize) else { continue };
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }
            row.flags ^= Instance::FLAG_INSIDE;
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.instances));
```

**Replace with:**

```rust
            ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.rows));
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.objects_base.shrink_to_fit();
        self.base_f32.shrink_to_fit();
        self.object_bounds_world.shrink_to_fit();
        self.inside.shrink_to_fit();
        self.bounded_rows.shrink_to_fit();
        self.instances.shrink_to_fit();
        self.instances.push(Instance::placeholder());
```

**Replace with:**

```rust
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.bounded.shrink_to_fit();
        self.rows.push(Instance::placeholder());
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// Forget every row; the buffer keeps its capacity.
    pub fn reset(&mut self) {
        self.objects_base.clear();
        self.base_f32.clear();
        self.object_bounds_world.clear();
        self.inside.clear();
        self.instances.clear();
        self.buffer.reset();
        // DERIVED from object_bounds_world (rebuilt in append), so leaving it behind holds row
        // indices into a vector that is now empty: a scene cleared and then DRAWN before the
        // next upload would panic in update_inside on the stale rows.
        self.bounded_rows.clear();
```

**Replace with:**

```rust
    /// Forget every row; the buffer keeps its capacity. `bounded` goes with the rows it indexes:
    /// a scene cleared and then DRAWN before the next upload would test stale rows otherwise.
    pub fn reset(&mut self) {
        self.rows.clear();
        self.translation.clear();
        self.bounded.clear();
        self.buffer.reset();
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.instances.get(i as usize)
```

**Replace with:**

```rust
        self.rows.get(i as usize)
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.instances.len() as u32
```

**Replace with:**

```rust
        self.rows.len() as u32
```

### `src/engine/gpu/upload.rs`

**Find** in `src/engine/gpu/upload.rs`:

```rust
//! `Upload` - the walked rows on their way to the GPU: every family's table for one file (a
//! DELTA) plus the cumulative object columns. Built by `app::scene::Scene`, borrowed by
```

**Replace with:**

```rust
//! `Upload` - the walked rows on their way to the GPU: every family's table for one file, the
//! object columns included - ALL deltas. Built by `app::scene::Scene`, borrowed by
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
/// `obj` holds the TRUE per-object transform + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
```

**Replace with:**

```rust
/// `obj` holds the TRUE per-object transform + tint + flags of this upload's rows; `Gpu`
/// builds instance rows from it, keeps the f64 translation, and rebases as the camera moves.
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    /// Forget the uploaded rows: the GPU is their only holder now. Every drawn table goes -
    /// nothing reads them back (picking goes through the kernel Meshes in `Doc.session`), and a
    /// kept copy is what let lanes rebuild whole buffers per file. `obj` STAYS: the instance
    /// table is rebased from it on every re-anchor, and the walk indexes it by global row.
    pub fn drop_uploaded(&mut self) {
```

**Replace with:**

```rust
    /// Forget the uploaded rows: the GPU is their only holder now. Every table goes - nothing
    /// reads them back (picking goes through the kernel Meshes in `Doc.session`), and a kept
    /// copy is what let lanes rebuild whole buffers per file. The object columns go too: the
    /// instance table keeps the one f64 translation per row the re-anchor needs.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        drop_rows(&mut self.obj.bounds);
        drop_rows(&mut self.obj.spacing);
```

### `src/selftest.rs`

**Find** in `src/selftest.rs`:

```rust
    gpu_report(&gpu);
    out
}

```

**Add below it:**

```rust
/// Bytes of viewer bookkeeping per OBJECT ROW: the scene is loaded display_only (the walk
/// releases every kernel document), uploaded, and what `live` still counts afterwards -
/// `Scene`'s columns plus `InstanceTable`'s mirrors - is divided by the object count. `live`
/// is the caller's counting allocator (examples/probe_objects.rs), read in MB.
pub fn object_bytes(files: &[SceneFile], live: fn() -> f64) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(900, 700)).expect("headless gpu");
    let mut scene = Scene::new();
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let base = live();
    for f in files {
        let doc = load(f);
        scene.add_file(FileDoc { display_only: true, ..doc });
        scene.upload_to(&mut gpu);
    }
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let objects = gpu.objects.len() as f64;
    let held = live() - base;
    let docs = scene.docs.len();
    drop(scene);
    let gpu_side = live() - base;
    format!(
        "{} objects | live after upload {held:.1} MB = {:.0} B/object | Scene dropped ({docs} docs): {gpu_side:.1} MB = {:.0} B/object in InstanceTable\n",
        objects as u64, held * 1.048576e6 / objects, gpu_side * 1.048576e6 / objects)
}

```

## Item 9 — the MSAA policy (declared change)

4x samples only when solid geometry exists (faces, pipes, spheres) AND the canvas is at most
4.2 million pixels; no MSAA texture at 1x; `VIEWER_MSAA` / `?msaa=` overrides. Pure sheets and
high-DPI canvases change pixels; every gate scene has solids at 0.63 Mpx, so its rows hold.

| | before | after |
|---|---|---|
| pure sheet, 900x700 | 6.05 ms, 21 MiB of targets | 3.45 ms, 2.9 MiB |
| bunny, 3840x2160 | 13.1 ms, 266 MiB | 5.7 ms, 36 MiB |

### `src/engine/gpu/arena.rs`

**Find** in `src/engine/gpu/arena.rs`:

```rust
    /// Vertices on the GPU - the MSAA test and the scene log read it.
```

**Replace with:**

```rust
    /// Vertices on the GPU - the scene log reads it.
```

**Find** in `src/engine/gpu/arena.rs`:

```rust
        self.verts.len()
    }
```

**Add below it:**

```rust

    /// Indices in the SOLID faces run - the MSAA policy reads it; sheet fills are not solid.
    pub fn face_count(&self) -> u32 {
        self.faces.len()
    }
```

### `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // first solid geometry arrives.
```

**Replace with:**

```rust
        // first solid geometry arrives (`Targets::samples_for`).
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let samples = self.msaa_now();
        if samples != self.targets.samples {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
```

**Replace with:**

```rust
        self.retarget(false);
    }

    /// Bring the targets to the sample count the scene and canvas call for now: on a change
    /// every pipeline follows (the count belongs to the PASS); `resized` remakes the targets
    /// even at the same count, since they are sized to the surface.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;
        if flip || resized {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
        }
        if flip {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.targets = Targets::new(&self.ctx, &self.config, self.targets.samples);
```

**Replace with:**

```rust
            self.retarget(true);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
    /// while ribbons and dots antialias themselves in the shader. A 2D sheet therefore pays
    /// nothing, and a model with meshes gets clean silhouettes.
    /// MSAA follows what is ON THE GPU, not what arrived in the latest upload.
    ///
    /// This used to read `up.verts`/`up.pipes`/`up.spheres`, which was correct while every lane
    /// was cumulative. Now that the arena arrives as a DELTA, an upload carrying only cloud rows
    /// has an empty `up.verts` - so it reported "no solids", flipped 4x back to 1x, and rebuilt
    /// every pipeline and both render targets. In the mixed scene that thrashed 4x -> 1x -> 4x
    /// on every single append.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.vert_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
        if solid { 4 } else { 1 }
```

**Replace with:**

```rust
    /// MSAA sample count for the scene NOW. It cannot be chosen per lane: the count belongs to
    /// the render PASS, so it is picked per scene from what is ON THE GPU (an upload is a delta;
    /// reading it thrashed 4x -> 1x -> 4x on every cloud append). Solid = the faces run, pipes or
    /// spheres - the vertex count would make a pure sheet (fills only) pay for 4x it cannot use.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.face_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
        Targets::samples_for(solid, self.config.width * self.config.height, self.view.msaa_override)
```

### `src/engine/gpu/targets.rs`

**Find** in `src/engine/gpu/targets.rs`:

```rust
//! surface at the sample count the scene chose, and the one render pass that clears them.
//! Nothing here knows what is drawn; it only opens the pass.
```

**Replace with:**

```rust
//! surface at the sample count the scene chose (`samples_for`), and the one render pass that
//! clears them. Nothing here knows what is drawn; it only opens the pass.
```

**Find** in `src/engine/gpu/targets.rs`:

```rust
/// The two attachments of the frame's render pass, and the sample count they were made at.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: wgpu::TextureView,
```

**Replace with:**

```rust
/// Above this many pixels the frame stays at 1x: 4x colour + 4x depth scale with DPR², and at
/// 3840x2160 they were 266 MiB against 36 at 1x. 4.2 M = 2560x1440 DPR 1.1, the common laptop.
const MSAA_MAX_PIXELS: u32 = 4_200_000;

/// The attachments of the frame's render pass, and the sample count they were made at. `msaa`
/// exists only at 4x - the 1x texture used to be allocated and never bound (8-127 MiB).
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: Option<wgpu::TextureView>,
```

**Find** in `src/engine/gpu/targets.rs`:

```rust
        let msaa = msaa_view(&ctx.device, config, samples);

        Self { depth, msaa, samples }
```

**Replace with:**

```rust
        let msaa = if samples > 1 { Some(msaa_view(&ctx.device, config, samples)) } else { None };

        Self { depth, msaa, samples }
    }

    /// The sample count a frame gets: 4x only when SOLID geometry (faces, pipes, spheres) is on
    /// the GPU AND the canvas is at most `MSAA_MAX_PIXELS`, else 1x. Hard edges are the only
    /// thing MSAA smooths; ribbons, dots and splats antialias themselves, so a pure sheet pays
    /// nothing. `override` (`VIEWER_MSAA` / `?msaa=`) wins outright: 4 is 4x, anything else 1x.
    pub fn samples_for(solid: bool, pixels: u32, override_samples: Option<u32>) -> u32 {
        if let Some(s) = override_samples {
            return if s == 4 { 4 } else { 1 };
        }
        if solid && pixels <= MSAA_MAX_PIXELS { 4 } else { 1 }
```

**Find** in `src/engine/gpu/targets.rs`:

```rust
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            // MSAA off (samples == 1): draw straight to the swapchain view - a
            // 1-sample attachment must NOT carry a resolve target.
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if self.samples > 1 { &self.msaa } else { view },
                resolve_target: if self.samples > 1 { Some(view) } else { None },
```

**Replace with:**

```rust
        // MSAA off: draw straight to the swapchain view - a 1-sample attachment must NOT
        // carry a resolve target.
        let (target, resolve) = match &self.msaa {
            Some(msaa) => (msaa, Some(view)),
            None => (view, None),
        };
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: resolve,
```

### `src/engine/gpu/view.rs`

**Find** in `src/engine/gpu/view.rs`:

```rust
    pub thickness_px: f32,
```

**Add below it:**

```rust
    /// Force the sample count (`VIEWER_MSAA` / `?msaa=`): 4 = 4x, anything else 1x; None = the
    /// policy in `Targets::samples_for`.
    pub msaa_override: Option<u32>,
```

**Find** in `src/engine/gpu/view.rs`:

```rust
            thickness_px: thickness_px(),
        }
```

**Replace with:**

```rust
            thickness_px: thickness_px(),
            msaa_override: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
        }
    }
}

/// One knob's raw text: the `?name=` query value on wasm (env vars are unreachable there), the
/// `ENV` variable natively.
fn knob(env: &str, query: &str) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = env;
        let search = web_sys::window()?.location().search().ok()?;
        let prefix = format!("{query}=");
        return search.trim_start_matches('?').split('&').find_map(|pair| pair.strip_prefix(prefix.as_str()).map(str::to_owned));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = query;
        std::env::var(env).ok()
```

**Find** in `src/engine/gpu/view.rs`:

```rust
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window()
            .and_then(|w| w.location().search().ok())
            .and_then(|search| {
                search
                    .trim_start_matches('?')
                    .split('&')
                    .find_map(|pair| pair.strip_prefix("thickness=").map(str::to_owned))
            })
            .and_then(|value| value.parse().ok())
            .filter(|px: &f32| px.is_finite() && *px > 0.0)
            .unwrap_or(2.0);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("VIEWER_THICKNESS").ok().and_then(|v| v.parse().ok()).unwrap_or(2.0)
    }
```

**Replace with:**

```rust
    knob("VIEWER_THICKNESS", "thickness")
        .and_then(|value| value.parse().ok())
        .filter(|px: &f32| px.is_finite() && *px > 0.0)
        .unwrap_or(2.0)
```

## Item 10 — the translation split (declared change, measured identical)

`Instance.model` keeps rotation and scale with a zero translation column; the anchored
translation is a second 16-byte-per-row buffer at group 2 binding 1, added to the seven point
transforms in the five shaders and never to a direction. The splat records fold model and
camera on the CPU, so `anchored_model` rebuilds the full matrix for them; the profile's clock
now starts at the real `now_ms()`; `translations_mirror` is the fifth test.

| forced re-anchor | before | after |
|---|---|---|
| drawings, 744k rows | 21.7 ms CPU + 6.4 ms GPU (68 MiB) | 10.7 + 1.6 ms (11.4 MiB) |
| drawings_rotated, 155k rows | 5.9 + 1.7 ms | 3.2 + 0.5 ms |

### `src/engine/gpu/instance.rs`

**Find** in `src/engine/gpu/instance.rs`:

```rust
//! declare the same row. No buffer and no bind group here: `objects.rs` owns the table.
```

**Replace with:**

```rust
//! declare the same row and the same translation table. No buffer and no bind group here:
//! `objects.rs` owns both tables.
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
/// One object row as the five instance-reading shaders see it: the anchored model matrix,
/// the tint, the flag bits and two scalars the ink lanes read. 96 B, the storage stride.
```

**Replace with:**

```rust
/// One object row as the five instance-reading shaders see it: the model's rotation/scale
/// with a ZERO translation column (the anchored translation is the 16 B row at group 2
/// binding 1 - `InstanceTable::translations`), the tint, the flag bits and two scalars the
/// ink lanes read. 96 B, the storage stride.
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    pub(crate) model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
```

**Replace with:**

```rust
    pub(crate) model: [f32; 16], // 64 B - column-major, from Xform::to_f32(), [12..15] = 0
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
    }

```

**Add below it:**

```rust
    /// Every instance-reading shader binds the 16 B translation table at group 2 binding 1
    /// and adds it to exactly its POINT transforms (`model * vec4(p, 1.0)`), never to a
    /// direction; the Rust row (`[f32; 4]`) is the 16 B stride, and the placeholder row's
    /// model carries no translation of its own.
    #[test]
    fn translations_mirror() {
        let shaders = [
            ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"), 1),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl"), 2),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"), 2),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl"), 1),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"), 1),
        ];
        let binding = "@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;";

        for (name, src, points) in shaders {
            assert!(src.contains(binding), "{name}: translations binding");
            let point_lines: Vec<&str> = src.lines().filter(|l| l.contains("model * vec4<f32>(") && l.contains(", 1.0)")).collect();
            assert_eq!(point_lines.len(), points, "{name}: point transforms");
            for line in &point_lines {
                assert!(line.contains("translations["), "{name}: a point transform without the translation: {line}");
            }
            assert_eq!(src.matches("translations[").count(), points, "{name}: the translation reaches no direction");
        }
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(&Instance::placeholder().model[12..15], &[0.0; 3]);
    }

```

### `src/engine/gpu/objects.rs`

**Find** in `src/engine/gpu/objects.rs`:

```rust
//! of bounded rows, the re-anchor, the inside test, the buffer and its bind group.
```

**Replace with:**

```rust
//! of bounded rows, the re-anchor, the inside test, the two buffers and their bind group.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
use super::buffers::{rows_group, GpuCtx, GrowBuf};
```

**Replace with:**

```rust
use super::buffers::{GpuCtx, GrowBuf};
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
/// The object rows as the GPU sees them, and the two things the CPU keeps to rewrite them: the
/// TRUE f64 translation per row (the rotation/scale is cast once into the row) and the sparse
/// bounded rows. 96 + 24 B per object, plus 32 B per bounded row.
```

**Replace with:**

```rust
/// The object rows as the GPU sees them (rotation/scale, tint, flags - the translation column
/// ZERO), the TRUE f64 translation per row, and the sparse bounded rows. The anchored
/// translations live in their own 16 B/row buffer: a re-anchor rewrites that, never the rows.
/// 96 + 24 B per object on the CPU, plus 32 B per bounded row.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    buffer: GrowBuf, // `rebuild` rewrites it whole on every re-anchor
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    /// Group 2 of every instance-reading pipeline; rebuilt when the buffer grows.
```

**Replace with:**

```rust
    buffer: GrowBuf, // the rows; written at append and per flipped inside flag
    translations: GrowBuf, // `[f32; 4]` per row; `rebuild` rewrites it whole on every re-anchor
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    /// Group 2 of every instance-reading pipeline (rows + translations); rebuilt when either grows.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
impl InstanceTable {
    /// One placeholder row, so the first frame binds a real buffer and draws nothing from it.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buffer = GrowBuf::new(ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, rows);
        let group = rows_group(ctx, &l.instance, "instances.bind_group", &buffer.buf);
```

**Replace with:**

```rust
/// Group 2: the rows at binding 0, the anchored translations at binding 1.
fn instance_group(ctx: &GpuCtx, l: &Layouts, rows: &wgpu::Buffer, translations: &wgpu::Buffer) -> wgpu::BindGroup {
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("instances.bind_group"),
        layout: &l.instance,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: rows.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: translations.as_entire_binding() },
        ],
    })
}

impl InstanceTable {
    /// One placeholder row in both tables, so the first frame binds real buffers and draws
    /// nothing from them.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buffer = GrowBuf::new(ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, usage);
        let translations = GrowBuf::new(ctx, "instance.translations", 16, usage);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            buffer,
```

**Add below it:**

```rust
            translations,
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
```

**Replace with:**

```rust
            // First upload, or a rebuild that rewound everything: start the GPU tables over too,
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            self.buffer.reset();
```

**Add below it:**

```rust
            self.translations.reset();
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            self.rows.push(Instance {
                model: mat_to_f32(m),
```

**Replace with:**

```rust
            let mut model = mat_to_f32(m);
            model[12] = 0.0;
            model[13] = 0.0;
            model[14] = 0.0;
            self.rows.push(Instance {
                model,
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        let fresh = &self.rows[self.buffer.len() as usize..];
        if self.buffer.append(ctx, fresh) {
            self.group = rows_group(ctx, &l.instance, "instances.bind_group", &self.buffer.buf);
```

**Replace with:**

```rust
        // The translations for the new rows are zero until the next frame rebases the whole
        // table (`last_origin` cleared below); the append only makes room and keeps the lengths equal.
        let fresh = &self.rows[self.buffer.len() as usize..];
        let zeros = vec![[0.0f32; 4]; fresh.len()];
        let grew = self.buffer.append(ctx, fresh);
        if self.translations.append(ctx, &zeros) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// translation, then the cast to f32. What the GPU sees never holds a coordinate bigger than
    /// the camera's distance from `origin`, however far the scene sits from (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        for (row, t) in self.rows.iter_mut().zip(&self.translation) {
            row.model[12] = (t[0] - origin[0]) as f32;
            row.model[13] = (t[1] - origin[1]) as f32;
            row.model[14] = (t[2] - origin[2]) as f32;
        }
        ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.rows));
```

**Replace with:**

```rust
    /// translation, then the cast to f32, into the 16 B/row translation table - the 96 B rows
    /// are not touched. What the GPU sees never holds a coordinate bigger than the camera's
    /// distance from `origin`, however far the scene sits from (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        let mut anchored: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());
        for t in &self.translation {
            anchored.push([(t[0] - origin[0]) as f32, (t[1] - origin[1]) as f32, (t[2] - origin[2]) as f32, 0.0]);
        }
        anchored.resize(self.rows.len(), [0.0; 4]); // the placeholder row of an empty scene
        ctx.queue.write_buffer(&self.translations.buf, 0, bytemuck::cast_slice(&anchored));
    }

    /// Row `i`'s model as a shader composes it: rotation/scale plus the anchored translation.
    /// The splat records fold this with the camera on the CPU.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;
        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            model[12] = (t[0] - o[0]) as f32;
            model[13] = (t[1] - o[1]) as f32;
            model[14] = (t[2] - o[2]) as f32;
        }
        Some(model)
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    /// answer rides the instance row. The buffer is rewritten only when some answer flips, which
    /// orbit/zoom almost never does; the row's own flag bit is the change detector.
```

**Replace with:**

```rust
    /// answer rides the instance row. Only a row whose answer FLIPS is written (96 B at its own
    /// offset), which orbit/zoom almost never does; the row's own flag bit is the change detector.
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        let in_scene = (0..3).all(|k| ew[k] >= scene.min[k] as f64 && ew[k] <= scene.max[k] as f64);
        let mut dirty = false;
```

**Replace with:**

```rust
        let in_scene = (0..3).all(|k| ew[k] >= scene.min[k] as f64 && ew[k] <= scene.max[k] as f64);
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
            dirty = true;
        }
        if dirty {
            ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.rows));
```

**Replace with:**

```rust
            ctx.queue.write_buffer(&self.buffer.buf, b.row as u64 * std::mem::size_of::<Instance>() as u64, bytemuck::bytes_of(row));
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.group = rows_group(ctx, &l.instance, "instances.bind_group", &self.buffer.buf);
    }

    /// Forget every row; the buffer keeps its capacity. `bounded` goes with the rows it indexes:
```

**Replace with:**

```rust
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }

    /// Forget every row; the buffers keep their capacity. `bounded` goes with the rows it indexes:
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        self.bounded.clear();
        self.buffer.reset();
```

**Add below it:**

```rust
        self.translations.reset();
```

### `src/engine/gpu/splat.rs`

**Find** in `src/engine/gpu/splat.rs`:

```rust
        let Some(row) = cx.objects.row(d.instance) else { return None };
```

**Add below it:**

```rust
        let Some(model) = cx.objects.anchored_model(d.instance) else { return None };
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        let (a, b) = (cx.mvp, &row.model);
```

**Replace with:**

```rust
        let (a, b) = (cx.mvp, &model);
```

**Find** in `src/engine/gpu/splat.rs`:

```rust
        let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();

        Some(Self { m, model: row.model, tint, mscale, px, first: d.first, spacing: d.spacing })
```

**Replace with:**

```rust
        let mscale = ((model[0] as f64).powi(2) + (model[1] as f64).powi(2) + (model[2] as f64).powi(2)).sqrt();

        Some(Self { m, model, tint, mscale, px, first: d.first, spacing: d.spacing })
```

### `src/engine/pipelines/layouts.rs`

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
        entries: &[buffer_entry(0, wgpu::ShaderStages::VERTEX, wgpu::BufferBindingType::Storage { read_only: true })],
```

**Add below it:**

```rust
    })
}

/// The instance group: the 96 B rows at binding 0 and the 16 B anchored translations at
/// binding 1 - split so a re-anchor rewrites 16 B per object instead of 96.
fn instance_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let read = wgpu::BufferBindingType::Storage { read_only: true };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("instance.layout"),
        entries: &[
            buffer_entry(0, wgpu::ShaderStages::VERTEX, read),
            buffer_entry(1, wgpu::ShaderStages::VERTEX, read),
        ],
```

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
/// 2 = instances, 3 = the family's row table.
```

**Replace with:**

```rust
/// 2 = instances (rows + translations), 3 = the family's row table.
```

**Find** in `src/engine/pipelines/layouts.rs`:

```rust
            instance: storage_layout(device, "instance.layout"),
```

**Replace with:**

```rust
            instance: instance_layout(device),
```

### `src/selftest.rs`

**Find** in `src/selftest.rs`:

```rust
/// the threshold band and the clock past the 200 ms throttle each time, medians reported.
```

**Replace with:**

```rust
/// the threshold band and the clock a second past the 200 ms throttle each time, medians reported.
```

**Find** in `src/selftest.rs`:

```rust
    let (mut cpu, mut gpu_ms) = (Vec::new(), Vec::new());
```

**Add below it:**

```rust
    let clock = now_ms();
```

**Find** in `src/selftest.rs`:

```rust
        let _ = gpu.rebase_anchor(&origin, camera.distance_world(), 1.0e5 * (i + 1) as f64);
```

**Replace with:**

```rust
        let _ = gpu.rebase_anchor(&origin, camera.distance_world(), clock + 1000.0 * (i + 1) as f64);
```

### `src/shaders/cylinder.wgsl`

**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
```

**Add below it:**

```wgsl
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
```

**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
    let w0 = (model * vec4<f32>(vec3<f32>(seg.p0x, seg.p0y, seg.p0z), 1.0)).xyz;
    let w1 = (model * vec4<f32>(vec3<f32>(seg.p1x, seg.p1y, seg.p1z), 1.0)).xyz;
```

**Replace with:**

```wgsl
    let w0 = (model * vec4<f32>(vec3<f32>(seg.p0x, seg.p0y, seg.p0z), 1.0) + vec4<f32>(translations[seg.instance_id].xyz, 0.0)).xyz;
    let w1 = (model * vec4<f32>(vec3<f32>(seg.p1x, seg.p1y, seg.p1z), 1.0) + vec4<f32>(translations[seg.instance_id].xyz, 0.0)).xyz;
```

### `src/shaders/glyph.wgsl`

**Find** in `src/shaders/glyph.wgsl`:

```wgsl
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
```

**Add below it:**

```wgsl
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
```

**Find** in `src/shaders/glyph.wgsl`:

```wgsl
    let world = (model * vec4<f32>(g.center, 1.0)).xyz;
```

**Replace with:**

```wgsl
    let world = (model * vec4<f32>(g.center, 1.0) + vec4<f32>(translations[g.instance_id].xyz, 0.0)).xyz;
```

### `src/shaders/ribbon.wgsl`

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
```

**Add below it:**

```wgsl
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
```

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    let w0 = (model * vec4<f32>(l0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(l1, 1.0)).xyz;
```

**Replace with:**

```wgsl
    let w0 = (model * vec4<f32>(l0, 1.0) + vec4<f32>(translations[seg.instance_id].xyz, 0.0)).xyz;
    let w1 = (model * vec4<f32>(l1, 1.0) + vec4<f32>(translations[seg.instance_id].xyz, 0.0)).xyz;
```

### `src/shaders/sphere.wgsl`

**Find** in `src/shaders/sphere.wgsl`:

```wgsl
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
```

**Add below it:**

```wgsl
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
```

**Find** in `src/shaders/sphere.wgsl`:

```wgsl
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;
```

**Replace with:**

```wgsl
    let centre = (model * vec4<f32>(g.center, 1.0) + vec4<f32>(translations[g.instance_id].xyz, 0.0)).xyz;
```

### `src/shaders/triangle.wgsl`

**Find** in `src/shaders/triangle.wgsl`:

```wgsl
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
```

**Add below it:**

```wgsl
// The anchored translation per row (Instance.model carries none): added to POINTS only.
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
```

**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    let world = inst.model * vec4<f32>(in.position, 1.0);
```

**Replace with:**

```wgsl
    let world = inst.model * vec4<f32>(in.position, 1.0) + vec4<f32>(translations[in.inst_id].xyz, 0.0);
```

## Item 11 — render on demand

`State::render` draws one frame and never asks for the next; `needs_frame` is set by every
message, every input that changed the camera or a knob, a resize, and a deferred re-anchor. `App`
requests a redraw only then. `?perf=1` (or `VIEWER_PERF`) keeps continuous mode for benchmarking,
and `frames drawn: N` logs every 60th frame so you can see it stop.

| a still `drawings` scene | before | after |
|---|---|---|
| frames per second | 9-13 (85-106 ms each, iGPU at 100%) | 0 |

### `src/engine/gpu/mod.rs`

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use objects::InstanceTable;
```

**Replace with:**

```rust
pub use objects::{InstanceTable, Rebase};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Point {
        let (anchor, moved) = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
        if moved {
            self.splat.invalidate();
        }
        anchor
```

**Replace with:**

```rust
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64, now: f64) -> Rebase {
        let rebase = self.objects.rebase_anchor(&self.ctx, origin, view_dist, now);
        if rebase.moved {
            self.splat.invalidate();
        }
        rebase
```

### `src/engine/gpu/objects.rs`

**Find** in `src/engine/gpu/objects.rs`:

```rust
    pub spacing: Vec<f32>,
```

**Add below it:**

```rust
}

/// What one `rebase_anchor` call reports: the anchor in force, whether the table was just
/// rebuilt (the splats are stale), and whether a rebuild is due but throttled - the caller
/// then asks for another frame, or an idle viewer would keep the drifted anchor forever.
pub struct Rebase {
    pub anchor: Point,
    pub moved: bool,
    pub pending: bool,
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64, now: f64) -> (Point, bool) {
```

**Replace with:**

```rust
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64, now: f64) -> Rebase {
```

**Find** in `src/engine/gpu/objects.rs`:

```rust
        (self.last_origin.clone().unwrap(), moved)
```

**Replace with:**

```rust
        Rebase { anchor: self.last_origin.clone().unwrap(), moved, pending: need && !moved }
```

### `src/engine/performance.rs`

**Find** in `src/engine/performance.rs`:

```rust
/// Frame-timing helper: smooths frame time and logs fps / draws / objects once a second.
```

**Replace with:**

```rust
/// Frame-timing helper: smooths frame time and logs fps / draws / objects once a second, and
/// counts frames - every 60th is logged, which is how render-on-demand is seen to idle.
```

**Find** in `src/engine/performance.rs`:

```rust
    frame_ms: f64,  // smoothed frame time
```

**Add below it:**

```rust
    frames: u64, // frames drawn since start
```

**Find** in `src/engine/performance.rs`:

```rust
        Self { prev_frame: t, last_log: t, frame_ms: 0.0 }
```

**Replace with:**

```rust
        Self { prev_frame: t, last_log: t, frame_ms: 0.0, frames: 0 }
```

**Find** in `src/engine/performance.rs`:

```rust
        self.prev_frame = t;
```

**Add below it:**

```rust
        self.frames += 1;
        if self.frames % 60 == 0 {
            log::info!("frames drawn: {}", self.frames);
        }
```

**Find** in `src/engine/performance.rs`:

```rust
/// Whether to print the once-a-second frame line. OFF unless asked for.
```

**Replace with:**

```rust
/// Whether to print the once-a-second frame line AND keep rendering continuously (the
/// benchmark mode - see `State::render`). OFF unless asked for.
```

**Find** in `src/engine/performance.rs`:

```rust
#[cfg(target_arch = "wasm32")]
fn perf_logging() -> bool {
```

**Replace with:**

```rust
#[cfg(target_arch = "wasm32")]
pub fn perf_logging() -> bool {
```

**Find** in `src/engine/performance.rs`:

```rust
/// Native builds have a real environment, so the harness keeps using it.
#[cfg(not(target_arch = "wasm32"))]
fn perf_logging() -> bool {
```

**Replace with:**

```rust
/// Native builds have a real environment, so the harness keeps using it (`VIEWER_PERF`).
#[cfg(not(target_arch = "wasm32"))]
pub fn perf_logging() -> bool {
```

### `src/lib.rs`

**Find** in `src/lib.rs`:

```rust
        self.state = Some(state);
    }
```

**Add below it:**

```rust

    /// The one place a frame is asked for: whenever the handler that just ran left
    /// `needs_frame` set. Render on demand - a still scene asks for nothing.
    fn request_if_needed(&self) {
        if let Some(state) = &self.state {
            if state.needs_frame {
                state.window.request_redraw();
            }
        }
    }
```

**Find** in `src/lib.rs`:

```rust
    /// Every message after `Ready` drives `State` and asks for a frame. The first document
    /// (or a finished scan) fits the camera; later ones only grow its extent.
```

**Replace with:**

```rust
    /// Every message after `Ready` drives `State`; each one changes the scene, so each one
    /// leaves `needs_frame` set. The first document (or a finished scan) fits the camera;
    /// later ones only grow its extent.
```

**Find** in `src/lib.rs`:

```rust
        state.window.request_redraw();
    }

    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether to redraw.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let redraw = match event {
```

**Replace with:**

```rust
        state.needs_frame = true;
        self.request_if_needed();
    }

    /// Redraw and resize here; keys and the mouse go to `Input`, which says whether anything
    /// changed. A frame is requested only when something did (`request_if_needed`).
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let changed = match event {
```

**Find** in `src/lib.rs`:

```rust
                false
            }
```

**Replace with:**

```rust
                false // `render` decides on its own whether the next frame is due
            }
            WindowEvent::Resized(_) => true, // the canvas changed; the redraw above resizes the surface
```

**Find** in `src/lib.rs`:

```rust
        if redraw { state.window.request_redraw(); }
```

**Replace with:**

```rust
        if changed { state.needs_frame = true; }
        self.request_if_needed();
```

### `src/selftest.rs`

**Find** in `src/selftest.rs`:

```rust
    let origin = camera.origin();
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms());
```

**Replace with:**

```rust
    let origin = camera.origin();
    let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms()).anchor;
```

**Find** in `src/selftest.rs`:

```rust
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms());
```

**Replace with:**

```rust
        let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now_ms()).anchor;
```

**Find** in `src/selftest.rs`:

```rust
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now);
```

**Replace with:**

```rust
            let anchor = gpu.rebase_anchor(&origin, camera.distance_world(), now).anchor;
```

### `src/state.rs`

**Find** in `src/state.rs`:

```rust
//! is wired together (ARCHITECTURE.md §1). Today it owns one layer (`gpu`); future chapters add
//! `scene`, `gumball`, `ui`, … as fields, each its own sub-struct — higher layers may drive lower
//! ones, lower layers never reach up.
```

**Replace with:**

```rust
//! is wired together (ARCHITECTURE.md §1). It owns the layers (`gpu`, `scene`, `camera`) and
//! ONE bit of shell state, `needs_frame`: the viewer renders on demand, and this is the demand.
//! Higher layers may drive lower ones, lower layers never reach up.
```

**Find** in `src/state.rs`:

```rust
use crate::engine::performance::{heap_mb, now_ms};
```

**Replace with:**

```rust
use crate::engine::performance::{heap_mb, now_ms, perf_logging};
```

**Find** in `src/state.rs`:

```rust
    pub scene: Scene, // the DOCUMENT set (kernel Sessions + placements + row/hidden bookkeeping)
```

**Add below it:**

```rust
    /// Something changed since the last frame: the shell asks the window for a redraw when it
    /// sees this set. `render` clears it, then sets it again only in `?perf=1` / `VIEWER_PERF`
    /// (continuous mode, for benchmarking) or when a throttled re-anchor is still due.
    pub needs_frame: bool,
```

**Find** in `src/state.rs`:

```rust
        Ok(Self {window, gpu, camera: Camera::new(), scene })
```

**Replace with:**

```rust
        Ok(Self {window, gpu, camera: Camera::new(), scene, needs_frame: true })
```

**Find** in `src/state.rs`:

```rust
            t1 - t0, now_ms() - t1, self.scene.docs.len(), heap_mb());
```

**Add below it:**

```rust
        self.needs_frame = true;
```

**Find** in `src/state.rs`:

```rust
        self.gpu.cloud_begin(count, row);
```

**Add below it:**

```rust
        self.needs_frame = true;
```

**Find** in `src/state.rs`:

```rust
        self.camera.fit(self.gpu.bounds.min, self.gpu.bounds.max, aspect);
```

**Add below it:**

```rust
        self.needs_frame = true;
```

**Find** in `src/state.rs`:

```rust
    }

    /// Continuous redraw: schedule the next frame first, then clear. Any state change is therefore
    /// visible on the following frame without a manual repaint (ARCHITECTURE.md §2).
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();
```

**Replace with:**

```rust
        self.needs_frame = true;
    }

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this
    /// returns. The shell requests the next frame when `needs_frame` is set again - by an
    /// input, a message, a resize, a re-anchor the throttle deferred, or continuous mode.
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.needs_frame = false;
```

**Find** in `src/state.rs`:

```rust
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world(), now_ms);
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

        self.gpu.clear(&FrameInput { view_proj, clear, now_ms })
```

**Replace with:**

```rust
        let rebase = self.gpu.rebase_anchor(&origin, self.camera.distance_world(), now_ms);
        let view_proj = self.camera.view_proj_anchored(aspect, &rebase.anchor);
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

        let drawn = self.gpu.clear(&FrameInput { view_proj, clear, now_ms });
        self.needs_frame = rebase.pending || perf_logging();
        drawn
```

## Item 12 — the map

`ARCHITECTURE.md` describes the tree you now have: the two axes, the symptom-to-file table, the
five rules, the gates, the knobs, what was left on purpose, and the numbers above. It is a
markdown file with code fences of its own, so it is not typed from a listing: copy
`docs/51_refactored/ARCHITECTURE.md` over `session_viewer/ARCHITECTURE.md`. The whole end-of-50
crate sits in `docs/51_refactored/` to diff your tree against.

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings in the crate
cargo xtest                                                  # 5 passed
grep -c request_redraw src/state.rs                          # 0
./docs/_gate.sh                                              # see below
```

Items 1-4 and 6-11 leave every mandatory gate row identical (items 9 and 10 only because every
gate scene has solids at 0.63 Mpx and the split is exact). Item 5 changes what the
`VIEWER_REBUILD=1` rows of `drawings_rotated` count: the sheets are released after the walk, so a
rebuild re-walks only the five solids and reports 5 objects, as `drawings` already did. After
item 5 re-baseline once with `./docs/_gate.sh --record` and commit the new `docs/_GOLDENS.tsv`;
the rows that move are listed in the table below, every other row stays.

| scene, `VIEWER_REBUILD=1` | before (ink / draws / objects) | after |
|---|---|---|
| lion | 77543 / 4 / 1 | 7109 / 3 / 1 |
| bunny_cloud | 7511 / 4 / 1 | 8361 / 3 / 1 |
| drawings_rotated | 25043 / 10 / 155465 | 56405 / 9 / 5 |
| bunny_drawings (advisory) | 41997 / 10 / 8 | 44215 / 9 / 6 |
| cloud_mix (advisory) | 7469 / 11 / 210892 | 42526 / 9 / 6 |
| lidar14 (advisory) | 3549 / 4 / 1 | 737 / 3 / 1 |

A released document has no kernel geometry to re-walk, so after a rebuild only the pickable
objects remain — `rebuild` logs a warning per released document. The `default`, `tubes` and
`INCREMENTAL` rows of every scene are unchanged. The recorded file is
`docs/51_refactored/_GOLDENS.tsv`.

In the browser, open a scene with `?perf=1`, leave the mouse alone, and watch the `frames drawn`
line: without `perf=1` it stops advancing once the scene is loaded.

## Recap

- Measure twice, with the load average, before and after; a change that cannot be measured is
  not a performance change.
- A still scene costs nothing; an event costs one frame.
- One owner per object row, one growth policy per table, one MSAA decision per scene.

## Next

The refactor block is done. `docs/_ROADMAP.md` lists what returns from the archive next,
starting with picking on the row registry this block kept intact.
