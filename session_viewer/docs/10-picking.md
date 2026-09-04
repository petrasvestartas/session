# 10 Picking with an id buffer

- A left click names what is under the cursor: the document, the geometry's guid, its object row and, in a cloud, the point (its index in the file, its stable id, its position). The console prints it and the object turns amber; Escape clears it.
- The GPU answers, not a ray cast: on request every lane redraws once, opaque, at 1x, into an `Rg32Uint` target that holds `(object row + 1, sub-object id + 1)` per pixel, and one texel is copied out and mapped asynchronously.
- The answer lands a frame later, so the frame loop keeps running while a pick is in flight (`Picker::busy`) and `State::apply_pick` selects when it arrives.
- A streamed cloud never existed on the CPU, so nothing on the CPU can be asked; the point lane writes its point row into the second channel and the pick names the point from what was drawn.
- The id pass is the scene list again, under the same toggles and in the same order: what a lane hides it cannot pick.
- Selection is one bit on the object row (`FLAG_SELECTED`); every shader mixes `SELECT_COLOR` in when it sees it, and the point lane bakes it into its records.

<svg viewBox="0 0 720 330" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="app/ asks for a pixel and gets a Pick back over the seam; engine/ draws the id pass through every lane's id pipeline and reads one texel" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="pk" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#333"/></marker></defs>
  <g fill="#222" font-size="11">
    <text x="14" y="18">app/</text><text x="360" y="18" text-anchor="middle">the seam</text><text x="706" y="18" text-anchor="end">engine/</text>
  </g>
  <g fill="none" stroke="#333">
    <rect x="14" y="30" width="216" height="52"/><rect x="14" y="94" width="216" height="66"/><rect x="14" y="172" width="216" height="66"/>
    <rect x="490" y="30" width="216" height="40"/><rect x="490" y="82" width="216" height="66"/><rect x="490" y="160" width="216" height="52"/>
    <rect x="490" y="224" width="216" height="52"/><rect x="490" y="288" width="216" height="34"/>
  </g>
  <g fill="#222" font-size="11">
    <text x="22" y="46">input.rs</text><text x="22" y="110">state.rs</text><text x="22" y="188">scene.rs</text>
    <text x="498" y="46">pipelines/mod.rs</text><text x="498" y="98">gpu/pick.rs</text><text x="498" y="176">gpu/render.rs</text>
    <text x="498" y="240">lanes + shaders</text><text x="498" y="304">gpu/objects.rs</text>
  </g>
  <g fill="#666" font-size="10">
    <text x="22" y="61">left(): press, release within 4 px</text><text x="22" y="75">Escape: select(None)</text>
    <text x="22" y="125">request_pick(x, y)</text><text x="22" y="139">select(row) / apply_pick(pick)</text><text x="22" y="153">render: pick.poll() every frame</text>
    <text x="22" y="203">Picked { doc, guid, row, point }</text><text x="22" y="217">resolve(Pick) -&gt; doc_of, point_at</text><text x="22" y="231">selected: Option&lt;u32&gt;</text>
    <text x="498" y="61">Target::ID = Rg32Uint, 1 sample</text>
    <text x="498" y="113">Picker: request, take_pending</text><text x="498" y="127">begin_pass, copy_texel, map</text><text x="498" y="141">poll -&gt; Option&lt;Option&lt;Pick&gt;&gt;</text>
    <text x="498" y="191">id_pass: the scene list again</text><text x="498" y="205">opaque, 1x, same toggles</text>
    <text x="498" y="255">draw_*_ids through id pipelines</text><text x="498" y="269">fs_id -&gt; (row+1, 0) · splat (row+1, pt+1)</text>
    <text x="498" y="318">set_flag(row, FLAG_SELECTED)</text>
  </g>
  <g stroke="#333" marker-end="url(#pk)">
    <line x1="230" y1="120" x2="488" y2="106"/>
    <line x1="488" y1="134" x2="230" y2="146"/>
    <line x1="230" y1="228" x2="488" y2="304"/>
  </g>
  <g fill="#444" font-size="10">
    <text x="360" y="104" text-anchor="middle">gpu.pick.request(x, y)</text>
    <text x="360" y="152" text-anchor="middle">Option&lt;Pick { row, sub }&gt;</text>
    <text x="360" y="260" text-anchor="middle">gpu.set_selected(row, on)</text>
  </g>
  <g fill="none" stroke="#333" stroke-dasharray="4 3"><rect x="254" y="30" width="212" height="40"/></g>
  <text x="360" y="54" fill="#666" font-size="10" text-anchor="middle">Upload: unchanged this lesson</text>
