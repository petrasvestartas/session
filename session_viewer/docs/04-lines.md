# 04 Lines, polylines, points: the flat lane

- At the end the local scene's polylines and points draw over the faces with a pen that stays the same width on screen at every zoom, and `W` / `Q` hide and show them.
- Every straight piece of ink is one 40 B `CylinderSegment` row and every point one 48 B `GlyphPoint` row; a producer under `app/walk/` fills the rows, a lane under `engine/gpu/` owns the table and its pipeline, and `Upload` carries the rows across the line between the two halves.
- A segment is drawn as a camera-facing quad, not a tube: at a screen-constant width a cylinder's roundness is never visible, and the quad's capsule SDF antialiases itself with a feather.
- Ink never writes depth. It draws after the faces with the test set to `GreaterEqual` and lifts a quarter pixel toward the eye, so a line that lies on a face wins the tie and a line behind a plate stays behind it.
- The row layout is the contract lesson 5 keeps: the `facing` word is already there, `FACING_UNKNOWN` is its only value for now, and mesh wires will fill it without adding a column.

<svg viewBox="0 0 720 342" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Lesson 4 on the two-halves map: producers under app/walk fill SegRows and GlyphRows in Upload; segments.rs and glyphs.rs under engine/gpu own the tables and draw them through ribbon.wgsl and glyph.wgsl" style="max-width:100%;height:auto;font:11px ui-monospace,SFMono-Regular,Menlo,Consolas,monospace">
  <defs><marker id="l4a" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#f0b35c"/></marker><marker id="l4b" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <g fill="currentColor" font-size="11">
    <text x="14" y="18" fill="#f0b35c">app/  (the walk)</text>
    <text x="360" y="18" fill="#7ed37e" text-anchor="middle">Upload  (the line)</text>
    <text x="706" y="18" fill="#6fb3ff" text-anchor="end">engine/  (the GPU)</text>
  </g>
  <line x1="14" y1="24" x2="706" y2="24" stroke="currentColor" stroke-opacity="0.25"/>
  <g fill="none" stroke="#f0b35c">
    <rect x="14" y="36" width="200" height="36"/>
    <rect x="14" y="80" width="200" height="36"/>
    <rect x="14" y="124" width="200" height="36"/>
    <rect x="14" y="168" width="200" height="36"/>
    <rect x="14" y="212" width="200" height="36"/>
    <rect x="14" y="256" width="200" height="36"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="22" y="51">walk/encode.rs</text><text x="22" y="65" fill-opacity="0.6">encode_width · pack_rgba · Pen · FACING_UNKNOWN</text>
    <text x="22" y="95">walk/curves.rs</text><text x="22" y="109" fill-opacity="0.6">Line · Polyline · NurbsCurve -&gt; ribbons</text>
    <text x="22" y="139">walk/frames.rs</text><text x="22" y="153" fill-opacity="0.6">Plane square · OBB edges -&gt; ribbons</text>
    <text x="22" y="183">walk/points.rs</text><text x="22" y="197" fill-opacity="0.6">Point -&gt; dots</text>
    <text x="22" y="227">walk/mod.rs</text><text x="22" y="241" fill-opacity="0.6">Walk { arena, seg, glyph } · walk_geometry arms</text>
    <text x="22" y="271">input.rs</text><text x="22" y="285" fill-opacity="0.6">Q flips show_points · W flips show_lines</text>
  </g>
  <line x1="214" y1="98" x2="262" y2="112" stroke="#f0b35c" marker-end="url(#l4a)"/>
  <line x1="214" y1="142" x2="262" y2="112" stroke="#f0b35c" marker-end="url(#l4a)"/>
  <line x1="214" y1="186" x2="262" y2="198" stroke="#f0b35c" marker-end="url(#l4a)"/>
  <g fill="none" stroke="#7ed37e" stroke-width="1.3">
    <rect x="264" y="86" width="192" height="52"/>
    <rect x="264" y="172" width="192" height="52"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="272" y="101">seg: SegRows</text><text x="272" y="116" fill-opacity="0.6">ribbons: Vec&lt;CylinderSegment&gt;</text><text x="272" y="130" fill-opacity="0.6">40 B: p0 radius p1 id color facing</text>
    <text x="272" y="187">glyph: GlyphRows</text><text x="272" y="202" fill-opacity="0.6">dots: Vec&lt;GlyphPoint&gt;</text><text x="272" y="216" fill-opacity="0.6">48 B: center radius color id facing x3</text>
  </g>
  <line x1="456" y1="112" x2="504" y2="112" stroke="#6fb3ff" marker-end="url(#l4b)"/>
  <line x1="456" y1="198" x2="504" y2="198" stroke="#6fb3ff" marker-end="url(#l4b)"/>
  <g fill="none" stroke="#6fb3ff">
    <rect x="506" y="36" width="200" height="40"/>
    <rect x="506" y="86" width="200" height="52"/>
    <rect x="506" y="172" width="200" height="52"/>
    <rect x="506" y="236" width="200" height="40"/>
  </g>
  <g fill="currentColor" font-size="10">
    <text x="514" y="51">pipelines/layouts.rs · mod.rs · gpu/buffers.rs</text><text x="514" y="66" fill-opacity="0.6">rows (group 3) · ReadOnlyEqual · Blended · index_buffer</text>
    <text x="514" y="101">gpu/segments.rs + shaders/ribbon.wgsl</text><text x="514" y="116" fill-opacity="0.6">SegTable · SegmentLane · RIBBON_INDICES</text><text x="514" y="130" fill-opacity="0.6">6 verts folded, caps along the 3D line, hair lift</text>
    <text x="514" y="187">gpu/glyphs.rs + shaders/glyph.wgsl</text><text x="514" y="202" fill-opacity="0.6">GlyphTable · GlyphLane · 3 verts per dot</text><text x="514" y="216" fill-opacity="0.6">one triangle whose incircle is the disc</text>
    <text x="514" y="251">gpu/view.rs · frame.rs · render.rs</text><text x="514" y="266" fill-opacity="0.6">show_lines · show_points · thickness_px -&gt; LineUniform</text>
  </g>
  <line x1="14" y1="302" x2="706" y2="302" stroke="currentColor" stroke-opacity="0.25"/>
  <g fill="currentColor" font-size="10">
    <text x="14" y="320">scene_list: 1 background · 2 grid · 3 faces (write depth) · <tspan fill="#6fb3ff">4 lines</tspan> · <tspan fill="#6fb3ff">5 point dots</tspan> (blended, depth read-only)</text>
    <text x="14" y="336" fill-opacity="0.6">orange = a producer, green = the rows Upload carries, blue = the lane and the floor it stands on; every box is created or edited in this lesson</text>
  </g>
