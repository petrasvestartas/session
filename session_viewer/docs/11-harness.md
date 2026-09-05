# 11 The native harness, the perf line and memory

- At the end `cargo run --example selftest` renders the frame the page renders, into a `.ppm`, and prints its ink count; from here on every number in these lessons comes through that path, because a browser screenshot cannot be diffed and a browser timer measures the compositor as much as the frame.
- `bench_frame` prints a median frame time for a still and a moving camera with the GPU drained after every frame, so a shader change is measured on the GPU it runs on, not on vsync.
- `?perf=1` puts one line in the page's corner (frame number, gap, encode time, heap) and `?spin=1` orbits the camera every frame; both are `View` knobs, read once like every other knob.
- Every load log line ends with `heap N MB`, a high-water mark: `WebAssembly.Memory` never shrinks, so the number after the last file is what the tab holds until reload.
- The engine gains nothing browser-specific: `Gpu::new_headless` is the same `build` with no window, `render_offscreen` runs the same `encode_frame` into a texture and reads it back, and the harness (`src/selftest.rs`) is `#[cfg(not(target_arch = "wasm32"))]`.
- The crate becomes an `rlib` so `examples/` can link it; 22 examples move in - probe scenes that turn a bug into a pixel count, inspectors that say where a load's milliseconds and megabytes go, and cloud tools that write the LOD octree offline.

<svg viewBox="0 0 720 312" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Lesson 11 on the two-halves map: app/ state.rs and lib.rs on the left, the per-frame contract in the middle, engine/ performance.rs, gpu/mod.rs, gpu/present.rs and gpu/view.rs on the right; below the line the native-only harness src/selftest.rs and examples/ linked through Cargo.toml" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <defs><marker id="m11" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#333"/></marker></defs>
  <g fill="#222" font-size="11" font-weight="bold">
    <text x="14" y="14">app/</text><text x="360" y="14" text-anchor="middle">the contract</text><text x="486" y="14">engine/</text>
  </g>
  <g fill="none" stroke="#333">
    <rect x="14" y="22" width="220" height="112"/><rect x="486" y="22" width="220" height="112"/>
  </g>
  <rect x="250" y="22" width="220" height="112" fill="none" stroke="#888" stroke-dasharray="4 3"/>
  <g fill="#222" font-size="11">
    <text x="22" y="40">state.rs</text>
    <text x="22" y="56">  SPIN_STEP, last_frame_ms</text>
    <text x="22" y="70">  render(): spin, gap, perf_line()</text>
    <text x="22" y="84">  heap in every load log line</text>
    <text x="22" y="106">lib.rs</text>
    <text x="22" y="120">  pub mod selftest (native only)</text>
    <text x="360" y="40" text-anchor="middle">Upload - unchanged</text>
    <text x="360" y="70" text-anchor="middle">gpu/frame.rs</text>
    <text x="360" y="84" text-anchor="middle">FrameInput { view_proj, clear,</text>
    <text x="360" y="98" text-anchor="middle">  now_ms }  one clock read a frame</text>
    <text x="494" y="40">performance.rs</text>
    <text x="494" y="54">  Performance, heap_mb(), perf_line()</text>
    <text x="494" y="70">gpu/mod.rs</text>
    <text x="494" y="84">  Gpu.performance, new_headless()</text>
    <text x="494" y="100">gpu/present.rs</text>
    <text x="494" y="114">  render_offscreen(), bench_frames()</text>
    <text x="494" y="128">gpu/view.rs  perf, spin</text>
  </g>
  <line x1="234" y1="90" x2="248" y2="90" stroke="#333" marker-end="url(#m11)"/>
  <line x1="470" y1="90" x2="484" y2="90" stroke="#333" marker-end="url(#m11)"/>
  <line x1="14" y1="150" x2="706" y2="150" stroke="#333"/>
  <text x="360" y="146" fill="#555" font-size="10" text-anchor="middle">native only, below this line</text>
  <g fill="none" stroke="#2a7f2a" stroke-width="1.3">
    <rect x="14" y="164" width="300" height="72"/><rect x="330" y="164" width="376" height="72"/>
  </g>
  <g fill="#222" font-size="11">
    <text x="22" y="182">src/selftest.rs</text>
    <text x="22" y="198">  SceneFile::from_args, camera_from_env</text>
    <text x="22" y="212">  render_scene  -&gt; .ppm + ink count</text>
    <text x="22" y="226">  frame_profile -&gt; still / moving ms</text>
    <text x="338" y="182">examples/  (22 files)</text>
    <text x="338" y="198">  selftest, bench_frame - the front doors</text>
    <text x="338" y="212">  mk_* probes; bench_load, probe_mem, ...</text>
    <text x="338" y="226">  add_lod, potree_import, mk_bunny_cloud</text>
  </g>
  <line x1="330" y1="200" x2="316" y2="200" stroke="#333" marker-end="url(#m11)"/>
  <line x1="110" y1="164" x2="110" y2="136" stroke="#333" marker-end="url(#m11)"/>
  <line x1="260" y1="164" x2="540" y2="136" stroke="#333" marker-end="url(#m11)"/>
  <g fill="#555" font-size="10">
    <text x="116" y="152">Scene, Camera</text>
    <text x="392" y="158">Gpu::new_headless, render_offscreen</text>
    <text x="14" y="258">Cargo.toml  crate-type += rlib; [[example]] tables; pollster for the native target only</text>
    <text x="14" y="274">docs/_gate.sh + docs/_count_colors.py - outside the tree: the gate that runs all of this</text>
    <text x="14" y="290">green = created in this lesson; the #perf line is a DOM element perf_line() creates, index.html is untouched</text>
  </g>
</svg>

## Step 1 - Count the frames

- `Performance` keeps a smoothed frame time and, with `perf` on, logs one line a second with the draw and object counts of the frame; it lives beside `now_ms` so both targets share one clock. The module's first line says what the file now holds.

_Type it._
**Find** in `src/engine/performance.rs`:

```rust
//! Clocks: `now_ms` on both targets. Native builds read the system clock.
```

**Replace with:**

```rust
//! Clocks and counters: the frame timer that logs fps once a second, the browser heap size,
//! and `now_ms` on both targets. Native builds read the system clock.

/// Frame timing: a smoothed frame time and one log line a second when `perf` is on.
pub struct Performance {
    prev_frame: f64,
    last_log: f64,
    frame_ms: f64,
    pub frames: u64,
}

impl Performance {
    /// Start the clock now.
    pub fn new() -> Self {
        let t = now_ms();
        Self { prev_frame: t, last_log: t, frame_ms: 0.0, frames: 0 }
    }

    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32, now: f64, perf: bool) {
        let dt = now - self.prev_frame;
        self.prev_frame = now;
        self.frames += 1;
        self.frame_ms = if self.frame_ms == 0.0 { dt } else { self.frame_ms * 0.9 + dt * 0.1 };

        if perf && now - self.last_log >= 1000.0 {
            let fps = if self.frame_ms > 0.0 { 1000.0 / self.frame_ms } else { 0.0 };
            log::info!("perf: {:.1} fps | {:.2} ms | {} draws | {} objects | heap {:.0} MB", fps, self.frame_ms, draws, objects, heap_mb());
            self.last_log = now;
        }
    }
}
```

## Step 2 - Measure the heap

- The browser number is the length of the wasm `Memory` buffer, a high-water mark because it never shrinks; natively the resident set from `/proc` is the closest cheap measure, and elsewhere there is none.

_Type it._
**Find** in `src/engine/performance.rs`:

```rust
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() * 1000.0
}
```

**Add below it:**

```rust

/// The wasm heap in MB - a high-water mark, since `WebAssembly.Memory` never shrinks.
#[cfg(target_arch = "wasm32")]
pub fn heap_mb() -> f64 {
    use wasm_bindgen::JsCast;
    wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()
        .ok()
        .map(|m| m.buffer().unchecked_into::<js_sys::ArrayBuffer>().byte_length() as f64 / 1.048576e6)
        .unwrap_or(0.0)
}

/// Native: resident set size from /proc, the closest thing to the same measure.
#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub fn heap_mb() -> f64 {
    std::fs::read_to_string("/proc/self/statm")
        .ok()
        .and_then(|s| s.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok()))
        .map(|pages| pages * 4096.0 / 1.048576e6)
        .unwrap_or(0.0)
}

/// Native, non-Linux: no cheap measure.
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "linux")))]
pub fn heap_mb() -> f64 {
    0.0
}
```

- A DOM line survives a busy console and shows in a screenshot; `perf_line` creates the `#perf` element on first use, so `index.html` needs nothing.

_Paste it._
**Find** in `src/engine/performance.rs`:

```rust
pub fn heap_mb() -> f64 {
    0.0
}
```

**Add below it:**

```rust

/// Write one line into the `#perf` element in the page's top-left corner, creating it on
/// first use. A DOM line survives a busy console and shows in a screenshot.
#[cfg(target_arch = "wasm32")]
pub fn perf_line(text: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let el = match doc.get_element_by_id("perf") {
        Some(e) => e,
        None => {
            let Ok(e) = doc.create_element("pre") else { return };
            e.set_id("perf");
            let _ = e.set_attribute("style", "position:fixed;left:0;top:0;margin:0;padding:2px 6px;font:12px monospace;color:#000;background:rgba(255,255,255,.7);z-index:9;pointer-events:none");
            if let Some(b) = doc.body() {
                let _ = b.append_child(&e);
            }
            e
        }
    };
    el.set_text_content(Some(text));
}
```

## Step 3 - Give the frame its timestamp

- The re-anchor throttle and the fps counter both need "now"; one `now_ms` in `FrameInput` means one clock read per frame and one answer for both.

_Type it._
**Find** in `src/engine/gpu/frame.rs`:

```rust
/// What one frame needs from the caller: the camera and the clear colour.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
}
```

**Replace with:**

```rust
/// What one frame needs from the caller: the camera, the clear colour and the frame's ONE
/// timestamp (ms) - the re-anchor throttle and the fps counter both read it.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
    pub now_ms: f64,
}
```

## Step 4 - Add the two knobs

- `perf` and `spin` are `View` knobs like every other: `?perf=1` on the page, `VIEWER_PERF=1` natively, read once at start and never polled.

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
    pub msaa_forced: Option<u32>,
```

**Add below it:**

```rust
    /// Continuous rendering with a frame line on the page (`?perf=1` / `VIEWER_PERF`).
    pub perf: bool,
    /// Orbit a little every frame - a moving-camera benchmark (`?spin=1`).
    pub spin: bool,
```

_Type it._
**Find** in `src/engine/gpu/view.rs`:

```rust
            msaa_forced: knob("VIEWER_MSAA", "msaa").and_then(|v| v.parse().ok()),
```

**Add below it:**

```rust
            perf: knob("VIEWER_PERF", "perf").is_some(),
            spin: knob("VIEWER_SPIN", "spin").is_some(),
```

## Step 5 - Hang the counter on Gpu and open it headless

- `Gpu` owns the counter because it owns the draw counts; `new_headless` is the same `build` with no window and no surface, so the harness runs the page's pipelines, never a copy of them.

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::{Layouts, Target};
```

**Add above it:**

```rust
use crate::engine::performance::Performance;
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub pick: Picker,
```

**Add below it:**

```rust
    pub performance: Performance,
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
            pick: Picker::new(),
```

**Add below it:**

```rust
            performance: Performance::new(),
```

_Type it._
**Find** in `src/engine/gpu/mod.rs`:

```rust
        Self::build(Some(window), (size.width, size.height)).await
    }
```

**Add below it:**

```rust

    /// The same stack with no window and no surface, rendering into an offscreen texture.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, (width, height)).await
    }
```

## Step 6 - Feed the counter from present

- `encode_frame` already returns `(draws, objects)`; `present` keeps them now and hands them, with the frame's timestamp, to the counter once the swapchain has the frame. The `targets` import is native-only, for the two exits Steps 7 and 8 add.

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
//! How a frame leaves `Gpu`: presented to the swapchain (`present`), which writes the
//! uniforms, encodes through `encode_frame`, and submits.

use super::frame::{FrameCx, FrameInput};
```

**Replace with:**

```rust
//! The three ways a frame leaves `Gpu`: presented to the swapchain (`present`), read back from
//! an offscreen texture (`render_offscreen`, the native harness), or timed in a batch
//! (`bench_frames`). Each writes the uniforms, encodes through `encode_frame`, and submits.

use super::frame::{FrameCx, FrameInput};
#[cfg(not(target_arch = "wasm32"))]
use super::targets::{texture, TextureSpec};
```

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
        self.encode_frame(&mut encoder, &view, input.clear);
```

**Replace with:**

```rust
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
```

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
        output.present();
```

**Add below it:**

```rust
        self.performance.frame(draws, objects, input.now_ms, self.view.perf);
```

- The frame list's own line names the reader of that pair.

_Type it._
**Find** in `src/engine/gpu/render.rs`:

```rust
    /// Encode the whole frame into `view`. Returns (draws, objects).
```

**Replace with:**

```rust
    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
```

## Step 7 - Render offscreen and read the pixels back

- The harness frame goes into a texture with `COPY_SRC`, then into a buffer whose rows are padded to 256 bytes (wgpu's rule) and unpadded on the way out; the bytes are the ones the swapchain would have shown.

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
        Some(encode_ms)
    }
```

**Add below it:**

```rust

    /// Render one frame into an offscreen texture and read the pixels back (RGBA8, tightly
    /// packed, top row first). Native only: the harness behind every measured number.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_offscreen(&mut self, input: &FrameInput) -> Vec<u8> {
        let (w, h) = (self.config.width, self.config.height);
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC;
        let tex = texture(&self.ctx, "headless.color", &TextureSpec { size: (w, h), format: self.config.format, samples: 1, usage });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let padded = (w * 4).div_ceil(256) * 256;
        let readback = self.ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("headless.readback"),
            size: (padded * h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.write_frame_uniforms(input);
        let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
        let (draws, objects) = self.encode_frame(&mut encoder, &view, input.clear);
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &readback, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(h) } },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
        log::info!("headless frame: {draws} draws, {objects} objects, {w}x{h}");

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((w * 4 * h) as usize);
        for row in 0..h {
            let a = (row * padded) as usize;
            out.extend_from_slice(&data[a..a + (w * 4) as usize]);
        }
        drop(data);
        readback.unmap();
        out
    }
```

## Step 8 - Time a batch of frames

- `bench_frames` submits and drains the GPU after every frame, so the time is the frame's and not the queue's; the caller discards a first warm-up call, where pipeline compilation lands.

_Type it._
**Find** in `src/engine/gpu/present.rs`:

```rust
        readback.unmap();
        out
    }
```

**Add below it:**

```rust

    /// Time `frames` full frames into one offscreen target, GPU drained after each; returns
    /// seconds for the batch. The caller warms the caches with a first call it discards.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bench_frames(&mut self, input: &FrameInput, frames: u32) -> f64 {
        let (w, h) = (self.config.width, self.config.height);
        let tex = texture(&self.ctx, "bench.color", &TextureSpec { size: (w, h), format: self.config.format, samples: 1, usage: wgpu::TextureUsages::RENDER_ATTACHMENT });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        self.write_frame_uniforms(input);

        let t0 = std::time::Instant::now();
        for _ in 0..frames {
            let mut encoder = self.ctx.device.create_command_encoder(&Default::default());
            self.encode_frame(&mut encoder, &view, input.clear);
            self.ctx.queue.submit([encoder.finish()]);
            let _ = self.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
        }
        t0.elapsed().as_secs_f64()
    }
```

## Step 9 - Log the heap when files arrive

- Every load log line ends with `heap N MB`; the value after the last file is what the tab holds until reload.

_Type it._
**Find** in `src/state.rs`:

```rust
use crate::engine::performance::now_ms;
```

**Replace with:**

```rust
use crate::engine::performance::{heap_mb, now_ms};
```

_Type it._
**Find** in `src/state.rs`:

```rust
        log::info!("appended: walk {:.0} ms, upload {:.0} ms | {} docs", t1 - t0, now_ms() - t1, self.scene.docs.len());
