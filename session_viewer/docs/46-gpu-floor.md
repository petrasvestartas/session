# 46 The GPU floor — the things every lane stands on

> Second refactor lesson. Start from the end of lesson 45. Pixels stay identical: `./docs/_gate.sh`
> prints `gate OK` at the end.

<svg viewBox="0 0 720 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Upload is the contract between app and engine; below it the GPU floor: GpuCtx, GrowBuf with its append rule, Targets, and FrameUniforms" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="fa" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="fb" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <rect x="14" y="20" width="170" height="48" fill="none" stroke="#f0b35c"/>
  <text x="99" y="40" fill="#f0b35c" font-size="11" text-anchor="middle">app/scene.rs</text>
  <text x="99" y="56" fill="#d7dae0" font-size="10" text-anchor="middle">Scene.tables: Upload</text>
  <line x1="184" y1="44" x2="248" y2="44" stroke="#f0b35c" marker-end="url(#fa)"/>
  <rect x="250" y="12" width="220" height="64" fill="none" stroke="#7ed37e" stroke-width="1.3"/>
  <text x="360" y="30" fill="#d7dae0" font-size="11" text-anchor="middle">Upload { verts/idx, pipes/spheres,</text>
  <text x="360" y="45" fill="#d7dae0" font-size="11" text-anchor="middle">segments/glyphs, cloud_*, objects }</text>
  <text x="360" y="64" fill="#888" font-size="9" text-anchor="middle">drop_uploaded(): every table except obj</text>
  <line x1="470" y1="44" x2="534" y2="44" stroke="#6fb3ff" marker-end="url(#fb)"/>
  <rect x="536" y="20" width="170" height="48" fill="none" stroke="#6fb3ff"/>
  <text x="621" y="40" fill="#6fb3ff" font-size="11" text-anchor="middle">engine/gpu/mod.rs</text>
  <text x="621" y="56" fill="#d7dae0" font-size="10" text-anchor="middle">Gpu::set_scene(&amp;Upload)</text>
  <text x="360" y="92" fill="#888" font-size="10" text-anchor="middle">the contract: typed per family; the engine never names session_rust except RenderVertex, Xform and Point</text>
  <line x1="14" y1="102" x2="706" y2="102" stroke="#3a3a3a"/>
  <g fill="none" stroke="#7ed37e">
    <rect x="14" y="116" width="150" height="70"/><rect x="180" y="116" width="240" height="120"/>
    <rect x="436" y="116" width="130" height="70"/><rect x="582" y="116" width="124" height="70"/>
  </g>
  <g fill="#d7dae0" font-size="11">
    <text x="22" y="134">GpuCtx</text><text x="188" y="134">GrowBuf { buf, len, cap }</text>
    <text x="444" y="134">Targets</text><text x="590" y="134">FrameUniforms</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="22" y="150">device, queue</text><text x="22" y="163">device.rs → DeviceSetup</text><text x="22" y="176">buffers.rs</text>
    <text x="444" y="150">depth, msaa, samples</text><text x="444" y="163">begin_pass(encoder,</text><text x="444" y="176">  view, clear)</text>
    <text x="590" y="150">mvp, line, cloud</text><text x="590" y="163">write(ctx, FrameInput,</text><text x="590" y="176">  FrameCx)</text>
    <text x="188" y="150">append(ctx, rows) -&gt; bool grew</text>
  </g>
  <g stroke="#0d0f12">
    <rect x="188" y="160" width="110" height="18" fill="#2b4a63"/><rect x="298" y="160" width="60" height="18" fill="#2b4a2b"/>
    <rect x="358" y="160" width="54" height="18" fill="none" stroke="#3a3a3a" stroke-dasharray="3 2"/>
  </g>
  <g fill="#d7dae0" font-size="9" text-anchor="middle">
    <text x="243" y="172">live prefix</text><text x="328" y="172">new rows</text><text x="385" y="172" fill="#666">free</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="188" y="196">1 cap doubles (max(need, cap·2))</text>
    <text x="188" y="209">2 prefix copied GPU-side (copy_buffer_to_buffer)</text>
    <text x="188" y="222">3 only the new rows are written</text>
  </g>
  <g fill="#888" font-size="10">
    <text x="14" y="258">view.rs — View::from_env(), read once · frame.rs — FrameInput { view_proj, clear }, Binds { mvp, line, instances }</text>
    <text x="14" y="274">present.rs — clear, render_offscreen, bench_frames · buffers.rs — Template { vbo, ibo, index_count }: cylinder, quad</text>
    <text x="14" y="290">green = created in lesson 46</text>
  </g>
</svg>

## Goal

Seven small files take the device negotiation, the growable buffer, the upload contract, the
runtime knobs, the per-frame uniforms, the render targets and the three ways a frame leaves the
GPU out of `gpu/mod.rs`. `Gpu` goes from 102 fields to 64 and the file from 2125 lines to 1379.

## Why

Every lane repeated the same three things: a `(buffer, count, cap)` triple with a hand-rolled
grow-and-copy, a bind group pointing at it, and an environment read at the wrong time. Naming
them once (`GrowBuf`, `View`, `FrameUniforms`) removes the repetition before lesson 47 moves the
lanes themselves. The uniforms and the targets get the same treatment so that lesson 48 can
write the frame as a list.

## Files

| file | change | lines after |
|---|---|---|
| `src/engine/gpu/device.rs` | created | 96 |
| `src/engine/gpu/buffers.rs` | created | 129 |
| `src/engine/gpu/upload.rs` | created | 101 |
| `src/engine/gpu/view.rs` | created | 91 |
| `src/engine/gpu/frame.rs` | created | 185 |
| `src/engine/gpu/targets.rs` | created | 86 |
| `src/engine/gpu/present.rs` | created | 125 |
| `src/engine/gpu/mod.rs` | edited | 1379 (was 2125) |
| `src/app/scene.rs`, `src/state.rs`, `src/lib.rs`, `src/selftest.rs` | edited | — |

The build only compiles again at the end: create the seven files first, then edit `gpu/mod.rs`.

## Step 1 — `src/engine/gpu/device.rs`

The instance → surface → adapter → device → surface-format negotiation, moved out of `Gpu::build`
into `open`, which hands back a `DeviceSetup` and keeps nothing.

**Create `src/engine/gpu/device.rs`**

```rust
//! Device negotiation: instance -> surface -> adapter -> device + queue -> surface format.
//! Produces one `DeviceSetup` and owns nothing afterwards: no buffer, no pipeline, no frame
//! state. Headless callers pass no window and get no surface.

use std::sync::Arc;
use winit::window::Window;

/// What `open` negotiated: the surface (None when headless), the device/queue pair, and the
/// surface configuration it was configured with.
pub struct DeviceSetup {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

/// Set up the wgpu objects in order: Instance -> Surface -> Adapter -> Device + Queue -> configure.
/// `size` is the canvas in pixels; a zero side is clamped to 1 so the surface can be configured.
pub async fn open(window: Option<Arc<Window>>, size: (u32, u32)) -> anyhow::Result<DeviceSetup> {
    // 1. Instance — the driver entry point. WebGPU only in the browser, never WebGL.
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: if cfg!(target_arch = "wasm32") {
            wgpu::Backends::BROWSER_WEBGPU
        } else {
            wgpu::Backends::PRIMARY //Vulkan / Metal / DX12 for native selftest
        },
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    // 2. Surface — the drawable canvas. 3. Adapter — a physical GPU compatible with it.
    let surface = match &window { Some(w) => Some(instance.create_surface(w.clone())?), None => None };
    // LowPower = the iGPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames can't be shared to the compositor - the canvas stays black.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await?;
    let info = adapter.get_info();
    log::info!("adapter: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
    if info.device_type == wgpu::DeviceType::Cpu {
        log::warn!("software adapter - rendering on the CPU will be slow");
    }

    // Limit to 128 mb, then the flat merge becomes the grid
    let mut limits = wgpu::Limits::default();
    let hw = adapter.limits();
    limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
    limits.max_buffer_size = hw.max_buffer_size;

    // 4. Device (creates resources) + Queue (submits work).
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,  // unlock the WEBGpu storage buffers
            memory_hints: Default::default(),
            ..Default::default()
        })
        .await?;

    device.on_uncaptured_error(Arc::new(|e|{ log::error!("wgpu on_uncaptured_error: {e}") }));

    // 5. Configure the surface: pixel format (prefer sRGB), size, vsync.
    // Headless has no capabilities to ask, so pick the format the readback path wants.
    let (format, present_mode, alpha_mode) = match &surface {
        Some(s) => {
            let caps = s.get_capabilities(&adapter);
            let f = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
            (f, caps.present_modes[0], caps.alpha_modes[0])
        }
        None => (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::PresentMode::Fifo,
            wgpu::CompositeAlphaMode::Auto,
        ),
    };
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.0.max(1),
        height: size.1.max(1),
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    if let Some(s) = &surface { s.configure(&device, &config); }

    Ok(DeviceSetup { surface, device, queue, config })
}
```

## Step 2 — `src/engine/gpu/buffers.rs`

`GpuCtx` is the device/queue pair every resource is made with; `GrowBuf` is the one growable
table, the body of the old `append_rows` with its count and capacity inside.

**Create `src/engine/gpu/buffers.rs`**

