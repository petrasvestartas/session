# 12 Depth buffer

Add a **depth buffer** so nearer surfaces hide farther ones, regardless of draw order. Right
now the *last* triangle drawn always paints over earlier ones, even when it's physically
behind. A depth buffer makes **distance**, not draw order, decide.

<svg viewBox="0 0 420 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a ray through one pixel hits both triangles; the depth test keeps the nearer orange hit" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <circle cx="20" cy="65" r="3" fill="#d7dae0"/>
  <text x="20" y="82" fill="#666" text-anchor="middle">eye</text>
  <line x1="20" y1="65" x2="400" y2="65" stroke="#555" stroke-width="1" stroke-dasharray="2,2"/>
  <line x1="160" y1="20" x2="160" y2="110" stroke="#e08a3c" stroke-width="3"/>
  <text x="160" y="16" fill="#e08a3c" text-anchor="middle">near · z=+0.3 · orange</text>
  <line x1="300" y1="20" x2="300" y2="110" stroke="#6fb3ff" stroke-width="3"/>
  <text x="300" y="16" fill="#6fb3ff" text-anchor="middle">far · z=-0.3 · blue</text>
  <circle cx="160" cy="65" r="4" fill="#d7dae0"/>
  <text x="210" y="100" fill="#d7dae0" text-anchor="middle">depth test keeps this hit — draw order doesn't matter</text>
</svg>

## How a depth buffer works

A depth buffer is a second screen-sized texture holding one number per pixel: the depth of the
*closest* fragment drawn so far. Each new fragment compares against that value and survives
only if nearer. Cleared to `1.0` (far — every pixel starts maximally distant); a fragment
passes when its depth is **less** than what's stored. Format: `Depth32Float`.

```
depth buffer:   per-pixel "nearest so far"; farther fragments are discarded
standard depth: near -> 0.0, far -> 1.0    (clear 1.0, test Less, write the nearer)
```

> **Aside — reverse-Z (deferred).** *Reverse-Z* (near→`1.0`, far→`0.0`, clear `0.0`, test
> `Greater`) gives better `float32` precision far from the camera — the `1/z` curve crowds
> precision exactly where float is densest. Real benefit, but not from the viewport:
> `setViewport(…, minDepth, maxDepth)` requires `minDepth ≤ maxDepth`, so a `1.0 → 0.0` swap is
> **rejected at runtime**. Proper reverse-Z lives in the **projection matrix** — revisited with
> the camera-precision work. For two triangles at the origin, standard depth is enough.

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
    fn create_depth_view(device: &wgpu::Device,
                         config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth"),
            size: wgpu::Extent3d { width: config.width.max(1), height: config.height.max(1),
                                   depth_or_array_layers: 1 },
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
```

then add `depth_view` at the end of the returned struct — the full line now reads:

```rust
        Ok(Self { surface, device, queue, config, pipelines,
                  mvp_buffer, mvp_bind_group, vertex_buffer, num_vertices,
                  time: 0.0, time_buffer, time_bind_group,
                  perspective: true, yaw: 0.6, pitch: 0.5, distance: 3.0,
                  target: [0.0, 0.0, 0.0], depth_view })
```

**(d)** The depth texture must match the canvas, so recreate it on resize — in `resize`, after
`self.surface.configure(...)`. **Mind the `self.`**: in `new`, `device`/`config` are local
variables; in `resize` they're *fields*, so pass `&self.device, &self.config` — the bare
`new`-style names aren't in scope here:

```rust
            self.depth_view = Self::create_depth_view(&self.device, &self.config);
```


## Step 2 — depth test in the pipeline: `build.rs`

Replace the pipeline's `depth_stencil: None,` with a real depth state:

```rust
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                // standard depth: keep the nearer (smaller) depth
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
```


## Step 3 — attach the depth buffer + two objects: `gpu.rs`

**(a)** Give the render pass a depth attachment, cleared to `1.0` (the far plane). Not in
`create_render_pipeline` — that was Step 2's `depth_stencil`. This goes in the
`wgpu::RenderPassDescriptor` inside `clear()`: find `depth_stencil_attachment: None,` (right
after `color_attachments: &[ … ]`) and replace `None` with:

```rust
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        // 1.0 = far; every pixel starts maximally distant
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
```

**(b)** Give the depth test something to prove: draw **two triangles at different depths**.
Orange sits in front at `z = +0.3`; blue is behind at `z = -0.3` and drawn **second** —
without depth testing it would wrongly paint over orange. Replace the `TRIANGLE` constant with:

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

and rename `TRIANGLE` → `TRIANGLES` in the two lines that build the buffer and the count:

```rust
            contents: bytemuck::cast_slice(TRIANGLES),   // &[Vertex] → &[u8]
```
```rust
        let num_vertices = TRIANGLES.len() as u32;
```


## Step 4 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Orange sits correctly in front of blue — and **stays** correct as you right-drag to orbit
around them in 3D. Test it: set `depth_compare: Some(wgpu::CompareFunction::Always)` (or drop
the depth attachment) and blue, drawn *last*, wrongly jumps in front. That's the draw-order
bug the depth buffer removes.


## Recap

```
Ch 11: one triangle; last-drawn wins (no depth)
Ch 12: depth texture + Less test (clear 1.0); the nearer fragment wins, whatever the draw order
```

Edited: `build.rs` (`depth_stencil`), `gpu.rs` (`depth_view` field + `create_depth_view`, built
in `new`/`resize`, depth attachment cleared to `1.0`, second triangle). Shader, mvp uniform,
and camera unchanged.


## Next

`13-camera-module.md` — camera state and orbit/pan/zoom + `view_proj` move out of `gpu.rs`
into `camera.rs` (`Camera` + `CameraController`): the "distribute, don't smash" refactor.
