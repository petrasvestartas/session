# 50 The frame is a list

> Lesson [58](58-nurbscurve.md) adds a geometry type and never opens this file,
> [94](94-gtao.md) inserts an ambient-occlusion pass and changes one line of it,
> [119](119-hiz-occlusion.md) reorders two entries. All three are possible because after this
> lesson the frame is eleven lines you can read top to bottom, each naming the family that owns
> those rows. Nothing visible changes: same ink, same draw count, same object count, on every
> scene and config.
> Answer key: `git diff end-of-48..end-of-49 -- session_viewer/src` is this lesson as one patch.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file; if you find yourself improving a line while moving it, stop — the
> deferral list at the end says which lesson owns that change.**

## 1. Why this seam

### 1a. The evidence — run it on your own tree

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE 'point_|splat_|stream_|cloud_'
awk '/pub fn encode_frame/,/^    \}$/' src/engine/gpu/mod.rs | wc -l
grep -c 'extend_from_slice' src/engine/gpu/mod.rs
grep -c '256 \* 144' src/engine/gpu/mod.rs
```

```text
43   fields on Gpu
28   of them are the point lanes and the splat machinery
163  lines of encode_frame
 5   extend_from_slice — four of them pack ONE record, by hand
 2   sites that spell the record table's size as a magic `256 * 144`
```

Twenty-eight of forty-three fields, and they are three things wearing one coat.

Nine hold **the walked point lane**: three parallel buffers, three capacities, a count, two CPU
tables. Eight hold **the streamed lane** — the same three columns again, plus a capacity, a count
and two write cursors, separate because the walked lane is rebuilt whole by every `set_scene` and
a streamed cloud has nothing to be rebuilt from. Eleven hold **the rasterizer**: two record
tables, four bind groups, a per-pixel depth and colour pair, a resolve group, a total and a
staleness cache.

Then the record itself, which is why this lesson exists. It is currently written like this —
quoted here for reference, not to type:

```rust
recs.extend_from_slice(bytemuck::cast_slice(&m));
recs.extend_from_slice(bytemuck::cast_slice(&tint));
recs.extend_from_slice(bytemuck::cast_slice(&[f, c, *cum, (k as f32).to_bits()]));
recs.extend_from_slice(bytemuck::cast_slice(&[ b[0], b[1], b[2], 0.0f32, /* … */ ]));
```

Thirty-six words, packed by four calls into a `Vec<u8>` and read back by literal index —
`table[b + 22u]` is the cumulative thread count, `rec_f(base, 19u)` the minimum radius. **There is
no Rust type for it.** The size appears as `256 * 144` in two constructors and as
`REC_WORDS = 36u` in the shader, and nothing checks that those three numbers agree.

### 1b. The law this enforces, stated as what it forbids

**F8 — the frame is an ORDERED LIST, and nothing in it may reach past its own family.** One
function names what is drawn and in what order; every entry is a call into the module that owns
those rows and returns the draws it issued. No entry may bind another family's buffer, set
another family's pipeline, or know why the entry above it came first.

Testable: after this lesson `render.rs` names no `wgpu::Buffer` and no `.wgsl` file at all.

### 1c. The rejected alternative

The obvious cut is a render graph. Do not make it. The order is not derivable — it encodes four
physical arguments (depth writers first; markers with the solids so ink tests against them; the
cloud opaque via `frag_depth`; the lettering last because a page paints text over its own hatching)
and a scheduler would need each one as a constraint to rediscover an order a human can just read.
Lesson **113** adds a hi-Z pass and **88** an occlusion pass; both are one line in a list.

## 2. Where the code lives after this lesson

| symbol | today's home | new home | who may touch it |
|---|---|---|---|
| `CloudDraw`, `LodNode`, the 9 point fields | `gpu/mod.rs` / `Gpu` | `gpu/cloud.rs` — `CloudRows`, `CloudLane` | `cloud.rs`; `splat.rs` READS its draws |
| the 8 `stream_*` fields, `stream_reserve` | `Gpu` | `gpu/stream.rs` — `StreamLane` | `stream.rs` only |
| the 11 splat fields, 3 group builders, `splat_records` | `Gpu` | `gpu/splat.rs` — `SplatRecord`, `PixelBufs`, `SplatSlot`, `Splat` | `splat.rs` only |
| `grid`/`background` descs + their 2 shaders | `pipelines/mod.rs` | `gpu/backdrop.rs` — `Pipes`, `draw` | `backdrop.rs` only |
| `INK_DEPTH_PREPASS`, `encode_frame` | `gpu/mod.rs` | `gpu/render.rs` — + `scene_list` | `render.rs` only |
| `Upload.cloud_{pos,col,nrm,nodes,draws}` | `Upload` | `Upload.cloud: CloudRows` | the walk writes, `CloudLane::append` reads |

```text
        walk ──rows──▶ Upload.cloud ──▶ CloudLane  ─┐   three point buffers
                                                    ├──▶ SplatSlot ──┐
        socket ─────▶ StreamLane ───────────────────┘   (own records) │
                                                                      ▼
                                                    PixelBufs   depth ⊕ colour
                                                       (SHARED — atomics compose)
                                                                      │
        render.rs ──── scene_list ── entry 4 of 11 ───────────────────┘
```

**Exit litmus:** `grep -cE 'wgpu::Buffer|\.wgsl' src/engine/gpu/render.rs` is **0** — the file that
decides the frame touches no buffer and compiles no shader.

The chain table's point row, filled in completely for the first time:

| geometry | walk writes | engine sink | family | shader |
|---|---|---|---|---|
| PointCloud (walked) | `cloud.{pos,col,nrm,nodes,draws}` | `CloudLane` | `cloud.rs` holds → `splat.rs` draws | `splat.wgsl` ×2 entries → `splat_resolve.wgsl` |
| PointCloud (streamed) | nothing — it never reaches the CPU | `StreamLane` | same | same |
| — (no geometry) | — | — | `backdrop.rs` | `grid.wgsl`, `background.wgsl` |

`PointCloud` is the only type that is its own compartment end to end, and `backdrop.rs` the only
family with no rows at all — both legitimate answers to "which row does this family own".

## 3. Files we touch

| file | what | step | why |
|---|---|---|---|
| `src/engine/gpu/cloud.rs` | **NEW**, 141 lines | 4.1 | the point rows and their three buffers |
| `src/engine/gpu/stream.rs` | **NEW**, 143 lines | 4.2 | the lane that is filled once and never re-derived |
| `src/engine/gpu/splat.rs` | **NEW**, 419 lines | 4.3 | `SplatRecord`, and the rasterizer both lanes go through |
| `src/engine/gpu/backdrop.rs` | **NEW**, 66 lines | 4.4 | the family with no rows |
| `src/engine/gpu/render.rs` | **NEW**, 211 lines | 4.5 | `encode_frame` and the eleven-line list |
| `src/engine/gpu/mod.rs` | 1,055 → **524** | 6.1-6.5 | 28 fields become 3 |
| `src/engine/gpu/upload.rs` | 94 → 94 | 6.6 | five flat columns become one group |
| `src/engine/pipelines/mod.rs` | 67 → **52** | 6.7 | the last two row-less descs leave |
| `src/app/scene.rs`, `examples/check_*.rs` | small | 6.8 | the walk and the harnesses follow the columns |

## 4. The five destination files, created first

### 4.1 `src/engine/gpu/cloud.rs`

The header first, then the two row structs by Move — nothing about them changes.

**Create `src/engine/gpu/cloud.rs`**

```rust
//! `cloud.rs` - the point lane's FEED: the rows, the three buffers, and the draw table.
//!
//! This file owns no shader. A point cloud is the only geometry in the viewer that is not drawn
//! by a pipeline at all - `splat.rs` rasterizes it with a compute shader - so the family splits
//! in two: the rows and their buffers here, the rasterizer and its record format there.
//!
//! Three parallel buffers, one row per point, indexed by the same absolute row number:
//!
//! ```text
//!   pos  3 x f32   12 B   the point, in the cloud's LOCAL units
//!   col  1 x u32    4 B   RGBA8
//!   nrm  1 x u32    4 B   oct16, or u32::MAX for "this point has no normal"
//! ```
//!
//! and two CPU-side tables that say which rows belong to which cloud: `draws`, one entry per
//! cloud, and `nodes`, the octree that lets a far cloud send a coarse subsample instead of all
//! of it. `u32::MAX` as the no-normal sentinel is load-bearing: a zeroed normal buffer is NOT
//! "no normal", because oct code 0 decodes to a real direction (+Z).

use super::buffers::{GpuCtx, append_rows, zeroed_buffer};

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

/// The cloud group of `Upload`: one file's points, and the two tables that index them.
pub struct CloudRows {
    pub pos: Vec<f32>,   // 3 per point
    pub col: Vec<u32>,   // RGBA8
    pub nrm: Vec<u32>,   // oct16; u32::MAX = none
    /// Every walked cloud's octree nodes; a draw owns one contiguous slice.
    pub nodes: Vec<LodNode>,
    pub draws: Vec<CloudDraw>,
}

impl Default for CloudRows {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudRows {
    pub fn new() -> Self {
        Self { pos: Vec::new(), col: Vec::new(), nrm: Vec::new(), nodes: Vec::new(), draws: Vec::new() }
    }
}

/// The walked point lane on the GPU.
pub struct CloudLane {
    pub(super) pos: wgpu::Buffer,
    pub(super) col: wgpu::Buffer,
    pub(super) nrm: wgpu::Buffer,
    pos_cap: u64,   // capacity in POINTS; the positions buffer holds three floats each
    col_cap: u64,
    nrm_cap: u64,
    count: u32,
    nodes: Vec<LodNode>,
    draws: Vec<CloudDraw>,
}

impl CloudLane {
    pub fn new(device: &wgpu::Device) -> Self {
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        Self {
            pos: zeroed_buffer(device, "points.buffer", 12, usage),
            col: zeroed_buffer(device, "points.col.buffer", 4, usage),
            nrm: zeroed_buffer(device, "points.nrm.buffer", 4, usage),
            pos_cap: 0,
            col_cap: 0,
            nrm_cap: 0,
            count: 0,
            nodes: Vec::new(),
            draws: Vec::new(),
        }
    }

    /// Points on the GPU.
    pub fn points(&self) -> u32 {
        self.count
    }

    pub fn draws(&self) -> &[CloudDraw] {
        &self.draws
    }

    pub fn nodes(&self) -> &[LodNode] {
        &self.nodes
    }

