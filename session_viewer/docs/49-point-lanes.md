# 49 Point lanes and the frame list — `Gpu` reaches 17 fields

> Fourth refactor lesson. Start from the end of lesson 48. Pixels stay identical.

<svg viewBox="0 0 720 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the walked cloud lane and the streamed lane both feed one Splat with two slots and shared pixel buffers; the compute prelude fills the pixels and the resolve draws them in the render pass" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="sa" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="sb" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <g fill="none" stroke="#f0b35c"><rect x="14" y="40" width="180" height="44"/><rect x="14" y="150" width="180" height="44"/></g>
  <g fill="#d7dae0" font-size="10">
    <text x="22" y="57">app/scene.rs (walk)</text><text x="22" y="167">lib.rs (stream loader)</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="22" y="72">PointCloud → CloudRows + LodNode</text><text x="22" y="182">Msg::CloudBegin · CloudPos · CloudCol</text>
  </g>
  <g stroke="#f0b35c" marker-end="url(#sa)"><line x1="194" y1="62" x2="228" y2="62"/><line x1="194" y1="172" x2="228" y2="172"/></g>
  <g fill="none" stroke="#6fb3ff"><rect x="230" y="40" width="180" height="44"/><rect x="230" y="150" width="180" height="44"/></g>
  <g fill="#d7dae0" font-size="10">
    <text x="238" y="57">CloudLane</text><text x="238" y="167">StreamLane</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="238" y="72">pos, col, nrm: GrowBuf · draws, nodes</text><text x="238" y="182">begin · push_pos · push_col · retarget</text>
  </g>
  <g stroke="#6fb3ff" marker-end="url(#sb)"><line x1="410" y1="62" x2="448" y2="92"/><line x1="410" y1="172" x2="448" y2="142"/></g>
  <rect x="450" y="40" width="256" height="154" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="458" y="58" fill="#d7dae0" font-size="11">Splat</text>
  <g fill="#d7dae0" font-size="10">
    <text x="458" y="82">walked: SplatSlot</text><text x="458" y="126">streamed: SplatSlot</text>
    <text x="458" y="160">pixels: PixelBufs { depth, color }</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="458" y="95">recs, group0, group1, total</text><text x="458" y="139">recs, group0, group1, total</text>
    <text x="458" y="173">shared by both slots · resolve_group</text>
    <text x="458" y="188">is_current(mvp, cloud_size) → skip the prelude</text>
  </g>
  <line x1="14" y1="212" x2="706" y2="212" stroke="#3a3a3a"/>
  <g fill="none">
    <rect x="14" y="226" width="200" height="34" stroke="#6fb3ff"/><rect x="250" y="226" width="180" height="34" stroke="#6fb3ff"/><rect x="466" y="226" width="240" height="34" stroke="#6fb3ff"/>
  </g>
  <g stroke="#6fb3ff" marker-end="url(#sb)"><line x1="214" y1="243" x2="248" y2="243"/><line x1="430" y1="243" x2="464" y2="243"/></g>
  <g fill="#d7dae0" font-size="10">
    <text x="22" y="241">records(RecordCx, draws, nodes)</text><text x="258" y="241">splat_depth · splat_color</text><text x="474" y="241">splat_resolve — render entry 6</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="22" y="254">SplatRecord ×N, 144 B each, asserted</text><text x="258" y="254">one compute pass · 2 pipelines × 2 lanes</text><text x="474" y="254">reads PixelBufs, writes depth + colour</text>
  </g>
  <text x="360" y="284" fill="#888" font-size="10" text-anchor="middle">cloud.rs · stream.rs · splat.rs are created in lesson 49; the prelude reruns when (mvp, cloud_size) changed or a lane invalidated it</text>
</svg>

## Goal

The two point lanes (walked clouds, streamed clouds) and the compute splatter over them get
their own files, the grid and background move to `backdrop.rs`, and `encode_frame` becomes a
list of eleven draws in `render.rs`. `gpu/mod.rs` ends at 259 lines and `Gpu` at 17 fields:
the floor, the four families, the two lanes, the splatter, the perf counter (`performance`)
and the scene box (`bounds`).

## Why

The splat record was thirty-six words packed by hand in four `extend_from_slice` calls, and the
shader read them by literal index: a wrong word put a cloud in the wrong place with no error
anywhere. A `#[repr(C)] SplatRecord` with a size assert is the same bytes with a name per word.
The frame list is the other half: when the draw order is eleven lines that each call the family
owning the rows, a wrong draw order is a one-line diff.

## Files

| file | change | lines after |
|---|---|---|
| `src/engine/gpu/cloud.rs` | created | 103 |
| `src/engine/gpu/stream.rs` | created | 168 |
| `src/engine/gpu/splat.rs` | created | 400 |
| `src/engine/gpu/backdrop.rs` | created | 22 |
| `src/engine/gpu/render.rs` | created | 140 |
| `src/engine/gpu/upload.rs`, `present.rs` | edited | 71 · 136 |
| `src/engine/gpu/mod.rs` | rewritten | 259 (was 799) |
| `src/app/scene.rs`, `examples/check_determinism.rs` | edited | — |

Steps 1-5 create files nothing declares yet, and they name each other's types and `Gpu`'s new
fields before those exist; Step 8 replaces `gpu/mod.rs` whole (`Create` on an existing path, as
in lesson 48: delete every line, then paste) and wires everything. The first `cargo check` is in
Check.

## Step 1 — `src/engine/gpu/cloud.rs`