```

**Replace with:**

```rust
        log::info!("appended: walk {:.0} ms, upload {:.0} ms | {} docs | heap {:.0} MB", t1 - t0, now_ms() - t1, self.scene.docs.len(), heap_mb());
```

_Type it._
**Find** in `src/state.rs`:

```rust
        log::info!("cloud slice: {to} points resident");
```

**Replace with:**

```rust
        log::info!("cloud slice: {to} points resident | heap {:.0} MB", heap_mb());
```

## Step 10 - Spin the camera and time the gap

- `?spin=1` orbits a small step before every frame, a moving-camera benchmark; `last_frame_ms` gives the gap between two frames, the number a smoothed fps hides.

_Type it._
**Find** in `src/state.rs`:

```rust
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };
```

**Add below it:**

```rust

/// Orbit step per frame in `?spin=1` mode.
const SPIN_STEP: f32 = 0.004;
```

_Type it._
**Find** in `src/state.rs`:

```rust
    pub needs_frame: bool,
```

**Add below it:**

```rust
    last_frame_ms: f64,
```

_Type it._
**Find** in `src/state.rs`:

```rust
        Ok(Self { window, gpu, camera: Camera::new(), scene, needs_frame: true })
```

**Replace with:**

```rust
        Ok(Self { window, gpu, camera: Camera::new(), scene, needs_frame: true, last_frame_ms: 0.0 })
```

_Type it._
**Find** in `src/state.rs`:

```rust
    /// throttled re-anchor still due or a pick in flight.
    pub fn render(&mut self) {
        self.needs_frame = false;
```

**Replace with:**

```rust
    /// throttled re-anchor still due, a pick in flight, or continuous mode.
    pub fn render(&mut self) {
        self.needs_frame = false;
        if self.gpu.view.spin {
            self.camera.orbit(SPIN_STEP, 0.0);
        }
```

_Type it._
**Find** in `src/state.rs`:

```rust
        let view_proj = self.camera.view_proj_anchored(self.aspect(), &rebase.anchor);
```

**Add below it:**

```rust
        let gap = now_ms - self.last_frame_ms;
        self.last_frame_ms = now_ms;
```

_Type it._
**Find** in `src/state.rs`:

```rust
        let drawn = self.gpu.present(&FrameInput { view_proj, clear: CLEAR });
```

**Replace with:**

```rust
        let drawn = self.gpu.present(&FrameInput { view_proj, clear: CLEAR, now_ms });
```

## Step 11 - Keep asking for frames and write the perf line

- In `perf` or `spin` mode every frame asks for the next one; the perf line is a wasm function only, since natively the harness prints its own numbers.

_Type it._
**Find** in `src/state.rs`:

```rust
            self.apply_pick(pick);
        }
```

**Add below it:**

```rust
        if let (true, Some(encode_ms)) = (self.gpu.view.perf, drawn) {
            self.perf_line(gap, encode_ms);
        }
```

_Type it._
**Find** in `src/state.rs`:

```rust
        self.needs_frame |= dropped || rebase.pending || self.gpu.pick.busy();
```

**Replace with:**

```rust
        self.needs_frame |= dropped || rebase.pending || self.gpu.pick.busy() || self.gpu.view.perf || self.gpu.view.spin;
```

_Type it._
**Find** in `src/state.rs`:

```rust
        self.needs_frame |= dropped || rebase.pending || self.gpu.pick.busy() || self.gpu.view.perf || self.gpu.view.spin;
    }
```

**Add below it:**

```rust

    /// The `?perf=1` line: frame number, gap since the previous frame, encode time, heap.
    #[cfg(target_arch = "wasm32")]
    fn perf_line(&self, gap_ms: f64, encode_ms: f64) {
        let line = format!("f{} gap {gap_ms:.0} enc {encode_ms:.1} ms heap {:.0} MB", self.gpu.performance.frames, heap_mb());
        crate::engine::performance::perf_line(&line);
    }

    /// Natively the perf line goes nowhere (the harness prints its own numbers).
    #[cfg(not(target_arch = "wasm32"))]
    fn perf_line(&self, _gap_ms: f64, _encode_ms: f64) {}
```

## Step 12 - The harness

- `src/selftest.rs` loads files the way the browser does (a manifest resolved against its `pb/` directory, or one `.pb` at its own origin), fits the camera, applies the `VIEWER_*` camera knobs, renders one frame through `render_offscreen` and writes a PPM; `frame_profile` runs `bench_frames` for a still and a moving camera. It is native-only, so the wasm build never sees it; nothing here builds natively before Step 17.

_Type it._
**Create `src/selftest.rs`**

```rust
//! Headless harness, native only: the same `encode_frame` the browser runs, aimed at an
//! offscreen texture and read back, so a shader change can be LOOKED AT and measured here.
//! Every number in the docs comes through this file.

use crate::math::eye_from_view_proj;
use std::rc::Rc;
use crate::app::manifest::Manifest;
use crate::app::scene::{FileDoc, Scene};
use crate::camera::{Camera, View};
use crate::engine::gpu::{FrameInput, Gpu, Pick};
use crate::engine::performance::now_ms;
use session_rust::{Session, Xform};

/// Background colour of a harness frame.
const CLEAR: wgpu::Color = wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 };

/// One file the harness loads: its path, placement, point size and whether the session is
/// released after the walk.
pub struct SceneFile {
    pub path: String,
    pub place: Xform,
    pub point_px: f32,
    pub display_only: bool,
}

impl SceneFile {
    /// The harness's arguments: a `.yaml`/`.json` argument is a manifest resolved the way the
    /// browser resolves it (files relative to the directory holding `pb/`), anything else one
    /// `.pb` at its own origin.
    pub fn from_args(args: &[String]) -> Vec<SceneFile> {
        let mut out = Vec::new();
        for p in args {
            if !(p.ends_with(".json") || p.ends_with(".yaml") || p.ends_with(".yml")) {
                out.push(SceneFile { path: p.clone(), place: Xform::identity(), point_px: 0.0, display_only: false });
                continue;
            }
            let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("cannot read manifest {p}: {e}"));
            let man = Manifest::parse(&bytes).unwrap_or_else(|e| panic!("cannot parse manifest {p}: {e}"));
            let root = assets_root(p, &man);
            for (i, item) in man.items.iter().enumerate() {
                let path = root.join(&item.file).to_string_lossy().into_owned();
                out.push(SceneFile { path, place: man.place(i, [3000.0, 3000.0]), point_px: item.point_size as f32, display_only: item.display_only });
            }
        }
        out
    }
}

/// The directory a manifest's `file` entries hang off: its own, or its parent.
fn assets_root(manifest: &str, man: &Manifest) -> std::path::PathBuf {
    let here = std::path::Path::new(manifest).parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
    let first = man.items.first().map(|i| i.file.clone()).unwrap_or_default();
    if here.join(&first).exists() { here } else { here.join("..") }
}

/// The harness's camera knobs: `VIEWER_ORBIT="dx,dy"`, `VIEWER_ORTHO`, `VIEWER_VIEW`, `VIEWER_ZOOM`.
fn camera_from_env(gpu: &Gpu, aspect: f64) -> Camera {
    let mut camera = Camera::new();
    camera.fit(&gpu.bounds, aspect);
    if let Ok(o) = std::env::var("VIEWER_ORBIT") {
        let mut it = o.split(',').filter_map(|v| v.trim().parse::<f32>().ok());
        camera.orbit(it.next().unwrap_or(0.0), it.next().unwrap_or(0.0));
    }
    if std::env::var("VIEWER_ORTHO").is_ok() {
        camera.toggle_projection();
    }
    if let Ok(v) = std::env::var("VIEWER_VIEW") {
        camera.set_view(match v.as_str() {
            "top" => View::Top,
            "bottom" => View::Bottom,
            "front" => View::Front,
            "right" => View::Right,
            _ => View::Iso,
        });
    }
    if let Ok(z) = std::env::var("VIEWER_ZOOM") {
        let n: i32 = z.trim().parse().unwrap_or(0);
        for _ in 0..n.abs() {
            camera.zoom(if n > 0 { 1.0 } else { -1.0 });
        }
    }
    let eye = eye_from_view_proj(&camera.view_proj(aspect));
    log::info!("camera: eye ({:.1}, {:.1}, {:.1}) mm, target ({:.1}, {:.1}, {:.1}) mm, distance {:.1} mm", eye[0], eye[1], eye[2], camera.target[0], camera.target[1], camera.target[2], camera.distance);
    camera
}

/// Load every file into `scene`, uploading per file when `VIEWER_INCREMENTAL` is set (the
/// browser's path) and once at the end otherwise. Prints per-file load costs.
fn load_files(scene: &mut Scene, gpu: &mut Gpu, files: &[SceneFile]) {
    let incremental = std::env::var("VIEWER_INCREMENTAL").is_ok();
    for f in files {
        let t0 = std::time::Instant::now();
        let bytes = std::fs::read(&f.path).unwrap_or_else(|e| panic!("cannot read {}: {e}", f.path));
        let session = Session::pb_loads(&bytes).unwrap_or_else(|e| panic!("cannot parse {}: {e:?}", f.path));
        let t1 = t0.elapsed();
        let name = f.path.rsplit('/').next().unwrap_or(&f.path).to_string();
        scene.add_file(FileDoc { name: name.clone(), session: Rc::new(session), place: f.place.clone(), point_px: f.point_px, display_only: f.display_only });
        println!("  {name}: {:.1} MB | decode {t1:?} | walk {:?}", bytes.len() as f64 / 1.048576e6, t0.elapsed() - t1);
        if incremental {
            scene.upload_to(gpu);
        }
    }
    if !incremental {
        scene.upload_to(gpu);
    }
    if std::env::var("VIEWER_REBUILD").is_ok() {
        let t = std::time::Instant::now();
        scene.rebuild(gpu);
        println!("rebuild {:?}", t.elapsed());
    }
}

/// The frame input for `camera` on `gpu`, re-anchoring first.
fn frame_input(gpu: &mut Gpu, camera: &Camera, aspect: f64) -> FrameInput {
    let now = now_ms();
    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), now);
    FrameInput { view_proj: camera.view_proj_anchored(aspect, &rebase.anchor), clear: CLEAR, now_ms: now }
}

/// Write RGBA8 rows as a binary PPM (P6).
fn write_ppm(path: &str, rgba: &[u8], w: u32, h: u32) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    write!(f, "P6\n{w} {h}\n255\n")?;
    for px in rgba.chunks_exact(4) {
        f.write_all(&px[..3])?;
    }
    f.flush()
}

/// Load, frame, render one frame, write it out; `VIEWER_FRAMES=N` times N frames first and
/// `VIEWER_PICK="x,y"` reports what the id pass finds under that pixel.
pub fn render_scene(files: &[SceneFile], w: u32, h: u32, out: &str) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    load_files(&mut scene, &mut gpu, files);
    let aspect = w as f64 / h as f64;
    let camera = camera_from_env(&gpu, aspect);

    if let Some(n) = std::env::var("VIEWER_FRAMES").ok().and_then(|v| v.parse::<usize>().ok()) {
        let mut ms: Vec<f64> = Vec::new();
        for _ in 0..n.max(1) {
            let input = frame_input(&mut gpu, &camera, aspect);
            let t = std::time::Instant::now();
            let _ = gpu.render_offscreen(&input);
            ms.push(t.elapsed().as_secs_f64() * 1000.0);
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("frames: n={} median {:.1} ms ({:.0} fps) min {:.1} max {:.1}", ms.len(), ms[ms.len() / 2], 1000.0 / ms[ms.len() / 2], ms[0], ms[ms.len() - 1]);
    }

    let input = frame_input(&mut gpu, &camera, aspect);
    let rgba = gpu.render_offscreen(&input);
    write_ppm(out, &rgba, w, h).expect("write ppm");

    if let Ok(v) = std::env::var("VIEWER_PICK") {
        let mut it = v.split(',').filter_map(|t| t.trim().parse::<u32>().ok());
        if let (Some(px), Some(py)) = (it.next(), it.next()) {
            report_pick(&mut gpu, &scene, &input, (px, py));
        }
    }

    let ink = rgba.chunks_exact(4).filter(|p| p[0] < 200 || p[1] < 200 || p[2] < 200).count();
    format!("wrote {out}  {w}x{h}  non-background pixels: {ink} ({:.1}%)\n", 100.0 * ink as f64 / (w * h) as f64)
}

/// Pick at `at` through the id pass, blocking on the GPU, and print the answer.
fn report_pick(gpu: &mut Gpu, scene: &Scene, input: &FrameInput, at: (u32, u32)) {
    gpu.pick.request(at.0, at.1);
    let _ = gpu.render_offscreen(input);
    let _ = gpu.ctx.device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None });
    let pick: Option<Pick> = gpu.pick.poll().flatten();
    match pick.and_then(|p| scene.resolve(p, gpu)) {
        Some(hit) => match hit.point {
            Some(pt) => println!("pick: ({},{}) doc='{}' row={} point={} id={} pos=({:.0}, {:.0}, {:.0})", at.0, at.1, hit.doc, hit.row, pt.local, pt.id, pt.position[0], pt.position[1], pt.position[2]),
            None => println!("pick: ({},{}) doc='{}' guid={} row={}", at.0, at.1, hit.doc, hit.guid, hit.row),
        },
        None => println!("pick: ({},{}) nothing", at.0, at.1),
    }
}

/// Where a frame's milliseconds go - uniforms, encode, GPU - for a still and a moving camera.
pub fn frame_profile(files: &[SceneFile], w: u32, h: u32) -> String {
    let mut gpu = pollster::block_on(Gpu::new_headless(w, h)).expect("headless gpu");
    let mut scene = Scene::new();
    load_files(&mut scene, &mut gpu, files);
    let aspect = w as f64 / h as f64;
    let n: usize = std::env::var("BENCH_FRAMES").ok().and_then(|v| v.parse().ok()).unwrap_or(120);
    let mut camera = camera_from_env(&gpu, aspect);
    let mut out = String::new();
    for (label, spin) in [("still", 0.0f32), ("moving", 0.35f32)] {
        let mut ms: Vec<f64> = Vec::new();
        for i in 0..n + 5 {
            camera.orbit(spin, 0.0);
            let input = frame_input(&mut gpu, &camera, aspect);
            let secs = gpu.bench_frames(&input, 1);
            if i >= 5 {
                ms.push(secs * 1000.0);
            }
        }
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let med = ms[ms.len() / 2];
        out.push_str(&format!("{label:>6}: {med:6.2} ms/frame ({:5.0} fps)\n", 1000.0 / med));
    }
    out
}
```

_Type it._
**Find** in `src/lib.rs`:

```rust
pub mod math;
```

**Add below it:**

```rust
#[cfg(not(target_arch = "wasm32"))]
pub mod selftest;
```

## Step 13 - The two front doors

- `examples/selftest.rs` installs a stderr logger before anything else: wgpu reports a broken shader through `log`, and without a logger it renders black in silence. `bench_frame.rs` is the same door for `frame_profile`.

_Type it._
**Create `examples/selftest.rs`**

```rust
// cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- <out.ppm> <scene.yaml | file.pb>...
//
// Renders one headless frame and prints the ink count; VIEWER_FRAMES=N times N frames first,
// VIEWER_PICK="x,y" reports what the id pass finds under a pixel. VIEWER_W / VIEWER_H size it.

use session_viewer::selftest::{render_scene, SceneFile};

/// wgpu reports validation errors through `log`; without a logger a broken shader renders black.
struct StderrLog;

impl log::Log for StderrLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, r: &log::Record) {
        eprintln!("[{}] {}", r.level(), r.args());
    }

    fn flush(&self) {}
}

