# 28 Perf counter — watch the draw calls before you fight them

The next two lessons **collapse draw calls**: instancing (29) turns many mesh draws into one, the GPU
arena (30) fuses buffers. But "it's faster now" is worthless without a number — you can't see a win
you don't measure. So before optimizing, we add the gauge: frame time, fps, and **how many draw
calls** the frame actually issued. Console-first today (graduates to an on-screen HUD in lesson 72),
logging once a second.

## Why

Every frame currently issues a draw call for **each** thing: background, grid, and one per mesh and
edge-set. With three meshes:

```
background(1) + grid(1) + meshes(3) + edges(3)  =  8 draws  for  3 objects
```

That 8 is the number instancing and batching will drive down — you want to *see* it fall, not take it
on faith. The counter logs a line like:

```
perf: 60.0 fps | 16.67 ms | 8 draws | 3 objects
```

Two things keep it honest: a **rolling average** of frame time (one frame is too jittery to read as
fps), and a **`draws += 1` beside every draw call** so the count can't drift out of sync with the code
— when a draw call disappears in lesson 29, the number drops by itself.

## Files we touch

```
Cargo.toml                    # add "Performance" to web-sys features (for the timer)
src/engine/performance.rs     # NEW — Performance: frame timer + counters, logs once/sec
src/engine/mod.rs             # register the module
src/engine/gpu.rs             # own a Performance, count draws, report at frame end
```

## Step 1 — the timer needs `Performance`: `Cargo.toml`

The browser clock, `window.performance.now()`, sits behind a web-sys feature. Add `"Performance"` to
the existing `web-sys` features list:

```toml
web-sys = { version = "0.3", features = [
    "Document", "Window", "Element", "HtmlCanvasElement", "EventTarget", "Event",
    "CanvasRenderingContext2d", "ImageData", "Location", "Performance",   # ← add
] }
```

## Step 2 — the counter: `src/engine/performance.rs`

A tiny struct that remembers the last frame's timestamp, smooths frame time, and logs once a second.
The clock is target-gated so the native `selftest` build still compiles — browser uses
`performance.now()`, native falls back to the system clock:

```rust
//! Frame-time + draw-call counter (ARCHITECTURE.md §9). Console-first; the HUD reads it in ch 52.

pub struct Performance {
    prev_frame: f64,   // ms timestamp of the previous frame
    last_log: f64,     // ms timestamp of the last console line
    frame_ms: f64,     // smoothed frame time
}

impl Performance {
    pub fn new() -> Self {
        let t = now_ms();
        Self { prev_frame: t, last_log: t, frame_ms: 0.0 }
    }

    /// Call once at the end of every frame with the counts gathered during it.
    pub fn frame(&mut self, draws: u32, objects: u32) {
        let t = now_ms();
        let dt = t - self.prev_frame;
        self.prev_frame = t;
        // exponential moving average — one raw frame is too jittery to show as fps
        self.frame_ms = if self.frame_ms == 0.0 { dt } else { self.frame_ms * 0.9 + dt * 0.1 };

        if t - self.last_log >= 1000.0 {
            let fps = if self.frame_ms > 0.0 { 1000.0 / self.frame_ms } else { 0.0 };
            log::info!("perf: {:.1} fps | {:.2} ms | {} draws | {} objects",
                fps, self.frame_ms, draws, objects);
            self.last_log = t;
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

#[cfg(not(target_arch = "wasm32"))]
fn now_ms() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64() * 1000.0
}
```

## Step 3 — register it: `src/engine/mod.rs`

```rust
pub mod performance;      // next to `pub mod gpu;` / `pub mod pipelines;`
```

## Step 4 — count and report: `src/engine/gpu.rs`

Give `Gpu` a `Performance`: add the import and field, and build it in `new()`:

```rust
use crate::engine::performance::Performance;   // near the other `use crate::engine::…` lines
```

```rust
    pub performance: Performance,   // in `pub struct Gpu { … }`
```

```rust
        // in new()'s returned `Ok(Self { … })`
        performance: Performance::new(),
```

In `clear()`, tally a **local** `draws` counter beside each draw call (a local dodges any borrow fight
with the render pass), and report once the pass ends:

```rust
        let mut draws = 0u32;
        {
            let mut pass = encoder.begin_render_pass(/* … unchanged … */);

            pass.set_pipeline(&self.pipelines.background);
            pass.draw(0..3, 0..1);                    draws += 1;

            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.draw(0..50, 0..1);                   draws += 1;

            pass.set_pipeline(&self.pipelines.triangle);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            pass.set_bind_group(1, &self.time_bind_group, &[]);
            for mesh in &mut self.meshes {
                let gm = mesh.gpu_mesh(&self.device);
                pass.set_vertex_buffer(0, gm.vbo.slice(..));
                pass.set_index_buffer(gm.ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..gm.index_count, 0, 0..1);   draws += 1;
            }

            pass.set_pipeline(&self.pipelines.edges);
            pass.set_bind_group(0, &self.mvp_bind_group, &[]);
            for (vbo, count) in &self.edge_buffers {
                pass.set_vertex_buffer(0, vbo.slice(..));
                pass.draw(0..*count, 0..1);           draws += 1;
            }
        }   // pass drops here

        let objects = self.meshes.len() as u32;
        self.queue.submit([encoder.finish()]);
        output.present();
        self.performance.frame(draws, objects);
        Ok(())
```

`draws += 1` sits beside each real draw call, so the count is always the truth — nothing to keep in
sync by hand.

## Step 5 — run

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Open the console (F12). Once a second you'll see the perf line — around **60 fps / 16.7 ms**, **8
draws**, **3 objects**. Orbit and the ms/fps react to load. Note the mismatch: **8 draw calls for 3
objects** — the waste lesson 29 fixes, when three mesh draws become one instanced call and you watch
this number fall.

## Recap

```
Ch 27: unlocked storage buffers — the tool the next lessons use to cut draw calls.
Ch 28: add the gauge first. A Performance struct times each frame (exponential-average ms → fps) and takes
       a draws count tallied `+= 1` next to every draw call, logging once a second. Today: ~8 draws
       for 3 objects. The point is to watch that 8 drop as instancing (29) and batching (30) land —
       measured, not assumed. Console now; on-screen HUD in lesson 72.
```

Edited: `Cargo.toml` (web-sys `"Performance"`), `engine/performance.rs` (new `Performance`),
`engine/mod.rs` (`pub mod performance`), `engine/gpu.rs` (`Performance` field + `new()` +
per-draw tally + `frame()` at end).

## Next

`29-instancing.md` — one mesh, many transforms. An `Instance { model, color, flags }` row per copy in
a **storage buffer** (unlocked in 27), read by `@builtin(instance_index)`; a 10×10 field of
dodecahedra drawn with **one** `draw_indexed(.., 0..100)` — today's counter shows 100 objects at a
fraction of the draws.