</svg>

## Step 1 - Name the id target

- A pipeline is built for a `Target` (format, samples); the id pass has its own: two unsigned 32-bit channels, never multisampled, so a texel is an exact `(row + 1, sub + 1)` and 0 is the background.

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}
```

**Add below it:**

```rust

impl Target {
    /// The id pass: (object row + 1, sub-object id + 1) per pixel, never multisampled.
    pub const ID: Target = Target { format: wgpu::TextureFormat::Rg32Uint, samples: 1 };
}
```

## Step 2 - Create the picker

- `Picker` is the whole round trip in one struct: the pending pixel, the id targets (made on the first pick, dropped on resize), a 256-byte readback buffer and a flag the map callback flips. It knows no lane.
- `map` runs once per copy and never twice: a second `map_async` on a buffer still mapped is a wgpu panic, so a click during a pick in flight is dropped in `request`.

_Type it._
**Create `src/engine/gpu/pick.rs`**

```rust
//! Picking by id pass: on request the lanes redraw ONCE at 1x into an `Rg32Uint` target -
//! (object row + 1, sub-object id + 1) per pixel - and one texel is copied out and mapped
//! asynchronously. The answer arrives a frame later from `poll`. No CPU ray cast, and it
//! works for streamed clouds that never existed on the CPU.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use super::buffers::GpuCtx;
use super::targets::{TextureSpec, texture, texture_view};

/// What a pixel answered: the object row and the sub-object id (point row for clouds, 0 else).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pick {
    pub row: u32,
    pub sub: u32,
}

/// The id pass's attachments, made on the first pick and kept until the canvas resizes.
struct IdTargets {
    id: wgpu::Texture,
    id_view: wgpu::TextureView,
    depth: wgpu::TextureView,
    size: (u32, u32),
}

/// The pending request, the targets, the readback buffer and its completion flag.
pub struct Picker {
    pending: Option<(u32, u32)>,
    inflight: bool,
    /// A copy was encoded this frame and its buffer must be mapped once the submit is in.
    copied: bool,
    ready: Arc<AtomicBool>,
    readback: Option<wgpu::Buffer>,
    targets: Option<IdTargets>,
}

impl Picker {
    /// Nothing requested, nothing allocated.
    pub fn new() -> Self {
        Self { pending: None, inflight: false, copied: false, ready: Arc::new(AtomicBool::new(false)), readback: None, targets: None }
    }

    /// Ask for the ids under pixel (x, y). Ignored while an earlier pick is still in flight.
    pub fn request(&mut self, x: u32, y: u32) {
        if !self.inflight {
            self.pending = Some((x, y));
        }
    }

    /// Whether a pick is waiting for its answer (the shell keeps frames coming until it lands).
    pub fn busy(&self) -> bool {
        self.inflight || self.pending.is_some()
    }

    /// The request to serve this frame, if any.
    pub fn take_pending(&mut self) -> Option<(u32, u32)> {
        self.pending.take()
    }

    /// Open the id pass over targets of `size`, cleared to 0 (= nothing) and reverse-Z far.
    pub fn begin_pass<'a>(&'a mut self, ctx: &GpuCtx, encoder: &'a mut wgpu::CommandEncoder, size: (u32, u32)) -> wgpu::RenderPass<'a> {
        if self.targets.as_ref().map(|t| t.size) != Some(size) {
            let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
            let id = texture(ctx, "pick.id", &TextureSpec { size, format: wgpu::TextureFormat::Rg32Uint, samples: 1, usage });
            let id_view = id.create_view(&wgpu::TextureViewDescriptor::default());
            let depth = texture_view(ctx, "pick.depth", &TextureSpec { size, format: wgpu::TextureFormat::Depth32Float, samples: 1, usage: wgpu::TextureUsages::RENDER_ATTACHMENT });
            self.targets = Some(IdTargets { id, id_view, depth, size });
        }
        let t = self.targets.as_ref().unwrap();
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pick pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &t.id_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &t.depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        })
    }

    /// Copy the texel at (x, y) into the readback buffer and start mapping it.
    pub fn copy_texel(&mut self, ctx: &GpuCtx, encoder: &mut wgpu::CommandEncoder, at: (u32, u32)) {
        let Some(t) = &self.targets else { return };
        let (x, y) = (at.0.min(t.size.0 - 1), at.1.min(t.size.1 - 1));
        let buf = self.readback.get_or_insert_with(|| readback_buffer(ctx));
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &t.id, mip_level: 0, origin: wgpu::Origin3d { x, y, z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: buf, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(256), rows_per_image: Some(1) } },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        self.inflight = true;
        self.copied = true;
    }

    /// After the copy was submitted: map the buffer ONCE; `ready` flips when the map completes.
    /// A second `map_async` on a buffer still mapped is a wgpu panic, so this is a no-op until
    /// the next copy.
    pub fn map(&mut self) {
        if !self.copied {
            return;
        }
        self.copied = false;
        let Some(buf) = &self.readback else { return };
        let flag = self.ready.clone();
        buf.slice(..).map_async(wgpu::MapMode::Read, move |_| flag.store(true, Ordering::Release));
    }

    /// Collect a pick asked for earlier: `None` while in flight, `Some(None)` for background.
    pub fn poll(&mut self) -> Option<Option<Pick>> {
        if !self.inflight || !self.ready.load(Ordering::Acquire) {
            return None;
        }
        let buf = self.readback.as_ref()?;
        let (object, sub) = {
            let bytes = buf.slice(..).get_mapped_range();
            let object = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let sub = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            (object, sub)
        };
        buf.unmap();
        self.ready.store(false, Ordering::Release);
        self.inflight = false;
        if object == 0 {
            return Some(None);
        }
        Some(Some(Pick { row: object - 1, sub: sub.saturating_sub(1) }))
    }

    /// Drop the targets (the canvas resized); they are remade on the next pick.
    pub fn resize(&mut self) {
        self.targets = None;
    }
}