`CloudDraw` and `LodNode` move here with `CloudRows` (one upload's five cloud columns) and
`CloudLane`: three `GrowBuf`s, the draw list, the node table. `append` returns whether a buffer
was replaced, because the splat groups then need rebinding. `PointBufs` is Step 3's; the import
resolves once `splat.rs` exists.

**Create `src/engine/gpu/cloud.rs`**

```rust
//! The walked cloud lane - points that came through the kernel walk: three flat tables
//! (positions, RGBA8 colours, oct16 normals), one draw record per cloud and the octree nodes
//! the LOD walk reads. `CloudRows` is one upload; `CloudLane` the GPU. Streamed clouds live in
//! `stream.rs`; the splatter that reads both lanes is `splat.rs`.

use super::buffers::{GpuCtx, GrowBuf};
use super::splat::PointBufs;

/// One cloud's contiguous point range, as the record builder sees it. It was a
/// `(first, count, instance, spacing)` tuple until the octree gave every cloud a second
/// range - its slice of the LOD node table - and six positional fields is where a tuple
/// stops being readable.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub first: u32,      // absolute first row in the cloud tables
    pub count: u32,
    pub instance: u32,   // the instance row this cloud draws against
    pub spacing: f32,    // measured point spacing, world units (0 = unknown)
    pub node_first: u32, // first LodNode of this cloud in the nodes table (walked lane)
    pub node_count: u32, // 0 = no octree (streamed clouds) - the record covers everything
}

/// One octree node of a WALKED cloud (kernel `SpatialOctree`): its own spacing-limited
/// subsample as a row range, its cube for the screen-error test, and the accept spacing
/// that drives the attenuated splat radius. `first` is RELATIVE to the cloud's own first
/// point and `children` are indices RELATIVE to the cloud's node slice; -1 = none.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // cube centre, cloud-LOCAL units
    pub size: f32,        // cube edge, cloud-local units
    pub spacing: f32,     // accept spacing, cloud-local units
    pub first: u32,       // row offset from the draw's own `first`
    pub count: u32,
    pub children: [i32; 8],
}

/// One upload's clouds: this file's rows only. A draw's `first` is ABSOLUTE (`Scene` keeps the
/// running base across files); a node's ranges are relative to its own cloud.
#[derive(Default)]
pub struct CloudRows {
    pub pos: Vec<f32>, // 3 floats per point, 12 B
    pub col: Vec<u32>, // RGBA8 per point, 4 B
    pub nrm: Vec<u32>, // oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub draws: Vec<CloudDraw>, // first, count, instance, point spacing world units
    pub nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
}

/// The walked lane on the GPU: three append-only point tables (the splat compute binds them)
/// and the cumulative draw and node lists the record builder walks every frame.
pub struct CloudLane {
    pub pos: GrowBuf, // positions, array<f32> - three rows per point
    pub col: GrowBuf, // colours, array<u32> RGBA8
    pub nrm: GrowBuf, // normals, array<u32> oct16 (u32::MAX = none)
    pub draws: Vec<CloudDraw>,
    pub nodes: Vec<LodNode>,
    pub point_count: u32,
}

impl CloudLane {
    /// Three one-row tables - empty until the first set_scene fills them from an upload.
    pub fn new(ctx: &GpuCtx) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = GrowBuf::new(ctx, "points.buffer", 4, rows);
        let col = GrowBuf::new(ctx, "points.col.buffer", 4, rows);
        let nrm = GrowBuf::new(ctx, "points.nrm.buffer", 4, rows);

        Self { pos, col, nrm, draws: Vec::new(), nodes: Vec::new(), point_count: 0 }
    }

    /// Append one file's rows (a DELTA). Returns true when any buffer was replaced: the splat
    /// groups bind these three buffers and must be rebound. `draws` carries each cloud's
    /// absolute first-point offset, which `Scene` keeps running across files - so the draw
    /// records append too.
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        let mut moved = self.pos.append(ctx, &up.pos);
        moved |= self.col.append(ctx, &up.col);
        moved |= self.nrm.append(ctx, &up.nrm);
        self.point_count = self.pos.len() / 3;

        // The walk numbers a cloud's nodes from the start of ITS upload; the lane's table is
        // cumulative, so every draw's node slice is rebased on the way in - the same thing
        // `Scene::cloud_base` already does for the point rows.
        let node_base = self.nodes.len() as u32;
        self.nodes.extend_from_slice(&up.nodes);
        self.draws.extend(up.draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
        moved
    }

    /// Forget every row and record; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.pos.reset();
        self.col.reset();
        self.nrm.reset();
        self.point_count = 0;
        self.draws.clear();
        self.nodes.clear();
    }

    /// The three point buffers as the splat group binds them.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos.buf, col: &self.col.buf, nrm: &self.nrm.buf }
    }
}
```

## Step 2 — `src/engine/gpu/stream.rs`

`StreamLane`: the exact-fit reserve, `begin`, `push_pos` (with the first-slice spacing estimate
as a named `median_gap`), `push_col`, `retarget`. The three `Gpu::cloud_*` wrappers the loader
calls sit at the bottom; `rebind_splat` and the `stream`/`splat` fields they use come with
Step 8's `Gpu`.

**Create `src/engine/gpu/stream.rs`**

```rust
//! The STREAM cloud lane - clouds whose points never existed on the CPU: three exact-fit
//! buffers, every slice written from the socket at a known offset, one draw record per cloud.
//! The splat groups over these buffers belong to `splat.rs`; the `Gpu` entry points at the
//! bottom keep those groups current when the lane's buffers move.

use super::buffers::{zeroed_buffer, GpuCtx};
use super::cloud::CloudDraw;
use super::splat::PointBufs;
use super::Gpu;

/// The STREAM lane: clouds whose points never existed on the CPU. Their own three buffers
/// and record table - the walked lane above is rebuilt whole by every set_scene, so a
/// streamed cloud cannot live in it. The two lanes meet in the shared per-pixel
/// depth/colour buffers: atomics compose across dispatches.
pub struct StreamLane {
    pos: wgpu::Buffer,
    col: wgpu::Buffer,
    nrm: wgpu::Buffer,
    capacity: u64, // rows
    count: u32,
    pos_at: u32,
    col_at: u32,
    pub draws: Vec<CloudDraw>, // (first, count, instance, spacing)
}

impl StreamLane {
    /// One-row placeholders; `begin` grows them for real, exactly, once per cloud.
    pub fn new(ctx: &GpuCtx) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&ctx.device, "stream.pos", 12, usage);
        let col = zeroed_buffer(&ctx.device, "stream.col", 4, usage);
        let nrm = zeroed_buffer(&ctx.device, "stream.nrm", 4, usage);

        Self { pos, col, nrm, capacity: 1, count: 0, pos_at: 0, col_at: 0, draws: Vec::new() }
    }

    /// Make room for `need` stream rows total, copying the live prefix GPU-side.
    ///
    /// Returns true when the buffers were replaced: the splat group over them must be rebound.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste over a
    /// hundred MB on a multi-scan scene AND worsen the worst transient (old+new live at once).
    /// What doubling avoids is a GPU-side copy - the one thing here that never touches wasm.
    fn reserve(&mut self, ctx: &GpuCtx, need: u64) -> bool {
        if need <= self.capacity {
            return false;
        }
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&ctx.device, "stream.nrm", cap * 4, usage);
        if self.count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.pos, 0, &pos, 0, self.count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.col, 0, &col, 0, self.count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.nrm, 0, &nrm, 0, self.count as u64 * 4);
            ctx.queue.submit([enc.finish()]);
        }
        // The wire has no normals, and a zeroed buffer is NOT "no normal" - oct code 0 decodes
        // to a real direction. Fill the new region with the sentinel, in 1M-row slabs so the
        // staging cost stays bounded.
        let fill = vec![u32::MAX; 1 << 20];
        let mut at = self.count as u64;
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            ctx.queue.submit([]);
            at += n as u64;
        }
        self.pos = pos;
        self.col = col;
        self.nrm = nrm;
        self.capacity = cap;
        true
    }

    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so all three buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn begin(&mut self, ctx: &GpuCtx, count: u32, instance: u32) -> bool {
        let moved = self.reserve(ctx, self.count as u64 + count as u64);
        self.draws.push(CloudDraw { first: self.count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
        self.pos_at = self.count;
        self.col_at = self.count;
        self.count += count;
        moved
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 41's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn push_pos(&mut self, ctx: &GpuCtx, pos: &[f32]) {
        if let Some(d) = self.draws.last_mut() {
            if d.spacing == 0.0 && self.pos_at == d.first && pos.len() >= 6 {
                d.spacing = median_gap(pos);
            }
        }
        ctx.queue.write_buffer(&self.pos, self.pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        ctx.queue.submit([]);
    }

    /// The colour run, packed to RGBA8.
    pub fn push_col(&mut self, ctx: &GpuCtx, col: &[u32]) {
        ctx.queue.write_buffer(&self.col, self.col_at as u64 * 4, bytemuck::cast_slice(col));
        self.col_at += col.len() as u32;
        ctx.queue.submit([]);
    }

    /// Re-issue the instance row draw `i` draws against - a rebuild renumbers the objects while
    /// the streamed points keep their GPU rows.
    pub fn retarget(&mut self, i: usize, row: u32) {
        if let Some(d) = self.draws.get_mut(i) {
            d.instance = row;
        }
    }

    /// The three point buffers as the splat group binds them.
    pub fn buffers(&self) -> PointBufs<'_> {
        PointBufs { pos: &self.pos, col: &self.col, nrm: &self.nrm }
    }
}

/// Median distance between consecutive points over the first 2048 - scan order is surface
/// order, so this is an honest point spacing; 0.0 when no two consecutive points differ.
fn median_gap(pos: &[f32]) -> f32 {
    let n = (pos.len() / 3).min(2048);
    let mut gaps: Vec<f32> = (1..n).map(|i| point_gap(pos, i)).filter(|g| *g > 0.0).collect();
    if gaps.is_empty() {
        return 0.0;
    }
    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
    gaps[gaps.len() / 2]
}

/// Distance from point `i - 1` to point `i` of a flat xyz run.
fn point_gap(pos: &[f32], i: usize) -> f32 {
    let (a, b) = ((i - 1) * 3, i * 3);
    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
}

impl Gpu {
    /// A cloud is about to stream in: reserve its rows; when the lane's buffers moved, re-point
    /// the splat groups at them.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        if self.stream.begin(&self.ctx, count, instance) {
            self.rebind_splat();
            self.splat.invalidate();
        }
    }

    /// One slice of positions; new points, so the splat buffers are stale.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.stream.push_pos(&self.ctx, pos);
        self.splat.invalidate();
    }

    /// One slice of colours; same staleness.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.stream.push_col(&self.ctx, col);
        self.splat.invalidate();
    }
}
```

## Step 3 — `src/engine/gpu/splat.rs`

`Splat` = `PixelBufs` + two `SplatSlot`s (one per lane) + the resolve group + the static-skip
key; `records` turns a lane's draws into `SplatRecord`s (144 bytes, asserted), with `walk_nodes`
doing the octree case and `CloudCx` holding one cloud's constants.

**Create `src/engine/gpu/splat.rs`**

```rust
//! The compute splatter over both cloud lanes: the per-pixel depth/colour pair both lanes
//! contest, one record table + two bind groups per lane (`SplatSlot`), the resolve group, the
//! static-skip key, and the record builder (`records`) that folds a cloud's per-frame state
//! into 144 B records the shader reads by word index. It owns no points - the lanes do.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{zeroed_buffer, GpuCtx};
use super::cloud::{CloudDraw, LodNode};
use super::frame::FrameUniforms;
use super::instance::Instance;
use super::objects::InstanceTable;

/// Words per record: `splat.wgsl` reads the table by literal word index, so this is the contract.
pub const REC_WORDS: u32 = 36;

/// Records a lane's table holds (16 B header + 256 x 144 B); the builder stops at the cap.
pub const MAX_RECORDS: u32 = 256;

/// One record = one contiguous point range at one spacing, as the shader reads it: words 0-15
/// mvp x model (column-major), 16-19 tint (.a = minimum radius px), 20 first, 21 count, 22 cum
/// (the range's first thread), 23 k (attenuation), 24-35 the model's rotation columns padded
/// to vec4 - so a thread does one mat-vec and no instance fetch.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    pub mvp_model: [f32; 16],
    pub tint: [f32; 4],
    pub first: u32,
    pub count: u32,
    pub cum: u32,
    pub k: f32,
    pub rot: [f32; 12],
}

// splat.wgsl walks the table 36 words at a time; a field added here misreads every record after the first.
const _: () = assert!(std::mem::size_of::<SplatRecord>() == REC_WORDS as usize * 4);

/// The two per-pixel u32 buffers both lanes contest: winning reverse-Z bits (0 = empty), winner's RGBA8.
pub struct PixelBufs {
    pub depth: wgpu::Buffer,
    pub color: wgpu::Buffer,
}

impl PixelBufs {
    /// Framebuffer-sized; COPY_DST so `clear_buffer` can zero them before every rebuilt frame.
    pub fn new(ctx: &GpuCtx, size: (u32, u32)) -> Self {
        let pixels = (size.0.max(1) * size.1.max(1)) as u64 * 4;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        let depth = zeroed_buffer(&ctx.device, "splat.depth", pixels, usage);
        let color = zeroed_buffer(&ctx.device, "splat.color", pixels, usage);

        Self { depth, color }
    }
}

/// A lane's three point buffers, borrowed for one bind.
pub struct PointBufs<'a> {
    pub pos: &'a wgpu::Buffer,
    pub col: &'a wgpu::Buffer,
    pub nrm: &'a wgpu::Buffer,
}

/// What every splat bind group is made from besides the lane's own buffers.
pub struct SplatCx<'a> {
    pub ctx: &'a GpuCtx,
    pub layouts: &'a Layouts,
    pub frame: &'a FrameUniforms,
}

/// One frame's records for one lane: the 4-word header {n, total, 0, 0}, the records, the threads.
#[derive(Default)]
pub struct Records {
    pub header: [u32; 4],
    pub recs: Vec<SplatRecord>,
    pub total: u32,
}

/// One lane's slot: its record table, group 0 (frame + records), group 1 (points + pixels), threads.
pub struct SplatSlot {
    recs: wgpu::Buffer,
    group0: wgpu::BindGroup,
    group1: wgpu::BindGroup,
    pub total: u32,
}

impl SplatSlot {
    /// A zeroed record table (`label`) and its two groups over `points` and `pixels`.
    pub fn new(cx: &SplatCx, label: &str, points: PointBufs, pixels: &PixelBufs) -> Self {
        let recs = zeroed_buffer(&cx.ctx.device, label, 16 + (MAX_RECORDS * REC_WORDS * 4) as u64, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let group0 = group0(cx, &recs);
        let group1 = group1(cx, points, pixels);

        Self { recs, group0, group1, total: 0 }
    }

    /// Re-point both groups at the current buffers - the lane grew or the canvas resized.
    pub fn rebind(&mut self, cx: &SplatCx, points: PointBufs, pixels: &PixelBufs) {
        self.group0 = group0(cx, &self.recs);
        self.group1 = group1(cx, points, pixels);
    }

    /// Upload this frame's records: the header at 0, the records at 16.
    pub fn write(&self, ctx: &GpuCtx, r: &Records) {
        ctx.queue.write_buffer(&self.recs, 0, bytemuck::bytes_of(&r.header));
        ctx.queue.write_buffer(&self.recs, 16, bytemuck::cast_slice(&r.recs));
    }

    /// Bind this lane and run the set pipeline over its threads; nothing when the lane is empty.
    pub(super) fn dispatch(&self, cp: &mut wgpu::ComputePass<'_>) {
        if self.total == 0 {
            return;
        }
        let (gx, gy) = dispatch_grid(self.total);
        cp.set_bind_group(0, &self.group0, &[]);
        cp.set_bind_group(1, &self.group1, &[]);
        cp.dispatch_workgroups(gx, gy, 1);
    }
}

/// 2D grid for `n` threads at 64 per group: a 1D dispatch caps at 65535 workgroups (~4.2M threads)
/// and an oversized dispatch silently invalidates the WHOLE command buffer; 4096-wide rows cover any count.
fn dispatch_grid(n: u32) -> (u32, u32) {
    let g = n.div_ceil(64);
    (g.min(4096), g.div_ceil(4096))
}

/// Splat group 0 for one lane: the frame uniforms and that lane's record table.
fn group0(cx: &SplatCx, recs: &wgpu::Buffer) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.group0"),
        layout: &cx.layouts.splat_group0,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: cx.frame.mvp_buffer.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: cx.frame.cloud_buffer.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: recs.as_entire_binding()},
        ],
    })
}

/// Splat group 1 for one lane: its point buffers and the shared per-pixel depth/colour pair.
fn group1(cx: &SplatCx, points: PointBufs, pixels: &PixelBufs) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.group1"),
        layout: &cx.layouts.splat_group1,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: points.pos.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: points.col.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: pixels.depth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: pixels.color.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 4, resource: points.nrm.as_entire_binding()},
        ],
    })
}

/// The resolve pass's view of the per-pixel splat buffers.
fn resolve_group(cx: &SplatCx, pixels: &PixelBufs) -> wgpu::BindGroup {
    cx.ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("splat.resolve.group"),
        layout: &cx.layouts.splat_resolve,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: pixels.depth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: pixels.color.as_entire_binding()},
        ],
    })
}

/// The splatter over both lanes; `state` = the (mvp, cloud_size) the pixel buffers hold, None = stale.
pub struct Splat {
    pub pixels: PixelBufs,
    pub walked: SplatSlot,
    pub streamed: SplatSlot,
    pub resolve_group: wgpu::BindGroup,
    state: Option<([f32; 16], f32)>,
}

impl Splat {
    /// Per-pixel buffers for `size`, one slot per lane, the resolve group; nothing splatted yet.
    pub fn new(cx: &SplatCx, size: (u32, u32), walked: PointBufs, streamed: PointBufs) -> Self {
        let pixels = PixelBufs::new(cx.ctx, size);
        let walked = SplatSlot::new(cx, "splat.rescales", walked, &pixels);
        let streamed = SplatSlot::new(cx, "splat.stream.recs", streamed, &pixels);
        let resolve_group = resolve_group(cx, &pixels);

        Self { pixels, walked, streamed, resolve_group, state: None }
    }

    /// Re-point the five bind groups at the current buffers (set_scene, resize, stream growth).
    pub fn rebind(&mut self, cx: &SplatCx, walked: PointBufs, streamed: PointBufs) {
        self.walked.rebind(cx, walked, &self.pixels);
        self.streamed.rebind(cx, streamed, &self.pixels);
        self.resolve_group = resolve_group(cx, &self.pixels);
    }

    /// True when the buffers already hold the frame for (mvp, cloud_size) - the static skip.
    pub fn is_current(&self, mvp: &[f32; 16], cloud_size: f32) -> bool {
        self.state == Some((*mvp, cloud_size))
    }

    /// The buffers now hold the frame for (mvp, cloud_size).
    pub fn mark_current(&mut self, mvp: &[f32; 16], cloud_size: f32) {
        self.state = Some((*mvp, cloud_size));
    }

    /// Points, instances or pixels changed under the buffers: splat again next frame.
    pub fn invalidate(&mut self) {
        self.state = None;
    }

    /// Threads this frame over both lanes; 0 = no cloud on screen, the resolve is skipped.
    pub fn total(&self) -> u32 {
        self.walked.total + self.streamed.total
    }

    /// The cloud lane's one draw: the compute prelude resolved every cloud into the per-pixel
    /// buffers, so one fullscreen triangle composites them, writing frag_depth against the solids.
    pub fn draw_resolve(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, cloud: &wgpu::BindGroup) -> u32 {
        if self.total() == 0 {
            return 0;
        }

        pass.set_pipeline(&p.splat_resolve);
        pass.set_bind_group(0, cloud, &[]);
        pass.set_bind_group(1, &self.resolve_group, &[]);
        pass.draw(0..3, 0..1);
        1
    }
}

/// The per-frame facts the record builder needs, gathered once.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16],
    pub ortho_h: f32,
    pub eye: [f32; 3],
    pub size: (u32, u32),
    pub cloud_size: f32,
    pub lod_split_px: f32,
    pub objects: &'a InstanceTable,
}

/// One cloud's constants, shared by every record it emits: the folded matrix, the model (the
/// LOD walk places nodes with it), tint, scale, size factor, the draw's first row and spacing.
struct CloudCx {
    m: [f32; 16],
    model: [f32; 16],
    tint: [f32; 4],
    mscale: f64,
    px: f32,
    first: u32,
    spacing: f32,
}

impl CloudCx {
    /// The constants for one draw; `None` when its instance is missing or hidden, or px is zero.
    fn new(cx: &RecordCx, d: &CloudDraw) -> Option<Self> {
        let Some(row) = cx.objects.row(d.instance) else { return None };
        if row.flags & Instance::FLAG_HIDDEN != 0 {
            return None;
        }
        let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;
        if px <= 0.0 {
            return None;
        }

        // column-major 4x4: combined = mvp x model - one per cloud, shared by every
        // record the cloud emits
        let (a, b) = (cx.mvp, &row.model);
        let mut m = [0.0f32; 16];
        for col in 0..4 {
            for r in 0..4 {
                m[col * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[col * 4 + k]).sum();
            }
        }
        // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
        // floor, attenuation turns distant clouds to dust. With octree LOD a far node
        // carries BIGGER spacing (Potree's answer), but the floor still guards leaves.
        let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
        // spacing is in the cloud's LOCAL units; col0's length is the model scale
        let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();

        Some(Self { m, model: row.model, tint, mscale, px, first: d.first, spacing: d.spacing })
    }

    /// One record = one contiguous range at one spacing. world radius = spacing x
    /// (px/6); k folds the projection so the shader only divides by clip.w:
    ///   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
    ///   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
    fn record(&self, cx: &RecordCx, first: u32, count: u32, sp: f32) -> SplatRecord {
        let world_r = (sp as f64).max(1.0e-9) * self.mscale * 0.001 * (self.px as f64) / 6.0; // metres
        let k = if cx.ortho_h > 0.0 { world_r / (2.0 * cx.ortho_h as f64) }
                else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
        // the MODEL rotation columns (translation-free), so a cloud with
        // normals can rotate them into world space for the lambert term
        let b = &self.model;
        let rot = [
            b[0], b[1], b[2], 0.0f32,
            b[4], b[5], b[6], 0.0,
            b[8], b[9], b[10], 0.0,
        ];

        SplatRecord { mvp_model: self.m, tint: self.tint, first, count, cum: 0, k: k as f32, rot }
    }
}

/// Append a record with its thread offset; a full table drops it.
fn push_record(out: &mut Records, mut rec: SplatRecord) {
    if out.header[0] >= MAX_RECORDS {
        return;
    }
    rec.cum = out.total;
    out.header[0] += 1;
    out.total += rec.count;
    out.recs.push(rec);
}

/// Build the record table for one cloud lane. A record folds the cloud's whole per-frame
/// state: mvp x rebased model as ONE matrix, the tint, the attenuation constant and the
/// model rotation - so a thread does one mat-vec, no instance fetch.
/// Attenuated (world-sized) dots, Potree-style: the record carries k such that the
/// shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its own
/// world-space footprint, so near surfaces close up gap-free and far points shrink.
/// The manifest px is a size FACTOR on the measured spacing.
pub fn records(cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode]) -> Records {
    let mut out = Records::default();
    for d in draws {
        let Some(cloud) = CloudCx::new(cx, d) else { continue };
        if cx.lod_split_px > 0.0 && d.node_count > 0 {
            let slice = &nodes[d.node_first as usize..(d.node_first + d.node_count) as usize];
            walk_nodes(cx, &cloud, slice, &mut out);
        } else {
            push_record(&mut out, cloud.record(cx, d.first, d.count, d.spacing));
        }
    }
    out.header[1] = out.total;
    out
}

/// Octree LOD, Potree-style screen-error selection: every VISITED node
/// contributes its own subsample, and the walk descends while the node's
/// projected point spacing is coarser than the cutoff - far nodes stop at
/// the root (a handful of coarse points), near nodes go deep. Coarse nodes
/// carry big spacing, so attenuation grows their dots to close the gaps.
fn walk_nodes(cx: &RecordCx, cloud: &CloudCx, slice: &[LodNode], out: &mut Records) {
    let ortho_h = cx.ortho_h as f64;
    let vp_h = cx.size.1 as f64;
    let aspect = cx.size.0 as f64 / cx.size.1 as f64;
    let eye = cx.eye;
    let (m, mscale) = (&cloud.m, cloud.mscale);
    let mut stack: Vec<usize> = vec![0];
    while let Some(ni) = stack.pop() {
        if out.header[0] >= MAX_RECORDS {
            break;
        }
        let nd = slice[ni];
        let c = nd.center;
        // FRUSTUM CULL on the node's bounding sphere, in clip space through the
        // folded matrix: an off-screen subtree costs nothing - and without this
        // a close zoom would visit every node and starve the 256-record table.
        let r_m = nd.size as f64 * 0.8660254 * mscale * 0.001; // sphere radius, metres
        let cw = (m[3] * c[0] + m[7] * c[1] + m[11] * c[2] + m[15]) as f64;
        if ortho_h <= 0.0 && cw < -r_m { continue; } // fully behind the eye
        let clip_x = (m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12]) as f64;
        let clip_y = (m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13]) as f64;
        let (ndc_x, ndc_y, ry) = if ortho_h > 0.0 {
            (clip_x, clip_y, r_m / ortho_h)
        } else {
            let w = cw.max(1.0e-9);
            (clip_x / w, clip_y / w, r_m * 1.7320508 / w)
        };
        if ndc_x.abs() > 1.0 + ry / aspect.min(1.0) || ndc_y.abs() > 1.0 + ry {
            continue; // the whole subtree is outside the view
        }
        // node centre in anchored world units - the eye's space
        let w = [
            cloud.model[0] * c[0] + cloud.model[4] * c[1] + cloud.model[8] * c[2] + cloud.model[12],
            cloud.model[1] * c[0] + cloud.model[5] * c[1] + cloud.model[9] * c[2] + cloud.model[13],
            cloud.model[2] * c[0] + cloud.model[6] * c[1] + cloud.model[10] * c[2] + cloud.model[14],
        ];
        let dist_m = (((w[0] - eye[0]).powi(2) + (w[1] - eye[1]).powi(2) + (w[2] - eye[2]).powi(2)) as f64).sqrt() * 0.001;
        let sp_m = nd.spacing as f64 * mscale * 0.001;
        let sp_px = if ortho_h > 0.0 { sp_m * vp_h / (2.0 * ortho_h) }
                    else { sp_m * 1.7320508 * 0.5 * vp_h / dist_m.max(1.0e-9) };
        let leaf = nd.children.iter().all(|&ch| ch < 0);
        let refine = !leaf && sp_px > cx.lod_split_px as f64;
        // Dot size: a REFINED node's region also receives all its deeper
        // points, so its own subsample renders at the cloud's measured
        // spacing - otherwise coarse dots blob over the fine layer under
        // them. Only the unrefined FRINGE keeps its coarse node spacing
        // (its points are the only ink there - big dots close the gaps);
        // a node can never be DENSER than the raw cloud, so the measured
        // spacing is also the floor there. Leaves hold raw points.
        let sp = if refine || leaf { cloud.spacing } else { nd.spacing.max(cloud.spacing) };
        // `nd.first` is relative to this cloud's own first point
        push_record(out, cloud.record(cx, cloud.first + nd.first, nd.count, sp));
        if refine {
            for &ch in &nd.children {
                if ch >= 0 { stack.push(ch as usize); }
            }
        }
    }
}
```

## Step 4 — `src/engine/gpu/backdrop.rs`

The two draws that own no rows.

**Create `src/engine/gpu/backdrop.rs`**

```rust
//! The backdrop - the two vertexless draws that open every frame: the background triangle and
//! the 50-vertex grid (`grid.wgsl` builds both from the vertex index). No table, no Gpu field;
//! each returns its draw count like every family draw.

use crate::engine::pipelines::Pipelines;
use super::frame::Binds;

/// The background: one fullscreen triangle, nothing bound. Always 1 draw.
pub fn draw_background(pass: &mut wgpu::RenderPass<'_>, p: &Pipelines) -> u32 {
    pass.set_pipeline(&p.background);
    pass.draw(0..3, 0..1);
    1
}

/// Grid first as the depth writes are off, all objects paints over it; the line block carries the anchor.
pub fn draw_grid(pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
    pass.set_pipeline(&p.grid);
    pass.set_bind_group(0, b.mvp, &[]);
    pass.set_bind_group(1, b.line, &[]);
    pass.draw(0..50, 0..1);
    1
}
```

## Step 5 — `src/engine/gpu/render.rs`

`splat_prelude`, `encode_frame`, and `scene_list`: eleven entries in frame order (one of them
the prepass call); with the two prepass lines and `encode_frame`'s own, `grep` finds fourteen.

**Create `src/engine/gpu/render.rs`**

```rust
//! The frame list - `Gpu::encode_frame`: the compute prelude (splat records for both lanes,
//! the static skip, the 2x2 dispatches), then ONE render pass whose `scene_list` is eleven
//! draws in a fixed order. The order is the contract; every draw is a call into the family
//! that owns the rows, and every family hands back its draw count.

use super::backdrop::{draw_background, draw_grid};
use super::frame::Binds;
use super::splat::{records, RecordCx};
use super::Gpu;

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

impl Gpu {
    /// Splat the clouds by COMPUTE before the render pass. One thread per point, twice (depth
    /// race, then colour claim); the render pass composites the result with one fullscreen
    /// triangle. TWO record sets - the walked lane and the stream lane bind different point
    /// buffers - but one pixel buffer pair: atomics compose across dispatches, so both lanes
    /// contest the same per-pixel depth race.
    fn splat_prelude(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let cx = RecordCx {
            mvp: &self.frame.mvp_f32,
            ortho_h: self.frame.ortho_h,
            eye: self.frame.eye,
            size: (self.config.width, self.config.height),
            cloud_size: self.view.cloud_size,
            lod_split_px: self.view.lod_split_px,
            objects: &self.objects,
        };
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
        encoder.clear_buffer(&self.splat.pixels.depth, 0, None); // 0 bits = reverse-Z far = empty
        encoder.clear_buffer(&self.splat.pixels.color, 0, None);

        // BOTH lanes' depth races must settle before EITHER lane claims colours -
        // dispatches in one pass are ordered, so lane order inside each phase is free.
        let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        cp.set_pipeline(&self.pipelines.splat_depth);
        self.splat.walked.dispatch(&mut cp);
        self.splat.streamed.dispatch(&mut cp);
        cp.set_pipeline(&self.pipelines.splat_color);
        self.splat.walked.dispatch(&mut cp);
        self.splat.streamed.dispatch(&mut cp);
        self.splat.mark_current(&mvp, cloud_size);
    }

    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> (u32, u32) {
        let mut draws = 0u32;
        self.splat_prelude(encoder);

        {
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let mut pass = self.targets.begin_pass(encoder, view, color);
            draws += self.scene_list(&mut pass, &b);
        }

        (draws, self.objects.len())
    }

    /// The scene list - eleven draws, and the ORDER is the contract:
    /// background -> grid -> triangles -> sphere markers -> cylinders -> CLOUD -> ink
    /// prepass -> ribbon -> glyph. Everything that WRITES depth comes first (the cloud
    /// included, since it went opaque); the flat ink lanes read that depth and never
    /// write it. The markers go with the solids so the line ink tests against them -
    /// a vertex marker is the topmost ink at its own joint.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let (p, v) = (&self.pipelines, &self.view);
        let mut draws = 0u32;

        draws += draw_background(pass, p);
        draws += draw_grid(pass, p, b);

        draws += self.arena.draw_faces(pass, p, b);
        draws += self.arena.draw_print(pass, p, b);
        if v.show_mesh_edges {
            draws += self.segments.draw_pipes(pass, p, b, v.line_style);
        }

        draws += self.splat.draw_resolve(pass, p, &self.frame.cloud_group);

        // Markers go LAST of the solid lane - see `GlyphLane::draw_spheres`.
        if v.show_mesh_edges && v.markers {
            draws += self.glyphs.draw_spheres(pass, p, b);
        }

        draws += self.ink_depth_prepass(pass, b);

        if v.show_lines {
            draws += self.segments.draw_ribbons(pass, p, b);
        }

        draws += self.arena.draw_text(pass, p, b);

        if v.show_points {
            draws += self.glyphs.draw_dots(pass, p, b);
        }
        draws
    }

    /// FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
    /// write depth (its AA feather would leave halos), so without this nothing in the flat
    /// lane occludes anything else in it and pure draw order wins - a point dot then sits
    /// on top of a polyline it is behind, at every camera angle.
    /// COST: it draws the whole flat lane a SECOND time. On 2D sheets (600k segments, all
    /// ribbons) that doubles the frame - so it is off by default and only worth enabling
    /// for 3D scenes where ink-vs-ink order is actually visible.
    fn ink_depth_prepass(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = 0u32;

        if INK_DEPTH_PREPASS && self.view.show_lines {
            draws += self.segments.draw_ribbon_depth(pass, &self.pipelines, b);
        }
        if INK_DEPTH_PREPASS && self.view.show_points {
            draws += self.glyphs.draw_dot_depth(pass, &self.pipelines, b);
        }
        draws
    }
}
```

## Step 6 — `src/engine/gpu/upload.rs`

The five cloud columns become `cloud: CloudRows`.

**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::arena::ArenaRows;
```

**Add below it:**

```rust
use super::cloud::CloudRows;
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::segments::SegRows;
use super::{CloudDraw, LodNode};
```

**Replace with:**

```rust
use super::segments::SegRows;
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    pub cloud_pos: Vec<f32>, // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>, // Raw lane: RBGA8 per point, 4 B
    pub cloud_nrm: Vec<u32>, // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
    pub cloud_draws: Vec<CloudDraw>, // first, count, instance, point spacing world units
```

**Replace with:**

```rust
    pub cloud: CloudRows,
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
            cloud_pos: Vec::new(),
            cloud_col: Vec::new(),
            cloud_nrm: Vec::new(),
            cloud_draws: Vec::new(),
            cloud_nodes: Vec::new(),
```

**Replace with:**

```rust
            cloud: CloudRows::default(),
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
        drop_rows(&mut self.cloud_pos);
        drop_rows(&mut self.cloud_col);
        drop_rows(&mut self.cloud_nrm);
        drop_rows(&mut self.cloud_draws);
        drop_rows(&mut self.cloud_nodes);
```

**Replace with:**

```rust
        drop_rows(&mut self.cloud.pos);
        drop_rows(&mut self.cloud.col);
        drop_rows(&mut self.cloud.nrm);
        drop_rows(&mut self.cloud.draws);
        drop_rows(&mut self.cloud.nodes);
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud_pos`
```

**Replace with:**

```rust
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud.pos`
```

## Step 7 — `src/engine/gpu/present.rs`

`write_frame_uniforms` is added here, next to its three callers; the copy in `gpu/mod.rs` goes
when Step 8 rewrites that file.

**Find** in `src/engine/gpu/present.rs`:

```rust
//! (`bench_frames`). Each writes the uniforms, encodes through `encode_frame`, and submits.
```

**Replace with:**

```rust
//! (`bench_frames`). Each writes the uniforms (`write_frame_uniforms`), encodes through
//! `encode_frame` (render.rs), and submits.
```

**Find** in `src/engine/gpu/present.rs`:

```rust
use super::frame::FrameInput;

impl Gpu {
```

**Replace with:**

```rust
use super::frame::{FrameCx, FrameInput};

impl Gpu {
    /// Per-frame uniforms through `FrameUniforms::write`, then the inside-flag refresh, which
    /// reads the eye it solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let anchor = self.objects.anchor_f32();
        let cx = FrameCx { view: &self.view, anchor, size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
        self.objects.update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

```

## Step 8 — `src/engine/gpu/mod.rs`

What is left: the struct, the constructors, `set_scene`, `rebase_anchor`, `rebind_splat`,
`grow_scene`, `resize`, `reset_arena`, `msaa_now`.

**Create `src/engine/gpu/mod.rs`**

```rust
//! `Gpu` - the lowest layer of the viewer (ARCHITECTURE.md §1): the floor (surface, `GpuCtx`,
//! layouts, pipelines, frame uniforms, targets, view), the four row families, the two point
//! lanes and the splatter over them - one file each. This file only builds the struct, appends
//! an upload, and keeps the splat groups current; the frame list is `render.rs`.

pub mod arena;
pub mod backdrop;
pub mod buffers;
pub mod cloud;
pub mod device;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
pub mod stream;
pub mod targets;
pub mod upload;
pub mod view;

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Pipelines, Target};
use crate::math::Aabb;
use session_rust::Point;

use buffers::GpuCtx;
use cloud::CloudLane;
use device::DeviceSetup;
use frame::FrameUniforms;
use glyphs::GlyphLane;
use segments::SegmentLane;
use splat::{PixelBufs, Splat, SplatCx};
use stream::StreamLane;
use targets::Targets;

pub use arena::Arena;
pub use cloud::{CloudDraw, LodNode};
pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
pub use objects::InstanceTable;
pub use segments::{CylinderSegment, LineStyle};
pub use upload::Upload;
pub use view::View;

/// Everything on the GPU side of the viewer, 17 fields: the floor, the families, the lanes.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // Screen to draw pixels on; None when headless.
    pub ctx: GpuCtx,                         // Device (makes resources) + queue (submits work).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    /// Layouts survive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,
    pub pipelines: Pipelines,
    pub frame: FrameUniforms,                // mvp / line / cloud uniforms + this frame's eye and ortho
    pub targets: Targets, // depth + MSAA colour at the sample count this scene chose (see `msaa_now`)
    /// The runtime knobs: what to show, ink style, cloud/EDL/LOD scalars, pen weight.
    pub view: View,
    /// The object rows: instances, their f64 mirrors, the re-anchor and the inside test.
    pub objects: InstanceTable,
    /// The mesh arena: one vertex table, three index runs (faces, sheet fills, lettering).
    pub arena: Arena,
    /// The segment family: pipes (solid lane) and ribbons (flat lane) over one row layout.
    pub segments: SegmentLane,
    /// The glyph family: spheres (solid lane markers) and dots (flat lane) over one row layout.
    pub glyphs: GlyphLane,
    /// The walked cloud lane: three point tables, one draw per cloud, the octree nodes.
    pub cloud: CloudLane,
    /// The stream lane: clouds written slice by slice, never held on the CPU.
    pub stream: StreamLane,
    /// The compute splatter over both lanes: pixel buffers, record slots, the static-skip key.
    pub splat: Splat,
    pub performance: Performance,
    pub bounds: Aabb,
}

impl Gpu {
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// The scene starts empty - every upload, including the first file, goes through `set_scene`
    /// (progressive loading calls it once per appended file), One code path, not two.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), size.width.max(1), size.height.max(1)).await
    }

    /// Same stack with no window and no surface, rendering into an offscreen texture. Exists so
    /// a shader can be checked against a PNG on this machine instead of against the user's eyes.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, width.max(1), height.max(1)).await
    }

    /// The shared constructor: negotiate the device, make every layout, buffer, bind group and
    /// pipeline, and start with an empty scene.
    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, (width, height)).await?;
        let ctx = GpuCtx { device, queue };

        // Depth and MSAA - the empty scene starts flat (1x); set_scene flips to 4x when the
        // first solid geometry arrives.
        let samples = 1;
        let targets = Targets::new(&ctx, &config, samples);

        // Every bind-group layout, once; pipelines and bind groups are made from these.
        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, (config.width, config.height));

        // The four row families start as one zeroed row each: wgpu cannot bind a 0-byte
        // buffer, and every length is 0, so the first frame draws nothing. The loader calls
        // set_scene the moment the first file's tables exist.
        let objects = InstanceTable::new(&ctx, &layouts);
        let arena = Arena::new(&ctx);
        let segments = SegmentLane::new(&ctx, &layouts);
        let glyphs = GlyphLane::new(&ctx, &layouts);

        // The walked cloud lane - empty until set_scene fills it from the upload.
        let cloud = CloudLane::new(&ctx);

        // The stream lane: its own buffers, grown for real by `StreamLane::begin`.
        let stream = StreamLane::new(&ctx);

        // The compute splatter over both lanes: framebuffer-sized per-pixel buffers and one
        // record slot per lane, bound over the lanes' placeholder buffers for now.
        let splat_cx = SplatCx { ctx: &ctx, layouts: &layouts, frame: &frame };
        let splat = Splat::new(&splat_cx, (config.width, config.height), cloud.buffers(), stream.buffers());

        // Pipelines - render and compute, one set per sample count.
        let pipelines = Pipelines::new(&ctx.device, Target { format: config.format, samples }, &layouts);

        // Output
        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            pipelines,
            frame,
            targets,
            view: View::from_env(),
            objects,
            arena,
            segments,
            glyphs,
            cloud,
            stream,
            splat,
            performance: Performance::new(),
            bounds: Aabb { min: [0.0; 3], max: [0.0; 3] },
        })
    }

    /// Append one upload to every family - called once per file while progressive loading
    /// appends. Every table but `obj` is a DELTA: only this file's rows travel, and a bind group
    /// is rebuilt only when the buffer behind it grew. An MSAA flip (first solid file after
    /// flat-only ones) also rebuilds the targets and every pipeline: sample count belongs to the PASS.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);

        if self.cloud.append(&self.ctx, &up.cloud) {
            self.rebind_splat();
        }
        self.splat.invalidate();

        if up.bounds.is_finite() { // an empty upload (the State boots before any file) knows no box
            self.bounds = up.bounds;
        }

        log::info!(
            "scene: {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres) {} cloud points",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count() + self.segments.ribbon_count(), self.segments.pipe_count(),
            self.glyphs.sphere_count() + self.glyphs.dot_count(), self.glyphs.sphere_count(), self.cloud.point_count
        );

        let samples = self.msaa_now();
        if samples != self.targets.samples {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
            self.pipelines = Pipelines::new(&self.ctx.device, Target { format: self.config.format, samples }, &self.layouts);
            log::info!("msaa: {}x", samples);
        }
    }

    /// The anchor the instance table is rebased about - see `InstanceTable::rebase_anchor`.
    /// A rebase moves every instance model, so the splats are stale.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point {
        let (anchor, moved) = self.objects.rebase_anchor(&self.ctx, origin, view_dist);
        if moved {
            self.splat.invalidate();
        }
        anchor
    }

    /// Re-point the splat groups at the current buffers - a lane grew or the canvas resized.
    fn rebind_splat(&mut self) {
        let cx = SplatCx { ctx: &self.ctx, layouts: &self.layouts, frame: &self.frame };
        self.splat.rebind(&cx, self.cloud.buffers(), self.stream.buffers());
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, world: &Aabb) {
        if !world.is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.bounds.min[0] >= self.bounds.max[0] {
            self.bounds = *world;
            return;
        }
        self.bounds.union(world);
    }

    /// Reconfigure the surface and recreate the depth + MSAA targets for a new canvas size.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            if let Some(s) = &self.surface { s.configure(&self.ctx.device, &self.config); }
            self.targets = Targets::new(&self.ctx, &self.config, self.targets.samples);
            self.splat.pixels = PixelBufs::new(&self.ctx, (width, height));
            self.rebind_splat();
            self.splat.invalidate();
        }
    }

    /// Forget every family's rows, so the next upload writes from row 0 again. Every lane
    /// appends, so a rebuild has to rewind every lane - leaving one set would append the
    /// re-walked scene BEHIND the copy already there. Capacity stays: a rebuild costs no allocation.
    pub fn reset_arena(&mut self) {
        self.objects.reset();
        self.arena.reset();
        self.segments.reset();
        self.glyphs.reset();
        self.cloud.reset();
    }

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
    }
}
```

## Step 9 — `src/app/scene.rs`

The walk writes `t.cloud.pos`, `t.cloud.draws` and so on, and `rebuild` retargets the stream
draws through `StreamLane::retarget`.

**Find** in `src/app/scene.rs`:

```rust
            if let Some(d) = gpu.stream_draws.get_mut(i) {
                d.instance = row;
            }