    /// Append one file's points. Returns `true` if any of the three buffers was replaced, so the
    /// caller knows the splat bind groups pointing at them are stale.
    ///
    /// `draws` carries each cloud's ABSOLUTE first-point offset, which `Scene` keeps running
    /// across files, so the draw records append too. The walk numbers a cloud's nodes from the
    /// start of ITS upload; this table is cumulative, so every draw's node slice is rebased on
    /// the way in - the same thing `Scene::cloud_base` already does for the point rows.
    pub fn append(&mut self, ctx: &GpuCtx, up: &CloudRows) -> bool {
        let mut pos_rows = self.count * 3;
        let mut grew = append_rows(ctx, "points.buffer", &mut self.pos, &mut pos_rows, &mut self.pos_cap, &up.pos);
        let mut col_rows = self.count;
        grew |= append_rows(ctx, "points.col.buffer", &mut self.col, &mut col_rows, &mut self.col_cap, &up.col);
        let mut nrm_rows = self.count;
        grew |= append_rows(ctx, "points.nrm.buffer", &mut self.nrm, &mut nrm_rows, &mut self.nrm_cap, &up.nrm);
        self.count = pos_rows / 3;
        let node_base = self.nodes.len() as u32;
        self.nodes.extend_from_slice(&up.nodes);
        self.draws.extend(up.draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
        grew
    }

    /// Rewind the lane. Buffers and capacity stay; only the counters and the two tables move.
    pub fn reset(&mut self) {
        self.count = 0;
        self.draws.clear();
        self.nodes.clear();
    }
}
```

`CloudDraw` and `LodNode` are byte-identical to the versions in `gpu/mod.rs`: extract them from
your own tree rather than retyping, then delete the originals.

**Remove** `src/engine/gpu/mod.rs` `/// One cloud's contiguous point range, as the record builder sees it. It was a` **through** `}`

**Remove** `src/engine/gpu/mod.rs`

```rust
/// One octree node of a WALKED cloud (kernel `SpatialOctree`): its own spacing-limited
```

```rust
}
```

The second anchor is written as two fenced blocks because its first line contains a backtick and
the region verb reads inline `code spans`. **Any anchor with a backtick in it must be written this
way.**

**Gate.** `cargo check --target wasm32-unknown-unknown --lib` — errors, because `mod.rs` still
declares `CloudDraw`. Expected until 6.1.

### 4.2 `src/engine/gpu/stream.rs`

Same three columns as the walked lane, its own buffers. The reason is a LIFETIME, not a format:
`set_scene` rebuilds the walked lane whole, and a streamed cloud has nothing to be rebuilt from.

**Create `src/engine/gpu/stream.rs`**

```rust
//! `stream.rs` - clouds whose points never existed on the CPU.
//!
//! A streamed cloud arrives over the wire and goes straight from the socket into GPU memory: the
//! protobuf packed-double length prefix gives the point count BEFORE the first point is read, so
//! all three buffers are sized once, exactly, and every slice afterwards lands at a known offset.
//! No growth mid-cloud, and no CPU-side copy of the cloud ever exists.
//!
//! It cannot live in `cloud.rs`'s lane, and the reason is a lifetime rather than a format: the
//! walked lane is rebuilt WHOLE by every `set_scene`, and a streamed cloud has nothing to be
//! rebuilt from. Same three columns, same row format, same record builder - a separate set of
//! buffers, because they are filled once and never re-derived.
//!
//! The two lanes meet in `splat.rs`, at the shared per-pixel depth and colour buffers: atomics
//! compose across dispatches, so both lanes contest one depth race and one composite.

use super::buffers::{GpuCtx, zeroed_buffer};
use super::cloud::CloudDraw;

/// The streamed point lane on the GPU, plus the write cursors one cloud's slices advance.
pub struct StreamLane {
    pub(super) pos: wgpu::Buffer,
    pub(super) col: wgpu::Buffer,
    pub(super) nrm: wgpu::Buffer,
    capacity: u64, // rows
    count: u32,
    pos_at: u32,
    col_at: u32,
    draws: Vec<CloudDraw>,
}

impl StreamLane {
    pub fn new(device: &wgpu::Device) -> Self {
        // Same layouts as the walked lane, its own buffers; grown for real by `reserve`.
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        Self {
            pos: zeroed_buffer(device, "stream.pos", 12, usage),
            col: zeroed_buffer(device, "stream.col", 4, usage),
            nrm: zeroed_buffer(device, "stream.nrm", 4, usage),
            capacity: 0,
            count: 0,
            pos_at: 0,
            col_at: 0,
            draws: Vec::new(),
        }
    }

    pub fn draws(&self) -> &[CloudDraw] {
        &self.draws
    }

    /// Re-aim one streamed cloud at a different object row. A reload re-issues the instance
    /// rows but keeps the GPU points, and order is preserved on both sides, so index `i` here is
    /// index `i` in the caller's list.
    pub fn retarget(&mut self, i: usize, instance: u32) {
        if let Some(d) = self.draws.get_mut(i) {
            d.instance = instance;
        }
    }

    /// Make room for `need` rows total, copying the live prefix GPU-side. Returns `true` when the
    /// buffers were replaced, so the caller knows the bind groups pointing at them are stale.
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

    /// A cloud is about to stream in. Sizes the buffers for it and opens its draw record;
    /// returns `true` if the buffers were replaced.
    pub fn begin(&mut self, ctx: &GpuCtx, count: u32, instance: u32) -> bool {
        let grew = self.reserve(ctx, self.count as u64 + count as u64);
        self.draws.push(CloudDraw { first: self.count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
        self.pos_at = self.count;
        self.col_at = self.count;
        self.count += count;
        grew
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 41's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn push_pos(&mut self, ctx: &GpuCtx, pos: &[f32]) {
        if let Some(d) = self.draws.last_mut() {
            if d.spacing == 0.0 && self.pos_at == d.first && pos.len() >= 6 {
                let n = (pos.len() / 3).min(2048);
                let mut gaps: Vec<f32> = (1..n).map(|i| {
                    let (a, b) = ((i - 1) * 3, i * 3);
                    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
                }).filter(|g| *g > 0.0).collect();
                if !gaps.is_empty() {
                    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    d.spacing = gaps[gaps.len() / 2];
                }
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
}
```

### 4.3 `src/engine/gpu/splat.rs`

The biggest file in the block, and the only one that introduces a type rather than moving one.
Read `SplatRecord` before you paste it: thirty-six words in the order the shader indexes them,
with the word numbers in the doc comments so the two sides can be checked by eye.

**Create `src/engine/gpu/splat.rs`**

```rust
//! `splat.rs` - the point family's rasterizer: `splat.wgsl` + `splat_resolve.wgsl`.
//!
//! A point cloud is not drawn by a render pipeline. It is COMPUTED: one thread per point, twice
//! over, into two per-pixel buffers that the render pass then composites with a single
//! fullscreen triangle.
//!
//! ```text
//!   pass 1  cs_depth   every point of every lane races for its pixel   -> depth  (atomicMax)
//!   pass 2  cs_color   the winner of each pixel writes its colour      -> colour
//!   draw    resolve    one triangle reads both and writes frag_depth   -> the frame
//! ```
//!
//! Two RECORD tables - the walked lane and the streamed lane bind different point buffers - but
//! ONE pixel-buffer pair: atomics compose across dispatches, so both lanes contest the same
//! per-pixel depth race. That sharing is why `SplatSlot` holds the per-lane bind groups while
//! `PixelBufs` is held once, beside them.
//!
//! The record is this file's real subject. Until now it had no Rust type at all: thirty-six
//! words packed by four `extend_from_slice` calls and read back in the shader by literal index
//! (`table[b + 22u]` is the cumulative count, if you were wondering). A wrong index there puts a
//! cloud in the wrong place at the wrong size, and nothing anywhere reports it.

use crate::engine::pipelines::layouts::Layouts;

use super::buffers::{GpuCtx, zeroed_buffer};
use super::cloud::{CloudDraw, LodNode};
use super::instance::Instance;
use super::objects::InstanceTable;

/// Words per record, mirrored as `const REC_WORDS: u32 = 36u;` in `splat.wgsl`.
pub const REC_WORDS: u64 = 36;

/// Records the table can hold. The builder stops emitting at this many, so a scene with more
/// clouds (or more LOD nodes) than this simply draws the first 256 - it never overruns.
pub const MAX_RECORDS: u64 = 256;

/// One contiguous point range at one spacing, as the compute shader reads it.
///
/// `#[repr(C)]` and 144 bytes, matching `4 + 36 * n` words in the table the shader indexes. The
/// shader still reads it by literal word index; this type is what makes the WRITING side legible
/// and keeps the four pieces from drifting apart.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SplatRecord {
    /// mvp x model, column-major - ONE matrix per record, shared by every point it covers, so a
    /// thread does one mat-vec and no instance fetch. Words 0-15.
    pub mvp_model: [f32; 16],
    /// The instance tint, with `.a` smuggling the MINIMUM radius in pixels (the manifest px,
    /// halved, floored at 0.5). Without a floor, attenuation turns distant clouds to dust.
    /// Words 16-19.
    pub tint: [f32; 4],
    pub first: u32, // word 20 - absolute first point row
    pub count: u32, // word 21
    pub cum: u32,   // word 22 - this record's first GLOBAL thread id
    /// Word 23. Folds the cloud's world-space point footprint and the projection, so the shader
    /// gets its screen radius with one divide by `clip.w`.
    pub k: f32,
    /// Words 24-35: the model's ROTATION columns, translation-free, as three padded vec4s - so a
    /// cloud with normals can rotate them into world space for the lambert term.
    pub rot: [f32; 12],
}

const _: () = assert!(std::mem::size_of::<SplatRecord>() as u64 == REC_WORDS * 4);

/// The two per-pixel buffers both lanes contest, and the group the resolve pass reads them with.
pub struct PixelBufs {
    pub(super) depth: wgpu::Buffer, // one u32 per pixel: winning reverse-Z bits (0 = empty)
    pub(super) color: wgpu::Buffer, // one u32 per pixel: winner's RGBA8
    pub resolve_group: wgpu::BindGroup,
}

impl PixelBufs {
    pub fn new(device: &wgpu::Device, layouts: &Layouts, width: u32, height: u32) -> Self {
        let pixels = (width.max(1) * height.max(1)) as u64 * 4;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        let depth = zeroed_buffer(device, "splat.depth", pixels, usage);
        let color = zeroed_buffer(device, "splat.color", pixels, usage);
        let resolve_group = mk_splat_resolve_group(device, &layouts.splat_resolve, &depth, &color);
        Self { depth, color, resolve_group }
    }
}

/// One lane's record table and the two bind groups that aim the compute shader at it.
pub struct SplatSlot {
    pub(super) recs: wgpu::Buffer,
    pub(super) group0: wgpu::BindGroup,
    pub(super) group1: wgpu::BindGroup,
}

impl SplatSlot {
    /// `label` names the record buffer, `points` are THIS lane's three point buffers, and
    /// `shared` is what every lane binds identically: the frame's two uniforms, the object
    /// table, and the pixel pair the lanes compose into.
    ///
    /// The three point buffers travel as ONE tuple and the shared four as another, in the order
    /// `mk_splat_group1` wants them. That is deliberate: the two groups take five and six
    /// same-typed `&Buffer` arguments, and the stream lane spent five lessons passing its
    /// pixel buffers where its normals belonged, silently, because nothing in the type system
    /// could tell them apart.
    pub fn new(
        device: &wgpu::Device,
        layouts: &Layouts,
        label: &'static str,
        points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer, &PixelBufs),
    ) -> Self {
        let recs = zeroed_buffer(device, label, 16 + MAX_RECORDS * REC_WORDS * 4,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let (mvp, cloud, instances, pixels) = shared;
        let (pos, col, nrm) = points;
        Self {
            group0: mk_splat_group0(device, &layouts.splat_group0, mvp, cloud, instances, &recs),
            group1: mk_splat_group1(device, &layouts.splat_group1, pos, col, nrm, &pixels.depth, &pixels.color),
            recs,
        }
    }

    /// Re-aim both groups after any bound buffer was re-created (a lane grew, or the window
    /// resized). Same argument shape as `new`, for the same reason.
    pub fn rebind(
        &mut self,
        device: &wgpu::Device,
        layouts: &Layouts,
        points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer, &PixelBufs),
    ) {
        let (mvp, cloud, instances, pixels) = shared;
        let (pos, col, nrm) = points;
        self.group0 = mk_splat_group0(device, &layouts.splat_group0, mvp, cloud, instances, &self.recs);
        self.group1 = mk_splat_group1(device, &layouts.splat_group1, pos, col, nrm, &pixels.depth, &pixels.color);
    }
}

