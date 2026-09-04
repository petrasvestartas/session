# 01 A canvas that clears

- You end with a crate `session_viewer/` beside `session_rust/` that `trunk serve` compiles to wasm, and a browser tab whose canvas is cleared to light grey: one frame on load, one after every resize, nothing in between.
- Thirteen files for one cleared frame, because the shape is the lesson: `app/` on the left knows the browser (and later the geometry), `engine/` on the right knows wgpu and never a kernel type, and `state.rs` is the one seam between them.
- Rendering is on demand from the first frame: `State::render` draws once and stops, and the shell asks for another frame only when a handler set `needs_frame`. Every lane added later inherits the rule, so a still tab never burns GPU time.
- The GPU comes up asynchronously: WebGPU's adapter and device are promises the event loop cannot await, so `app/loader.rs` builds `State` as a browser future and posts it back as `Msg::Ready`.

<svg viewBox="0 0 720 362" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="The crate shell on top; below the line the two halves: app/ (lib.rs, state.rs, app/loader.rs, app/mod.rs) on the left, an empty Upload contract in the middle, engine/ (engine/mod.rs, gpu/mod.rs, gpu/device.rs, gpu/buffers.rs, gpu/present.rs) on the right; two arrows from State cross the line: Gpu::new and Gpu::present" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="ah" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#333"/></marker></defs>
  <g fill="none" stroke="#333">
    <rect x="14" y="12" width="160" height="40"/><rect x="188" y="12" width="160" height="40"/>
    <rect x="362" y="12" width="160" height="40"/><rect x="536" y="12" width="170" height="40"/>
  </g>
  <g fill="#222">
    <text x="22" y="28">Cargo.toml</text><text x="196" y="28">.cargo/config.toml</text>
    <text x="370" y="28">Trunk.toml</text><text x="544" y="28">index.html</text>
  </g>
  <g fill="#666" font-size="9">
    <text x="22" y="44">wgpu 29 · winit 0.30 · wasm-bindgen</text><text x="196" y="44">target = wasm32-unknown-unknown</text>
    <text x="370" y="44">release = true · port 8770</text><text x="544" y="44">&lt;canvas id="canvas"&gt; full viewport</text>
  </g>
  <line x1="14" y1="70" x2="706" y2="70" stroke="#999"/>
  <g fill="#222">
    <text x="14" y="88">app/  the browser side: winit, the loader, State</text>
    <text x="410" y="88">engine/  wgpu only, never a kernel type</text>
  </g>
  <g fill="none" stroke="#333">
    <rect x="14" y="96" width="304" height="86"/>
    <rect x="14" y="194" width="304" height="60"/>
    <rect x="14" y="266" width="304" height="60"/>
    <rect x="410" y="122" width="296" height="60"/>
    <rect x="410" y="194" width="296" height="60"/>
    <rect x="410" y="266" width="296" height="60"/>
    <rect x="326" y="122" width="72" height="40" stroke-dasharray="3 2"/>
  </g>
  <g fill="#222">
    <text x="22" y="112">lib.rs — App, the winit handler</text>
    <text x="22" y="210">state.rs — State { window, gpu, needs_frame }</text>
    <text x="22" y="282">app/loader.rs — boot(window, proxy)</text>
    <text x="418" y="138">gpu/mod.rs — Gpu { surface, ctx, config }</text>
    <text x="418" y="210">gpu/device.rs — open(window, size)</text>
    <text x="418" y="282">gpu/present.rs — present(clear)</text>
    <text x="362" y="138" text-anchor="middle">Upload</text>
  </g>
  <g fill="#666" font-size="10">
    <text x="22" y="128">resumed: bind #canvas, spawn loader::boot</text>
    <text x="22" y="142">user_event: Msg::Ready(state) -&gt; adopt</text>
    <text x="22" y="156">RedrawRequested -&gt; State::render</text>
    <text x="22" y="170">Resized -&gt; needs_frame -&gt; request_redraw</text>
    <text x="22" y="226">render: present ONE frame, then stop</text>
    <text x="22" y="240">a dropped frame sets needs_frame again</text>
    <text x="22" y="298">State::new(window).await</text>
    <text x="22" y="312">proxy.send_event(Msg::Ready(state))</text>
    <text x="418" y="154">new(window) -&gt; build(Some(window), size)</text>
    <text x="418" y="168">resize: reconfigure the surface</text>
    <text x="418" y="226">instance -&gt; surface -&gt; adapter -&gt; device</text>
    <text x="418" y="240">-&gt; DeviceSetup { surface, device, queue, config }</text>
    <text x="418" y="298">acquire -&gt; clear -&gt; submit -&gt; present</text>
    <text x="418" y="312">None = the surface was reconfigured, ask again</text>
    <text x="362" y="153" text-anchor="middle" font-size="9">not yet</text>
  </g>
  <g fill="#666" font-size="10">
    <text x="14" y="340">app/mod.rs — pub mod loader (wasm only)</text>
    <text x="410" y="112">engine/mod.rs — pub mod gpu</text>
    <text x="410" y="340">gpu/buffers.rs — GpuCtx { device, queue }</text>
  </g>
  <g stroke="#333" marker-end="url(#ah)">
    <line x1="318" y1="212" x2="410" y2="152"/>
    <line x1="318" y1="238" x2="410" y2="290"/>
  </g>
  <g fill="#333" font-size="9" text-anchor="middle">
    <text x="364" y="172">Gpu::new(window)</text>
    <text x="364" y="278">Gpu::present(CLEAR)</text>
  </g>
  <text x="14" y="358" fill="#666" font-size="9">every box is created in this lesson · the two arrows are the only calls that cross the line today</text>
