# 24 MSAA — smooth edges and lines everywhere

Zoom into the box's silhouette or a glancing grid line and you'll see the staircase: hard pixel steps
on every slanted edge. That's **aliasing** — one sample per pixel, so a pixel is either fully edge
color or fully background. The cheapest fix is **MSAA** (multisample anti-aliasing): render into a
texture holding **4 samples per pixel**, let the GPU decide how many a triangle covers, then average
down to one final pixel — soft gradient steps instead of hard ones.

## Why

```
1 sample / pixel                 4 samples / pixel (MSAA), then resolve
┌───┬───┬───┐                    ┌───┬───┬───┐        edge covers 2 of 4
│ ■ │ ■ │   │  hard step         │▞▞ │▞▞ │   │  →  averaged: 50% edge, 50% bg
└───┴───┴───┘                    └───┴───┴───┘        soft step
```

MSAA is nearly free: extra samples matter only at triangle *edges*, so the fragment shader still runs
**once per pixel**. The cost is memory (4× color + 4× depth texture) and one **resolve** step
averaging the 4 samples into the single-sample surface.

Three things have to agree or wgpu rejects the frame:

```
render pass attachments  →  4 samples   (a 4× color texture + a 4× depth texture)
every pipeline           →  4 samples   (triangle, grid, edges — ALL of them)
the surface itself       →  1 sample    (unchanged — the resolve target we average INTO)
```

The surface stays single-sample: you never *present* 4 samples, only the resolved average.

<svg viewBox="0 0 600 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="4x color and depth textures resolve into the single-sample surface" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="90" y="16" fill="#888" text-anchor="middle">4× color</text>
  <rect x="30" y="26" width="120" height="70" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="90" y="64" fill="#d7dae0" text-anchor="middle">msaa_view</text>
  <text x="90" y="80" fill="#666" text-anchor="middle">4 samples/px</text>
  <text x="230" y="16" fill="#888" text-anchor="middle">4× depth</text>
  <rect x="170" y="26" width="120" height="70" fill="none" stroke="#3a3a3a" stroke-width="1.5"/>
  <text x="230" y="64" fill="#d7dae0" text-anchor="middle">depth_view</text>
  <text x="230" y="80" fill="#666" text-anchor="middle">4 samples/px</text>
  <text x="345" y="62" fill="#6fb3ff" font-size="16">▶</text>
  <text x="345" y="78" fill="#666" font-size="10">resolve</text>
  <text x="480" y="16" fill="#888" text-anchor="middle">surface</text>
  <rect x="400" y="26" width="160" height="70" fill="none" stroke="#6fb3ff" stroke-width="1.5"/>
  <text x="480" y="64" fill="#d7dae0" text-anchor="middle">resolve_target</text>
  <text x="480" y="80" fill="#666" text-anchor="middle">1 sample/px — presented</text>
</svg>

## Files we touch

```
src/engine/pipelines/build.rs   # a sample-count const + multisample.count on every pipeline
src/engine/gpu.rs               # 4× color + 4× depth textures; resolve in the render pass; resize
```

## Step 1 — one sample-count knob: `src/engine/pipelines/build.rs`

Put the sample count in one `const` — a single MSAA knob (`1` = off, `4` = normal, universally
supported). Add it at the top of the file:

```rust
/// Samples per pixel for MSAA. 1 = off, 4 = standard (universally supported). Must match the
/// sample_count of the color+depth textures in gpu.rs.
pub const MSAA_SAMPLES: u32 = 4;
```

Then in **each** of the three pipeline builders, replace the default multisample state with the
const. Change `multisample: wgpu::MultisampleState::default(),` (1 sample) to:

```rust
            multisample: wgpu::MultisampleState {
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
```

Do this in `build_triangle_pipeline`, `build_grid_pipeline`, **and** `build_edges_pipeline` — miss
one and wgpu throws a validation error the moment it draws into the 4× pass, since a pipeline's
sample count must equal its attachment's.

## Step 2 — two multisampled targets: `src/engine/gpu.rs`

The depth texture must now hold 4 samples too (depth and color counts must match), plus a **new** 4×
color texture to render into. Import the const, bump `create_depth_view` from `sample_count: 1` to
it, then add a twin helper for color:

```rust
use crate::engine::pipelines::build::MSAA_SAMPLES;
```

```rust
    fn create_depth_view(device: &wgpu::Device,
                         config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1),
                depth_or_array_layers: 1 },
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
    fn create_msaa_view(device: &wgpu::Device,
                        config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("msaa_color"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1),
                depth_or_array_layers: 1 },
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

The line that makes MSAA actually happen. In `clear()`, the color attachment now renders into the
**4× texture** and names the **surface** as its `resolve_target` — wgpu averages the 4 samples
automatically when the pass ends. The depth attachment needs no change — only its texture got
deeper:

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

Everything else in the pass — depth attachment, grid/mesh/edge draws — is untouched; they were
already drawing into `self.depth_view`, now simply 4-sample.

## Step 4 — rebuild both on resize: `src/engine/gpu.rs`

MSAA textures are sized to the window, so they must be recreated on resize, like the depth texture
already is. In `resize()`, next to the depth line:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config);
            self.msaa_view = Self::create_msaa_view(&self.device, &self.config);   // ← new
```

Forget this and the app runs fine until the first resize, then panics on a size mismatch between the
old MSAA texture and the new surface.

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Orbit and zoom into any silhouette: box edges, dodecahedron pentagons, grid lines, colored frame axes
are all smooth now — no staircase. To compare with MSAA off, flip `MSAA_SAMPLES` to `1` **and** revert
Step 3's attachment for the experiment (`view: &view, resolve_target: None`) — wgpu rejects a
single-sample texture as a resolve source. Jaggies return, proving the effect rides on that one const.

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
