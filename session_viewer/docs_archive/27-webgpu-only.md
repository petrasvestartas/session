# 27 WebGPU-only — unlock storage buffers (Phase 4 opens)

Everything so far ran under **WebGL2 fallback limits**, which set
`max_storage_buffers_per_shader_stage = 0` — storage buffers are *forbidden*. That's the one feature
the next phase needs: instancing (29), the GPU arena (30), and cylinder-lines (31) all stream
thousands of objects through a **storage buffer** the vertex shader indexes by `instance_index`. This
lesson locks in **browser-only + WebGPU-only**, flips three switches to turn storage buffers on, and
adds a graceful "WebGPU required" screen for browsers that can't run it.

## Why this trade

```
current (downlevel_webgl2_defaults)     after (Limits::default())
────────────────────────────────────    ──────────────────────────────
max_storage_buffers_per_stage = 0    →   = 8   (instancing, arena, lines)
max_compute_* = 0                    →   > 0   (future GPU culling, 76)
runs on WebGL2 fallback browsers     →   needs a real WebGPU browser
```

WebGPU ships in Chrome, Edge, Firefox, and Safari 18+, so "needs WebGPU" is a small ask for a 2026 CAD tool —
and it buys the entire scalable-rendering half of the roadmap. We drop the WebGL fallback rather than
carry two code paths forever (locked decision, `reference_webgpu_cad_caveats`).

## Files we touch

```
Cargo.toml                    # drop wgpu's "webgl" feature
src/engine/gpu.rs             # WebGPU-only backend + full limits + error logging
index.html                   # "WebGPU required" overlay + a navigator.gpu check
```

## Step 1 — drop the WebGL feature: `Cargo.toml`

The `webgl` feature pulls in wgpu's WebGL2 backend — remove it so the wasm build targets WebGPU only:

```toml
wgpu = { version = "29.0" }        # was: features = ["webgl"]
```

## Step 2 — WebGPU-only device: `src/engine/gpu.rs`

Three edits in `new()`. First, the **backend**: drop `| Backends::GL`, target-gated so native builds
(the `selftest` example) keep working — browser gets WebGPU, native gets the real platform backends:

```rust
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: if cfg!(target_arch = "wasm32") {
                wgpu::Backends::BROWSER_WEBGPU        // was BROWSER_WEBGPU | GL
            } else {
                wgpu::Backends::PRIMARY               // Vulkan/Metal/DX12 for native selftest
            },
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
```

Second, the **limits** — the line that unlocks storage buffers. Swap the WebGL2 downlevel defaults for
the full WebGPU defaults:

```rust
                required_limits: wgpu::Limits::default(),   // was downlevel_webgl2_defaults()
```

Third, right after `let (device, queue) = adapter.request_device(…).await?;`, register an
uncaptured-error logger so GPU validation errors show up in the console instead of vanishing (takes an
**`Arc`**, not a `Box`):

```rust
        device.on_uncaptured_error(std::sync::Arc::new(|e| {
            log::error!("wgpu uncaptured error: {e}");
        }));
```

`request_adapter(…).await?` already turns "no WebGPU adapter" into an error that propagates out of
`new()` — the next step makes that failure legible to the user.

## Step 3 — "WebGPU required" screen: `index.html`

With no WebGPU at all, `Gpu::new()` fails somewhere deep and the canvas just stays black. Catch it up
front with one script — no wasm needed, so it works even when nothing else loads. It injects the
message **only when WebGPU is missing**, so nothing sits dead in the DOM on the common path. Style
goes in the existing `<style>` block in the head:

```css
    #no-webgpu { position:fixed; inset:0; background:#111; color:#eee;
                 font:1rem system-ui; text-align:center; padding-top:40vh; }
```

…and the check inside `<body>`, after the canvas:

```html
  <script>
    if (!navigator.gpu) {
      document.body.insertAdjacentHTML("beforeend",
  '<div id="no-webgpu">WebGPU required — use a recent Chrome, Edge, Firefox, or Safari 18+.</div>');
    }
  </script>
```

Plain reading: *if there's no WebGPU, append the message*. `insertAdjacentHTML("beforeend", …)`
*appends* — never wipes the body, so the canvas and trunk's wasm loader stay intact. The div is
`position:fixed` and added last, so it paints over the canvas with no `z-index` needed.

## Step 4 — run and confirm the unlock

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

The scene looks exactly the same — this lesson changes capabilities, not pixels. Confirm the unlock
two ways:

- Log the limit once — in `new()`, right after the `on_uncaptured_error` line from Step 2, add
  `log::info!("storage buffers/stage: {}", device.limits().max_storage_buffers_per_shader_stage);`
  — the console now prints **8**, not 0 (the downlevel limits clamped it). Delete the line after
  checking, or keep it as a startup sanity print.
- Open the page in a browser with WebGPU disabled (or an old one) — the **"WebGPU required"** overlay
  appears instead of a black canvas.

That positive storage-buffer limit is the green light for lesson 29 onward.

## Recap

```
Ch 1–26: ran under WebGL2 downlevel limits → storage buffers = 0, the wall Phase 4 hits.
Ch 27:   commit to browser + WebGPU only. Drop wgpu's "webgl" feature; set backend to
         BROWSER_WEBGPU (native keeps PRIMARY via cfg); swap downlevel_webgl2_defaults() for
         Limits::default() — storage buffers now = 8; log uncaptured GPU errors; and show a
         "WebGPU required" overlay when navigator.gpu is absent. Same pixels, new powers.
```

Edited: `Cargo.toml` (drop `webgl`), `engine/gpu.rs` (`BROWSER_WEBGPU`/`PRIMARY` cfg,
`Limits::default()`, `on_uncaptured_error`), `index.html` (overlay + `navigator.gpu` check).

## Next

`28-perf.md` — before collapsing draw calls, we need to *see* them. A tiny `engine/performance.rs` counts
frame time, fps, and draw calls (≈3 today: grid, meshes, edges), logging once a second — so when
instancing and batching land in 29–30, you watch the draw count fall instead of taking it on faith.
