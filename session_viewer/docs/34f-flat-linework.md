# 34f Flat linework at scale — capsule ribbons, glyph dots, paper-space widths

> **Big picture.** 503k objects, 4 draw calls, CPU near-idle — and 10 fps. The arithmetic:
> 598,604 segments × 48 triangles (12-sided capped cylinder) ≈ **29M triangles and 30M
> vertex-shader runs for ~1.2M pixels of actual ink**. One instanced draw kills the CPU cost that
> scales with object count; it does nothing for GPU vertex/fill cost — and no API (wgpu included)
> changes how many triangles silicon rasterizes. The CAD answer is *pay per pixel*: our thickness
> is screen-constant 2–4px at EVERY zoom, so the cylinder's roundness is never visible — a flat
> camera-facing capsule makes the same pixels for 2 triangles. And because the capsule edge is an
> SDF we get **analytic anti-aliasing** almost free: a 1px alpha-blended ramp at the edge plus
> the **hairline rule** (sub-pixel widths become 1px lines at proportional opacity) — the two
> tricks that make every CAD viewer's linework look calm and uniform at any zoom. Measured end to
> end (headless probe, same scene):

```
12-sided cylinders   149 ms/frame          6-sided (still 3D)    ~75 ms      (2.0×)
capsule ribbons       28 ms/frame (5.3×)   user's GPU            ~18 ms
```

## Files we touch

```
src/engine/gpu/mod.rs           # Step 1: CYL_SIDES 6 + LINEWORK_SOLID · Step 2: LineUniform+vp_w
                                # Step 5: mode-aware draws · Step 6b: planar paper-width pass
src/shaders/cylinder.wgsl       # Step 2: LineUniform struct sync (32 B)
src/shaders/sphere.wgsl         # Step 2: LineUniform struct sync
src/shaders/ribbon.wgsl         # Step 3: NEW — capsule edge quads (round caps!)
src/shaders/glyph.wgsl          # Step 3: NEW — SDF circle dots (1 triangle)
src/engine/pipelines/build.rs   # Step 4: build_ribbon_pipeline + build_glyph_pipeline
src/engine/pipelines/mod.rs     # Step 4: ribbon + glyph fields; cylinder + sphere STAY
src/engine/gpu/adapters.rs      # Step 6a: encode_width — kernel width enters the radius encoding
```

## Step 1 — two consts: `src/engine/gpu/mod.rs`

Screen-constant thickness means the cross-section is never visibly round — 12 sides is waste at
every zoom (48→24 tris/segment, pixels identical, measured 2×). And the whole lesson hangs off
one switch, so add it in the same visit. **Find at the top of the file:**

```rust
/// const for the unit_cylinder method
const CYL_SIDES: u32 = 12;
```

**Replace with** (the switch does nothing until Step 5 branches on it):

```rust
/// const for the unit_cylinder method
const CYL_SIDES: u32 = 6;

/// Linework style switch. Thickness is screen-constant px in BOTH modes, so at 2-4px the two are
/// pixel-identical — but not cost-identical:
/// `false` (default): FLAT — camera-facing ribbon edges (2 tris/segment) + circle-glyph dots
///                    (1 tri/dot). 598k segments ≈ 1.2M tris.
/// `true`:            SOLID — 3D cylinder edges + sphere dots. 598k segments ≈ 14M tris —
///                    measured ~5x slower at stress scale; kept for close-up/handle work.
const LINEWORK_SOLID: bool = false;
```

## Step 2 — `LineUniform` grows `vp_w` (five small edits, 1 struct + 2 write sites + 2 shaders)

Flat shapes offset corners in CLIP space — pixels→NDC needs BOTH viewport axes. Uniform sizes are
16-byte multiples, so the struct grows from one vec4 to two.

**2a. The Rust struct.** Near the bottom of `gpu/mod.rs`, find:

```rust
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
} // 16 B - one vec4, no padding
```

Replace the last two lines (`vp_h: …` and the closing `}`) so it reads:

```rust
struct LineUniform{
    thickness: f32, // on-screwwn width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half.heigh x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    _pad: [f32; 3],
} // 32 B - two vec4s
```

**2b. Write site 1 — `new()`.** Find (inside the `line_buffer` init; note `vp_h` has no comma):

```rust
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: config.height as f32
            }),
```

Replace with:

```rust
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: config.height as f32,
                vp_w: config.width as f32,
                _pad: [0.0; 3],
            }),
```

**2c. Write site 2 — `clear()`.** Find:

```rust
            ortho_h: 0.0, // perspective, set the ortho half-height when ortho
            vp_h: self.config.height as f32,
        };
```

Replace with:

```rust
            ortho_h: 0.0, // perspective, set the ortho half-height when ortho
            vp_h: self.config.height as f32,
            vp_w: self.config.width as f32,
            _pad: [0.0; 3],
        };
```

**2d. Mirror in `src/shaders/cylinder.wgsl`.** Find:

```wgsl
struct LineUniform{
    thickness: f32, // desired on-screen width, in pixels
    proj_y: f32, // vertical projection scale + unit scale (perspective: cot(fovy)/2) mm > m)
    ortho_h: f32, // ortho world-height * unit scale; 0.0 in perspcetive
    vp_h: f32, // framebuffer height, in pixels
};
```

Add four fields before the closing `};`:

```wgsl
    vp_w: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
```

**2e. Mirror in `src/shaders/sphere.wgsl`.** Find:

```wgsl
struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
};
```

Add the same four fields before the closing `};`.

A mismatched uniform struct is a pipeline-creation error at startup — after this step both 3D
pipelines still build, and `cargo check` passes again.

## Step 3 — two NEW shader files

**Create `src/shaders/ribbon.wgsl`** (next to `cylinder.wgsl`) with exactly this content. One
segment = 6 buffer-less verts (2 triangles) forming a screen-space **capsule**: the quad is
extruded perpendicular to the segment's SCREEN direction (always camera-facing), extended
half-width past both ends, and the fragment shader rounds the caps with an SDF — **round line
ends**, and polyline corners join smoothly because neighbouring caps overlap. Depth comes from
each endpoint's own clip position, so occlusion still works per-pixel.

Two quality rules live here, and they are what separates CAD-grade linework from "jaggy GL
lines". **(1) Analytic AA:** the SDF edge is a 1px *alpha ramp*, not a binary `discard`. A
`discard` cannot be smoothed by MSAA — the fragment shader runs once per PIXEL, so all 4 samples
live or die together and the capsule edge would stay pixel-stepped; the blended ramp
anti-aliases it exactly. (Alpha-to-coverage is the cheaper cousin, but with 4 samples it
quantizes alpha to five steps — faint hairlines round down to invisible and mid-tones band; for
drawing-quality ink, blend.) **(2) The hairline rule:** a width below 1px is never rasterized
thinner — the deficit moves into opacity (a 0.3px pen = a 1px line at 30% alpha). Without it,
~1px opaque lines snap to the pixel grid: one line lands on a pixel row and reads crisp, its
neighbour straddles two rows and reads fat or broken — identical widths LOOK different at every
zoom.

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{ model: mat4x4<f32>, color: vec4<f32>, flags: u32, };
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment (48 B) — same table the cylinder pipeline reads.
struct CylinderSegment{
    p0: vec3<f32>, radius: f32, p1: vec3<f32>, instance_id: u32, color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32,
    vp_w: f32, _pad0: f32, _pad1: f32, _pad2: f32,
};

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>, // this fragment's screen position, px
    @location(2) @interpolate(flat) a: vec2<f32>,   // segment endpoints on screen, px
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(linear) hw: f32,      // half-width, px
    @location(5) @interpolate(linear) fade: f32,    // sub-pixel opacity (hairline rule)
};

//   corner 0: e0−   1: e1−   2: e1+     (tri 1)
//   corner 3: e0−   4: e1+   5: e0+     (tri 2)
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let model = instances[seg.instance_id].model;

    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);
    let clip = select(c0, c1, at_end1);

    // Endpoints in screen pixels (one consistent mapping, used both ways)
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (c0.xy / max(abs(c0.w), 1e-6) * 0.5 + 0.5) * vp;
    let s1 = (c1.xy / max(abs(c1.w), 1e-6) * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // px half-width at this end: global thickness, or a world radius projected (>0) —
    // the inverse of cylinder.wgsl's screen_radius, solved for pixels.
    var px = line.thickness;
    if (seg.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = seg.radius * line.vp_h / line.ortho_h;
        } else {
            px = seg.radius * line.proj_y * line.vp_h / clip.w;
        }
    }

    // Hairline rule: never rasterize thinner than 1px — carry the deficit into OPACITY
    // instead. A 0.3px pen renders as a 1px line at 30% alpha, so apparent weight stays
    // continuous across zoom instead of snapping per pixel row.
    var fade = 1.0;
    if (px < 0.5) {
        fade = px / 0.5;
        px = 0.5;
    }

    // Corner in px: sideways ± half-width, PAST the end by half-width (cap room),
    // +0.5px on both so the AA feather ramp fits inside the quad
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * (px + 0.5);

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.zw);
    o.color = seg.color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw = px;
    o.fade = fade;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // Capsule SDF in screen px — rounds both caps. Analytic AA: a 1px alpha ramp centered
    // on the edge, alpha-blended (a binary discard cannot be smoothed by MSAA — all 4
    // samples of a pixel live or die together), times the hairline fade.
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let d = length(pa - ba * h);
    let alpha = clamp(in.hw + 0.5 - d, 0.0, 1.0) * in.fade;
    if (alpha <= 0.0) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