fn mk_splat_group0(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    mvp: &wgpu::Buffer,
    cloud: &wgpu::Buffer,
    instances: &wgpu::Buffer,
    recs: &wgpu::Buffer
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor{
        label: Some("splat.group0"),
        layout,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: mvp.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: cloud.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: instances.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: recs.as_entire_binding()},
        ],
    })
}

fn mk_splat_group1(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    pos: &wgpu::Buffer,
    col: &wgpu::Buffer,
    nrm: &wgpu::Buffer,
    sdepth: &wgpu::Buffer,
    scolor: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor{
        label: Some("splat.group1"),
        layout,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: pos.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: col.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 2, resource: sdepth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 3, resource: scolor.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 4, resource: nrm.as_entire_binding()},
        ],
    })
}

fn mk_splat_resolve_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    sdepth: &wgpu::Buffer,
    scolor: &wgpu::Buffer,
) -> wgpu::BindGroup{
    device.create_bind_group(&wgpu::BindGroupDescriptor{
        label: Some("splat.resolve.group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry{binding: 0, resource: sdepth.as_entire_binding()},
            wgpu::BindGroupEntry{binding: 1, resource: scolor.as_entire_binding()},
        ],
    })
}

/// The whole rasterizer: one record slot per lane, the shared pixel pair, and the two pieces of
/// per-frame state that decide whether any of it has to run again.
pub struct Splat {
    pub pixels: PixelBufs,
    pub(super) walked: SplatSlot,
    pub(super) stream: SplatSlot,
    /// Points that will be dispatched this frame, both lanes. 0 = the resolve pass is skipped.
    pub total: u32,
    /// The (mvp, cloud_size) the buffers currently hold splats for; `None` = stale. A still
    /// camera at the same scale re-uses them and the whole compute prelude costs nothing.
    state: Option<([f32; 16], f32)>,
}

impl Splat {
    pub fn new(
        device: &wgpu::Device,
        layouts: &Layouts,
        width: u32,
        height: u32,
        walked_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        stream_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) -> Self {
        let pixels = PixelBufs::new(device, layouts, width, height);
        let (mvp, cloud, instances) = shared;
        Self {
            walked: SplatSlot::new(device, layouts, "splat.rescales", walked_points, (mvp, cloud, instances, &pixels)),
            stream: SplatSlot::new(device, layouts, "splat.stream.recs", stream_points, (mvp, cloud, instances, &pixels)),
            pixels,
            total: 0,
            state: None,
        }
    }

    /// Whatever a bound buffer was re-created - a lane grew, the window resized, the object
    /// table moved - both slots are re-aimed and the cached frame state is dropped.
    pub fn rebind(
        &mut self,
        device: &wgpu::Device,
        layouts: &Layouts,
        walked_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        stream_points: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
        shared: (&wgpu::Buffer, &wgpu::Buffer, &wgpu::Buffer),
    ) {
        let (mvp, cloud, instances) = shared;
        self.walked.rebind(device, layouts, walked_points, (mvp, cloud, instances, &self.pixels));
        self.stream.rebind(device, layouts, stream_points, (mvp, cloud, instances, &self.pixels));
        self.state = None;
    }

    /// The buffers no longer hold this frame's splats. Every writer of a point, a transform or a
    /// knob calls this; the frame calls `is_current` to find out.
    pub fn invalidate(&mut self) {
        self.state = None;
    }

    /// Does the pixel pair already hold exactly this frame? If so the compute prelude is skipped
    /// entirely - a still camera at an unchanged scale pays nothing for its cloud.
    pub fn is_current(&self, state: ([f32; 16], f32)) -> bool {
        self.state == Some(state)
    }

    pub fn mark_current(&mut self, state: ([f32; 16], f32)) {
        self.state = Some(state);
    }

    /// Upload one lane's record table: the four-word header, then the records.
    pub fn write(&self, ctx: &GpuCtx, lane: Lane, header: &[u32; 4], recs: &[SplatRecord]) {
        let slot = match lane { Lane::Walked => &self.walked, Lane::Stream => &self.stream };
        ctx.queue.write_buffer(&slot.recs, 0, bytemuck::bytes_of(header));
        ctx.queue.write_buffer(&slot.recs, 16, bytemuck::cast_slice(recs));
    }

    pub(super) fn slot(&self, lane: Lane) -> &SplatSlot {
        match lane { Lane::Walked => &self.walked, Lane::Stream => &self.stream }
    }
}

/// Which of the two point lanes a record table belongs to.
#[derive(Clone, Copy)]
pub enum Lane {
    Walked,
    Stream,
}

/// Everything the record builder reads that is not a cloud: the frame's camera, the viewport,
/// two runtime knobs, and the object table it looks each cloud's row up in.
///
/// A struct rather than eight parameters, and every field is a copy or a `&` - the builder
/// writes nothing at all.
pub struct RecordCx<'a> {
    pub mvp: &'a [f32; 16],
    pub ortho_h: f32,   // ortho world half-height x unit scale; 0 in perspective
    pub eye: [f32; 3],  // anchored world units
    pub vp_w: f32,
    pub vp_h: f32,
    pub cloud_size: f32,
    pub lod_split_px: f32,
    pub objects: &'a InstanceTable,
}