fn main() {
    let _ = log::set_logger(&StderrLog);
    log::set_max_level(log::LevelFilter::Info);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args.first().cloned().unwrap_or_else(|| "out.ppm".into());
    let files = SceneFile::from_args(&args[1.min(args.len())..]);
    let w = std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900);
    let h = std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700);
    print!("{}", render_scene(&files, w, h, &out));
}
```

_Type it._
**Create `examples/bench_frame.rs`**

```rust
// cargo run --example bench_frame --target x86_64-unknown-linux-gnu --release -- <scene.yaml | file.pb>...
//
// Median frame time for a still and a moving camera. BENCH_FRAMES=N frames per leg;
// VIEWER_LINE_STYLE=tubes|flat picks the solid-lane style; VIEWER_W / VIEWER_H size it.

use session_viewer::selftest::{frame_profile, SceneFile};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let files = SceneFile::from_args(&args);
    let w = std::env::var("VIEWER_W").ok().and_then(|v| v.parse().ok()).unwrap_or(900);
    let h = std::env::var("VIEWER_H").ok().and_then(|v| v.parse().ok()).unwrap_or(700);
    print!("{}", frame_profile(&files, w, h));
}
```

## Step 14 - The probe scenes

- Each `mk_*` writes one `.pb` built so that a bug becomes a count: a plate whose bottom outline is magenta (a magenta pixel from above is ink through the plate), two quads wound opposite ways (one render proves both facing branches), red linework behind a box (any red pixel is penetration).

_Paste it._
**Create `examples/mk_facing_probe.rs`**

```rust
// Back-face probe: two coplanar 200 mm quads side by side, wound in OPPOSITE directions.
// From any one camera exactly one of them shows its back — so a single render proves both
// branches of the front_facing test at once.
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/facing.pb".into());
    let p = |x: f64, y: f64| session_rust::Point::new(x, y, 0.0);
    let polys = vec![
        vec![p(0.0, 0.0), p(200.0, 0.0), p(200.0, 200.0), p(0.0, 200.0)],      // CCW seen from +Z
        vec![p(240.0, 0.0), p(240.0, 200.0), p(440.0, 200.0), p(440.0, 0.0)],  // CW  seen from +Z
    ];
    let m = session_rust::Mesh::from_polylines(polys, None);
    let mut s = session_rust::Session::new("facing");
    s.add_mesh(m, None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_wedge_scene.rs`**

```rust
// Generates the wedge acceptance scene: ONE box with red 10 mm world pens, plus a tiny anchor
// point at the origin. The anchor pulls the fit target ~400 mm off the box, so VIEWER_ZOOM
// dollies the camera to a hover just OUTSIDE the box's west face instead of inside the solid
// (where the facing cull would legitimately remove every edge). No third object means no
// genuine occlusion: the depth-on vs depth-forced-Always comparison isolates the bug.
//
// cargo run --example mk_wedge_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/onebox.pb".into());

    let mut m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = m.edges_with_colors().len();
    m.set_linecolors(vec![session_rust::Color::red(); n], vec![10.0; n]); // 10 mm plot pen
    let guid = m.guid().to_string();

    let mut s = session_rust::Session::new("wedge_box");
    s.add_mesh(m, None);
    s.set_xform(&guid, session_rust::Xform::translation(600.0, 0.0, 0.0));
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_cube_scene.rs`**

```rust
// Grey-cube probe for the vertex-marker check: ONE default box (grey faces, black edges - the
// colors an unauthored mesh gets), so the corner markers come out black on the pens' own color
// path. Same anchor trick as mk_wedge_scene, so VIEWER_ZOOM hovers outside the west face.
//
// cargo run --example mk_cube_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/greycube.pb".into());

    let mut m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = m.edges_with_colors().len();
    // Grey faces, black 10 mm world pens - wide enough that a corner marker that is NOT fully in
    // front shows a bite out of its disc.
    m.set_linecolors(vec![session_rust::Color::black(); n], vec![10.0; n]);
    let guid = m.guid().to_string();

    let mut s = session_rust::Session::new("grey_box");
    s.add_mesh(m, None);
    s.set_xform(&guid, session_rust::Xform::translation(600.0, 0.0, 0.0));
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None);
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_dots_scene.rs`**

```rust
// Dots-only probe for the glyph-size investigation: the SAME box corners as mk_wedge_scene,
// but as 8 Point objects with a 10 mm pen and no mesh - so the glyph lane renders alone and
// the disc's intended size can be measured without the bands drawing over it.
//
// cargo run --example mk_dots_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/wedge/dots.pb".into());

    let mut s = session_rust::Session::new("dots_box");
    for &sx in &[0.0, 400.0] {
        for &sy in &[0.0, 400.0] {
            for &sz in &[0.0, 400.0] {
                let mut p = session_rust::Point::new(600.0 - 200.0 + sx, -200.0 + sy, -200.0 + sz);
                p.width = 10.0;
                p.pointcolor = session_rust::Color::red();
                s.add_point(p, None);
            }
        }
    }
    s.add_point(session_rust::Point::new(0.0, 0.0, 0.0), None); // anchor, same fit as onebox.pb
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_freelines_scene.rs`**

```rust
// Probe: a grey 400 mm box PLUS the same 12 edges as free Lines (FACING_UNKNOWN, so the facing
// cull never fires and hidden-line removal must come from the depth test alone). Any far-side
// line drawn over the box is ink lifted in front of the surface it decorates.
//
// cargo run --example mk_freelines_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/freelines.pb".into());
    let m = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let edges = m.edges();
    let mut lines = Vec::new();
    for (a, b) in &edges {
        lines.push({ let p0 = m.vertex_point(*a).unwrap(); let p1 = m.vertex_point(*b).unwrap(); session_rust::Line::new(p0[0], p0[1], p0[2], p1[0], p1[1], p1[2]) });
    }
    let mut s = session_rust::Session::new("freelines");
    s.add_mesh(m, None);
    for l in lines {
        s.add_line(l, None);
    }
    s.pb_dump(&out);
    println!("wrote {out} ({} lines)", edges.len());
}
```

_Paste it._
**Create `examples/mk_twobox_scene.rs`**

```rust
// Occlusion probe: a grey 400 box in FRONT, and 400 mm of red linework BEHIND it (mesh edges as
// free Lines at x = -600, so they ride the ribbon lane with FACING_UNKNOWN). From the +x side
// the red ink must be fully hidden; any red pixel is ink through a face, and countable.
//
// cargo run --example mk_twobox_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/twobox.pb".into());
    let front = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let back = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let edges = back.edges();
    let mut lines = Vec::new();
    for (a, b) in &edges {
        let p0 = back.vertex_point(*a).unwrap();
        let p1 = back.vertex_point(*b).unwrap();
        lines.push(session_rust::Line::new(p0[0] - 600.0, p0[1], p0[2], p1[0] - 600.0, p1[1], p1[2]));
    }
    let mut s = session_rust::Session::new("twobox");
    s.add_mesh(front, None);
    for mut l in lines {
        l.linecolor = session_rust::Color::red();
        s.add_line(l, None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_twobox_mesh.rs`**

```rust
// Tube-lane ground truth for mk_twobox_scene: the SAME back box, but as a mesh with red
// screen-px pens, so its edges render as real cylinder geometry with honest depth. Red on the
// front box here = genuinely visible; red in the ribbon scene but not here = penetration.
//
// cargo run --example mk_twobox_mesh --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/twobox_mesh.pb".into());
    let front = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let mut back = session_rust::Mesh::create_box(400.0, 400.0, 400.0);
    let n = back.edges_with_colors().len();
    back.set_linecolors(vec![session_rust::Color::red(); n], vec![-1.0; n]); // screen-px pen
    let guid = back.guid().to_string();
    let mut s = session_rust::Session::new("twobox_mesh");
    s.add_mesh(front, None);
    s.add_mesh(back, None);
    s.set_xform(&guid, session_rust::Xform::translation(-600.0, 0.0, 0.0));
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_plate_scene.rs`**

```rust
// Thin-shell probe: a 400 x 400 x 8 mm plate, subdivided 8x8 on the big faces, black default
// pens. The regime where ink lift + face push can exceed the OBJECT's own thickness - if they
// do, the far face's wireframe surfaces through the near face (the bunny-ear black-out).
//
// cargo run --example mk_plate_scene --target x86_64-unknown-linux-gnu --release -- <out.pb>
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "target/plate.pb".into());
    let mut polys = Vec::new();
    let n = 8;
    let s = 400.0 / n as f64;
    for i in 0..n {
        for j in 0..n {
            let (x0, y0) = (i as f64 * s, j as f64 * s);
            for z in [0.0, 8.0] {
                polys.push(vec![
                    session_rust::Point::new(x0, y0, z),
                    session_rust::Point::new(x0 + s, y0, z),
                    session_rust::Point::new(x0 + s, y0 + s, z),
                    session_rust::Point::new(x0, y0 + s, z),
                ]);
            }
        }
    }
    // side walls
    for i in 0..n {
        let x0 = i as f64 * s;
        polys.push(vec![
            session_rust::Point::new(x0, 0.0, 0.0), session_rust::Point::new(x0 + s, 0.0, 0.0),
            session_rust::Point::new(x0 + s, 0.0, 8.0), session_rust::Point::new(x0, 0.0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(x0, 400.0, 0.0), session_rust::Point::new(x0 + s, 400.0, 0.0),
            session_rust::Point::new(x0 + s, 400.0, 8.0), session_rust::Point::new(x0, 400.0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(0.0, x0, 0.0), session_rust::Point::new(0.0, x0 + s, 0.0),
            session_rust::Point::new(0.0, x0 + s, 8.0), session_rust::Point::new(0.0, x0, 8.0)]);
        polys.push(vec![
            session_rust::Point::new(400.0, x0, 0.0), session_rust::Point::new(400.0, x0 + s, 0.0),
            session_rust::Point::new(400.0, x0 + s, 8.0), session_rust::Point::new(400.0, x0, 8.0)]);
    }
    let m = session_rust::Mesh::from_polylines(polys, None);
    let mut s2 = session_rust::Session::new("plate");
    s2.add_mesh(m, None);
    s2.pb_dump(&out);
    println!("wrote {out}");
}
```

_Paste it._
**Create `examples/mk_plate_outline.rs`**

```rust
// Depth-fight probe: three grey plates, each with its TOP outline as a closed pure-blue
// polyline on the rim of its top face and its BOTTOM outline as a closed pure-magenta polyline
// on its bottom face, inset 20 mm inside the footprint so it is hidden by the plate from every
// angle above - a magenta pixel seen from above is therefore ink THROUGH the plate, never a
// silhouette. Plate 1: 4000 x 300 x 40 mm flat at y = 0 (the thin regime). Plate 2: the same
// 200 mm thick at y = 600. Plate 3: the 40 mm plate rotated 30 deg about its long axis at
// y = 1200, baked into its vertices (no xform): its axis-aligned box is 185 mm thick, the plate
// 40, which is the case a box-based thickness gets wrong. Render with VIEWER_NO_EDGES=1 so the
// meshes' own black wireframe stays out of the count.
//
// cargo run --example mk_plate_outline --target x86_64-unknown-linux-gnu --release -- <out.pb>
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};

const INSET: f64 = 20.0;

/// A closed rectangle at height `z`, `inset` inside the plate footprint, as a polyline.
fn outline(y0: f64, z: f64, inset: f64, color: Color) -> Polyline {
    let (x0, x1, ya, yb) = (inset, 4000.0 - inset, y0 + inset, y0 + 300.0 - inset);
    let mut pl = Polyline::new(vec![Point::new(x0, ya, z), Point::new(x1, ya, z), Point::new(x1, yb, z), Point::new(x0, yb, z), Point::new(x0, ya, z)]);
    pl.linecolor = color;
    pl
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or("target/plate_outline.pb".to_string());
    let mut s = Session::new("plate_outline");
    for (y0, dz, tilt) in [(0.0, 40.0, 0.0), (600.0, 200.0, 0.0), (1200.0, 40.0, 30.0)] {
        let mut plate = Mesh::create_box(4000.0, 300.0, dz);
        plate.transform(&Xform::translation(2000.0, y0 + 150.0, dz * 0.5));
        plate.set_objectcolor(Color::grey());
        let mut top = outline(y0, dz, 0.0, Color::blue());
        let mut bottom = outline(y0, 0.0, INSET, Color::magenta());
        if tilt != 0.0 {
            let about = Xform::translation(0.0, y0 + 150.0, dz * 0.5) * Xform::rotation_x(tilt, true) * Xform::translation(0.0, -(y0 + 150.0), -dz * 0.5);
            plate.transform(&about);
            top.transform(&about);
            bottom.transform(&about);
        }
        s.add_mesh(plate, None);
        s.add_polyline(top, None);
        s.add_polyline(bottom, None);
    }
    s.pb_dump(&out);
    println!("wrote {out}");
}
```

## Step 15 - The inspectors

- Where a load's time and memory go, and whether two loads of the same bytes agree: `bench_load` splits decode / build / walk, `probe_mem` counts the live heap with a counting allocator (RSS cannot say what costs what), `check_determinism` loads every file twice, `stream_decode_check` proves the range reader lands on the kernel's own values, `census_plates` judges the depth rule by ray-casting every outline sample.

_Paste it._
**Create `examples/pb_bbox.rs`**

```rust
// Print each point cloud's bounding box from .pb files

fn main() {
    for path in std::env::args().skip(1){
        let bytes = std::fs::read(&path).expect("read");
        let s = session_rust::Session::pb_loads(&bytes).expect("parse");
        for g in s.order(){
            if let Some(session_rust::Geometry::PointCloud(pc)) = s.lookup.get(&g){
                let c = pc.coords();
                let mut mn = [f64::INFINITY; 3];
                let mut mx = [f64::NEG_INFINITY; 3];
                for i in (0..c.len()).step_by(3){
                    for k in 0..3{
                        mn[k] = mn[k].min(c[i+k]);
                        mx[k] = mx[k].max(c[i+k]);
                    }
                    // percentile bounds too: a scane's min/max box is mostly empty air
                    let n = c.len() / 3;
                    let mut pl = [0.0f64; 3];
                    let mut ph = [0.0f64; 3];
                    for k in 0..3 {
                        let mut v: Vec<f64> = (0..n).step_by((n / 20000).max(1)).map(|i| c[i*3 + k]).collect();
                        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        pl[k] = v[v.len() * 2 / 100];
                        ph[k] = v[v.len() * 98 / 100];
                    }
                    println!("{path} {mn:?} {mx:?} p2 {pl:?} p98 {ph:?}");
                }
            }
        }
    }
}
```

_Paste it._
**Create `examples/dump_geometry.rs`**

```rust
// Print every object of a .pb: type, name, and its box (meshes) or points (polylines/lines).
fn main() {
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let s = session_rust::Session::pb_loads(&bytes).expect("parse");
        let world = s.world_xforms();
        for g in s.order() {
            if let Some(x) = world.get(&g) && x.m != session_rust::Xform::identity().m {
                println!("  xform t=({:.0},{:.0},{:.0})", x.m[12], x.m[13], x.m[14]);
            }
            match s.lookup.get(&g) {
                Some(session_rust::Geometry::Mesh(m)) => {
                    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
                    for v in m.vertex.values() {
                        for (k, c) in [v.x, v.y, v.z].iter().enumerate() {
                            lo[k] = lo[k].min(*c);
                            hi[k] = hi[k].max(*c);
                        }
                    }
                    println!("mesh {:?} box {:?} .. {:?} color {:?}", m.name, lo, hi, m.objectcolor());
                }
                Some(session_rust::Geometry::Polyline(p)) => {
                    let pts: Vec<[f64; 3]> = p.coords.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
                    println!("polyline {:?} width {} color {:?} points {:?}", p.name, p.width, p.linecolor, pts);
                }
                Some(session_rust::Geometry::Point(pt)) => println!("point {:?}", [pt[0], pt[1], pt[2]]),
                Some(other) => println!("{}", std::any::type_name_of_val(other)),
                None => {}
            }
        }
    }
}
```

_Paste it._
**Create `examples/probe_mesh.rs`**

```rust
// cargo run --example probe_mesh --target x86_64-unknown-linux-gnu --release -- <file.pb>...
// What is in a .pb, and which meshes are "print fills" (broadcast width 0)?
fn main() {
    use session_rust::Geometry;
    for path in std::env::args().skip(1) {
        let bytes = std::fs::read(&path).expect("read");
        let session = session_rust::Session::pb_loads(&bytes).expect("parse");
        println!("{path}:");
        let (mut nmesh, mut npoly, mut nline, mut npoint, mut nother) = (0, 0, 0, 0, 0);
        let mut width_hist: std::collections::HashMap<usize, usize> = Default::default();
        let mut red_meshes = 0;
        let mut print_fills = 0;
        let mut empty_widths = 0;
        let mut alpha_hist: std::collections::HashMap<String, usize> = Default::default();
        for guid in session.order() {
            match session.lookup.get(&guid) {
                Some(Geometry::Mesh(m)) => {
                    nmesh += 1;
                    let wl = m.widths().len();
                    *width_hist.entry(wl).or_default() += 1;
                    if wl == 0 { empty_widths += 1; }
                    if wl == 1 && m.widths()[0] == 0.0 { print_fills += 1; }
                    let oc = m.objectcolor();
                    *alpha_hist.entry(format!("{:.2}", oc.a)).or_default() += 1;
                    if oc.r > 0.5 && oc.g < 0.4 && oc.b < 0.4 { red_meshes += 1; }
                }
                Some(Geometry::Polyline(_)) => npoly += 1,
                Some(Geometry::Line(_)) => nline += 1,
                Some(Geometry::Point(_)) => npoint += 1,
                _ => nother += 1,
            }
        }
        println!("  meshes={nmesh} polylines={npoly} lines={nline} points={npoint} other={nother}");
        println!("  mesh widths_len histogram: {width_hist:?}");
        println!("  print_fills(broadcast 0)={print_fills} empty_widths={empty_widths} reddish_meshes={red_meshes}");
        println!("  mesh objectcolor alpha histogram: {alpha_hist:?}");
    }
}
```

_Paste it._
**Create `examples/probe_mem.rs`**

```rust
//! EXACT live-heap accounting for one .pb, via a counting global allocator.
//!
//! RSS cannot answer "what costs what" - it counts allocator slack and never shrinks. A counting
//! allocator can: load the file, then DROP one part at a time and read the delta.
use session_rust::{Session, Geometry, Line, Point, Polyline, Mesh, Color, NurbsCurve};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static NALLOC: AtomicUsize = AtomicUsize::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let n = LIVE.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(n, Relaxed);
        NALLOC.fetch_add(1, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        LIVE.fetch_sub(l.size(), Relaxed);
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, new: usize) -> *mut u8 {
        LIVE.fetch_add(new, Relaxed);
        LIVE.fetch_sub(l.size(), Relaxed);
        PEAK.fetch_max(LIVE.load(Relaxed), Relaxed);
        NALLOC.fetch_add(1, Relaxed);
        unsafe { System.realloc(p, l, new) }
    }
}
#[global_allocator]
static A: Counting = Counting;

