# 12 Depth buffer

Add a **depth buffer** so nearer surfaces correctly hide farther ones — no matter what order
we draw them in. So far the *last* triangle drawn always paints over the earlier ones, even
when it's physically behind. A depth buffer makes **distance**, not draw order, decide.

## How a depth buffer works

A depth buffer is a second texture, the size of the screen, holding one number per pixel: the
depth of the *closest* fragment drawn so far. Each new fragment compares its depth against that
value and is kept only if it's nearer — so distance, not draw order, decides who wins. We clear
it to `1.0` (the far plane — every pixel starts maximally distant) and keep a fragment when its
depth is **less** than what's stored. We use a `Depth32Float` texture.

```
depth buffer:   per-pixel "nearest so far"; farther fragments are discarded
standard depth: near -> 0.0, far -> 1.0    (clear 1.0, test Less, write the nearer)
```

> **Aside — reverse-Z (deferred).** You'll hear that *reverse-Z* (map near→`1.0`, far→`0.0`,
> clear `0.0`, test `Greater`) gives better `float32` precision far from the camera, because the
> `1/z` curve crowds the distance exactly where float is densest. That's real — but in **WebGPU**
> you can't get it from the viewport: `setViewport(…, minDepth, maxDepth)` requires
> `minDepth ≤ maxDepth`, so a `1.0 → 0.0` swap is **rejected at runtime** (that's the validation
> error if you try it). Proper reverse-Z lives in the **projection matrix**; we'll revisit it with
> the camera-precision work. For two triangles at the origin, plain standard depth is all we need.

## Files we touch

```
src/engine/pipelines/build.rs   # the pipeline gains a depth_stencil (Depth32Float, Less)
src/engine/gpu.rs               # a depth texture; attach it and clear to 1.0; a 2nd triangle
```


## Step 1 — a depth texture: `gpu.rs`

**(a)** Add the depth view to `struct Gpu`:

```rust
    pub depth_view: wgpu::TextureView,
```

**(b)** A helper that builds (and rebuilds) it at the surface size — add it inside `impl Gpu`:

```rust
    fn create_depth_view(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
```

**(c)** Build it in `new` (right after `surface.configure(&device, &config);`) and add the field
to the returned `Ok(Self { … })`:

```rust
        let depth_view = Self::create_depth_view(&device, &config);
        // …
        Ok(Self { /* …existing fields…, */ depth_view })
```

**(d)** The depth texture must always match the canvas, so re-create it on resize — in `resize`,
after `self.surface.configure(...)`. **Mind the `self.`**: in `new`, `device` and `config` are
local variables, but in `resize` they're *fields*, so you pass `&self.device, &self.config`.
Bare `&device, &config` (the `new` form) won't compile here — those names aren't in scope:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config);
```


## Step 2 — depth test in the pipeline: `build.rs`

Replace the pipeline's `depth_stencil: None,` with a real depth state:

```rust
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),   // standard depth: keep the nearer (smaller) depth
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
```


## Step 3 — attach the depth buffer + two objects: `gpu.rs`

**(a)** Give the render pass a depth attachment, cleared to `1.0` (the far plane). This is **not**
in `create_render_pipeline` — that was Step 2's `depth_stencil`. This goes in the
`wgpu::RenderPassDescriptor` inside the `clear()` method: find the `depth_stencil_attachment: None,`
line (right after `color_attachments: &[ … ]`) and replace the `None` with:

```rust
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),    // 1.0 = far; every pixel starts maximally distant
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
```

**(b)** Now give the depth test something to prove: draw **two triangles at different depths in
space**. The orange one sits in front at `z = +0.3`; the blue one is behind it at `z = -0.3` and
is drawn **second** — so without the depth buffer it would wrongly paint over the orange one.
Replace the `TRIANGLE` constant (and rename `TRIANGLE` → `TRIANGLES` in the two lines that build
`vertex_buffer` and `num_vertices`):

```rust
        const TRIANGLES: &[Vertex] = &[
            // near triangle (drawn first), z = +0.3 — orange, in front
            Vertex { position: [-0.2,  0.5,  0.3], color: [1.0, 0.5, 0.1] },
            Vertex { position: [-0.7, -0.4,  0.3], color: [1.0, 0.5, 0.1] },
            Vertex { position: [ 0.3, -0.4,  0.3], color: [1.0, 0.5, 0.1] },
            // far triangle (drawn second), z = -0.3 — blue, behind
            Vertex { position: [ 0.2,  0.5, -0.3], color: [0.1, 0.5, 1.0] },
            Vertex { position: [-0.3, -0.4, -0.3], color: [0.1, 0.5, 1.0] },
            Vertex { position: [ 0.7, -0.4, -0.3], color: [0.1, 0.5, 1.0] },
        ];
```


## Step 4 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The orange triangle sits correctly in front of the blue one — and **stays** correct as you
right-drag to orbit around them in 3D. As a test, set `depth_compare: Some(wgpu::CompareFunction::Always)`
(or drop the depth attachment): the blue triangle, drawn *last*, wrongly jumps in front. That's
the draw-order bug the depth buffer removes.


## Recap

```
Ch 11: one triangle; last-drawn wins (no depth)
Ch 12: depth texture + Less test (clear 1.0); the nearer fragment wins, whatever the draw order
```

Edited: `build.rs` (`depth_stencil`), `gpu.rs` (`depth_view` field + `create_depth_view` +
build it in `new`/`resize`, depth attachment cleared to `1.0`, a second triangle). The shader,
mvp uniform, and camera are unchanged.


## Next

`13-camera-module.md` — pull the camera state and orbit/pan/zoom + `view_proj` out of `gpu.rs`
into its own `camera.rs` (`Camera` + `CameraController`): the "distribute, don't smash" refactor.