```rust
//! The GPU floor every lane stands on: `GpuCtx` (device + queue), `GrowBuf` (a table that
//! grows by appending, its live prefix copied GPU-side), `Template` (a unit mesh drawn N
//! times) and the two buffer helpers. No lane, no shader and no per-frame state lives here.

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

/// A growable GPU table: capacity doubles when it runs out, the live prefix is copied GPU-side
/// and only the new rows are written. Appending is what lets the CPU copy go after upload -
/// a lane that rebuilt its whole buffer per file had to keep every row twice.
pub struct GrowBuf {
    pub buf: wgpu::Buffer,
    len: u32,
    cap: u64,
    stride: u64,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl GrowBuf {
    /// One zeroed row: wgpu cannot bind a 0-byte buffer, and `len` starts at 0 so nothing
    /// draws from it. COPY_SRC is what lets a grown buffer take the old prefix GPU-side.
    pub fn new(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        let buf = zeroed_buffer(&ctx.device, label, stride, usage);

        Self { buf, len: 0, cap: 1, stride, usage, label }
    }

    /// Append rows. Returns `true` when the buffer was replaced, so the caller knows to rebuild
    /// the bind group pointing at it.
    pub fn append<T: Pod>(&mut self, ctx: &GpuCtx, data: &[T]) -> bool {
        debug_assert_eq!(std::mem::size_of::<T>() as u64, self.stride);

        if data.is_empty() {
            return false;
        }
        let stride = self.stride;
        let need = self.len as u64 + data.len() as u64;
        let mut grew = false;
        if need > self.cap {
            let new_cap = need.max(self.cap * 2);
            let nb = zeroed_buffer(&ctx.device, self.label, new_cap * stride, self.usage);
            if self.len > 0 {
                // the prefix moves GPU-side; it never travels back through wasm memory
                let mut enc = ctx.device.create_command_encoder(&Default::default());
                enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.len as u64 * stride);
                ctx.queue.submit([enc.finish()]);
            }
            self.buf = nb;
            self.cap = new_cap;
            grew = true;
        }
        ctx.queue.write_buffer(&self.buf, self.len as u64 * stride, bytemuck::cast_slice(data));
        self.len += data.len() as u32;
        grew
    }

    /// Forget the rows; the buffer and its capacity stay, so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.len = 0;
    }

    /// Rows on the GPU - the base for the next append and the instance count of a draw.
    pub fn len(&self) -> u32 {
        self.len
    }

    /// No rows: the draw that reads this table is skipped.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// A unit mesh drawn N times by an instanced lane (the cylinder, the marker quad).
pub struct Template {
    pub vbo: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub index_count: u32,
}

impl Template {
    /// Upload positions and indices once; `label` names the lane (`<label>.vbo`, `<label>.ibo`).
    pub fn new(ctx: &GpuCtx, label: &str, verts: &[[f32; 3]], idx: &[u32]) -> Self {
        let vbo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.vbo")),
            contents: bytemuck::cast_slice(verts),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ibo = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label}.ibo")),
            contents: bytemuck::cast_slice(idx),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { vbo, ibo, index_count: idx.len() as u32 }
    }
}

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the
/// empty-category placeholders both rely on that guarantee.
pub fn zeroed_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages
) -> wgpu::Buffer {
    device.create_buffer(
        &wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
}

/// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
pub fn rows_group(ctx: &GpuCtx, layout: &wgpu::BindGroupLayout, label: &str, buf: &wgpu::Buffer) -> wgpu::BindGroup {
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
    })
}
```

## Step 3 — `src/engine/gpu/upload.rs`

`ArenaUpload` becomes `Upload`, unchanged in shape, with `Default` and `drop_uploaded` (the
fourteen `drop_rows` calls that lived in `scene.rs`).

**Create `src/engine/gpu/upload.rs`**

```rust
//! `Upload` - the walked rows on their way to the GPU: every family's table for one file (a
//! DELTA) plus the cumulative object columns. Built by `app::scene::Scene`, borrowed by
//! `Gpu::set_scene`, then emptied. No wgpu type and no kernel type but `RenderVertex` here.

use crate::math::{Aabb, Mat4};
use session_rust::RenderVertex;
use super::{CloudDraw, CylinderSegment, GlyphPoint, LodNode};

/// Everything `Gpu` needs to fill its buffers, built and owned by `app::scene::Scene`;
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `objects` holds the TRUE per-object transform + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
pub struct Upload {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub pipes: Vec<CylinderSegment>, // Solid lane: Mesh/Brep edges, drawn as 3D cylinders
    pub spheres: Vec<GlyphPoint>, // Solid lane: Mesh/Brep vertices, radius matched to the pipes
    pub segments: Vec<CylinderSegment>, // Flat lane: line/polyline, drawn as camera-facing ribbons
    pub glyphs: Vec<GlyphPoint>, // Flat lane: points, draw as SDF dots,
    pub cloud_pos: Vec<f32>, // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>, // Raw lane: RBGA8 per point, 4 B
    pub cloud_nrm: Vec<u32>, // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
    pub cloud_draws: Vec<CloudDraw>, // first, count, instance, point spacing world units
    /// Sheet lanes. A PDF's fills are exactly coplanar, so they must NOT arbitrate by depth -
    /// they are split off the solid index run and drawn in document order with depth write off.
    /// `idx_text` is the lettering, drawn LAST of all, after the ink lanes, because a page puts
    /// its text on top of both its hatching and its linework.
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
    pub objects: Vec<(Mat4, [f32; 4], u32)>,
    /// Mesh-local AABB per object row, aligned with `objects`. None for linework/points/clouds:
    /// only the solid lane's facing cull needs it (see `Instance::FLAG_INSIDE`).
    pub object_bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per object row, world units, aligned with `objects`. 0 = unknown (linework,
    /// points, clouds), which the ink lanes read as "never density-cull".
    pub object_spacing: Vec<f32>,
    pub bounds: Aabb,
}

impl Default for Upload {
    /// Every lane empty and the box inverted, ready for the first walk.
    fn default() -> Self {
        Self {
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            pipes: Vec::new(),
            spheres: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
            cloud_pos: Vec::new(),
            cloud_col: Vec::new(),
            cloud_nrm: Vec::new(),
            cloud_draws: Vec::new(),
            cloud_nodes: Vec::new(),
            idx_print: Vec::new(),
            idx_text: Vec::new(),
            objects: Vec::new(),
            object_bounds: Vec::new(),
            object_spacing: Vec::new(),
            bounds: Aabb::empty(),
        }
    }
}

impl Upload {
    /// Forget the uploaded rows: the GPU is their only holder now. Every drawn table goes -
    /// nothing reads them back (picking goes through the kernel Meshes in `Doc.session`), and a
    /// kept copy is what let lanes rebuild whole buffers per file. `objects`, `object_bounds`
    /// and `object_spacing` STAY: the instance table is rebased from them on every re-anchor,
    /// and the walk indexes them by global row.
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

## Step 4 — `src/engine/gpu/view.rs`

Every runtime knob read once, at start: the three show toggles, the line style, the cloud
scale, EDL, LOD, the pen weight, and the marker switch that `encode_frame` used to read from
the environment on every frame.

**Create `src/engine/gpu/view.rs`**

```rust
//! `View` - the runtime knobs a frame reads: what to show, how the solid ink is drawn, the
//! cloud / EDL / LOD scalars and the pen weight. Read from the environment (or the query
//! string) ONCE at startup; the key handlers in lib.rs flip them afterwards. No GPU here.

/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory - which is the whole reason
/// the two lanes were built over one buffer. Easy3D ships exactly this pair
/// (`lines_cylinders_*` against `lines_plain_*_width_control`) and lets you pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface it
    /// decorates so silhouette edges never lose the depth test.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}