/// Build the record table for one cloud lane. A record folds the cloud's whole per-frame
/// state: mvp x rebased model as ONE matrix, the tint, the attenuation constant and the
/// model rotation - so a thread does one mat-vec, no instance fetch.
/// Attenuated (world-sized) dots, Potree-style: the record carries k such that the
/// shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its own
/// world-space footprint, so near surfaces close up gap-free and far points shrink.
/// The manifest px is a size FACTOR on the measured spacing.
pub fn records(cx: &RecordCx, draws: &[CloudDraw], nodes: &[LodNode]) -> ([u32; 4], Vec<SplatRecord>, u32) {
    let lod_split = cx.lod_split_px;
    let mut header = [0u32; 4];
    let mut recs: Vec<SplatRecord> = Vec::new();
    let mut cum = 0u32;
    let ortho_h = cx.ortho_h as f64;
    let vp_h = cx.vp_h as f64;
    let aspect = cx.vp_w as f64 / cx.vp_h as f64;
    let eye = cx.eye;
    for &CloudDraw { first, count, instance: inst, spacing, node_first, node_count } in draws {
        let Some(row) = cx.objects.row(inst as usize) else { continue };
        if row.flags & Instance::FLAG_HIDDEN != 0 { continue; }
        let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * cx.cloud_size;
        if px <= 0.0 || header[0] as u64 >= MAX_RECORDS { continue; }
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
        // one record = one contiguous range at one spacing. world radius = spacing x
        // (px/6); k folds the projection so the shader only divides by clip.w:
        //   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
        //   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
        let emit = |f: u32, c: u32, sp: f32, recs: &mut Vec<SplatRecord>, header: &mut [u32; 4], cum: &mut u32| {
            if header[0] as u64 >= MAX_RECORDS { return; }
            let world_r = (sp as f64).max(1.0e-9) * mscale * 0.001 * (px as f64) / 6.0; // metres
            let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h) }
                    else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
            recs.push(SplatRecord {
                mvp_model: m,
                tint,
                first: f,
                count: c,
                cum: *cum,
                k: k as f32,
                // the MODEL rotation columns (translation-free), so a cloud with
                // normals can rotate them into world space for the lambert term
                rot: [
                    b[0], b[1], b[2], 0.0f32,
                    b[4], b[5], b[6], 0.0,
                    b[8], b[9], b[10], 0.0,
                ],
            });
            header[0] += 1;
            *cum += c;
        };
        if cx.lod_split_px > 0.0 && node_count > 0 {
            // Octree LOD, Potree-style screen-error selection: every VISITED node
            // contributes its own subsample, and the walk descends while the node's
            // projected point spacing is coarser than the cutoff - far nodes stop at
            // the root (a handful of coarse points), near nodes go deep. Coarse nodes
            // carry big spacing, so attenuation grows their dots to close the gaps.
            let slice = &nodes[node_first as usize..(node_first + node_count) as usize];
            let mut stack: Vec<usize> = vec![0];
            while let Some(ni) = stack.pop() {
                if header[0] as u64 >= MAX_RECORDS { break; }
                let nd = slice[ni];
                let c = nd.center;
                // FRUSTUM CULL on the node's bounding sphere, in clip space through the
                // folded matrix: an off-screen subtree costs nothing - and without this
                // a close zoom would visit every node and starve the 256-record table.
                let r_m = nd.size as f64 * 0.8660254 * mscale * 0.001; // sphere radius, metres
                let cw = (m[3] * c[0] + m[7] * c[1] + m[11] * c[2] + m[15]) as f64;
                if ortho_h <= 0.0 && cw < -r_m { continue; } // fully behind the eye
                let cx = (m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12]) as f64;
                let cy = (m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13]) as f64;
                let (ndc_x, ndc_y, ry) = if ortho_h > 0.0 {
                    (cx, cy, r_m / ortho_h)
                } else {
                    let w = cw.max(1.0e-9);
                    (cx / w, cy / w, r_m * 1.7320508 / w)
                };
                if ndc_x.abs() > 1.0 + ry / aspect.min(1.0) || ndc_y.abs() > 1.0 + ry {
                    continue; // the whole subtree is outside the view
                }
                // node centre in anchored world units - the eye's space
                let w = [
                    row.model[0] * c[0] + row.model[4] * c[1] + row.model[8] * c[2] + row.model[12],
                    row.model[1] * c[0] + row.model[5] * c[1] + row.model[9] * c[2] + row.model[13],
                    row.model[2] * c[0] + row.model[6] * c[1] + row.model[10] * c[2] + row.model[14],
                ];
                let dist_m = (((w[0] - eye[0]).powi(2) + (w[1] - eye[1]).powi(2) + (w[2] - eye[2]).powi(2)) as f64).sqrt() * 0.001;
                let sp_m = nd.spacing as f64 * mscale * 0.001;
                let sp_px = if ortho_h > 0.0 { sp_m * vp_h / (2.0 * ortho_h) }
                            else { sp_m * 1.7320508 * 0.5 * vp_h / dist_m.max(1.0e-9) };
                let leaf = nd.children.iter().all(|&ch| ch < 0);
                let refine = !leaf && sp_px > lod_split as f64;
                // Dot size: a REFINED node's region also receives all its deeper
                // points, so its own subsample renders at the cloud's measured
                // spacing - otherwise coarse dots blob over the fine layer under
                // them. Only the unrefined FRINGE keeps its coarse node spacing
                // (its points are the only ink there - big dots close the gaps);
                // a node can never be DENSER than the raw cloud, so the measured
                // spacing is also the floor there. Leaves hold raw points.
                let sp = if refine || leaf { spacing } else { nd.spacing.max(spacing) };
                // `nd.first` is relative to this cloud's own first point
                emit(first + nd.first, nd.count, sp, &mut recs, &mut header, &mut cum);
                if refine {
                    for &ch in &nd.children {
                        if ch >= 0 { stack.push(ch as usize); }
                    }
                }
            }
        } else {
            emit(first, count, spacing, &mut recs, &mut header, &mut cum);
        }
    }
    header[1] = cum;
    (header, recs, cum)
}
```

Three things in there earn their place:

- **`const _: () = assert!(size_of::<SplatRecord>() as u64 == REC_WORDS * 4);`** — the record is
  144 bytes on both sides or the build stops. That assert is what the two `256 * 144` literals
  were standing in for.
- **`SplatSlot::new` takes two tuples, not seven buffers.** Six same-typed `&Buffer` parameters in
  a row is how the streamed lane spent five lessons binding its pixel buffers where its normals
  belonged. Grouping them by WHERE THEY COME FROM (this lane's points; the shared frame) makes the
  mistake visible at the call site.
- **`Splat` owns the staleness cache.** `invalidate()` / `is_current(state)` / `mark_current()`
  replace a bare `splat_state = None` written at seven unrelated call sites.

### 4.4 `src/engine/gpu/backdrop.rs`

Two pipelines, two draws, and not one byte of vertex data.

**Create `src/engine/gpu/backdrop.rs`**

```rust
//! `backdrop.rs` - the family that owns no rows: `grid.wgsl` + `background.wgsl`.
//!
//! Two pipelines, two draws, and NOT ONE byte of vertex data. The background is three vertices
//! at the far plane built from `@builtin(vertex_index)`; the grid is fifty vertices of
//! `LineList` built the same way. Neither reads a storage buffer, neither has an instance row,
//! and neither appears in `Gpu`'s field list - which is exactly why they belong together and
//! why this file is the shortest in `engine/gpu/`.
//!
//! It is here to make the point that a family is defined by the ROW it owns, and that "no row"
//! is a legitimate answer. Everything drawn from `@builtin(vertex_index)` alone lands here.

use crate::engine::pipelines::layouts::Layouts;
use crate::engine::pipelines::{PipelineDesc, Target, build::build};

use super::frame::Binds;

const GRID: &str = include_str!("../../shaders/grid.wgsl");
const BACKGROUND: &str = include_str!("../../shaders/background.wgsl");

/// Vertices the grid draws. Fifty, from `@builtin(vertex_index)`, matching the `FLOOR + 6` the
/// shader builds them from.
const GRID_VERTS: u32 = 50;

/// The two pipelines nothing indexes.
pub struct Pipes {
    pub grid: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
}

impl Pipes {
    pub fn descs(device: &wgpu::Device, t: Target, l: &Layouts) -> Self {
        Self {
            // Buffer-less LineList - positions come from @builtin(vertex_index). Depth-tested
            // so geometry hides it, never depth-writing, so it hides nothing.
            grid: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::LineList,
                depth_write: false,
                ..PipelineDesc::opaque("grid", GRID, &[&l.mvp, &l.line])
            }),
            // A buffer-less triangle at the far plane, with no bind groups at all. `Always`,
            // never depth-writing: it paints under everything and blocks nothing.
            background: build(device, t, &PipelineDesc {
                depth_write: false,
                depth_compare: wgpu::CompareFunction::Always,
                ..PipelineDesc::opaque("background", BACKGROUND, &[])
            }),
        }
    }
}

/// Paint the backdrop: the background first, then the grid over it. Returns the draws issued.
///
/// Order is the whole content of this function. The background is `Always` and never writes
/// depth, so it paints under everything and blocks nothing; the grid is depth-TESTED so geometry
/// hides it, and never depth-WRITING so it hides nothing. Both must precede every lane that
/// writes depth, which is why the frame calls this first.
pub fn draw(pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
    pass.set_pipeline(&b.p.backdrop.background);
    pass.draw(0..3, 0..1);
    // Grid second as the depth writes are off, all objects paints over it
    pass.set_pipeline(&b.p.backdrop.grid);
    pass.set_bind_group(0, b.mvp, &[]);
    pass.set_bind_group(1, b.line, &[]);   // for the anchor
    pass.draw(0..GRID_VERTS, 0..1);
    2
}
```

### 4.5 `src/engine/gpu/render.rs`

The frame. `encode_frame` keeps the compute prelude and the pass setup; everything drawn INSIDE
the pass becomes `scene_list`, and the module header carries the order as a table so the argument
for it lives next to the code implementing it.

**Create `src/engine/gpu/render.rs`**

```rust
//! `render.rs` - the frame, as a list you can read.
//!
//! One function decides what a frame IS, and after this lesson you can read it in one screen:
//!
//! ```text
//!   backdrop      background, then grid          neither writes depth
//!   arena         faces, then the sheet fills    depth WRITE on, then off
//!   segments      the solid lane, by LineStyle   tube or ribbon, one branch
//!   splat         the cloud composite            frag_depth, so it occludes solids
//!   glyphs        the vertex markers             after the bands, GreaterEqual
//!   ink prepasses flat depth, if enabled         off by default - see the const
//!   segments      the flat lane                  blended, writes no depth
//!   arena         the lettering                  LAST, on top of its own linework
//!   glyphs        the flat dots
//! ```
//!
//! That order is the entire argument. Everything that WRITES depth comes first (the cloud
//! included, since it went opaque); the flat ink lanes read that depth and never write it. The
//! markers go with the solids so the line ink tests against them - a vertex marker is the topmost
//! ink at its own joint. And the lettering is last of all, because a page paints its text over
//! both its hatching and its linework, which is the one thing draw order can express that a
//! depth buffer cannot when every glyph is coplanar at z = 0.
//!
//! A LIST, not a graph. Nine entries, each a call into the family that owns those rows, each
//! returning the draws it issued. Moving one is moving one line - which is what the payoff at
//! the end of the lesson has you do.


use super::backdrop;
use super::frame::Binds;
use super::{Gpu, splat};

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

impl Gpu {
    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> (u32, u32) {
        let mut draws = 0u32;

        // Splat the clouds by COMPUTE before the render pass. One thread per point,
        // twice (depth race, then colour claim); the render pass composites the result
        // with one fullscreen triangle. TWO record sets - the walked lane and the stream
        // lane bind different point buffers - but one pixel buffer pair: atomics compose
        // across dispatches, so both lanes contest the same per-pixel depth race.
        {
            let cx = splat::RecordCx {
                mvp: &self.frame.mvp_f32,
                ortho_h: self.frame.last_ortho_h,
                eye: self.frame.last_eye,
                vp_w: self.config.width as f32,
                vp_h: self.config.height as f32,
                cloud_size: self.view.cloud_size,
                lod_split_px: self.view.lod_split_px,
                objects: &self.objects,
            };
            let (header, recs, cum) = splat::records(&cx, self.cloud.draws(), self.cloud.nodes());
            let (header_s, recs_s, cum_s) = splat::records(&cx, self.stream.draws(), &[]);
            self.splat.total = cum + cum_s;
            // Static skip: camera still, same scale, nothing rebuilt - the buffers already
            // hold this exact frame's splats, so the whole compute prelude is free.
            let state = (self.frame.mvp_f32, self.view.cloud_size);
            if self.splat.total > 0 && !self.splat.is_current(state) {
                self.splat.write(&self.ctx, splat::Lane::Walked, &header, &recs);
                self.splat.write(&self.ctx, splat::Lane::Stream, &header_s, &recs_s);
                encoder.clear_buffer(&self.splat.pixels.depth, 0, None); // 0 bits = reverse-Z far = empty
                encoder.clear_buffer(&self.splat.pixels.color, 0, None);
                // 2D grid: a 1D dispatch caps at 65535 workgroups (~4.2M threads) and an
                // oversized dispatch invalidates the WHOLE command buffer - the frame
                // silently never draws. 4096-wide rows cover any point count.
                let grid = |n: u32| { let g = n.div_ceil(64); (g.min(4096), g.div_ceil(4096)) };
                let ((gx, gy), (sx, sy)) = (grid(cum), grid(cum_s));
                let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                // BOTH lanes' depth races must settle before EITHER lane claims colours -
                // dispatches in one pass are ordered, so lane order inside each phase is free.
                cp.set_pipeline(&self.pipelines.splat_depth);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat.slot(splat::Lane::Walked).group0, &[]);
                    cp.set_bind_group(1, &self.splat.slot(splat::Lane::Walked).group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat.slot(splat::Lane::Stream).group0, &[]);
                    cp.set_bind_group(1, &self.splat.slot(splat::Lane::Stream).group1, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                cp.set_pipeline(&self.pipelines.splat_color);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat.slot(splat::Lane::Walked).group0, &[]);
                    cp.set_bind_group(1, &self.splat.slot(splat::Lane::Walked).group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat.slot(splat::Lane::Stream).group0, &[]);
                    cp.set_bind_group(1, &self.splat.slot(splat::Lane::Stream).group1, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                self.splat.mark_current(state);
            }
        }

        {
            // Groups 0-2 for this frame, all shared, taken BEFORE the pass opens (B3).
            let b = self.frame.binds(&self.pipelines, &self.objects.bind_group);
            let mut pass = self.targets.begin_pass(encoder, view, wgpu::LoadOp::Clear(color),
                                                  Some(wgpu::LoadOp::Clear(0.0)));

            // Pipelines - sequence of drawing is important:
            // background -> grid -> triangles -> sphere markers -> cylinders -> CLOUD -> ink
            // prepass -> ribbon -> glyph. Everything that WRITES depth comes first (the cloud
            // included, since it went opaque); the flat ink lanes read that depth and never
            // write it. The markers go with the solids so the line ink tests against them -
            // a vertex marker is the topmost ink at its own joint.
            draws += self.scene_list(&mut pass, &b);
        }

        (draws, self.objects.len() as u32)
    }

