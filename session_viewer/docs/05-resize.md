# 04 Resize

Stop the triangle from stretching. Make it fill the window crisply at any size.

In Chapter 3 the triangle looked squashed — wide and flat instead of evenly
shaped. Nothing is wrong with the shader; the problem is the **canvas size**. This
chapter fixes it with one small helper, and along the way explains the three
different "sizes" a web canvas has. No new files — we only edit `lib.rs`.


## Why it stretches (read this first)

A browser canvas has **two independent sizes**, and a third number that links them:

- **CSS size** — how big the canvas *looks* on the page. Ours is `100vw × 100vh`
  (full window), set in `index.html`.
- **Drawing-buffer size** — how many *pixels* the GPU actually renders into
  (`canvas.width × canvas.height`). This defaults to **300 × 150** and does **not**
  change just because the CSS size does.
- **devicePixelRatio (DPR)** — how many physical screen pixels equal one CSS pixel.
  `1.0` on a normal monitor, `2.0` on a Retina/4K laptop, `1.25`/`1.5` at OS zoom.

The stretch is a **mismatch of aspect ratios**: the GPU draws into a 300 × 150
buffer (2:1, wide) and the browser then stretches that image to fill a 16:9 window.
Anything drawn — including our triangle — gets distorted by the same factor.

```
drawing buffer  300 × 150  (2:1)          what the GPU renders
        │  stretched by the browser to…
        ▼
CSS size       1920 × 1080 (16:9)         what you see  →  squashed
```

The fix: make the drawing buffer the **same size and shape** as the display area.
And to also be sharp on high-DPI screens, multiply by DPR so one buffer pixel maps
to one *physical* pixel.

```
want:  buffer = clientWidth × DPR  ,  clientHeight × DPR
```


## The plumbing we already have

Resizing the GPU surface already exists from Chapter 2 — `gpu.rs` has:

```rust
pub fn resize(&mut self, width: u32, height: u32) {
    if width > 0 && height > 0 {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);   // <- also sizes the canvas buffer
    }
}
```

On the web, `surface.configure` sets the canvas drawing-buffer to `width × height`
for us. So all we're missing is: **compute the right width/height and call `resize`
with them.** That's this whole chapter.


## Step 1 — a helper that reads the desired size

Add this free function to `src/lib.rs` (anywhere at the top level — e.g. just above
`run_web`). It asks the browser three questions — DPR, canvas client width, canvas
client height — and returns the pixel size the buffer *should* be.

```rust
/// The real pixel size the canvas's drawing buffer should have: its CSS display size
/// (`clientWidth`/`clientHeight`, in CSS px) times the device-pixel-ratio. wgpu's
/// `surface.configure` then sizes the canvas backing store to match, so one buffer
/// pixel maps to one physical screen pixel — the aspect ratio matches (no stretch)
/// and the resolution matches (no blur). `None` before the canvas exists or while it
/// reports zero size.
fn desired_canvas_size() -> Option<(u32, u32)> {
    use wasm_bindgen::JsCast;
    let win = web_sys::window()?;
    let dpr = win.device_pixel_ratio();
    let canvas = win.document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    let w = (canvas.client_width()  as f64 * dpr).round() as u32;
    let h = (canvas.client_height() as f64 * dpr).round() as u32;
    (w > 0 && h > 0).then_some((w, h))
}
```

- `client_width`/`client_height` are the **CSS** size in CSS pixels.
- `device_pixel_ratio()` is the DPR.
- We return `None` (rather than `0`) before the canvas is laid out, so the caller can
  simply skip the resize that frame.


## Step 2 — set the size when the viewer starts

`user_event` runs once, right after `State` finishes initialising. Use the helper for
the first sizing instead of winit's `inner_size` (which is unreliable for an adopted
canvas). Replace the body of `user_event`:

```rust
fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut state: State) {
    let (w, h) = desired_canvas_size()
        .unwrap_or_else(|| { let s = state.window.inner_size(); (s.width, s.height) });
    state.resize(w, h);
    state.window.request_redraw();
    self.state = Some(state);
}
```


## Step 3 — keep it correct every frame

The window can change size at any moment — the user drags the corner, zooms the page,
or drags the tab to a screen with a different DPR. The simplest robust approach: check
the desired size **each frame** before drawing, and only reconfigure when it actually
changed (reconfiguring every frame would be wasteful). Edit the `RedrawRequested` arm
in `window_event`:

```rust
WindowEvent::RedrawRequested => {
    // Before drawing, make the GPU surface match the canvas's real pixel size.
    // Cheap check every frame; reconfigure only on a genuine change.
    if let Some((w, h)) = desired_canvas_size() {
        if (w, h) != (state.gpu.config.width, state.gpu.config.height) {
            state.resize(w, h);
        }
    }
    if let Err(e) = state.render() { log::error!("render: {e}"); }
}
```

> Why poll instead of using `WindowEvent::Resized`? For a canvas embedded in a page,
> the `Resized` event is delivered inconsistently across browsers/winit versions, and
> it won't fire when only the **DPR** changes. A once-per-frame size check is a couple
> of cheap DOM reads and is always correct. (A fancier version uses a JS
> `ResizeObserver`; we keep it simple here.)


## Step 4 — run it

```bash
cd session_viewer && trunk serve   # http://localhost:8770  (open in Chrome/Edge)
```

The triangle is now **evenly shaped** and stays that way as you resize the window. On
a high-DPI screen the edges are crisp, not soft. Try dragging the window between two
monitors with different scaling — it should re-sharpen on the next frame.


## What changed vs Chapter 3 (recap)

```
Chapter 3:  buffer stuck at 300×150  →  browser stretches it to fill  →  squashed
Chapter 4:  buffer = clientSize × DPR →  1 buffer px = 1 screen px      →  crisp & correct
```

Edited: `lib.rs` only — one new helper `desired_canvas_size`, used in `user_event`
(initial) and `window_event` (every frame).
Untouched: `state.rs`, `gpu.rs`, the shader, the pipeline.


## Compare to the archive

`session_viewer_archive` takes the same "configure the surface to a chosen size"
route, but reads the size from winit's reported `inner_size()` on `Resized` events
(see its `user_event`/`window_event`). It notes in a comment that an adopted canvas's
backing store follows its CSS size and that winit ignores `request_inner_size` there —
which is exactly the gap our per-frame `desired_canvas_size()` closes, and additionally
handles DPR for sharpness.


## Next

A clear, correctly-sized triangle is the last "hello world" milestone. From here the
viewer starts becoming a CAD tool — beginning with a **camera** so we can look at the
scene from any angle instead of fixed clip-space coordinates. See the curriculum in
`viewer_sections/` for the full path.