</svg>

## Step 1 - Encode a pen once

Every producer turns a kernel width and colour into the same two words, so the encoding lives in one file: a width of 1.0 (the untouched default) is radius 0 = the screen-constant pen, anything else a world-mm radius; a colour is RGBA8 in one `u32`, low byte red, the layout `unpack4x8unorm` reads.

_Type it._
**Create `src/app/walk/encode.rs`**

```rust
//! Row encodings shared by every producer: pen widths to radii, colours to RGBA8. Pure
//! functions on numbers.

/// An authored width (kernel millimetres) as the world-mm RADIUS the shaders project; the
/// untouched 1.0 default (and 0 / non-finite) is 0.0 = the screen-constant pen.
pub fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 {
        (w as f32) * 0.5
    } else {
        0.0
    }
}

/// One colour channel to a byte, rounded.
fn quant8(v: f32) -> u32 {
    ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff
}

/// RGBA8 in one word, low byte red - the layout `unpack4x8unorm` expects in WGSL.
pub fn pack_rgba(c: [f32; 4]) -> u32 {
    quant8(c[0]) | quant8(c[1]) << 8 | quant8(c[2]) << 16 | quant8(c[3]) << 24
}

/// `facing` meaning "no adjacency, always draw". All-ones: (0, 0) is the honest code for +Z.
pub const FACING_UNKNOWN: u32 = u32::MAX;

/// One pen for a run of segments: the object row, the encoded radius and the packed colour.
pub struct Pen {
    pub row: u32,
    pub radius: f32,
    pub color: u32,
}
```

## Step 2 - Give every ink lane a group 3

An ink lane pulls its rows from a storage buffer by `instance_index` (no vertex buffer at all), so the layouts gain one shape: a single read-only storage buffer at binding 0, bound at group 3 after mvp, pen and instances.

_Type it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
//! 0 = mvp, 1 = line/pen uniform, 2 = instances (rows + anchored translations).
```

**Replace with:**

```rust
//! 0 = mvp, 1 = line/pen uniform, 2 = instances (rows + anchored translations), 3 = the lane's rows.
```

_Type it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
        entries: &[buffer_entry(0, stages, wgpu::BufferBindingType::Uniform)],
    })
}
```

**Add below it:**

```rust

/// One read-only storage buffer at binding 0: the row table every ink lane reads.
fn rows_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries: &[storage_entry(0)],
    })
}
```

_Type it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
    pub instance: wgpu::BindGroupLayout,
```

**Add below it:**

```rust
    pub rows: wgpu::BindGroupLayout,
```

_Type it._
**Find** in `src/engine/pipelines/layouts.rs`:

```rust
            instance: instance_layout(device),
```

**Add below it:**

```rust
            rows: rows_layout(device, "rows.layout"),
```

## Step 3 - Blended colour and a depth test that ties

Ink is alpha-blended (its feather is an alpha ramp) and never writes depth; it tests with `GreaterEqual` so a fragment at exactly a face's depth still draws. `PipelineDesc` gains the colour variant and a `color` builder to select it.

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    ReadOnly,
```

**Add below it:**

```rust
    /// Test only, `GreaterEqual`: blended ink that must tie with its prepass and with faces.
    ReadOnlyEqual,
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
            DepthMode::ReadOnly => (false, wgpu::CompareFunction::Greater),
```

**Add below it:**

```rust
            DepthMode::ReadOnlyEqual => (false, wgpu::CompareFunction::GreaterEqual),
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
    Opaque,
}

impl ColorWrite {
```

**Replace with:**

```rust
    Opaque,
    /// Alpha-blend: ink with an AA feather.
    Blended,
}

impl ColorWrite {
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
            ColorWrite::Opaque => (None, wgpu::ColorWrites::ALL),
```

**Add below it:**

```rust
            ColorWrite::Blended => (Some(wgpu::BlendState::ALPHA_BLENDING), wgpu::ColorWrites::ALL),
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
/// derives its variants with `with` and `depth`.
#[derive(Clone)]
pub struct PipelineDesc<'a> {
```

**Replace with:**

```rust
/// derives its variants with `with`, `color` and `depth`.
#[derive(Clone)]
pub struct PipelineDesc<'a> {
```

_Type it._
**Find** in `src/engine/pipelines/mod.rs`:

```rust
        d.fs = fs;
        d
    }
```

**Add below it:**

```rust

    /// The same desc with another colour mode.
    pub fn color(mut self, color: ColorWrite) -> Self {
        self.color = color;
        self
    }
```

## Step 4 - A static index buffer

A ribbon is six vertices drawn as four triangles, and the same twelve indices serve every instance; the floor gains a one-line constructor for such a pattern.

_Type it._
**Find** in `src/engine/gpu/buffers.rs`:

```rust
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
```

**Add below it:**

```rust

/// A static index buffer holding `indices`, for a per-instance vertex pattern.
pub fn index_buffer(ctx: &GpuCtx, label: &str, indices: &[u16]) -> wgpu::Buffer {
    ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents: bytemuck::cast_slice(indices), usage: wgpu::BufferUsages::INDEX })
}
```

## Step 5 - The segment lane