    /// THE LIST. Nine entries, in the order the frame draws them; each returns its own draw
    /// count so no shared counter has to be threaded through a `&self` borrow (see 48's B2).
    fn scene_list(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        let mut draws = 0u32;
        draws += backdrop::draw(pass, b);

        // Meshes - coordinates, colors and normals are inside the gb.vbo computed
        pass.set_pipeline(&b.p.arena.triangle);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.time, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        draws += self.arena.draw_faces(pass);

        // SHEET FILLS, second - the same vertex table with depth write off. See draw_print.
        draws += self.arena.draw_print(pass, b);

        // Linework, ONE draw per lane, each over its OWN table.
        // pipes = mesh/BRep edges -> real cylinders: the tube radius lifts the ink off the
        // surface it sits on, so silhouette edges never lose the depth test.
        // segments = line/polyline -> flat ribbons: nothing to fight with, and they stay
        // screen-constant and cheap.
        draws += self.seg.draw_solid(pass, &b, self.view.line_style);

        // The cloud lane. drawn with the solids: the compute splatter already resovled
        // every cloud into the per-pixel depth/color buffers, so the whoel lane is one fullscreen triangle
        // that composites them - depth-writing via frag_depth, so splat and solids occlude each other exactly.
        if self.splat.total > 0 {
            pass.set_pipeline(&b.p.splat_resolve);
            pass.set_bind_group(0, b.cloud, &[]);
            pass.set_bind_group(1, &self.splat.pixels.resolve_group, &[]);
            pass.draw(0..3, 0..1);
            draws += 1;
        }

        // Vertex markers are drawn LAST of the solid lane, after the bands, and their
        // pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
        // had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
        // takes the pixel on any tie - so every pixel where the two computed the same depth
        // went to the band, and the disc lost a bite of its rim wherever a band cap crossed
        // it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
        // keep the pixel, which is a strictly weaker condition, so it can only ever draw more
        // of the disc. Real occlusion is untouched - anything genuinely nearer still has a
        // higher depth and still wins.
        //
        // Faces are already down by this point, so a vertex hidden inside the solid stays
        // hidden, which was the reason markers went early in the first place.
        if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
            draws += self.glyphs.draw_markers(pass, b);
        }

        // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
        // write depth (its AA feather would leave halos), so without this nothing in the flat
        // lane occludes anything else in it and pure draw order wins - a point dot then sits
        // on top of a polyline it is behind, at every camera angle.
        // COST: it draws the whole flat lane a SECOND time. On 2D sheets (600k segments, all
        // ribbons) that doubles the frame - so it is off by default and only worth enabling
        // for 3D scenes where ink-vs-ink order is actually visible.
        if INK_DEPTH_PREPASS && self.view.show_lines {
            draws += self.seg.draw_flat_depth(pass, b);
        }
        if INK_DEPTH_PREPASS && self.view.show_points {
            draws += self.glyphs.draw_dots_depth(pass, b);
        }

        if self.view.show_lines {
            draws += self.seg.draw_flat(pass, b);
        }

        // LETTERING, last of everything. A page paints its text on top of its hatching AND
        // its linework, so it lands after the ink lanes above - the one thing draw order can
        // express that a depth buffer cannot, since all of it is coplanar at z = 0.
        draws += self.arena.draw_text(pass, b);

        // Vertex ink, same split: the sphere table is mesh/BRep vertices -> markers (DRAWN
        // EARLIER - right after the faces; see there), this one is flat SDF dots.
        if self.view.show_points {
            draws += self.glyphs.draw_dots(pass, b);
        }

        draws
    }
}
```

**Gate.**

```bash
wc -l src/engine/gpu/cloud.rs src/engine/gpu/stream.rs src/engine/gpu/splat.rs \
      src/engine/gpu/backdrop.rs src/engine/gpu/render.rs
```

```text
 141 src/engine/gpu/cloud.rs
 143 src/engine/gpu/stream.rs
 419 src/engine/gpu/splat.rs
  66 src/engine/gpu/backdrop.rs
 211 src/engine/gpu/render.rs
```

## 5. Where the borrow checker bites — B3, and it is why `scene_list` returns a number

> `scene_list` takes `&self` and a `&mut RenderPass` that was built FROM `self.targets`. Try to
> increment a counter on `Gpu` from inside it and the borrow checker stops you:
>
> ```rust
> fn scene_list(&mut self, pass: &mut wgpu::RenderPass, b: &Binds) { .. }
> //             ^^^^^^^^^ E0502: cannot borrow `*self` as mutable — `self.targets` and
> //                       `self.frame` are already borrowed by `pass` and `b`
> ```
>
> The fix is lesson 49's contract applied to the whole frame: **every draw returns the number of
> draws it issued, and the caller sums them.** That is why `scene_list` ends in a bare `draws` and
> why each of the eleven lines is `draws += …`.

## 6. The steps

### 6.1 `mod.rs` — the modules, the imports, and 28 fields become 3

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod buffers;
pub mod frame;
```

**Replace with:**

```rust
pub mod backdrop;
pub mod buffers;
pub mod cloud;
pub mod frame;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod present;
pub mod segments;
```

**Replace with:**

```rust
pub mod present;
pub mod render;
pub mod segments;
pub mod splat;
pub mod stream;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use arena::Arena;
use glyphs::GlyphLane;
```

**Replace with:**

```rust
use arena::Arena;
use cloud::CloudLane;
pub use cloud::{CloudDraw, LodNode};
use glyphs::GlyphLane;
use splat::Splat;
use stream::StreamLane;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use buffers::{GpuCtx, append_rows, zeroed_buffer};
```

**Replace with:**

```rust
use buffers::GpuCtx;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use bytemuck::bytes_of_mut;

```

**Replace with:**

```rust

```

**Remove** `src/engine/gpu/mod.rs` `/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline` **through** `const INK_DEPTH_PREPASS: bool = false;`

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub point_buffer: wgpu::Buffer, // positions, array<f32>
    pub point_col_buffer: wgpu::Buffer, // colours, array<u32> RGBA8
    pub point_nrm_buffer: wgpu::Buffer, // normals, array<u32> oct16 (u32::MAX = none)
    pub point_cap: u64,     // capacity in POINTS; positions hold 3 floats each
    pub point_col_cap: u64,
    pub point_nrm_cap: u64,
    splat_depth_buf: wgpu::Buffer, // one u32 per pixel: winning reverse-Z bits (0 = empty)
    splat_color_buf: wgpu::Buffer, // one u32 per pixel: winner's RBGA8
    splat_recs: wgpu::Buffer,
    splat_group0: wgpu::BindGroup,
    splat_group1: wgpu::BindGroup,
    splat_resolve_group: wgpu::BindGroup,
    splat_total: u32,
    splat_state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers were build for; None = stale
    cloud_nodes: Vec<LodNode>,
    cloud_draws: Vec<CloudDraw>, // (first, count, instance, spacing)
    pub point_count: u32,
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
    pub stream_draws: Vec<CloudDraw>, // (first, count, instance, spacing)
    splat_stream_recs: wgpu::Buffer,
    splat_group0_stream: wgpu::BindGroup,
    splat_group1_stream: wgpu::BindGroup,
```

**Replace with:**

```rust
    /// The walked point lane: three parallel buffers and the two tables that index them
    /// (`cloud.rs`). Rebuilt whole by every `set_scene`, like the ink lanes.
    pub cloud: CloudLane,
    /// The STREAM lane (`stream.rs`): clouds whose points never existed on the CPU. Its own
    /// three buffers, because the walked lane above is rebuilt whole by every `set_scene` and a
    /// streamed cloud has nothing to be rebuilt from.
    pub stream: StreamLane,
    /// The rasterizer both lanes go through (`splat.rs`): two record tables, and ONE pair of
    /// per-pixel buffers - atomics compose across dispatches, so the lanes contest one depth
    /// race and one composite.
    pub splat: Splat,
```

### 6.2 The constructor

Three lanes build themselves, so `Gpu::build` stops knowing that a record table is
`16 + 256 * 144` bytes or that a normals buffer must be filled with `u32::MAX` rather than
zeroed.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Point cloud tables - empty until set_scene fill them from Upload
        let point_count = 0u32;
        let (point_cap, point_col_cap, point_nrm_cap) = (3u64, 1u64, 1u64);
        let point_buffer = zeroed_buffer(&device, "points.buffer", 12, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_col_buffer = zeroed_buffer(&device, "points.col.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_nrm_buffer = zeroed_buffer(&device, "points.nrm.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
```

**Replace with:**

```rust
        // The point lane - empty until set_scene fills it from Upload.
        let cloud = CloudLane::new(&device);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let pixels = (config.width.max(1) * config.height.max(1)) as u64 * 4;
        let splat_depth_buf = zeroed_buffer(&device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_color_buf = zeroed_buffer(&device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_recs = zeroed_buffer(&device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0 = Self::mk_splat_group0(
            &device,
            &layouts.splat_group0,
            &mvp_buffer,
            &cloud_buffer,
            &objects.buffer,
            &splat_recs
        );

        let splat_group1 = Self::mk_splat_group1(
            &device,
            &layouts.splat_group1,
            &point_buffer,
            &point_col_buffer,
            &point_nrm_buffer,
            &splat_depth_buf,
            &splat_color_buf,
        );
```