</svg>

## Step 1 - Declare the crate

- `crate-type = ["cdylib"]` because the browser loads a wasm library, not a binary; the list holds only what a cleared canvas needs, and `session_rust` is linked in the next lesson, with the first code that needs a kernel type.
- Make the directory beside `session_rust/` now: `Trunk.toml` already watches `../session_rust/src`, and the kernel dependency will say `path = "../session_rust"`.

_Paste it._
**Create `Cargo.toml`**

```toml
[package]
name = "session_viewer"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
anyhow = "1.0"
winit = { version = "0.30", features = ["android-native-activity"] }
wgpu = { version = "29.0"}
log = "0.4"
console_error_panic_hook = "0.1.6"
console_log = "1.0"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Document", 
    "Window", 
    "Element", 
    "HtmlCanvasElement", 
    "EventTarget", 
    "Event", 
    "CanvasRenderingContext2d", 
    "ImageData", 
    ] }
getrandom = { version = "0.2", features = ["js"] }

js-sys = "0.3"

[profile.dev.package."*"]
opt-level = 3

# parse speed in debug builds
[profile.release]
strip = true

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

## Step 2 - Make the browser the default target

- Every plain `cargo check` and `cargo build` targets wasm32, so no command needs `--target` and nothing needs a cfg gate just to type-check; `cargo xtest` is the native door for the tests and examples of later lessons.
- The link argument raises the wasm memory ceiling, which a big point cloud will need long before the code that loads one exists.

_Type it._
**Create `.cargo/config.toml`**

```toml
# Single target: the browser. Every `cargo` command (check/build/clippy) defaults to wasm32,
# so the code needs no `#[cfg(target_arch = "wasm32")]` gates. Trunk also builds this target.
[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
rustflags = ["-C", "link-arg=--max-memory=4294967296"]

# Tests and examples are NATIVE: wasm32 has no test runner and no std::fs.
[alias]
xtest = "test --target x86_64-unknown-linux-gnu"
```

## Step 3 - Configure trunk

- Release by default: a debug wasm measures the lack of optimisation, and every number written in this series comes from the release build (the file carries the measured gap).
- `no-store` on the dev server, because trunk does not hash `index.html` and a cached copy can name the previous bundle; the watch list is what makes a kernel edit rebuild the page.

_Paste it._
**Create `Trunk.toml`**

```toml
[build]
# RELEASE BY DEFAULT. `trunk serve` builds DEBUG unless told otherwise, and for this viewer that
# is not a small difference - it is the difference between measuring the code and measuring the
# lack of optimisation. Same machine, same scene (bunny + lion + two sheets), debug -> release:
#   bunny        parse 831 -> 100 ms   walk 900 -> 90 ms
#   lion         parse 539 ->  30 ms   walk 235 -> 16 ms
#   Querschnitt  parse 2491 -> 256 ms  walk 895 -> 116 ms
#   Treppenhaus  parse 3550 -> 373 ms  walk 1268 -> 188 ms
#   total parse + walk 10.7 s -> 1.07 s, and the .wasm 12.3 MB -> 7.7 MB
# Nothing in the source changed between those two columns. Every load number written in a lesson
# has to come from this build, or it is a measurement of the wrong thing.
#
# It costs almost NOTHING to watch in release, which is the part that is easy to get wrong. Live
# reload works the same either way, and the edit -> reloaded-page cycle, measured twice each:
#   debug   4.99 / 4.70 s     release   5.31 / 5.30 s
# Half a second. `cargo build` alone IS twice as slow in release (1.39 s -> 2.85 s), but
# wasm-bindgen dominates the cycle and it runs LONGER on the fatter debug .wasm (12.3 vs 7.7 MB),
# which cancels most of it.
# Comment this out only when you need a real panic backtrace - [profile.release] sets strip = true.
release = true
no_sri = true
# Relative asset URLs so the built dist/ works when iframed under /session/viewer/.
public_url = "./"

[watch]
# assets included: the local scene manifest is meant to be edited and reloaded WITHOUT a
# rebuild, which only works if trunk re-copies it into dist when it changes. It is the only
# manifest served from here - every other scene is read from the R2 bucket.
# ../session_rust/src IS load-bearing: this list REPLACES trunk's default (the project dir), and
# the viewer is `session_rust = { path = "../session_rust" }`. Without it a kernel edit rebuilt
# NOTHING - the page kept serving the last wasm, so kernel work looked like it had no effect while
# regenerated .pb assets did change. Verified: a real edit here rebuilds in ~38 s. (A bare `touch`
# does not - the watcher filters metadata-only events; change bytes when you test this.)
watch = ["src", "Cargo.toml", "index.html", "assets/view_local.toml", "../session_rust/src"]
enable_cooldown = true

[serve]
# index.html is the one file trunk does NOT hash, and it names the hashed bundle. Trunk sends no
# Cache-Control, so Chrome falls back to HEURISTIC freshness (~10% of the age of the cached copy's
# Last-Modified) and can serve a cached index.html that still points at the PREVIOUS bundle - the
# page then runs old code while freshly re-copied .pb assets show the new model. no-store on the
# dev server removes the question; the 7.7 MB re-fetch is local and costs milliseconds.
headers = { "Cache-Control" = "no-store" }
addresses = ["127.0.0.1"]
# 8770 so `trunk serve` (viewer) and `npm run dev` (docs, 8769) run side by side; the docs
# Viewer/Learn iframes point here in dev for live reload, and at the static build in prod.
port = 8770
```

## Step 4 - Put a canvas on the page

- One full-viewport `<canvas id="canvas">` that `lib.rs` finds by id; the `outline: none` rules exist because winit gives the canvas a tabindex and the browser then paints a focus ring over the model.
- The script replaces the page with a message when `navigator.gpu` is missing: WebGPU only, never a WebGL fallback.

_Paste it._
**Create `index.html`**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Session Viewer</title>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    html, body { height: 100%; }
    body { background: #000; overflow: hidden; }
    /* No focus ring. winit gives the canvas a tabindex so it can receive key
       events, and the browser then rings the focused element - a black
       rectangle around the whole viewport, drawn over the model. Both spellings
       are needed: engines differ on which one they paint. */
    canvas { display: block; width: 100vw; height: 100vh; }
    canvas:focus, canvas:focus-visible { outline: none; }
    :focus, :focus-visible { outline: none; }
    #no-webgpu { position:fixed; inset:0; background:#111; color:#eee;
                 font:1rem system-ui; text-align:center; padding-top:40vh; }
  </style>
</head>
<body>
  <link data-trunk rel="rust" data-target-name="session_viewer" data-wasm-opt="0"/>
  <canvas id="canvas"></canvas>
  <!-- Message telling that Webgpu is not available. -->
  <script>
    if (!navigator.gpu) {
      document.body.insertAdjacentHTML("beforeend",
        '<div id="no-webgpu">WebGPU required — use a recent Chrome, Edge, Firefox, or Safari 18+.</div>');
    }
  </script>
</body>
</html>
```

## Step 5 - Name the GPU floor

- Every resource of every later lane is created with the device and written through the queue, so the pair gets one name now and travels as one argument.

_Type it._
**Create `src/engine/gpu/buffers.rs`**

```rust
//! The GPU floor every lane stands on: `GpuCtx` (device + queue).
//! No lane, no shader and no per-frame state lives here.

/// The device/queue pair every resource is made with and every write goes through.
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
```

## Step 6 - Negotiate the device

- Instance, surface, adapter, device, surface format, in that order, ending in one `DeviceSetup` that `open` hands back and forgets; the window is an `Option` so a headless caller (the native harness of a later lesson) gets a device with no surface.
- `LowPower` picks the GPU the compositor runs on: on a hybrid laptop the discrete GPU renders fine, but its frames never reach the screen and the canvas stays black. The storage limits are raised to the hardware's before any buffer exists.

_Paste it._
**Create `src/engine/gpu/device.rs`**

```rust
//! Device negotiation: instance -> surface -> adapter -> device + queue -> surface format.
//! Produces one `DeviceSetup` and owns nothing afterwards. Headless callers pass no window
//! and get no surface.

use std::sync::Arc;
use winit::window::Window;

/// What `open` negotiated: the surface (None when headless), the device/queue pair, and the
/// surface configuration it was configured with.
pub struct DeviceSetup {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

/// Set up the wgpu objects in order. `size` is the canvas in pixels; a zero side is clamped
/// to 1 so the surface can be configured.
pub async fn open(window: Option<Arc<Window>>, size: (u32, u32)) -> anyhow::Result<DeviceSetup> {
    // WebGPU only in the browser, never WebGL; Vulkan / Metal / DX12 for the native harness.
    let backends = if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY };
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    let surface = match &window {
        Some(w) => Some(instance.create_surface(w.clone())?),
        None => None,
    };

    // LowPower = the GPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames cannot be shared to the compositor and the canvas stays black.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await?;
    let info = adapter.get_info();
    log::info!("adapter: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
    if info.device_type == wgpu::DeviceType::Cpu {
        log::warn!("software adapter - rendering on the CPU will be slow");
    }

    // The default 128 MB storage-binding limit is smaller than one big cloud table.
    let hw = adapter.limits();
    let limits = wgpu::Limits {
        max_storage_buffer_binding_size: hw.max_storage_buffer_binding_size,
        max_buffer_size: hw.max_buffer_size,
        ..wgpu::Limits::default()
    };

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            memory_hints: Default::default(),
            ..Default::default()
        })
        .await?;
    device.on_uncaptured_error(Arc::new(report_gpu_error));

    let (format, present_mode, alpha_mode) = match &surface {
        Some(s) => {
            let caps = s.get_capabilities(&adapter);
            let f = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
            (f, caps.present_modes[0], caps.alpha_modes[0])
        }
        None => (wgpu::TextureFormat::Rgba8UnormSrgb, wgpu::PresentMode::Fifo, wgpu::CompositeAlphaMode::Auto),
    };
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.0.max(1),
        height: size.1.max(1),
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    if let Some(s) = &surface {
        s.configure(&device, &config);
    }

    Ok(DeviceSetup { surface, device, queue, config })
}

/// wgpu validation errors go to the log instead of a panic.
fn report_gpu_error(e: wgpu::Error) {
    log::error!("wgpu: {e}");
}
```

## Step 7 - Build Gpu

- `Gpu` owns what a frame needs: the surface (an `Option`, for the same headless reason), the `GpuCtx`, and the surface configuration that `resize` rewrites. `new` takes the window and `build` is the shared body, so a headless constructor plugs in beside `new` later without touching it.

_Type it._
**Create `src/engine/gpu/mod.rs`**

```rust
//! `Gpu` - the lowest layer of the viewer: the floor (surface, device), one file each.
//! This file builds the struct; presenting is `present.rs`.

pub mod buffers;
pub mod device;
pub mod present;

use buffers::GpuCtx;
use device::DeviceSetup;

/// Everything on the GPU side of the viewer: the floor.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>,
    pub ctx: GpuCtx,
    pub config: wgpu::SurfaceConfiguration,
}

impl Gpu {
    /// The stack over a canvas window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Negotiate the device, start empty.
    async fn build(window: Option<std::sync::Arc<winit::window::Window>>, size: (u32, u32)) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, size).await?;
        let ctx = GpuCtx { device, queue };

        log::info!("viewer init OK - surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
        })
    }

    /// Reconfigure the surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }
    }
}
```

## Step 8 - Present a cleared frame

- A frame is acquire, clear, submit, present: a render pass with no draw call is a clear. When the surface has no texture to give, `present` reconfigures it and returns `None`, so the caller can ask again instead of losing the frame silently.

_Type it._
**Create `src/engine/gpu/present.rs`**

```rust
//! How a frame leaves `Gpu`: presented to the swapchain (`present`), which clears and submits.

use super::Gpu;

impl Gpu {
    /// Draw one frame to the swapchain: the clear. Returns `None` when the surface had no
    /// texture to give (it was reconfigured; the caller asks for another frame).
    pub fn present(&mut self, clear: wgpu::Color) -> Option<()> {
        let surface = self.surface.as_ref()?;
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        Some(())
    }
}
```

## Step 9 - Open the engine

- `engine/` is the half that talks to wgpu and never to the kernel; today it holds one module.

_Type it._
**Create `src/engine/mod.rs`**

```rust
//! engine — the reusable, scene-agnostic viewer core (ARCHITECTURE.md §9).
//! Grows into: gpu/ · pipelines/ · camera · pick · text · gumball/. App-specific code lives
//! in `app/` (added when the first scene/CLI/tool chapter needs it), never here.

pub mod gpu;
```

## Step 10 - Put State on the seam

- `State` is the only type both halves see: the window, the `Gpu`, and `needs_frame`. `render` clears the flag and draws once; a resize sets it, and a dropped frame (`present` returned `None` while a surface exists) sets it again so the shell asks once more.

_Type it._
**Create `src/state.rs`**

```rust
//! `State` - the viewer itself: the `gpu` layer and ONE bit of shell
//! state, `needs_frame`. The viewer renders on demand, and this is the demand. Higher
//! layers drive lower ones, never the other way round.

use std::sync::Arc;
use winit::window::Window;
use crate::engine::gpu::Gpu;

/// Background colour of every frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

pub struct State {
    pub window: Arc<Window>,
    pub gpu: Gpu,
    /// Something changed since the last frame; the shell asks for a redraw when it sees this.
    pub needs_frame: bool,
}

impl State {
    /// Wire the stack around the canvas window.
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = Gpu::new(window.clone()).await?;
        Ok(Self { window, gpu, needs_frame: true })
    }

    /// Forward a canvas resize to the GPU layer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        self.needs_frame = true;
    }

    /// Draw ONE frame and never ask for the next: a still scene costs nothing after this.
    /// The shell asks again when `needs_frame` is set - by a resize.
    pub fn render(&mut self) {
        self.needs_frame = false;
        let drawn = self.gpu.present(CLEAR);
        let dropped = drawn.is_none() && self.gpu.surface.is_some();
        self.needs_frame |= dropped;
    }
}
```

## Step 11 - Bring the canvas up asynchronously

- Adapter and device requests are promises, and winit's loop cannot await, so `boot` runs as a browser future and posts the finished `State` back through the event-loop proxy as `Msg::Ready`. The loader names no GPU type.
- The module is wasm-only because the proxy and the window it receives come from winit's web platform, which a native build does not have.

_Type it._
**Create `src/app/loader.rs`**

```rust
//! The async loader (wasm): bring the canvas up EMPTY. Touches no GPU.

use std::sync::Arc;
use winit::event_loop::EventLoopProxy;
use winit::window::Window;
use crate::{Msg, State};

/// Start-up: the empty canvas.
pub async fn boot(window: Arc<Window>, proxy: EventLoopProxy<Msg>) {
    let state = State::new(window).await.expect("State init failed");
    let _ = proxy.send_event(Msg::Ready(Box::new(state)));
}
```

_Type it._
**Create `src/app/mod.rs`**

```rust
//! The app layer: how the viewer is brought up (the loader). Above the engine, below the
//! shell in lib.rs. Never names a wgpu type.

#[cfg(target_arch = "wasm32")]
pub mod loader;
```

## Step 12 - Wire the shell

- `App` is the winit handler and this file is the whole control flow of the viewer: `resumed` binds `#canvas` and spawns `boot`; `user_event` adopts the `State` from `Msg::Ready`, sizes it to the canvas and asks for the first frame; `window_event` renders on `RedrawRequested` and records a resize as a change.
- `request_if_needed` is the one place a frame is requested, which is what makes "why does it never redraw, why does it never stop" a one-function question later.
- The `wasm32` gates keep winit's web platform out of a native build of the same tree, which the harness of a later lesson needs.

_Type it._
**Create `src/lib.rs`**

```rust
//! session_viewer - a browser-only (WebGPU/wgpu + winit) CAD viewer over `session_rust`.
//! This file is the shell only: the canvas window, the event loop and the `Msg` handlers,
//! each delegating to `State`. Loading is `app/loader.rs`.

mod engine;
mod state;
pub mod app;

pub use state::State;

/// Async loader -> event-loop messages. `Ready` carries the `State` built around an empty
/// scene.
pub enum Msg {
    Ready(Box<State>),
}

#[cfg(target_arch = "wasm32")]
use {
    crate::app::loader,
    std::sync::Arc,
    wasm_bindgen::prelude::*,
    wasm_bindgen::JsCast,
    winit::application::ApplicationHandler,
    winit::event::WindowEvent,
    winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    winit::window::{Window, WindowId},
};

/// The winit application handler: owns `State` once async init completes.
#[cfg(target_arch = "wasm32")]
pub struct App {
    state: Option<State>,
    proxy: Option<EventLoopProxy<Msg>>,
}

#[cfg(target_arch = "wasm32")]
impl App {
    /// Create the event loop and spawn the app on the browser's main loop.
    pub fn run() -> anyhow::Result<()> {
        console_log::init_with_level(log::Level::Info).ok();
        let event_loop = EventLoop::<Msg>::with_user_event().build()?;
        let app = App { proxy: Some(event_loop.create_proxy()), state: None };
        event_loop.spawn_app(app);
        Ok(())
    }

    /// `Ready`: adopt the State, size it to the canvas, draw.
    fn adopt(&mut self, mut state: State) {
        if let Some((w, h)) = desired_canvas_size() {
            state.resize(w, h);
        }
        state.window.request_redraw();
        self.state = Some(state);
    }

    /// The one place a frame is asked for: whenever a handler left `needs_frame` set.
    fn request_if_needed(&self) {
        if let Some(state) = &self.state && state.needs_frame {
            state.window.request_redraw();
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler<Msg> for App {
    /// Bind to the `#canvas` element and start the loader; `State` comes back as `Msg::Ready`.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let canvas = web_sys::window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let attrs = Window::default_attributes().with_canvas(Some(canvas));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        if let Some(proxy) = self.proxy.take() {
            wasm_bindgen_futures::spawn_local(loader::boot(window, proxy));
        }
    }

    /// The one message, `Ready`: the loader hands over the State.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, msg: Msg) {
        match msg {
            Msg::Ready(state) => self.adopt(*state),
        }
    }

    /// Redraw and resize here. A frame is requested only when something changed.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        let changed = match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                false
            }
            WindowEvent::RedrawRequested => {
                if let Some((w, h)) = desired_canvas_size() && (w, h) != (state.gpu.config.width, state.gpu.config.height) {
                    state.resize(w, h);
                }
                state.render();
                false
            }
            WindowEvent::Resized(_) => true,
            _ => false,
        };
        if changed {
            state.needs_frame = true;
        }
        self.request_if_needed();
    }
}