**Create `src/shaders/glyph.wgsl`** — the dot sibling: 32b's 3-corner triangle whose incircle is
the disc, reading the GLYPH table, depth-writing, with the same 1px AA rim ramp and hairline
fade as the ribbon:

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust GlyphPoint (48 B) — same table the sphere pipeline reads.
struct GlyphPoint{
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform{
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32,
    vp_w: f32, _pad0: f32, _pad1: f32, _pad2: f32,
};

// 32b's equilateral triangle: the INCIRCLE (radius 1 in corner space) is the visible dot.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>( 0.0,        2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>( 1.7320508, -1.0),
);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(linear) px: f32,   // dot radius, px
    @location(3) @interpolate(linear) fade: f32, // sub-pixel opacity (hairline rule)
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let g = glyphs[vid / 3u];   // 3 verts per dot
    let model = instances[g.instance_id].model;
    let world = (model * vec4<f32>(g.center, 1.0)).xyz;
    let clip = mvp * vec4<f32>(world, 1.0);

    // px radius: global thickness, or a world radius projected (>0) — the same inverse of
    // cylinder.wgsl's screen_radius that ribbon.wgsl uses.
    var px = line.thickness;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h / line.ortho_h;
        } else {
            px = g.radius * line.proj_y * line.vp_h / max(clip.w, 1e-6);
        }
    }

    // Hairline rule: sub-pixel dots render at 1px with proportional opacity (ribbon.wgsl)
    var fade = 1.0;
    if (px < 0.5) {
        fade = px / 0.5;
        px = 0.5;
    }

    // Triangle scaled to px + 0.5 so the AA feather ramp fits inside it
    let corner = CORNERS[vid % 3u];
    let off = corner * (px + 0.5) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + off, clip.zw);
    o.color = g.color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Circle SDF in px (corner length 1 = px + 0.5 on screen), 1px AA ramp at the rim
    let d = length(in.corner) * (in.px + 0.5);
    let alpha = clamp(in.px + 0.5 - d, 0.0, 1.0) * in.fade;
    if (alpha <= 0.0) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
```

## Step 4 — pipelines

**4a. `src/engine/pipelines/build.rs`** — paste both builders at the very END of the file, after
`build_point_pipeline`'s closing `}`. Both are buffer-less (verts come from `vertex_index`) and
**alpha-blended** (`ALPHA_BLENDING` — the shader's AA ramp and hairline fade need real blending;
see Step 3). Depth is **tested but not written**: every sheet line sits at the same depth, and a
blended 5%-alpha feather pixel that wrote depth would BLOCK the full-opacity core of the next
line crossing it — light holes at every intersection. Test-only keeps meshes occluding linework
while ink blends freely over ink:

```rust
/// Pipeline for flat capsule ribbons — buffer-less, 6 verts/segment, opaque, depth-writing.
pub fn build_ribbon_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    segment_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("ribbon.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ribbon.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout),
            Some(segment_layout)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("ribbon"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],                          // buffer-less — no template
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING), // smooth AA feather + hairline fade
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false), // blended ink must not block later ink at the same depth (line crossings)
            depth_compare: Some(wgpu::CompareFunction::Greater),   // reverse-Z (26)
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: MSAA_SAMPLES, mask: !0,
            alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}

/// Pipeline for flat SDF dots — the ribbon recipe with the glyph names; GLYPH layout at group 3.
pub fn build_glyph_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    glyph_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("glyph.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/glyph.wgsl").into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("glyph.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout),
            Some(glyph_layout)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("glyph"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false), // blended ink must not block later ink at the same depth (line crossings)
            depth_compare: Some(wgpu::CompareFunction::Greater),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: MSAA_SAMPLES, mask: !0,
            alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}
