# 03 Pipeline

Draw a single triangle on the grey background.

In Chapter 2 the GPU only *cleared* the screen. Now we make it *draw*. We add the smallest possible thing a GPU can draw: one triangle whose 3 corners are written **inside the shader** — no data buffers yet. Buffers come in a later chapter; this chapter proves the drawing machinery works.


## Mental model (read this first)

Three new words. Keep them straight and the rest is easy.

- **Shader** — a tiny program that runs *on the GPU*. We write it in a language
  called **WGSL** (`.wgsl` file). It has two parts:
  - the **vertex shader** decides *where* each corner of the triangle goes,
  - the **fragment shader** decides *what colour* each pixel is.
- **Pipeline** — the *recipe* that tells the GPU: "use this shader, draw triangles,
  write into this pixel format". You build it once, then reuse it every frame.
- **Draw call** — the actual command `draw(0..3, 0..1)` = "run the vertex shader 3
  times (3 corners = 1 triangle), 1 copy of it".


## The file tree (what we add)

We copy the archive's layout exactly: shaders live in `src/shaders/`, pipeline
builders live in `src/engine/pipelines/`.

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

`lib.rs` and `state.rs` do **not** change. All the new work is inside `gpu.rs` and
the two new files it leans on.


## Step 1 — the shader: `src/shaders/triangle.wgsl`

This is the GPU program. Create the file with exactly this:

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

What to notice:
- Screen coordinates here go from `-1` to `+1`. `(0, 0)` is the centre, `+y` is up.
  So `(0.0, 0.5)` is top-middle. This space is called **clip space** (camera/3D
  comes in a later chapter — for now we place corners directly).
- The vertex shader **returns** a `VsOut`; the GPU automatically blends those values
  across the triangle and hands the blended result to the fragment shader. That's
  why a red+green+blue triangle has a smooth rainbow inside.


## Step 2 — the pipeline builder: `src/engine/pipelines/build.rs`

This is the recipe constructor. It is a trimmed-down copy of the archive's
`build_background_pipeline` (the archive's simplest pipeline — also no buffers, no
bind groups). Create the file:

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

You don't need to memorise every field — most are `None`/`default` and stay that way
for chapters. The three that matter: the **shader module** (step 1), the
**entry points** (`vs_main`/`fs_main`), and the **format** (must match the surface).


## Step 3 — the `Pipelines` struct: `src/engine/pipelines/mod.rs`

The archive keeps all its pipelines in one struct called `Pipelines`, built in a
`new()`. We do the same — but with a single field today. Create the file:

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

This is the same shape as the archive's `Pipelines::new` — just with one line
instead of twenty. Every future pipeline is "add a field here, add a builder in
`build.rs`".


## Step 4 — register the module: `src/engine/mod.rs`

The engine folder needs to know `pipelines` exists. Add one line:

```rust
pub mod gpu;
pub mod pipelines;   // <- ADD THIS
```


## Step 5 — hold the pipeline in `Gpu`: `src/engine/gpu.rs`

The recipe is built once and reused every frame, so `Gpu` stores it. Three small
edits.

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

    log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
    Ok(Self { surface, device, queue, config, pipelines })    // <- add `pipelines`
}
```

Note we pass `config.format` — that's the same pixel format we told the pipeline to
target in Step 2. They must agree, or wgpu rejects the draw.


## Step 6 — draw it: the render pass in `gpu.rs`

This is the payoff. In Chapter 2 the render pass was empty — it just cleared. Now we
give it two commands: "use the triangle recipe" and "draw 3 vertices".

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

Two changes only: `let _pass` becomes `let mut pass` (we need to call methods on it),
and the two new lines before the block closes.


## Step 7 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

You should see a triangle on the grey background: red at the top, green bottom-left,
blue bottom-right, with the colours blending smoothly inside.

If the screen is still plain grey, check the browser console (F12):
- a WGSL error → typo in `triangle.wgsl` (it's compiled at runtime, so errors show
  there, not at `cargo build`).
- "format mismatch" → Step 5(c) didn't pass `config.format`, or Step 2's target
  format differs.


## What changed vs Chapter 2 (recap)

```
Chapter 2:  begin render pass → clear grey → end → present
Chapter 3:  begin render pass → clear grey → set_pipeline → draw 3 → end → present
                                              └── new: the triangle ──┘
```

New files: `shaders/triangle.wgsl`, `engine/pipelines/mod.rs`, `engine/pipelines/build.rs`.
Edited: `engine/mod.rs` (one line), `engine/gpu.rs` (import + field + build + 2 draw lines).
Untouched: `lib.rs`, `state.rs`.


## Next

Move the triangle's corners out of the shader and into a **vertex buffer** — the
first time we upload real data to the GPU (this is what `bytemuck` in the deps is
for). That's the gateway to drawing actual geometry from the kernel.