```

**Replace with:**

```rust
            gpu.stream.retarget(i, row);
```

**Find** in `src/app/scene.rs`:

```rust
        self.cloud_base += (self.tables.cloud_pos.len() / 3) as u32;
```

**Replace with:**

```rust
        self.cloud_base += (self.tables.cloud.pos.len() / 3) as u32;
```

**Find** in `src/app/scene.rs`:

```rust
        let draw0 = self.tables.cloud_draws.len();
```

**Replace with:**

```rust
        let draw0 = self.tables.cloud.draws.len();
```

**Find** in `src/app/scene.rs`:

```rust
                    // cumulative while `cloud_pos` is only this upload's delta.
                    let first = cb + (t.cloud_pos.len() / 3) as u32;
                    let node_first = t.cloud_nodes.len() as u32;
                    push_cloud(pc, &mut t.cloud_pos, &mut t.cloud_col, &mut t.cloud_nrm, &mut t.cloud_nodes);
                    let node_count = t.cloud_nodes.len() as u32 - node_first;
                    t.cloud_draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
```

**Replace with:**

```rust
                    // cumulative while `cloud.pos` is only this upload's delta.
                    let first = cb + (t.cloud.pos.len() / 3) as u32;
                    let node_first = t.cloud.nodes.len() as u32;
                    push_cloud(pc, &mut t.cloud.pos, &mut t.cloud.col, &mut t.cloud.nrm, &mut t.cloud.nodes);
                    let node_count = t.cloud.nodes.len() as u32 - node_first;
                    t.cloud.draws.push(CloudDraw { first, count: pc.len() as u32, instance: ri, spacing: cloud_spacing(pc), node_first, node_count });