fn live() -> f64 { LIVE.load(Relaxed) as f64 / 1.048576e6 }
fn mb(b: usize) -> f64 { b as f64 / 1.048576e6 }

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let bytes = std::fs::read(&path).unwrap();
    let file_mb = mb(bytes.len());
    let base = live();
    let t = Instant::now();
    let mut s = Session::pb_loads(&bytes).unwrap();
    let decode = t.elapsed();
    drop(bytes);
    let n0 = NALLOC.load(Relaxed);
    let total = live() - base + file_mb;
    println!("{}", path.rsplit('/').next().unwrap());
    println!("  file {file_mb:.1} MB | pb_loads {decode:?} | live heap {:.1} MB ({:.1}x file) | peak {:.1} MB | {:.2} M allocations",
        total, total / file_mb, mb(PEAK.load(Relaxed)), n0 as f64 / 1e6);
    println!("  sizeof: Line {} Point {} Polyline {} Mesh {} Color {} NurbsCurve {} Geometry {}",
        std::mem::size_of::<Line>(), std::mem::size_of::<Point>(), std::mem::size_of::<Polyline>(),
        std::mem::size_of::<Mesh>(), std::mem::size_of::<Color>(), std::mem::size_of::<NurbsCurve>(),
        std::mem::size_of::<Geometry>());
    let o = &s.objects;
    println!("  counts: {} lines {} plines {} points {} meshes {} nurbs | lookup {} | graph v {} e {}",
        o.lines.len(), o.polylines.len(), o.points.len(), o.meshes.len(), o.nurbscurves.len(),
        s.lookup.len(), s.graph.vertex_count, s.graph.edges.len());
    let (mv, mf): (usize, usize) = o.meshes.iter().fold((0, 0), |(a, b), m| (a + m.vertex.len(), b + m.face.len()));
    println!("  mesh interiors: {mv} verts {mf} faces");

    // Drop one part at a time; each delta is that part's exact live cost.
    let mut prev = live();
    let step = |name: &str, now: f64, prev: &mut f64| {
        println!("  {name:<28} {:>7.1} MB", *prev - now);
        *prev = now;
    };
    s.lookup = Default::default();               step("lookup (guid -> Rc)", live(), &mut prev);
    s.graph = Default::default();                step("graph", live(), &mut prev);
    s.tree = Default::default();                 step("tree", live(), &mut prev);
    {
        let lines = std::mem::take(&mut s.objects.lines);
        let mut owned: Vec<Line> = lines.into_iter().filter_map(|l| std::rc::Rc::try_unwrap(l).ok()).collect();
        println!("  unwrapped {} lines", owned.len());
        let mut p2 = live();
        for l in owned.iter_mut() { l.name = String::new(); }
        step("  line.name", live(), &mut p2);
        for l in owned.iter_mut() { l.dash = Vec::new(); }
        step("  line.dash", live(), &mut p2);
        for l in owned.iter_mut() { l.linecolor.name = String::new(); }
        step("  line.linecolor.name", live(), &mut p2);
        for l in owned.iter_mut() { l.linecolor = Color::new(0.0, 0.0, 0.0, 1.0); }
        step("  line.linecolor.guid", live(), &mut p2);
        drop(owned);
        step("  line guid + struct", live(), &mut p2);
    }
    step("lines TOTAL", live(), &mut prev);
    s.objects.nurbscurves = Vec::new();          step("nurbscurves", live(), &mut prev);
    s.objects.polylines = Vec::new();            step("polylines", live(), &mut prev);
    s.objects.points = Vec::new();               step("points", live(), &mut prev);
    {
        println!("  sizeof VertexData {} | HashMap {} | Vec<usize> {}",
            std::mem::size_of::<session_rust::mesh::VertexData>(),
            std::mem::size_of::<std::collections::HashMap<usize, f64>>(),
            std::mem::size_of::<Vec<usize>>());
        let meshes = std::mem::take(&mut s.objects.meshes);
        let mut owned: Vec<Mesh> = meshes.into_iter().filter_map(|m| std::rc::Rc::try_unwrap(m).ok()).collect();
        println!("  unwrapped {} meshes", owned.len());
        let mut p2 = live();
        for m in owned.iter_mut() { m.vertex = Default::default(); }
        step("  mesh.vertex", live(), &mut p2);
        for m in owned.iter_mut() { m.face = Default::default(); }
        step("  mesh.face", live(), &mut p2);
        for m in owned.iter_mut() { m.triangulation = Default::default(); }
        step("  mesh.triangulation", live(), &mut p2);
        for m in owned.iter_mut() { m.facedata = Default::default(); m.edgedata = Default::default(); m.face_holes = Default::default(); }
        step("  mesh.facedata/edge/holes", live(), &mut p2);
        for m in owned.iter_mut() { m.clear_pointcolors(); m.clear_facecolors(); m.clear_linecolors(); }
        step("  mesh colors+widths", live(), &mut p2);
        drop(owned);
        step("  mesh rest", live(), &mut p2);
    }
    step("meshes TOTAL", live(), &mut prev);
    s.objects.pointclouds = Vec::new();          step("pointclouds", live(), &mut prev);
    drop(s);                                     step("the rest", live(), &mut prev);
    println!("  residual {:.1} MB", live() - base);
}
```

_Paste it._
**Create `examples/bench_load.rs`**

```rust
//! Where the load time actually goes: prost decode vs object build vs lookup vs walk.
use std::time::Instant;
use std::rc::Rc;
use prost::Message;
use session_rust::{proto, Session, Geometry, Polyline, Point, Line, Mesh, PointCloud, Xform};
use session_viewer::app::scene::{FileDoc, Scene};

