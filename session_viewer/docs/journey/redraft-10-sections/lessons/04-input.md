# 1.4 · Commands that change the view

**Goal:** Use a command to switch backgrounds without changing the triangle. · **Steps:** 1 · **Source:** [`State`](../../../../src/state.rs) · [`engine/gpu/frame.rs`](../../../../src/engine/gpu/frame.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CSS Viewport (W × H × DPR)</text>
  <rect x="56" y="46" width="260" height="114" fill="#f8fafc" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <line x1="56" y1="42" x2="316" y2="42" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arrB)"/>
  <line x1="48" y1="46" x2="48" y2="160" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="64" text-anchor="middle" font-family="monospace" font-size="10" fill="#2563eb">width = clientWidth * dpr</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="108" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">(0, 0) ────────► +X (px)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#dc2626">▼ +Y (px)   clamped 1..max_2d</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">RenderPass LoadOp::Clear(r,g,b,1)</text>
  <rect x="440" y="48" width="240" height="110" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="440" y1="76" x2="680" y2="76" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="440" y1="104" x2="680" y2="104" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="440" y1="132" x2="680" y2="132" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="500" y1="48" x2="500" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="560" y1="48" x2="560" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <line x1="620" y1="48" x2="620" y2="158" stroke="#e2e8f0" stroke-width="1"/>
  <circle cx="470" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <circle cx="530" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <circle cx="590" cy="62" r="5" fill="#facc15" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="122" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::Color { r, g, b, a: 1.0 }</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Viewer Swapchain Surface</text>
  <rect x="802" y="46" width="124" height="112" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="864" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">rgb(1.0)</text>
  <rect x="942" y="46" width="124" height="112" rx="4" fill="#e2e8f0" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1004" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">rgb(0.9)</text>
</svg>
</div>

<a id="checkpoint-04-input"></a>

## 1. `src/background.rs`

Create the small piece of application state. It knows colours, but knows nothing about HTML or GPU resources.

Create the file and type:

```rust
#[derive(Default)]
pub struct Background {
    light: bool,
}

impl Background {
    pub fn toggle(&mut self) {
        self.light = !self.light;
    }

    pub fn rgb(&self) -> [f64; 3] {
        if self.light {
            [0.9, 0.9, 0.9]
        } else {
            [1.0, 1.0, 1.0]
        }
    }
}
```

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU CommandEncoder Records GPU Commands</text>
  <rect x="54" y="50" width="264" height="102" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::CommandEncoder</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">1. begin_render_pass (3D Scene)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">2. begin_render_pass (Overlay/UI)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">3. encoder.finish()</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pass 1 (LoadOp::Clear) -&gt; Pass 2 (LoadOp::Load)</text>
  <rect x="416" y="52" width="128" height="96" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="480" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Clear</text>
  <polygon points="452,132 480,88 508,132" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="52" width="128" height="96" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Load</text>
  <polygon points="612,126 640,86 668,126" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <rect x="590" y="130" width="100" height="12" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.submit([cmd_buf]) -&gt; GPU Execution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,132 934,58 984,132" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="136" width="224" height="18" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="149" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Composited Frame Ready</text>
</svg>
</div>

## 2. `src/lib.rs`

Register the state module so both the browser and the renderer can use it.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
mod browser;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod background;
#[cfg(target_arch = "wasm32")]
mod browser;
```

## 3. `src/renderer.rs`

Borrow Background in draw and read its RGB value. Clear the render target with that RGB value.

**Locate this existing block:**

```rust
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn draw(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
    ) {
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("canvas"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
```

## 4. `src/browser.rs`

Import the background state. Own the background and toggle it when the dock submits Background. Pass the background into presentation and update the status message. Draw the scene with the current background.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view);
    panel.draw(renderer, &view);
```

Replace it with:

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view, background);
    panel.draw(renderer, &view);
```

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Type Help in the command field and press Enter.");
    Ok(())
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Change the background. The triangle keeps its shape and colour.");
    Ok(())
}

fn present(
    surface: &wgpu::Surface<'_>,
    renderer: &Renderer,
    background: &Background,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

**Locate this block:**

```rust
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &["Help"],
    );
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let click =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let _line = match panel.update(Some(&event), &input_canvas) {
                Ok(line) => line,
                Err(error) => {
                    report(&format!("Cannot read command: {error:?}"));
                    return;
                }
            };
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
                    "Cannot lay out commands: {error:?}"
                ));
                return;
            }
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        });
```

Replace it with:

```rust
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &["Help", "Background"],
    );
    panel.update(None, &canvas)?;
    let mut background = Background::default();
    present(&surface, &renderer, &background, &mut panel)?;
    let input_canvas = canvas.clone();
    let click =
        Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let line = match panel.update(Some(&event), &input_canvas) {
                Ok(line) => line,
                Err(error) => {
                    report(&format!("Cannot read command: {error:?}"));
                    return;
                }
            };
            if line.as_deref() == Some("background") {
                background.toggle();
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
                    "Cannot lay out commands: {error:?}"
                ));
                return;
            }
            if let Err(error) = present(&surface, &renderer, &background, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        });
```

**Locate this block:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

Replace it with:

```rust
use crate::background::Background;
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

## Build & Run

![Recorded visual output for Lesson 1.4 (04-input)](../../../screenshots/journey/04-input-browser.png)

```sh
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 1.4
npm --prefix ../session_tests run course -- save 1.4
```

Continue with [2.1 · Buffers that describe a shared mesh](../../consolidation/05-vertices.md).