```

**Find** in `src/app/scene.rs`:

```rust
        for &CloudDraw { first, count, instance: inst, .. } in t.cloud_draws.iter().skip(draw0){
            let Some((xf, _, _)) = t.obj.rows.get(inst as usize) else { continue };
            // `first` is absolute; `cloud_pos` starts at `cb`.
            for i in (first - cb) as usize..(first - cb + count) as usize {
                let p = [t.cloud_pos[i*3], t.cloud_pos[i*3+1], t.cloud_pos[i*3 + 2]];
```

**Replace with:**

```rust
        for &CloudDraw { first, count, instance: inst, .. } in t.cloud.draws.iter().skip(draw0){
            let Some((xf, _, _)) = t.obj.rows.get(inst as usize) else { continue };
            // `first` is absolute; `cloud.pos` starts at `cb`.
            for i in (first - cb) as usize..(first - cb + count) as usize {
                let p = [t.cloud.pos[i*3], t.cloud.pos[i*3+1], t.cloud.pos[i*3 + 2]];
```

## Step 10 — `examples/check_determinism.rs`

Three comparisons follow the rename.

**Find** in `examples/check_determinism.rs`:

```rust
        same!(cloud_pos); same!(cloud_col); same!(cloud_nrm);
```

**Replace with:**

```rust
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings
cargo xtest                                                  # 4 passed
grep -c 'draws +=' src/engine/gpu/render.rs                  # 14: 11 in scene_list, 2 in the prepass, 1 in encode_frame
./docs/_gate.sh                                              # gate OK
```

`Gpu` has 17 fields (was 39); `gpu/mod.rs` is 259 lines (was 2447 at the start of lesson 46).
The goldens do not move.

## Recap

- `SplatRecord`: one name per word, size-asserted.
- The frame is an ordered list; every entry is a family call that returns its draw count.
- `Gpu` is now what it says: a floor, four families, two lanes, one splatter.

## Next

Lesson [50](50-walk-and-shell.md) — the walk and the shell: one file per geometry type under
`app/walk/`, and `lib.rs` becomes a loader, an input handler and a shell.