```

**4b. `src/engine/pipelines/mod.rs`** — three edits, top to bottom.

Find:

```rust
use build::build_point_pipeline;
```

Insert after it:

```rust
use build::build_ribbon_pipeline;
use build::build_glyph_pipeline;
```

Find in `pub struct Pipelines`:

```rust
    pub point: wgpu::RenderPipeline,
```

Insert after it:

```rust
    pub ribbon: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
```

Find in `Pipelines::new` (one long line):

```rust
            point: build_point_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
```

Insert after it (this file calls the mvp layout `aspect_layout` — keep that name):

```rust
            ribbon: build_ribbon_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            glyph: build_glyph_pipeline(device, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
```

`cylinder` and `sphere` keep theirs — SOLID mode still uses them.

## Step 5 — the switch: `gpu/mod.rs` `clear()`

Both linework blocks branch on `LINEWORK_SOLID` inside their count-guards — SOLID keeps the exact
template draws, FLAT drops the template buffers. **Find the whole Edges block:**

```rust
            if self.segment_count > 0 {
                pass.set_pipeline(&self.pipelines.cylinder);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.segment_count); // one template, N edges
                draws += 1;
            }
```

Replace with:

```rust
            if self.segment_count > 0 {
                if LINEWORK_SOLID {
                    pass.set_pipeline(&self.pipelines.cylinder);
                    pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                    pass.set_bind_group(1, &self.line_bind_group, &[]);
                    pass.set_bind_group(2, &self.instance_bind_group, &[]);
                    pass.set_bind_group(3, &self.segment_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                    pass.set_index_buffer(self.cyl_template_ibo.slice(..),
                        wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.segment_count);
                } else {
                    pass.set_pipeline(&self.pipelines.ribbon);
                    pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                    pass.set_bind_group(1, &self.line_bind_group, &[]);
                    pass.set_bind_group(2, &self.instance_bind_group, &[]);
                    pass.set_bind_group(3, &self.segment_bind_group, &[]);
                    pass.draw(0..6 * self.segment_count, 0..1); // 6 verts/segment, no template
                }
                draws += 1;
            }
```

**Find the whole Spheres block** (right below):

```rust
            if self.glyph_count > 0 {
                pass.set_pipeline(&self.pipelines.sphere);
                pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                pass.set_bind_group(1, &self.line_bind_group, &[]);
                pass.set_bind_group(2, &self.instance_bind_group, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.glyph_count); // one template, N glyphs
                draws += 1;
            }
```

Replace with:

```rust
            if self.glyph_count > 0 {
                if LINEWORK_SOLID {
                    pass.set_pipeline(&self.pipelines.sphere);
                    pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                    pass.set_bind_group(1, &self.line_bind_group, &[]);
                    pass.set_bind_group(2, &self.instance_bind_group, &[]);
                    pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                    pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                    pass.set_index_buffer(self.sph_template_ibo.slice(..),
                        wgpu::IndexFormat::Uint32);
                    pass.draw_indexed(0..self.sph_index_count, 0, 0..self.glyph_count);
                } else {
                    pass.set_pipeline(&self.pipelines.glyph);
                    pass.set_bind_group(0, &self.mvp_bind_group, &[]);
                    pass.set_bind_group(1, &self.line_bind_group, &[]);
                    pass.set_bind_group(2, &self.instance_bind_group, &[]);
                    pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                    pass.draw(0..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                }
                draws += 1;
            }
```

**Checkpoint:** `cargo check` passes; `trunk serve` already renders the wall flat and fast.

## Step 6 — paper-space lineweights (the 2D/3D split)

CAD convention: **2D drawing sheets have paper-mm lineweights** — zoom out and the ink thins like
a print; **model geometry (3D lines, mesh/BRep edges) stays screen-constant**. The shader already
has both lanes (`radius == 0` px-constant, `> 0` world-mm); route drawing widths into the world
lane.

**6a. `src/engine/gpu/adapters.rs`** — three edits. At the very END of the file, after
`point_to_glyph`'s closing `}`, add:

```rust
/// Kernel width (dimensionless, default 1.0) → the radius encoding's NEGATIVE lane (px
/// multiplier); 0.0 = plain global default. `walk_session` flips negatives into the POSITIVE
/// (world-mm) lane for planar 2D drawings — paper-space lineweights that scale with zoom.
pub(super) fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 { -(w as f32) } else { 0.0 }
}
```

In `line_to_segment`, find `radius: 0.0,` and replace with:

```rust
        radius: encode_width(l.width),
```

In `polyline_to_segments`, find `radius: 0.0,` and replace with:

```rust
        radius: encode_width(pl.width),