/// A 256 B readback buffer - one row of copy alignment holds our 8 bytes.
fn readback_buffer(ctx: &GpuCtx) -> wgpu::Buffer {
    ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("pick.readback"),
        size: 256,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    })
}
```

## Step 3 - Let every shader answer with its row

- Each lane's fragment stage gets an `fs_id` twin that writes `inst_id + 1` where `fs_main` wrote colour; the same vertex stage feeds both, so the id lands exactly where the pixel is.
- The ink shaders keep their coverage test, so a pick hits the stroke, not the quad around it; the point lane adds its point row in the second channel.

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    return vec4<f32>(base * select(lit, 1.0, in.print > 0.5), 1.0);
}
```

**Add below it:**

```wgsl

// The id pass: (object row + 1, 0).
@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

_Type it._
**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
    return in.color;
}
```

**Add below it:**

```wgsl

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

_Type it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

**Add below it:**

```wgsl

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

_Paste it._
**Find** in `src/shaders/sphere.wgsl`:

```wgsl
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

**Add below it:**

```wgsl

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

_Paste it._
**Find** in `src/shaders/glyph.wgsl`:

```wgsl
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

**Add below it:**

```wgsl

@fragment
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) < 0.5) {
        discard;
    }
    return vec2<u32>(in.inst_id + 1u, 0u);
}
```

_Type it._
**Find** in `src/shaders/splat.wgsl`:

```wgsl
    return in.color;
}
```

**Add below it:**

```wgsl

// The id pass: (object row + 1, point row + 1).
@fragment
fn fs_point_id(in: PointOut) -> @location(0) vec2<u32> {
    if (outside(in)) {
        discard;
    }
    return vec2<u32>(in.instance + 1u, in.row + 1u);
}
```

## Step 4 - The arena's id pipelines

- The arena draws its index runs through two more pipelines built for `Target::ID`: the faces keep their depth bias, and the sheet twin reads depth with ties allowed, since nothing blends on a uint target and a fill must win or lose its pixel outright.
- `draw_face_ids` covers the faces and the print fills; `draw_text_ids` comes after the ink, as the lettering does in the colour pass.

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The two pipelines over the arena: solid faces (opaque: the shader writes alpha 1) and sheet
/// runs (blended, depth read-only).
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
    sheet: wgpu::RenderPipeline,
}
```

**Replace with:**

```rust
/// The four pipelines over the arena: solid faces (opaque: the shader writes alpha 1), sheet
/// runs (blended, depth read-only), and their id-pass twins.
struct ArenaPipelines {
    faces: wgpu::RenderPipeline,
    sheet: wgpu::RenderPipeline,
    id_faces: wgpu::RenderPipeline,
    id_sheet: wgpu::RenderPipeline,
}
```

_Type it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        self.draw_run(pass, b, &self.pipes.sheet, &self.text)
    }
