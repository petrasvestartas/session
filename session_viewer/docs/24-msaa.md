# 24 MSAA — smooth edges and lines everywhere

Zoom in on the box's silhouette or a glancing grid line and you'll see the staircase: hard pixel
steps along every slanted edge. That's **aliasing** — one sample per pixel, so a pixel is either
fully the edge's color or fully the background, nothing in between. Every CAD app hides this, and
the cheapest fix is **MSAA** (multisample anti-aliasing): render into a texture that keeps **4
samples per pixel**, let the GPU decide how many of the 4 a triangle covers, then average them down
to one final pixel. Slanted edges get soft gradient steps instead of hard ones — instantly cleaner.

## Why

```
1 sample / pixel                 4 samples / pixel (MSAA), then resolve
┌───┬───┬───┐                    ┌───┬───┬───┐        edge covers 2 of 4
│ ■ │ ■ │   │  hard step         │▞▞ │▞▞ │   │  →  averaged: 50% edge, 50% bg
└───┴───┴───┘                    └───┴───┴───┘        soft step
```

MSAA is nearly free because the extra samples only matter at triangle *edges* — the fragment shader
still runs **once per pixel**, not once per sample. The cost is memory (a 4× color + 4× depth
texture) and one **resolve** step that averages the 4 samples into the single-sample surface.

Three things have to agree or wgpu rejects the frame:

```
render pass attachments  →  4 samples   (a 4× color texture + a 4× depth texture)
every pipeline           →  4 samples   (triangle, grid, edges — ALL of them)
the surface itself       →  1 sample    (unchanged — the resolve target we average INTO)
```

The surface stays single-sample: you never *present* 4 samples, you present the resolved average.

## Files we touch

```
src/engine/pipelines/build.rs   # a sample-count const + multisample.count on every pipeline
src/engine/gpu.rs               # 4× color + 4× depth textures; resolve in the render pass; resize
```

## Step 1 — one sample-count knob: `src/engine/pipelines/build.rs`

Put the sample count in a single `const` so the whole app has one MSAA knob (set it to `1` to turn
MSAA off, `4` for the normal quality — 4 is universally supported). Add it at the top of the file:

```rust
/// Samples per pixel for MSAA. 1 = off, 4 = standard (universally supported). Must match the
/// sample_count of the color+depth textures in gpu.rs.
pub const MSAA_SAMPLES: u32 = 4;
```

Then in **each** of the three pipeline builders, replace the default multisample state with one that
uses the const. Change `multisample: wgpu::MultisampleState::default(),` (which is 1 sample) to:

```rust
            multisample: wgpu::MultisampleState {
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
```

Do this in `build_triangle_pipeline`, `build_grid_pipeline`, **and** `build_edges_pipeline`. Miss
one and wgpu throws a validation error the moment that pipeline draws into the 4× pass (the gotcha
the roadmap warns about — the pipeline's sample count must equal the attachment's).

## Step 2 — two multisampled targets: `src/engine/gpu.rs`

The depth texture must now hold 4 samples too (depth and color sample counts have to match), and we
need a **new** 4× color texture to render into. Import the const and bump `create_depth_view` from
`sample_count: 1` to the const, then add a twin helper for the color texture:

```rust
use crate::engine::pipelines::build::MSAA_SAMPLES;
```

```rust
    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: MSAA_SAMPLES,          // ← was 1
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    /// The 4× color texture we render into; it gets resolved down to the single-sample surface
    /// at the end of the pass. Same format as the surface, MSAA sample count.
    fn create_msaa_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: MSAA_SAMPLES,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,               // must match the surface it resolves into
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
```

Add the field to the struct and build it in `new()` next to `depth_view`:

```rust
    pub msaa_view: wgpu::TextureView,      // in `pub struct Gpu { … }`
```

```rust
        let depth_view = Self::create_depth_view(&device, &config);
        let msaa_view = Self::create_msaa_view(&device, &config);   // ← new
```

…and return `msaa_view` in the `Ok(Self { … })` block alongside `depth_view`.

## Step 3 — resolve in the render pass: `src/engine/gpu.rs`

This is the one line that makes MSAA actually happen. In `clear()`, the color attachment now renders
into the **4× texture** and names the **surface** as its `resolve_target` — wgpu averages the 4
samples into the surface automatically when the pass ends. The depth attachment just points at the
now-4× depth view (no change to that line — only its texture got deeper):

```rust
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_view,             // ← render into the 4× texture (was &view)
                    resolve_target: Some(&view),       // ← average it down into the surface
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
```

Everything else in the pass — the depth attachment, the grid/mesh/edge draws — is untouched. They
were already drawing into `self.depth_view`; it's simply 4-sample now.

## Step 4 — rebuild both on resize: `src/engine/gpu.rs`

The MSAA textures are sized to the window, so they must be recreated when it changes, exactly like
the depth texture already is. In `resize()`, next to the depth line:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config);   // ← new
```

Forget this and the app renders fine until the first resize, then panics with a size mismatch
between the old MSAA texture and the new surface.

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Orbit and zoom into any silhouette: the box edges, the dodecahedron pentagons, the grid lines, and
the colored frame axes are all smooth now — no staircase. Flip `MSAA_SAMPLES` back to `1`, rebuild,
and compare: the jaggies return, proving the whole effect rides on that one const.

## Recap

```
Ch 23: dark edges over shaded solids — but every slanted edge and grid line is a pixel staircase.
Ch 24: render into a 4×-sample color texture + 4×-sample depth texture; set multisample.count = 4
       on EVERY pipeline; name the surface as the color attachment's resolve_target so the GPU
       averages 4 samples → 1 on present. Surface stays single-sample. One MSAA_SAMPLES const is
       the whole quality/off knob; recreate both textures on resize.
```

Edited: `pipelines/build.rs` (`MSAA_SAMPLES` const + `multisample.count` on all three pipelines),
`engine/gpu.rs` (`create_msaa_view`, 4× `create_depth_view`, `msaa_view` field + build in `new()`,
`resolve_target` in `clear()`, rebuild in `resize()`).

## Next

`25-background.md` — the scene still floats in flat grey. A fullscreen background shader (vertexless,
like the grid) paints a soft vertical gradient behind everything, drawn first with depth-write off so
all geometry covers it — the last cheap win before the viewer reads as a real CAD app.