/// The canvas's pixel size (CSS size x device-pixel-ratio), or `None` if zero or unavailable.
#[cfg(target_arch = "wasm32")]
fn desired_canvas_size() -> Option<(u32, u32)> {
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?.get_element_by_id("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width() as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}

/// wasm entry point: install the panic hook and run the app.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    App::run().map_err(|e| wasm_bindgen::JsValue::from_str(&e.to_string()))
}
```

## Run

```bash
trunk serve
```

- Open http://localhost:8770 in a WebGPU browser (Chrome, Edge, Firefox with `dom.webgpu.enabled`, Safari 18+): the tab is one light-grey rectangle, the canvas cleared to `CLEAR`, and the console shows `adapter: ...` then `viewer init OK - surface 1x1, ...` (the canvas is measured after that line, in `adopt`).
- Resize the window and a frame is drawn at the new size; leave it still and nothing is drawn at all.

## Why

- Two halves from the first file: `app/` will talk to `session_rust` and `engine/` to wgpu, and neither names the other's types. The line is empty today (nothing is uploaded yet), but every later lane is added on one side of it and can be deleted the same way.
- One seam, `State`: the shell in `lib.rs` calls `State`, `State` calls `Gpu`, never the other way round. Higher layers drive lower ones, so wgpu never learns what a winit event is.
- Render on demand, not a render loop: a CAD viewer is still most of the time, and an animation-frame loop would clear the canvas at the display rate for nothing. `needs_frame` is the demand, and the dropped-frame `Option` is how a lost frame re-arms it.
- The async loader is the only place a promise is awaited; the event loop stays synchronous and receives finished objects as messages. Every later load (a file, a streamed chunk, a live reload) arrives through the same `Msg`.
- `Option<Surface>` and `DeviceSetup` are shaped for a headless caller before one exists, so the native harness can reuse `open` and `build` instead of forking them.
- wasm32 as cargo's default target and release as trunk's default build remove two ways of measuring the wrong thing: a native check that passes on code the browser cannot run, and a debug wasm whose timings are not the viewer's.