```

**Add below it:**

```rust

    /// The id pass for the faces and the sheet fills, each fragment its object row.
    pub fn draw_face_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_faces, &self.faces) + self.draw_run(pass, b, &self.pipes.id_sheet, &self.print)
    }

    /// The id pass for the lettering, after the ink as in the colour pass.
    pub fn draw_text_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_run(pass, b, &self.pipes.id_sheet, &self.text)
    }
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
/// The two arena pipelines for `target`.
```

**Replace with:**

```rust
/// The four arena pipelines for `target`.
```

_Paste it._
**Find** in `src/engine/gpu/arena.rs`:

```rust
        sheet: build(dev, target, &base.with("triangle.sheet", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnly)),
```

**Add below it:**

```rust
        id_faces: build(dev, Target::ID, &base.with("triangle.id", "fs_id").bias(FACE_BIAS)),
        id_sheet: build(dev, Target::ID, &base.with("triangle.sheet.id", "fs_id").depth(DepthMode::ReadOnlyEqual)),
```

## Step 5 - The segment lane's id pipelines

- The solid lane picks in whatever style the colour pass drew: tubes through the cylinder twin, flat through the ribbon twin. The flat lane's ribbons go through the same ribbon twin, opaque.

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
    ribbon_depth: wgpu::RenderPipeline,
```

**Add below it:**

```rust
    id_cylinder: wgpu::RenderPipeline,
    id_ribbon: wgpu::RenderPipeline,
```

_Type it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }
```

**Add below it:**

```rust

    /// The id pass for the solid lane, in the style the colour pass used.
    pub fn draw_pipe_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, style: LineStyle) -> u32 {
        match style {
            LineStyle::Tubes => self.draw_tubes(pass, b, &self.gpu.id_cylinder),
            LineStyle::Flat => self.draw_table(pass, b, &self.gpu.id_ribbon, &self.pipes),
        }
    }

    /// The id pass for the flat lane: opaque quads.
    pub fn draw_ribbon_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.id_ribbon, &self.ribbons)
    }
```

_Paste it._
**Find** in `src/engine/gpu/segments.rs`:

```rust
        ribbon_depth: build(dev, target, &quad.with("ribbon.depth", "fs_depth").color(ColorWrite::Masked)),
```

**Add below it:**

```rust
        id_cylinder: build(dev, Target::ID, &tube.with("cylinder.id", "fs_id")),
        id_ribbon: build(dev, Target::ID, &quad.with("ribbon.id", "fs_id")),
```

## Step 6 - The glyph lane's id pipelines

- The markers and the dots get the same treatment: one opaque twin each, drawn through the lane's existing table walkers.

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    dot: wgpu::RenderPipeline,
```

**Add below it:**

```rust
    id_sphere: wgpu::RenderPipeline,
    id_dot: wgpu::RenderPipeline,
```

_Type it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }
```

**Add below it:**

```rust

    /// The id pass for the markers: the template, opaque.
    pub fn draw_sphere_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_markers(pass, b, &self.gpu.id_sphere)
    }

    /// The id pass for the dots: triangles, opaque.
    pub fn draw_dot_ids(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.id_dot)
    }
```

_Paste it._
**Find** in `src/engine/gpu/glyphs.rs`:

```rust
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
```

**Add below it:**

```rust
        id_sphere: build(dev, Target::ID, &marker.with("sphere.id", "fs_id")),
        id_dot: build(dev, Target::ID, &disc.with("glyph.id", "fs_id")),
```

## Step 7 - The point lane's id pipeline

- The point lane has no scene-pass draw of its own (it resolves a compute result), so its id twin is the point pipeline itself, built for `Target::ID` with `fs_point_id`, and `draw_ids` draws the same quads into the id pass.

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
    resolve_pipeline: wgpu::RenderPipeline,
```

**Add below it:**

```rust
    id_pipeline: wgpu::RenderPipeline,
```

_Paste it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
    /// two pipelines; the targets wait for the first cloud.
```

**Replace with:**

```rust
    /// three pipelines; the targets wait for the first cloud.
```

_Paste it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
        let point_pipeline = build_point(ctx, l, &point_shader, &PointVariant { target: Target { format: COLOR_FORMAT, samples: 1 }, label: "splat.points", fs: "fs_point" });
```

**Add below it:**

```rust
        let id_pipeline = build_point(ctx, l, &point_shader, &PointVariant { target: Target::ID, label: "splat.points.id", fs: "fs_point_id" });
```

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
            resolve_pipeline,
```

**Add below it:**

```rust
            id_pipeline,
```

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
        pass.draw(0..3, 0..1);
        1
    }
```

**Add below it:**