```

(`point_to_glyph` KEEPS its `radius: 0.0` — 34h wires point widths.)

**6b. `gpu/mod.rs` `walk_session`** — planarity IS the discriminator (every PDF conversion is
exactly z ≡ 0; no flag needed in the data). Find the END of `walk_session` — the glyph extent
fold and the return:

```rust
        for g in &t.glyphs { for k in 0..3 {
            t.min[k] = t.min[k].min(g.center[k]);
            t.max[k] = t.max[k].max(g.center[k]);
        } }
        t
    }
```

Insert between the fold and the `t` line:

```rust
        // 2D DRAWING SHEETS (exactly planar, z ≡ 0 — every PDF conversion) get PAPER-SPACE
        // lineweights: kernel width (mm on the sheet) → the radius WORLD lane, so zooming out
        // thins the ink like a real print. 3D model files keep screen-constant px linework.
        let planar = t.min[2].is_finite() && (t.max[2] - t.min[2]).abs() < 1e-3;
        if planar {
            for s in &mut t.segments {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } // width mm → half-width mm
                           else { 0.5 };                        // default width 1.0 → 0.5mm pen
            }
        }
```

The hairline rule in ribbon.wgsl (Step 3) keeps zoomed-out ink at 1px — fading its opacity
instead of letting it shimmer away or snap between 1 and 2 pixel rows — also standard CAD
behavior.

## Verify — measured, not vibed

The headless probe (no browser interaction needed) must target the VIEWER APP URL (usually
`http://127.0.0.1:8770/` from `trunk serve`), not the docs server (`docs/serve.py` on 8771).
The docs page is static and will not emit runtime `"perf ..."` lines.

```bash
mkdir -p /tmp/hl
CHROME_LOG_FILE=/tmp/hl/chrome_debug.log timeout 45s \
google-chrome --headless=new --enable-unsafe-webgpu --enable-logging --v=0 \
    --disable-background-networking --no-first-run --no-default-browser-check \
    --user-data-dir=/tmp/hl --window-size=1758,1347 http://127.0.0.1:8770/ \
    > /tmp/hl/chrome.stderr 2>&1 || true

grep -oE '"perf[^"]*"' /tmp/hl/chrome_debug.log | tail -5
```

If that grep is empty, check fallback output (some Chrome builds emit logs on stderr instead of the
debug log file):

```bash
grep -oE '"perf[^"]*"' /tmp/hl/chrome.stderr | tail -5
```

What to look for:

- You should see recent `"perf ..."` lines with non-zero draw/object counts.
- In flat mode (`LINEWORK_SOLID = false`), frame time should be near the chapter target and clearly
    better than solid mode.
- Flip `LINEWORK_SOLID = true`, re-run, and confirm a clear slowdown with near-identical visuals at
    2-4px thickness.

Same 503k-object wall: `149ms → 28ms` headless (5.3×), `~100ms → ~18ms` on a real GPU. Flip
`LINEWORK_SOLID = true` and diff by eye: identical at 2–4px. Zoom into a drawing cell: pens
fatten like paper (0.28/0.51/0.71mm weights now visibly differ); zoom out: hairlines, floored at
1px. For the 3D lane, temporarily add `"session_data/floor_model.pb"` to `DEMO_SESSION_URLS`
(its `copy-file` is already in `index.html` from 34a): that cell's mesh edges stay 2px at every
zoom — non-planar files are untouched.

## Recap

```
Ch 34f: PAY PER PIXEL. CYL_SIDES 6 (roundness is invisible at screen-constant width — free 2×).
        ribbon.wgsl: screen-space CAPSULE, 2 tris/segment, SDF round caps, per-endpoint depth,
        0.5px inward analytic-AA edge ramp (discard is per-pixel — MSAA can't smooth it) +
        hairline rule (sub-pixel width → 1px at proportional opacity). glyph.wgsl: 1-tri SDF
        discs, same AA. Both pipelines opaque + alpha_to_coverage (order-independent — no
        blending RMW), depth-writing. LINEWORK_SOLID keeps cylinders+spheres one constant away —
        same tables, same pixels. LineUniform +vp_w (32 B ×3 mirrors). Paper-space lineweights:
        planar files route kernel width → world-mm lane (zoom-dependent ink, 1px hairline); 3D
        stays screen-constant. 149→28ms measured on the 503k wall.
```

## Next

`34g-camera-ux.md` — the wall renders fast; now it has to FEEL like CAD: cursor-centered zoom
with no range stops, and middle-mouse pan.