**Replace with:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // stream lane: same layouts, its own buffers; grown for real by stream_reserve
        let stream_usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stream_pos_buf = zeroed_buffer(&device, "stream.pos", 12, stream_usage);
        let stream_col_buf = zeroed_buffer(&device, "stream.col", 4, stream_usage);
        let stream_nrm_buf = zeroed_buffer(&device, "stream.nrm", 4, stream_usage);
        let splat_stream_recs = zeroed_buffer(&device, "splat.stream.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0_stream = Self::mk_splat_group0(&device, &layouts.splat_group0, &mvp_buffer, &cloud_buffer, &objects.buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&device, &layouts.splat_group1, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf, &splat_depth_buf, &splat_color_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &device,
            &layouts.splat_resolve,
            &splat_depth_buf,
            &splat_color_buf,
        );
```

**Replace with:**

```rust
        let stream = StreamLane::new(&device);
        let splat = Splat::new(&device, &layouts, config.width, config.height,
            (&cloud.pos, &cloud.col, &cloud.nrm), (&stream.pos, &stream.col, &stream.nrm),
            (&mvp_buffer, &cloud_buffer, &objects.buffer));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            point_buffer,
            point_col_buffer,
            point_nrm_buffer,
            point_cap,
            point_col_cap,
            point_nrm_cap,
            splat_depth_buf,
            splat_color_buf,
            splat_recs,
            splat_group0,
            splat_group1,
            splat_resolve_group,
            splat_total: 0,
            splat_state: None,
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

**Replace with:**

```rust
            cloud,
            stream,
            splat,
```

### 6.3 The three bind-group builders leave, and `rebuild_splat_groups` becomes a forwarder

Note the destructure in the replacement: `let Gpu { splat, cloud, stream, ctx, layouts, frame,
objects, .. } = self;` — B1 from lesson 47, and the body that needs it most: six disjoint fields,
one of them `&mut`.

**Remove** `src/engine/gpu/mod.rs` `    // splat helpers - one compute-visible buffer entry, and the three bind groups,` **through** `    }`

**Remove** `src/engine/gpu/mod.rs` `    fn mk_splat_group1(` **through** `    }`

**Remove** `src/engine/gpu/mod.rs` `    fn mk_splat_resolve_group(` **through** `    }`

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.objects.buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.point_buffer, &self.point_col_buffer, &self.point_nrm_buffer, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_group0_stream = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.objects.buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.ctx.device, &self.layouts.splat_resolve, &self.splat_depth_buf, &self.splat_color_buf);

    }
```

**Replace with:**

```rust
    /// Re-aim both splat slots after any bound buffer was re-created.
    fn rebuild_splat_groups(&mut self){
        let Gpu { splat, cloud, stream, ctx, layouts, frame, objects, .. } = self;
        splat.rebind(&ctx.device, layouts,
            (&cloud.pos, &cloud.col, &cloud.nrm), (&stream.pos, &stream.col, &stream.nrm),
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));
    }
```

### 6.4 `set_scene`, the log, and the streaming API

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Raw cloud lane, same deal. `cloud_draws` carries each cloud's absolute first-point
        // offset, which `Scene` keeps running across files - so the draw records append too.
        let mut pos_rows = self.point_count * 3;
        append_rows(&self.ctx, "points.buffer",
            &mut self.point_buffer, &mut pos_rows, &mut self.point_cap, &up.cloud_pos);
        let mut col_rows = self.point_count;
        append_rows(&self.ctx, "points.col.buffer",
            &mut self.point_col_buffer, &mut col_rows, &mut self.point_col_cap, &up.cloud_col);
        let mut nrm_rows = self.point_count;
        append_rows(&self.ctx, "points.nrm.buffer",
            &mut self.point_nrm_buffer, &mut nrm_rows, &mut self.point_nrm_cap, &up.cloud_nrm);
        self.point_count = pos_rows / 3;
        // The walk numbers a cloud's nodes from the start of ITS upload; the lane's table is
        // cumulative, so every draw's node slice is rebased on the way in - the same thing
        // `Scene::cloud_base` already does for the point rows.
        let node_base = self.cloud_nodes.len() as u32;
        self.cloud_nodes.extend_from_slice(&up.cloud_nodes);
        self.cloud_draws.extend(up.cloud_draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
        self.rebuild_splat_groups();
        self.splat_state = None;
```

**Replace with:**

```rust
        // The point lane, same deal - and its three buffers are bound by the splat groups,
        // so a growth there is what makes them stale.
        if self.cloud.append(&self.ctx, &up.cloud) {
            self.rebuild_splat_groups();
        }
        self.splat.invalidate();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
self.glyphs.spheres() + self.glyphs.dots(), self.glyphs.spheres(), self.point_count
```

**Replace with:**

```rust
self.glyphs.spheres() + self.glyphs.dots(), self.glyphs.spheres(), self.cloud.points()
```

**Find** in `src/engine/gpu/mod.rs`:

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
        let pos = zeroed_buffer(&self.ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&self.ctx.device, "stream.nrm", cap * 4, usage);
        if self.stream_count > 0 {
            let mut enc = self.ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.stream_pos_buf, 0, &pos, 0, self.stream_count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.stream_col_buf, 0, &col, 0, self.stream_count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.stream_nrm_buf, 0, &nrm, 0, self.stream_count as u64 * 4);
            self.ctx.queue.submit([enc.finish()]);
        }
        // The wire has no normals, and a zeroed buffer is NOT "no normal" - oct code 0 decodes
        // to a real direction. Fill the new region with the sentinel, in 1M-row slabs so the
        // staging cost stays bounded.
        let fill = vec![u32::MAX; 1 << 20];
        let mut at = self.stream_count as u64;
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            self.ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            self.ctx.queue.submit([]);
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
        self.stream_draws.push(CloudDraw { first: self.stream_count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
        self.stream_pos_at = self.stream_count;
        self.stream_col_at = self.stream_count;
        self.stream_count += count;
    }
```

**Replace with:**

```rust
    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so all three buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        if self.stream.begin(&self.ctx, count, instance) {
            self.rebuild_splat_groups();
        }
        self.splat.invalidate();
    }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 41's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        if let Some(d) = self.stream_draws.last_mut() {
            if d.spacing == 0.0 && self.stream_pos_at == d.first && pos.len() >= 6 {
                let n = (pos.len() / 3).min(2048);
                let mut gaps: Vec<f32> = (1..n).map(|i| {
                    let (a, b) = ((i - 1) * 3, i * 3);
                    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
                }).filter(|g| *g > 0.0).collect();
                if !gaps.is_empty() {
                    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    d.spacing = gaps[gaps.len() / 2];
                }
            }
        }
        self.ctx.queue.write_buffer(&self.stream_pos_buf, self.stream_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.stream_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.ctx.queue.submit([]);
        self.splat_state = None; // new points - the splat buffers are stale
    }
```

**Replace with:**

```rust
    /// One slice of a streaming cloud's positions. The lane writes it; `Gpu` only has to know
    /// that new points make the splat buffers stale.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        self.stream.push_pos(&self.ctx, pos);
        self.splat.invalidate(); // new points - the splat buffers are stale
    }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.ctx.queue.write_buffer(&self.stream_col_buf, self.stream_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.stream_col_at += col.len() as u32;
        self.ctx.queue.submit([]);
        self.splat_state = None;
    }
```

**Replace with:**

```rust
    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.stream.push_col(&self.ctx, col);
        self.splat.invalidate();
    }
```

### 6.5 `resize`, the anchor, and the two big bodies that leave

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let pixels = (width * height) as u64 * 4;
            self.splat_depth_buf = zeroed_buffer(&self.ctx.device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.splat_color_buf = zeroed_buffer(&self.ctx.device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.rebuild_splat_groups();
            self.splat_state = None;
```

**Replace with:**

```rust
            self.splat.pixels = splat::PixelBufs::new(&self.ctx.device, &self.layouts, width, height);
            self.rebuild_splat_groups();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.splat_state = None; // instance model moved - splats are stale
```

**Replace with:**

```rust
            self.splat.invalidate(); // instance model moved - splats are stale
```

**Remove** `src/engine/gpu/mod.rs` `    /// Build the record table for one cloud lane. A record folds the cloud's whole per-frame` **through** `    }`

**Remove** `src/engine/gpu/mod.rs`

```rust
    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
```

```rust
    }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.point_count = 0;
        self.cloud_draws.clear();
        self.cloud_nodes.clear();
```

**Replace with:**

```rust
        self.cloud.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            cloud_draws: Vec::new(),
            cloud_nodes: Vec::new(),
            point_count,
```

**Delete**

**Find** in `src/engine/gpu/mod.rs`:

```rust
            stream,
            splat,

            view: View::from_env(),
```

**Replace with:**

```rust
            stream,
            splat,
            view: View::from_env(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));
    }

    /// A cloud is about to STREAM in.
```

**Replace with:**

```rust
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));
    }


    /// A cloud is about to STREAM in.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.objects.update_inside_flags(&self.ctx, view_proj, self.scene_min, self.scene_max);
    }



    /// MSAA sample count for a scene.
```

**Replace with:**

```rust
        self.objects.update_inside_flags(&self.ctx, view_proj, self.scene_min, self.scene_max);
    }


    /// MSAA sample count for a scene.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
/// Routing lives in `app::scene::Scene`, one draw per lane in `clear`.





pub struct Gpu {
```

**Replace with:**

```rust
/// Routing lives in `app::scene::Scene`, one draw per lane in `clear`.




pub struct Gpu {
```

### 6.6 `Upload` — the last five flat columns become a group

**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::arena::ArenaRows;
```

**Replace with:**

```rust
use super::arena::ArenaRows;
use super::cloud::CloudRows;
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
    /// The point family's rows: three columns per point plus the two tables that index them
    /// (`cloud.rs`). 20 B a point, and a lidar scan brings tens of millions.
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
            cloud: CloudRows::new(),
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
use super::{CloudDraw, LodNode};

```

**Replace with:**

```rust

```

### 6.7 `pipelines/mod.rs` — down to the list

What is left is the LIST plus the two compute pipelines and the fullscreen resolve: 52 lines, from
845 at the start of the block.

**Find** in `src/engine/pipelines/mod.rs`:

```rust
use crate::engine::gpu::{arena, glyphs, segments};
```

**Replace with:**

```rust
use crate::engine::gpu::{arena, backdrop, glyphs, segments};
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
const GRID: &str = include_str!("../../shaders/grid.wgsl");
const BACKGROUND: &str = include_str!("../../shaders/background.wgsl");