/// The knobs one frame reads.
pub struct View {
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`. ON by default; turn it
    /// off for a model whose outlines are drawn as polylines too, where the mesh's own topology
    /// gives those edges a second time and two strokes a fraction of a pixel apart read as one.
    pub show_mesh_edges: bool,
    /// Vertex markers on top of the solid ink; `BENCH_NO_MARKERS` turns them off for timing.
    pub markers: bool,
    /// Solid-lane style; `VIEWER_LINE_STYLE=tubes` picks Tubes at startup.
    pub line_style: LineStyle,
    /// Global SCALE on per-cloud point sizes, `[` and `]` keys (`VIEWER_CLOUD_SCALE`).
    pub cloud_size: f32,
    /// Eye-Dome Lighting strength; 0 = off (`VIEWER_EDL`).
    pub edl_strength: f32,
    /// Octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (`VIEWER_LOD`).
    pub lod_split_px: f32,
    /// On-screen pen weight, px (`VIEWER_THICKNESS` natively, `?thickness=` on wasm).
    pub thickness_px: f32,
    /// Lanes left out of the frame (`VIEWER_SKIP=pipes,ribbon,...`, native only) so
    /// `examples/bench_frame` can price each by subtraction. Names: background grid arena fills
    /// pipes splat splat_points spheres ribbon_depth glyph_depth ribbon text glyph.
    pub skip_lanes: Vec<String>,
}

impl View {
    /// Read every knob once. Env vars are unreachable on wasm, so there the defaults hold and
    /// only the pen weight has a query-string override.
    pub fn from_env() -> Self {
        let tubes = std::env::var("VIEWER_LINE_STYLE").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false);

        Self {
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            markers: std::env::var("BENCH_NO_MARKERS").is_err(),
            line_style: if tubes { LineStyle::Tubes } else { LineStyle::Flat },
            cloud_size: env_f32("VIEWER_CLOUD_SCALE", 1.0),
            edl_strength: env_f32("VIEWER_EDL", 0.25),
            lod_split_px: env_f32("VIEWER_LOD", 1.0),
            thickness_px: thickness_px(),
            skip_lanes: std::env::var("VIEWER_SKIP").unwrap_or_default().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        }
    }

    /// True when `VIEWER_SKIP` names this lane. Never true in the browser (no environment).
    pub fn skip(&self, lane: &str) -> bool {
        self.skip_lanes.iter().any(|l| l == lane)
    }
}

/// A float knob from the environment; `default` when unset or unparsable.
fn env_f32(name: &str, default: f32) -> f32 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// On-screen pen weight in px, default 2.0 - the floor at which 4x MSAA has something to
/// work with: a 1 px pen lands on one or two coverage samples and resolves dim and broken,
/// and the density taper (`WIRE_MIN_PENS`) can thin it to 0.15 of that on a dense mesh.
/// `?thickness=1.5` tunes an embed without a rebuild; `VIEWER_THICKNESS` does the same natively.
fn thickness_px() -> f32 {
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
}
```

## Step 5 — `src/engine/gpu/frame.rs`

The three uniform buffers and their bind groups, written once per frame from a `FrameInput`
(camera matrix + clear colour) and a `FrameCx` (knobs, anchor, size). The eye and the ortho
half-height are solved here once and read by everyone else.

**Create `src/engine/gpu/frame.rs`**

```rust
//! The per-frame uniforms every shader reads: the camera matrix (group 0), the line/pen block
//! and the cloud block (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the splat records and the inside test.

use crate::engine::pipelines::Layouts;
use crate::math::{eye_from_view_proj, ortho_half_height};
use session_rust::Xform;
use super::buffers::GpuCtx;
use super::view::View;
use wgpu::util::DeviceExt;

/// What one frame needs from the camera, computed once per frame by the caller.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
}

/// What `FrameUniforms::write` needs besides the camera: the knobs, the anchor the instance
/// rows are rebased about, and the framebuffer size in pixels.
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32),
}

/// The three bind groups every family draw needs, borrowed for one pass.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,
    pub line: &'a wgpu::BindGroup,
    pub instances: &'a wgpu::BindGroup,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    // Camera position, in the SAME anchored frame the instance rows use - so a shader can build
    // the view ray to a point as `eye - p`. That is what the per-edge facing test needs, and it
    // has to be the real eye rather than a constant forward direction: at this 60 degree FOV a
    // constant direction is off by up to 30 degrees at the frame corner, and it is precisely the
    // near-silhouette edges - the ones whose classification is in doubt - that would flip.
    eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
    // The camera-relative ANCHOR, world units. Instance rows are rebased about it, so anything
    // NOT an instance - the grid, the axes - has to subtract it too or it drifts away from the
    // scene every time re-anchoring fires.
    anchor: [f32; 3],
    _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
} // 48 B - three vec4s

// The shaders declare this same struct with `anchor: vec3<f32>`, which WGSL aligns to 16 - so the
// uniform is 48 B there, not the 32 B a naive Rust layout gives. A mismatch is not a compile error:
// it surfaces at run time as "buffer bound with size 32 ... requires at least 48 bytes", every
// frame, from every pipeline that binds group 1.
const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);

// Points global attributes
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudUniform{
    size: f32, // global point-cloud size SCALE ([ and ] keys)
    vp_w: f32, // framebuffer width, px
    vp_h: f32, // framebuffer height, px
    _pad: f32,
} // 16 B - one vec4; its own buffer + bind group

/// The three uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    pub(super) mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    pub(super) cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32: the splat static-skip key and the record fold.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective), for the splat k.
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test and the LOD screen-error test.
    pub eye: [f32; 3],
}

impl FrameUniforms {
    /// The three buffers and bind groups with no camera yet: identity mvp, a 2 px pen, size-4
    /// clouds. The cloud block reuses the line layout (one uniform at binding 0).
    pub fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let mvp_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &l.mvp,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        let line_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: size.1 as f32,
                vp_w: size.0 as f32,
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &l.line,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });

        let cloud_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: size.0 as f32,
                vp_h: size.1 as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &l.line,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });

        Self {
            mvp_buffer,
            line_buffer,
            cloud_buffer,
            mvp_group,
            line_group,
            cloud_group,
            mvp_f32: [0.0; 16],
            ortho_h: 0.0,
            eye: [0.0; 3],
        }
    }

    /// Per-frame uniforms: camera, the line/pen block, and the cloud block. The eye and the
    /// ortho half-height are solved once here and kept for the rest of the frame.
    pub fn write(&mut self, ctx: &GpuCtx, input: &FrameInput, cx: &FrameCx) {
        self.mvp_f32 = input.view_proj.to_f32();
        self.ortho_h = ortho_half_height(&input.view_proj);
        self.eye = eye_from_view_proj(&input.view_proj);
        ctx.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform{
            thickness: cx.view.thickness_px,
            proj_y: 1.0 / (30.0_f32).to_radians().tan() * 0.001, // cot(fovy/2) mm-m unit scale
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
            _pad1: 0.0,
        };
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
        ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&CloudUniform{
            size: cx.view.cloud_size,
            vp_w: cx.size.0 as f32,
            vp_h: cx.size.1 as f32,
            _pad: cx.view.edl_strength, // EDL strength, read by the splat resolve
        }));
    }
}
```

## Step 6 — `src/engine/gpu/targets.rs`

The depth and MSAA attachments and the one render pass that clears them.

**Create `src/engine/gpu/targets.rs`**

```rust
//! `Targets` - the depth and MSAA colour attachments a frame renders into, sized to the
//! surface at the sample count the scene chose, and the one render pass that clears them.
//! Nothing here knows what is drawn; it only opens the pass.

use super::buffers::GpuCtx;

/// The two attachments of the frame's render pass, and the sample count they were made at.
pub struct Targets {
    pub depth: wgpu::TextureView,
    pub msaa: wgpu::TextureView,
    pub samples: u32,
}

impl Targets {
    /// Both attachments for `config`'s size and format at `samples` (1 or 4).
    pub fn new(ctx: &GpuCtx, config: &wgpu::SurfaceConfiguration, samples: u32) -> Self {
        let depth = depth_view(&ctx.device, config, samples);
        let msaa = msaa_view(&ctx.device, config, samples);

        Self { depth, msaa, samples }
    }

    /// Open the frame's render pass: colour cleared to `clear`, depth cleared to 0 (reverse-Z
    /// far). At 1x the pass draws straight into `view`.
    pub fn begin_pass<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,
        clear: wgpu::Color,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            // MSAA off (samples == 1): draw straight to the swapchain view - a
            // 1-sample attachment must NOT carry a resolve target.
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: if self.samples > 1 { &self.msaa } else { view },
                resolve_target: if self.samples > 1 { Some(view) } else { None },
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth,
                depth_ops: Some(
                    wgpu::Operations{load: wgpu::LoadOp::Clear(0.0),
                    store:wgpu::StoreOp::Store,
                }),
                stencil_ops: None }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }
}

/// Create the reverse-Z depth texture view, sized to the surface at the MSAA sample count.
fn depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
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
fn msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, samples: u32) -> wgpu::TextureView {
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
```

## Step 7 — `src/engine/gpu/present.rs`

`clear` and `render_offscreen` take a `FrameInput` now; `bench_frames` builds its own. The bodies are unchanged.

**Create `src/engine/gpu/present.rs`**

```rust
//! The three ways a frame leaves `Gpu`: presented to the swapchain (`clear`), read back from
//! an offscreen texture (`render_offscreen`, the native harness), or timed in a batch
//! (`bench_frames`). Each writes the uniforms, encodes through `encode_frame`, and submits.

use session_rust::Xform;
use crate::engine::performance::{heap_mb, now_ms, perf_logging};
use super::Gpu;
use super::frame::FrameInput;

impl Gpu {
    /// Draw one frame to the swapchain. The frame ENCODING lives in `encode_frame` so a
    /// headless harness can aim the same code at an offscreen texture and read the pixels back -
    /// see `selftest.rs`. Shader work that is only ever checked in a browser is shader work
    /// checked by somebody else's eyes.
    pub fn clear(&mut self, input: &FrameInput) -> anyhow::Result<()> {
        self.write_frame_uniforms(input);

        // wgpu 29: get_current_texture() returns an enum, not a Result.
        let Some(surface) = &self.surface else { return Ok(()) }; // headless: nothing to present
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => { surface.configure(&self.ctx.device, &self.config); return Ok(()); }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("clear encoder"),
        });
        let t0 = now_ms();
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        let t1 = now_ms();
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        let t2 = now_ms();
        self.performance.frame(draws, objects);
        self.frame_no += 1;
        // `?perf=1`: the frame line in the page's top-left corner. Console and title are not
        // enough - a busy page keeps DevTools from ever showing a console line, and the tab
        // title is cached by the browser UI. A DOM element is readable from a screenshot.
        #[cfg(target_arch = "wasm32")]
        if perf_logging() {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let line = format!("f{} gap {:.0} enc {:.0} submit {:.0} ms heap {:.0} MB", self.frame_no, t0 - self.last_frame_t, t1 - t0, t2 - t1, heap_mb());
                let el = match doc.get_element_by_id("perf") {
                    Some(e) => Some(e),
                    None => doc.create_element("pre").ok().and_then(|e| {
                        e.set_id("perf");
                        let _ = e.set_attribute("style", "position:fixed;left:0;top:0;margin:0;padding:2px 6px;font:12px monospace;color:#000;background:rgba(255,255,255,.7);z-index:9;pointer-events:none");
                        doc.body().map(|b| { let _ = b.append_child(&e); e })
                    }),
                };
                if let Some(el) = el { el.set_text_content(Some(&line)); }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (t0, t1, perf_logging, heap_mb);
        self.last_frame_t = t2;
        Ok(())
    }

    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
    /// packed, top row first). Native only - this is the harness that lets a shader be looked at
    /// on this machine before it is shipped to a browser.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, input: &FrameInput) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let tex = self.ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("headless.color"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        // copy_texture_to_buffer needs each row padded to 256 B
        let unpadded = w * 4;
        let pad = (256 - unpadded % 256) % 256;
        let padded = unpadded + pad;
        let readback = self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("headless.readback"),
            size: (padded * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.write_frame_uniforms(input);
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(h) },
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.ctx.queue.submit([encoder.finish()]);
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((unpadded * h) as usize);
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + unpadded as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }

    /// Time `frames` full frames (encode + submit), reusing one offscreen target, and wait for
    /// the GPU to drain. Native bench helper: returns seconds for the whole batch, warmup
    /// excluded, so two line styles can be compared on the same scene.
    pub fn bench_frames(&mut self, view_proj: &Xform, frames: u32) -> f64 {
        let (w, h) = (self.config.width, self.config.height);
        let tex = self.ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bench.color"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
        let input = FrameInput { view_proj: view_proj.clone(), clear };
        self.write_frame_uniforms(&input);
        for _ in 0..3 { // warmup: pipeline/driver caches
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, clear);
            self.ctx.queue.submit([encoder.finish()]);
        }
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let t0 = std::time::Instant::now();
        for _ in 0..frames {
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, clear);
            self.ctx.queue.submit([encoder.finish()]);
            let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        }
        t0.elapsed().as_secs_f64()
    }
}
```

## Step 8 — `src/engine/gpu/mod.rs`

`Gpu` keeps the lanes and loses the floor: `device`/`queue` become `ctx`, the tables become
`GrowBuf`s, the knobs `view`, the uniforms `frame`, the attachments `targets`. The edits go top
to bottom; a Remove that stops one line short of a closing brace leaves it for the Add that
follows.

**Find** in `src/engine/gpu/mod.rs`:

```rust
//! `Gpu` — our handle to the graphics card and the lowest layer of the viewer (ARCHITECTURE.md §1).
//!
//! It owns the three things wgpu needs to draw:
//!   • `device` — makes GPU resources (textures, buffers, pipelines)
//!   • `queue`  — sends work to the GPU
//!   • `surface`— the canvas pixels we present each frame
//! plus the `config` describing the surface size/format. It knows nothing app-specific — its whole
//! job is "hand me a cleared frame". Higher layers sit on top and only talk to this.

use crate::engine::pipelines::{Pipelines, Target, Layouts, SPLAT_COLOR_FORMAT};
```

**Replace with:**

```rust
//! `Gpu` - the lowest layer of the viewer (ARCHITECTURE.md §1): the surface, the `GpuCtx`
//! (device + queue), the layouts and pipelines, the per-frame uniforms and targets, and every
//! lane's tables. The floor it stands on lives in the child modules - device, buffers, upload,
//! view, frame, targets, present. It knows nothing app-specific.

pub mod buffers;
pub mod device;
pub mod frame;
pub mod present;
pub mod targets;
pub mod upload;
pub mod view;

use crate::engine::pipelines::{Pipelines, Target, Layouts, SPLAT_COLOR_FORMAT};
use device::DeviceSetup;
use buffers::{GpuCtx, GrowBuf, Template, zeroed_buffer, rows_group};
pub use upload::Upload;
pub use frame::FrameInput;
use frame::{Binds, FrameCx, FrameUniforms};
use targets::Targets;
pub use view::{LineStyle, View};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use session_rust::{Xform, RenderVertex, Point};
use crate::math::{Mat4, mat_to_f32, eye_from_view_proj, ortho_half_height, Aabb};
```

**Replace with:**

```rust
use session_rust::{Xform, RenderVertex, Point};
use crate::math::{Mat4, mat_to_f32, Aabb};
```

The two append helpers are `GrowBuf::append` now; only the constant stays.

**Find** in `src/engine/gpu/mod.rs`:

```rust
const INK_DEPTH_PREPASS: bool = false;

/// Grow-and-append one index run. Same shape as the solid arena's own append: the existing
/// prefix is copied GPU-side, never back through wasm memory.
/// Append rows to a growable STORAGE buffer: double the capacity when it runs out, move the
/// prefix GPU-side, and write only the new rows. Returns `true` when the buffer was replaced, so
/// the caller knows to rebuild the bind group pointing at it.
///
/// This is the same deal the mesh arena already struck, extended to the lanes that had not taken
/// it: a lane that rebuilds its whole buffer per file re-sends every earlier file's rows (five
/// files means the last one travels once and the first one five times), and it can only do that
/// because the CPU-side table is still there to re-send FROM - so the rows are held twice, in
/// wasm memory and on the GPU, for the whole session. On a 13.8 M-point scan that second copy is
/// 280 MB of browser heap.
fn append_rows<T: bytemuck::Pod>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    buf: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[T],
) -> bool {
    if data.is_empty() {
        return false;
    }
    let stride = std::mem::size_of::<T>() as u64;
    let need = *count as u64 + data.len() as u64;
    let mut grew = false;
    if need > *cap {
        let new_cap = need.max(*cap * 2);
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let nb = zeroed_buffer(device, label, new_cap * stride, usage);
        if *count > 0 {
            // the prefix moves GPU-side; it never travels back through wasm memory
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(buf, 0, &nb, 0, *count as u64 * stride);
            queue.submit([enc.finish()]);
        }
        *buf = nb;
        *cap = new_cap;
        grew = true;
    }
    queue.write_buffer(buf, *count as u64 * stride, bytemuck::cast_slice(data));
    *count += data.len() as u32;
    grew
}

fn append_index_run(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label: &str,
    ibo: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[u32],
) {
    if data.is_empty() {
        return;
    }
    let need = *count as u64 + data.len() as u64;
    if need > *cap {
        let new_cap = need.max(*cap * 2);
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let nb = zeroed_buffer(device, label, new_cap * 4, iu);
        if *count > 0 {
            let mut enc = device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(ibo, 0, &nb, 0, *count as u64 * 4);
            queue.submit([enc.finish()]);
        }
        *ibo = nb;
        *cap = new_cap;
    }
    queue.write_buffer(ibo, *count as u64 * 4, bytemuck::cast_slice(data));
    *count += data.len() as u32;
}
```

**Replace with:**

```rust
const INK_DEPTH_PREPASS: bool = false;
```

Remove from the `ArenaUpload` doc line through `pub line_bind_group` inclusive: `ArenaUpload`,
`LineStyle` and the head of `Gpu` go; the next edit re-opens `Gpu` under `LodNode` with its
floor fields.

**Remove** `src/engine/gpu/mod.rs` **through**

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
```

```rust
    pub line_bind_group: wgpu::BindGroup,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub children: [i32; 8],
}

