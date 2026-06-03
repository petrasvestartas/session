# 01 · Window + clear screen

## What this step does
The viewer opens a `<canvas>` and clears it to a flat colour every frame. This is the whole render
loop with **no geometry yet** — the plumbing everything else sits on.

The five wgpu objects, in order: **Instance** (driver) → **Surface** (the canvas) → **Adapter**
(a physical GPU) → **Device** (makes resources) + **Queue** (submits work). On the web, creating the
Device is *async*, so we make the window, kick off async init, and hand the finished `State` back as
a winit *user event*.

## Code
```rust
// session_viewer/src/lib.rs — State::new (the 5 objects)
let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL, ..Default::default()
});
let surface  = instance.create_surface(window.clone())?;
let adapter  = instance.request_adapter(&wgpu::RequestAdapterOptions {
    compatible_surface: Some(&surface), ..Default::default() }).await?;
let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
    required_limits: wgpu::Limits::downlevel_webgl2_defaults(), ..Default::default() }).await?;
surface.configure(&device, &config);
```

```rust
// State::render — one render pass that only clears
let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &view, resolve_target: None, depth_slice: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }),
            store: wgpu::StoreOp::Store,
        },
    })],
    ..   // depth_stencil / timestamps / occlusion / multiview_mask = None
});
```

## My notes
> Write here in your own words as you read the code — what each call does, what confused you, etc.

## Compare to the archive
In `session_viewer_archive/` the same five objects live in `State::new` (lib.rs) but the clear is
driven by `self.gpu.clear_color`, and the render pass also attaches a **depth buffer** and draws the
grid + geometry. We add those in the next steps.

## Verify
`#/viewer` → **Live** sub-tab shows a light-grey canvas.