One 40 B row per straight piece of ink, one `GrowBuf` table with the group 3 that binds it, one blended read-only pipeline, and a draw that runs the vertex shader six times per row through `RIBBON_INDICES`. The ends are six flat `f32`s because a `vec3` would pad the row to 48 B, and the mirror test holds the shader to that layout.

_Type it._
**Create `src/engine/gpu/segments.rs`**

```rust
//! The segment lane: every straight piece of ink. One table of 40 B rows - ribbons
//! (line/polyline/curve, the FLAT lane, blended camera-facing quads). `SegRows` is one upload.

use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, index_buffer, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"))];

/// A ribbon is six vertices (side, centre, side at each end - folded along its centre line so
/// each half can lie in its own face plane at a crease) drawn as four triangles through this
/// index pattern, one instance per segment: the vertex shader runs six times, not twelve.
const RIBBON_INDICES: [u16; 12] = [0, 1, 4, 0, 4, 3, 1, 2, 5, 1, 5, 4];

/// One segment row, 40 B, the layout ribbon.wgsl declares. The ends are
/// flat f32s: a `vec3` would pad the row to 48 B.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    pub p0: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub p1: [f32; 3],
    pub instance_id: u32,
    /// RGBA8, low byte red.
    pub color: u32,
    /// Two oct16 adjacent face normals; `FACING_UNKNOWN` = no adjacency, always drawn.
    pub facing: u32,
}

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// One upload's segments: the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.ribbons);
    }
}

/// One segment table on the GPU with the group 3 that binds it.
struct SegTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl SegTable {
    /// A one-row table.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<CylinderSegment>() as u64, ROWS);
        let group = bind_group(ctx, &l.rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Append rows; the group is rebuilt only when the buffer grew.
    fn append(&mut self, ctx: &GpuCtx, l: &Layouts, rows: &[CylinderSegment]) {
        if self.buf.append(ctx, rows) {
            self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
        }
    }

    /// Hand the buffer back and re-point the group at the one-row table.
    fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.buf.release(ctx);
        self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
    }
}

/// The shader module the lane's pipeline is built from.
struct SegShaders {
    ribbon: wgpu::ShaderModule,
}

/// The pipeline over the table: the blended, depth-read-only quad.
struct SegPipelines {
    ribbon: wgpu::RenderPipeline,
}

/// The segment lane on the GPU: the table, the ribbon's index pattern, the shader, the
/// pipeline.
pub struct SegmentLane {
    ribbons: SegTable,
    ribbon_ibo: wgpu::Buffer,
    shaders: SegShaders,
    gpu: SegPipelines,
}

impl SegmentLane {
    /// A one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = SegShaders {
            ribbon: module(&ctx.device, "ribbon.shader", include_str!("../../shaders/ribbon.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);
        let ribbon_ibo = index_buffer(ctx, "ribbon.ibo", &RIBBON_INDICES);

        Self { ribbons: SegTable::new(ctx, l, "ribbons"), ribbon_ibo, shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        self.ribbons.append(ctx, l, &up.ribbons);
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended, depth read-only.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_table(pass, b, &self.gpu.ribbon, &self.ribbons)
    }

    /// One table as ribbons through `pipeline`; 0 draws when empty.
    fn draw_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline, table: &SegTable) -> u32 {
        if table.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &table.group, &[]);
        pass.set_index_buffer(self.ribbon_ibo.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..RIBBON_INDICES.len() as u32, 0, 0..table.buf.len());
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.ribbons.buf.reset();
    }

    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.ribbons.release(ctx, l);
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.buf.len()
    }
}

/// Every segment pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &SegShaders, target: Target) -> SegPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let quad = PipelineDesc::new(&s.ribbon, &groups, &[], TriangleList);
    let dev = &ctx.device;

    SegPipelines {
        ribbon: build(dev, target, &quad.with("ribbon", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// ribbon.wgsl reads the 40 B segment row (ends as scalars).
    #[test]
    fn cylinder_segment_mirror() {
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
    }
}
```

## Step 6 - The ribbon shader

The vertex shader clips the segment against the near plane before any divide, projects both ends, and places corner `k` of six on a quad folded along its centre line (side, centre, side at each end) whose caps are extended along the 3D line in clip space; it lifts the ink a quarter pixel toward the eye, capped by a quarter of the object's thickness and by half a millimetre. The fragment is a capsule SDF: coverage through the feather, and a width under one pixel is drawn one pixel wide with the deficit paid in alpha.

_Type it._
**Create `src/shaders/ribbon.wgsl`**