```

**Add below it:**

```rust
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
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    arena_ibo_print: wgpu::Buffer,
    arena_print_count: u32,
    arena_print_cap: u64,
    arena_ibo_text: wgpu::Buffer,
    arena_text_count: u32,
    arena_text_cap: u64,
```

**Replace with:**

```rust
    arena_ibo_print: GrowBuf,
    arena_ibo_text: GrowBuf,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Per-object WORLD AABB (ArenaUpload.object_bounds through the true transform), aligned with
```

**Replace with:**

```rust
    /// Per-object WORLD AABB (Upload.object_bounds through the true transform), aligned with
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Layouts survive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,

    instance_buffer: wgpu::Buffer, // new() builds this storage buffer as a local and drops it, only the bidn group survives; rebuild_instances() reuploads into it every frame, so the buffer handle itself must live on GPU, not vanish atht eh of new()
    instance_rows: u32, // instance rows already ON the GPU - the base for the next append
    instance_cap: u64,
    pub instance_bind_group: wgpu::BindGroup,
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
```

**Replace with:**

```rust
    instance_buffer: GrowBuf, // rebuild_instances() rewrites it whole on every re-anchor
    pub instance_bind_group: wgpu::BindGroup,
    pub cyl_template: Template,
```

One `GrowBuf` per ink lane.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub pipe_buffer: wgpu::Buffer,
    pub pipe_bind_group: wgpu::BindGroup,
    pub pipe_count: u32,
    pub pipe_cap: u64,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub segment_cap: u64,
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    /// Vertex ink, split the same way: spheres are mesh/BRep vertices, glyphs are flat dots.
    pub sphere_buffer: wgpu::Buffer,
    pub sphere_bind_group: wgpu::BindGroup,
    pub sphere_count: u32,
    pub sphere_cap: u64,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub glyph_cap: u64,
    pub point_buffer: wgpu::Buffer, // positions, array<f32>
    pub point_col_buffer: wgpu::Buffer, // colours, array<u32> RGBA8
    pub point_nrm_buffer: wgpu::Buffer, // normals, array<u32> oct16 (u32::MAX = none)
    pub point_cap: u64,     // capacity in POINTS; positions hold 3 floats each
    pub point_col_cap: u64,
    pub point_nrm_cap: u64,
```

**Replace with:**

```rust
    pub pipes: GrowBuf,
    pub pipe_bind_group: wgpu::BindGroup,
    pub segments: GrowBuf,
    pub segment_bind_group: wgpu::BindGroup,
    pub sph_template: Template,
    /// Vertex ink, split the same way: spheres are mesh/BRep vertices, glyphs are flat dots.
    pub spheres: GrowBuf,
    pub sphere_bind_group: wgpu::BindGroup,
    pub glyphs: GrowBuf,
    pub glyph_bind_group: wgpu::BindGroup,
    pub point_pos: GrowBuf, // positions, array<f32> - three rows per point
    pub point_col: GrowBuf, // colours, array<u32> RGBA8
    pub point_nrm: GrowBuf, // normals, array<u32> oct16 (u32::MAX = none)
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

The knobs go to `view`, the cloud uniform to `frame`, the attachments to `targets`; only
`last_rebase_ms` stays.

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Solid-lane style; `VIEWER_LINE_STYLE=flat` picks Flat at startup.
    pub line_style: LineStyle,
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// Mesh/BRep edges and their vertex markers - the SOLID lane. `E`.
    ///
    /// ON by default. Turn it off for a model whose outlines are drawn as
    /// polylines too - a plate with its cut outline, say - where the mesh's own
    /// topology gives those same edges a second time, and two strokes a fraction
    /// of a pixel apart read as one thick ragged line rather than as two things.
    pub show_mesh_edges: bool,
    pub cloud_buffer: wgpu::Buffer,
    pub cloud_size: f32, // global SCALE on per-cloud sizes, [ and ] keys
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    pub edl_strength: f32, // Eye-Dome Lighting strength; 0 = off (VIEWER_EDL)
    last_ortho_h: f32, // ortho half-height this frame (0=perspective), for the plat k
    last_eye: [f32; 3], // eye in anchored world units, for the LOD screen-error test
    pub lod_split_px: f32, // octree LOD cutoff: descend while a node's spacing projects wider; 0 = off (VIEWER_LOD)
    pub cloud_bind_group: wgpu::BindGroup,
    pub depth_view: wgpu::TextureView,
    pub msaa_view: wgpu::TextureView,
    pub samples: u32, // MSAA sample count this scene chose (see `msaa_for`)
```

**Replace with:**

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
```

Drop the blank line after `impl Gpu {` so `new` follows the brace directly.

**Find** in `src/engine/gpu/mod.rs`:

```rust
impl Gpu {

```

**Replace with:**

```rust
impl Gpu {
```

The whole negotiation is `device::open`. Search the first line with its leading spaces (the same
text ends the two constructor signatures above); the Remove takes the signature line too, and
the next edit puts it back with the new body.

**Remove** `src/engine/gpu/mod.rs` `    ) -> anyhow::Result<Self> {` **through** `        // The loader calls set_scene the moment the first file's tables exist.`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        height: u32,
```

**Add below it:**

```rust
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

        // The scene-shaped tables start as one zeroed row each: wgpu cannot bind a 0-byte
        // buffer, and every length is 0, so the first frame draws nothing. The loader calls
        // set_scene the moment the first file's tables exist.
```

**Find** in `src/engine/gpu/mod.rs`:

```rust

        // COPY_SRC because the table GROWS by appending: when it outgrows its buffer the prefix
        // is copied GPU-side into the bigger one, and a buffer without COPY_SRC cannot be the
        // source of that copy.
        let instance_buffer = zeroed_buffer(
            &device,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
```

**Replace with:**

```rust
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let instance_buffer = GrowBuf::new(&ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, rows);
        let instance_bind_group = rows_group(&ctx, &layouts.instance, "instances.bind_group", &instance_buffer.buf);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
        let iu_sheet = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_ibo_print = zeroed_buffer(&device, "arena.ibo.print", 4, iu_sheet);
        let arena_ibo_text = zeroed_buffer(&device, "arena.ibo.text", 4, iu_sheet);
        let (arena_print_count, arena_print_cap) = (0u32, 1u64);
        let (arena_text_count, arena_text_cap) = (0u32, 1u64);
```

**Replace with:**

```rust

        // The mesh arena keeps its exact-fit growth (see set_scene); the two sheet index runs
        // grow by appending like every other lane.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_vbo = zeroed_buffer(&ctx.device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu);
        let arena_vids = zeroed_buffer(&ctx.device, "arena.vids", 4, vu);
        let arena_ibo = zeroed_buffer(&ctx.device, "arena.ibo", 4, iu);
        let arena_ibo_print = GrowBuf::new(&ctx, "arena.ibo.print", 4, iu);
        let arena_ibo_text = GrowBuf::new(&ctx, "arena.ibo.text", 4, iu);
        let arena_index_count = 0u32;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instances.bind_group"),
            layout: &layouts.instance,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: instance_buffer.as_entire_binding()}],
        });

        // One zeroed row each - wgpu cannot bind a 0-byte buffer, and arena_index_count starts
        // at 0 so nothing is drawn from them until real geometry appends.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_vbo = zeroed_buffer(&device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu);
        let arena_vids = zeroed_buffer(&device, "arena.vids", 4, vu);
        let arena_ibo = zeroed_buffer(&device, "arena.ibo", 4, iu);

        // Unit-cylinder tempalte (positions only) - one mesh, instance per edge.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let cyl_index_count = cyl_i.len() as u32;

        let cyl_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.vbo"),
            contents: bytemuck::cast_slice(&cyl_v),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let cyl_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.ibo"),
            contents: bytemuck::cast_slice(&cyl_i),
            usage: wgpu::BufferUsages::INDEX,
        });
```

**Replace with:**

```rust
        // Unit-cylinder template (positions only) - one mesh, an instance per edge.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let cyl_template = Template::new(&ctx, "cyl.template", &cyl_v, &cyl_i);
