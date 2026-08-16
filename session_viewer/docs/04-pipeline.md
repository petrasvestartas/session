# 04 Pipeline

> **Big picture.** Every drawing this viewer will ever make — meshes, line tubes, point sprites, the
> ground, the UI — is one of these: a **pipeline** (the fixed recipe) fed by **buffers** (the data)
> inside a **render pass** (the frame). The course builds ~10 pipelines; all of them are this
> lesson's descriptor with different fields filled in. Learn what each part does *once*, here, and
> every later `build_*_pipeline` reads as "same recipe, three fields changed".

Draw a single triangle on the grey background.

Chapter 3 only *cleared* the screen; now the GPU *draws*. The smallest thing a GPU
can draw: one triangle whose 3 corners are written **inside the shader** — no
vertex buffers yet (those come later). This chapter proves the drawing machinery
works.

<svg viewBox="0 0 680 210" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the GPU pipeline stages from vertex input through vertex shader, primitive assembly, rasterizer, fragment shader, to the color target — each stage labeled with the descriptor field that configures it" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="8" y="40" width="96" height="44"/><rect x="128" y="40" width="96" height="44"/><rect x="248" y="40" width="110" height="44"/><rect x="382" y="40" width="96" height="44"/><rect x="502" y="40" width="96" height="44"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="56" y="58">vertex input</text><text x="56" y="72" fill="#666" font-size="9">(none today)</text>
    <text x="176" y="58">vertex shader</text><text x="176" y="72" fill="#666" font-size="9">where is corner i?</text>
    <text x="303" y="58">assembly + raster</text><text x="303" y="72" fill="#666" font-size="9">3 verts → pixels inside</text>
    <text x="430" y="58">fragment shader</text><text x="430" y="72" fill="#666" font-size="9">what colour is this px?</text>
    <text x="550" y="58">color target</text><text x="550" y="72" fill="#666" font-size="9">the swapchain image</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.3">
    <line x1="104" y1="62" x2="126" y2="62" marker-end="url(#ah04)"/><line x1="224" y1="62" x2="246" y2="62" marker-end="url(#ah04)"/><line x1="358" y1="62" x2="380" y2="62" marker-end="url(#ah04)"/><line x1="478" y1="62" x2="500" y2="62" marker-end="url(#ah04)"/>
  </g>
  <g fill="#888" text-anchor="middle" font-size="10">
    <text x="56" y="110">vertex:</text><text x="56" y="123">buffers</text>
    <text x="176" y="110">vertex:</text><text x="176" y="123">module + entry_point</text>
    <text x="303" y="110">primitive:</text><text x="303" y="123">topology · cull · face</text>
    <text x="430" y="110">fragment:</text><text x="430" y="123">module + entry_point</text>
    <text x="550" y="110">fragment.targets:</text><text x="550" y="123">format · blend · mask</text>
  </g>
  <text x="340" y="155" fill="#666" text-anchor="middle" font-size="10">grey row = the RenderPipelineDescriptor field that configures each stage</text>
  <text x="340" y="180" fill="#555" text-anchor="middle" font-size="10">not shown: depth_stencil (12/26 — the depth test between raster and target) · multisample (24) · layout (07 — external data)</text>
  <defs><marker id="ah04" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>


## Mental model (read this first)

Three terms, and the rest is easy:

- **Shader** — a tiny GPU program, written in **WGSL** (`.wgsl`). Two parts: the
  **vertex shader** decides *where* each corner goes; the **fragment shader**
  decides *what colour* each pixel is.
- **Pipeline** — the *recipe*: "use this shader, draw triangles, write into this
  pixel format". Built once, reused every frame.
- **Draw call** — `draw(0..3, 0..1)` = run the vertex shader 3 times (3 corners =
  1 triangle), 1 copy.


## The file tree (what we add)

Same layout as the archive: shaders in `src/shaders/`, pipeline builders in
`src/engine/pipelines/`.

```bash
session_viewer/
└── src/
    ├── shaders/
    │   └── triangle.wgsl       # NEW — the GPU program (corners + colour)
    ├── engine/
    │   ├── mod.rs              # EDIT — add one line: `pub mod pipelines;`
    │   ├── gpu.rs              # EDIT — hold the pipeline + draw it each frame
    │   └── pipelines/
    │       ├── mod.rs          # NEW — the `Pipelines` struct (holds the recipe)
    │       └── build.rs        # NEW — the function that builds the recipe
```

`lib.rs` and `state.rs` don't change — all new work is in `gpu.rs` and the two
files it leans on.


## Step 1 — the shader: `src/shaders/triangle.wgsl`