```

**Replace with:**

```rust

```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub grid: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
```

**Replace with:**

```rust
    /// The two pipelines that read no row at all (`backdrop.rs`).
    pub backdrop: backdrop::Pipes,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            // Buffer-less LineList - positions come from @builtin(vertex_index). Depth-tested
            // so geometry hides it, never depth-writing, so it hides nothing.
            grid: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::LineList,
                depth_write: false,
                ..PipelineDesc::opaque("grid", GRID, &[&l.mvp, &l.line])
            }),
            seg: segments::Pipes::descs(device, t, l),
            glyphs: glyphs::Pipes::descs(device, t, l),
            // A buffer-less triangle at the far plane, with no bind groups at all. `Always`,
            // never depth-writing: it paints under everything and blocks nothing.
            background: build(device, t, &PipelineDesc {
                depth_write: false,
                depth_compare: wgpu::CompareFunction::Always,
                ..PipelineDesc::opaque("background", BACKGROUND, &[])
            }),
```

**Replace with:**

```rust
            seg: segments::Pipes::descs(device, t, l),
            glyphs: glyphs::Pipes::descs(device, t, l),
            backdrop: backdrop::Pipes::descs(device, t, l),
```

### 6.8 The walk and the harnesses

**Replace-all** `src/app/scene.rs` `t.cloud_pos` -> `t.cloud.pos` (5 hits)

**Replace-all** `src/app/scene.rs` `t.cloud_col` -> `t.cloud.col` (1 hits)

**Replace-all** `src/app/scene.rs` `t.cloud_nrm` -> `t.cloud.nrm` (1 hits)

**Replace-all** `src/app/scene.rs` `t.cloud_nodes` -> `t.cloud.nodes` (3 hits)

**Replace-all** `src/app/scene.rs` `t.cloud_draws` -> `t.cloud.draws` (2 hits)

**Replace-all** `src/app/scene.rs` `self.tables.cloud_draws` -> `self.tables.cloud.draws` (1 hits)

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

**Find** in `examples/check_determinism.rs`:

```rust
        same!(cloud_pos); same!(cloud_col); same!(cloud_nrm);
```

**Replace with:**

```rust
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
```

**Find** in `examples/check_lean.rs`:

```rust
        same!(cloud_pos); same!(cloud_col); same!(cloud_nrm);
```

**Replace with:**

```rust
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
```

**Replace-all** `src/app/scene.rs` `self.tables.cloud_pos` -> `self.tables.cloud.pos` (1 hits)

## Step 6 — `build` is a list of lanes

`Gpu::build` is 232 lines. Seventy-four of them negotiate with the driver — instance, adapter,
limits, device, surface format — and eighty more build four uniform buffers. Neither is a lane.
What is left once both move out is what the function is for: one `::new` per lane, then the
struct literal.

The driver half goes first, into a file of its own. It is the only code in the viewer that talks
to the machine rather than to the scene, and that is the seam: everything in `device.rs` is
settled once at start-up by what the hardware has, everything in `mod.rs` is settled per scene
and per frame by what the file holds.

`open` hands back the four values by name rather than a built `Gpu`, so `build` destructures them
and every line after it still reads `device`, `queue` and `config` exactly as before.

**Create `src/engine/gpu/device.rs`**:

```rust
//! `device.rs` — the five wgpu objects, in order, and nothing else.
//!
//! Instance → Surface → Adapter → Device + Queue → configure. This is the only file in the
//! viewer that talks to the driver rather than to the scene, and it is separate for that reason:
//! everything here is decided ONCE at start-up by what the machine has, while everything in
//! `mod.rs` is decided per scene and per frame by what the file holds. Mixing the two put 74
//! lines of adapter negotiation in front of `Gpu::build`, which is a list of lanes.
//!
//! `open` returns the four values by name rather than a built `Gpu`, so `build` destructures
//! them and every line after it reads `device`/`queue`/`config` exactly as before.

/// What the driver hands back: the canvas (`None` when headless), the two handles, and the
/// surface settings the rest of start-up reads for size and format.
pub struct Opened {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub async fn open(
    window: Option<std::sync::Arc<winit::window::Window>>,
    width: u32,
    height: u32,
) -> anyhow::Result<Opened> {

    Ok(Opened { surface, device, queue, config })
}

```

**Move** from `src/engine/gpu/mod.rs` to `src/engine/gpu/device.rs`, **after**:

```rust
        // 1. Instance — the driver entry point. WebGPU only in the browser, never WebGL.
```

```rust
        if let Some(s) = &surface { s.configure(&device, &config); }
```

```rust
) -> anyhow::Result<Opened> {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod cloud;
```

**Add below it:**

```rust
pub mod device;
```

Now `build` calls it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    ) -> anyhow::Result<Self> {
```

**Add below it:**

```rust
        // Instance → Surface → Adapter → Device + Queue → configure, all in `device.rs`:
        // the only part of start-up decided by the machine rather than by the scene.
        let device::Opened { surface, device, queue, config } = device::open(window, width, height).await?;
```

#### The four uniform blocks belong to the frame

The camera matrix, the clock, the pen and the cloud block are read by every family and written by
none of them, so they are the frame's. `frame.rs` already owns the struct that holds them; it
should own their construction too. In `build` they were eighty lines of `create_buffer_init`
followed by `create_bind_group`, four times over, differing only in which struct they carry.

None of the initial values is a default worth tuning — an identity matrix, `t = 0`, a pen with no
eye and no anchor. The first frame overwrites all of them through `write_camera`. They exist so
the buffers have a defined size before a bind group points at one.

**Find** in `src/engine/gpu/frame.rs`:

```rust
impl FrameUniforms {
```

**Add below it:**

```rust
    /// Build the four uniform blocks. They are the frame's, not any lane's, so they are made
    /// here rather than in `Gpu::build` - which had 80 lines of `create_buffer_init` +
    /// `create_bind_group` in the middle of its list of lanes, four times over, differing only
    /// in which struct they carry.
    ///
    /// The initial values are all "no camera yet": an identity matrix, t = 0, and a pen whose
    /// eye and anchor are zero. The first frame overwrites every one of them through
    /// `write_camera`, so nothing here is a default worth tuning - it exists so the buffers have
    /// a defined size before a bind group points at them.
    pub fn new(
        device: &wgpu::Device,
        layouts: &crate::engine::pipelines::layouts::Layouts,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        use wgpu::util::DeviceExt;

        // Camera MVP uniform - buffer + layout + bind group (group 0)
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &layouts.mvp,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        // Time Uniform
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &layouts.time,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
        });

        // Line uniform - scree-constant thickness
        let line_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: config.height as f32,
                vp_w: config.width as f32,
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });

        // point cloud unioform - the cloud's OWN global size + viewport (reuses layouts.line)
        let cloud_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: config.width as f32,
                vp_h: config.height as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });

        Self {
            mvp_buffer,
            mvp_bind_group,
            line_buffer,
            line_bind_group,
            time: 0.0,
            time_buffer,
            time_bind_group,
            cloud_buffer,
            cloud_bind_group,
            mvp_f32: [0.0; 16],
            last_ortho_h: 0.0,
            last_eye: [0.0; 3],
        }
    }


```

Then the three blocks come out of `build`, and what stood in for them is one call.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Camera MVP uniform - buffer + layout + bind group (group 0)
        use wgpu::util::DeviceExt;
        let mvp_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_bind_group: wgpu::BindGroup = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &layouts.mvp,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        // Time Uniform
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("time.buffer"),
            contents: bytemuck::bytes_of(&0.0f32),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("time.bind_group"),
            layout: &layouts.time,
            entries: &[wgpu::BindGroupEntry{ binding: 0, resource: time_buffer.as_entire_binding() }],
        });
```

**Replace with:**

```rust
        // The four per-frame uniform blocks - camera, time, pen, cloud - are the FRAME's,
        // not any lane's, so `frame.rs` builds them (S5).
        let frame = FrameUniforms::new(&device, &layouts, &config);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Line uniform - scree-constant thickness
        let line_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: config.height as f32,
                vp_w: config.width as f32,
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });
```

**Delete**

**Find** in `src/engine/gpu/mod.rs`:

```rust

        // The point lane - empty until set_scene fills it from Upload.
        let cloud = CloudLane::new(&device);

        // point cloud unioform - the cloud's OWN global size + viewport (reuses layouts.line)
        let cloud_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: config.width as f32,
                vp_h: config.height as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &layouts.line,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });
```

**Replace with:**

```rust
        // The point lane - empty until set_scene fills it from Upload.
        let cloud = CloudLane::new(&device);

```

The splat groups bind two of those buffers, and the struct literal no longer builds the value it
is given.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            (&mvp_buffer, &cloud_buffer, &objects.buffer));
```

**Replace with:**

```rust
            (&frame.mvp_buffer, &frame.cloud_buffer, &objects.buffer));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            frame: FrameUniforms {
                mvp_buffer, // shared: camera
                mvp_bind_group,
                line_buffer,  // shared: px-sizing for cylinders + spheres
                line_bind_group,
                time: 0.0,
                time_buffer,    // shared: animation
                time_bind_group,
                cloud_buffer,
                cloud_bind_group,
                mvp_f32: [0.0; 16],
                last_ortho_h: 0.0,
                last_eye: [0.0; 3],
            },
```

**Replace with:**

```rust
            frame,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use frame::{CloudUniform, FrameInput, FrameUniforms, LineUniform};
