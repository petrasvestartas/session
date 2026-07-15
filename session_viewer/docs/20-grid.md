# 20 Grid — a second pipeline

The box floats in a void. A ground grid fixes that, teaching a core lesson: **a render pass can run
more than one pipeline.** The grid is a *second* pipeline beside `triangle` — own shader, primitive
type (**lines**), depth rule — drawn in the *same* pass before the box, giving scale (1 m cells). The
pattern reused for every later overlay: edges, gizmos, selection highlights.

The grid is **static** — never changes — so needs no vertex buffer: the shader *generates* every line
endpoint from `@builtin(vertex_index)`, CPU just says "draw 46 vertices." No buffer, no `RenderVertex`,
no kernel geometry.

## Why

A pipeline bakes in *one* shader + *one* primitive topology + *one* depth/blend rule. The box needs
filled triangles that write depth; the grid needs thin lines that must **never** hide it — different
pipelines, sharing the camera and frame:

```
render pass (one frame, one depth buffer)
  ├─ grid pipeline      LineList,     depth_write OFF  → drawn 1st  (floor; never occludes)
  └─ triangle pipeline  TriangleList, depth_write ON   → drawn 2nd  (solid box paints over grid)
both bind group(0) = camera mvp; the grid skips group(1) = time
```

Drawing the grid **first with depth-writes off** is the trick: it lays down floor pixels but leaves
the depth buffer untouched, so the box (drawn second, writes on) paints over it — solid, sitting *in*
the grid, lines visible where the box isn't. No z-fighting, no bleed-through.

## Files we touch

```
src/shaders/grid.wgsl              # NEW — vertexless lines shader (builds the grid from vertex_index)
src/engine/pipelines/build.rs      # build_grid_pipeline (LineList, depth_write off, no vertex buffer)
src/engine/pipelines/mod.rs        # add `grid` to Pipelines + build it
src/engine/gpu.rs                  # draw the grid first in clear() — no buffer, just draw(0..50)
```

## Step 1 — the grid shader: `src/shaders/grid.wgsl`

No vertex input: size, spacing, colors are compile-time `const`s; the vertex shader turns each
`@builtin(vertex_index)` into a world-space line endpoint by arithmetic. Authored in **mm** —
camera's `mvp` applies mm→m (lesson 16) — so a ±5000 mm grid stepping every 1000 mm is a ±5 m floor,
1 m cells.

Floor lines are **all grey**. Colored axes form a coordinate *frame*: three segments running
**outward from the origin, positive direction only** — +X red, +Y green, +Z blue — sitting exactly on
the grey centre lines, so each axis reads colored on its positive half, grey on the negative. Since
the grid never writes depth, the axes (drawn last, highest `vid`) paint over the grey underneath.

Index layout: 11 lines/direction × 2 endpoints = 22 vertices/direction, X-parallel then Y-parallel =
**44** floor vertices, plus **6** for the three axes = **50** total. From `vid` we recover direction,
line, endpoint:

<svg viewBox="0 0 620 100" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="vertex_index layout: 44 floor vertices then 6 axis vertices, 50 total" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="278" y="14" fill="#888" text-anchor="middle">vid 0 .. 49 — one draw(0..50), no vertex buffer</text>
  <g stroke="#0d0f12" stroke-width="1">
    <rect x="16"  y="26" width="231" height="28" fill="#2b4a63"/>
    <rect x="247" y="26" width="231" height="28" fill="#2b4a63"/>
    <rect x="478" y="26" width="21"  height="28" fill="#6b3a3a"/>
    <rect x="499" y="26" width="21"  height="28" fill="#3a6b3a"/>
    <rect x="520" y="26" width="21"  height="28" fill="#3a4a6b"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="131" y="44">X-parallel · 22</text>
    <text x="362" y="44">Y-parallel · 22</text>
  </g>
  <g fill="#666" text-anchor="middle" font-size="10">
    <text x="16"  y="70">0</text>
    <text x="247" y="70">22</text>
    <text x="478" y="70">44</text>
    <text x="541" y="70">50</text>
  </g>
  <g fill="#888" text-anchor="middle" font-size="10">
    <text x="488" y="88">+X</text>
    <text x="509" y="88">+Y</text>
    <text x="530" y="88">+Z</text>
  </g>
</svg>

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

const STEP: f32 = 1000.0;   // mm per cell (1 m)
const HALF: f32 = 5000.0;   // ±5 m floor
const N:    u32 = 5u;       // lines per side of centre (2*N + 1 = 11 lines per direction)
const PER_DIR: u32 = 22u;   // (2*N + 1) lines * 2 endpoints
const FLOOR:   u32 = 44u;   // 2 * PER_DIR  (X-parallel + Y-parallel); axes are vid 44..49