```

The four ink lanes, the point buffers and the two templates: `GrowBuf::new` and `Template::new`
instead of ninety lines of descriptors.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let pipe_cap = 1u64;
        let segment_cap = 1u64;
        let pipe_buffer = zeroed_buffer(
            &device, "pipes.buffer",
            std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let segment_buffer =  zeroed_buffer(
            &device, "segments.buffer",
            std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);

        let pipe_bind_group = Self::mk_rows_group(&device, &layouts.segment, "pipes.bind_group", &pipe_buffer);
        let segment_bind_group = Self::mk_rows_group(&device, &layouts.segment, "segments.bind_group", &segment_buffer);

        // Camera-facing quad template (positions-only) - one mesh, instance per marker
        let (sph_v, sph_i) = unit_quad();
        let sph_index_count = sph_i.len() as u32;
        let sph_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.vbo"),
            contents: bytemuck::cast_slice(&sph_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sph_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.ibo"),
            contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });
        let sphere_cap = 1u64;
        let glyph_cap = 1u64;
        let sphere_buffer = zeroed_buffer(
            &device,
            "spheres.buffer",
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let glyph_buffer =  zeroed_buffer(
            &device,
            "glyphs.buffer",
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let sphere_bind_group = Self::mk_rows_group(&device, &layouts.glyph, "spheres.bind_group", &sphere_buffer);
        let glyph_bind_group = Self::mk_rows_group(&device, &layouts.glyph, "glyphs.bind_group", &glyph_buffer);

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

        // Point cloud tables - empty until set_scene fill them from ArenaUpload
        let point_count = 0u32;
        let (point_cap, point_col_cap, point_nrm_cap) = (3u64, 1u64, 1u64);
        let point_buffer = zeroed_buffer(&device, "points.buffer", 12, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_col_buffer = zeroed_buffer(&device, "points.col.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let point_nrm_buffer = zeroed_buffer(&device, "points.nrm.buffer", 4, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);

        // point cloud unioform - the cloud's OWN global size + viewport (reuses line_layout)
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
        let seg_stride = std::mem::size_of::<CylinderSegment>() as u64;
        let pipes = GrowBuf::new(&ctx, "pipes.buffer", seg_stride, rows);
        let segments = GrowBuf::new(&ctx, "segments.buffer", seg_stride, rows);
        let pipe_bind_group = rows_group(&ctx, &layouts.segment, "pipes.bind_group", &pipes.buf);
        let segment_bind_group = rows_group(&ctx, &layouts.segment, "segments.bind_group", &segments.buf);

        // Camera-facing quad template (positions only) - one mesh, an instance per marker.
        let (sph_v, sph_i) = unit_quad();
        let sph_template = Template::new(&ctx, "sph.template", &sph_v, &sph_i);
        let glyph_stride = std::mem::size_of::<GlyphPoint>() as u64;
        let spheres = GrowBuf::new(&ctx, "spheres.buffer", glyph_stride, rows);
        let glyphs = GrowBuf::new(&ctx, "glyphs.buffer", glyph_stride, rows);
        let sphere_bind_group = rows_group(&ctx, &layouts.glyph, "spheres.bind_group", &spheres.buf);
        let glyph_bind_group = rows_group(&ctx, &layouts.glyph, "glyphs.bind_group", &glyphs.buf);

        // Point cloud tables - empty until set_scene fills them from the upload.
        let point_count = 0u32;
        let point_pos = GrowBuf::new(&ctx, "points.buffer", 4, rows);
        let point_col = GrowBuf::new(&ctx, "points.col.buffer", 4, rows);
        let point_nrm = GrowBuf::new(&ctx, "points.nrm.buffer", 4, rows);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let (splat_depth_view, splat_color_view) = Self::create_splat_targets(&device, &config);
        let splat_recs = zeroed_buffer(&device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0 = Self::mk_splat_group0(
            &device,
            &layouts.splat_group0,
            &mvp_buffer,
            &cloud_buffer,
```

**Replace with:**

```rust
        let (splat_depth_view, splat_color_view) = Self::create_splat_targets(&ctx.device, &config);
        let splat_recs = zeroed_buffer(&ctx.device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0 = Self::mk_splat_group0(
            &ctx.device,
            &layouts.splat_group0,
            &frame.mvp_buffer,
            &frame.cloud_buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            &device,
            &layouts.splat_group1,
            &point_buffer,
            &point_col_buffer,
            &point_nrm_buffer,
```

**Replace with:**

```rust
            &ctx.device,
            &layouts.splat_group1,
            &point_pos.buf,
            &point_col.buf,
            &point_nrm.buf,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let stream_pos_buf = zeroed_buffer(&device, "stream.pos", 12, stream_usage);
        let stream_col_buf = zeroed_buffer(&device, "stream.col", 4, stream_usage);
        let stream_nrm_buf = zeroed_buffer(&device, "stream.nrm", 4, stream_usage);
        let splat_stream_recs = zeroed_buffer(&device, "splat.stream.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0_stream = Self::mk_splat_group0(&device, &layouts.splat_group0, &mvp_buffer, &cloud_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&device, &layouts.splat_group1, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &device,
```

**Replace with:**

```rust
        let stream_pos_buf = zeroed_buffer(&ctx.device, "stream.pos", 12, stream_usage);
        let stream_col_buf = zeroed_buffer(&ctx.device, "stream.col", 4, stream_usage);
        let stream_nrm_buf = zeroed_buffer(&ctx.device, "stream.nrm", 4, stream_usage);
        let splat_stream_recs = zeroed_buffer(&ctx.device, "splat.stream.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0_stream = Self::mk_splat_group0(&ctx.device, &layouts.splat_group0, &frame.mvp_buffer, &frame.cloud_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&ctx.device, &layouts.splat_group1, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &ctx.device,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let pipelines = Pipelines::new(&device, Target { format: config.format, samples }, &layouts);
```

**Replace with:**

```rust
        let pipelines = Pipelines::new(&ctx.device, Target { format: config.format, samples }, &layouts);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            device,
            queue,
            config,
            pipelines,
            mvp_buffer, // shared: camera
            mvp_bind_group,
            line_buffer,  // shared: px-sizing for cylinders + spheres
            line_bind_group,
```

**Replace with:**

```rust
            ctx,
            config,
            layouts,
            pipelines,
            frame,
            targets,
            view: View::from_env(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            arena_print_count,
            arena_print_cap,
            arena_ibo_text,
            arena_text_count,
            arena_text_cap,
```

**Replace with:**

```rust
            arena_ibo_text,
```

The struct literal follows the new field list.

**Find** in `src/engine/gpu/mod.rs`:

```rust
            layouts,
            instance_buffer, // was a dropped local in new(), now moved onto GPU so rebuild_instances() can write into every frame
            instance_rows: 0,
            instance_cap: 1,
            instance_bind_group,
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            pipe_buffer,
            pipe_bind_group,
            pipe_count,
            pipe_cap,
            segment_buffer,
            segment_bind_group,
            segment_count,
            segment_cap,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            sphere_buffer,
            sphere_bind_group,
            sphere_count,
            sphere_cap,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            glyph_cap,
            point_buffer,
            point_col_buffer,
            point_nrm_buffer,
            point_cap,
            point_col_cap,
            point_nrm_cap,
```

**Replace with:**

```rust
            instance_buffer,
            instance_bind_group,
            cyl_template,
            pipes,
            pipe_bind_group,
            segments,
            segment_bind_group,
            sph_template,
            spheres,
            sphere_bind_group,
            glyphs,
            glyph_bind_group,
            point_pos,
            point_col,
            point_nrm,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat_group1_stream,
            mvp_f32: [0.0; 16],
```

**Replace with:**

```rust
            splat_group1_stream,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            line_style: if std::env::var("VIEWER_LINE_STYLE").map(|v| v.eq_ignore_ascii_case("tubes")).unwrap_or(false) {
                LineStyle::Tubes
            } else {
                LineStyle::Flat
            },
            cloud_buffer,
            cloud_size: std::env::var("VIEWER_CLOUD_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            last_rebase_ms: 0.0,
            edl_strength: std::env::var("VIEWER_EDL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.25),
            last_ortho_h: 0.0,
            last_eye: [0.0; 3],
            lod_split_px: std::env::var("VIEWER_LOD").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0),
            cloud_bind_group,
            depth_view,
            msaa_view,
            samples,
            performance: Performance::new(),
            frame_no: 0,
            last_frame_t: 0.0,
            bounds: Aabb::empty(),
         })

```

**Replace with:**

```rust
            last_rebase_ms: 0.0,
            performance: Performance::new(),
            frame_no: 0,
            last_frame_t: 0.0,
            bounds: Aabb::empty(),
        })
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub fn set_scene(&mut self, up: &ArenaUpload){
```

**Replace with:**

```rust
    pub fn set_scene(&mut self, up: &Upload){
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.instance_rows = 0;
```

**Replace with:**

```rust
            self.instance_buffer.reset();
```

The instance table appends through its `GrowBuf` too; a grown buffer means a new bind group.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let mut rows = self.instance_rows;
        let fresh = &self.instances[rows as usize..];
        if append_rows(&self.device, &self.queue, "instance.buffer",
            &mut self.instance_buffer, &mut rows, &mut self.instance_cap, fresh) {
            self.instance_bind_group = Self::mk_rows_group(&self.device, &self.layouts.instance, "instances.bind_group", &self.instance_buffer);
        }
        self.instance_rows = rows;
