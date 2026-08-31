# 46 The floor is not a lane

> Lesson [88](88-gtao.md) samples this frame's depth buffer for ambient occlusion, and lesson
> [113](113-hiz-occlusion.md) builds a hi-Z pyramid from it. Both cost one field access — because
> `targets.rs` keeps the depth **Texture** with `TEXTURE_BINDING` on, not just a view you cannot
> re-view. Nothing you can see changes: same ink, same draw count, same object count, on every
> scene and config. Answer key: branch `end-of-46`, so
> `git diff end-of-45..end-of-46 -- session_viewer/src` is this lesson as one patch.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file; if you find yourself improving a line while moving it, stop — the
> deferral list at the end says which lesson owns that change.**

## 1. Why this seam

### 1a. The evidence — run it on your own tree

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE '_(cap|capacity):'
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE '_(count|rows):'
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE ': wgpu::Buffer,'
grep -c '&self.device, &self.queue,' src/engine/gpu/mod.rs
grep -oE '&mut self\.[a-z_0-9]+_(cap|capacity)' src/engine/gpu/mod.rs | sort -u | wc -l
```

```text
106   fields on Gpu
 13   capacities        (12 `*_cap` + `stream_capacity`)
 14   counts            (`*_count` + `instance_rows`)
 28   wgpu::Buffer fields, of which 16 are growable row tables
 10   call sites that pass `&self.device, &self.queue,` as a pair
 10   capacities grown through `append_rows`/`append_index_run`
```

Thirteen capacities, fourteen counts, and the sixteen buffers they govern: **forty-three of
`Gpu`'s 106 fields** — two in every five — spell the same `(buffer, count, cap)` triple out
longhand, over and over. Four of them, quoted here:

```rust
    pub pipe_buffer: wgpu::Buffer,
    pub pipe_count: u32,
    pub pipe_cap: u64,