```wgsl
// Flat linework: one camera-facing quad per segment (6 verts pulled by index, no vertex
// buffer), a capsule SDF in the fragment. Draws the ribbon table (free linework). Group 3 =
// the segment table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct CylinderSegment {
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,
    facing: u32,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
};

const MM_TO_M: f32 = 0.001;
const HAIRLINE_MIN_ALPHA: f32 = 0.5;

// Faces never move (arena.rs). A segment lifts a hair to win ties, capped by LIFT_MAX_THICK
// of its object's thickness and by LIFT_MAX_MM outright, so even far away it cannot cross the
// millimetres of a joint.
const LIFT_HAIR_PX: f32 = 0.25;
const LIFT_MAX_THICK: f32 = 0.25;
const LIFT_MAX_MM: f32 = 0.5;

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

// The lift as a fraction of eye depth `w` (metres), capped by the object's thickness (mm) and
// by LIFT_MAX_MM.
fn lift_capped(lift: f32, w: f32, thickness: f32) -> f32 {
    var cap_mm = LIFT_MAX_MM;
    if (thickness > 0.0) {
        cap_mm = min(cap_mm, LIFT_MAX_THICK * thickness);
    }
    let max_lift = cap_mm * MM_TO_M / max(w, 1e-9);
    return clamp(min(lift, max_lift), 0.0, 0.5);
}

// One end's lifted w: `lift_px` pixels' worth of depth toward the camera, as a fraction of w.
fn lifted_w(lift_px: f32, e: vec4<f32>, thickness: f32) -> f32 {
    let lift = lift_px * 2.0 * MM_TO_M / (line.proj_y * line.vp_h);
    return e.w * (1.0 - lift_capped(lift, e.w, thickness));
}

fn ndc_z_per_world() -> f32 {
    return length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
}

// The ortho lift in ndc: `lift_px` pixels' worth of world depth, capped, through the z row.
fn ortho_lift_ndc(lift_px: f32, thickness: f32) -> f32 {
    let lift = lift_px * 2.0 * line.ortho_h / line.vp_h;
    var cap = LIFT_MAX_MM;
    if (thickness > 0.0) {
        cap = min(cap, LIFT_MAX_THICK * thickness);
    }
    return min(lift, cap) * ndc_z_per_world();
}

// Half-width in px at one end: half the global pen, or a world radius projected.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5;
}

// Hairline rule: never thinner than 1 px, the deficit goes into alpha (floored).
fn floor_hairline(px: f32) -> f32 {
    return max(px, 0.5);
}

fn hairline_fade(px: f32) -> f32 {
    if (px < 0.5) {
        return max(px / 0.5, HAIRLINE_MIN_ALPHA);
    }
    return 1.0;
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>,
    @location(2) @interpolate(flat) a: vec2<f32>,
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
    @location(7) @interpolate(flat) inst_id: u32,
};

// The fragment's half-width and fade at `h` along the segment. Resolved per pixel from the
// two flat end values: a per-vertex width is projective over a trapezoid and the two
// triangles disagree along the diagonal.
fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), hairline_fade(raw));
}

fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.p = vec2<f32>(0.0);
    dead.a = vec2<f32>(0.0);
    dead.b = vec2<f32>(0.0);
    dead.hw0 = 0.0;
    dead.hw1 = 0.0;
    dead.inst_id = 0u;
    return dead;
}

// Which quad corner vertex `k` of 6 is: 0 = e0-, 1 = e0+, 2 = e1-, 3 = e1+.
// Six vertices per segment, one instance per segment, four triangles through the lane's
// index pattern: a ribbon FOLDED along its centre line (corners 0-2 at end 0, 3-5 at end 1;
// lane 0 = side -1, 1 = the centre at the edge's own depth, 2 = side +1), so each half can
// lie in its own face plane at a crease.

@vertex
fn vs_main(@builtin(vertex_index) corner: u32, @builtin(instance_index) iid: u32) -> VsOut {
    let seg = segments[iid];
    let inst = instances[seg.instance_id];

    let w0 = place(seg.instance_id, vec3<f32>(seg.p0x, seg.p0y, seg.p0z));
    let w1 = place(seg.instance_id, vec3<f32>(seg.p1x, seg.p1y, seg.p1z));

    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);
    let at_end1 = corner >= 3u;
    let side = f32(corner % 3u) - 1.0;

    // Clip against the near plane (z - w = 0 in reverse-Z) BEFORE any divide: a hand divide
    // behind the eye mirrors the point through the screen centre.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;
    if (f0 > 0.0 && f1 > 0.0) {
        return dead_vertex();
    }
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);

    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // The quad is a trapezoid under perspective: both end widths go down flat.
    let raw0 = half_width_px(seg.radius, e0.w);
    let raw1 = half_width_px(seg.radius, e1.w);
    let px = floor_hairline(select(raw0, raw1, at_end1));
    let off = px + 0.5 * line.feather;

    // The round caps extend the ribbon `off` px past each end ALONG THE 3D LINE (a clip-space
    // extrapolation), so the depth ramp along the ribbon is the edge's own; extending on
    // screen with the end's depth shifted the ramp by `off` px and lost the tie against the
    // faces halfway along every crease. A near-degenerate or eye-pointing segment keeps its end.
    let ext = off / max(len, 1e-3);
    let e_own = select(e0, e1, at_end1);
    let e_other = select(e1, e0, at_end1);
    var e = e_own + (e_own - e_other) * ext;
    if (e.w < 0.5 * e_own.w || ext > 4.0) {
        e = e_own;
    }
    let s_end = (e.xy / e.w * 0.5 + 0.5) * vp;
    let p = s_end + n * side * off;

    // Lift the ink toward the camera: in w for perspective, in ndc z for ortho.
    let thick = inst.thickness;
    var wn = e.w;
    var zn = e.z;
    if (line.ortho_h > 0.0) {
        zn = e.z + ortho_lift_ndc(LIFT_HAIR_PX, thick);
    } else {
        wn = lifted_w(LIFT_HAIR_PX, e, thick);
        zn = e.z / wn;
    }

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * wn, select(e.z, zn * wn, line.ortho_h > 0.0), wn);
    o.color = unpack4x8unorm(seg.color) * inst.color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0;
    o.hw1 = raw1;
    o.inst_id = seg.instance_id;
    return o;
}

// Coverage of the capsule at this fragment, in [0, 1], times the hairline fade.
fn coverage(in: VsOut) -> f32 {
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let d = length(pa - ba * h);
    let hf = resolve_width(in, h);
    return clamp((hf.x + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * hf.y;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

## Step 7 - The glyph lane

The same lane shape over the 48 B `GlyphPoint` row: a table, a group 3, one blended pipeline. A dot has no index pattern; the draw issues three vertices per row and the shader picks the corner from `vertex_index`.

_Paste it._
**Create `src/engine/gpu/glyphs.rs`**

```rust
//! The glyph lane: every vertex-sized piece of ink. One table of 48 B rows -
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.

use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"))];

/// Vertices per dot: one triangle whose incircle is the disc.
const DOT_VERTS: u32 = 3;

/// One dot row, 48 B, the layout glyph.wgsl declares.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    /// Up to SIX incident face normals as oct16 pairs, widest edge's two first;
    /// `FACING_UNKNOWN` = no adjacency / no more.
    pub facing: u32,
    pub facing_ext: [u32; 2],
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.dots);
    }
}