```rust

    /// The id pass: the same quads, writing (object row, point row) instead of colour.
    pub fn draw_ids(&self, pass: &mut wgpu::RenderPass<'_>, cloud_group: &wgpu::BindGroup) -> u32 {
        if self.total == 0 {
            return 0;
        }
        pass.set_pipeline(&self.id_pipeline);
        pass.set_bind_group(0, cloud_group, &[]);
        pass.set_bind_group(1, &self.points_group, &[]);
        pass.draw(0..POINT_VERTS * self.total, 0..1);
        1
    }
```

## Step 8 - Gpu owns a Picker

- `Gpu` gets one more field beside the lanes; a resize drops the id targets so the next pick remakes them at the new size.

_Paste it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
//! is `render.rs`, presenting is `present.rs`.
```

**Replace with:**

```rust
//! is `render.rs`, presenting is `present.rs`, picking is `pick.rs`.
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod objects;
```

**Add below it:**

```rust
pub mod pick;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use objects::InstanceTable;
```

**Add below it:**

```rust
use pick::Picker;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use objects::{ObjectRow, Rebase};
```

**Add below it:**

```rust
pub use pick::Pick;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub splat: Splat,
```

**Add below it:**

```rust
    pub pick: Picker,
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat,
```

**Add below it:**

```rust
            pick: Picker::new(),
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.splat.resize();
```

**Add below it:**

```rust
        self.pick.resize();
```

## Step 9 - The id pass in the frame list

- Only when a pick is pending does the frame grow a third pass: the scene list again, opaque, at 1x, under the same toggles and in the same order, then one texel copied out.
- `map` is called right after the submit in `present`: the copy must be in the queue before the buffer can be mapped.

_Paste it._
**Find** in `src/engine/gpu/render.rs`:

```rust
//! is the ordered lane draws. The order is the contract: everything that writes depth first,
//! the blended ink after, lettering last.
```

**Replace with:**

```rust
//! is the ordered lane draws, then - only when a pick is pending - the id pass. The order is
//! the contract: everything that writes depth first, the blended ink after, lettering last.
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
            self.scene_list(&mut pass, &b)
        };
```

**Add below it:**

```rust

        if let Some(at) = self.pick.take_pending() {
            self.id_pass(encoder, at);
        }
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        draws
    }
```

**Add below it:**

```rust

    /// The id pass: the scene list again, opaque, at 1x, under the same toggles and in the
    /// same order (what a lane hides it cannot pick), then one texel copied out for `Picker`.
    fn id_pass(&mut self, encoder: &mut wgpu::CommandEncoder, at: (u32, u32)) {
        let size = (self.config.width, self.config.height);
        {
            let v = &self.view;
            let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
            let mut pass = self.pick.begin_pass(&self.ctx, encoder, size);
            self.arena.draw_face_ids(&mut pass, &b);
            if v.show_mesh_edges {
                self.segments.draw_pipe_ids(&mut pass, &b, v.line_style);
            }
            self.splat.draw_ids(&mut pass, &self.frame.cloud_group);
            if v.show_mesh_edges && v.markers {
                self.glyphs.draw_sphere_ids(&mut pass, &b);
            }
            if v.show_lines {
                self.segments.draw_ribbon_ids(&mut pass, &b);
            }
            self.arena.draw_text_ids(&mut pass, &b);
            if v.show_points {
                self.glyphs.draw_dot_ids(&mut pass, &b);
            }
        }
        self.pick.copy_texel(&self.ctx, encoder, at);
    }
```

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
        self.ctx.queue.submit([encoder.finish()]);
```

**Add below it:**

```rust
        self.pick.map();
```

## Step 10 - The selection flag

- Selection is one bit on one object row, written back as that single row; every lane already reads the row, so no table is re-uploaded. The point lane bakes its tint into records, so it is invalidated.

_Type it._
**Find** in `src/engine/gpu/objects.rs`:

```rust
            self.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }
```

**Add below it:**

```rust

    /// Set or clear one flag bit on one row and write that row back.
    pub fn set_flag(&mut self, ctx: &GpuCtx, row: u32, bit: u32, on: bool) {
        let Some(r) = self.rows.get_mut(row as usize) else { return };
        let was = r.flags & bit != 0;
        if was == on {
            return;
        }
        r.flags ^= bit;
        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.bounds = Aabb::empty();
        self.retarget(false);
    }
```

**Add below it:**

```rust

    /// Flip the selection flag on one object row.
    pub fn set_selected(&mut self, row: u32, on: bool) {
        self.objects.set_flag(&self.ctx, row, Instance::FLAG_SELECTED, on);
        self.splat.invalidate();
    }
```

## Step 11 - Tint the selected object