```

and then `segment_*`, `sphere_*`, `glyph_*`, `point_*`, `point_col_*`, `point_nrm_*`, the four
arena runs, the instance table, the stream lane. Ten of the thirteen are appended by one function
— the last grep's number — and three grow by hand in the same shape. Its signature currently reads:

```rust
fn append_rows<T: bytemuck::Pod>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    buf: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[T],
) -> bool {
```

Seven parameters, four of which are one value split into four; the first two are the pair `grep`
just found ten times. Wherever both handles are needed, they are needed together.

That is the whole lesson. `GpuCtx` folds the pair, `GrowBuf` names the triple, and the four files
under them — `frame.rs`, `targets.rs`, `present.rs`, `view.rs` — take everything that belongs to
no family, so lessons 47-49 can hand a family its own file and find nothing generic left in it.

### 1b. The law this enforces, stated as what it forbids

> **The floor knows no lane.** Nothing in `buffers.rs`, `frame.rs`, `targets.rs`, `present.rs` or
> `view.rs` may name a row type, a `.wgsl` file or a `Geometry::` variant — and `upload.rs`, on
> the other side of the same line, may not name a `wgpu::` type. A buffer, its count and its cap
> are ONE value; a knob is not a uniform.

A family that needs something from the floor gets it as a parameter (`&GpuCtx`, `&Binds`,
`&Targets`), never by the floor learning about the family. Lessons 47, 48 and 49 each move a
family under this floor; a `CylinderSegment`-shaped hole would have to be widened three times.

### 1c. The rejected alternative

The obvious cut is `RowTable<T>` right now — `GrowBuf` plus a CPU-side `Vec<T>`, the bind group
and a `guid → Range` map, so a lane is one generic value and `set_scene` is a loop. **Do not make
it.** That CPU mirror is the second copy of the scene lessons 37 and 38 spent themselves deleting:
263 MB of browser heap freed on a 13.8 M-point lidar scan, precisely by *not* keeping the rows
after upload. `append_rows` already returns the `grew` bool, which is all lessons 45-51 need. It
lands at **57**, where the reconcile pass is the first caller that needs the `guid → Range` half.

## 2. Where the code lives after this lesson

| symbol | today's home | new home | who may touch it |
|---|---|---|---|
| `device`, `queue` | two `Gpu` fields, passed as a pair 10× | **`buffers.rs::GpuCtx`** | everything under `gpu/`, as `&GpuCtx` — never held |
| `GrowBuf { buf, count, cap, usage, label }` | 43 fields spelling it out | **`buffers.rs`** | each family folds its own copy in, at 47-49 |
| `append_rows`, `append_index_run`, `zeroed_buffer`, `mk_rows_group` | free fns + one `impl Gpu` assoc fn in `gpu/mod.rs` | **`buffers.rs`** | anything under `gpu/`; knows no row format |
| `ArenaUpload` | `gpu/mod.rs`, next to the GPU it feeds | **`upload.rs::Upload`** | `app/` writes it, `Gpu::set_scene` reads it, nobody else |
| `drop_rows` + the 14-line drop sweep | `app/scene.rs` | **`upload.rs::Upload::drop_uploaded`** | `Scene::upload_to` only |
| `mvp_*`, `line_*`, `time*`, `cloud_buffer`, `cloud_bind_group`, `mvp_f32`, `last_ortho_h`, `last_eye` | 12 `Gpu` fields | **`frame.rs::FrameUniforms`** | the frame encoder writes; families READ through `Binds` |
| `LineUniform`, `CloudUniform`, `line_thickness_px` | bottom of `gpu/mod.rs` | **`frame.rs`** | `write_camera`/`write_cloud` and `Gpu::build`'s two literals |
| the 6 group-0-to-2 bind groups at 42 draw sites | `&self.<x>_bind_group` | **`frame.rs::Binds<'a>`** | taken once, before the pass opens (B3) |
| `depth_view`, `msaa_view`, `samples`, `create_depth_view`, `create_msaa_view` | 3 fields + 2 assoc fns | **`targets.rs::Targets`** | `resize`, `set_scene`'s MSAA flip, `begin_pass` |
| the 24-line `RenderPassDescriptor` | inline in `encode_frame` | **`targets.rs::begin_pass`** | every encoder under `gpu/`, 4 params |
| `clear`, `render_offscreen`, `bench_frames` | `impl Gpu` in `gpu/mod.rs` | **`present.rs`** | `state.rs` and the headless harness |
| `line_style`, `show_points`, `show_lines`, `show_mesh_edges`, `cloud_size`, `edl_strength`, `lod_split_px` | 7 `Gpu` fields, 4 of them reading env in the struct literal | **`view.rs::View` + `from_env`** | `lib.rs`'s key handler writes; the encoder reads |

The compartment, and what crosses each boundary:

```text
              app/scene.rs  ---- &Upload, 19 columns ---->  Gpu::set_scene
                    |                                             |
                    v                                             |
            +-----------------+                                   |
            |   upload.rs     |  Upload · drop_uploaded           |
            |  no `wgpu::`    |  the app/engine line              |
            +-----------------+                                   v
  +--------------------------------------------------------------------------+
  | engine/gpu/mod.rs   Gpu { surface, ctx, config, layouts, pipelines,       |
  |                           frame, targets, view, … }   86 fields (was 106) |
  | build · set_scene · encode_frame · resize · msaa_now · rebase_anchor      |
  +----+------------+-------------+---------------+--------------+-----------+
       |            |             |               |              |
   &GpuCtx     &FrameInput    &TextureView    &mut Encoder   &View (read)
   bool grew   Binds<'a>      RenderPass<'e>  Option<Frame>   Q W E L [ ]
       v            v             v               v              v
  buffers.rs    frame.rs      targets.rs      present.rs      view.rs
  GpuCtx        FrameUniforms Targets         Frame           View
  GrowBuf       FrameInput    new             begin_present   from_env
  append_rows   Binds         begin_pass      end_present     line_style
  append_index  write_camera  depth: Texture  clear           show_points
  zeroed_buffer write_cloud   msaa:  Texture  render_offscr   show_lines
  mk_rows_group LineUniform   samples         bench_frames    show_mesh_edges
                CloudUniform                                  cloud_size
                line_thick_px                                 edl_strength
                                                              lod_split_px
  --------------------------------------------------------------------------
   not one of the five names a row type, a `.wgsl` file or a Geometry:: variant
```

**Exit litmus, grep it when you are done:**

```bash
grep -c 'wgpu::' src/engine/gpu/upload.rs
grep -nE 'CylinderSegment|GlyphPoint|RenderVertex|include_str!' \
     src/engine/gpu/buffers.rs src/engine/gpu/frame.rs src/engine/gpu/targets.rs \
     src/engine/gpu/present.rs src/engine/gpu/view.rs
```

`0`, and no output. The first says `upload.rs` is app-side data; the second says the floor knows
no lane.

## 3. Files we touch

| file | what | step | why |
|---|---|---|---|
| `src/engine/gpu/buffers.rs` | **NEW, 140 lines** | 4.1, 6.1, 6.2 | a buffer, its count and its cap are one value |
| `src/engine/gpu/upload.rs` | **NEW, 110 lines** | 4.2, 6.3 | the app/engine line deserves a file, not a neighbour |
| `src/engine/gpu/view.rs` | **NEW, 50 lines** | 4.3, 6.4 | a knob gates a draw; it is not a uniform |
| `src/engine/gpu/frame.rs` | **NEW, 177 lines** | 4.4, 6.5 | what every shader reads, once per frame |
| `src/engine/gpu/targets.rs` | **NEW, 88 lines** | 4.5, 6.6 | what a pass writes into, and the one descriptor |
| `src/engine/gpu/present.rs` | **NEW, 162 lines** | 4.6, 6.7 | the three ways a frame leaves the encoder |
| `src/engine/gpu/mod.rs` | 2139 → **1691** | 6.1-6.7 | loses the floor; keeps `build`, `set_scene`, `encode_frame` |
| `src/app/scene.rs` | 1365 → **1340** | 6.3 | `drop_rows` and the 14-line sweep go with the table |
| `src/lib.rs` | 5 Replace-alls | 6.4 | the key handler now toggles `gpu.view.*` |
| `src/selftest.rs` | 2 Replace-alls | 6.4 | two knob reads |

**Line budgets.** A bad paste is visible by size alone: `buffers.rs` = **140**, `upload.rs` =
**110**, `view.rs` = **50**, `frame.rs` = **177**, `targets.rs` = **88**, `present.rs` = **162**.
727 new lines against 473 cut, so the tree gains **254**. Off by more than a line or two? Re-read
the file before you run the gate.

New code this lesson may invent: `GpuCtx`, `GrowBuf`, `FrameInput`, `Binds`, `Frame`,
`begin_present`/`end_present`, `View::from_env`, `Targets`, and the six `//!` headers. Everything
else already existed. Shape taken while a body is moving — `usage`/`label` on `GrowBuf`, the two
`LoadOp` parameters on `begin_pass`, the depth kept as a `Texture` — is free; the rest §9 defers.

## 4. The six destination files, created first

Every new file is created before anything is cut, so each step is a deletion plus a re-point, not
a two-ended edit you cannot compile in the middle of, and almost every move can land `at the end`.

`targets.rs` arrives with its bodies already in it, because **a Move that also renames is two
edits pretending to be one**: `Targets::new` is the two `create_*_view` bodies fused, and
`begin_pass` loses four spaces of indent. Step 6.6 is then three deletions; the rest are skeletons.

### 4.1 `src/engine/gpu/buffers.rs`

`GpuCtx` and `GrowBuf` are the two genuinely new values here. Nothing constructs `GrowBuf` yet —
each family folds its own triple in at 47-49 — so it carries `#[allow(dead_code)]` until 47. The
`label` field is not decoration: without it, the binding-size error a scene throws the first time
it crosses a cap names an anonymous buffer.

**Create `src/engine/gpu/buffers.rs`**

```rust
//! `buffers.rs` — the floor beneath the five families (ARCHITECTURE.md §4).
//!
//! Owns the two values every family is built from: `GpuCtx`, the device+queue pair a family is
//! handed instead of holding (F4), and `GrowBuf`, one growable row table — its buffer, the rows
//! already on it, and the rows that fit — plus the four free functions that append to and bind
//! one. Callable by anything under `engine/gpu/`; it knows no shader, no row format and no
//! `Geometry::` variant, which is what "floor" means.

/// The two wgpu handles every buffer operation needs, as ONE value.
///
/// A family never HOLDS these (F4) - it is handed `&GpuCtx` at the call, which is what keeps
/// `new`/`append`/`rebind` at three parameters instead of four and lets one `let Gpu { ctx, .. }`
/// destructure serve every lane in a body (§7 B1).
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// A growable GPU row table: the buffer, the rows already ON it, and the rows that FIT.
///
/// The three always move together - `append_rows` takes all three by `&mut` and writes all three
/// - and `Gpu` carries that triple twelve times over, which is thirty-three of its fields. Each
/// family collapses its own copy into this struct as it is created (47-49), so `pipe_count`
/// becomes `pipes.count` and the arithmetic that keeps them in step stops being spelled out at
/// every call site.
///
/// `usage` and `label` ride along because growth RE-CREATES the buffer: the new one must be made
/// with the same usage flags, and it must keep the same name or the binding-size error a scene
/// throws the moment it crosses a cap names an anonymous buffer. Sixteen distinct labels are in
/// use today.
///
/// Nothing constructs one YET: `Gpu` still carries the twelve triples spread flat, and each is
/// folded here by the lesson that creates its family - so the attribute comes off at 47, with
/// `Arena`.
#[allow(dead_code)]
pub struct GrowBuf {
    pub buf: wgpu::Buffer,
    pub count: u32,
    pub cap: u64,
    pub usage: wgpu::BufferUsages,
    pub label: &'static str,
}
```

### 4.2 `src/engine/gpu/upload.rs`

Header and imports only — the table itself is moved in, whole, at step 6.3.

**Create `src/engine/gpu/upload.rs`**

```rust
//! `upload.rs` — the CPU side of one `set_scene` (ARCHITECTURE.md §4).
//!
//! Owns `Upload`: every row `app::scene::Scene` walks, handed to `Gpu::set_scene` once and then
//! forgotten. Written by `app/`, read by `engine/gpu/`, and it names no wgpu type in either
//! direction - which is the line between the two halves of the viewer. It reads no shader and
//! no `Geometry::` variant; the walk decides which column a variant lands in.
//!
//! It arrives here FLAT - one struct, nineteen columns, today's names. Each family regroups its
//! own columns into a `<Family>Rows` sink as it is created (47-49), so a producer can be handed
//! the two columns it may write instead of all nineteen.

use crate::math::Mat4;
use session_rust::RenderVertex;

use super::{CloudDraw, CylinderSegment, GlyphPoint, LodNode};
```

### 4.3 `src/engine/gpu/view.rs`

Three of the seven knobs are single lines scattered through `Gpu`, so they are typed here; the
other four are a contiguous block, moved in at 6.4. `from_env` splits the same way: three
`VIEWER_*` reads below, four defaults moved in.

**Create `src/engine/gpu/view.rs`**

```rust
//! `view.rs` — the runtime knobs (ARCHITECTURE.md §4, §5.6).
//!
//! Owns `View`: every toggle the keyboard reaches. A knob GATES a draw or PICKS a pipeline -
//! `show_lines` decides whether the ribbon draw happens at all, `line_style` decides which of
//! two pipelines the edge table is drawn through - and it is never a uniform: nothing here is
//! written into a buffer or read by a `.wgsl` file. Read by the frame encoder, written by the
//! key handler in `lib.rs`; a family never reads it, it is TOLD.

use super::LineStyle;

/// The knobs, as one value on `Gpu`.
pub struct View {
    pub cloud_size: f32, // global SCALE on per-cloud sizes, [ and ] keys
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
    pub lod_split_px: f32, // octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (VIEWER_LOD)
}

impl View {
    /// The startup values. Four knobs read a `VIEWER_*` env var, which is native-only - on wasm
    /// `std::env::var` always fails, so the browser gets the defaults and query strings like
    /// `?thickness=` are the only runtime knob there.
    pub fn from_env() -> Self {
        Self {
            cloud_size: std::env::var("VIEWER_CLOUD_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
            lod_split_px: std::env::var("VIEWER_LOD").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
        }
    }
}
```

### 4.4 `src/engine/gpu/frame.rs`

Three structs and an empty impl. `FrameUniforms` is the twelve `Gpu` fields collected;
`FrameInput` is what the two writers READ, so they take three parameters instead of six; `Binds`
is groups 0-2 for one frame, every field a `&` for the reason §5 gives.

**Create `src/engine/gpu/frame.rs`**

```rust
//! `frame.rs` — what every shader reads, once per frame (ARCHITECTURE.md §4, seam A3/S5).
//!
//! Owns the three uniform blocks that are not any one family's: the camera matrix (group 0), the
//! pen/`LineUniform` block (group 1) and the cloud block, each with its buffer and bind group,
//! plus the camera values the CPU caches for the same frame. `ribbon.wgsl`, `cylinder.wgsl`,
//! `sphere.wgsl`, `glyph.wgsl` and `grid.wgsl` all declare `LineUniform`; `splat.wgsl` and
//! `splat_resolve.wgsl` declare `CloudUniform`. Written once by the frame encoder, read by every
//! family through `Binds` - a family never writes a uniform (F5).

use session_rust::{Point, Xform};

use crate::engine::pipelines::Pipelines;

use super::buffers::GpuCtx;
use super::{View, eye_from_view_proj, ortho_half_height};

/// The per-frame uniform blocks, as one value on `Gpu`.
pub struct FrameUniforms {
    pub(super) mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub(super) mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub(super) line_buffer: wgpu::Buffer, // shared: px-sizing for cylinders + spheres
    pub(super) line_bind_group: wgpu::BindGroup,
    pub(super) time: f32,  // shared: animation
    pub(super) time_buffer: wgpu::Buffer,
    pub(super) time_bind_group: wgpu::BindGroup,
    pub(super) cloud_buffer: wgpu::Buffer,
    pub(super) cloud_bind_group: wgpu::BindGroup,
    pub(super) mvp_f32: [f32; 16],
    pub(super) last_ortho_h: f32, // ortho half-height this frame (0=perspective), for the plat k
    pub(super) last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
}

/// Everything `write_camera`/`write_cloud` READ, as one borrowed value.
///
/// It exists so those two take three parameters instead of six, and so the read set is written
/// down: the surface size, this frame's view-projection, the anchor the instance rows are rebased
/// about, and the knobs. `anchor` is `None` until the first rebase.
pub struct FrameInput<'a> {
    pub config: &'a wgpu::SurfaceConfiguration,
    pub view_proj: &'a Xform,
    pub anchor: Option<&'a Point>,
    pub view: &'a View,
}

/// Groups 0-2 for one frame, plus the pipelines - all of it SHARED (§7 B3).
///
/// Every field is a `&`, which is the whole point: a `RenderPass<'e>` borrows the encoder for the
/// pass's entire lifetime, so anything read inside the pass must already be a shared reborrow
/// taken before it. One `&mut` in here and no draw would compose.
pub struct Binds<'a> {
    pub(crate) p: &'a Pipelines,
    pub(crate) mvp: &'a wgpu::BindGroup,
    pub(crate) time: &'a wgpu::BindGroup,
    pub(crate) line: &'a wgpu::BindGroup,
    pub(crate) cloud: &'a wgpu::BindGroup,
    pub(crate) instances: &'a wgpu::BindGroup,
}

impl FrameUniforms {
}
```

### 4.5 `src/engine/gpu/targets.rs`

Created whole. `Targets::new` is the two `create_*_view` bodies fused — same textures, sizes and
sample count, with three changes: each local is named after the field it becomes, the TEXTURE is
kept and not just its view, and depth gains `TEXTURE_BINDING`. `begin_pass` is `encode_frame`'s
descriptor one indent shallower, with the two `LoadOp` literals as parameters.

Keeping `depth` as a `wgpu::Texture` is the free shape here: a view cannot be re-viewed and a
usage flag cannot be added after creation, so a lesson that wants to SAMPLE the depth would
otherwise change this field's type and every line that reads it. `depth_load: Option<LoadOp<f32>>`
is the other: `None` means no depth attachment, which is what lesson 74's gumball overlay needs.

**Create `src/engine/gpu/targets.rs`**

```rust
//! `targets.rs` — the attachments a render pass writes into (ARCHITECTURE.md §4, seam S2a).
//!
//! Owns the reverse-Z depth texture and the multisampled colour target, the sample count they
//! were built at, and `begin_pass` - the one place a `RenderPassDescriptor` is spelled out. Every
//! encoder under `engine/gpu/` opens its pass through it; no `.wgsl` file and no row format is
//! named here. Sample count belongs to the PASS, so it cannot be chosen per family (see
//! `Gpu::msaa_now`).