/// One glyph table on the GPU with the group 3 that binds it.
struct GlyphTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl GlyphTable {
    /// A one-row table.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Append rows; the group is rebuilt only when the buffer grew.
    fn append(&mut self, ctx: &GpuCtx, l: &Layouts, rows: &[GlyphPoint]) {
        if self.buf.append(ctx, rows) {
            self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
        }
    }

    /// Hand the buffer back and re-point the group at the one-row table.
    fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.buf.release(ctx);
        self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
    }
}

/// The shader module the lane's pipeline is built from.
struct GlyphShaders {
    dot: wgpu::ShaderModule,
}

/// The pipeline over the table.
struct GlyphPipelines {
    dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: the table, the shader, the pipeline.
pub struct GlyphLane {
    dots: GlyphTable,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// A one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = GlyphShaders {
            dot: module(&ctx.device, "glyph.shader", include_str!("../../shaders/glyph.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { dots: GlyphTable::new(ctx, l, "dots"), shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        self.dots.append(ctx, l, &up.dots);
    }

    /// The flat lane's colour pass: SDF dots, three verts each, no template.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }

    /// The dot table through `pipeline`; 0 draws when empty.
    fn draw_dot_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.dots.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.dots.group, &[]);
        pass.draw(0..DOT_VERTS * self.dots.buf.len(), 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.dots.buf.reset();
    }

    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.dots.release(ctx, l);
    }

    /// Flat-lane rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.buf.len()
    }
}

/// Every glyph pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList);
    let dev = &ctx.device;

    GlyphPipelines {
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// glyph.wgsl reads the 48 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
    }
}
```

## Step 8 - The dot shader

One equilateral triangle per dot whose incircle is the disc, the same pen rule (global pixels or a projected world radius), the same hairline fade, and a lift of half its radius under the ribbon's two caps, with the SDF reduced to a distance from the centre.

_Paste it._
**Create `src/shaders/glyph.wgsl`**

```wgsl
// Free points as SDF dots: one triangle per dot (its incircle is the disc), no template.
// Group 3 = the glyph table.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct GlyphPoint {
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    facing: u32,
    facing_ext: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform {
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,
    feather: f32,
};

const HAIRLINE_MIN_ALPHA: f32 = 0.5;
const MM_TO_M: f32 = 0.001;
const LIFT_RADII: f32 = 0.5;
const LIFT_MAX_THICK: f32 = 0.25;
const LIFT_MAX_MM: f32 = 0.5;

// An equilateral triangle whose incircle (radius 1 in corner space) is the visible dot.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>(1.7320508, -1.0),
);

fn place(i: u32, p: vec3<f32>) -> vec3<f32> {
    return (instances[i].model * vec4<f32>(p, 1.0)).xyz + translations[i].xyz;
}

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(linear) px: f32,
    @location(3) @interpolate(linear) fade: f32,
    @location(4) @interpolate(flat) inst_id: u32,
};

fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
    dead.fade = 0.0;
    dead.inst_id = 0u;
    return dead;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let g = glyphs[vid / 3u];
    let inst = instances[g.instance_id];
    let world = place(g.instance_id, g.center);
    let clip = mvp * vec4<f32>(world, 1.0);
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    var px = line.thickness * 0.5;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h * 0.5 / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
        }
    }
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }
    var fade = 1.0;
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    let corner = CORNERS[vid % 3u];
    var lift = 0.0;
    var zlift = 0.0;
    if (line.ortho_h > 0.0) {
        let lw = px * LIFT_RADII * 2.0 * line.ortho_h / line.vp_h;
        let cap = select(LIFT_MAX_MM, min(LIFT_MAX_MM, LIFT_MAX_THICK * inst.thickness), inst.thickness > 0.0);
        zlift = min(lw, cap) * length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z));
    } else {
        lift = px * LIFT_RADII * 2.0 * MM_TO_M / (line.proj_y * line.vp_h);
        var cap_mm = LIFT_MAX_MM;
        if (inst.thickness > 0.0) {
            cap_mm = min(cap_mm, LIFT_MAX_THICK * inst.thickness);
        }
        lift = clamp(min(lift, cap_mm * MM_TO_M / max(clip.w, 1e-9)), 0.0, 0.5);
    }
    let wn = clip.w * (1.0 - lift);
    let off = corner * (px + 0.5 * line.feather) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z + zlift * wn, wn);
    o.color = g.color * inst.color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    o.inst_id = g.instance_id;
    return o;
}