The GPU program. Create the file with exactly this:

```wgsl
// One hard-coded triangle. No vertex buffer — the 3 corners live here in the
// shader and are picked by the vertex number (0, 1, 2). Simplest possible draw.

struct VsOut {
    @builtin(position) pos: vec4<f32>,   // where this corner lands on screen
    @location(0) color: vec3<f32>,       // colour passed to the fragment shader
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    // vi is 0, 1, or 2. Look up that corner.
    var positions = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),   // 0: top
        vec2<f32>(-0.5, -0.5),   // 1: bottom-left
        vec2<f32>( 0.5, -0.5),   // 2: bottom-right
    );
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),   // red
        vec3<f32>(0.0, 1.0, 0.0),   // green
        vec3<f32>(0.0, 0.0, 1.0),   // blue
    );
    var o: VsOut;
    o.pos   = vec4<f32>(positions[vi], 0.0, 1.0);
    o.color = colors[vi];
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);   // colour is smoothly blended between corners
}
```

Notice:
- Screen coordinates run `-1` to `+1`, `(0, 0)` centre, `+y` up — so `(0.0, 0.5)` is
  top-middle. This is **clip space** (camera/3D comes later; for now corners are
  placed directly).
- The vertex shader **returns** a `VsOut`; the GPU blends those values across the
  triangle and hands the result to the fragment shader — hence the smooth rainbow
  inside a red+green+blue triangle.


## Step 2 — the pipeline builder: `src/engine/pipelines/build.rs`

The recipe constructor — a trimmed copy of the archive's `build_background_pipeline`
(its simplest pipeline: no buffers, no bind groups). Create the file:

```rust

/// The simplest possible pipeline: one shader, no vertex buffers, no bind groups,
/// no depth. Just draws the 3 hard-coded corners from triangle.wgsl.
pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    // 1. Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("triangle.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/triangle.wgsl").into()),
    });

    // 2. Layout = what external data the shader reads. Ours reads nothing → empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("triangle.layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    // 3. The recipe itself.
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("triangle"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),   // the @vertex function
            buffers: &[],                   // no vertex buffer — corners are in the shader
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),   // the @fragment function
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,       // must match the surface's pixel format
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,  // every 3 vertices = 1 triangle
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,                // don't hide back-facing triangles
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,   // no 3D depth test yet
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

Three fields matter today: the **shader module** (step 1), the **entry points**
(`vs_main`/`fs_main`), and the **format** (must match the surface). But every field in that
descriptor configures a real stage of the GPU — and each one becomes load-bearing in a specific
later lesson. This table is the reference the whole course leans on:

## Anatomy — what each part of the recipe does

| field | what it configures | today | becomes load-bearing in |
|---|---|---|---|
| `layout` | what **external data** the shader may read: the list of bind-group layouts (uniforms, storage buffers, textures) — the shader's function signature toward the CPU | empty — our shader reads nothing | **07** (first uniform), **29/31** (storage tables), **72** (texture) |
| `vertex.module` + `entry_point` | which compiled WGSL function runs **per vertex** — it answers "where does corner *i* land in clip space?" | `vs_main` | every shader lesson |
| `vertex.buffers` | the **memory layout** of per-vertex input: stride, and which bytes feed which `@location` — how the GPU walks your `Vec<Vertex>` | empty — corners are hard-coded in the shader | **06** (first vertex buffer), **30** (the per-vertex id, slot 1) |
| `fragment.module` + `entry_point` | which function runs **per covered pixel** — "what colour is this pixel?" | `fs_main` | every shader lesson |
| `fragment.targets[0].format` | the pixel format of the image being drawn into — must equal the surface's, or the draw is rejected | `config.format` | stays; offscreen targets (71) add their own |
| `fragment.targets[0].blend` | how the fragment's colour **combines** with what's already in the target: replace, or alpha-mix | `None` = replace (opaque) | **32b** (translucent point sprites), **65** (ground fade) |
| `fragment.targets[0].write_mask` | which colour channels the draw may touch | ALL | stays ALL |
| `primitive.topology` | how the vertex stream groups into shapes: every 3 = a triangle (`TriangleList`), every 2 = a line (`LineList`), … | `TriangleList` | **20** (the grid is `LineList`) |
| `primitive.front_face` + `cull_mode` | which winding order counts as "front", and whether back-facing triangles are **skipped** before rasterizing (an inside-a-box optimization) | `Ccw`, no culling | stays off — CAD views solids from inside too |
| `primitive.polygon_mode` | fill triangles, or draw only their edges/vertices (debug looks) | `Fill` | stays `Fill` (real wireframes are 31's tubes) |
| `depth_stencil` | the **depth test**: compare each fragment's z against the depth buffer, keep the winner — what makes near things hide far things | `None` — one triangle has nothing to hide | **12** (depth buffer), **26** (reverse-Z: compare `Greater`) |
| `multisample` | how many coverage **samples per pixel** — the hardware anti-aliasing that smooths edges | default = 1 | **24** (4× MSAA), **69** (coverage-mask outline) |
| `multiview_mask` / `cache` | multi-layer rendering / pipeline caching — niche | `None` | never, in this course |

Read it top to bottom and it's the SVG above in words: *data in* (`layout`, `buffers`) → *vertex
stage* → *assembly* (`primitive`) → *depth* (`depth_stencil`) → *fragment stage* → *output*
(`targets`, `multisample`). Every one of the ~10 pipelines this course builds is exactly this
descriptor with three or four rows changed — and each builder lesson names which rows.


## Step 3 — the `Pipelines` struct: `src/engine/pipelines/mod.rs`

The archive keeps all pipelines in one `Pipelines` struct, built in `new()` — same
shape here, just one field today. Create the file:

```rust
//! Render pipelines. Today just one: the hard-coded triangle. As the viewer grows,
//! this struct gains fields (mesh, line, grid, …), each built here in `new()`.