- Every shader that reads the instance row mixes `SELECT_COLOR` into its vertex colour when the flag is set; the alpha is untouched, so the ink's coverage still fades as before.
- The point lane's colour comes from its record, not the instance row per fragment, so the tint is chosen on the CPU when the records are built.

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
const FLAG_PRINT: u32 = 8u;
```

**Replace with:**

```wgsl
const FLAG_SELECTED: u32 = 1u;
const FLAG_PRINT: u32 = 8u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
```

_Type it._
**Find** in `src/shaders/triangle.wgsl`:

```wgsl
    o.color = in.color.rgb * inst.color.rgb;
```

**Replace with:**

```wgsl
    var color = in.color.rgb * inst.color.rgb;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = mix(color, SELECT_COLOR, 0.6);
    }
    o.color = color;
```

_Type it._
**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
```

**Replace with:**

```wgsl
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
```

_Type it._
**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
    o.color = unpack4x8unorm(seg.color) * inst.color;
```

**Replace with:**

```wgsl
    var color = unpack4x8unorm(seg.color) * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
```

_Paste it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
```

**Replace with:**

```wgsl
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const FLAG_SHEET: u32 = 32u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
```

_Paste it._
**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    o.color = unpack4x8unorm(seg.color) * inst.color;
```

**Replace with:**

```wgsl
    var color = unpack4x8unorm(seg.color) * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
```

_Paste it._
**Find** in `src/shaders/sphere.wgsl`:

```wgsl
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
```

**Replace with:**

```wgsl
const FLAG_SELECTED: u32 = 1u;
const FLAG_INSIDE: u32 = 4u;
const FLAG_OPEN: u32 = 16u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
```

_Paste it._
**Find** in `src/shaders/sphere.wgsl`:

```wgsl
    o.color = g.color * inst.color;
```

**Replace with:**

```wgsl
    var color = g.color * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
```

_Paste it._
**Find** in `src/shaders/glyph.wgsl`:

```wgsl
const HAIRLINE_MIN_ALPHA: f32 = 0.5;
```

**Add above it:**

```wgsl
const FLAG_SELECTED: u32 = 1u;
const SELECT_COLOR: vec3<f32> = vec3<f32>(1.0, 0.75, 0.2);
```

_Paste it._
**Find** in `src/shaders/glyph.wgsl`:

```wgsl
    o.color = g.color * inst.color;
```

**Replace with:**

```wgsl
    var color = g.color * inst.color;
    if ((inst.flags & FLAG_SELECTED) != 0u) {
        color = vec4<f32>(mix(color.rgb, SELECT_COLOR, 0.6), color.a);
    }
    o.color = color;
```

_Type it._
**Find** in `src/engine/gpu/splat.rs`:

```rust
            let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
```

**Replace with:**

```rust
            let selected = row.flags & Instance::FLAG_SELECTED != 0;
            let tint = if selected { [1.0, 0.85, 0.3, (px * 0.5).max(0.5)] } else { [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)] };
```

## Step 12 - What a pick means

- A `Pick` is two numbers; `Scene` is the only layer that can turn a row into a guid and a guid into a document, and a point row into the kernel point behind it (position and stable id) when the cloud was walked, not streamed.
- The selection row lives beside `hidden` and is forgotten with the rest on `clear`.

_Type it._
**Find** in `src/app/scene.rs`:

```rust
use session_rust::{Session, Xform};
```

**Replace with:**

```rust
use session_rust::{Geometry, Session, Xform};
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
```

**Replace with:**

```rust
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Pick, Upload};
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    pub total: u32,
    pub point_px: f32,
}
```

**Add below it:**

```rust

/// What a pick resolved to: the document, the geometry's guid, its object row, and for a
/// cloud the point index and its stable id.
#[derive(Clone, Debug)]
pub struct Picked {
    pub doc: String,
    pub guid: String,
    pub row: u32,
    pub point: Option<PickedPoint>,
}

/// A picked cloud point: the row-local index (this file version only) and the stable id.
#[derive(Clone, Debug)]
pub struct PickedPoint {
    pub local: u32,
    pub id: u32,
    pub position: [f64; 3],
}
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
    pub hidden: HashSet<String>,
```

**Add below it:**

```rust
    pub selected: Option<u32>,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
            hidden: HashSet::new(),
```

**Add below it:**

```rust
            selected: None,
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.hidden.clear();
```

**Add below it:**

```rust
        self.selected = None;
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.guid_to_row.clear();
        self.bases = Bases::default();
```

**Replace with:**

```rust
        self.guid_to_row.clear();
        self.selected = None;
        self.bases = Bases::default();