fn main() {
    let path = std::env::args().nth(1).expect("usage: bench_load <file.pb>");
    let t = Instant::now();
    let bytes = std::fs::read(&path).unwrap();
    println!("read           {:>7.0} ms  ({:.1} MB)", t.elapsed().as_secs_f64()*1e3, bytes.len() as f64/1.048576e6);

    let t = Instant::now();
    let p = proto::Session::decode(&bytes[..]).unwrap();
    println!("prost decode   {:>7.0} ms  (full: objects + tree + graph)", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let lean = session_rust::proto::Session::decode(&bytes[..]).unwrap();
    println!("lean decode    {:>7.0} ms  (objects + xforms only, {} xforms)", t.elapsed().as_secs_f64()*1e3, lean.xforms.len());

    {
        use prost::Message;
        fn count(n: &session_rust::proto::TreeNode) -> (usize, usize) {
            let mut c = 1; let mut b = n.name.len();
            for ch in &n.children { let (cc, bb) = count(ch); c += cc; b += bb; }
            (c, b)
        }
        let tree_len = p.tree.as_ref().map_or(0, |t| t.encoded_len());
        let (nodes, namebytes) = p.tree.as_ref().and_then(|t| t.root.as_ref()).map_or((0,0), count);
        println!("  tree: {:.1} MB encoded | {nodes} nodes | {:.1} MB of names", tree_len as f64/1.048576e6, namebytes as f64/1.048576e6);
        println!("  xforms: {} entries | graph {:.1} MB", p.xforms.len(), p.graph.as_ref().map_or(0, |g| g.encoded_len()) as f64/1.048576e6);
        println!("  objects total: {:.1} MB", p.objects.as_ref().map_or(0, |o| o.encoded_len()) as f64/1.048576e6);
    }
    {
        use prost::Message;
        use session_rust::proto as sp;
        #[derive(Clone, PartialEq, prost::Message)]
        struct LinesOnly { #[prost(message, repeated, tag = "4")] lines: Vec<sp::Line> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct MeshesOnly { #[prost(message, repeated, tag = "9")] meshes: Vec<sp::Mesh> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LinesSess { #[prost(message, optional, tag = "3")] objects: Option<LinesOnly> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct MeshSess { #[prost(message, optional, tag = "3")] objects: Option<MeshesOnly> }
        let t = Instant::now();
        let l = LinesSess::decode(&bytes[..]).unwrap();
        println!("  lines only   {:>7.0} ms  ({} lines)", t.elapsed().as_secs_f64()*1e3, l.objects.map_or(0, |o| o.lines.len()));
        // Wire-identical mirror: protobuf encodes map<K,V> exactly as repeated {K key=1; V value=2},
        // so declaring the map fields `repeated` turns 700k+ hashed map inserts into Vec pushes.
        #[derive(Clone, PartialEq, prost::Message)]
        struct VEntry { #[prost(uint64, tag="1")] k: u64, #[prost(message, optional, tag="2")] v: Option<sp::VertexData> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct FEntry { #[prost(uint64, tag="1")] k: u64, #[prost(message, optional, tag="2")] v: Option<sp::FaceData> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshP {
            #[prost(message, repeated, tag="3")] vertices: Vec<VEntry>,
            #[prost(message, repeated, tag="4")] faces: Vec<FEntry>,
        }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshesOnly { #[prost(message, repeated, tag = "9")] meshes: Vec<LeanMeshP> }
        #[derive(Clone, PartialEq, prost::Message)]
        struct LeanMeshSess { #[prost(message, optional, tag = "3")] objects: Option<LeanMeshesOnly> }
        let t = Instant::now();
        let lm = LeanMeshSess::decode(&bytes[..]).unwrap();
        let (nv, nf) = lm.objects.as_ref().map_or((0,0), |o| (
            o.meshes.iter().map(|m| m.vertices.len()).sum::<usize>(),
            o.meshes.iter().map(|m| m.faces.len()).sum::<usize>()));
        println!("  meshes VEC   {:>7.0} ms  ({nv} verts {nf} faces, no map hashing)", t.elapsed().as_secs_f64()*1e3);

        // Decode the map fields as repeated entries (wire-identical), then BULK-BUILD the
        // BTreeMap the generated type wants. std's FromIterator sorts + bulk-builds, which beats
        // 362k individual B-tree inserts.
        let t = Instant::now();
        let lm2 = LeanMeshSess::decode(&bytes[..]).unwrap();
        let t2 = Instant::now();
        let mut nb = 0usize;
        if let Some(o) = lm2.objects {
            for mm in o.meshes {
                let v: std::collections::BTreeMap<u64, sp::VertexData> =
                    mm.vertices.into_iter().filter_map(|e| e.v.map(|x| (e.k, x))).collect();
                let f: std::collections::BTreeMap<u64, sp::FaceData> =
                    mm.faces.into_iter().filter_map(|e| e.v.map(|x| (e.k, x))).collect();
                nb += v.len() + f.len();
            }
        }
        println!("  VEC+bulk     {:>7.0} ms  (decode {:.0} + build {:.0}, {nb} entries)",
            t.elapsed().as_secs_f64()*1e3, (t2 - t).as_secs_f64()*1e3, t2.elapsed().as_secs_f64()*1e3);

        let t = Instant::now();
        let m = MeshSess::decode(&bytes[..]).unwrap();
        println!("  meshes only  {:>7.0} ms  ({} meshes)", t.elapsed().as_secs_f64()*1e3, m.objects.map_or(0, |o| o.meshes.len()));
    }
    let o = p.objects.as_ref().unwrap();
    println!("  counts: {} pts {} lines {} plines {} meshes {} clouds",
        o.points.len(), o.lines.len(), o.polylines.len(), o.meshes.len(), o.pointclouds.len());

    if let Some(l) = o.lines.first() {
        // P6: coords/linecolor_rgba are packed; the Point and Color sub-messages are gone.
        println!("  sample line: encoded {} B | guid {:?} name {:?} dash {} coords {} rgba {}",
            l.encoded_len(), l.guid, l.name, l.dash.len(), l.coords.len(), l.linecolor_rgba.len());
        let tot: usize = o.lines.iter().map(|l| l.encoded_len()).sum();
        let guids: usize = o.lines.iter().map(|l| l.guid.len() + l.name.len()).sum();
        let dash: usize = o.lines.iter().map(|l| l.dash.len()*8).sum();
        println!("  lines total {:.1} MB | guid+name {:.1} MB | dash {:.1} MB",
            tot as f64/1.048576e6, guids as f64/1.048576e6, dash as f64/1.048576e6);
    }

    {
        use prost::Message;
        let ms = &o.meshes;
        let tot: usize = ms.iter().map(|m| m.encoded_len()).sum();
        let verts: usize = ms.iter().map(|m| m.vertices.len()).sum();
        let attrs: usize = ms.iter().map(|m| m.vertices.values().map(|v| v.attributes.len()).sum::<usize>()).sum();
        let faces: usize = ms.iter().map(|m| m.faces.len()).sum();
        println!("  meshes: {:.1} MB encoded | {verts} verts ({attrs} attr entries) | {faces} faces",
            tot as f64/1.048576e6);
    }

    // object build only (no lookup)
    let o = p.objects.unwrap();
    let t = Instant::now();
    let plines: Vec<Rc<Polyline>> = o.polylines.into_iter().map(|x| Rc::new(Polyline::from_proto(x))).collect();
    println!("polyline build {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, plines.len());
    let t = Instant::now();
    let lines: Vec<Rc<Line>> = o.lines.into_iter().map(|x| Rc::new(Line::from_proto(x))).collect();
    println!("line build     {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, lines.len());
    let t = Instant::now();
    let meshes: Vec<Rc<Mesh>> = o.meshes.into_iter().map(|x| Rc::new(Mesh::from_proto(x))).collect();
    println!("mesh build     {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, meshes.len());
    let t = Instant::now();
    let clouds: Vec<Rc<PointCloud>> = o.pointclouds.into_iter().map(|x| Rc::new(PointCloud::from_proto(x))).collect();
    println!("cloud build    {:>7.0} ms  ({} objs)", t.elapsed().as_secs_f64()*1e3, clouds.len());

    // lookup insert cost, measured on its own
    let mut s = Session::new("bench");
    let t = Instant::now();
    for g in &lines { s.lookup.insert(g.guid().to_string(), Geometry::Line(Rc::clone(g))); }
    for g in &plines { s.lookup.insert(g.guid().to_string(), Geometry::Polyline(Rc::clone(g))); }
    for g in &meshes { s.lookup.insert(g.guid().to_string(), Geometry::Mesh(Rc::clone(g))); }
    for g in &clouds { s.lookup.insert(g.guid().to_string(), Geometry::PointCloud(Rc::clone(g))); }
    println!("lookup insert  {:>7.0} ms  ({} keys)", t.elapsed().as_secs_f64()*1e3, s.lookup.len());
    s.objects.polylines = plines;
    s.objects.lines = lines;
    s.objects.meshes = meshes;
    s.objects.pointclouds = clouds;

    {
        // Do the sheet's fills share a plane? If so the depth buffer cannot order them.
        let mut zs: std::collections::BTreeMap<i64, usize> = std::collections::BTreeMap::new();
        for m in &s.objects.meshes {
            for v in m.vertex.values() { *zs.entry((v.z * 1e6).round() as i64).or_insert(0) += 1; }
        }
        let shown: Vec<String> = zs.iter().take(6).map(|(z, n)| format!("z={:.6} x{n}", *z as f64 / 1e6)).collect();
        println!("  mesh vertex Z levels: {} distinct -> {}", zs.len(), shown.join(", "));
    }

    {
        // What distinguishes text from hatch? Look at the names/colors the importer wrote.
        let mut mnames: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for m in &s.objects.meshes { *mnames.entry(m.name.clone()).or_insert(0) += 1; }
        println!("  mesh names: {:?}", mnames.iter().take(12).collect::<Vec<_>>());
        fn walk(n: &session_rust::proto::TreeNode, out: &mut std::collections::BTreeMap<String, usize>, d: usize) {
            if d < 3 { *out.entry(format!("d{d}:{}", n.name)).or_insert(0) += 1; }
            for c in &n.children { walk(c, out, d + 1); }
        }
        let mut tn: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        if let Some(t) = p.tree.as_ref().and_then(|t| t.root.as_ref()) { walk(t, &mut tn, 0); }
        println!("  tree names (depth<3): {:?}", tn.iter().take(14).collect::<Vec<_>>());
        for (i, m) in s.objects.meshes.iter().enumerate() {
            let oc = m.objectcolor();
            let n = m.number_of_vertices();
            let f = m.face.len();
            let ws: Vec<f64> = m.widths().iter().take(2).copied().collect();
            println!("    mesh[{i}] verts {n:>7} faces {f:>7} color ({:.2},{:.2},{:.2},a={:.2}) widths{:?} pcols {} fcols {}",
                oc.r, oc.g, oc.b, oc.a, ws, m.get_pointcolors().len(), m.get_facecolors().len());
        }
        let mut lnames: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for l in &s.objects.lines { *lnames.entry(l.name.clone()).or_insert(0) += 1; }
        println!("  line names: {:?}", lnames.iter().take(12).collect::<Vec<_>>());
        
    }

    // Is the per-line cost really Point::new (name String + Color::black) x2?
    let t = Instant::now();
    let mut acc = 0.0f64;
    for l in &s.objects.lines { let a = l.start(); let b = l.end(); acc += a.to_f32()[0] as f64 + b.to_f32()[0] as f64; }
    println!("  start()+end()  {:>7.0} ms  (acc {acc:.0})", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let mut acc2 = 0.0f64;
    for l in &s.objects.lines { acc2 += l.length(); }
    println!("  length() only  {:>7.0} ms  (acc {acc2:.0})", t.elapsed().as_secs_f64()*1e3);

    let t = Instant::now();
    let mut closed = 0;
    for m in &s.objects.meshes { if m.is_closed() { closed += 1; } }
    println!("  is_closed()    {:>7.0} ms  ({closed}/{} closed)", t.elapsed().as_secs_f64()*1e3, s.objects.meshes.len());
    let t = Instant::now();
    let mut nv = 0;
    for m in &s.objects.meshes { nv += m.number_of_vertices(); }
    println!("  n_vertices()   {:>7.0} ms  ({nv} verts)", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let mut nr = 0;
    for m in &s.objects.meshes { nr += m.to_render().vertices.len(); }
    println!("  to_render()    {:>7.0} ms  ({nr} rows)", t.elapsed().as_secs_f64()*1e3);

    // what add_file pays BEFORE touching geometry: order() Strings + lookup hashing
    let t = Instant::now();
    let ord = s.order();
    println!("  order()      {:>7.0} ms  ({} guid Strings)", t.elapsed().as_secs_f64()*1e3, ord.len());
    let t = Instant::now();
    let mut hit = 0usize;
    for g in &ord { if s.lookup.contains_key(g) { hit += 1; } }
    println!("  lookup.get() {:>7.0} ms  ({hit} hits)", t.elapsed().as_secs_f64()*1e3);
    let t = Instant::now();
    let w = s.world_xforms();
    println!("  world_xforms {:>7.0} ms  ({} entries)", t.elapsed().as_secs_f64()*1e3, w.len());

    let t = Instant::now();
    let mut scene = Scene::new();
    scene.add_file(FileDoc { name: "bench".into(), session: std::rc::Rc::new(s), place: Xform::identity(), point_px: 1.0, display_only: false });
    println!("walk           {:>7.0} ms", t.elapsed().as_secs_f64()*1e3);
    let _ = Point::new(0.0,0.0,0.0);
    let _ = Line::default();
}
```

_Paste it._
**Create `examples/check_determinism.rs`**

```rust
// Flake hunt: load each file TWICE in one process and compare everything a golden test could
// look at. Rust seeds every HashMap differently, so any place the kernel lets map iteration
// order reach an ORDERED result (a Vec, a float sum, a last-writer-wins insert) shows up here as
// a difference between two loads of identical bytes.
//
// cargo run --release --target x86_64-unknown-linux-gnu --example check_determinism -- <file.pb>...
//
// Green means: two loads of the same bytes produce the same GPU tables, the same area/volume/
// centroid, the same edge and halfedge topology, the same JSON. `PB_BYTES=1` additionally
// requires the ENCODED .pb bytes to match - see the note at that check for why it is off.
use session_rust::{Session, Xform};
use session_viewer::app::scene::{FileDoc, Scene};

fn tables(bytes: &[u8]) -> Scene {
    let s = Session::pb_loads(bytes).expect("pb_loads");
    let mut sc = Scene::new();
    sc.add_file(FileDoc { name: "d".into(), session: std::rc::Rc::new(s), place: Xform::identity(), point_px: 0.0, display_only: false });
    sc
}

fn main() {
    let mut bad = 0;
    for path in std::env::args().skip(1) {
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let name = path.rsplit('/').next().unwrap_or(&path).to_string();
        let mut fails: Vec<String> = Vec::new();

        // 1. the GPU tables, byte for byte
        let (a, b) = (tables(&bytes), tables(&bytes));
        macro_rules! same { ($l:ident.$f:ident) => {
            if bytemuck::cast_slice::<_, u8>(&a.tables.$l.$f) != bytemuck::cast_slice::<_, u8>(&b.tables.$l.$f) {
                fails.push(format!("tables.{}.{}", stringify!($l), stringify!($f)));
            }
        }; }
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
        same!(cloud.pos); same!(cloud.col); same!(cloud.nrm);
        if a.tables.bounds != b.tables.bounds { fails.push("tables.bounds".into()) }

        // 2. per-mesh kernel readers a test or an exporter would call
        let (sa, sb) = (Session::pb_loads(&bytes).unwrap(), Session::pb_loads(&bytes).unwrap());
        for (i, (ma, mb)) in sa.objects.meshes.iter().zip(&sb.objects.meshes).enumerate() {
            let mut m = |what: &str| fails.push(format!("mesh[{i}].{what}"));
            if ma.area().to_bits() != mb.area().to_bits() { m("area") }
            if ma.volume().to_bits() != mb.volume().to_bits() { m("volume") }
            let (ca, cb) = (ma.centroid(), mb.centroid());
            if ca.to_f32() != cb.to_f32() { m("centroid") }
            if ma.is_closed() != mb.is_closed() { m("is_closed") }
            if ma.edges_with_colors().iter().map(|e| (e.0, e.1)).ne(
               mb.edges_with_colors().iter().map(|e| (e.0, e.1))) { m("edges_with_colors") }
            if ma.edge_face_map() != mb.edge_face_map() { m("edge_face_map") }
            if ma.to_vertices_and_faces().1 != mb.to_vertices_and_faces().1 { m("to_vertices_and_faces") }
            if ma.jsondump() != mb.jsondump() {
                m("jsondump");
                if std::env::var("DETAIL").is_ok() {
                    let (x, y) = (ma.jsondump().to_string(), mb.jsondump().to_string());
                    let at = x.bytes().zip(y.bytes()).position(|(p, q)| p != q).unwrap_or(x.len().min(y.len()));
                    let lo = at.saturating_sub(120);
                    println!("    jsondump differs at {at}:\n      A: ...{}\n      B: ...{}",
                        &x[lo..(at + 60).min(x.len())], &y[lo..(at + 60).min(y.len())]);
                }
            }
            // ACCEPTED EXCEPTION, not an oversight: prost writes a map field in iteration order,
            // and Mesh's four big maps (vertices, faces, halfedges, triangulation) stay HashMap
            // ON PURPOSE - a BTreeMap there cost 55% on every DECODE (216 -> 338 ms on one 52 MB
            // sheet) to fix an order that only matters when WRITING, and the two encodings are
            // the same file semantically. Nothing in the repo compares .pb bytes. PB_BYTES=1
            // re-enables the check if that ever changes.
            if std::env::var("PB_BYTES").is_ok() && ma.pb_dumps() != mb.pb_dumps() { m("pb_dumps") }
            let (wa, wb) = (ma.weld(0.001), mb.weld(0.001));
            if wa.to_vertices_and_faces().1 != wb.to_vertices_and_faces().1 { m("weld") }
            let (mut ua, mut ub) = ((**ma).clone(), (**mb).clone());
            ua.unify_winding(); ub.unify_winding();
            if ua.to_vertices_and_faces().1 != ub.to_vertices_and_faces().1 { m("unify_winding") }
            if fails.len() > 40 { break }
        }

        if fails.is_empty() {
            println!("{name}: DETERMINISTIC");
        } else {
            bad += 1;
            let mut seen: Vec<String> = Vec::new();
            for f in &fails {
                let kind = f.split('.').next_back().unwrap_or(f).to_string();
                if !seen.contains(&kind) { seen.push(kind) }
            }
            println!("{name}: FLAKY -> {}", seen.join(", "));
        }
    }
    if bad > 0 { std::process::exit(1) }
}
```

_Paste it._
**Create `examples/stream_decode_check.rs`**

```rust
//! Does the streaming reader see exactly what the kernel parser sees?
//!
//! The browser opens a large cloud by BYTE RANGE: it locates `coords` by walking tag/length
//! varints and casts the bytes, never building a protobuf message. That is only safe if it
//! lands on the kernel's own values, and a wrong offset is silent - it renders a plausible
//! cloud in the wrong place or the wrong colour. So assert it, against the whole file.
//!
//!   cargo run --release --target x86_64-unknown-linux-gnu --example stream_decode_check -- <file.pb>

use session_rust::{Geometry, Session};
use session_viewer::app::stream::{positions_from, varint, walk_to_coords};

fn main() {
    let path = std::env::args().nth(1).expect("usage: stream_decode_check <file.pb>");
    let bytes = std::fs::read(&path).expect("read");
    let session = Session::pb_loads(&bytes).expect("parse");
    let g = session.order()[0].clone();
    let Some(Geometry::PointCloud(pc)) = session.lookup.get(&g) else { panic!("not a cloud") };

    // COORDS: field 3, located the way `cloud_fields` locates it.
    let (at, len) = walk_to_coords(&bytes).expect("no coords field");
    let streamed = positions_from(&bytes[at as usize..(at + len) as usize]);
    let kernel = pc.coords();
    assert_eq!(streamed.len(), kernel.len(), "coord count");
    let worst = streamed.iter().zip(kernel).fold(0.0f64, |m, (a, b)| m.max((*a as f64 - *b).abs()));
    // The reader casts f64 to f32 for the GPU, so an f32 ulp is the only allowed difference.
    let tol = kernel.iter().fold(0.0f64, |m, v| m.max(v.abs())) * f32::EPSILON as f64;
    assert!(worst <= tol, "coords differ by more than an f32 cast: {worst:e} > {tol:e}");
    println!("coords: {} values identical to the kernel (worst {worst:.3e}, f32 bound {tol:.3e})", streamed.len());

    // COLOURS: the tag/length pair immediately after the coords run, then packed varints.
    let after = (at + len) as usize;
    let (tag, n) = varint(&bytes, after).expect("tag after coords");
    assert_eq!((tag >> 3, tag & 7), (4, 2), "expected the colours field next");
    let (clen, n2) = varint(&bytes, after + n).expect("colours length");
    let (mut j, mut seen) = (after + n + n2, 0usize);
    let stop = j + clen as usize;
    let kc = pc.colors();
    while j < stop && seen * 4 < kc.len() {
        for k in 0..4 {
            let (v, m) = varint(&bytes, j).expect("colour varint");
            j += m;
            assert_eq!(v as i64, kc[seen * 4 + k] as i64, "colour {seen} channel {k}");
        }
        seen += 1;
    }
    assert_eq!(seen * 4, kc.len(), "colour count");
    println!("colors: {seen} points, every channel identical to the kernel");

    // The browser frames the scene off the PREFIX it has resident, not the whole cloud, so the
    // prefix's box has to be the cloud's box or `fit` aims at the wrong place.
    let box_of = |n: usize| {
        let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in kernel.chunks_exact(3).take(n) {
            for a in 0..3 { lo[a] = lo[a].min(p[a]); hi[a] = hi[a].max(p[a]); }
        }
        (lo, hi)
    };
    let (flo, fhi) = box_of(kernel.len() / 3);
    let (plo, phi) = box_of(2_000_000);
    let cover = (0..3).map(|a| (phi[a] - plo[a]) / (fhi[a] - flo[a])).fold(f64::INFINITY, f64::min);
    assert!(cover > 0.99, "the 2 M prefix box is only {cover:.3} of the cloud's - `fit` would be wrong");
    println!("bounds: the 2 M-point prefix spans {cover:.4} of the full box");
}
```

_Paste it._
**Create `examples/census_plates.rs`**

```rust
//! Plate census: per-mesh AABB extents + face-normal thickness, per-polyline distance to the nearest mesh face plane, the fit camera, and the depth rule judged by ray-casting every outline sample against the plates in front of it at 1x, 4x and 16x the fit distance (VIEWER_W/H size the pen; CENSUS_RECOLOR=<out.pb> writes a copy whose outline segments are magenta when covered, blue when visible, cyan when partly covered).

use session_rust::{Color, Mesh, Polyline, Quaternion, Session, Vector, Xform};
use session_viewer::math::{mat_scale, mat_to_f32, xform_point_f64, Mat4};
use std::cmp::Ordering;
use std::collections::HashMap;

// The rule under test (objects.rs, triangle.wgsl, ribbon.wgsl) and the harness pen.
const THICK_FLOOR: f64 = 0.001;
const PUSH_FRAC: f64 = 0.004;
const PUSH_MAX_THICK: f64 = 0.0;
const LIFT_HAIR_PX: f64 = 0.25;
const LIFT_MAX_MM: f64 = 0.5;
const LIFT_RADII_FREE: f64 = 1.0;
const LIFT_MAX_THICK: f64 = 0.25;
const PEN_PX: f64 = 2.0;
const ON_FACE_TOL: f64 = 0.01;
const SAMPLE_MM: f64 = 50.0;
const SCALES: [f64; 3] = [1.0, 4.0, 16.0];

struct Face {
    n: [f64; 3],
    d: f64,
    behind: f64,
}

struct Plate {
    verts: Vec<[f64; 3]>,
    faces: Vec<Face>,
    tris: Vec<[[f64; 3]; 3]>,
    lo: [f64; 3],
    hi: [f64; 3],
    ext: [f64; 3],
    diag: f64,
    t_rule: f64,
    t_real: f64,
    big_nz: f64,
}

struct Outline {
    pts: Vec<[f64; 3]>,
    samples: Vec<[f64; 3]>,
    lo: [f64; 3],
    hi: [f64; 3],
    ext: [f64; 3],
    diag: f64,
    t_rule: f64,
    nz: f64,
    dist: f64,
    plate: usize,
    face: usize,
}

struct Fit {
    eye: [f64; 3],
    fwd: [f64; 3],
    distance: f64,
}

// One outline sample against the rule: covered by a plate in front?, its eye depth (m), the
// binding cover's separation along the ray, its push, the sample's lift, the margin (mm), the
// binding plate.
#[derive(Clone)]
struct Verdict {
    covered: bool,
    w: f64,
    sep: f64,
    push: f64,
    lift: f64,
    margin: f64,
    plate: usize,
}

fn sub(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn norm(a: &[f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

fn unit(a: &[f64; 3]) -> [f64; 3] {
    let l = norm(a).max(1e-300);
    [a[0] / l, a[1] / l, a[2] / l]
}

fn by_first(a: &(f64, usize), b: &(f64, usize)) -> Ordering {
    a.0.total_cmp(&b.0)
}

fn env_f64(name: &str, default: f64) -> f64 {
    match std::env::var(name) {
        Ok(v) => v.trim().parse().unwrap_or(default),
        Err(_) => default,
    }
}

fn grow(lo: &mut [f64; 3], hi: &mut [f64; 3], p: &[f64; 3]) {
    for k in 0..3 {
        lo[k] = lo[k].min(p[k]);
        hi[k] = hi[k].max(p[k]);
    }
}

fn box_of(pts: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in pts {
        grow(&mut lo, &mut hi, p);
    }
    (lo, hi)
}

fn sorted_extents(lo: &[f64; 3], hi: &[f64; 3]) -> [f64; 3] {
    let mut e = [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]];
    e.sort_by(f64::total_cmp);
    e
}

fn centroid(pts: &[[f64; 3]]) -> [f64; 3] {
    let mut c = [0.0; 3];
    for p in pts {
        for k in 0..3 {
            c[k] += p[k];
        }
    }
    let n = pts.len().max(1) as f64;
    [c[0] / n, c[1] / n, c[2] / n]
}

fn push_mm(w_m: f64, t_rule: f64) -> f64 {
    (PUSH_FRAC * w_m * 1000.0).min(PUSH_MAX_THICK * t_rule)
}

#[allow(dead_code)]
fn lift_free_mm(w_m: f64, t_rule: f64, vp_h: f64) -> f64 {
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    let raw_px = (PEN_PX * 0.5).max(0.5);
    let uncapped = raw_px * LIFT_RADII_FREE * w_m / (proj_y * vp_h);
    uncapped.min(LIFT_MAX_THICK * t_rule)
}

fn stats(label: &str, v: &mut [f64]) {
    if v.is_empty() {
        println!("  {label}: none");
        return;
    }
    v.sort_by(f64::total_cmp);
    let n = v.len();
    let p10 = v[(n - 1) * 10 / 100];
    let med = v[(n - 1) / 2];
    let p90 = v[(n - 1) * 90 / 100];
    println!("  {label}: n={n} min {:.2} p10 {p10:.2} median {med:.2} p90 {p90:.2} max {:.2}", v[0], v[n - 1]);
}

fn zero_translation(m: &Mat4) -> Mat4 {
    let mut out = *m;
    out[12] = 0.0;
    out[13] = 0.0;
    out[14] = 0.0;
    out
}

fn plate_of(m: &Mesh, place: &Mat4) -> Plate {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(m.vertex.len());
    for key in m.vertices() {
        let v = &m.vertex[&key];
        local.push([v.x, v.y, v.z]);
    }
    let mut verts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        verts.push(xform_point_f64(place, *p));
    }
    let (lo, hi) = box_of(&verts);
    let ext = sorted_extents(&lo, &hi);
    let diag = norm(&sub(&hi, &lo));
    let mc = centroid(&verts);
    let mut faces = Vec::new();
    let mut tris = Vec::new();
    let mut t_real = f64::INFINITY;
    let mut big_area = 0.0;
    let mut big_nz = 0.0;
    for fk in m.faces() {
        let Some(nv) = m.face_normal(fk) else { continue };
        let Some(fpts) = m.face_points(fk) else { continue };
        let mut pts: Vec<[f64; 3]> = Vec::with_capacity(fpts.len());
        for p in &fpts {
            pts.push(xform_point_f64(place, [p[0], p[1], p[2]]));
        }
        for i in 1..pts.len().saturating_sub(1) {
            tris.push([pts[0], pts[i], pts[i + 1]]);
        }
        let mut n = unit(&xform_point_f64(&zero_translation(place), [nv[0], nv[1], nv[2]]));
        let c = centroid(&pts);
        if dot(&n, &sub(&c, &mc)) < 0.0 {
            n = [-n[0], -n[1], -n[2]];
        }
        let d = dot(&n, &c);
        let mut lo_n = f64::INFINITY;
        for v in &verts {
            lo_n = lo_n.min(dot(&n, v));
        }
        let behind = d - lo_n;
        t_real = t_real.min(behind);
        let area = m.face_area(fk).unwrap_or(0.0);
        if area > big_area {
            big_area = area;
            big_nz = n[2].abs();
        }
        faces.push(Face { n, d, behind });
    }
    if !t_real.is_finite() {
        t_real = 0.0;
    }
    let t_rule = t_real.max(THICK_FLOOR * diag);
    Plate { verts, faces, tris, lo, hi, ext, diag, t_rule, t_real, big_nz }
}

fn newell_nz(pts: &[[f64; 3]]) -> f64 {
    let mut n = [0.0; 3];
    for i in 0..pts.len() {
        let a = &pts[i];
        let b = &pts[(i + 1) % pts.len()];
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    if norm(&n) <= 0.0 {
        return f64::NAN;
    }
    unit(&n)[2].abs()
}

// The nearest mesh face plane (max point deviation) and the outline's samples: its vertices
// and its edge midpoints.
fn outline_of(pl: &Polyline, place: &Mat4, plates: &[Plate]) -> Outline {
    let mut local: Vec<[f64; 3]> = Vec::with_capacity(pl.coords.len() / 3);
    for c in pl.coords.chunks_exact(3) {
        local.push([c[0], c[1], c[2]]);
    }
    let scale = mat_scale(&mat_to_f32(place));
    let mut pts: Vec<[f64; 3]> = Vec::with_capacity(local.len());
    for p in &local {
        pts.push(xform_point_f64(place, *p));
    }
    let (lo, hi) = box_of(&pts);
    let ext = sorted_extents(&lo, &hi);
    let diag = norm(&sub(&hi, &lo));
    let mut nn = [0.0f64; 3];
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        nn[0] += (a[1] - b[1]) * (a[2] + b[2]);
        nn[1] += (a[2] - b[2]) * (a[0] + b[0]);
        nn[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let nl = norm(&nn);
    let mut spread = 0.0;
    if nl > 0.0 {
        let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in &pts {
            let d = (p[0] * nn[0] + p[1] * nn[1] + p[2] * nn[2]) / nl;
            smin = smin.min(d);
            smax = smax.max(d);
        }
        spread = smax - smin;
    }
    let t_rule = (spread * scale).max(THICK_FLOOR * diag);
    let mut best = (f64::INFINITY, usize::MAX, usize::MAX);
    for (pi, plate) in plates.iter().enumerate() {
        for (fi, f) in plate.faces.iter().enumerate() {
            let mut dev: f64 = 0.0;
            for p in &pts {
                dev = dev.max((dot(&f.n, p) - f.d).abs());
            }
            if dev < best.0 {
                best = (dev, pi, fi);
            }
        }
    }
    let mut samples: Vec<[f64; 3]> = Vec::with_capacity(pts.len() * 2);
    for w in pts.windows(2) {
        samples.push(w[0]);
        let steps = (norm(&sub(&w[1], &w[0])) / SAMPLE_MM).ceil().max(1.0) as usize;
        for k in 1..steps {
            let t = k as f64 / steps as f64;
            samples.push([w[0][0] + (w[1][0] - w[0][0]) * t, w[0][1] + (w[1][1] - w[0][1]) * t, w[0][2] + (w[1][2] - w[0][2]) * t]);
        }
    }
    if let Some(last) = pts.last() && pts.first() != Some(last) {
        samples.push(*last);
    }
    let nz = newell_nz(&pts);
    let hosted = best.0 <= ON_FACE_TOL;
    let t_rule = if hosted { plates[best.1].t_real.max(THICK_FLOOR * plates[best.1].diag) } else { t_rule };
    Outline { pts, samples, lo, hi, ext, diag, t_rule, nz, dist: best.0, plate: best.1, face: best.2 }
}

fn iso_frame() -> ([f64; 3], [f64; 3], [f64; 3]) {
    let yaw_q = Quaternion::from_axis_angle(Vector::z_axis(), -std::f64::consts::FRAC_PI_6);
    let rv = yaw_q.rotate_vector(Vector::x_axis());
    let pitch_q = Quaternion::from_axis_angle(rv, -std::f64::consts::FRAC_PI_6);
    let o = (pitch_q * yaw_q).normalized();
    let f = o.rotate_vector(Vector::y_axis());
    let u = o.rotate_vector(Vector::z_axis());
    let r = o.rotate_vector(Vector::x_axis());
    ([f[0], f[1], f[2]], [u[0], u[1], u[2]], [r[0], r[1], r[2]])
}

// Camera::fit (src/camera.rs) at the default iso orientation, 60 deg vertical fov, mm -> m.
fn fit(lo: &[f64; 3], hi: &[f64; 3], aspect: f64) -> Fit {
    let (fwd, up, right) = iso_frame();
    let ty = 30.0_f64.to_radians().tan();
    let tx = aspect * ty;
    let s = 0.001;
    let target = [(lo[0] + hi[0]) * 0.5 * s, (lo[1] + hi[1]) * 0.5 * s, (lo[2] + hi[2]) * 0.5 * s];
    let mut distance: f64 = 0.0;
    for c in 0..8u32 {
        let p = [
            (if c & 1 == 0 { lo[0] } else { hi[0] }) * s - target[0],
            (if c & 2 == 0 { lo[1] } else { hi[1] }) * s - target[1],
            (if c & 4 == 0 { lo[2] } else { hi[2] }) * s - target[2],
        ];
        let (x, y, z) = (dot(&p, &right), dot(&p, &up), dot(&p, &fwd));
        distance = distance.max(x.abs() / tx + z);
        distance = distance.max(y.abs() / ty + z);
    }
    let distance = distance * 1.05;
    let eye = [target[0] - fwd[0] * distance, target[1] - fwd[1] * distance, target[2] - fwd[2] * distance];
    Fit { eye, fwd, distance }
}

fn eye_at(f: &Fit, k: f64) -> [f64; 3] {
    let back = f.distance * (k - 1.0);
    [f.eye[0] - f.fwd[0] * back, f.eye[1] - f.fwd[1] * back, f.eye[2] - f.fwd[2] * back]
}

// Ray `o + t d` against the slab box, true when it can hit for some t in [0, 1].
fn hits_box(o: &[f64; 3], d: &[f64; 3], lo: &[f64; 3], hi: &[f64; 3]) -> bool {
    let mut t0: f64 = 0.0;
    let mut t1: f64 = 1.0;
    for k in 0..3 {
        if d[k].abs() < 1e-300 {
            if o[k] < lo[k] || o[k] > hi[k] {
                return false;
            }
            continue;
        }
        let a = (lo[k] - o[k]) / d[k];
        let b = (hi[k] - o[k]) / d[k];
        t0 = t0.max(a.min(b));
        t1 = t1.min(a.max(b));
    }
    t0 <= t1
}

// Moller-Trumbore: t of the hit of `o + t d` on the triangle, if any.
fn hit_tri(o: &[f64; 3], d: &[f64; 3], tri: &[[f64; 3]; 3]) -> Option<f64> {
    let e1 = sub(&tri[1], &tri[0]);
    let e2 = sub(&tri[2], &tri[0]);
    let p = cross(d, &e2);
    let det = dot(&e1, &p);
    if det.abs() < 1e-18 {
        return None;
    }
    let inv = 1.0 / det;
    let s = sub(o, &tri[0]);
    let u = dot(&s, &p) * inv;
    if !(-1e-9..=1.0 + 1e-9).contains(&u) {
        return None;
    }
    let q = cross(&s, &e1);
    let v = dot(d, &q) * inv;
    if v < -1e-9 || u + v > 1.0 + 1e-9 {
        return None;
    }
    Some(dot(&e2, &q) * inv)
}

// Every plate face between the eye (m) and the sample (mm) along the sample's view ray, as
// (t along the ray, plate) with t < 1: the sample's own face (t = 1) is not a cover.
fn covers(plates: &[Plate], eye: &[f64; 3], s: &[f64; 3]) -> Vec<(f64, usize)> {
    let o = [eye[0] * 1000.0, eye[1] * 1000.0, eye[2] * 1000.0];
    let d = sub(s, &o);
    let mut out = Vec::new();
    for (pi, p) in plates.iter().enumerate() {
        if !hits_box(&o, &d, &p.lo, &p.hi) {
            continue;
        }
        for tri in &p.tris {
            if let Some(t) = hit_tri(&o, &d, tri) && t > 1e-9 && t < 1.0 - 1e-7 {
                out.push((t, pi));
            }
        }
    }
    out
}

// The rule at one sample: the outline surfaces only if EVERY cover is pushed behind it, so
// the margin is the best cover's (separation - push) minus the outline's lift.
fn judge(o: &Outline, s: &[f64; 3], plates: &[Plate], eye: &[f64; 3], fwd: &[f64; 3], vp_h: f64) -> Verdict {
    let s_m = [s[0] * 0.001, s[1] * 0.001, s[2] * 0.001];
    // CENSUS_ORTHO_H=<half height mm>: parallel rays along `fwd` from a virtual eye 1 km back;
    // the shader's implied distance is ortho_h / tan(30 deg) and the lift is the ortho formula.
    let ortho_h = env_f64("CENSUS_ORTHO_H", 0.0);
    let eye_used = if ortho_h > 0.0 { [s_m[0] - fwd[0] * 1000.0, s_m[1] - fwd[1] * 1000.0, s_m[2] - fwd[2] * 1000.0] } else { *eye };
    let eye = &eye_used;
    let to_s = sub(&s_m, eye);
    let w = if ortho_h > 0.0 { ortho_h / 30.0_f64.to_radians().tan() * 0.001 } else { dot(&to_s, fwd) };
    let len_mm = norm(&to_s) * 1000.0;
    // mm per pixel at the sample, the host face's slope to the ray, the lift the ribbon needs.
    let mmpp = if ortho_h > 0.0 { 2.0 * ortho_h / vp_h } else { 2.0 * w * 30.0_f64.to_radians().tan() * 1000.0 / vp_h };
    let ray = if ortho_h > 0.0 { *fwd } else { let l = norm(&to_s); [to_s[0] / l, to_s[1] / l, to_s[2] / l] };
    // A hosted outline is drawn IN its face plane: no lift at all. An unhosted one lifts a
    // hair, capped by a quarter of its thickness and by LIFT_MAX_MM.
    let _ = ray;
    let lift = (LIFT_HAIR_PX * mmpp).min(LIFT_MAX_THICK * o.t_rule).min(LIFT_MAX_MM);
    let mut best = Verdict { covered: false, w, sep: 0.0, push: 0.0, lift, margin: f64::INFINITY, plate: usize::MAX };
    for (t, pi) in covers(plates, eye, s) {
        let sep = (1.0 - t) * len_mm;
        let push = if ortho_h > 0.0 { push_mm(w, plates[pi].t_rule) } else { push_mm(w * t, plates[pi].t_rule) };
        let margin = sep - push - lift;
        if !best.covered || margin > best.margin {
            best = Verdict { covered: true, w, sep, push, lift, margin, plate: pi };
        }
    }
    best
}

fn placement(world: &HashMap<String, Xform>, guid: &str) -> Mat4 {
    match world.get(guid) {
        Some(x) => x.m,
        None => Xform::identity().m,
    }
}

// The scene box the harness fits: every drawn object's placed points.
fn scene_box(s: &Session, world: &HashMap<String, Xform>, plates: &[Plate], outlines: &[Outline]) -> ([f64; 3], [f64; 3]) {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in plates {
        grow(&mut lo, &mut hi, &p.lo);
        grow(&mut lo, &mut hi, &p.hi);
    }
    for o in outlines {
        grow(&mut lo, &mut hi, &o.lo);
        grow(&mut lo, &mut hi, &o.hi);
    }
    for p in &s.objects.points {
        grow(&mut lo, &mut hi, &xform_point_f64(&placement(world, p.guid()), [p[0], p[1], p[2]]));
    }
    for l in &s.objects.lines {
        let m = placement(world, l.guid());
        grow(&mut lo, &mut hi, &xform_point_f64(&m, [l[0], l[1], l[2]]));
        grow(&mut lo, &mut hi, &xform_point_f64(&m, [l[3], l[4], l[5]]));
    }
    (lo, hi)
}

// The meshes as they are and every outline recoloured by the fit view: magenta when every
// sample is covered (any magenta pixel is ink through a face), blue when none is, cyan between.
fn recolor(s: &Session, outlines: &[Outline], plates: &[Plate], f0: &Fit, vp_h: f64, out: &str) {
    let mut s2 = Session::new("census");
    for m in &s.objects.meshes {
        s2.add_mesh(m.duplicate(), None);
    }
    let mut counts = [0usize; 3];
    for (i, o) in outlines.iter().enumerate() {
        let mut n = 0;
        for smp in &o.samples {
            if judge(o, smp, plates, &f0.eye, &f0.fwd, vp_h).covered {
                n += 1;
            }
        }
        let class = if n == o.samples.len() { 0 } else if n == 0 { 1 } else { 2 };
        counts[class] += 1;
        let mut p = s.objects.polylines[i].duplicate();
        p.linecolor = [Color::new(1.0, 0.0, 1.0, 1.0), Color::new(0.0, 0.0, 1.0, 1.0), Color::new(0.0, 1.0, 1.0, 1.0)][class].clone();
        s2.add_polyline(p, None);
    }
    s2.pb_dump(out);
    println!("recolored copy: {out}  magenta (fully covered at the fit view) {}  blue (visible) {}  cyan (partly covered) {}", counts[0], counts[1], counts[2]);
}

fn census(path: &str) {
    let vp_w = env_f64("VIEWER_W", 900.0);
    let vp_h = env_f64("VIEWER_H", 700.0);
    let bytes = std::fs::read(path).expect("read pb");
    let s = Session::pb_loads(&bytes).expect("parse pb");
    let world = s.world_xforms();
    println!("== {path}  ({:.2} MB, {} objects, {} xforms, {} meshes, {} polylines, {} points, {} lines)", bytes.len() as f64 / 1.048576e6, s.lookup.len(), s.xforms.len(), s.objects.meshes.len(), s.objects.polylines.len(), s.objects.points.len(), s.objects.lines.len());

    let mut plates: Vec<Plate> = Vec::new();
    for m in &s.objects.meshes {
        plates.push(plate_of(m, &placement(&world, m.guid())));
    }
    println!("meshes: i verts faces | extents sorted (thin mid long) diag | t_rule t_real ratio | big face |nz|");
    for (i, p) in plates.iter().enumerate() {
        println!("  m{i:<3} {:>4} {:>4} | {:8.2} {:8.2} {:8.2} {:8.2} | {:7.2} {:7.2} {:6.2} | {:.3}", p.verts.len(), p.faces.len(), p.ext[0], p.ext[1], p.ext[2], p.diag, p.t_rule, p.t_real, p.t_rule / p.t_real.max(1e-9), p.big_nz);
    }

    let mut outlines: Vec<Outline> = Vec::new();
    for pl in &s.objects.polylines {
        outlines.push(outline_of(pl, &placement(&world, pl.guid()), &plates));
    }
    let (lo, hi) = scene_box(&s, &world, &plates, &outlines);
    let mut f0 = fit(&lo, &hi, vp_w / vp_h);
    if let Ok(e) = std::env::var("CENSUS_EYE") {
        let v: Vec<f64> = e.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if v.len() == 3 {
            let centre = [(lo[0] + hi[0]) * 0.0005, (lo[1] + hi[1]) * 0.0005, (lo[2] + hi[2]) * 0.0005];
            let to = sub(&centre, &[v[0], v[1], v[2]]);
            let d = norm(&to);
            let mut fwd = [to[0] / d, to[1] / d, to[2] / d];
            if let Ok(f) = std::env::var("CENSUS_FWD") {
                let fv: Vec<f64> = f.split(',').filter_map(|t| t.trim().parse().ok()).collect();
                if fv.len() == 3 {
                    let n = (fv[0] * fv[0] + fv[1] * fv[1] + fv[2] * fv[2]).sqrt();
                    fwd = [fv[0] / n, fv[1] / n, fv[2] / n];
                }
            }
            f0 = Fit { eye: [v[0], v[1], v[2]], fwd, distance: d };
            println!("  CENSUS_EYE override: eye ({:.2}, {:.2}, {:.2}) m, distance {:.3} m", v[0], v[1], v[2], d);
        }
    }
    println!("polylines: i pts | extents sorted diag | t_rule | |newell nz| | nearest face plane: dist plate face, thickness normal to it | samples covered at k=1 4 16 | failing samples at k=1 4 16 | min margin mm at k=1 4 16");
    let mut per_k: Vec<Vec<Verdict>> = Vec::new();
    for _ in SCALES {
        per_k.push(Vec::new());
    }
    for (i, o) in outlines.iter().enumerate() {
        let on = o.dist <= ON_FACE_TOL;
        let behind = if on { plates[o.plate].faces[o.face].behind } else { f64::NAN };
        let mut cov = String::new();
        let mut fail = String::new();
        let mut mins = String::new();
        let mut worst1: Option<(Verdict, [f64; 3])> = None;
        for (ki, k) in SCALES.iter().enumerate() {
            let eye = eye_at(&f0, *k);
            let mut n_cov = 0;
            let mut n_fail = 0;
            let mut min_margin = f64::INFINITY;
            for smp in &o.samples {
                let j = judge(o, smp, &plates, &eye, &f0.fwd, vp_h);
                if j.covered {
                    n_cov += 1;
                    min_margin = min_margin.min(j.margin);
                    if j.margin < 0.0 {
                        n_fail += 1;
                        if ki == 0 && worst1.as_ref().is_none_or(|(w, _)| j.margin < w.margin) {
                            worst1 = Some((j.clone(), *smp));
                        }
                    }
                }
                per_k[ki].push(j);
            }
            cov.push_str(&format!(" {n_cov:>2}"));
            fail.push_str(&format!(" {n_fail:>2}"));
            mins.push_str(&format!(" {:8.2}", if min_margin.is_finite() { min_margin } else { f64::NAN }));
        }
        println!("  p{i:<3} {:>2} | {:8.2} {:8.2} {:8.2} {:8.2} | {:6.2} | {:.3} | {:7.4} m{:<3} f{:<3} {:7.2} |{cov} of {:>2} |{fail} |{mins}", o.pts.len(), o.ext[0], o.ext[1], o.ext[2], o.diag, o.t_rule, o.nz, o.dist, o.plate, o.face, behind, o.samples.len());
        if let Some((w, smp)) = &worst1 {
            println!("    WORST k=1 p{i}: sample ({:.0}, {:.0}, {:.0}) mm on m{} f{}  cover m{} (t_real {:.2}) push {:.2} lift {:.2} sep {:.2} margin {:.2}", smp[0], smp[1], smp[2], o.plate, o.face, w.plate, plates[w.plate].t_real, w.push, w.lift, w.sep, w.margin);
        }
    }

    println!("plates: {}", plates.len());
    let mut v = Vec::new();
    for p in &plates {
        v.push(p.ext[0]);
    }
    stats("thickness mm (thinnest AABB axis = what the rule sees)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_rule);
    }
    stats("thickness mm (rule: max(thinnest, 0.001 x diag))", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_real);
    }
    stats("thickness mm (min extent along any face normal = real)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.t_rule / p.t_real.max(1e-9));
    }
    stats("t_rule / t_real (rule overestimate on rotated plates)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.ext[2]);
    }
    stats("length mm (longest AABB axis)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag);
    }
    stats("diagonal mm", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag / p.ext[0].max(1e-9));
    }
    stats("diagonal / thickness(AABB)", &mut v);
    v.clear();
    for p in &plates {
        v.push(p.diag / p.t_real.max(1e-9));
    }
    stats("diagonal / thickness(real)", &mut v);
    let mut flat = 0;
    let mut rotated = 0;
    let mut tris = 0;
    for p in &plates {
        if p.ext[0] < ON_FACE_TOL {
            flat += 1;
        }
        if p.t_rule > p.t_real * 1.01 {
            rotated += 1;
        }
        tris += p.tris.len();
    }
    println!("  flat meshes (thinnest axis < {ON_FACE_TOL} mm): {flat}   plates whose t_rule exceeds t_real by >1%: {rotated}   triangles ray-cast: {tris}");

    println!("polylines: {}", outlines.len());
    v.clear();
    for o in &outlines {
        v.push(o.dist);
    }
    stats("distance to nearest mesh face plane mm (max point deviation)", &mut v);
    v.clear();
    for o in &outlines {
        v.push(o.ext[0]);
    }
    stats("outline thinnest AABB axis mm (its lift cap is 0.25 x max(this, 0.001 x diag))", &mut v);
    v.clear();
    for o in &outlines {
        v.push(o.t_rule);
    }
    stats("outline t_rule mm", &mut v);
    let mut on = 0;
    v.clear();
    for o in &outlines {
        if o.dist <= ON_FACE_TOL {
            on += 1;
            v.push(plates[o.plate].faces[o.face].behind);
        }
    }
    println!("  on a mesh face (<= {ON_FACE_TOL} mm): {on} of {}", outlines.len());
    stats("that plate's thickness normal to the face mm", &mut v);

    println!("scene AABB mm: min [{:.1}, {:.1}, {:.1}] max [{:.1}, {:.1}, {:.1}]  extent {:.1} x {:.1} x {:.1}  diagonal {:.1}", lo[0], lo[1], lo[2], hi[0], hi[1], hi[2], hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2], norm(&sub(&hi, &lo)));
    let proj_y = 1.0 / 30.0_f64.to_radians().tan() * 0.001;
    for (w, h) in [(vp_w, vp_h), (1280.0, 720.0), (1920.0, 1080.0)] {
        let f = fit(&lo, &hi, w / h);
        let px_per_mm = proj_y * h / (2.0 * f.distance);
        println!("  fit {w:.0}x{h:.0}: distance {:.3} m  eye ({:.2}, {:.2}, {:.2}) m  fwd ({:.3}, {:.3}, {:.3})  at target {:.4} px/mm, 1 px = {:.2} mm, uncapped free lift (0.5 px) = {:.2} mm", f.distance, f.eye[0], f.eye[1], f.eye[2], f.fwd[0], f.fwd[1], f.fwd[2], px_per_mm, 1.0 / px_per_mm, 0.5 / px_per_mm * LIFT_RADII_FREE);
    }

    println!("rule at k x fit distance ({vp_w:.0}x{vp_h:.0}), covered outline samples: margin = best cover's (separation along the ray - its face push) - the outline's lift");
    for (ki, k) in SCALES.iter().enumerate() {
        let mut n_cov = 0;
        let mut n_fail = 0;
        let mut seps = Vec::new();
        let mut pushes = Vec::new();
        let mut lifts = Vec::new();
        let mut margins = Vec::new();
        let mut fail_outlines: Vec<usize> = Vec::new();
        let mut cov_outlines = 0;
        let mut grazing = 0;
        let mut at = 0;
        for (i, o) in outlines.iter().enumerate() {
            let mut any_cov = false;
            let mut any_fail = false;
            for _ in &o.samples {
                let j = &per_k[ki][at];
                at += 1;
                if !j.covered {
                    continue;
                }
                any_cov = true;
                n_cov += 1;
                seps.push(j.sep);
                pushes.push(j.push);
                lifts.push(j.lift);
                margins.push(j.margin);
                if j.margin < 0.0 {
                    any_fail = true;
                    n_fail += 1;
                    if j.sep < plates[j.plate].t_real {
                        grazing += 1;
                    }
                }
            }
            if any_cov {
                cov_outlines += 1;
            }
            if any_fail {
                fail_outlines.push(i);
            }
        }
        println!(" k={k:<3} distance {:.2} m  0.4% of it = {:.1} mm  samples {} covered {n_cov} FAIL {n_fail} (of which {grazing} graze a cover's edge: separation < that plate's t_real)  outlines covered {cov_outlines} with a FAIL {}: {fail_outlines:?}", f0.distance * k, PUSH_FRAC * f0.distance * k * 1000.0, per_k[ki].len(), fail_outlines.len());
        stats("separation along the ray mm", &mut seps);
        stats("cover's face push mm", &mut pushes);
        stats("outline lift mm", &mut lifts);
        stats("margin mm", &mut margins);
    }

    let mut order: Vec<(f64, usize)> = Vec::new();
    for (i, p) in plates.iter().enumerate() {
        let mut has_outline = false;
        for o in &outlines {
            has_outline = has_outline || (o.plate == i && o.dist <= ON_FACE_TOL);
        }
        if has_outline && p.t_real > ON_FACE_TOL {
            order.push((p.t_real, i));
        }
    }
    order.sort_by(by_first);
    if order.is_empty() {
        return;
    }
    let picks = [("thinnest outlined plate", order[0].1), ("median outlined plate", order[(order.len() - 1) / 2].1), ("thickest outlined plate", order[order.len() - 1].1)];
    for (label, pi) in picks {
        let p = &plates[pi];
        println!("{label}: m{pi}  extents {:.2} x {:.2} x {:.2}  diag {:.2}  t_rule {:.2} (push cap {:.2})  t_real {:.2}  big face |nz| {:.3}", p.ext[0], p.ext[1], p.ext[2], p.diag, p.t_rule, PUSH_MAX_THICK * p.t_rule, p.t_real, p.big_nz);
        for (oi, o) in outlines.iter().enumerate() {
            if o.plate != pi || o.dist > ON_FACE_TOL {
                continue;
            }
            for k in SCALES {
                let eye = eye_at(&f0, k);
                let mut worst = Verdict { covered: false, w: 0.0, sep: 0.0, push: 0.0, lift: 0.0, margin: f64::INFINITY, plate: usize::MAX };
                let mut n_cov = 0;
                for smp in &o.samples {
                    let j = judge(o, smp, &plates, &eye, &f0.fwd, vp_h);
                    if j.covered {
                        n_cov += 1;
                        if j.margin < worst.margin {
                            worst = j;
                        }
                    }
                }
                if !worst.covered {
                    println!("  p{oi:<3} k={k:<3} no sample covered: the outline is in view");
                    continue;
                }
                let fail = if worst.margin < 0.0 { "FAIL" } else { "" };
                println!("  p{oi:<3} k={k:<3} covered {n_cov}/{}  worst sample: eye depth {:6.2} m  0.4% = {:6.1} mm  cover m{} (t_rule {:.2}, t_real {:.2}) push {:6.2} mm  lift {:5.2} mm (cap {:.2})  separation {:7.2} mm  margin {:8.2} mm {fail}", o.samples.len(), worst.w, PUSH_FRAC * worst.w * 1000.0, worst.plate, plates[worst.plate].t_rule, plates[worst.plate].t_real, worst.push, worst.lift, LIFT_MAX_THICK * o.t_rule, worst.sep, worst.margin);
            }
        }
    }
    if let Ok(out) = std::env::var("CENSUS_RECOLOR") {
        recolor(&s, &outlines, &plates, &f0, vp_h, &out);
    }
}

fn main() {
    for path in std::env::args().skip(1) {
        census(&path);
    }
}
```

## Step 16 - The cloud tools

- `add_lod` writes the LOD octree into a `.pb` once, offline, so no browser pays for it; `potree_import` turns a Potree 1.x tree into one kernel cloud; `mk_bunny_cloud` samples a mesh into a cloud with normals for the splat lane.

_Paste it._
**Create `examples/mk_bunny_cloud.rs`**

```rust
// Sample a mesh's surface into a point cloud WITH normals - the demo data for the splat
// lane's lambert shading (scans carry no normals; a sampled surface does). Area-weighted
// triangle sampling, barycentric position + interpolated vertex normal per point.
//
// cargo run --example mk_bunny_cloud --target x86_64-unknown-linux-gnu --release -- \
//     assets/pb/mesh_bunny_grey.pb assets/pb/bunny_cloud.pb 400000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let src = a.first().cloned().unwrap_or_else(|| "assets/pb/mesh_bunny_grey.pb".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/bunny_cloud.pb".into());
    let count: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(400_000);

    let bytes = std::fs::read(&src).expect("read src pb");
    let session = session_rust::Session::pb_loads(&bytes).expect("parse src pb");
    let mesh = session.order().into_iter().find_map(|g| match session.lookup.get(&g) {
        Some(session_rust::Geometry::Mesh(m)) => Some(m.clone()),
        _ => None,
    }).expect("no mesh in source pb");

    let rm = mesh.to_render();
    // cumulative triangle areas for area-weighted sampling
    let tri = |i: usize| {
        let f = [rm.indices[i * 3] as usize, rm.indices[i * 3 + 1] as usize, rm.indices[i * 3 + 2] as usize];
        f.map(|k| rm.vertices[k])
    };
    let ntri = rm.indices.len() / 3;
    let mut cum = Vec::with_capacity(ntri);
    let mut total = 0.0f64;
    for i in 0..ntri {
        let [a, b, c] = tri(i);
        let u = [b.position[0] as f64 - a.position[0] as f64, b.position[1] as f64 - a.position[1] as f64, b.position[2] as f64 - a.position[2] as f64];
        let v = [c.position[0] as f64 - a.position[0] as f64, c.position[1] as f64 - a.position[1] as f64, c.position[2] as f64 - a.position[2] as f64];
        let x = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        total += 0.5 * (x[0] * x[0] + x[1] * x[1] + x[2] * x[2]).sqrt();
        cum.push(total);
    }

    // deterministic LCG - no rand dependency, same cloud every run
    let mut state = 0x2545F491_4F6CDD1Du64;
    let mut rnd = move || { state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (state >> 33) as f64 / (1u64 << 31) as f64 };

    let mut coords = Vec::with_capacity(count * 3);
    let mut colors = Vec::with_capacity(count * 4);
    let mut normals = Vec::with_capacity(count * 3);
    for _ in 0..count {
        let r = rnd() * total;
        let t = cum.partition_point(|&c| c < r).min(ntri - 1);
        let [va, vb, vc] = tri(t);
        // uniform barycentric via the sqrt trick
        let (mut b1, mut b2) = (rnd(), rnd());
        if b1 + b2 > 1.0 { b1 = 1.0 - b1; b2 = 1.0 - b2; }
        let b0 = 1.0 - b1 - b2;
        for k in 0..3 {
            coords.push(b0 * va.position[k] as f64 + b1 * vb.position[k] as f64 + b2 * vc.position[k] as f64);
            normals.push(b0 * va.normal[k] as f64 + b1 * vb.normal[k] as f64 + b2 * vc.normal[k] as f64);
        }
        // near-white so the lambert term IS the picture
        colors.extend_from_slice(&[235, 230, 220, 255]);
    }

    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "bunny_cloud".to_string();
    let mut s = session_rust::Session::new("bunny_cloud");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {count} points with normals");
}
```

_Paste it._
**Create `examples/add_lod.rs`**

```rust
//! Write the LOD octree into a .pb: load, `build_lod`, save.
//!
//! The octree is built ONCE, offline, by whoever publishes the cloud - a browser paying ten
//! seconds per 14 M cloud to recompute what the file could have carried is the trade this
//! avoids. `build_lod` also REORDERS the points into octree order, so every node becomes one
//! contiguous byte range in the written file and a reader can fetch a node with one HTTP Range
//! request instead of downloading the whole cloud.
//!
//!   cargo run --example add_lod --target x86_64-unknown-linux-gnu --release -- <file.pb> [leaf]
//!
//! The root grid spacing is always derived from the cloud's own bounding box; each level halves
//! it. Leaf capacity defaults to 8192.

use std::time::Instant;

use session_rust::{Geometry, Session};

/// Root grid spacing from the cloud's own size: the longest bounding-box edge over 128, which
/// is Potree's rule of thumb. It has to scale with the cloud or the same number means a coarse
/// tree on one scan and a pointlessly deep one on the next.
fn auto_spacing(coords: &[f64]) -> f64 {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in coords.chunks_exact(3) {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let edge = (0..3).map(|k| hi[k] - lo[k]).fold(0.0, f64::max);
    (edge / 128.0).max(1.0e-6)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: add_lod <file.pb> [root_spacing] [leaf_capacity]");
    // The root grid is ALWAYS derived from the cloud - see `auto_spacing`. There is no manual
    // override on purpose: a per-cloud magic number is a number nobody can justify later, and
    // the derived one measured BETTER than the hand-tuned values it replaced (13.8 M scan at
    // the fit view: 492x fewer points, against 363x for the tuned tree).
    let leaf_capacity: usize = args.next().map_or(8192, |v| v.parse().expect("leaf_capacity"));

    let t = Instant::now();
    let bytes = std::fs::read(&path).expect("read");
    let mut session = Session::pb_loads(&bytes).expect("parse");
    println!("read           {:>7.0} ms  ({:.1} MB)", t.elapsed().as_secs_f64() * 1e3, bytes.len() as f64 / 1.048576e6);

    let guids: Vec<String> = session.order().to_vec();
    let mut built = 0usize;
    for g in guids {
        let Some(Geometry::PointCloud(rc)) = session.lookup.get(&g) else { continue };
        let mut pc = (**rc).clone();
        let n = pc.point_count();
        let spacing = auto_spacing(pc.coords());
        let t = Instant::now();
        pc.build_lod(spacing, leaf_capacity);
        println!("build_lod      {:>7.0} ms  ({} points -> {} nodes, spacing {:.0})", t.elapsed().as_secs_f64() * 1e3, n, pc.lod_node_count(), spacing);
        session.lookup.insert(g, Geometry::PointCloud(std::rc::Rc::new(pc)));
        built += 1;
    }
    if built == 0 {
        println!("no point clouds in {path} - nothing to do");
        return;
    }

    let t = Instant::now();
    let out = session.pb_dumps();
    std::fs::write(&path, &out).expect("write");
    println!("write          {:>7.0} ms  ({:.1} MB, {:+.1} MB)", t.elapsed().as_secs_f64() * 1e3,
        out.len() as f64 / 1.048576e6, (out.len() as f64 - bytes.len() as f64) / 1.048576e6);
}
```

_Paste it._
**Create `examples/potree_import.rs`**

```rust
// Import a Potree 1.x octree (POSITION_CARTESIAN + COLOR_PACKED + NORMAL_OCT16) into one
// kernel PointCloud .pb. Every point lives in exactly ONE node file, so the union of all
// r*.bin files IS the cloud; each node's positions are u32 * scale relative to the NODE's
// bounding box min, and a node's box comes from the root cube by walking the digits of its
// name (bit 4 = +x half, bit 2 = +y half, bit 1 = +z half).
//
// cargo run --example potree_import --target x86_64-unknown-linux-gnu --release -- \
//     assets/lion_src assets/pb/lion.pb 1000

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let dir = a.first().cloned().unwrap_or_else(|| "assets/lion_src".into());
    let out = a.get(1).cloned().unwrap_or_else(|| "assets/pb/lion.pb".into());
    let unit: f64 = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(1000.0); // metres -> mm

    let cloud: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{dir}/cloud.js")).expect("cloud.js")).expect("json");
    let bb = &cloud["boundingBox"];
    let root_min = [bb["lx"].as_f64().unwrap(), bb["ly"].as_f64().unwrap(), bb["lz"].as_f64().unwrap()];
    let root_size = bb["ux"].as_f64().unwrap() - root_min[0]; // potree root is a cube
    let scale = cloud["scale"].as_f64().unwrap();

    let mut coords = Vec::new();
    let mut colors = Vec::new();
    let mut normals = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "bin"))
        .collect();
    files.sort();
    for path in &files {
        let name = path.file_stem().unwrap().to_str().unwrap(); // "r", "r07", ...
        let (mut min, mut size) = (root_min, root_size);
        for d in name[1..].chars() {
            let i = d.to_digit(10).unwrap();
            size *= 0.5;
            if i & 0b100 != 0 { min[0] += size; }
            if i & 0b010 != 0 { min[1] += size; }
            if i & 0b001 != 0 { min[2] += size; }
        }
        let data = std::fs::read(path).unwrap();
        for rec in data.chunks_exact(18) {
            for k in 0..3 {
                let v = u32::from_le_bytes(rec[k * 4..k * 4 + 4].try_into().unwrap()) as f64;
                coords.push((v * scale + min[k]) * unit);
            }
            colors.extend_from_slice(&[rec[12] as i32, rec[13] as i32, rec[14] as i32, 255]);
            // potree NORMAL_OCT16: two UNSIGNED bytes mapped to [-1,1], octahedral unfold
            let u = rec[16] as f64 / 255.0 * 2.0 - 1.0;
            let v = rec[17] as f64 / 255.0 * 2.0 - 1.0;
            let z = 1.0 - u.abs() - v.abs();
            let (x, y) = if z < 0.0 {
                let s = |t: f64| if t < 0.0 { -1.0 } else { 1.0 };
                ((1.0 - v.abs()) * s(u), (1.0 - u.abs()) * s(v))
            } else { (u, v) };
            let l = (x * x + y * y + z * z).sqrt().max(1e-9);
            normals.extend_from_slice(&[x / l, y / l, z / l]);
        }
    }
    let n = coords.len() / 3;
    let mut pc = session_rust::PointCloud::from_coords(coords, colors, normals);
    pc.point_size = 3.0;
    pc.name = "lion_takanawa".to_string();
    let mut s = session_rust::Session::new("lion_takanawa");
    s.add_pointcloud(pc, None);
    s.pb_dump(&out);
    println!("wrote {out}: {n} points from {} nodes", files.len());
}
```

## Step 17 - Link the crate natively

- An example links the crate as an `rlib`, which a `cdylib`-only crate does not offer; `pollster` blocks on the async device setup natively. The target table stays at the END of the file: a target table placed mid-file silently swallows every `[dependencies]` entry after it, which is how prost and bytemuck once vanished from the wasm build.

_Type it._
**Find** in `Cargo.toml`:

```toml
crate-type = ["cdylib"]
```

**Replace with:**

```toml
crate-type = ["cdylib", "rlib"]  # rlib so examples/selftest.rs can link the crate
```

_Paste it._
**Find** in `Cargo.toml`:

```toml
wasm-opt = false
```

**Add below it:**

```toml

[[example]]
name = "selftest"
path = "examples/selftest.rs"

# Native-only: the headless verification harness (examples/selftest.rs). MUST stay at the end -
# a target table placed mid-file silently swallows every [dependencies] entry after it, which is
# how prost and bytemuck vanished from the wasm build.
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
pollster = "0.4"

[[example]]
name = "bench_load"
path = "examples/bench_load.rs"

[[example]]
name = "mk_facing_probe"
path = "examples/mk_facing_probe.rs"


[[example]]
name = "check_determinism"
path = "examples/check_determinism.rs"
```

## Run

```bash
cargo xtest
cargo run --release --target x86_64-unknown-linux-gnu --example mk_plate_outline -- plate.pb
cargo run --release --target x86_64-unknown-linux-gnu --example selftest -- plate.ppm plate.pb
VIEWER_PICK=450,350 cargo run --release --target x86_64-unknown-linux-gnu --example selftest -- plate.ppm plate.pb
cargo run --release --target x86_64-unknown-linux-gnu --example bench_frame -- plate.pb
trunk serve
```

- `selftest` logs the adapter, the camera and `headless frame: 8 draws, 9 objects, 900x700`, then prints `non-background pixels: 105270 (16.7%)`: the same count on two runs (Intel RPL-S iGPU, Vulkan, 2026-09-04), and the frame is in `plate.ppm`. With `VIEWER_PICK` it adds `pick: (450,350) doc='plate.pb' guid=... row=7`.
- `bench_frame` prints `still: 1.36 ms/frame` and `moving: 1.32 ms/frame` on that iGPU (two runs: 1.36 / 1.35 and 1.32 / 1.32).
- `cargo xtest` runs the crate's tests natively; `.cargo/config.toml` defined the alias in lesson 01.
- Open `http://127.0.0.1:8770/?perf=1&spin=1`: a `#perf` line sits top-left, `f<frame> gap <ms> enc <ms> ms heap <MB>`, the console logs `perf: ... fps | ... ms | ... draws | ... objects | heap ... MB` once a second, and every `appended:` line ends with `heap N MB`.
- The repo's `docs/_gate.sh` is these commands over the local scene plus the plate probe counted by `docs/_count_colors.py`; it prints `gate OK` or the first number that failed.

## Why

- Every number in these lessons comes through `render_offscreen`, because a browser screenshot cannot be diffed byte for byte and a browser timer measures the compositor as much as the frame.
- The harness runs the page's own `encode_frame` and pipelines through `Gpu::new_headless`; a second code path would measure something else.
- `bench_frames` drains the GPU after every frame so the median is the frame's cost, not the queue depth; the first call is discarded because pipeline compilation lands in it.
- The heap is logged as a high-water mark because `WebAssembly.Memory` never shrinks; what the page holds after the last file is what it holds until reload, and the number goes into the load log where the file that cost it is named.
- The perf line is a DOM element, not a console line: it survives a busy console, shows in a screenshot, and `index.html` stays untouched because `perf_line` creates the element on first use.
- `perf` and `spin` are `View` knobs so the page and the harness read them the same way, once; `spin` keeps the camera moving without a hand on the mouse, which is what a moving-camera benchmark needs.
- The native dependency table sits at the end of `Cargo.toml` because a target table mid-file swallows every `[dependencies]` line after it; the wasm build lost prost and bytemuck that way once.
- The probe scenes turn a rendering bug into a count the gate can refuse: `docs/_gate.sh` fails when more than 4 magenta pixels show above a plate, and the count comes from a PPM the harness wrote, not from a screenshot.