/// The frame's attachments, as one value on `Gpu`.
///
/// The TEXTURES are kept, not just their views. A view cannot be re-viewed and a usage flag
/// cannot be added after creation, so keeping `depth` (with `TEXTURE_BINDING` already on) is what
/// lets a later pass SAMPLE the depth without any field here changing type.
pub struct Targets {
    #[allow(dead_code)] // read by 88's GTAO and 113's hi-Z, which sample this texture
    pub(super) depth: wgpu::Texture,
    pub(super) depth_view: wgpu::TextureView,
    #[allow(dead_code)] // same: 107 reads the resolved colour back
    pub(super) msaa: wgpu::Texture,
    pub(super) msaa_view: wgpu::TextureView,
    pub(super) samples: u32, // MSAA sample count this scene chose (see `msaa_for`)
}

impl Targets {
    /// Create the reverse-Z depth texture and the multisampled colour target the frame
    /// renders into (resolved to the surface each frame), both sized to the surface.
    pub(crate) fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> Self {
        let depth = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth.create_view(&wgpu::TextureViewDescriptor::default());
        let msaa = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d{ width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let msaa_view = msaa.create_view(&wgpu::TextureViewDescriptor::default());
        Self { depth, depth_view, msaa, msaa_view, samples }
    }

    /// Open the scene pass. `load` is what happens to the colour attachment, `depth_load` to the
    /// depth one - `None` means no depth attachment at all, which is what an overlay pass that
    /// must not test against the scene wants.
    pub(crate) fn begin_pass<'e>(
        &self,
        encoder: &'e mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        load: wgpu::LoadOp<wgpu::Color>,
        depth_load: Option<wgpu::LoadOp<f32>>,
    ) -> wgpu::RenderPass<'e> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            // MSAA off (samples == 1): draw straight to the swapchain view - a
            // 1-sample attachment must NOT carry a resolve target.
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if self.samples > 1 { &self.msaa_view } else { target },
                resolve_target: if self.samples > 1 { Some(target) } else { None },
                depth_slice: None,
                ops: wgpu::Operations {
                    load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: depth_load.map(|load| wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(
                    wgpu::Operations{load,
                    store:wgpu::StoreOp::Store,
                }),
                stencil_ops: None }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}
```

### 4.6 `src/engine/gpu/present.rs`

`Frame` and an empty `impl Gpu`. The three methods are moved in at 6.7, and `begin_present` /
`end_present` are cut out of `clear`'s own body there — so nothing of them is typed twice.

**Create `src/engine/gpu/present.rs`**

```rust
//! `present.rs` — getting a finished frame onto the screen, or into a buffer (§4, seam S1b).
//!
//! Owns the three ways a frame LEAVES the encoder: the swapchain (`begin_present`/`end_present`,
//! and `clear` as their composition), a readback texture (`render_offscreen`, which is what the
//! pixel gate drives) and a timing loop (`bench_frames`). It encodes nothing itself - every one
//! of them calls `encode_frame` in `mod.rs` - and it names no shader and no row format.
//!
//! `begin_present` and `end_present` are split so a caller can append its OWN pass to the same
//! encoder, in a fixed order, before the frame is submitted: that is how the UI layer will draw
//! without `engine/` ever naming it.

use session_rust::Xform;

use super::Gpu;

/// One frame in flight: what was acquired, where it draws, and the commands so far.
///
/// `surface` is `None` for a frame that has nothing to present. `view` and `encoder` are public
/// because the point of the split is that a later pass can use them.
pub struct Frame {
    surface: Option<wgpu::SurfaceTexture>,
    #[allow(dead_code)] // read by 69's egui pass and 74's overlay, which draw into this view
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

impl Gpu {
}
```

### 4.7 The six module lines

`view.rs` is declared and re-exported here rather than in its own step, because `frame.rs` says
`use super::View` and would not compile without it.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::performance::Performance;
```

**Add below it:**

```rust

pub mod buffers;
pub mod frame;
pub mod present;
pub mod targets;
pub mod upload;
pub mod view;
pub use view::View;
```

Gate. Six modules, no callers yet. The unused-import and never-read warnings it prints are the
worklist for §6 — every one is answered by a step below.

```bash
cargo check --target wasm32-unknown-unknown --lib
wc -l src/engine/gpu/*.rs
```

```text
   42 src/engine/gpu/buffers.rs     140 when 6.1 and 6.2 are done
   60 src/engine/gpu/frame.rs       177
 2147 src/engine/gpu/mod.rs        1691
   28 src/engine/gpu/present.rs     162
   88 src/engine/gpu/targets.rs      88   created whole, and already final
   15 src/engine/gpu/upload.rs      110
   29 src/engine/gpu/view.rs         50
```

## 5. Where the borrow checker bites — B1, and it bites in the next step

Read this before you type 6.2 — the one place where a hand-typing reader is likely to conclude
the refactor was wrong, when what is wrong is the shape of the call.

> **The failing form.** Once `device` and `queue` are one field, appending looks like a method:
>
> ```rust
> impl Gpu {
>     fn append_pipes(&mut self, rows: &[CylinderSegment]) -> bool {
>         append_rows(&self.ctx, "pipes.buffer",
>             &mut self.pipe_buffer, &mut self.pipe_count, &mut self.pipe_cap, rows)
>     }
> }
> // and then, in set_scene:
> if self.append_pipes(&up.pipes) { self.rebind_pipes(); }
> ```
>
> **The error.** `error[E0499]: cannot borrow *self as mutable more than once at a time`, or
> `error[E0502]: cannot borrow self.ctx as immutable because it is also borrowed as mutable`, the
> moment two lanes appear in one body. **A method borrows ALL of `self`.**
>
> **The compiling form** borrows FIELDS, which are disjoint places:
>
> ```rust
> let Gpu { ctx, layouts, arena, seg, glyphs, .. } = self;   // ONE destructure
> if arena.append(ctx, &up.arena) { arena.rebind(ctx, layouts); }
> if seg  .append(ctx, &up.seg)   { seg  .rebind(ctx, layouts); }
> ```
>
> **The rule.** Pass `&GpuCtx`; never hold it. A free function or a method on the LANE takes
> `(ctx, …)`, and the caller borrows `ctx` and the lane as two separate fields of `Gpu` — which is
> why every family method is `fn append(&mut self, ctx: &GpuCtx, rows: &Rows) -> bool`.
>
> **It recurs** whenever two lanes are touched in one body: at 47, 48, 49 and 51. Today is the
> easy case — the calls stay free functions, so `append_rows(&self.ctx, …, &mut self.pipe_buffer,
> …)` is already two disjoint field borrows. Type it that way now and 47's method is right.

## 6. The steps

Leaves before roots, one file per step, and the field list changes LAST within a step. Each step
is the round trip: extend the destination, move the bodies to the end, Replace-all the paths
INSIDE the new file, move the fields, fix the call sites, delete the dead forwarders, gate.

A move pastes its region with a blank line in front. That is right at the end of a file; inside an
item the next edit is a **stitch** — a `Find` spanning the blank line to join the halves. Every
stitch is marked, and each is where the body's signature or indent changes.

### 6.1 `buffers.rs` — the four functions that know no lane

`append_rows`, `append_index_run` and `zeroed_buffer` are free functions in `gpu/mod.rs`;
`mk_rows_group` is an associated function on `Gpu` that never reads a field. Four bodies, none of
which knows what a row means — the definition of the floor.

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use view::View;
```

**Add below it:**

```rust
use buffers::{GpuCtx, append_index_run, append_rows, mk_rows_group, zeroed_buffer};
```

`GpuCtx` comes along unused until the next step — one warning, and the import line is already in
its final form, which is what ladder 2 wants.

Now the two appenders: the twelve-line doc comment plus all of `append_rows`, 45 lines ending on
`}` at column 0. The doc's first two lines wrongly describe the index run below it, and they stay
wrong — **a body you are moving is not a body you are fixing.**

**Move** `src/engine/gpu/mod.rs`

```rust
/// Grow-and-append one index run. Same shape as the solid arena's own append: the existing
```

**through**

```rust
}
```

**to** `src/engine/gpu/buffers.rs` **at the end**

**Move** `src/engine/gpu/mod.rs` `fn append_index_run(` **through** `}` **to** `src/engine/gpu/buffers.rs` **at the end**

28 lines, ending on the same `}`. Both are private in `gpu/mod.rs` and public in the floor, which
is one word each:

**Replace-all** `src/engine/gpu/buffers.rs` `fn append_rows` → `pub fn append_rows` (1 hit)

**Replace-all** `src/engine/gpu/buffers.rs` `fn append_index_run` → `pub fn append_index_run` (1 hit)

Two cuts, three blank lines left where two functions used to be:

**Find** in `src/engine/gpu/mod.rs`:

```rust
const INK_DEPTH_PREPASS: bool = false;



/// One cloud's contiguous point range, as the record builder sees it. It was a
```

**Replace with:**

```rust
const INK_DEPTH_PREPASS: bool = false;

/// One cloud's contiguous point range, as the record builder sees it. It was a
```

`mk_rows_group` is the one body here that cannot be moved: inside `impl Gpu`, it lands one indent
shallower, and a move that also re-indents is two edits pretending to be one. Typed here, cut there.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
    queue.write_buffer(ibo, *count as u64 * 4, bytemuck::cast_slice(data));
    *count += data.len() as u32;
}
```

**Add below it:**

```rust

/// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
pub fn mk_rows_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str, buf: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
    })
}
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
    fn mk_rows_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, label: &str, buf: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
        })
    }

    fn rebuild_splat_groups(&mut self){
```

**Replace with:**

```rust
    fn rebuild_splat_groups(&mut self){
```

**Replace-all** `src/engine/gpu/mod.rs` `Self::mk_rows_group` → `mk_rows_group` (9 hits)

If that count is not 9, you cut the wrong region — every one of the nine is a bind group built
over one storage buffer, four in `build` and five in `set_scene`.

Last, `zeroed_buffer`. Its doc comment is not above it — a lesson-36 edit glued it to the top of
`line_thickness_px`'s doc, twenty lines down. Move the function, then re-home the comment.

**Move** `src/engine/gpu/mod.rs` `fn zeroed_buffer(` **through** `}` **to** `src/engine/gpu/buffers.rs` **at the end**

**Replace-all** `src/engine/gpu/buffers.rs` `fn zeroed_buffer` → `pub fn zeroed_buffer` (1 hit)

**Find** in `src/engine/gpu/buffers.rs`:

```rust
pub fn zeroed_buffer(
```

**Add above it:**

```rust
/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the empty-category placeholders both rely on that guarantee.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the empty-category placeholders both rely on that guarantee.
/// On-screen pen weight in px. Default 2.0.
```

**Replace with:**

```rust
/// On-screen pen weight in px. Default 2.0.
```