fn coverage(in: VsOut) -> f32 {
    let d = length(in.corner) * (in.px + 0.5 * line.feather);
    return clamp((in.px + 0.5 * line.feather - d) / line.feather, 0.0, 1.0) * in.fade;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let alpha = coverage(in);
    if (alpha <= 0.0) {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

## Step 9 - Walk lines, polylines and curves

A line is one row, a polyline one row per span, a NURBS curve a polyline sampled with a count that follows the size of its control box. A polyline reports `polyline_thickness` (its spread across its own plane) so the lift cap knows a planar outline is flat; every producer reports its box, which is where the object's thickness comes from.

_Type it._
**Create `src/app/walk/curves.rs`**

```rust
//! Lines, polylines and NURBS curves into the FLAT ribbon lane: one segment per span,
//! `FACING_UNKNOWN` because free linework has no adjacent faces. Every producer reports the
//! object's local box, which caps the ink lift so a line behind a plate stays behind it.

use session_rust::{Line, NurbsCurve, Polyline};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use crate::math::Aabb;
use super::{Row, WalkCx};
use super::bounds::polyline_thickness;
use super::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};

/// Segments between consecutive points, growing `bounds` as they go.
fn push_polyline(seg: &mut SegRows, pts: &[[f32; 3]], pen: &Pen, bounds: &mut Aabb) {
    seg.ribbons.reserve(pts.len().saturating_sub(1));
    for w in pts.windows(2) {
        bounds.grow(w[0]);
        seg.ribbons.push(CylinderSegment { p0: w[0], radius: pen.radius, p1: w[1], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
    }
    if let Some(last) = pts.last() {
        bounds.grow(*last);
    }
}

/// One ribbon segment; the ends are read by index (no kernel `Point` allocations).
pub fn walk_line(seg: &mut SegRows, l: &Line, row: u32) -> Row {
    let p0 = [l[0] as f32, l[1] as f32, l[2] as f32];
    let p1 = [l[3] as f32, l[4] as f32, l[5] as f32];
    let mut bounds = Aabb::empty();
    bounds.grow(p0);
    bounds.grow(p1);
    seg.ribbons.push(CylinderSegment { p0, radius: encode_width(l.width), p1, instance_id: row, color: pack_rgba(l.linecolor.to_f32()), facing: FACING_UNKNOWN });
    Row::thin(bounds)
}

/// One segment per span, straight from the flat coordinate array.
pub fn walk_polyline(seg: &mut SegRows, pl: &Polyline, cx: &WalkCx) -> Row {
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        pts.push([c[0] as f32, c[1] as f32, c[2] as f32]);
    }
    let pen = Pen { row: cx.row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    let thickness = polyline_thickness(&pts);
    Row { thickness, ..Row::thin(bounds) }
}

/// The box of the control points (a NURBS curve never leaves its control net).
fn control_box(c: &NurbsCurve) -> Option<([f64; 3], [f64; 3])> {
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for i in 0..c.m_cv_count {
        let Some(cv) = c.cv(i) else { continue };
        let w = if c.m_is_rat && cv.len() > 3 && cv[3] != 0.0 { cv[3] } else { 1.0 };
        for k in 0..3 {
            lo[k] = lo[k].min(cv[k] / w);
            hi[k] = hi[k].max(cv[k] / w);
        }
    }
    if lo[0] > hi[0] { None } else { Some((lo, hi)) }
}

/// Sample the curve into a polyline whose segment count follows its size, then walk that.
pub fn walk_nurbscurve(seg: &mut SegRows, c: &NurbsCurve, row: u32) -> Row {
    let Some((lo, hi)) = control_box(c) else { return Row::thin(Aabb::empty()) };
    let size = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt();
    let n = ((size / 0.2).sqrt().ceil() as usize).clamp(4, 64);

    let (t0, t1) = c.domain();
    let mut pts: Vec<[f32; 3]> = Vec::with_capacity(n + 1);
    for i in 0..=n {
        pts.push(c.point_at(t0 + (t1 - t0) * i as f64 / n as f64).to_f32());
    }
    let color = c.linecolors.first().map(|c| c.to_f32()).unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let pen = Pen { row, radius: encode_width(c.width), color: pack_rgba(color) };
    let mut bounds = Aabb::empty();
    push_polyline(seg, &pts, &pen, &mut bounds);
    Row::thin(bounds)
}
```

## Step 10 - Walk planes and boxes

A plane draws as a square of `PLANE_SIZE` half-extent along its own axes and a box as its twelve edges over a corner table; both go through one `push_loop` with one pen.

_Paste it._
**Create `src/app/walk/frames.rs`**

```rust
//! Planes and oriented boxes into the FLAT ribbon lane as outlines: a 1 m square for a
//! plane, the 12 edges for a box.

use session_rust::{Plane, Point, Vector, OBB};
use crate::engine::gpu::segments::SegRows;
use crate::engine::gpu::CylinderSegment;
use crate::math::Aabb;
use super::Row;
use super::encode::{encode_width, pack_rgba, Pen, FACING_UNKNOWN};

/// Half-extent of the square drawn for an infinite plane, world mm.
const PLANE_SIZE: f64 = 500.0;

/// The 12 edges of a box whose corners are ordered bottom 0-3, top 4-7.
const BOX_EDGES: [[usize; 2]; 12] = [[0, 1], [1, 2], [2, 3], [3, 0], [4, 5], [5, 6], [6, 7], [7, 4], [0, 4], [1, 5], [2, 6], [3, 7]];

/// The square's corner at signs `s` along the plane's x/y axes.
fn corner(o: &Point, x: &Vector, y: &Vector, s: [f64; 2]) -> [f32; 3] {
    [0usize, 1, 2].map(|k| (o[k] + (x[k] * s[0] + y[k] * s[1]) * PLANE_SIZE) as f32)
}

/// The `edges` over `pts` as segments with one pen; returns the points' box.
fn push_loop(seg: &mut SegRows, pts: &[[f32; 3]], edges: &[[usize; 2]], pen: &Pen) -> Aabb {
    let mut bounds = Aabb::empty();
    for p in pts {
        bounds.grow(*p);
    }
    for &[i, j] in edges {
        seg.ribbons.push(CylinderSegment { p0: pts[i], radius: pen.radius, p1: pts[j], instance_id: pen.row, color: pen.color, facing: FACING_UNKNOWN });
    }
    bounds
}

/// The four edges of the plane's square.
pub fn walk_plane(seg: &mut SegRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y) = (pl.origin(), pl.x_axis(), pl.y_axis());
    let c = [corner(&o, &x, &y, [1.0, 1.0]), corner(&o, &x, &y, [-1.0, 1.0]), corner(&o, &x, &y, [-1.0, -1.0]), corner(&o, &x, &y, [1.0, -1.0])];
    let pen = Pen { row, radius: encode_width(pl.width), color: pack_rgba(pl.linecolor.to_f32()) };
    Row::thin(push_loop(seg, &c, &[[0, 1], [1, 2], [2, 3], [3, 0]], &pen))
}

/// A box is its 12 edges; the OBB type carries no pen, so they draw black at the default width.
pub fn walk_obb(seg: &mut SegRows, b: &OBB, row: u32) -> Row {
    let c = b.corners_f32();
    let pen = Pen { row, radius: 0.0, color: pack_rgba([0.0, 0.0, 0.0, 1.0]) };
    Row::thin(push_loop(seg, &c, &BOX_EDGES, &pen))
}
```

## Step 11 - Walk points

A free point is one dot row; its three facing words are `FACING_UNKNOWN` because it decorates no surface.

_Type it._
**Create `src/app/walk/points.rs`**

```rust
//! A free point into the FLAT glyph lane: one SDF dot, `FACING_UNKNOWN` because it
//! decorates no surface.

use session_rust::Point;
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::GlyphPoint;
use crate::math::Aabb;
use super::Row;
use super::encode::{encode_width, FACING_UNKNOWN};

/// One SDF dot.
pub fn walk_point(glyph: &mut GlyphRows, p: &Point, row: u32) -> Row {
    let center = p.to_f32();
    glyph.dots.push(GlyphPoint {
        center,
        radius: encode_width(p.width),
        color: p.pointcolor.to_f32(),
        instance_id: row,
        facing: FACING_UNKNOWN,
        facing_ext: [FACING_UNKNOWN; 2],
    });
    let mut bounds = Aabb::empty();
    bounds.grow(center);
    Row::thin(bounds)
}
```

## Step 12 - Upload carries the two tables

The rows cross from the walk to the GPU inside `Upload`, so it gains a field per lane and drops them with the rest once uploaded.

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::arena::ArenaRows;
```

**Add below it:**

```rust
use super::glyphs::GlyphRows;
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::objects::ObjectRows;
```

**Add below it:**

```rust
use super::segments::SegRows;
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
    pub arena: ArenaRows,
```

**Add below it:**

```rust
    pub seg: SegRows,
    pub glyph: GlyphRows,
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
            arena: ArenaRows::default(),
```

**Add below it:**

```rust
            seg: SegRows::default(),
            glyph: GlyphRows::default(),
```

_Type it._
**Find** in `src/engine/gpu/upload.rs`:

```rust
        self.arena.drop_rows();
```

**Add below it:**

```rust
        self.seg.drop_rows();
        self.glyph.drop_rows();
```

## Step 13 - Wire the producers

`Walk` lends the two new tables to a producer and `walk_geometry` sends six geometry types to them; the arms that returned an empty box become the calls.

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use crate::engine::gpu::arena::ArenaRows;
```

**Add below it:**

```rust
use crate::engine::gpu::glyphs::GlyphRows;
use crate::engine::gpu::segments::SegRows;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use crate::math::Aabb;
```

**Add below it:**

```rust
use curves::{walk_line, walk_nurbscurve, walk_polyline};
use frames::{walk_obb, walk_plane};
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
use mesh::{walk_mesh, MeshCx, MeshOpts};
```

**Add below it:**

```rust
use points::walk_point;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod bounds;
```

**Add below it:**

```rust
pub mod curves;
pub mod encode;
pub mod frames;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
pub mod mesh;
```

**Add below it:**

```rust
pub mod points;
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
    pub arena: &'a mut ArenaRows,
```

**Add below it:**

```rust
    pub seg: &'a mut SegRows,
    pub glyph: &'a mut GlyphRows,
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Self { arena: &mut t.arena }
```

**Replace with:**

```rust
        Self { arena: &mut t.arena, seg: &mut t.seg, glyph: &mut t.glyph }
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
/// One object into the tables. Meshes take the SOLID lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

**Replace with:**

```rust
/// One object into the tables. Meshes take the SOLID lane; free linework
/// and points the FLAT lane.
pub fn walk_geometry(w: &mut Walk, cx: &WalkCx, geom: &Geometry) -> Row {
```

_Type it._
**Find** in `src/app/walk/mod.rs`:

```rust
        Geometry::Line(_) => Row::thin(Aabb::empty()),
        Geometry::Polyline(_) => Row::thin(Aabb::empty()),
        Geometry::NurbsCurve(_) => Row::thin(Aabb::empty()),
        Geometry::Plane(_) => Row::thin(Aabb::empty()),
        Geometry::OBB(_) => Row::thin(Aabb::empty()),
        Geometry::Point(_) => Row::thin(Aabb::empty()),
```

**Replace with:**

```rust
        Geometry::Line(l) => walk_line(w.seg, l, cx.row),
        Geometry::Polyline(pl) => walk_polyline(w.seg, pl, cx),
        Geometry::NurbsCurve(c) => walk_nurbscurve(w.seg, c, cx.row),
        Geometry::Plane(p) => walk_plane(w.seg, p, cx.row),
        Geometry::OBB(b) => walk_obb(w.seg, b, cx.row),
        Geometry::Point(p) => walk_point(w.glyph, p, cx.row),
```

## Step 14 - Register the lanes in `Gpu`

A lane is one field of `Gpu` and one line in each of build, append, retarget, reset, release and the shader list; the scene log counts its rows.

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod frame;
```

**Add below it:**

```rust
pub mod glyphs;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod render;
```

**Add below it:**

```rust
pub mod segments;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use frame::FrameUniforms;
```

**Add below it:**

```rust
use glyphs::GlyphLane;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use objects::InstanceTable;
```

**Add below it:**

```rust
use segments::SegmentLane;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use frame::FrameInput;
```

**Add below it:**

```rust
pub use glyphs::GlyphPoint;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use objects::{ObjectRow, Rebase};
```

**Add below it:**

```rust
pub use segments::CylinderSegment;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub arena: ArenaLane,
```

**Add below it:**

```rust
    pub segments: SegmentLane,
    pub glyphs: GlyphLane,
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        let arena = ArenaLane::new(&ctx, &layouts, target);
```

**Add below it:**

```rust
        let segments = SegmentLane::new(&ctx, &layouts, target);
        let glyphs = GlyphLane::new(&ctx, &layouts, target);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            arena,
```

**Add below it:**

```rust
            segments,
            glyphs,
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena.append(&self.ctx, &up.arena);
```

**Add below it:**

```rust
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        log::info!("scene: {} objects, {} verts", self.objects.len(), self.arena.vert_count());
```

**Replace with:**

```rust
        log::info!(
            "scene: {} objects, {} verts, {} ribbons, {} dots",
            self.objects.len(), self.arena.vert_count(), self.segments.ribbon_count(), self.glyphs.dot_count()
        );
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.arena.retarget(&self.ctx, &self.layouts, target);
```

**Add below it:**

```rust
            self.segments.retarget(&self.ctx, &self.layouts, target);
            self.glyphs.retarget(&self.ctx, &self.layouts, target);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena.reset();
```

**Add below it:**

```rust
        self.segments.reset();
        self.glyphs.reset();
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena.release(&self.ctx);
```

**Add below it:**

```rust
        self.segments.release(&self.ctx, &self.layouts);
        self.glyphs.release(&self.ctx, &self.layouts);
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    out.extend_from_slice(arena::SHADERS);
```

**Add below it:**

```rust
    out.extend_from_slice(segments::SHADERS);
    out.extend_from_slice(glyphs::SHADERS);
```

## Step 15 - The pen and the two toggles

What a frame reads is one struct: `View` gains the two lane toggles and the pen weight (`?thickness=` on wasm, `VIEWER_THICKNESS` natively), and the line uniform takes its thickness from there instead of a literal.

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
//! `View` - the runtime knobs a frame reads. Read ONCE at startup from the query string
//! (wasm) or the environment (native). No GPU here.
```

**Replace with:**

```rust
//! `View` - the runtime knobs a frame reads: what to show, the
//! pen weight. Read ONCE at startup from the query string
//! (wasm) or the environment (native); the key handlers flip them afterwards. No GPU here.
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
pub struct View {
```

**Add below it:**

```rust
    /// Point markers - the FLAT lane's dots. `Q`.
    pub show_points: bool,
    /// Lines and polylines - the FLAT lane's ribbons. `W`.
    pub show_lines: bool,
    /// On-screen pen weight, px (`?thickness=` / `VIEWER_THICKNESS`).
    pub thickness_px: f32,
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
        Self {
```

**Add below it:**

```rust
            show_points: true,
            show_lines: true,
            thickness_px: knob_f32("VIEWER_THICKNESS", "thickness", 2.0).max(0.1),
```

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
            thickness: 2.0,
            feather: cx.view.feather_px,
```

**Replace with:**

```rust
            thickness: cx.view.thickness_px,
            feather: cx.view.feather_px,
```

## Step 16 - Two lines in the frame list

Lines and dots draw after the faces, each behind its toggle; they write no depth, so two lines on one pixel resolve by draw order.

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
    /// 1 background · 2 grid · 3 faces.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = 0u32;
```

**Replace with:**

```rust
    /// 1 background · 2 grid · 3 faces · 4 lines · 5 point dots. Lines write no depth: two lines on one
    /// pixel resolve by draw order (a depth prepass costs a second ribbon draw - measured +5 ms
    /// on view_mixed - for a case only coincident lines of different colours can show).
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let v = &self.view;
        let mut draws = 0u32;
```

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
        draws += self.arena.draw_faces(pass, b);
```

**Add below it:**

```rust
        if v.show_lines {
            draws += self.segments.draw_ribbons(pass, b);
        }
        if v.show_points {
            draws += self.glyphs.draw_dots(pass, b);
        }
```

## Step 17 - Keys Q and W

A key flips the toggle in `View` and reports a redraw like every other binding.

_Type it._
**Find** in `src/app/input.rs`:

```rust
//! 1-7 named views, Space projection, C reset, F fit.
```

**Replace with:**

```rust
//! 1-7 named views, Space projection, C reset, F fit, Q/W lane toggles.
```

_Type it._
**Find** in `src/app/input.rs`:

```rust
            Key::Character("f" | "F") => state.fit_all(),
```

**Add below it:**

```rust
            Key::Character("q" | "Q") => state.gpu.view.show_points = !state.gpu.view.show_points,
            Key::Character("w" | "W") => state.gpu.view.show_lines = !state.gpu.view.show_lines,
```

## Run

```bash
trunk serve
```

- Open http://localhost:8770: the local scene's `boxes` item now shows its polyline and its point beside the three boxes, and the `scene:` line in the console counts the ribbons and dots.
- `W` hides the lines, `Q` the points; `?thickness=4` widens the pen. A width left at the kernel default draws at the pen and keeps its screen width at every zoom; an authored width is millimetres and scales with the view.

## Why

- Pay per pixel: at a screen-constant width the roundness of a tube is never visible, so a camera-facing quad makes the same pixels for six vertex-shader runs and four triangles per segment, and the capsule SDF gives analytic antialiasing for free.
- A row, not a mesh, is the unit: the walk emits 40 B and 48 B rows, the lane appends them to a `GrowBuf` and rebuilds its group only when the buffer grew, so a lane is a table plus a pipeline and deleting one is deleting its file, its shader, its producer, its `Upload` field and its line in `render.rs`.
- Ink writes no depth and tests `GreaterEqual` after the faces: the hair lift decides a tie against a face, and draw order decides a tie between two lines, which spares a second ribbon draw for a depth prepass.
- The lift is a quarter pixel of eye depth, never more than a quarter of the object's thickness and never more than half a millimetre, so a line behind a plate cannot climb through it however far away the camera is; that is why every producer reports a box and a polyline its own planar thickness.
- The caps are extended along the 3D line in clip space rather than on screen at the end's depth, so the depth ramp along the ribbon is the edge's own; lesson 5 relies on it when mesh wires must tie with their faces.
- The near-plane clip happens before any divide: a hand divide behind the eye mirrors the point through the screen centre, so a segment crossing the near plane is cut at the plane instead of mirrored.
- `facing` ships now with one value, `FACING_UNKNOWN`: free linework has no adjacent faces, and lesson 5 fills the word for mesh wires without changing a row layout the mirror test already checks.
- The toggles and the pen live in `View` because it is the one struct a frame reads; a key handler flips a bool and the next frame is different, with no other state to keep in step.