```

**Replace with:**

```rust
use frame::{FrameInput, FrameUniforms};
```

`build` is now 81 lines and `gpu/mod.rs` 524 → 374, with no function in it over 81 lines. The
frame's uniforms are in `frame.rs`, the driver is in `device.rs`, and what remains between them
is the list this file was always meant to be.

## 7. Proving nothing changed — four ladders

**(1) The compiler.** Both targets, `--all-targets` natively, and exactly the warning set lesson 47
left. Three new ones appear — `bytes_of_mut`, `append_rows`, `zeroed_buffer`, all unused in
`mod.rs` once the lanes own their buffers — and step 6.1 removes them.

**(2) The tests.** `cargo xtest` — **4 passed**, unchanged from 48. They cannot catch this lesson's
whole subject: `SplatRecord`'s field ORDER against the shader's literal indices. The mirror tests
parse a `struct` out of a `.wgsl`, and `splat.wgsl` declares none — it reads `table[b + 22u]`. The
`const _: () = assert!` on the SIZE is the only mechanical check, and two swapped fields pass it.
Hence the word numbers in the record's doc comments.

**(3) The line multiset.**

```bash
python3 docs/_replay_check.py --moves <end-of-48 tree> /tmp/w49 docs/50-frame-list.md
```

```text
docs/50-frame-list.md: 57 ops, 0 failed
docs/50-frame-list.md: 1 move source(s), 0 not byte-identical
```

**(4) The pixels.** `./docs/_gate.sh` twice, plus the two harnesses. `lion` and `bunny_cloud` are
the only mandatory scenes that go through the record builder, and one wrong word index would put
the cloud in the wrong place at the wrong size with nothing reported.

```text
gate OK                        (both runs)
lion.pb: DETERMINISTIC
mesh_bunny.pb: IDENTICAL
```

**What none of the four covers: the streamed lane.** Every streaming scene is advisory and its
`.pb` is gitignored, so the mandatory gate never streams a point. `stream.rs` moves here on the
strength of the compiler and a reading — worth knowing when you change it.

## 8. What you can now do in one line

Reorder the frame. The vertex markers are drawn LAST of the solid lane, after the bands. Drawn
FIRST, a marker has to win the depth test STRICTLY, so every pixel where a disc and a band cap
compute the same depth goes to the band and the disc loses a bite of its rim. Move them up and
watch it happen.

**Type all four steps.** The first two move the entry up, the last two put it back. Do **not**
undo it with `git checkout` — you have not committed lesson 50 yet.

**8a.** Cut the entry. **Find** in `src/engine/gpu/render.rs`:

```rust
        // Vertex markers are drawn LAST of the solid lane, after the bands, and their
        // pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
        // had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
        // takes the pixel on any tie - so every pixel where the two computed the same depth
        // went to the band, and the disc lost a bite of its rim wherever a band cap crossed
        // it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
        // keep the pixel, which is a strictly weaker condition, so it can only ever draw more
        // of the disc. Real occlusion is untouched - anything genuinely nearer still has a
        // higher depth and still wins.
        //
        // Faces are already down by this point, so a vertex hidden inside the solid stays
        // hidden, which was the reason markers went early in the first place.
        if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
            draws += self.glyphs.draw_markers(pass, b);
        }

        // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
```

**Replace with:**

```rust
        // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
```

**8b.** Paste it above the linework. **Find** in `src/engine/gpu/render.rs`:

```rust
        // Linework, ONE draw per lane, each over its OWN table.
```

**Replace with:**

```rust
        // Vertex markers are drawn LAST of the solid lane, after the bands, and their
        // pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
        // had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
        // takes the pixel on any tie - so every pixel where the two computed the same depth
        // went to the band, and the disc lost a bite of its rim wherever a band cap crossed
        // it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
        // keep the pixel, which is a strictly weaker condition, so it can only ever draw more
        // of the disc. Real occlusion is untouched - anything genuinely nearer still has a
        // higher depth and still wins.
        //
        // Faces are already down by this point, so a vertex hidden inside the solid stays
        // hidden, which was the reason markers went early in the first place.
        if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
            draws += self.glyphs.draw_markers(pass, b);
        }

        // Linework, ONE draw per lane, each over its OWN table.
```

Render the bunny:

```bash
cargo run -q --release --example selftest --target x86_64-unknown-linux-gnu --     /tmp/mk.ppm assets/scenes/bunny.toml
```

```text
[INFO] headless frame: 9 draws, 6 objects, 900x700
wrote /tmp/mk.ppm  900x700  non-background pixels: 44214 (7.0%)
```

**25,353 pixels of 630,000 change** — every marker on the bunny's wireframe loses rim wherever a
band crosses it, and ink drops by exactly one, 44,215 to 44,214. The draw count does not move:
same eleven entries, same nine draws, different order. One line.

**8c.** Put it back. **Find** in `src/engine/gpu/render.rs`:

```rust
        // Vertex markers are drawn LAST of the solid lane, after the bands, and their
        // pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
        // had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
        // takes the pixel on any tie - so every pixel where the two computed the same depth
        // went to the band, and the disc lost a bite of its rim wherever a band cap crossed
        // it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
        // keep the pixel, which is a strictly weaker condition, so it can only ever draw more
        // of the disc. Real occlusion is untouched - anything genuinely nearer still has a
        // higher depth and still wins.
        //
        // Faces are already down by this point, so a vertex hidden inside the solid stays
        // hidden, which was the reason markers went early in the first place.
        if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
            draws += self.glyphs.draw_markers(pass, b);
        }

        // Linework, ONE draw per lane, each over its OWN table.
```

**Replace with:**

```rust
        // Linework, ONE draw per lane, each over its OWN table.
```

**8d.** **Find** in `src/engine/gpu/render.rs`:

```rust
        // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
```

**Replace with:**

```rust
        // Vertex markers are drawn LAST of the solid lane, after the bands, and their
        // pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
        // had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
        // takes the pixel on any tie - so every pixel where the two computed the same depth
        // went to the band, and the disc lost a bite of its rim wherever a band cap crossed
        // it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
        // keep the pixel, which is a strictly weaker condition, so it can only ever draw more
        // of the disc. Real occlusion is untouched - anything genuinely nearer still has a
        // higher depth and still wins.
        //
        // Faces are already down by this point, so a vertex hidden inside the solid stays
        // hidden, which was the reason markers went early in the first place.
        if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
            draws += self.glyphs.draw_markers(pass, b);
        }

        // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
```

`./docs/_gate.sh --only bunny` prints 44215 / 9 / 6 again.

## 9. What is deliberately not here

- **A render graph.** §1c. Eleven entries, four physical arguments, one readable order.
- **Splitting `Gpu::build`.** `mod.rs` ends at **524 lines**, over the 300 the architecture
  targets, and ~250 are `build` constructing uniforms, layouts and templates. Resource
  construction, not a lane; it goes with `RowTable<T>` at **57**.
- **A mirror test for `SplatRecord`.** `splat.wgsl` declares no struct to mirror — it indexes raw
  words. Giving it one is a shader change, not a moves-only job. **57**.
- **Fixing the streamed lane's lack of pixel coverage.** A streamed scene must join the mandatory
  gate before that lane can be changed safely. Named here so it is not forgotten.
- **`enum Spacing { World(f32), Pixels(f32) }`.** `Instance.spacing` still carries world units for
  meshes and PIXELS for clouds in one f32. Splitting it is a behaviour change under a pixel gate;
  the first lesson needing both units on one row names it.
- **Sub-object identity.** A row carries `instance_id` — the OBJECT — and nothing saying which
  face or edge of the kernel mesh it came from. `scene.rs` names the intended fix, the
  `guid -> range` map; lesson **114** builds it with the id buffer.

## 10. Expected state

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
wc -l src/engine/gpu/mod.rs src/engine/gpu/render.rs src/engine/pipelines/mod.rs
grep -cE 'wgpu::Buffer|\.wgsl' src/engine/gpu/render.rs
awk '/fn scene_list/,/^    \}$/' src/engine/gpu/render.rs | grep -cE 'draws \+='
cargo xtest 2>&1 | tail -3
```

```text
18

   524 src/engine/gpu/mod.rs
   211 src/engine/gpu/render.rs
    52 src/engine/pipelines/mod.rs

0

11

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

| | end-of-48 | end-of-49 |
|---|---|---|
| `Gpu` fields | 43 | **18** |
| `gpu/mod.rs` | 1,055 | **524** |
| `gpu/cloud.rs` | — | **141** |
| `gpu/stream.rs` | — | **143** |
| `gpu/splat.rs` | — | **419** |
| `gpu/backdrop.rs` | — | **66** |
| `gpu/render.rs` | — | **211** |
| `pipelines/mod.rs` | 67 | **52** |
| `encode_frame` | 163 lines | **the list is 11** |
| the splat record | 36 words, no type | **`SplatRecord`, 144 B, asserted** |
| `Upload` flat columns | 7 + 4 groups | **2 + 5 groups** |

## Recap

```text
45 made a pipeline a value. 46 put a floor under the families. 47 built the object table every
row points at, and arena.rs as the worked example. 48 did it twice more, for the two ink
families, and settled what a module IS: the row, every table of it, every pipeline that reads
it, every draw that issues one.

49 finishes the engine on the two things that were left. The point lanes - walked and streamed -
become cloud.rs and stream.rs, separated by lifetime rather than format; the rasterizer they
share becomes splat.rs, and the record it packs, which had no Rust type at all and was read
back in the shader by literal word index, becomes a 144-byte SplatRecord with an assert. The
grid and the background - two pipelines that own no row - become backdrop.rs, which is the
proof that "a family is defined by the row it owns" survives the case where the row is nothing.

And then encode_frame's 163 lines become a list of eleven, each an entry naming a family. Gpu
holds eighteen fields, down from a hundred and sixteen at the end of 44; gpu/mod.rs is 524 lines
down from 2,447. The law: the frame is an ordered list, and nothing in it reaches past its own
family. Grep render.rs for a buffer or a shader and you get nothing.
```

## Edited

`src/engine/gpu/cloud.rs`, `stream.rs`, `splat.rs`, `backdrop.rs`, `render.rs` (all NEW) ·
`src/engine/gpu/mod.rs` (28 fields → 3; the group builders, the record builder and `encode_frame`
all leave) · `src/engine/gpu/upload.rs` (the `cloud` group) ·
`src/engine/pipelines/mod.rs` (the last two row-less descs leave) ·
`src/app/scene.rs` (six `Replace-all`s and `stream.retarget`) ·
`examples/check_determinism.rs`, `examples/check_lean.rs`.

## Reference

`git diff end-of-48..end-of-49 -- session_viewer/src` is the whole lesson as one patch.

## Next

Lesson **50** — **a producer's signature names the shaders it can reach.** The engine is done;
everything above is `app/`. Run the evidence:

```bash
wc -l src/app/scene.rs
grep -c 'Geometry::' src/app/scene.rs
awk '/fn push_mesh/,/^    \}$/' src/app/scene.rs | wc -l
```

`scene.rs` is one 1,333-line file holding all thirteen geometry arms, the file sweeps, the
converters and a 314-line `push_mesh` with eight parameters. It becomes `app/walk/` — one file per
GEOMETRY TYPE this time, not per row format, because that is the axis the app side turns on:
`mesh.rs`, `curves.rs`, `points.rs`, `frames.rs`, `cloud.rs`, `brep.rs`, `surface.rs`. The
signatures become narrow sinks: `walk_line(s: &mut SegRows, ..)` cannot reach a cloud column — a
law the compiler enforces rather than a convention the reviewer checks.
