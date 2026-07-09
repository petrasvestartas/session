# 25 Background gradient — the scene stops floating in flat grey

Right now everything sits on a flat grey wash — the surface clear color. Real CAD viewers put a soft
**vertical gradient** back there (Rhino, SolidWorks, Fusion all do): lighter near the horizon,
deeper toward the top. It reads as depth and light without drawing anything real. This lesson paints
one with a **fourth pipeline** — and it's the cheapest pipeline yet: no camera, no vertex buffer,
three vertices, drawn once before everything else.

## Why

```
draw order this frame          depth-write   what it does
─────────────────────────────  ───────────   ────────────────────────────────
1. background  (TriangleList)   off           fills EVERY pixel with the gradient
2. grid        (LineList)       off           floor lines over the gradient
3. meshes      (TriangleList)   ON            solids, depth-tested, paint over both
4. edges       (LineList)       off           dark outlines, nudged in front
```

The background is a **fullscreen triangle** — one big triangle that overhangs the screen so its
interior covers all of `[-1,1]²`. It comes straight from `@builtin(vertex_index)` (no buffer, same
vertexless trick as the grid), and the fragment shader mixes two colors by screen height.

Two rules keep it *behind* everything without a camera:

```
depth_write: false   → it never blocks a later fragment
depth_compare: Always → it always draws (its z sits at the far plane; a `Less` test would reject it)
```

Because it writes no depth and is drawn first, every later object paints over it by normal depth
testing. The clear color is now never seen — the gradient covers the whole frame.

## Files we touch

```
src/shaders/background.wgsl        # NEW — fullscreen triangle + vertical color mix
src/engine/pipelines/build.rs      # build_background_pipeline (no bind groups, Always depth)
src/engine/pipelines/mod.rs        # add `background` to Pipelines
src/engine/gpu.rs                  # draw it FIRST in clear()
```

## Step 1 — the background shader: `src/shaders/background.wgsl`

No uniforms at all — the vertex shader hardcodes the three corners of a screen-covering triangle and
hands the fragment shader a `0→1` height value; the fragment mixes a bottom and top color by it:

```wgsl
struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) t: f32,           // 0 at the bottom of the screen, 1 at the top
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    // One oversized triangle whose interior covers the whole [-1,1] screen:
    //   (-1,-1)  (3,-1)  (-1,3)
    var corners = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    let p = corners[vid];
    var o: VsOut;
    o.pos = vec4<f32>(p, 1.0, 1.0);   // z = 1.0 = far plane; depth_compare Always draws it anyway
    o.t   = p.y * 0.5 + 0.5;          // NDC y (-1..1) → 0..1 across the visible screen
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let bottom = vec3<f32>(0.86, 0.89, 0.92);   // pale near the horizon
    let top    = vec3<f32>(0.52, 0.62, 0.75);   // deeper blue up high
    return vec4<f32>(mix(bottom, top, clamp(in.t, 0.0, 1.0)), 1.0);
}
```

Change `bottom`/`top` to taste — those two lines are the whole look. (The third corner sits at `y=3`,
off-screen, so `t` runs a clean `0→1` across the part you actually see.)

## Step 2 — the pipeline: `src/engine/pipelines/build.rs`

Simplest builder in the file — **no bind-group layouts** (the shader reads nothing) and **no vertex
buffer**. The two things that matter: depth `Always` + write-off so it never occludes, and
`count: MSAA_SAMPLES` so it's compatible with the 4× pass from lesson 24 (forget that and wgpu
validation-errors, exactly like a missed MSAA pipeline there):

```rust
pub fn build_background_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("background.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/background.wgsl").into()),
    });

    // No external data → an empty pipeline layout (no bind groups).
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("background.layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("background"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],                          // vertexless — positions from vertex_index
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
            depth_write_enabled: Some(false),                    // never blocks later fragments
            depth_compare: Some(wgpu::CompareFunction::Always),  // always draws (z is at the far plane)
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: MSAA_SAMPLES,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
```

`MSAA_SAMPLES` is already `pub` at the top of this file (lesson 24), so no new import.

## Step 3 — register it: `src/engine/pipelines/mod.rs`

A `background` field, built with just device + format (it needs no layouts):

```rust
use build::build_background_pipeline;   // add to the other `use build::…` lines

pub struct Pipelines {
    pub triangle: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,      // ← new
}
// …in new():
        background: build_background_pipeline(device, color_format),
```

## Step 4 — draw it first: `src/engine/gpu.rs`

In `clear()`, at the very top of the render pass — **before** the grid — set the background pipeline
and draw three vertices. It binds nothing (no camera, no time):

```rust
            // Background FIRST — fills every pixel; depth-write off so all geometry paints over it
            pass.set_pipeline(&self.pipelines.background);
            pass.draw(0..3, 0..1);

            // Grid next (unchanged)
            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.draw(0..50, 0..1);
```

The `LoadOp::Clear(color)` on the color attachment still runs, but you'll never see it again — the
background triangle covers every pixel the same frame. (You can leave the clear as-is; it's now just
a cheap wipe before the gradient.)

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The flat grey is gone: a soft blue-grey gradient sits behind the grid and models, lighter at the
bottom, deeper at the top. Orbit — the gradient is locked to the **screen**, not the world (it's
drawn in NDC, no camera), so it stays put like a studio backdrop while the scene turns inside it.

## Recap

```
Ch 24: crisp edges — but the scene floats on a flat grey clear color.
Ch 25: a fourth pipeline paints a fullscreen-triangle gradient FIRST, from @builtin(vertex_index)
       (no buffer, no camera), depth-write off + compare Always so every later object covers it.
       The clear color is never seen again. Two color lines in background.wgsl are the whole look.
```

Edited: `shaders/background.wgsl` (new), `pipelines/build.rs` (`build_background_pipeline`),
`pipelines/mod.rs` (`background` field), `engine/gpu.rs` (draw first in `clear()`).

## Next

`26-reverse-z.md` — the last camera fix. We reverse the depth mapping (near → 1, far → 0) so f32
depth precision stops collapsing in the distance — the proper cure for the z-fighting the lesson-23
edge nudge and the tightened clip range only worked *around*. Grid, edges, and background keep their
`Always`/off states; only the compare direction and clear value flip.