```

**Replace with:**

```rust
        let fresh = &self.instances[self.instance_buffer.len() as usize..];
        if self.instance_buffer.append(&self.ctx, fresh) {
            self.instance_bind_group = rows_group(&self.ctx, &self.layouts.instance, "instances.bind_group", &self.instance_buffer.buf);
        }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                let vbo = zeroed_buffer(&self.device, "arena.vbo", cap_v * vstride, vu);
                let vids = zeroed_buffer(&self.device, "arena.vids", cap_v * 4, vu);
                let ibo = zeroed_buffer(&self.device, "arena.ibo", cap_i * 4, iu);
                if self.arena_vert_count > 0 {
                    // the prefix moves GPU-side; it never travels back through wasm memory
                    let mut enc = self.device.create_command_encoder(&Default::default());
```

**Replace with:**

```rust
                let vbo = zeroed_buffer(&self.ctx.device, "arena.vbo", cap_v * vstride, vu);
                let vids = zeroed_buffer(&self.ctx.device, "arena.vids", cap_v * 4, vu);
                let ibo = zeroed_buffer(&self.ctx.device, "arena.ibo", cap_i * 4, iu);
                if self.arena_vert_count > 0 {
                    // the prefix moves GPU-side; it never travels back through wasm memory
                    let mut enc = self.ctx.device.create_command_encoder(&Default::default());
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    self.queue.submit([enc.finish()]);
```

**Replace with:**

```rust
                    self.ctx.queue.submit([enc.finish()]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.queue.write_buffer(&self.arena_vbo, self.arena_vert_count as u64 * vstride, bytemuck::cast_slice(&up.verts));
            self.queue.write_buffer(&self.arena_vids, self.arena_vert_count as u64 * 4, bytemuck::cast_slice(&up.vids));
            self.queue.write_buffer(&self.arena_ibo, self.arena_index_count as u64 * 4, bytemuck::cast_slice(&up.idx));
```

**Replace with:**

```rust
            self.ctx.queue.write_buffer(&self.arena_vbo, self.arena_vert_count as u64 * vstride, bytemuck::cast_slice(&up.verts));
            self.ctx.queue.write_buffer(&self.arena_vids, self.arena_vert_count as u64 * 4, bytemuck::cast_slice(&up.vids));
            self.ctx.queue.write_buffer(&self.arena_ibo, self.arena_index_count as u64 * 4, bytemuck::cast_slice(&up.idx));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            append_index_run(&self.device, &self.queue, "arena.ibo.print",
                &mut self.arena_ibo_print, &mut self.arena_print_count, &mut self.arena_print_cap, &up.idx_print);
            append_index_run(&self.device, &self.queue, "arena.ibo.text",
                &mut self.arena_ibo_text, &mut self.arena_text_count, &mut self.arena_text_cap, &up.idx_text);
```

**Replace with:**

```rust
            self.arena_ibo_print.append(&self.ctx, &up.idx_print);
            self.arena_ibo_text.append(&self.ctx, &up.idx_text);
```

Four lanes, one shape: `append` returns whether the buffer was replaced.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if append_rows(&self.device, &self.queue, "pipes.buffer",
            &mut self.pipe_buffer, &mut self.pipe_count, &mut self.pipe_cap, &up.pipes) {
            self.pipe_bind_group = Self::mk_rows_group(&self.device, &self.layouts.segment, "pipes.bind_group", &self.pipe_buffer);
        }
        if append_rows(&self.device, &self.queue, "segments.buffer",
            &mut self.segment_buffer, &mut self.segment_count, &mut self.segment_cap, &up.segments) {
            self.segment_bind_group = Self::mk_rows_group(&self.device, &self.layouts.segment, "segments.bind_group", &self.segment_buffer);
        }
        if append_rows(&self.device, &self.queue, "spheres.buffer",
            &mut self.sphere_buffer, &mut self.sphere_count, &mut self.sphere_cap, &up.spheres) {
            self.sphere_bind_group = Self::mk_rows_group(&self.device, &self.layouts.glyph, "spheres.bind_group", &self.sphere_buffer);
        }
        if append_rows(&self.device, &self.queue, "glyphs.buffer",
            &mut self.glyph_buffer, &mut self.glyph_count, &mut self.glyph_cap, &up.glyphs) {
            self.glyph_bind_group = Self::mk_rows_group(&self.device, &self.layouts.glyph, "glyphs.bind_group", &self.glyph_buffer);
```

**Replace with:**

```rust
        if self.pipes.append(&self.ctx, &up.pipes) {
            self.pipe_bind_group = rows_group(&self.ctx, &self.layouts.segment, "pipes.bind_group", &self.pipes.buf);
        }
        if self.segments.append(&self.ctx, &up.segments) {
            self.segment_bind_group = rows_group(&self.ctx, &self.layouts.segment, "segments.bind_group", &self.segments.buf);
        }
        if self.spheres.append(&self.ctx, &up.spheres) {
            self.sphere_bind_group = rows_group(&self.ctx, &self.layouts.glyph, "spheres.bind_group", &self.spheres.buf);
        }
        if self.glyphs.append(&self.ctx, &up.glyphs) {
            self.glyph_bind_group = rows_group(&self.ctx, &self.layouts.glyph, "glyphs.bind_group", &self.glyphs.buf);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let mut pos_rows = self.point_count * 3;
        append_rows(&self.device, &self.queue, "points.buffer",
            &mut self.point_buffer, &mut pos_rows, &mut self.point_cap, &up.cloud_pos);
        let mut col_rows = self.point_count;
        append_rows(&self.device, &self.queue, "points.col.buffer",
            &mut self.point_col_buffer, &mut col_rows, &mut self.point_col_cap, &up.cloud_col);
        let mut nrm_rows = self.point_count;
        append_rows(&self.device, &self.queue, "points.nrm.buffer",
            &mut self.point_nrm_buffer, &mut nrm_rows, &mut self.point_nrm_cap, &up.cloud_nrm);
        self.point_count = pos_rows / 3;
```

**Replace with:**

```rust
        self.point_pos.append(&self.ctx, &up.cloud_pos);
        self.point_col.append(&self.ctx, &up.cloud_col);
        self.point_nrm.append(&self.ctx, &up.cloud_nrm);
        self.point_count = self.point_pos.len() / 3;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.instances.len(), self.arena_vert_count, self.pipe_count + self.segment_count, self.pipe_count,
            self.sphere_count + self.glyph_count, self.sphere_count, self.point_count
```

**Replace with:**

```rust
            self.instances.len(), self.arena_vert_count, self.pipes.len() + self.segments.len(), self.pipes.len(),
            self.spheres.len() + self.glyphs.len(), self.spheres.len(), self.point_count
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if samples != self.samples {
            self.samples = samples;
            self.depth_view = Self::create_depth_view(&self.device, &self.config, samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, samples);
            self.pipelines = Pipelines::new(&self.device, Target { format: self.config.format, samples }, &self.layouts);
```

**Replace with:**

```rust
        if samples != self.targets.samples {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
            self.pipelines = Pipelines::new(&self.ctx.device, Target { format: self.config.format, samples }, &self.layouts);
```

Remove the dead pre-anchor upload comment.

**Remove** `src/engine/gpu/mod.rs` `        // let shift = Xform::translation(-origin[0], -origin[1], -origin[2]);` **through** `        // self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));`

**Find** in `src/engine/gpu/mod.rs`:

```rust
        }
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
```

**Replace with:**

```rust
        }
        self.ctx.queue.write_buffer(&self.instance_buffer.buf, 0, bytemuck::cast_slice(&self.instances));
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

    /// Re-point the five splat bind groups at the current buffers and targets (set_scene, resize, stream growth).
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.device, &self.layouts.splat_group0, &self.mvp_buffer, &self.cloud_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.device, &self.layouts.splat_group1, &self.point_buffer, &self.point_col_buffer, &self.point_nrm_buffer);
        self.splat_group0_stream = Self::mk_splat_group0(&self.device, &self.layouts.splat_group0, &self.mvp_buffer, &self.cloud_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.device, &self.layouts.splat_group1, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.device, &self.layouts.splat_resolve, &self.splat_depth_view, &self.splat_color_view);
```

**Replace with:**

```rust
    /// Re-point the five splat bind groups at the current buffers and targets (set_scene, resize, stream growth).
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.point_pos.buf, &self.point_col.buf, &self.point_nrm.buf);
        self.splat_group0_stream = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.ctx.device, &self.layouts.splat_resolve, &self.splat_depth_view, &self.splat_color_view);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let pos = zeroed_buffer(&self.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&self.device, "stream.nrm", cap * 4, usage);
        if self.stream_count > 0 {
            let mut enc = self.device.create_command_encoder(&Default::default());
```

**Replace with:**

```rust
        let pos = zeroed_buffer(&self.ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&self.ctx.device, "stream.nrm", cap * 4, usage);
        if self.stream_count > 0 {
            let mut enc = self.ctx.device.create_command_encoder(&Default::default());
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.queue.submit([enc.finish()]);
```

**Replace with:**

```rust
            self.ctx.queue.submit([enc.finish()]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            self.queue.submit([]);
```

**Replace with:**

```rust
            self.ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            self.ctx.queue.submit([]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.queue.write_buffer(&self.stream_pos_buf, self.stream_pos_at as u64 * 12, bytemuck::cast_slice(pos));
```

**Replace with:**

```rust
        self.ctx.queue.write_buffer(&self.stream_pos_buf, self.stream_pos_at as u64 * 12, bytemuck::cast_slice(pos));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.queue.submit([]);
```

**Replace with:**

```rust
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.ctx.queue.submit([]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.queue.write_buffer(&self.stream_col_buf, self.stream_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.stream_col_at += col.len() as u32;
        self.queue.submit([]);
```

**Replace with:**

```rust
        self.ctx.queue.write_buffer(&self.stream_col_buf, self.stream_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.stream_col_at += col.len() as u32;
        self.ctx.queue.submit([]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if let Some(s) = &self.surface { s.configure(&self.device, &self.config); }
            self.depth_view = Self::create_depth_view(&self.device, &self.config, self.samples);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config, self.samples);
            let (d, c) = Self::create_splat_targets(&self.device, &self.config);
```

**Replace with:**

```rust
            if let Some(s) = &self.surface { s.configure(&self.ctx.device, &self.config); }
            self.targets = Targets::new(&self.ctx, &self.config, self.targets.samples);
            let (d, c) = Self::create_splat_targets(&self.ctx.device, &self.config);
```

`clear`, `render_offscreen` and `bench_frames` live in `present.rs`. Remove through
`write_frame_uniforms`'s last statement, leaving its closing brace; the Add below supplies the
new two-line body up to that brace.

**Remove** `src/engine/gpu/mod.rs` **through**

```rust
    /// Acquire the next frame and clear it to `color`. Chapter 1 does nothing else — geometry passes
```

```rust
        self.update_inside_flags(view_proj);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.splat_state = None;

        }
    }

```

**Add below it:**

```rust
    /// Per-frame uniforms through `FrameUniforms::write`, then the inside-flag refresh, which
    /// reads the eye it solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let anchor = self.last_origin.as_ref().map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3]);
        let cx = FrameCx { view: &self.view, anchor, size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
        self.update_inside_flags();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    fn update_inside_flags(&mut self, view_proj: &Xform) {
```

**Replace with:**

```rust
    fn update_inside_flags(&mut self) {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let eye = eye_from_view_proj(view_proj); // anchored world units, like instances[]
```

**Replace with:**

```rust
        let eye = self.frame.eye; // anchored world units, like instances[]
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
```

**Replace with:**

```rust
            self.ctx.queue.write_buffer(&self.instance_buffer.buf, 0, bytemuck::cast_slice(&self.instances));
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let ortho_h = self.last_ortho_h as f64;
        let vp_h = self.config.height as f64;
        let aspect = self.config.width as f64 / self.config.height as f64;
        let eye = self.last_eye;
```

**Replace with:**

```rust
        let ortho_h = self.frame.ortho_h as f64;
        let vp_h = self.config.height as f64;
        let aspect = self.config.width as f64 / self.config.height as f64;
        let eye = self.frame.eye;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.cloud_size;
```

**Replace with:**

```rust
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.view.cloud_size;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let (a, b) = (&self.mvp_f32, &row.model);
```

**Replace with:**

```rust
            let (a, b) = (&self.frame.mvp_f32, &row.model);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.lod_split_px > 0.0 && node_count > 0 {
```

**Replace with:**

```rust
            if self.view.lod_split_px > 0.0 && node_count > 0 {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                    let refine = !leaf && sp_px > self.lod_split_px as f64;
```

**Replace with:**

```rust
                    let refine = !leaf && sp_px > self.view.lod_split_px as f64;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let state = (self.mvp_f32, self.cloud_size);
            if self.splat_total > 0 && self.splat_state != Some(state) && !skip("splat_points") {
                self.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.queue.write_buffer(&self.splat_recs, 16, &recs);
                self.queue.write_buffer(&self.splat_stream_recs, 0, bytemuck::bytes_of(&header_s));
                self.queue.write_buffer(&self.splat_stream_recs, 16, &recs_s);
```

**Replace with:**

```rust
            let state = (self.frame.mvp_f32, self.view.cloud_size);
            if self.splat_total > 0 && self.splat_state != Some(state) && !self.view.skip("splat_points") {
                self.ctx.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.ctx.queue.write_buffer(&self.splat_recs, 16, &recs);
                self.ctx.queue.write_buffer(&self.splat_stream_recs, 0, bytemuck::bytes_of(&header_s));
                self.ctx.queue.write_buffer(&self.splat_stream_recs, 16, &recs_s);
```

The lane switches read the `View` now; the free `skip` helper goes. **Find** in `src/engine/gpu/mod.rs`:

```rust
            if !skip("background") {
```

**Replace with:**

```rust
            if !self.view.skip("background") {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if !skip("grid") { pass.draw(0..50, 0..1); }
```

**Replace with:**

```rust
            if !self.view.skip("grid") { pass.draw(0..50, 0..1); }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.arena_index_count > 0 && !skip("arena") {
```

**Replace with:**

```rust
            if self.arena_index_count > 0 && !self.view.skip("arena") {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.splat_total > 0 && !skip("splat") {
```

**Replace with:**

```rust
            if self.splat_total > 0 && !self.view.skip("splat") {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
/// Native profiling knob: `VIEWER_SKIP=pipes,ribbon,...` leaves those lanes out of the frame so
/// `examples/bench_frame` can price each one by subtraction. Never read on wasm.
#[cfg(not(target_arch = "wasm32"))]
fn skip(lane: &str) -> bool {
    static LIST: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    LIST.get_or_init(|| std::env::var("VIEWER_SKIP").unwrap_or_default().split(',').map(|s| s.trim().to_string()).collect())
        .iter().any(|l| l == lane)
}
#[cfg(target_arch = "wasm32")]
fn skip(_lane: &str) -> bool { false }
```

**Delete**

One `Binds` before the pass; every draw below reads `b.mvp`, `b.line`, `b.instances`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        {
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
        let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.instance_bind_group };
        {
            let mut pass = self.targets.begin_pass(encoder, view, color);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);   // for the anchor
```

**Replace with:**

```rust
            pass.set_bind_group(0, b.mvp, &[]);
            pass.set_bind_group(1, b.line, &[]);   // for the anchor
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.line_bind_group, &[]);
            pass.set_bind_group(2, &self.instance_bind_group, &[]);
```

**Replace with:**

```rust
            pass.set_bind_group(0, b.mvp, &[]);
            pass.set_bind_group(1, b.line, &[]);
            pass.set_bind_group(2, b.instances, &[]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.arena_print_count > 0 && !skip("fills") {
```

**Replace with:**

```rust
            if !self.arena_ibo_print.is_empty() && !self.view.skip("fills") {
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_index_buffer(self.arena_ibo_print.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_print_count, 0, 0..1);
```

**Replace with:**

```rust
                pass.set_index_buffer(self.arena_ibo_print.buf.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_ibo_print.len(), 0, 0..1);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.pipe_count > 0 && self.show_mesh_edges && !skip("pipes") {
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.pipe_bind_group, &[]);
                match self.line_style {
                    LineStyle::Tubes => {
                        pass.set_pipeline(&self.pipelines.cylinder);
                        pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                        pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count); // one template, N edges
```

**Replace with:**

```rust
            if !self.pipes.is_empty() && self.view.show_mesh_edges && !self.view.skip("pipes") {
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.pipe_bind_group, &[]);
                match self.view.line_style {
                    LineStyle::Tubes => {
                        pass.set_pipeline(&self.pipelines.cylinder);
                        pass.set_vertex_buffer(0, self.cyl_template.vbo.slice(..));
                        pass.set_index_buffer(self.cyl_template.ibo.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.cyl_template.index_count, 0, 0..self.pipes.len()); // one template, N edges
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                        pass.draw(0..6 * self.pipe_count, 0..1);
                        pass.set_pipeline(&self.pipelines.ribbon_solid);
                        pass.draw(0..6 * self.pipe_count, 0..1);
```

**Replace with:**

```rust
                        pass.draw(0..6 * self.pipes.len(), 0..1);
                        pass.set_pipeline(&self.pipelines.ribbon_solid);
                        pass.draw(0..6 * self.pipes.len(), 0..1);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
                pass.set_bind_group(0, &self.cloud_bind_group, &[]);
```

**Replace with:**

```rust
                pass.set_bind_group(0, &self.frame.cloud_group, &[]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.sphere_count > 0 && self.show_mesh_edges && !skip("spheres") {
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.sphere_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                // Same prepass split as the solid ribbons - see the LineStyle::Flat note above.
                pass.set_pipeline(&self.pipelines.sphere_depth);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count);
                pass.set_pipeline(&self.pipelines.sphere);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count); // one template, N glyphs
```

**Replace with:**

```rust
            if !self.spheres.is_empty() && self.view.show_mesh_edges && self.view.markers && !self.view.skip("spheres") {
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.sphere_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template.vbo.slice(..));
                pass.set_index_buffer(self.sph_template.ibo.slice(..), wgpu::IndexFormat::Uint32);
                // Same prepass split as the solid ribbons - see the LineStyle::Flat note above.
                pass.set_pipeline(&self.pipelines.sphere_depth);
                pass.draw_indexed(0..self.sph_template.index_count, 0, 0..self.spheres.len());
                pass.set_pipeline(&self.pipelines.sphere);
                pass.draw_indexed(0..self.sph_template.index_count, 0, 0..self.spheres.len()); // one template, N glyphs
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if INK_DEPTH_PREPASS && self.segment_count > 0 && self.show_lines && !skip("ribbon_depth") {
                pass.set_pipeline(&self.pipelines.ribbon_depth);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.draw(0..6 * self.segment_count, 0..1); // 6 verts/segment, see ribbon.wgsl vs_main
                draws += 1;
            }
            if INK_DEPTH_PREPASS && self.glyph_count > 0 && self.show_points && !skip("glyph_depth") {
                pass.set_pipeline(&self.pipelines.glyph_depth);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyph_count, 0..1);
```

**Replace with:**

```rust
            if INK_DEPTH_PREPASS && !self.segments.is_empty() && self.view.show_lines && !self.view.skip("ribbon_depth") {
                pass.set_pipeline(&self.pipelines.ribbon_depth);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.draw(0..6 * self.segments.len(), 0..1); // 6 verts/segment, see ribbon.wgsl vs_main
                draws += 1;
            }
            if INK_DEPTH_PREPASS && !self.glyphs.is_empty() && self.view.show_points && !self.view.skip("glyph_depth") {
                pass.set_pipeline(&self.pipelines.glyph_depth);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyphs.len(), 0..1);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.segment_count > 0 && self.show_lines && !skip("ribbon") {
                pass.set_pipeline(&self.pipelines.ribbon);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                // instance_index IS the row: this table holds nothing but flat-lane segments
                pass.draw(0..6 * self.segment_count, 0..1); // 6 verts/segment, see ribbon.wgsl vs_main
```

**Replace with:**

```rust
            if !self.segments.is_empty() && self.view.show_lines && !self.view.skip("ribbon") {
                pass.set_pipeline(&self.pipelines.ribbon);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                // vertex_index / 6 IS the row: this table holds nothing but flat-lane segments
                pass.draw(0..6 * self.segments.len(), 0..1); // 6 verts/segment, see ribbon.wgsl vs_main
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.arena_text_count > 0 && !skip("text") {
                pass.set_pipeline(&self.pipelines.triangle_sheet);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_text.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_text_count, 0, 0..1);
```

**Replace with:**

```rust
            if !self.arena_ibo_text.is_empty() && !self.view.skip("text") {
                pass.set_pipeline(&self.pipelines.triangle_sheet);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_text.buf.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_ibo_text.len(), 0, 0..1);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.glyph_count > 0 && self.show_points && !skip("glyph") {
                pass.set_pipeline(&self.pipelines.glyph);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
```

**Replace with:**

```rust
            if !self.glyphs.is_empty() && self.view.show_points && !self.view.skip("glyph") {
                pass.set_pipeline(&self.pipelines.glyph);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyphs.len(), 0..1); // 3 verts/dot, no template
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    }

    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
```

**Replace with:**

```rust
    }

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena_print_count = 0;
        self.arena_text_count = 0;
```

**Replace with:**

```rust
        self.arena_ibo_print.reset();
        self.arena_ibo_text.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.pipe_count = 0;
        self.segment_count = 0;
        self.sphere_count = 0;
        self.glyph_count = 0;
```

**Replace with:**

```rust
        self.pipes.reset();
        self.segments.reset();
        self.spheres.reset();
        self.glyphs.reset();
        self.point_pos.reset();
        self.point_col.reset();
        self.point_nrm.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.instance_rows = 0;
```

**Replace with:**

```rust
        self.instance_buffer.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.bounded_rows.clear();
    }

```

**Add below it:**

```rust
    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let solid = self.arena_vert_count > 0 || self.pipe_count > 0 || self.sphere_count > 0;
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
```

**Replace with:**

```rust
        let solid = self.arena_vert_count > 0 || !self.pipes.is_empty() || !self.spheres.is_empty();
        if solid { 4 } else { 1 }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Vertex spacing in world units (see `ArenaUpload::object_spacing`). The ink lanes drop
```

**Replace with:**

```rust
    /// Vertex spacing in world units (see `Upload::object_spacing`). The ink lanes drop
```

`LineUniform` moved to `frame.rs`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
}                       // 40 B

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    // Camera position, in the SAME anchored frame the instance rows use - so a shader can build
    // the view ray to a point as `eye - p`. That is what the per-edge facing test needs, and it
    // has to be the real eye rather than a constant forward direction: at this 60 degree FOV a
    // constant direction is off by up to 30 degrees at the frame corner, and it is precisely the
    // near-silhouette edges - the ones whose classification is in doubt - that would flip.
    eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
    // The camera-relative ANCHOR, world units. Instance rows are rebased about it, so anything
    // NOT an instance - the grid, the axes - has to subtract it too or it drifts away from the
    // scene every time re-anchoring fires.
    anchor: [f32; 3],
    _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
} // 48 B - three vec4s

// The shaders declare this same struct with `anchor: vec3<f32>`, which WGSL aligns to 16 - so the
// uniform is 48 B there, not the 32 B a naive Rust layout gives. A mismatch is not a compile error:
// it surfaces at run time as "buffer bound with size 32 ... requires at least 48 bytes", every
// frame, from every pipeline that binds group 1.
const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);

```

**Replace with:**

```rust
}                       // 40 B

```

`CloudUniform` moved to `frame.rs`; the `GlyphPoint` size assert rides along with this Remove
and comes back in the next edit.

**Remove** `src/engine/gpu/mod.rs` `const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);` **through** `} // 16 B - one vec4; its own buffer + bind group`

**Find** in `src/engine/gpu/mod.rs`:

```rust
// array stride is the struct's, so a drift here misreads every row.
```

**Add below it:**

```rust
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);
```

`line_thickness_px` and `zeroed_buffer` are in `view.rs` / `buffers.rs` now.

**Find** in `src/engine/gpu/mod.rs`:

```rust
}

/// A fresh buffer of `size` bytes, zero-initialized by WebGPU - the write_buffer splice and the empty-category placeholders both rely on that guarantee.
/// On-screen pen weight in px. Default 2.0.
///
/// It was briefly 1.0, to stop an embedded viewer reading as a blob of ink. That trades one
/// problem for a worse one: a tube is opaque GEOMETRY, and 4x MSAA gives a pixel four coverage
/// samples - enough to smooth the edge of a shape that covers it, nothing at all for a shape
/// THINNER than it. A 1 px pen lands on one or two samples and resolves dim and broken, and the
/// density taper below (`WIRE_MIN_PENS`) can thin it to 0.15 of that again on a dense mesh. Two
/// pixels is the floor at which MSAA has something to work with.
///
/// Tune per embed with `?thickness=1.5` rather than rebuilding, the same query-string mechanism
/// as `?scene=`; `VIEWER_THICKNESS` does the same for native (env vars are unreachable on wasm).
fn line_thickness_px() -> f32 {
    #[cfg(target_arch = "wasm32")]
    {
        static PX: std::sync::OnceLock<f32> = std::sync::OnceLock::new();
        return *PX.get_or_init(|| {
            web_sys::window()
                .and_then(|w| w.location().search().ok())
                .and_then(|search| {
                    search
                        .trim_start_matches('?')
                        .split('&')
                        .find_map(|pair| pair.strip_prefix("thickness=").map(str::to_owned))
                })
                .and_then(|value| value.parse().ok())
                .filter(|px: &f32| px.is_finite() && *px > 0.0)
                .unwrap_or(2.0)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("VIEWER_THICKNESS").ok().and_then(|v| v.parse().ok()).unwrap_or(2.0)
    }
}

fn zeroed_buffer(
    device: &wgpu::Device,
    label: &str,
    size: u64,
    usage: wgpu::BufferUsages
) -> wgpu::Buffer {
    device.create_buffer(
        &wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
}


```

**Replace with:**

```rust
}

```

## Step 9 — `src/app/scene.rs`

`Upload::default()` and `drop_uploaded()` replace the constructor and the fourteen `drop_rows`.

**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint};
```

**Replace with:**

```rust
use crate::engine::gpu::{Upload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint};
```

**Find** in `src/app/scene.rs`:

```rust
    pub tables: ArenaUpload,
```

**Replace with:**

```rust
    pub tables: Upload,
```

**Find** in `src/app/scene.rs`:

```rust
        tables: ArenaUpload::new(),
```

**Replace with:**

```rust
        tables: Upload::default(),
```

**Find** in `src/app/scene.rs`:

```rust
        self.docs.clear();
        self.tables = ArenaUpload::new();
```

**Replace with:**

```rust
        self.docs.clear();
        self.tables = Upload::default();
```

**Find** in `src/app/scene.rs`:

```rust
        self.tables = ArenaUpload::new();
```

**Replace with:**

```rust
        self.tables = Upload::default();
```

**Find** in `src/app/scene.rs`:

```rust
    /// Upload the walked tables, then FORGET the rows: the GPU is their only holder.
    ///
    /// EVERY drawn table goes, not just the arena. Nothing reads any of them back - picking goes
    /// through the kernel Meshes in Doc.session, never through these flattened rows - and holding
    /// them cost twice over: the wasm heap kept a full second copy of the scene for the whole
    /// session (280 MB on a 13.8 M-point scan), and having that copy is exactly what let the ink
    /// and cloud lanes rebuild their whole buffer per file instead of appending. Keep only the
    /// running bases, so the next file's indices still land in the right place.
```

**Replace with:**

```rust
    /// Upload the walked tables, then FORGET the rows (`Upload::drop_uploaded`): the GPU is
    /// their only holder. Only the running bases stay, so the next file's indices still land in
    /// the right place.
```

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

**Remove** `src/app/scene.rs` **up to**

```rust
/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
```

```rust
/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
```

## Step 10 — `src/state.rs`

`render` builds the `FrameInput`.

**Find** in `src/state.rs`:

```rust
use crate::engine::gpu::Gpu;
```

**Replace with:**

```rust
use crate::engine::gpu::{FrameInput, Gpu};
```

**Find** in `src/state.rs`:

```rust
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj)
```

**Replace with:**

```rust
        let clear = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

        self.gpu.clear(&FrameInput { view_proj, clear })
```

## Step 11 — `src/lib.rs`

The key handlers flip `gpu.view.*`.

**Find** in `src/lib.rs`:

```rust
                            state.gpu.show_points = !state.gpu.show_points;
                            log::info!("points: {}", state.gpu.show_points);
```

**Replace with:**

```rust
                            state.gpu.view.show_points = !state.gpu.view.show_points;
                            log::info!("points: {}", state.gpu.view.show_points);
```

**Find** in `src/lib.rs`:

```rust
                            state.gpu.show_lines = !state.gpu.show_lines;
                            log::info!("lines: {}", state.gpu.show_lines);
```

**Replace with:**

```rust
                            state.gpu.view.show_lines = !state.gpu.view.show_lines;
                            log::info!("lines: {}", state.gpu.view.show_lines);
```

**Find** in `src/lib.rs`:

```rust
                            state.gpu.show_mesh_edges = !state.gpu.show_mesh_edges;
                            log::info!("mesh edges: {}", state.gpu.show_mesh_edges);
```

**Replace with:**

```rust
                            state.gpu.view.show_mesh_edges = !state.gpu.view.show_mesh_edges;
                            log::info!("mesh edges: {}", state.gpu.view.show_mesh_edges);
```

**Find** in `src/lib.rs`:

```rust
                            state.gpu.line_style = match state.gpu.line_style {
```

**Replace with:**

```rust
                            state.gpu.view.line_style = match state.gpu.view.line_style {
```

**Find** in `src/lib.rs`:

```rust
                            log::info!("line style: {:?}", state.gpu.line_style);
```

**Replace with:**

```rust
                            log::info!("line style: {:?}", state.gpu.view.line_style);
```

**Find** in `src/lib.rs`:

```rust
                            state.gpu.cloud_size = (state.gpu.cloud_size - 0.25).max(0.25);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
                        }
                        Key::Character("]") => {
                            state.gpu.cloud_size = (state.gpu.cloud_size + 0.25).min(8.0);
                            log::info!("cloud size scale: x{}", state.gpu.cloud_size);
```

**Replace with:**

```rust
                            state.gpu.view.cloud_size = (state.gpu.view.cloud_size - 0.25).max(0.25);
                            log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
                        }
                        Key::Character("]") => {
                            state.gpu.view.cloud_size = (state.gpu.view.cloud_size + 0.25).min(8.0);
                            log::info!("cloud size scale: x{}", state.gpu.view.cloud_size);
```

## Step 12 — `src/selftest.rs`

The harness builds `FrameInput`s and reads `gpu.ctx` / `gpu.view`.

**Find** in `src/selftest.rs`:

```rust
use crate::engine::gpu::Gpu;
```

**Replace with:**

```rust
use crate::engine::gpu::{FrameInput, Gpu};
```

**Find** in `src/selftest.rs`:

```rust
    let view_proj = camera.view_proj_anchored(w as f64 / h as f64, &anchor);
```

**Add below it:**

```rust
    let input = FrameInput { view_proj, clear: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 } };
```

**Find** in `src/selftest.rs`:

```rust
        let solved = crate::math::eye_from_view_proj(&view_proj);
```

**Replace with:**

```rust
        let solved = crate::math::eye_from_view_proj(&input.view_proj);
```

**Find** in `src/selftest.rs`:

```rust
            let _ = gpu.render_offscreen(wgpu::Color {r: 0.9, g: 0.9, b:0.9, a:1.0}, &view_proj);
```

**Replace with:**

```rust
            let _ = gpu.render_offscreen(&input);
```

**Find** in `src/selftest.rs`:

```rust
            n, ms[n / 2], 1000.0 / ms[n / 2], ms[0], ms[n - 1], gpu.cloud_size);
    }

    let rgba = gpu.render_offscreen(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj);
```

**Replace with:**

```rust
            n, ms[n / 2], 1000.0 / ms[n / 2], ms[0], ms[n - 1], gpu.view.cloud_size);
    }

    let rgba = gpu.render_offscreen(&input);
```

**Find** in `src/selftest.rs`:

```rust
            gpu.line_style = style;
```

**Replace with:**

```rust
            gpu.view.line_style = style;
```

**Find** in `src/selftest.rs`:

```rust
    let tex = gpu.device.create_texture(&wgpu::TextureDescriptor {
```

**Replace with:**

```rust
    let tex = gpu.ctx.device.create_texture(&wgpu::TextureDescriptor {
```

**Find** in `src/selftest.rs`:

```rust
            let t0 = std::time::Instant::now();
            let _ = gpu.clear(clear, &vp); // headless: uniforms, then returns
            let t1 = std::time::Instant::now();
            let mut encoder = gpu.device.create_command_encoder(&Default::default());
            gpu.encode_frame(&mut encoder, &view, clear);
            let t2 = std::time::Instant::now();
            gpu.queue.submit([encoder.finish()]);
            let _ = gpu.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
```

**Replace with:**

```rust
            let input = FrameInput { view_proj: vp, clear };
            let t0 = std::time::Instant::now();
            let _ = gpu.clear(&input); // headless: uniforms, then returns
            let t1 = std::time::Instant::now();
            let mut encoder = gpu.ctx.device.create_command_encoder(&Default::default());
            gpu.encode_frame(&mut encoder, &view, clear);
            let t2 = std::time::Instant::now();
            gpu.ctx.queue.submit([encoder.finish()]);
            let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 0 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 0 warnings
grep -c 'std::env::var' src/engine/gpu/mod.rs                # 0
grep -c ': GrowBuf' src/engine/gpu/mod.rs                    # 10 tables
./docs/_gate.sh                                              # gate OK
```

`Gpu` has 64 fields (was 102); the goldens do not move.

## Recap

- A growable table is one type, `GrowBuf`; a lane owns one and asks it to `append`.
- Knobs are read once into `View`; nothing in the frame loop touches the environment.
- The frame's inputs are two structs (`FrameInput`, `FrameCx`); the eye is solved once.

## Next

Lesson [46](47-row-families.md) — row families: `objects.rs`, `arena.rs`, `segments.rs`,
`glyphs.rs`, and the first four tests.