const GREY:  vec3<f32> = vec3<f32>(0.55, 0.55, 0.55);
const RED:   vec3<f32> = vec3<f32>(0.85, 0.30, 0.30);
const GREEN: vec3<f32> = vec3<f32>(0.30, 0.70, 0.30);
const BLUE:  vec3<f32> = vec3<f32>(0.30, 0.45, 0.85);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    let far = (vid % 2u) == 1u;             // endpoint: near or far
    var wp: vec3<f32>;
    var c:  vec3<f32>;

    if vid < FLOOR {                        // floor lines on the XY plane — all grey
        let dir  = vid / PER_DIR;           // 0 = X-parallel, 1 = Y-parallel
        let line = (vid % PER_DIR) / 2u;    // which line, 0..2*N
        let t    = (f32(line) - f32(N)) * STEP; // line offset, −HALF..HALF
        let end  = select(-HALF, HALF, far);
        wp = select(vec3<f32>(end, t, 0.0), vec3<f32>(t, end, 0.0), dir == 1u);
        c  = GREY;
    } else {                                // axes: colored, positive half only (origin → +HALF)
        let axis = (vid - FLOOR) / 2u;      // 0 = X, 1 = Y, 2 = Z
        let d    = select(0.0, HALF, far);  // near end = origin, far end = +HALF
        if axis == 0u {                     // +X red
            wp = vec3<f32>(d, 0.0, 0.0);
            c  = RED;
        } else if axis == 1u {              // +Y green
            wp = vec3<f32>(0.0, d, 0.0);
            c  = GREEN;
        } else {                            // +Z blue
            wp = vec3<f32>(0.0, 0.0, d);
            c  = BLUE;
        }
    }

    var o: VsOut;
    o.pos   = mvp * vec4<f32>(wp, 1.0);
    o.color = c;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

## Step 2 — the grid pipeline: `src/engine/pipelines/build.rs`

Copy `build_triangle_pipeline` and change four things: the shader, **`buffers: &[]`** (shader
generates positions), **`topology: LineList`**, and **`depth_write_enabled: Some(false)`**. Binds only
group 0 (the camera) — no `time_layout`:

```rust
pub fn build_grid_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("grid.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/grid.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("grid.layout"),
        bind_group_layouts: &[Some(aspect_layout)],   // group 0 = camera mvp only
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("grid"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],   // ← no vertex buffer; positions come from @builtin(vertex_index)
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,   // ← lines, not triangles
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),   // ← test against depth, but never write it
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

## Step 3 — register it: `src/engine/pipelines/mod.rs`

Add a `grid` field and build it. `Pipelines::new` already receives `aspect_layout`, so its signature
is unchanged:

```rust
use build::{build_grid_pipeline, build_triangle_pipeline};

pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        color_format: wgpu::TextureFormat,
        aspect_layout: &wgpu::BindGroupLayout,
        time_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format, aspect_layout, time_layout),
            grid: build_grid_pipeline(device, color_format, aspect_layout),
        }
    }
}
```

## Step 4 — draw it first: `src/engine/gpu.rs`

No new struct fields, no buffer, no `new()` changes — geometry lives in the shader. In `clear()`,
inside the render pass, **before** the mesh draw: run the grid pipeline, bind group 0 (camera), draw
with **no vertex buffer**:

```rust
            // grid first — depth-writes are off, so the solid mesh drawn next paints over it.
            // Vertexless: the shader builds all 50 line endpoints from @builtin(vertex_index).
            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.draw(0..50, 0..1);   // 44 floor (11 lines × 2 dirs × 2 ends) + 6 for the X/Y/Z axes

            // …then the existing mesh draw (set_pipeline(triangle) … draw_indexed) stays as-is
```

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The blue box sits on a grey grid with a coordinate frame at the origin: +X red, +Y green, +Z blue,
colored only on its positive half. Orbit (right-drag) — the grid anchors the motion, finally showing
you're circling a ground plane. `1`–`7`: **Top** looks straight down, **Front** edge-on. (Box is
centred on the origin, straddling the plane; reads solid since the grid never writes depth. Sitting
it *on* the grid is a later tweak.)

## Recap

```
Ch 19: one pipeline drew a kernel mesh from a vertex buffer.
Ch 20: a SECOND pipeline (grid.wgsl) shares the pass — LineList topology, depth_write OFF, drawn
       first so the solid box paints over it. Same camera (group 0), no time. VERTEXLESS: no buffer,
       no RenderVertex — the shader builds all 50 endpoints (44 grey floor + 6 for the +X/+Y/+Z axis
       frame) from @builtin(vertex_index). This is the overlay pattern for edges/gizmos.
```

Because the grid is static, generating it in the shader beats a CPU buffer: nothing to allocate,
upload, or sync. (Still hardware 1 px lines; a crisp anti-aliased *infinite* grid — fullscreen
triangle with `fract`/`fwidth` — is a later, separate step.)

Edited: `shaders/grid.wgsl` (new, vertexless), `pipelines/build.rs` (`build_grid_pipeline`,
`buffers: &[]`), `pipelines/mod.rs` (`grid` field), `engine/gpu.rs` (draw grid first in `clear()`).

## Next

`21-mesh-shading.md` — light the box using the `normal` already sitting in every `RenderVertex`: a
lambert/hemisphere term in the mesh fragment shader turns the flat blue solid into a shaded form,
faces reading by orientation.