mod build;

use build::build_triangle_pipeline;

pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, color_format: wgpu::TextureFormat) -> Self {
        Self {
            triangle: build_triangle_pipeline(device, color_format),
        }
    }
}
```

Same shape as the archive's `Pipelines::new`, one line instead of twenty. Every
future pipeline: add a field here, add a builder in `build.rs`.


## Step 4 — register the module: `src/engine/mod.rs`

Add one line so `engine` knows `pipelines` exists:

```rust
pub mod gpu;
pub mod pipelines;   // <- ADD THIS
```


## Step 5 — hold the pipeline in `Gpu`: `src/engine/gpu.rs`

Built once, reused every frame — `Gpu` stores it. Three edits.

**(a) Import it** (top of the file, near the other `use`s):

```rust
use crate::engine::pipelines::Pipelines;
```

**(b) Add a field** to the struct:

```rust
pub struct Gpu {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub pipelines: Pipelines,        // <- ADD THIS
}
```

**(c) Build it in `new()`**, right after `surface.configure(&device, &config);`,
then add it to the returned struct:

```rust
    surface.configure(&device, &config);

    let pipelines = Pipelines::new(&device, config.format);   // <- ADD THIS

    log::info!("viewer init OK — surface {}x{}, format {:?}",
        config.width, config.height, config.format);
    Ok(Self { surface, device, queue, config, pipelines })    // <- add `pipelines`
}
```

`config.format` must be the same pixel format passed to the pipeline in Step 2, or
wgpu rejects the draw.


## Step 6 — draw it: the render pass in `gpu.rs`

The payoff. Chapter 3's render pass only cleared; now it gets two commands: "use
the triangle recipe" and "draw 3 vertices".

Find the `clear` method's render-pass block and change it from this:

```rust
{
    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        // ... descriptor unchanged ...
    });
}
```

to this:

```rust
{
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        // ... descriptor unchanged ...
    });
    pass.set_pipeline(&self.pipelines.triangle);   // use the recipe
    pass.draw(0..3, 0..1);                          // 3 vertices (1 triangle), 1 instance
}
```

Two changes: `let _pass` becomes `let mut pass` (we need to call methods on it),
plus the two new lines before the block closes.


## Step 7 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

A triangle on the grey background: red top, green bottom-left, blue bottom-right,
blending smoothly inside.

Still plain grey? Check the browser console (F12):
- WGSL error → typo in `triangle.wgsl` (compiled at runtime, so errors show up
  there, not at `cargo build`).
- "format mismatch" → Step 5(c) didn't pass `config.format`, or Step 2's target
  format differs.


## What changed vs Chapter 3 (recap)

```
Chapter 3:  begin render pass → clear grey → end → present
Chapter 4:  begin render pass → clear grey → set_pipeline → draw 3 → end → present
                                              └── new: the triangle ──┘
```

New files: `shaders/triangle.wgsl`, `engine/pipelines/mod.rs`, `engine/pipelines/build.rs`.
Edited: `engine/mod.rs` (one line), `engine/gpu.rs` (import + field + build + 2 draw lines).
Untouched: `lib.rs`, `state.rs`.


## Next

Move the triangle's corners out of the shader into a **vertex buffer** — the first
real data upload to the GPU (what `bytemuck` is for). The gateway to drawing actual
geometry from the kernel.
