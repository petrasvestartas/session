# 03 Window

Create a simple window with a grey background.

## Get & run this chapter

📁 **[Source folder — `03_window/`](./03_window/)** — a standalone, runnable skeleton (the starting point for the next chapter). See its [`README.md`](./03_window/README.md).

```bash
rustup target add wasm32-unknown-unknown   # once
cargo install trunk                        # once

cd 03_window                               # the snapshot folder
trunk serve                                # http://localhost:8770 → grey window
```


## The file tree

```bash
session_viewer/
├── index.html          # the web page — contains <canvas id="canvas">
├── Cargo.toml          # crate name + dependencies
├── Trunk.toml          # how `trunk` compiles Rust → wasm and serves it (port 8770)
├── .cargo/config.toml  # default build target = wasm32  → the browser
└── src/
    ├── lib.rs          # ENTRY POINT: the browser shell + the winit event loop
    ├── state.rs        # State — the thing the event loop drives each frame
    └── engine/
        ├── mod.rs      # the engine's "index": lists what's inside the engine folder
        └── gpu.rs      # Gpu — our handle to the graphics card (device/queue/surface)
```

`the browser` -> `lib.rs` -> `state.rs` -> `engine/gpu.rs`

Minimal code to open a browser window and clear it grey.

## gpu.rs

### Struct declaration

`Gpu` holds a surface, device, queue, and config:

```rust
pub struct Gpu {
    pub surface: wgpu::Surface<'static>,     // Screen to draw pixels on.
    pub device: wgpu::Device,                // Handle to the GPU, used to create resources (textures, buffers, pipelines).
    pub queue: wgpu::Queue,                  // Used to submit work to the GPU (draw calls, resource updates).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
}
```

`Gpu` has three methods: `new`, `resize`, `clear`.

### `new(...)`

Boots wgpu. First, an instance with default parameters:
```rust
pub struct Gpu {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
}
```

Create the surface from the window, then request an adapter:
```rust
let surface = instance.create_surface(window.clone())?;

let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })
    .await?;
```

Open a connection to the adapter — `device` creates GPU resources (buffers, textures, shaders, pipelines); `queue` submits work and uploads data:
```rust
let (device, queue) = adapter
    .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
        memory_hints: Default::default(),
        ..Default::default()
    })
    .await?;
```

Pixel format for a broadly-compatible viewer:
```rust
let size = window.inner_size();
let caps = surface.get_capabilities(&adapter);
let format = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
let config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format,
    width: size.width.max(1),
    height: size.height.max(1),
    present_mode: caps.present_modes[0],
    alpha_mode: caps.alpha_modes[0],
    view_formats: vec![],
    desired_maximum_frame_latency: 2,
};
surface.configure(&device, &config);
```

Log the result and construct the struct:
```rust
log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
Ok(Self { surface, device, queue, config })
```

### `resize(...)`

Updates width/height and reconfigures the surface:

```rust
pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }
}
```

### `clear(...)`

Clears the window to a color:

```rust
pub fn clear(&mut self, color: wgpu::Color) -> anyhow::Result<()> {

    // wgpu 29: get_current_texture() returns an enum, not a Result.
    let output = match self.surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
        _ => { self.surface.configure(&self.device, &self.config); return Ok(()); }
    };

    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("clear encoder"),
    });

    {
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
    
    self.queue.submit([encoder.finish()]);
    output.present();
    Ok(())
}
```




## state.rs

Connects the event loop to the GPU; also holds a reference to the window.

### Struct declaration

Holds the window and `Gpu`:

```rust
pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
}
```

### `new(...)`

Receives the window already created in `lib.rs`:

```rust
pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
    let gpu = Gpu::new(window.clone()).await?;
    Ok(Self { window, gpu })
}
```

### `resize(...)`

Forwards to `Gpu::resize`:

```rust

pub fn resize(&mut self, width: u32, height: u32) {
    self.gpu.resize(width, height);
}
```

### `render(...)`

Queues the next frame. Any animation/state logic goes before the redraw call:

```rust
pub fn render(&mut self) -> anyhow::Result<()> {
    self.window.request_redraw();
    self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 })
}
```

## lib.rs

### Imports

- `mod state` looks for the single file `src/state.rs`.
- `mod engine` looks for a folder with `src/engine/mod.rs` — a container for sub-pieces: `gpu/`, `pipelines/`, `camera/`, `pick/`, `text/`, `gumball/`.
- `State` sits between the GPU and a public surface.
- `Arc` gives the window shared ownership.
- `wasm_bindgen` bridges Rust and JavaScript.
- `ApplicationHandler` is the callback trait for `resumed`, `user_event`, `window_event` — no manual loop needed.
- `WindowEvent` enumerates what happens to a window: resize, close, redraw.
- `event_loop` — the handle winit passes into callbacks; use it to create windows or exit. Created once.
- `window` — the window and its identifier.

```rust
mod engine;
mod state;

pub use state::State;

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
```

### App

`App` is the object `winit` drives. `state` is an `Option` because init is async — every handler checks whether it's ready yet. `EventLoopProxy` is a thread-safe sender that pushes the finished `State` back into the event loop as a custom user event from the async init task.

`run()` imports the wasm-only `spawn_app` extension, wires Rust's logger into the browser, builds an `EventLoop<State>`, constructs the proxy, then spawns the wasm app.


```rust
pub struct App {
    state: Option<State>,
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
}


impl App {
    pub fn run() -> anyhow::Result<()> {
        use winit::platform::web::EventLoopExtWebSys;
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<State>::with_user_event().build()?;
        let app = App { proxy: Some(event_loop.create_proxy()), state: None };
        event_loop.spawn_app(app);
        Ok(())
    }
}
```

### ApplicationHandler

Three callbacks:
- `resumed` — create the window, start async init
- `user_event` — receive the finished `State`
- `window_event` — the live loop

```rust
impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {...}
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {...}
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {...}
}
```

`resumed` reaches into JavaScript for the canvas, then spawns the async init:


```rust
fn resumed(&mut self, event_loop: &ActiveEventLoop) {

    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowAttributesExtWebSys;

    if self.state.is_some() { return; }

    let canvas = web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    let attrs = Window::default_attributes().with_canvas(Some(canvas));
    let window = Arc::new(event_loop.create_window(attrs).unwrap());

    if let Some(proxy) = self.proxy.take() {
        wasm_bindgen_futures::spawn_local(async move {
            let state = State::new(window).await.expect("State init failed");
            let _ = proxy.send_event(state);
        });
    }
}
```

This fires when `proxy.send_event(state)` is delivered:

```rust
fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {
    let size = state.window.inner_size();
    state.resize(size.width, size.height);
    state.window.request_redraw();
    self.state = Some(state);
}
```

Handles close, resize, and redraw:

```rust
fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    let state = match &mut self.state { Some(s) => s, None => return };
    match event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::Resized(size) => state.resize(size.width, size.height),
        WindowEvent::RedrawRequested => {
            if let Err(e) = state.render() { log::error!("render: {e}"); }
        }
        _ => {}
    }
}
```

### `run_web` = `main()`

The web entry point, equivalent to a native `main`. `App::run()` launches everything:

```rust
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
```

## Next

Draw a triangle.