`buffers.rs` so far — the table of contents you should be able to read off the file:

```text
  1-  7  //! header — the floor, and what it may not know
  9- 17  GpuCtx { device, queue }
 19- 42  GrowBuf { buf, count, cap, usage, label }   #[allow(dead_code)] until 47
 44- 89  append_rows — grow, copy the prefix GPU-side, write the tail, return `grew`
 91-117  append_index_run
119-126  mk_rows_group — one storage buffer at binding 0
128-142  zeroed_buffer
```

Gate:

```bash
cargo check --target wasm32-unknown-unknown --lib
wc -l src/engine/gpu/buffers.rs src/engine/gpu/mod.rs      # 142  2049
```

### 6.2 `device` and `queue` become `ctx`

Ten call sites pass the two as a pair, and every family from 47 on is handed the pair rather than
holding it. Fold them: the floor's two signatures, then the ten pairs, then the leftovers.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
pub fn append_rows<T: bytemuck::Pod>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
```

**Replace with:**

```rust
pub fn append_rows<T: bytemuck::Pod>(
    ctx: &GpuCtx,
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
pub fn append_index_run(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
```

**Replace with:**

```rust
pub fn append_index_run(
    ctx: &GpuCtx,
```

Four Replace-alls re-root the two bodies. `zeroed_buffer` and `mk_rows_group` keep their
`&wgpu::Device` parameter, which is why this is four narrow substitutions and not one rename:

**Replace-all** `src/engine/gpu/buffers.rs` `zeroed_buffer(device,` → `zeroed_buffer(&ctx.device,` (2 hits)

**Replace-all** `src/engine/gpu/buffers.rs` `enc = device.create_command_encoder` → `enc = ctx.device.create_command_encoder` (2 hits)

**Replace-all** `src/engine/gpu/buffers.rs` `queue.submit([enc.finish()]);` → `ctx.queue.submit([enc.finish()]);` (2 hits)

**Replace-all** `src/engine/gpu/buffers.rs` `queue.write_buffer` → `ctx.queue.write_buffer` (2 hits)

Now `gpu/mod.rs`. The two fields become one, and their two trailing comments become one — the only
comment on an existing line that this lesson rewrites:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub device: wgpu::Device,                // Handle to the GPU, used to create resources (textures, buffers, pipelines).
    pub queue: wgpu::Queue,                  // Used to submit work to the GPU (draw calls, resource updates).
```

**Replace with:**

```rust
    // `device` makes GPU resources (textures, buffers, pipelines), `queue` submits work to the
    // GPU (draw calls, resource updates). One value, so a family is HANDED `&GpuCtx` (F4).
    pub ctx: GpuCtx,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            surface,
            device,
            queue,
            config,
```

**Replace with:**

```rust
            surface,
            ctx: GpuCtx { device, queue },
            config,
```

The two locals in `build` keep their names — `GpuCtx { device, queue }` is field-init shorthand,
so nothing else in that 400-line constructor changes. Then the ten pairs:

**Replace-all** `src/engine/gpu/mod.rs` `&self.device, &self.queue,` → `&self.ctx,` (10 hits)

Three places are re-rooted by hand first: the two `create_*_view` call sites, which 6.6 deletes
whole, and the `submit` in `clear`, which 6.7 turns into `end_present`. `--moves` cannot cancel a
line the sweep renames and a later step deletes, so these five by hand keep ladder 2 silent:

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, samples);
```

**Replace with:**

```rust
            self.depth_view = Self::create_depth_view(&self.ctx.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.ctx.device, &self.config, samples);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, self.samples);
```

**Replace with:**

```rust
            self.depth_view = Self::create_depth_view(&self.ctx.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.ctx.device, &self.config, self.samples);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.queue.submit([encoder.finish()]);
        output.present();
```

**Replace with:**

```rust
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
```

**Replace-all** `src/engine/gpu/mod.rs` `self.device` → `self.ctx.device` (33 hits)

**Replace-all** `src/engine/gpu/mod.rs` `self.queue` → `self.ctx.queue` (25 hits)

33 and 25, after the ten pairs and the five hand-edited lines are gone. If your counts are 47 and
36 you ran these two first; undo and do them in the printed order. One hit is inside a
commented-out block in `rebuild_instances` — re-rooting a comment that quotes code is right.

Gate. This is where a `&mut self` method would have failed (§5): every call takes `&self.ctx`
beside a `&mut self.<field>`, which are disjoint places:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -c 'self\.device\|self\.queue' src/engine/gpu/mod.rs        # 0
./docs/_gate.sh
```

### 6.3 `upload.rs` — `ArenaUpload` becomes `Upload`, moved flat

`ArenaUpload` is the app's side of `set_scene`: nineteen columns of rows, written by
`app::scene::Scene`, read once by `Gpu`, then dropped. It names no `wgpu` type, so `gpu/mod.rs` is
the one place it does not belong. It moves **flat**; 47, 48 and 49 regroup the columns per family.

**Move** `src/engine/gpu/mod.rs`

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
```

**through**

```rust
}
```

**to** `src/engine/gpu/upload.rs` **at the end**

**Move** `src/engine/gpu/mod.rs` `impl ArenaUpload {` **through** `}` **to** `src/engine/gpu/upload.rs` **at the end**

**Replace-all** `src/engine/gpu/upload.rs` `ArenaUpload` → `Upload` (2 hits)

Same tidy-up as 6.1 — two regions cut, three blank lines left:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub children: [i32; 8],
}



/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
```

**Replace with:**

```rust
    pub children: [i32; 8],
}

/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
```

**Replace-all** `src/engine/gpu/mod.rs` `ArenaUpload` → `Upload` (4 hits)

Four in `gpu/mod.rs`: `set_scene`'s parameter and three comments. Then the re-export, so
`crate::engine::gpu::Upload` keeps working for `app/` and the examples:

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod view;
```

**Add below it:**

```rust
pub use upload::Upload;
```

**Replace-all** `src/app/scene.rs` `ArenaUpload` → `Upload` (5 hits)

Now the sweep that empties the table after an upload: fourteen `drop_rows` calls in
`Scene::upload_to`, reaching into `Upload`'s columns through a `let t = &mut self.tables` — a
method on `Upload` written out longhand at the call site. Both halves go to the table.

**Find** in `src/app/scene.rs`:

```rust
        let t = &mut self.tables;
        drop_rows(&mut t.verts);
        drop_rows(&mut t.vids);
        drop_rows(&mut t.idx);
        drop_rows(&mut t.idx_print);
        drop_rows(&mut t.idx_text);
        drop_rows(&mut t.pipes);
        drop_rows(&mut t.segments);
        drop_rows(&mut t.spheres);
        drop_rows(&mut t.glyphs);
        drop_rows(&mut t.cloud_pos);
        drop_rows(&mut t.cloud_col);
        drop_rows(&mut t.cloud_nrm);
        drop_rows(&mut t.cloud_draws);
        drop_rows(&mut t.cloud_nodes);
        // `objects`, `object_bounds` and `object_spacing` STAY: they are per-object rows the
        // instance table is rebased from every time the camera re-anchors, and the walk indexes
        // them by global row - they are the one table the GPU is not the only holder of.
```

**Replace with:**

```rust
        self.tables.drop_uploaded();
```

**Find** in `src/app/scene.rs`:

```rust
/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud_pos`
/// holds exactly as much wasm heap as a full one.
fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}

/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
```

**Replace with:**

```rust
/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
```

The other end — the one place a moved body is retyped rather than cut: `t.verts` becomes
`self.verts` fourteen times, and `Upload::new`'s closing brace becomes `impl Upload`'s.

**Find** in `src/engine/gpu/upload.rs`:

```rust
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        }
    }
}
```

**Replace with:**

```rust
            min: [f32::INFINITY; 3],
            max: [f32::NEG_INFINITY; 3],
        }
    }

    /// Hand every UPLOADED column's allocation back - the GPU is their only holder now.
    ///
    /// Fourteen of the nineteen columns; `objects`, `object_bounds` and `object_spacing` stay,
    /// and `min`/`max` are not rows.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.segments);
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.glyphs);
        drop_rows(&mut self.cloud_pos);
        drop_rows(&mut self.cloud_col);
        drop_rows(&mut self.cloud_nrm);
        drop_rows(&mut self.cloud_draws);
        drop_rows(&mut self.cloud_nodes);
        // `objects`, `object_bounds` and `object_spacing` STAY: they are per-object rows the
        // instance table is rebased from every time the camera re-anchors, and the walk indexes
        // them by global row - they are the one table the GPU is not the only holder of.
    }
}

/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud_pos`
/// holds exactly as much wasm heap as a full one.
fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
```

`upload.rs` so far:

```text
  1- 10  //! header — the app/engine line
 12- 15  use Mat4 · RenderVertex · CloudDraw · CylinderSegment · GlyphPoint · LodNode
 17- 52  Upload — 19 columns
 54-102  impl Upload — `new`, then `drop_uploaded` and its 14 columns
104-110  drop_rows
```

Gate — `check_lean` is the test that would notice if a column stopped being dropped:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -c 'wgpu::' src/engine/gpu/upload.rs      # 0
./docs/_gate.sh
```

### 6.4 `view.rs` — seven knobs become one field

A **knob is not a uniform.** `show_points` gates a draw; `line_style` picks a pipeline; neither is
written into a buffer and no `.wgsl` has seen one. `show_points`, `show_lines` and `show_mesh_edges`
were typed after the plan was measured, which is why `View` takes seven and not the four budgeted.

The four with doc comments are a contiguous block in `Gpu`, so they are cut whole:

**Move** `src/engine/gpu/mod.rs`

```rust
    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
```

**through** `    pub show_mesh_edges: bool,` **to** `src/engine/gpu/view.rs` **after** `pub struct View {`

Stitch — the paste leaves a blank line under the struct head:

**Find** in `src/engine/gpu/view.rs`:

```rust
pub struct View {

    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
```

**Replace with:**

```rust
pub struct View {
    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
```

Their four startup values are contiguous too, in the `Gpu` struct literal at the bottom of
`build`, and land in `from_env` at the same indentation:

**Move** `src/engine/gpu/mod.rs` `            show_points: true,` **through** `            },` **to** `src/engine/gpu/view.rs` **after** `        Self {`

Stitch:

**Find** in `src/engine/gpu/view.rs`:

```rust
        Self {

            show_points: true,
```

**Replace with:**

```rust
        Self {
            show_points: true,
```

Now the `Gpu` side. The end-state head is `surface, ctx, config, layouts, pipelines, frame,
targets, view`, and each step from here inserts its own field into it:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub pipelines: Pipelines,
```

**Add below it:**

```rust
    /// The runtime knobs - what the keyboard toggles (`view.rs`). A knob gates a draw or
    /// picks a pipeline; it is never a uniform, and no `.wgsl` file ever sees one.
    pub view: View,
```

The other three knobs are single lines wedged between unrelated fields, so each is cut against
the line above it:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cloud_buffer: wgpu::Buffer,
    pub cloud_size: f32, // global SCALE on per-cloud sizes, [ and ] keys
```

**Replace with:**

```rust
    pub cloud_buffer: wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
```

**Replace with:**

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
    pub lod_split_px: f32, // octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (VIEWER_LOD)
```

**Replace with:**

```rust
    last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
```

Then the struct literal. Three `std::env::var` reads leave `Gpu::build` for `View::from_env`, the
only expression this lesson relocates rather than copies:

**Find** in `src/engine/gpu/mod.rs`:

```rust
            point_count,
            cloud_buffer,
            cloud_size: std::env::var("VIEWER_CLOUD_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            last_rebase_ms: 0.0,
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
            last_ortho_h: 0.0,
            last_eye: [0.0; 3],
            lod_split_px: std::env::var("VIEWER_LOD").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            cloud_bind_group,
```

**Replace with:**

```rust
            point_count,
            view: View::from_env(),
            cloud_buffer,
            last_rebase_ms: 0.0,
            last_ortho_h: 0.0,
            last_eye: [0.0; 3],
            cloud_bind_group,
```

Seven readers inside the frame encoder, one substitution each. The counts are the map of who
actually reads a knob — two draw gates per lane, one pipeline choice, three cloud sizes:

**Replace-all** `src/engine/gpu/mod.rs` `self.show_points` → `self.view.show_points` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `self.show_lines` → `self.view.show_lines` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `self.show_mesh_edges` → `self.view.show_mesh_edges` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `self.line_style` → `self.view.line_style` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `self.cloud_size` → `self.view.cloud_size` (3 hits)

**Replace-all** `src/engine/gpu/mod.rs` `self.edl_strength` → `self.view.edl_strength` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `self.lod_split_px` → `self.view.lod_split_px` (2 hits)

The key handler in `lib.rs` is the only writer, and the headless harness reads two:

**Replace-all** `src/lib.rs` `state.gpu.show_points` → `state.gpu.view.show_points` (3 hits)

**Replace-all** `src/lib.rs` `state.gpu.show_lines` → `state.gpu.view.show_lines` (3 hits)

**Replace-all** `src/lib.rs` `state.gpu.show_mesh_edges` → `state.gpu.view.show_mesh_edges` (3 hits)

**Replace-all** `src/lib.rs` `state.gpu.line_style` → `state.gpu.view.line_style` (3 hits)

**Replace-all** `src/lib.rs` `state.gpu.cloud_size` → `state.gpu.view.cloud_size` (6 hits)

**Replace-all** `src/selftest.rs` `gpu.cloud_size` → `gpu.view.cloud_size` (1 hit)

**Replace-all** `src/selftest.rs` `gpu.line_style` → `gpu.view.line_style` (1 hit)

`view.rs` so far:

```text
  1-  7  //! header — a knob gates a draw, it is never a uniform
  9      use super::LineStyle
 11- 29  View — line_style · show_points · show_lines · show_mesh_edges
         · cloud_size · edl_strength · lod_split_px
 31- 50  View::from_env — the four defaults and the three VIEWER_* reads
```

Gate. `VIEWER_LINE_STYLE=tubes` is the config that proves `line_style` still arrives from the
environment, and `_gate.sh` runs it on every scene:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
./docs/_gate.sh
```

### 6.5 `frame.rs` — what every shader reads, once per frame

Three uniform blocks belong to no family: the camera matrix every vertex shader reads at group 0,
the pen block five shaders read at group 1, and the cloud block the splat pair reads. Twelve `Gpu`
fields carry them, and one function writes all of them.

Start with that function's body. `write_camera` and `write_cloud` are the two halves of one
24-line block; only the last line of `write_frame_uniforms` (`update_inside_flags`) stays behind.

**Move** `src/engine/gpu/mod.rs` `        // Time for triangle wgsl buffer.` **through** `        }));` **to** `src/engine/gpu/frame.rs` **at the end**

Stitch, and open `write_camera` over it:

**Find** in `src/engine/gpu/frame.rs`:

```rust
impl FrameUniforms {
}

        // Time for triangle wgsl buffer.
```

**Replace with:**

```rust
impl FrameUniforms {
    /// Per-frame uniforms: time, camera, and the line/pen block.
    pub(crate) fn write_camera(&mut self, ctx: &GpuCtx, f: &FrameInput<'_>) {
        // Time for triangle wgsl buffer.
```

Split the two halves at the line where one uniform ends and the next begins:

**Find** in `src/engine/gpu/frame.rs`:

```rust
        self.ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
        self.ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&CloudUniform{
```

**Replace with:**

```rust
        self.ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
    }

    /// The cloud block: the global point-size scale and the EDL strength the resolve pass reads.
    pub(crate) fn write_cloud(&self, ctx: &GpuCtx, f: &FrameInput<'_>) {
        self.ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&CloudUniform{
```

Close it, and add the one genuinely new function here. `binds` is not a convenience: it is the
only way groups 0-2 can be read inside a render pass — six shared reborrows taken before it opens.

**Find** in `src/engine/gpu/frame.rs`:

```rust
            _pad: self.view.edl_strength, // EDL strength, read by the splat resolve
        }));
```

**Add below it:**

```rust
    }

    /// Groups 0-2 for this frame. `instances` is the object table's bind group, which lives
    /// outside this file - `Gpu` hands it in rather than `frame` reaching for it.
    pub(crate) fn binds<'a>(&'a self, p: &'a Pipelines, instances: &'a wgpu::BindGroup) -> Binds<'a> {
        Binds {
            p,
            mvp: &self.mvp_bind_group,
            time: &self.time_bind_group,
            line: &self.line_bind_group,
            cloud: &self.cloud_bind_group,
            instances,
        }
    }
}
```

Now the re-roots — the payoff of a struct that owns exactly the right fields. `self.time_buffer`,
`self.mvp_buffer`, `self.line_buffer`, `self.mvp_f32`, `self.last_ortho_h` and `self.last_eye`
**do not change at all**: `self` is a `FrameUniforms` now. Only what came from outside moves:

**Replace-all** `src/engine/gpu/frame.rs` `self.ctx.queue.write_buffer(&self.time_buffer` → `ctx.queue.write_buffer(&self.time_buffer` (1 hit)

**Replace-all** `src/engine/gpu/frame.rs` `self.ctx.queue.write_buffer(&self.mvp_buffer` → `ctx.queue.write_buffer(&self.mvp_buffer` (1 hit)

**Replace-all** `src/engine/gpu/frame.rs` `self.ctx.queue.write_buffer(&self.line_buffer` → `ctx.queue.write_buffer(&self.line_buffer` (1 hit)

**Replace-all** `src/engine/gpu/frame.rs` `self.ctx.queue.write_buffer(&self.cloud_buffer` → `ctx.queue.write_buffer(&self.cloud_buffer` (1 hit)

**Find** in `src/engine/gpu/frame.rs`:

```rust
        self.mvp_f32 = view_proj.to_f32();
        self.last_ortho_h = ortho_half_height(view_proj);
        self.last_eye = eye_from_view_proj(view_proj);
```

**Replace with:**

```rust
        self.mvp_f32 = f.view_proj.to_f32();
        self.last_ortho_h = ortho_half_height(f.view_proj);
        self.last_eye = eye_from_view_proj(f.view_proj);
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
            ortho_h: ortho_half_height(view_proj),
```

**Replace with:**

```rust
            ortho_h: ortho_half_height(f.view_proj),
```

The fourth read is `view_proj`, deliberately NOT a Replace-all: `FrameInput` has a field of that
name, so `view_proj` → `f.view_proj` matches five times, the fifth being the declaration. A count
one higher than you expected has found a name you did not mean — check before you widen it.

**Replace-all** `src/engine/gpu/frame.rs` `vp_h: self.config.height` → `vp_h: f.config.height` (2 hits)

**Replace-all** `src/engine/gpu/frame.rs` `vp_w: self.config.width` → `vp_w: f.config.width` (2 hits)

**Replace-all** `src/engine/gpu/frame.rs` `self.last_origin.as_ref()` → `f.anchor` (1 hit)

**Replace-all** `src/engine/gpu/frame.rs` `size: self.view.cloud_size,` → `size: f.view.cloud_size,` (1 hit)

**Replace-all** `src/engine/gpu/frame.rs` `_pad: self.view.edl_strength,` → `_pad: f.view.edl_strength,` (1 hit)

Four reads, four fields on `FrameInput` — that is the seam A3 asked for, and it is why
`write_camera` takes three parameters and not six.

Next the two `#[repr(C)]` blocks the uniforms are written from. `Gpu::build` still fills each with
its startup values, so the struct and every field become `pub(crate)`. The move takes the struct
and the size assert; the two attribute lines above are retyped on arrival and deleted at source.

**Move** `src/engine/gpu/mod.rs` `struct LineUniform{` **through** `const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);` **to** `src/engine/gpu/frame.rs` **at the end**

**Find** in `src/engine/gpu/frame.rs`:

```rust
struct LineUniform{
```

**Add above it:**

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
```

**Replace with:**

```rust
pub(crate) struct LineUniform{
    pub(crate) thickness: f32, // on-screwwn width, px
    pub(crate) proj_y: f32, // vertical projection scale x unit scale
    pub(crate) ortho_h: f32, // ortho world half.heigh x unit scale
    pub(crate) vp_h: f32, // framebuffer height, px
    pub(crate) vp_w: f32, // framebuffer width, px - flat linework needs the aspect
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
    eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
```

**Replace with:**

```rust
    pub(crate) eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
```

**Find** in `src/engine/gpu/frame.rs`:

```rust
    anchor: [f32; 3],
    _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
```

**Replace with:**

```rust
    pub(crate) anchor: [f32; 3],
    pub(crate) _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
}                       // 40 B

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]

// One instance of the unit-sphere template.
```

**Replace with:**

```rust
}                       // 40 B

// One instance of the unit-sphere template.
```

`CloudUniform` is the same shape, three lines of preamble instead of two:

**Move** `src/engine/gpu/mod.rs` `struct CloudUniform{` **through** `} // 16 B - one vec4; its own buffer + bind group` **to** `src/engine/gpu/frame.rs` **at the end**

**Find** in `src/engine/gpu/frame.rs`:

```rust
struct CloudUniform{
    size: f32, // global point-cloud size SCALE ([ and ] keys)
    vp_w: f32, // framebuffer width, px
    vp_h: f32, // framebuffer height, px
    _pad: f32,
```

**Replace with:**

```rust
// Points global attributes
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CloudUniform{
    pub(crate) size: f32, // global point-cloud size SCALE ([ and ] keys)
    pub(crate) vp_w: f32, // framebuffer width, px
    pub(crate) vp_h: f32, // framebuffer height, px
    pub(crate) _pad: f32,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

// Points global attributes
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]

//////////////////////////////////////////////////////////////////////////////////////////////////
```

**Replace with:**

```rust
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

//////////////////////////////////////////////////////////////////////////////////////////////////
```

And the pen width, which is read by exactly one line of `write_camera` and by nothing else:

**Move** `src/engine/gpu/mod.rs` `/// On-screen pen weight in px. Default 2.0.` **through** `}` **to** `src/engine/gpu/frame.rs` **at the end**

That was the last item in the file, and `zeroed_buffer` left the same tail in 6.1, so `gpu/mod.rs`
now ends on three blank lines. The Find block below ends with **three** blank lines inside the
fence and the replacement with **one** — the one edit here where trailing whitespace is the edit:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    (v, idx)
}