```

_Type it._
**Find** in `src/app/scene.rs`:

```rust
        self.streamed[idx].done_to = to;
        self.upload_to(gpu);
    }
```

**Add below it:**

```rust

    /// What a pick means: the document, the guid, and for a cloud the point behind the row.
    pub fn resolve(&self, pick: Pick, gpu: &Gpu) -> Option<Picked> {
        let guid = self.order.get(pick.row as usize)?.to_string();
        let mut point = None;
        if let Some((_, local)) = gpu.cloud.row_of(pick.sub) {
            point = self.point_at(&guid, local);
        }
        let doc = self.doc_of(&guid).map(|d| d.name.clone()).unwrap_or_default();
        Some(Picked { doc, guid, row: pick.row, point })
    }

    /// The document holding `guid`.
    fn doc_of(&self, guid: &str) -> Option<&Doc> {
        self.docs.iter().find(|d| d.session.lookup.contains_key(guid))
    }

    /// The kernel point `local` of cloud `guid`: its stable id and position. `None` for a
    /// released or streamed cloud (the GPU is then the only holder).
    fn point_at(&self, guid: &str, local: u32) -> Option<PickedPoint> {
        let doc = self.doc_of(guid)?;
        let Some(Geometry::PointCloud(pc)) = doc.session.lookup.get(guid) else { return None };
        let c = pc.coords();
        let i = local as usize * 3;
        if i + 2 >= c.len() {
            return None;
        }
        Some(PickedPoint { local, id: pc.point_id(local as usize), position: [c[i], c[i + 1], c[i + 2]] })
    }
```

## Step 13 - State asks, then answers

- `request_pick` only leaves a request with the picker and asks for a frame; the frame draws the id pass, `render` polls every frame after, and `apply_pick` runs when the texel is back. Clicking the selection again clears it.
- `busy()` keeps `needs_frame` set while a pick is in flight: the mapped buffer is read by a frame, and with nothing else moving there would be no frame to read it.

_Type it._
**Find** in `src/state.rs`:

```rust
use crate::engine::gpu::{FrameInput, Gpu};
```

**Replace with:**

```rust
use crate::engine::gpu::{FrameInput, Gpu, Pick};
```

_Type it._
**Find** in `src/state.rs`:

```rust
        self.gpu.view.cloud_size = size.clamp(0.25, 8.0);
        self.needs_frame = true;
    }
```

**Add below it:**

```rust

    /// Ask what is under pixel (x, y); the answer lands in a later frame (`apply_pick`).
    pub fn request_pick(&mut self, x: u32, y: u32) {
        self.gpu.pick.request(x, y);
        self.needs_frame = true;
    }

    /// Make `row` the selection (or none), moving the highlight.
    pub fn select(&mut self, row: Option<u32>) {
        if let Some(old) = self.scene.selected.take() {
            self.gpu.set_selected(old, false);
        }
        if let Some(r) = row {
            self.gpu.set_selected(r, true);
        }
        self.scene.selected = row;
        self.needs_frame = true;
    }

    /// A pick came back: log what it hit and select it (clicking the selection clears it).
    fn apply_pick(&mut self, pick: Option<Pick>) {
        let Some(p) = pick else {
            log::info!("pick: nothing");
            self.select(None);
            return;
        };
        match self.scene.resolve(p, &self.gpu) {
            Some(hit) => {
                match &hit.point {
                    Some(pt) => log::info!("pick: '{}' {} row {} point {} id {} at ({:.1}, {:.1}, {:.1})", hit.doc, hit.guid, hit.row, pt.local, pt.id, pt.position[0], pt.position[1], pt.position[2]),
                    None => log::info!("pick: '{}' {} row {}", hit.doc, hit.guid, hit.row),
                }
                let toggle = if self.scene.selected == Some(hit.row) { None } else { Some(hit.row) };
                self.select(toggle);
            }
            None => log::info!("pick: row {} sub {} (no document)", p.row, p.sub),
        }
    }
```

_Paste it._
**Find** in `src/state.rs`:

```rust
    /// The shell asks again when `needs_frame` is set - by an input, a message, a resize or a
    /// throttled re-anchor still due.
```

**Replace with:**

```rust
    /// The shell asks again when `needs_frame` is set - by an input, a message, a resize, a
    /// throttled re-anchor still due or a pick in flight.
```

_Type it._
**Find** in `src/state.rs`:

```rust
        let drawn = self.gpu.present(&FrameInput { view_proj, clear: CLEAR });
```

**Add below it:**

```rust
        if let Some(pick) = self.gpu.pick.poll() {
            self.apply_pick(pick);
        }
```

_Type it._
**Find** in `src/state.rs`:

```rust
        self.needs_frame |= dropped || rebase.pending;
```

**Replace with:**

```rust
        self.needs_frame |= dropped || rebase.pending || self.gpu.pick.busy();
```

## Step 14 - The click

- A click is a left press and a release within `CLICK_SLOP` of it; the press only remembers where, so a drag that starts on an object never picks it. Escape clears the selection.

_Paste it._
**Find** in `src/app/input.rs`:

```rust
//! Every binding: RMB orbits, MMB (or Ctrl+RMB) pans, the wheel zooms toward the cursor;
//! 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles, L line style,
//! [ ] cloud size. Fingers go to `touch.rs`.
```

**Replace with:**

```rust
//! Every binding: RMB orbits, MMB (or Ctrl+RMB) pans, the wheel zooms toward the cursor, a
//! left click picks; 1-7 named views, Space projection, C reset, F fit, Q/W/E lane toggles,
//! L line style, [ ] cloud size, Escape clears the selection. Fingers go to `touch.rs`.
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
use super::touch::{Act, Touches};
```

**Add below it:**

```rust

/// A press that moves less than this (physical px) before release is a click.
const CLICK_SLOP: f64 = 4.0;
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
    last_cursor: (f64, f64),
```

**Add below it:**

```rust
    left_down: Option<(f64, f64)>,
```

_Paste it._
**Find** in `src/app/input.rs`:

```rust
        Self { orbiting: false, panning: false, ctrl: false, last_cursor: (0.0, 0.0), touch: Touches::new() }
```

**Replace with:**

```rust
        Self { orbiting: false, panning: false, ctrl: false, last_cursor: (0.0, 0.0), left_down: None, touch: Touches::new() }
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            Key::Named(NamedKey::Space) => state.camera.toggle_projection_framed(&state.gpu.bounds, state.aspect()),
```

**Add below it:**

```rust
            Key::Named(NamedKey::Escape) => state.select(None),
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            WindowEvent::CursorMoved { position, .. } => {
```

**Add above it:**

```rust
            WindowEvent::MouseInput { state: btn, button: MouseButton::Left, .. } => self.left(state, *btn),
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            _ => false,
        }
    }
```

**Add below it:**

```rust

    /// The left button: a press remembers where; a release within the slop is a click and
    /// asks the GPU what is under it.
    fn left(&mut self, state: &mut State, btn: ElementState) -> bool {
        match btn {
            ElementState::Pressed => {
                self.left_down = Some(self.last_cursor);
                false
            }
            ElementState::Released => {
                let Some(down) = self.left_down.take() else { return false };
                let moved = (self.last_cursor.0 - down.0).abs().max((self.last_cursor.1 - down.1).abs());
                if moved > CLICK_SLOP {
                    return false;
                }
                state.request_pick(self.last_cursor.0 as u32, self.last_cursor.1 as u32);
                true
            }
        }
    }
```

## Run

```bash
trunk serve
```

- Open http://127.0.0.1:8770/ and click a mesh: it turns amber and the browser console prints `pick: '<document>' <guid> row <n>`; a click on a cloud adds `point <i> id <id> at (x, y, z)`; the background prints `pick: nothing`. A second click on the same object, or Escape, clears it.

## Why

- An id buffer instead of a ray: a streamed cloud has no CPU copy to cast against, and only the lanes know what is on screen (the toggles, the LOD walk, the density taper). The id pass is the scene list again, so a pixel picks exactly what it shows.
- `row + 1`, not `row`: a cleared target reads 0 everywhere, which is the background, so no second "hit" channel is needed and the copy is 8 bytes.
- `Rg32Uint` at one sample: an id is not a colour; multisampling would average two rows into a third that exists nowhere.
- The answer comes a frame later: `map_async` completes after the submit, never inside it. Polling in `render` with `busy()` keeping frames coming costs one extra frame per click instead of a stall on the queue.
- The id pass runs only when a request is pending, so a still scene still costs nothing, and the id targets are made on the first pick, not at start-up.
- One flag bit on the object row: every lane already reads the row per vertex, so the highlight is a `mix` in the shader and one 64-byte row write, never a re-upload of a table.
- The point lane bakes the tint into its records because its fragments read a record, not the instance row; `set_selected` invalidates the records so the next frame rebuilds them.
- A click has a slop: a press that moved is a gesture that started on the object, not a choice of it; `left_down` is taken on release so a stray release without a press does nothing.