```

**Replace with:**

```rust
    (v, idx)
}

```

Back in `gpu/mod.rs`. `write_frame_uniforms` keeps its name and its one caller-visible job, and
becomes three lines — the second of which is the borrow rule from §5 in its mildest form:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Per-frame uniforms: time, camera, and the line/pen block.
    fn write_frame_uniforms(&mut self, view_proj: &Xform) {
        self.update_inside_flags(view_proj);
    }
```

**Replace with:**

```rust
    /// Per-frame uniforms, then the per-object flags they feed.
    ///
    /// B1: `f` borrows the FIELDS `config`, `last_origin` and `view`, and `write_camera`
    /// borrows the FIELD `frame` - four disjoint places, so this compiles. The same body
    /// written as `self.write_camera(&f)` does not: a method borrows ALL of `self`.
    fn write_frame_uniforms(&mut self, view_proj: &Xform) {
        let f = FrameInput { config: &self.config, view_proj, anchor: self.last_origin.as_ref(), view: &self.view };
        self.frame.write_camera(&self.ctx, &f);
        self.frame.write_cloud(&self.ctx, &f);
        self.update_inside_flags(view_proj);
    }
```

Groups 0-2 are taken once, before the pass opens:

**Find** in `src/engine/gpu/mod.rs`:

```rust
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

**Replace with:**

```rust
        {
            // Groups 0-2 for this frame, all shared, taken BEFORE the pass opens (B3).
            let b = self.frame.binds(&self.pipelines, &self.instance_bind_group);
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

Then the draw sites. Forty-two of them, six substitutions, and the counts are worth reading as a
census of the frame: fifteen pipeline switches, nine cameras, seven pens, eight object tables,
two clocks and one cloud block.

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_pipeline(&self.pipelines` → `pass.set_pipeline(&b.p` (15 hits)

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_bind_group(0, &self.mvp_bind_group, &[]);` → `pass.set_bind_group(0, b.mvp, &[]);` (9 hits)

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_bind_group(1, &self.line_bind_group, &[]);` → `pass.set_bind_group(1, b.line, &[]);` (7 hits)

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_bind_group(1, &self.time_bind_group, &[]);` → `pass.set_bind_group(1, b.time, &[]);` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_bind_group(2, &self.instance_bind_group, &[]);` → `pass.set_bind_group(2, b.instances, &[]);` (8 hits)

**Replace-all** `src/engine/gpu/mod.rs` `pass.set_bind_group(0, &self.cloud_bind_group, &[]);` → `pass.set_bind_group(0, b.cloud, &[]);` (1 hit)

Five reads survive outside the pass — the splat compute's bind groups and the record encoder,
which run before it:

**Replace-all** `src/engine/gpu/mod.rs` `&self.mvp_buffer, &self.cloud_buffer,` → `&self.frame.mvp_buffer, &self.frame.cloud_buffer,` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `(&self.mvp_f32, &row.model)` → `(&self.frame.mvp_f32, &row.model)` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `let state = (self.mvp_f32,` → `let state = (self.frame.mvp_f32,` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `let ortho_h = self.last_ortho_h` → `let ortho_h = self.frame.last_ortho_h` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `let eye = self.last_eye;` → `let eye = self.frame.last_eye;` (1 hit)

Each key is longer than the field it re-roots, deliberately: `self.mvp_buffer` alone would also
match the line that just moved into `frame.rs`, where the path is already right. A `Replace-all`
scopes a substitution to a file; how much of the line you put in the key scopes it to a site.

The field list changes last. Seven fields out of the head, one in:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub pipelines: Pipelines,
```

**Add below it:**

```rust
    pub frame: FrameUniforms,                // camera + pen + cloud uniforms (frame.rs)
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub mvp_buffer: wgpu::Buffer,            // Camera matrix
    pub mvp_bind_group: wgpu::BindGroup,     // Camera matrix
    pub line_buffer: wgpu::Buffer, // shared: px-sizing for cylinders + spheres
    pub line_bind_group: wgpu::BindGroup,
    pub time: f32,  // shared: animation
    pub time_buffer: wgpu::Buffer,
    pub time_bind_group: wgpu::BindGroup,
    pub arena_vbo: wgpu::Buffer,
```

**Replace with:**

```rust
    pub arena_vbo: wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers were build for; None = stale
    mvp_f32: [f32; 16],
```

**Replace with:**

```rust
    splat_state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers were build for; None = stale
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_group1_stream: wgpu::BindGroup,
    pub cloud_buffer: wgpu::Buffer,
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    last_ortho_h: f32, // ortho half-height this frame (0=perspective), for the plat k
    last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
    pub cloud_bind_group: wgpu::BindGroup,
```

**Replace with:**

```rust
    splat_group1_stream: wgpu::BindGroup,
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
```

Then the struct literal in `build`. The locals keep their names, so this is a brace and an indent:

**Find** in `src/engine/gpu/mod.rs`:

```rust
            mvp_buffer, // shared: camera
            mvp_bind_group,
            line_buffer,  // shared: px-sizing for cylinders + spheres
            line_bind_group,
            time_buffer,    // shared: animation
            time_bind_group,
            time: 0.0,
```

**Replace with:**

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

**Find** in `src/engine/gpu/mod.rs`:

```rust
            mvp_f32: [0.0; 16],
            cloud_draws: Vec::new(),
```

**Replace with:**

```rust
            cloud_draws: Vec::new(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            view: View::from_env(),
            cloud_buffer,
            last_rebase_ms: 0.0,
            last_ortho_h: 0.0,
            last_eye: [0.0; 3],
            cloud_bind_group,
```

**Replace with:**

```rust
            view: View::from_env(),
            last_rebase_ms: 0.0,
```

Last, the import. `Gpu::build` still writes both uniform literals, `write_frame_uniforms` builds
a `FrameInput`, and the struct carries a `FrameUniforms` — four names:

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use upload::Upload;
```

**Add below it:**

```rust
use frame::{CloudUniform, FrameInput, FrameUniforms, LineUniform};
```

`frame.rs` so far:

```text
  1-  8  //! header — the three blocks that are no family's
 10- 15  use Point · Xform · Pipelines · GpuCtx · View · the two camera solves
 17- 31  FrameUniforms — 12 fields, all `pub(super)`
 33- 43  FrameInput<'a> — config · view_proj · anchor · view
 45- 57  Binds<'a> — p · mvp · time · line · cloud · instances, all shared
 59-105  write_camera · write_cloud · binds
107-126  LineUniform
128-142  its 48 B assert, then CloudUniform
144-177  line_thickness_px
```

Gate. The step with the most re-rooted call sites in the block, so compile both targets first:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -c 'self.mvp_bind_group\|self.line_bind_group\|self.time_bind_group\|self.cloud_bind_group' src/engine/gpu/mod.rs
./docs/_gate.sh
```

The grep prints `0`: every group-0-to-2 read in the frame now goes through `Binds`.

### 6.6 `targets.rs` — what a pass writes into

`targets.rs` already exists, whole, from §4.5. This step is three deletions and five call sites.

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use upload::Upload;
```

**Add below it:**

```rust
use targets::Targets;
```

The two view builders were the last two functions in `impl Gpu`; `Targets::new` is both of them
with the textures kept:

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if solid { 4 } else { 1 }
    }

    /// Create the reverse-Z depth texture view, sized to the surface at the MSAA sample count.
    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView{
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// Create the multisampled color target the frame renders into (resolved to the surface each frame).
    fn create_msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d{ width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1},
            mip_level_count: 1,
            sample_count: samples,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}
```

**Replace with:**

```rust
        if solid { 4 } else { 1 }
    }
}
```

The pass descriptor. Twenty-four lines of `encode_frame` become one call, and the two `LoadOp`s
that were literals inside it become the two parameters lesson 74's overlay pass needs:

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                // MSAA off (samples == 1): draw straight to the swapchain view - a
                // 1-sample attachment must NOT carry a resolve target.
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: if self.samples > 1 { &self.msaa_view } else { view },
                    resolve_target: if self.samples > 1 { Some(view) } else { None },
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(
                        wgpu::Operations{load: wgpu::LoadOp::Clear(0.0),
                        store:wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
```

**Replace with:**

```rust
            let mut pass = self.targets.begin_pass(encoder, view, wgpu::LoadOp::Clear(color),
                                                  Some(wgpu::LoadOp::Clear(0.0)));
```

Three call sites rebuild the attachments — startup, the MSAA flip, a window resize — and each
becomes a single `Targets::new`, because the sample count now lives with the textures:

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let depth_view = Self::create_depth_view(&device, &config, samples);
        let msaa_view = Self::create_msaa_view(&device, &config, samples);
```

**Replace with:**

```rust
        let targets = Targets::new(&device, &config, samples);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if samples != self.samples {
            self.samples = samples;
            self.depth_view = Self::create_depth_view(&self.ctx.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.ctx.device, &self.config, samples);
```

**Replace with:**

```rust
        if samples != self.targets.samples {
            self.targets = Targets::new(&self.ctx.device, &self.config, samples);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.depth_view = Self::create_depth_view(&self.ctx.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.ctx.device, &self.config, self.samples);
```

**Replace with:**

```rust
            self.targets = Targets::new(&self.ctx.device, &self.config, self.targets.samples);
```

Fields last, three out and one in:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub frame: FrameUniforms,                // camera + pen + cloud uniforms (frame.rs)
```

**Add below it:**

```rust
    pub targets: Targets,                    // depth + MSAA attachments (targets.rs)
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    pub samples: u32, // MSAA sample count this scene chose (see `msaa_for`)
    pub performance: Performance,
```

**Replace with:**

```rust
    pub performance: Performance,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            depth_view,
            msaa_view,
            samples,
            performance: Performance::new(),
```

**Replace with:**

```rust
            targets,
            performance: Performance::new(),
```

Gate — the MSAA flip is the one path a golden alone would not exercise from a cold start, so run
the mixed scene as well as the default set:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -c 'begin_render_pass' src/engine/gpu/mod.rs      # 0
./docs/_gate.sh
```

### 6.7 `present.rs` — how a frame leaves

Three methods on `Gpu` are about getting an encoded frame OUT: to the swapchain, to a readback
buffer, or round a timing loop. None of them encodes anything — all three call `encode_frame`,
which stays in `mod.rs`. Move them first, then split the first one.

**Move** `src/engine/gpu/mod.rs`

```rust
    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
```

**through** `    }` **to** `src/engine/gpu/present.rs` **at the end**

**Move** `src/engine/gpu/mod.rs`

```rust
    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
```

**through** `    }` **to** `src/engine/gpu/present.rs` **at the end**

**Move** `src/engine/gpu/mod.rs`

```rust
    /// Time `frames` full frames (encode + submit), reusing one offscreen target, and wait for
```

**through** `    }` **to** `src/engine/gpu/present.rs` **at the end**

Three cuts leave four blank lines behind:

**Find** in `src/engine/gpu/mod.rs`:

```rust
    }




    /// Per-frame uniforms, then the per-object flags they feed.
```

**Replace with:**

```rust
    }

    /// Per-frame uniforms, then the per-object flags they feed.
```

Stitch the first one into the impl:

**Find** in `src/engine/gpu/present.rs`:

```rust
impl Gpu {
}

    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
```

**Replace with:**

```rust
impl Gpu {
    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
```

Now the split. `clear` writes the uniforms, acquires a swapchain texture, encodes the frame, then
submits and presents. A caller that wants its OWN pass — an egui overlay at 69, a gumball at 74 —
must get in between the third step and the fourth, and cannot. So `clear`'s body becomes
`begin_present`, and `clear` comes back as the two-line composition of the halves.

The `Option` is not cosmetic: this body has **two** early returns meaning "there is no frame" — a
headless `Gpu` with no surface, and a lost surface just reconfigured — and `Frame.view` has to
hold a real `TextureView`.

**Find** in `src/engine/gpu/present.rs`:

```rust
    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
    /// (mesh, line, grid, …) get added here in later chapters.
    /// Draw one frame to the swapchain. The frame ENCODING lives in `encode_frame` so a
    /// headless harness can aim the same code at an offscreen texture and read the pixels back -
    /// see `selftest.rs`. Shader work that is only ever checked in a browser is shader work
    /// checked by somebody else's eyes.
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {
```

**Replace with:**

```rust
    /// Acquire the next swapchain frame and encode the scene into it.
    ///
    /// `None` means there is nothing to present, for one of exactly two reasons: this `Gpu` is
    /// headless, or the surface was lost and has just been reconfigured. Both used to be an
    /// early `return Ok(())` inside `clear`, which is why the split needs an `Option` and not
    /// just a `Frame`.
    pub fn begin_present(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<Option<Frame>> {
```

**Find** in `src/engine/gpu/present.rs`:

```rust
        let Some(surface) = &self.surface else { return Ok(()) }; // headless: nothing to present
```

**Replace with:**

```rust
        let Some(surface) = &self.surface else { return Ok(None) }; // headless: nothing to present
```

**Replace-all** `src/engine/gpu/present.rs` `return Ok(()); }` → `return Ok(None); }` (1 hit)

**Find** in `src/engine/gpu/present.rs`:

```rust
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
    }
```

**Replace with:**

```rust
        let (draws, objects) = self.encode_frame(&mut encoder, &view, color);
        self.performance.frame(draws, objects);
        Ok(Some(Frame { surface: Some(output), view, encoder }))
    }

    /// Submit the frame's commands and hand the image to the compositor.
    pub fn end_present(&mut self, f: Frame) {
        self.ctx.queue.submit([f.encoder.finish()]);
        if let Some(o) = f.surface { o.present(); }
    }

    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
    /// (mesh, line, grid, …) get added here in later chapters.
    /// Draw one frame to the swapchain. The frame ENCODING lives in `encode_frame` so a
    /// headless harness can aim the same code at an offscreen texture and read the pixels back -
    /// see `selftest.rs`. Shader work that is only ever checked in a browser is shader work
    /// checked by somebody else's eyes.
    ///
    /// Written as the literal composition of the two halves, so its behaviour cannot drift
    /// away from theirs. The 2-argument signature is what `state.rs` calls.
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform) -> anyhow::Result<()> {
        if let Some(f) = self.begin_present(color, view_proj)? { self.end_present(f); }
        Ok(())
    }
```

The impl is still open — the stitch ate its closing brace — so close it after the last method:

**Find** in `src/engine/gpu/present.rs`:

```rust
        t0.elapsed().as_secs_f64()
    }
```

**Add below it:**

```rust
}
```

`present.rs` so far:

```text
  1- 10  //! header — the three ways a frame leaves the encoder
 12- 14  use Xform · Gpu
 16- 25  Frame { surface, view, encoder }
 27-162  impl Gpu — begin_present · end_present · clear = begin + end
         · render_offscreen and bench_frames, both native only
```

Gate. Nothing in the goldens calls `clear`, `begin_present` or `end_present` — the harness goes
through `render_offscreen` — so the compiler is the only automatic check here, and §7's browser
smoke test is the other:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
./docs/_gate.sh
```

The field count prints **86**.

## 7. Proving nothing changed — four ladders

**Ladder 1, the compiler.** Both targets, and `--all-targets` natively so the examples and
the headless harness are type-checked too:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
```

*What it cannot catch:* anything that type-checks. `b.line` where the old line read
`&self.time_bind_group` compiles perfectly — both are `&wgpu::BindGroup` — and so does a
`Targets::new` that forgot `TEXTURE_BINDING`. Nor can it see a `#[cfg]`-gated arm on the target
you did not build, and `render_offscreen` and `bench_frames` are both behind one.

**Ladder 2, `--moves`**, introduced in lesson 45 §7: the only proof a move took its lines
byte-identically.

```bash
python3 docs/_replay_check.py --moves <end-of-45 snapshot> /tmp/w46 docs/46-gpu-floor.md
```

```text
docs/46-gpu-floor.md: 143 ops, 0 failed
docs/46-gpu-floor.md: 1 move source(s), 0 not byte-identical
   ... lost-declared src/engine/gpu/mod.rs (over …) — 105 line(s)
```

The third line is informational, marked `...` not `!!`: those 105 lines left `gpu/mod.rs` and the
doc spells each one out in a `Find` block first. `0 not byte-identical` is the verdict.

All fourteen moves start in `src/engine/gpu/mod.rs` and land in five files. That is what makes the
ladder usable: two sources into one destination would report the other source's lines as an
undeclared gain, so the sweep out of `app/scene.rs` in 6.3 is a `Replace`, not a move.

*What it catches and the other three do not:* a line dropped inside a
`#[cfg(not(target_arch = "wasm32"))]` arm. `render_offscreen` is 52 such lines, it is what the
pixel gate itself runs, and it is invisible to `cargo check --target wasm32-unknown-unknown`.

**Ladder 3, the pixel gate, twice.**

```bash
./docs/_gate.sh && ./docs/_gate.sh
```

The same 64 rows lesson 45 §7 describes: four mandatory scenes × four configs × two passes, plus
four advisory scenes when their gitignored `.pb` assets are present, every row gated on **ink,
draw count and object count**, with the `nondet(splat)` and `nondet(mesh)` exemptions recorded in
`_GOLDENS.tsv`. Neither exemption is your bug.

*What it cannot catch:* everything `State` owns. The harness calls `render_offscreen` and never
constructs a `State`, so **`clear`, `begin_present` and `end_present` are compiler-gated only** —
and this lesson rewrote all three. That is what ladder 4 is for, and it is not optional here.

**Ladder 4, the browser, once.** The only run in this lesson that exercises the swapchain path:

```bash
trunk serve --release
```

Open `http://127.0.0.1:8080`, and read the console in this order:

```text
adapter: <your GPU> (<type>, <backend>)
viewer init OK — surface <w>x<h>, format <format>
scene: <n> objects <n> arena verts <n> segments (<n> pipes) <n> glyphs (<n> spheres) <n> cloud points
```

Three lines in that order, and then **no** `wgpu on_uncaptured_error` line, ever — a binding whose
size no longer matches its layout reports in that device error scope and nowhere else. Then, with
the scene up: orbit, pan and zoom (the `clear` path); view keys **1**-**7** (`write_camera`); **E**
(a knob gating a draw); **L** (a knob picking a pipeline); **[** and **]** (a knob the cloud block
reads); and **resize the window**, the only gesture that runs `Targets::new` after startup. That
exercises `present.rs`, `view.rs`, `frame.rs` and `targets.rs` on a real surface.

## 8. What you can now do in one line

Add a uniform the whole flat ink lane reads. Before this lesson: the struct near the bottom of a
2,139-line file, the write that fills it 500 lines above, the buffer 800 lines above THAT, and a
guess at which shaders declare the block. Now all of it is one 177-line file.

`LineUniform` ends in `_pad1`, four bytes that exist only because WGSL rounds the block up to 48.
Take them.

**Type all ten steps below.** The first five add the uniform, the last five take it back out — a
demonstration, not part of the end state, and `frame.rs`, `gpu/mod.rs` and `ribbon.wgsl` must be
back to what §6 left before §10. Do **not** undo it with `git checkout`: lesson 46 is not
committed, and that would throw it all away.

**8a.** The field. **Find** in `src/engine/gpu/frame.rs`:

```rust
    pub(crate) _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
```

**Replace with:**

```rust
    pub(crate) tint: f32, // 4 B - was _pad1; the block is still 48 B
```

**8b.** The writer. **Find** in `src/engine/gpu/frame.rs`:

```rust
            anchor: f.anchor.map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            _pad1: 0.0,
```

**Replace with:**

```rust
            anchor: f.anchor.map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            tint: 0.35,
```

**8c.** The zeroed copy the first frame uploads. **Find** in `src/engine/gpu/mod.rs`:

```rust
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
```

**Replace with:**

```rust
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                tint: 0.0,
```

**8d.** The other side of the contract. `frame.rs`'s header names the five shaders that declare
`LineUniform`; this one is the flat ribbon lane. **Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**Add below it:**

```wgsl
    tint: f32,           // offset 44, the four bytes after anchor's vec3 - still 48 B
```

**8e.** **Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
```

**Replace with:**

```wgsl
    return vec4<f32>(mix(in.color.rgb, vec3<f32>(1.0, 0.0, 0.0), line.tint), in.color.a * alpha);
```

Render the sheet scene, which is flat ink end to end:

```bash
cargo run -q --example selftest --target x86_64-unknown-linux-gnu --release -- \
    /tmp/tint.ppm assets/scenes/drawings_rotated.toml
```

Most of the sheet's ink is now 35% of the way to red — reddish pixels go from 976 to 3,322. The
ink COUNT barely moves: a colour change moves no geometry. But the frame is a different frame, and
`drawings_rotated` is the one scene a checksum gates, so take it back out before §10. Three lines
of Rust across two files, two lines of WGSL in one shader, and nothing else in the program knew.

**8f.** **Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    return vec4<f32>(mix(in.color.rgb, vec3<f32>(1.0, 0.0, 0.0), line.tint), in.color.a * alpha);
```

**Replace with:**

```wgsl
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
```

**8g.** Both lines together, because a delete verb would leave a blank line behind. **Find** in
`src/shaders/ribbon.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
    tint: f32,           // offset 44, the four bytes after anchor's vec3 - still 48 B
```

**Replace with:**

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**8h.** **Find** in `src/engine/gpu/mod.rs`:

```rust
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                tint: 0.0,
```

**Replace with:**

```rust
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
```

**8i.** **Find** in `src/engine/gpu/frame.rs`:

```rust
            anchor: f.anchor.map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            tint: 0.35,
```

**Replace with:**

```rust
            anchor: f.anchor.map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]),
            _pad1: 0.0,
```

**8j.** **Find** in `src/engine/gpu/frame.rs`:

```rust
    pub(crate) tint: f32, // 4 B - was _pad1; the block is still 48 B
```

**Replace with:**

```rust
    pub(crate) _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
```

The point is the size of the diff and the number of files you had to open: the uniform, its
writer, its `FrameInput` and the list of shaders that mirror it were all on one screen.

## 9. What is deliberately not here

- **`RowTable<T>`.** Each family folds its own `(buffer, count, cap)` triple in as it is created —
  `Arena` at **47**, `SegmentLane`/`GlyphLane` at **48**, cloud and stream at **49**. The
  CPU-mirror-plus-`guid`-map version lands at **57**, with its first honest caller.
- **`Upload` regrouped per family.** It moved FLAT. `obj`/`arena` are **47**, `seg`/`glyph` **48**,
  `cloud`/`span` **49** — one instalment per consumer, never one 170-site table.
- **`upload_rows(ctx, &[u32])`**, the flip-tracked partial upload of the object table: **62**,
  where frustum culling makes per-row updates worth the bookkeeping.
- **`Targets::new_sized(ctx, size, format, samples)`** and the half-res effect targets: **107** and
  **88**/**90**. `Targets` is sized from `config` today because that is all any caller needs.
- **`render_to_texture(size, view_proj)`** — `render_offscreen` still hardcodes the surface size
  and the surface format: **107**.
- **The `line_uniform_mirror` test**, asserting the Rust and WGSL `LineUniform` agree field by
  field: **47**, beside `instance_mirror`, where the `Instance` row moves.
- **Splitting `encode_frame`** into three fenced regions plus a twelve-line `scene_list`: **49**.
  This lesson took the pass descriptor and the bind groups out and left the draw order alone.
- **The three `#[allow(dead_code)]`s.** `GrowBuf` comes off at 47, `Targets.depth` at 88,
  `Frame.view` at 69 — each a field only the named lesson uses; the attribute is the receipt.
- **Fixing anything a move carried.** `append_rows`'s doc still describes the index run below it,
  `ArenaUpload`'s still says "owened", and lesson 45's `glyph`/`segment` layout alias is still
  there. A body you are moving is not a body you are fixing.

## 10. Expected state

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
python3 docs/_replay_check.py --moves <end-of-45 snapshot> /tmp/w46 docs/46-gpu-floor.md
./docs/_gate.sh && ./docs/_gate.sh
```

Both gate runs print `gate OK`, and `--moves` prints `1 move source(s), 0 not byte-identical`.

```bash
wc -l src/engine/gpu/*.rs src/app/scene.rs
```

```text
  140 src/engine/gpu/buffers.rs      NEW
  177 src/engine/gpu/frame.rs        NEW
 1691 src/engine/gpu/mod.rs          was 2139
  162 src/engine/gpu/present.rs      NEW
   88 src/engine/gpu/targets.rs      NEW
  110 src/engine/gpu/upload.rs       NEW
   50 src/engine/gpu/view.rs         NEW
 1340 src/app/scene.rs               was 1365
```

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
grep -c 'wgpu::' src/engine/gpu/upload.rs
grep -c 'begin_render_pass' src/engine/gpu/mod.rs
grep -c '&self.device, &self.queue,' src/engine/gpu/mod.rs
```

```text
86   Gpu fields   (was 106)
 0   wgpu:: in upload.rs — it is app-side data
 0   render passes opened outside targets.rs
 0   device/queue pairs left
```

`Gpu` 106 → 86: two handles became one `ctx`, twelve uniform fields one `frame`, three attachments
one `targets`, seven knobs one `view`. The plan predicted 88; the measured tree is 86, because
`mvp_f32`, `last_ortho_h` and `last_eye` went with the uniforms they are computed beside.
`gpu/mod.rs` 2139 → 1691: it still holds `build`, `set_scene`, `encode_frame` and eleven lanes'
worth of flat fields — 47-49 take those — but nothing left in it is plumbing.

## Recap

> **45.** A pipeline is data. Eleven near-identical builders existed because a pipeline was
> modelled as code, and code cannot be spread with `..`. `PipelineDesc` names the eleven settings
> that vary, four presets name the four recipes, and one `build` holds the single
> `create_render_pipeline` call. `Layouts` does the same one level down, and
> `Pipelines::new(device, t, &l)` is frozen at three parameters.
>
> **46.** A buffer, its row count and its capacity are one value; so are `device` and `queue`; so
> are the seven things the keyboard toggles. Forty-three of `Gpu`'s 106 fields spelled
> `(buffer, count, cap)` out longhand, ten call sites passed `&self.device, &self.queue,` as a
> pair, and twelve fields were the three uniform blocks every shader reads. None of that is a lane
> — a lane is a row format and a shader — so it all goes UNDER the lanes, in five files that name
> no row type and no `.wgsl`, plus `upload.rs` on the far side naming no `wgpu::` type. Two shapes
> fall out that 47-51 are built on: `&GpuCtx` handed to a lane instead of held by it, and
> `Binds<'a>`, six shared reborrows taken before the pass opens, because a `RenderPass` borrows
> the encoder for its whole life. **The law: the floor knows no lane. A buffer, its count and its
> cap are one value; a knob is not a uniform; and nothing under the families may name a row, a
> shader or a `Geometry::` variant.**

## Edited

`src/engine/gpu/buffers.rs` (NEW — `GpuCtx`, `GrowBuf`, the four free functions) ·
`src/engine/gpu/upload.rs` (NEW — `Upload` moved flat + `drop_uploaded`) ·
`src/engine/gpu/view.rs` (NEW — the seven knobs + `from_env`) · `src/engine/gpu/frame.rs` (NEW —
`FrameUniforms`, `FrameInput`, `Binds`, the two writers, the two uniform structs) ·
`src/engine/gpu/targets.rs` (NEW — `Targets` with the depth TEXTURE + `begin_pass`) ·
`src/engine/gpu/present.rs` (NEW — `Frame`, `begin_present`/`end_present`, `clear`,
`render_offscreen`, `bench_frames`) · `src/engine/gpu/mod.rs` (loses all six; gains `ctx`, `frame`,
`targets`, `view`; 106 → 86) · `src/app/scene.rs` · `src/lib.rs` (five knob paths) ·
`src/selftest.rs` (two).

## Reference

Built in eight checkpoints, each compiled, the last one gated twice:

| checkpoint | what landed |
|---|---|
| 46a | `gpu/buffers.rs` — `GpuCtx`, `GrowBuf`, and the four free functions out of `gpu/mod.rs` |
| 46b | `Gpu { device, queue }` → `ctx: GpuCtx` — 10 pairs folded, 37 + 26 re-rooted (§6.2 splits five of those out by hand and sweeps 33 + 25) |
| 46c | `gpu/upload.rs` — `ArenaUpload` → `Upload`, moved flat (11 sites); `drop_uploaded` joins it |
| 46d | `gpu/view.rs` — the seven runtime knobs become `View` + `View::from_env` |
| 46e | `gpu/frame.rs` — `FrameUniforms` + `FrameInput` + `Binds` (27 re-rooted, 42 draw sites) |
| 46f | `gpu/targets.rs` — `Targets` + `begin_pass` at 4 params; depth keeps `TEXTURE_BINDING` |
| 46g | `gpu/present.rs` — `Frame` + `begin_present`/`end_present`/`clear` + the two native harnesses |
| 46h | the `Gpu` head reordered to `surface, ctx, config, layouts, pipelines, frame, targets, view` |

46h is folded into the steps above: 6.4, 6.5 and 6.6 each insert their field where it finally
belongs, so the head is right the first time.

`git diff end-of-45..end-of-46 -- session_viewer/src` is the whole lesson as one patch; `diff -u`
any single file against it if a line count comes out wrong.

## Next

Lesson **47** — **one row per object.** Run the evidence:

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
grep -c 'instance_id' src/engine/gpu/mod.rs src/app/scene.rs
grep -n 'objects_base\|base_f32\|bounded_rows\|object_bounds_world\|inside' src/engine/gpu/mod.rs | head
```

86 fields, and six of them are parallel arrays indexed by the same row number — `instances`,
`objects_base`, `base_f32`, `bounded_rows`, `object_bounds_world`, `inside` — with every row
struct in the program carrying an `instance_id` that points into them. Exactly one
`(model, tint, flags)` per guid is the seam the whole design hangs on, and `arena.rs` is the
worked example of the family contract that sits on top of it.

