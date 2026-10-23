# 1.3 · Shaders and the command dock

**Section 1: WebGPU Foundations, Shaders, and the Command Dock** · **Checkpoints:** `03-triangle` → `03d-input` (25 steps) · **Production source:** [`lib.rs`](../../../src/lib.rs) · [`engine/pipelines/mod.rs`](../../../src/engine/pipelines/mod.rs) · [`command_dock/mod.rs`](../../../src/command_dock/mod.rs) · [`view.rs`](../../../src/command_dock/view.rs) · [`theme.rs`](../../../src/command_dock/theme.rs) · [`app/ui/command_line.rs`](../../../src/app/ui/command_line.rs) · [`app/ui/mod.rs`](../../../src/app/ui/mod.rs) · [`engine/gpu/ui.rs`](../../../src/engine/gpu/ui.rs)

> **Goal:** Return accepted commands while keeping editing inside Panel.

## 1. Visual WebGPU Schematic

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="220" viewBox="0 0 840 220" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="216" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="20" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="135" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">1. &lt;canvas&gt; (px)</text>
  <rect x="45" y="56" width="180" height="110" rx="6" fill="url(#dots)" stroke="#111111" stroke-width="1.5"/>
  <circle cx="45" cy="56" r="3.5" fill="#111111"/>
  <text x="53" y="70" font-family="monospace" font-size="10" fill="#111111">(0,0)</text>
  <text x="175" y="160" font-family="monospace" font-size="10" fill="#111111">(W,H)</text>
  <rect x="65" y="140" width="140" height="16" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text x="135" y="151" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">&gt; Command Dock</text>
  <text x="135" y="186" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">DOM Surface + Events</text>
  <line x1="250" y1="110" x2="298" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="304" y="20" width="230" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="419" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">2. CommandEncoder</text>
  <rect x="326" y="58" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="77" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 1: LoadOp::Clear</text>
  <text x="419" y="93" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Clear + Scene Triangle</text>
  <line x1="419" y1="102" x2="419" y2="118" stroke="#111111" stroke-width="1.5" marker-end="url(#arr)"/>
  <rect x="326" y="122" width="186" height="44" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="419" y="141" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Pass 2: LoadOp::Load</text>
  <text x="419" y="157" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep Pixels + Dock UI</text>
  <text x="419" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">queue.submit()</text>
  <line x1="534" y1="110" x2="582" y2="110" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="588" y="20" width="232" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text x="704" y="42" text-anchor="middle" font-family="monospace" font-size="12" font-weight="700" fill="#111111">3. Clip Space (-1..+1)</text>
  <rect x="644" y="56" width="120" height="108" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <line x1="644" y1="110" x2="764" y2="110" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="704" y1="56" x2="704" y2="164" stroke="#111111" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="704,68 662,148 746,148" fill="url(#hatch)" stroke="#111111" stroke-width="2"/>
  <circle cx="704" cy="68" r="3.5" fill="#111111"/>
  <circle cx="662" cy="148" r="3.5" fill="#111111"/>
  <circle cx="746" cy="148" r="3.5" fill="#111111"/>
  <text x="712" y="69" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,+1)</text>
  <text x="606" y="152" font-family="monospace" font-size="9" fill="#111111">(-1,-1)</text>
  <text x="767" y="152" font-family="monospace" font-size="9" fill="#111111">(+1,-1)</text>
  <text x="704" y="186" text-anchor="middle" font-family="monospace" font-size="10" fill="#111111">@vertex -&gt; @fragment</text>
</svg>

| Layer | Construct | Role |
| :--- | :--- | :--- |
| **HTML** | `<canvas id="canvas">` | Full-window pixel surface (`width/height: 100%`). |
| **Rust (`wgpu`)** | `Pass 1 (Clear)` → `Pass 2 (Load)` | Pass 1 clears and draws 3D scene; Pass 2 overlays Command Dock UI. |
| **WGSL** | `@vertex` & `@fragment` | Outputs clip coordinates `(-1..+1)` and RGBA pixel color. |

![Recorded visual output for Lesson 1.3 (03d-input)](../../screenshots/journey/03d-input-browser.png)

---

## 2. Complete Code Overview (All Steps Open)

<a id="checkpoint-03-triangle"></a>

### Step 1 of 25 · Give the GPU three corners (`03-triangle`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/triangle.wgsl, src/re…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">fragment colour</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">canvas</text>
</svg>

Draw one pink triangle inside the white GPU frame. The shader supplies three corners; the renderer keeps one pipeline and records its draw.

The vertex shader places corners in clip space, where x and y near −1 and +1 reach the image edges. The fragment shader supplies colour. `0..3` draws three vertices; `0..1` draws one instance.

> **Check:** *If you change the top vertex, which part of the picture changes, and which stays the same?* — The triangle silhouette changes because its corner positions changed. The surrounding white stays the same: the render pass clears the background before drawing the triangle.

### 1. `src/triangle.wgsl`

Create GPU vertex and fragment functions; the vertex function chooses a corner and the fragment function returns pink.

Create the file and type:

```wgsl
--8<-- "journey/code/03-triangle-01.wgsl"
```

### 2. `src/renderer.rs`

Create and retain the triangle shader pipeline. Bind the pipeline and draw its three vertices.

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { device, queue, pipeline }
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
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
                            r: 1.0, g: 1.0, b: 1.0, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
    }
}
```

### 3. `src/browser.rs`

Pass the surface format to the renderer; a pipeline must agree with the texture it draws into.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03-triangle-window-1.rs"
```

**Result after `03-triangle`:** A pink triangle appears on white. Change its top vertex in the shader, save, and check that only the triangle’s shape changes. Restore the vertex.

---

<a id="checkpoint-03a-fonts"></a>

### Step 2 of 25 · Prepare the command fonts and painter (`03a-fonts`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Cargo.toml, src/panel.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">egui context</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">GPU painter</text>
</svg>

Prepare one owner for the command drawing. It keeps an `egui::Context` and a GPU painter. The browser constructs this owner now; the triangle stays unchanged until we connect drawing.

`include_bytes!` embeds the three Noto font files in the program. Their static lifetime lets the font definitions retain them. `Arc` gives egui shared ownership of the font data; both families list the same font names. The `fonts` helper returns that configuration; `new` installs it, chooses the light theme and creates a painter using the existing device.

> **Check:** *Why can the font configuration keep the embedded byte slices?* — include_bytes! supplies bytes with a static lifetime, so FontData can retain those slices. Arc lets egui share the same font data allocation; both families refer to its registered name.

### 1. `Cargo.toml`

Add egui and its GPU painter.

Edit the existing block to match this code; keep unchanged lines:

```toml
--8<-- "journey/code/03a-fonts-01.toml"
```

### 2. `src/panel.rs`

Keep the context and painter together.

Create the file and type:

```rust
--8<-- "journey/code/03a-fonts-02.rs"
```

### 3. `src/browser.rs`

Create the painter owner before presenting the scene.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03a-fonts-03.rs"
```

### 4. `src/lib.rs`

Register the browser-only drawing modules.

After that block, type:

```rust
--8<-- "journey/code/03a-fonts-04.rs" skip=20
```

### 5. `src/command_dock/theme.rs`

Configure Noto fonts and the light theme.

Create the file and type:

```rust
--8<-- "journey/code/03a-fonts-05.rs"
```

### 6. `src/command_dock/mod.rs`

Register the shared styling module.

Create the file and type:

```rust
--8<-- "journey/code/03a-fonts-06.rs"
```

**Result after `03a-fonts`:** The triangle stays visible in the full window. The font and painter owner is constructed, but no interface drawing is connected yet.

---

<a id="checkpoint-03a-paint"></a>

### Step 3 of 25 · Paint command text over the scene (`03a-paint`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/panel.rs, src/browser…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">GPU upload</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">present</text>
</svg>

Draw the Command label after the triangle and before presenting the image. `RawInput` gives egui the available canvas size. `run_ui` calls our layout closure and returns shapes plus changed font-atlas pixels.

Upload those pixels, turn the shapes into triangle jobs and update their GPU buffers. The second render pass uses `Load`, so it keeps the scene pixels already drawn. The panel uses a white frame and the production margins.

> **Check:** *Why does the interface pass use Load?* — The triangle is already in the target image. Load keeps those pixels while the interface draws over them; another Clear would erase the scene.

### 1. `src/panel.rs`

Lay out the label, upload its font atlas and draw over the scene.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03a-paint-01.rs"
```

### 2. `src/browser.rs`

Create the panel and pass it into presentation. Draw the panel over the scene before presenting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        ..Default::default()
    });
    renderer.draw(&view);
    frame.present();
    Ok(())
}
```

Replace it with:

```rust
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

**Locate this block:**

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let _panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer)?;
    report("Three corners became a triangle.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
```

Replace it with:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    report("Our command line is drawn. Input comes next.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    report("Our command line is drawn. Input comes next.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

**Result after `03a-paint`:** Command: appears at the bottom of the full-window canvas. The triangle remains unchanged above it. Keyboard input is not connected yet.

---

<a id="checkpoint-03a-panel"></a>

### Step 4 of 25 · Lay out the command field (`03a-panel`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/panel.rs, src/command…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">local String</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">field response</text>
</svg>

Add the command field beside its label. Keep panel sizing, shared spacing and field styling in the view module so later command behaviour can reuse them.

The field receives `&mut String`: egui borrows the text for this draw instead of owning a separate copy. It returns a `Response` describing the widget. The current page still supplies no keyboard events, so drawing the field does not connect typing yet.

> **Check:** *Why does the field receive &mut String?* — The Panel owns the text. A mutable borrow lets the widget edit that same value during a draw without moving it or keeping a second copy.

### 1. `src/panel.rs`

Own the field text in Panel and import its layout helpers. Initialize empty text and limit input to one layout pass. Lay out the command label, field and history fold control.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            egui::Panel::bottom("command-line-collapsed").exact_size(30.0)
                .frame(egui::Frame::new().fill(egui::Color32::WHITE)
                    .inner_margin(egui::Margin::symmetric(6, 4)))
                .show_inside(root, |ui| {
                ui.label("Command:");
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
```

Replace it with:

```rust
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.line, "Type a command", 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, line: String::new() }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
```

**Locate this block:**

```rust
use crate::command_dock::theme;
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
}

impl Panel {
```

Replace it with:

```rust
use crate::command_dock::{theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    line: String,
}

impl Panel {
```

**Complete updated block reference:**

```rust
use crate::command_dock::{theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    line: String,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, line: String::new() }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.line, "Type a command", 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

### 2. `src/command_dock/theme.rs`

Style selection and the caret for the command field.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03a-panel-04.rs"
```

### 3. `src/command_dock/mod.rs`

Register the shared field layout.

After that block, type:

```rust
--8<-- "journey/code/03a-panel-05.rs" skip=22
```

### 4. `src/command_dock/view.rs`

Add reusable panel, spacing and field functions.

Create the file and type:

```rust
--8<-- "journey/code/03a-panel-06.rs"
```

**Result after `03a-panel`:** The production command field appears beside Command: over the triangle. Input handling comes in the next command-line steps.

---

<a id="checkpoint-03b-memory"></a>

### Step 5 of 25 · Give the command field its memory (`03b-memory`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/panel.rs, src/browser…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">field borrows text</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">placeholder borrows sta…</text>
</svg>

Replace the panel’s local `String` with `CommandLine`. The field now reads and edits `model.command`; its empty hint comes from `model.status`.

`String` owns editable text. `VecDeque<String>` retains ordered history and can remove the oldest entry from the front. `Option` represents values that do not exist yet, such as an undrawn widget’s rectangle. `Default` starts strings and queues empty, flags false and optional values absent.

> **Check:** *Which value keeps the command text between draws?* — CommandLine inside Panel owns it. The field borrows model.command during layout; finishing the draw does not discard that String.

### 1. `src/panel.rs`

Replace the local String with an owned CommandLine. Give the model its initial status; keep all other fields at their defaults. Borrow the model text for editing and its status for the empty-field hint.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.line, "Type a command", 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
```

Replace it with:

```rust
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, false), 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, line: String::new() }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            status: "The field now belongs to CommandLine.".into(),
            ..Default::default()
        } }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
```

**Locate this block:**

```rust
use crate::command_dock::{theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    line: String,
}

impl Panel {
```

Replace it with:

```rust
use crate::command_dock::{CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
}

impl Panel {
```

**Complete updated block reference:**

```rust
use crate::command_dock::{CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            status: "The field now belongs to CommandLine.".into(),
            ..Default::default()
        } }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, false), 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

### 2. `src/browser.rs`

Report that the field now uses the command model.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03b-memory-04.rs"
```

### 3. `src/command_dock/mod.rs`

Define the state and choose the empty-field hint from borrowed text.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03b-memory-05.rs"
```

**Result after `03b-memory`:** The empty field now shows “The field now belongs to CommandLine.” Change that initial status, save, and check the hint changes. Restore it. Keyboard input comes later.

---

<a id="checkpoint-03b-state"></a>

### Step 6 of 25 · Prepare the dock completion helpers (`03b-state`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Cargo.toml, src/command_d…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">optional inspection</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">character-based caret p…</text>
</svg>

The upcoming dock layout needs command names without knowing our scene types. `Commands` defines the methods the application will supply. Static string slices refer to its fixed vocabulary.

`Control` stores an inspection key, label and rectangle. `record` appends actual egui response bounds through a mutable borrow. If inspection is absent, let-else returns immediately. `serde::Serialize` lets the inspector describe the record as JSON.

> **Check:** *Why does the caret helper use chars().count()?* — String::len counts UTF-8 bytes. egui caret ranges use character indices, so é🙂x needs position 3 rather than its byte length.

### 1. `Cargo.toml`

Enable the browser event, canvas geometry and focus bindings. Add serde serialization and JSON inspection dependencies.

Edit the existing block to match this code; keep unchanged lines:

```toml
[package]
name = "viewer_journey"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[workspace]

[dependencies]
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "Event", "PointerEvent", "MouseEvent", "KeyboardEvent", "WheelEvent", "FocusOptions", "DomRect", "HtmlElement"] }
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
pollster = "=0.4.0"
log = "=0.4.34"
```

### 2. `src/command_dock/mod.rs`

Define the application vocabulary contract without coupling the dock to scene types. Record actual widget bounds only when inspection is enabled. Add character-based caret and prefix helpers for the dock. Keep the helper declarations separated by a blank line.

Edit the existing block to match this code; keep unchanged lines:

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,                  // command line shown
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
    pub(crate) completion: usize,                   // highlighted completion index
    pub(crate) completion_prefix: String,           // text the completions match
    pub(crate) inline_suffix: bool,                 // completion suffix shown in the field
    pub(crate) completion_visible: bool,            // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,            // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>,      // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>,    // where those buttons are, for taps
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}
```

### 3. `src/panel.rs`

Import the module name for the following shared helper calls.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03b-state-direct-04.rs"
```

**Result after `03b-state`:** The connected field looks exactly as before. The vocabulary, inspection and caret helpers are ready for the following layout and input lessons.

---

<a id="checkpoint-03c-history"></a>

### Step 7 of 25 · Draw retained command history (`03c-history`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Draw retained history a…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Draw retained history a…</text>
</svg>

Draw each retained String above the command field. ScrollArea borrows the entries; the model keeps them after drawing.

> **Check:** *Does drawing move the retained strings?* — No. The loop borrows each entry.

### 1. `src/browser.rs`

Report that history is drawn.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03c-history-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Draw retained history and its divider.

After that block, type:

```rust
--8<-- "journey/code/03c-history-direct-02.rs" skip=9
```

### 3. `src/panel.rs`

Start with expanded history and one retained sentence. Choose panel height from the expanded state. Draw history before laying out the field. Use the expanded state for the hint and fold symbol.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, false), 0.0, false);
                    let _ = ui.button("+").on_hover_text("Collapse or expand history");
                });
            });
        });
```

Replace it with:

```rust
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, false);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
```

**Locate this block:**

```rust
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            status: "The field now belongs to CommandLine.".into(),
            ..Default::default()
        } }
    }
```

Replace it with:

```rust
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, false);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03c-history`:** The history sentence appears above the field.

---

<a id="checkpoint-03ca-typing"></a>

### Step 8 of 25 · Type directly into the command field (`03ca-typing`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/panel…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Send the first printabl…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Send the first printabl…</text>
</svg>

Focus the field before sending the first Text event. Panel owns the event queue; mem::take leaves it empty after drawing.

> **Check:** *Why focus before Text?* — The editor needs focus when egui processes the first character.

### 1. `src/browser.rs`

Retain the panel in keyboard callbacks; redraw and inspect accepted input.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ca-typing-direct-01.rs"
```

### 2. `src/panel.rs`

Store pending egui events in Panel. Initialize the empty event queue. Focus before queuing printable text or Backspace; expose the current text. Move queued events into the frame once. Draw the field with its current input focus.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, false);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
```

Replace it with:

```rust
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" {
            egui::Event::Key {
                key: egui::Key::Backspace, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else { return false; };
        self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
```

**Locate this block:**

```rust
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
}

impl Panel {
```

Replace it with:

```rust
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" {
            egui::Event::Key {
                key: egui::Key::Backspace, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else { return false; };
        self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03ca-typing`:** Click the scene and type G. G appears immediately in the command field.

---

<a id="checkpoint-03cb-submit"></a>

### Step 9 of 25 · Submit command text once (`03cb-submit`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Clear submitted text an…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Clear submitted text an…</text>
</svg>

Consume Enter before drawing the field. take_command moves a nonblank String out and leaves an empty String. remember retains at most 200 entries.

> **Check:** *Why take rather than clone the String?* — Submission transfers its ownership once and clears the source.

### 1. `src/browser.rs`

Expose the actual retained history to browser inspection. Report that Enter submits text.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Typing goes directly into the command field.");
    Ok(())
}
```

Replace it with:

```rust
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Enter submits the owned command text.");
    Ok(())
}
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
        if panel.key(&event) {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Enter submits the owned command text.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 2. `src/command_dock/mod.rs`

Take nonblank owned text and retain at most 200 history entries.

After that block, type:

```rust
--8<-- "journey/code/03cb-submit-direct-03.rs" skip=19
```

### 3. `src/panel.rs`

Expose the model’s retained history by reference. Recognize Enter as an editing key. Queue the corresponding Enter or Backspace event. Consume Enter once; retain the submitted line and answer.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
```

Replace it with:

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.model.status = "Command submitted.".into();
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
```

**Locate this block:**

```rust

    pub fn command(&self) -> &str { &self.model.command }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" {
            egui::Event::Key {
                key: egui::Key::Backspace, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
```

Replace it with:

```rust

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" || key == "Enter" {
            egui::Event::Key {
                key: if key == "Enter" { egui::Key::Enter } else { egui::Key::Backspace }, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" || key == "Enter" {
            egui::Event::Key {
                key: if key == "Enter" { egui::Key::Enter } else { egui::Key::Backspace }, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else { return false; };
        self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.model.status = "Command submitted.".into();
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cb-submit`:** Type Help and press Enter. One entry appears and the field clears. Empty Enter adds nothing.

---

<a id="checkpoint-03cc-edit"></a>

### Step 10 of 25 · Edit the command text (`03cc-edit`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep text selection, de…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep text selection, de…</text>
</svg>

Translate modifiers and editing keys into egui events. Ctrl+A selects text; Backspace deletes it; Escape clears and closes the field.

> **Check:** *Why skip text when Ctrl or Alt is held?* — Those keys modify an editing action rather than insert ordinary text.

### 1. `src/browser.rs`

Report that editing stays in the field.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03cc-edit-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Separate the model helpers with a blank line.

After that block, type:

```rust
--8<-- "journey/code/03cc-edit-direct-02.rs" skip=46
```

### 3. `src/panel.rs`

Translate modifiers and named editing keys; ignore composing text. Consume Escape, clear text and surrender field focus.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
```

Replace it with:

```rust
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                    self.model.command.clear();
                    self.model.command_open = false;
                    ui.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
```

**Locate this block:**

```rust
    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() { return false; }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" || key == "Enter" {
            egui::Event::Key {
                key: if key == "Enter" { egui::Key::Enter } else { egui::Key::Backspace }, physical_key: None,
                pressed: down, repeat: event.repeat(), modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else { return false; };
        self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }
```

Replace it with:

```rust
    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.model.status = "Command submitted.".into();
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                    self.model.command.clear();
                    self.model.command_open = false;
                    ui.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cc-edit`:** Type abc, press Ctrl+A, type x, then Escape. The field clears.

---

<a id="checkpoint-03cd-vocabulary"></a>

### Step 11 of 25 · Recognize Help (`03cd-vocabulary`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/command_vocabulary.rs…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Connect known names and…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Connect known names and…</text>
</svg>

Commands borrows a fixed vocabulary. The adapter normalizes spacing and case; Help lists its names. Unknown input receives a useful reply.

A matching prefix is not enough to choose inline completion. Rank an exact command first, then a match that extends at a word boundary. Keep the remaining prefixes in registry order. This prevents Select from becoming Selection before the user can type Object, and keeps an exact named view ahead of its longer variant.

> **Check:** *Why must an exact or whole-word command rank ahead of a longer character prefix?* — Completion must preserve the words the user is typing. An exact match wins, a following space preserves the completed word, and a suffix inside that word must not replace it.

### 1. `src/command_vocabulary.rs`

Rank exact words, then a whole-word extension, then remaining prefixes without replacing typed command meaning.

Create the file and type:

```rust
--8<-- "journey/code/03cd-vocabulary-ranked.rs"
```

### 2. `src/browser.rs`

Supply Help as the initial application vocabulary. Report that Help is recognized.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Editing stays in the command field.");
    Ok(())
}
```

Replace it with:

```rust
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Help recognizes the command vocabulary.");
    Ok(())
}
```

**Locate this block:**

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
```

Replace it with:

```rust
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
        if panel.key(&event) {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Help recognizes the command vocabulary.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 3. `src/panel.rs`

Import the vocabulary trait methods. Normalize names, find completions and answer Help or unknown input. Store the vocabulary adapter in Panel. Accept a static vocabulary in the constructor. Initialize the supplied vocabulary. Answer the submitted line through the vocabulary adapter.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.model.status = "Command submitted.".into();
                        self.model.remember(format!("> {line}\n{}", self.model.status));
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
```

Replace it with:

```rust
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.commands.reply(&mut self.model, &line);
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
```

**Locate this block:**

```rust
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
```

Replace it with:

```rust
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
```

**Locate this block:**

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

Replace it with:

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.commands.reply(&mut self.model, &line);
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                    self.model.command.clear();
                    self.model.command_open = false;
                    ui.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

### 4. `src/lib.rs`

Compile the shared ranking helper alongside the first command dictionary.

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;

#[cfg(target_arch = "wasm32")]
mod panel;
```

**Result after `03cd-vocabulary`:** Type Help and press Enter. History shows Help as the available command.

---

<a id="checkpoint-03ce-complete"></a>

### Step 12 of 25 · Complete a command name (`03ce-complete`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Select only the suggest…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Select only the suggest…</text>
</svg>

After the field changes, find a matching command name. Replace the text and select only the suggested suffix, so the next letter replaces that suffix.

> **Check:** *Why count characters instead of bytes?* — egui caret positions use characters; UTF-8 characters can occupy several bytes.

### 1. `src/browser.rs`

Report that inline completion is connected.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ce-complete-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Match names after changes and select only the suggested suffix.

After that block, type:

```rust
--8<-- "journey/code/03ce-complete-direct-02.rs" skip=47
```

### 3. `src/panel.rs`

Keep the field response and refresh completion after editing.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ce-complete-direct-03.rs"
```

**Result after `03ce-complete`:** Type He. The field shows Help with the suggested ending selected. Backspace leaves He.

---

<a id="checkpoint-03cf-accept"></a>

### Step 13 of 25 · Accept or cancel a completion (`03cf-accept`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/command_dock/mod.rs,…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Use Tab, Enter and Esca…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Use Tab, Enter and Esca…</text>
</svg>

Read Enter and Tab before the field. finish accepts a completion, submits a nonblank command, or cancels with Escape.

> **Check:** *Why consume Enter once?* — The field and command dispatcher must not both submit the same event.

### 1. `src/command_dock/mod.rs`

Retain the editing-key results for this frame. Accept completion, submit Enter or cancel Escape once.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        model.completion_prefix.clone_from(&model.command);
    }
}
```

Replace it with:

```rust
        model.completion_prefix.clone_from(&model.command);
    }
}

pub(crate) fn finish(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    command: &mut Option<String>,
    commands: &impl Commands,
    complete: Option<&str>,
    keys: &Keys,
    escape_allowed: bool,
) -> bool {
    if let Some(name) = complete {
        let (text, run) = commands.accept(name);
        if run && !keys.tab {
            *command = Some(text);
            model.command.clear();
        } else {
            model.command = if run { format!("{text} ") } else { text };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
        let line = model
            .take_command()
            .unwrap_or_else(|| std::mem::take(&mut model.command));
        let (text, run) = commands.accept(&line);
        // a verb waiting for one of many options opens their list
        let waits = !run && commands.choosing_option(&text);
        if run {
            *command = Some(text);
        } else {
            model.completion_prefix.clone_from(&text);
            model.completion = 0;
            model.command = text;
        }
        model.completion_visible = waits;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused && ui.input(|input| input.key_pressed(egui::Key::Escape)) && escape_allowed {
        model.command.clear();
        model.completion_visible = false;
        model.inline_suffix = false;
        model.command_open = false;
        model.focus_command = false;
        response.surrender_focus();
        *command = Some("Escape".into());
        return true;
    }
    false
}
```

**Locate this block:**

```rust
    }
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
```

Replace it with:

```rust
    }
}

#[derive(Default)]
pub(crate) struct Keys {
    pub(crate) browse: isize,
    pub(crate) enter: bool,
    pub(crate) tab: bool,
    pub(crate) deletes: bool,
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
```

**Complete updated block reference:**

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,                  // command line shown
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
    pub(crate) completion: usize,                   // highlighted completion index
    pub(crate) completion_prefix: String,           // text the completions match
    pub(crate) inline_suffix: bool,                 // completion suffix shown in the field
    pub(crate) completion_visible: bool,            // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,            // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>,      // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>,    // where those buttons are, for taps
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}

/// Draw retained history above the field.
pub(crate) fn history(ui: &mut egui::Ui, model: &CommandLine, controls: &mut Option<Vec<Control>>) {
    if model.command_expanded {
        egui::ScrollArea::vertical()
            .id_salt("command-history")
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .min_scrolled_height(0.0)
            .max_height((ui.available_height() - 38.0).max(0.0))
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width());
                for text in &model.history {
                    ui.add(egui::Label::new(text).wrap());
                }
                if !model.status.is_empty()
                    && !model
                        .history
                        .back()
                        .is_some_and(|text| text.ends_with(&model.status))
                {
                    ui.label(&model.status);
                }
                if !model.drawing_prompt.is_empty() {
                    let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                    record(controls, "command/hint", &model.drawing_prompt, &response);
                }
            });
        // divider between history and the field
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
        ui.painter().hline(
            ui.max_rect().x_range().expand(6.0),
            rect.center().y,
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
        );
    }
}

impl CommandLine {
    pub(crate) fn take_command(&mut self) -> Option<String> {
        if self.command.trim().is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.command))
    }

    pub(crate) fn remember(&mut self, line: String) {
        if self.history.len() == 200 {
            self.history.pop_front();
        }
        self.history.push_back(line);
    }
}

#[derive(Default)]
pub(crate) struct Keys {
    pub(crate) browse: isize,
    pub(crate) enter: bool,
    pub(crate) tab: bool,
    pub(crate) deletes: bool,
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    commands: &impl Commands,
    id: egui::Id,
    deletes: bool,
) {
    if response.gained_focus() {
        model.command_open = true;
    }
    let agent_edit = model.agent_edit.take();
    if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
        model.command.remove(0);
    }
    if response.changed() || agent_edit.is_some() {
        model.completion = 0;
        model.completion_visible = !model.command.is_empty();
        model.completion_prefix.clone_from(&model.command);
        model.inline_suffix = false;
        let at_end = egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .is_some_and(|range| {
                range.is_empty() && range.primary.index == model.command.chars().count()
            });
        if !deletes
            && model.options.is_empty()
            && agent_edit != Some(true)
            && at_end
            && !model.command.is_empty()
            && !model.command.ends_with(' ')
            && let Some(name) = commands.completions(&model.command).first()
        {
            let prefix = spelled(name, &commands.canonical(&model.command));
            if name.chars().count() > prefix {
                model.command = (*name).into();
                command_cursor_select(ui.ctx(), id, prefix, model.command.chars().count());
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model.completion_prefix.clone_from(&model.command);
    }
}

pub(crate) fn finish(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    command: &mut Option<String>,
    commands: &impl Commands,
    complete: Option<&str>,
    keys: &Keys,
    escape_allowed: bool,
) -> bool {
    if let Some(name) = complete {
        let (text, run) = commands.accept(name);
        if run && !keys.tab {
            *command = Some(text);
            model.command.clear();
        } else {
            model.command = if run { format!("{text} ") } else { text };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
        let line = model
            .take_command()
            .unwrap_or_else(|| std::mem::take(&mut model.command));
        let (text, run) = commands.accept(&line);
        // a verb waiting for one of many options opens their list
        let waits = !run && commands.choosing_option(&text);
        if run {
            *command = Some(text);
        } else {
            model.completion_prefix.clone_from(&text);
            model.completion = 0;
            model.command = text;
        }
        model.completion_visible = waits;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused && ui.input(|input| input.key_pressed(egui::Key::Escape)) && escape_allowed {
        model.command.clear();
        model.completion_visible = false;
        model.inline_suffix = false;
        model.command_open = false;
        model.focus_command = false;
        response.surrender_focus();
        *command = Some("Escape".into());
        return true;
    }
    false
}
```

### 2. `src/panel.rs`

Consume Enter and Tab before the field handles input. Route completion and submission through finish.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)) {
                    if let Some(line) = self.model.take_command() {
                        self.commands.reply(&mut self.model, &line);
                    }
                }
                if self.model.command_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                    self.model.command.clear();
                    self.model.command_open = false;
                    ui.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
                }
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, ui.input(|input| input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)));
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
```

Replace it with:

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                let keys = command_dock::Keys {
                    enter: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)),
                    tab: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)),
                    deletes: ui.input(|input| input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)),
                    ..Default::default()
                };
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = keys.tab.then(|| self.commands.completions(&self.model.completion_prefix).first().copied()).flatten();
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                let keys = command_dock::Keys {
                    enter: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)),
                    tab: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)),
                    deletes: ui.input(|input| input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)),
                    ..Default::default()
                };
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = keys.tab.then(|| self.commands.completions(&self.model.completion_prefix).first().copied()).flatten();
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cf-accept`:** Type He, then Tab. Help gains a trailing space. Escape clears it.

---

<a id="checkpoint-03cg-keys"></a>

### Step 14 of 25 · Read editing keys before the field (`03cg-keys`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Consume command editing…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Consume command editing…</text>
</svg>

Move key handling into keys. Restore focus before text editing; suffix_space keeps a space inside a multiword completion rather than replacing the remaining name.

> **Check:** *Why restore focus before drawing the field?* — The field must own input when it handles queued events.

### 1. `src/browser.rs`

Report that editing keys are read first.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03cg-keys-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Handle a space inside the selected suffix; centralize editing and focus keys.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03cg-keys-direct-02.rs"
```

### 3. `src/panel.rs`

Restore field focus before accepting the first text event. Remove the replaced inline key collection. Read the editing keys before drawing the field.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                let keys = command_dock::Keys {
                    enter: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)),
                    tab: ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)),
                    deletes: ui.input(|input| input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)),
                    ..Default::default()
                };
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
```

Replace it with:

```rust
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
```

**Locate this block:**

```rust
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
```

Replace it with:

```rust
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = keys.tab.then(|| self.commands.completions(&self.model.completion_prefix).first().copied()).flatten();
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cg-keys`:** Type He, press Tab, then Enter. The command is submitted once.

---

<a id="checkpoint-03ch-popup"></a>

### Step 15 of 25 · Show matching commands (`03ch-popup`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Draw the matching names…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Draw the matching names…</text>
</svg>

Draw matching names above the field. A scroll area bounds the list, and the model remembers its actual rectangle. Pointer selection is connected later.

> **Check:** *Why store the drawn rectangle?* — Later pointer handling needs the actual layout bounds.

### 1. `src/browser.rs`

Report that matching commands are drawn.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ch-popup-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Draw matching names in a bounded popup and remember its rectangle.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ch-popup-direct-02.rs"
```

### 3. `src/panel.rs`

Clear the previous popup rectangle before layout. Draw current matches; preserve Tab acceptance.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = keys.tab.then(|| self.commands.completions(&self.model.completion_prefix).first().copied()).flatten();
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
```

Replace it with:

```rust
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let choices = self.commands.completions(&self.model.completion_prefix);
                    let complete = if self.model.completion_visible && !choices.is_empty() { command_dock::popup(ui, &mut self.model, &mut None, &choices, &response, 0) } else { None };
                    let complete = complete.or_else(|| keys.tab.then(|| choices.first().copied()).flatten());
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
```

**Locate this block:**

```rust
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
```

Replace it with:

```rust
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let choices = self.commands.completions(&self.model.completion_prefix);
                    let complete = if self.model.completion_visible && !choices.is_empty() { command_dock::popup(ui, &mut self.model, &mut None, &choices, &response, 0) } else { None };
                    let complete = complete.or_else(|| keys.tab.then(|| choices.first().copied()).flatten());
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03ch-popup`:** Type He. Help appears above the field as a suggestion.

---

<a id="checkpoint-03ci-browse"></a>

### Step 16 of 25 · Browse matching names (`03ci-browse`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Move through matching c…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Move through matching c…</text>
</svg>

Use the original typed prefix while browsing matches. Arrow keys wrap the highlighted index; the selected suffix remains replaceable.

> **Check:** *Why retain the prefix while browsing?* — The chosen suggestion must not become a new search filter.

### 1. `src/browser.rs`

Report that arrow browsing is connected.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ci-browse-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Wrap the match index and retain the original typed prefix.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ci-browse-direct-02.rs"
```

### 3. `src/panel.rs`

Use browse for arrow keys, Tab and popup choices.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03ci-browse-direct-03.rs"
```

**Result after `03ci-browse`:** Press ArrowDown in the empty field. Help appears. Escape clears it.

---

<a id="checkpoint-03cj-prepare"></a>

### Step 17 of 25 · Prepare the dock before painting (`03cj-prepare`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/panel…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep UI output owned un…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep UI output owned un…</text>
</svg>

Prepare UI output before painting. Append any additional layout output so changed font textures survive until the single GPU submission.

> **Check:** *Why append output before painting?* — Another layout may add texture changes that the renderer still needs.

### 1. `src/browser.rs`

Report that prepared UI is painted once.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03cj-prepare-direct-01.rs"
```

### 2. `src/panel.rs`

Own prepared output and the screen descriptor. Initialize pending output and screen size. Move input and layout into prepare. Convert physical dimensions into egui points. Append prepared output; take it once for GPU painting. Use the retained screen descriptor when uploading UI buffers. Use the same screen descriptor when painting UI.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
```

Replace it with:

```rust
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
```

**Locate this block:**

```rust
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
```

Replace it with:

```rust
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
```

**Locate this block:**

```rust
                });
            });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
```

Replace it with:

```rust
                });
            });
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
```

**Locate this block:**

```rust
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: 1.0 };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
```

Replace it with:

```rust
        true
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
```

**Locate this block:**

```rust
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
```

Replace it with:

```rust
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut None, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cj-prepare`:** Type Help. Text and the triangle remain visible after repeated key events.

---

<a id="checkpoint-03ck-rects"></a>

### Step 18 of 25 · Remember where the dock is drawn (`03ck-rects`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Record actual widget bo…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Record actual widget bo…</text>
</svg>

Record the real field and collapse-control rectangles. The inspector reads these bounds from egui responses rather than guessing their positions.

> **Check:** *Why use response rectangles?* — They reflect the actual layout and scale used for this frame.

### 1. `src/browser.rs`

Expose actual dock state and rectangles for inspection. Report that drawn bounds are recorded.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The prepared dock is painted once.");
    Ok(())
}
```

Replace it with:

```rust
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The dock records its actual rectangles.");
    Ok(())
}
```

**Locate this block:**

```rust
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
```

Replace it with:

```rust
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command-ui", &panel.inspect()) {
                report(&format!("Cannot inspect dock: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
        if panel.key(&event) {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command-ui", &panel.inspect()) {
                report(&format!("Cannot inspect dock: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The dock records its actual rectangles.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 2. `src/command_dock/mod.rs`

Record the fold control and toggle expanded history on click.

After that block, type:

```rust
--8<-- "journey/code/03ck-rects-direct-03.rs" skip=13
```

### 3. `src/panel.rs`

Store current control bounds and the panel’s top edge. Initialize inspection storage and the top edge. Serialize actual widget bounds, text, history and focus. Start a fresh control list for each layout. Keep the panel response to read its drawn bounds. Collect controls while drawing history. Record the actual field rectangle. Collect controls while drawing suggestions. Draw the recorded fold control. Retain the panel’s top edge for pointer ownership. Prepare the updated layout before painting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
```

Replace it with:

```rust
    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
```

**Locate this block:**

```rust
            ..Default::default()
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut None);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut None, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    let _ = ui.button(if self.model.command_expanded { "–" } else { "+" }).on_hover_text("Collapse or expand history");
                });
            });
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
```

Replace it with:

```rust
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut self.controls);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut self.controls, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    command_dock::collapse(ui, &mut self.model, &mut self.controls);
                });
            });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
```

**Locate this block:**

```rust
        true
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
```

Replace it with:

```rust
        true
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
```

**Locate this block:**

```rust

pub struct Panel {
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

Replace it with:

```rust

pub struct Panel {
    commands: Commands,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut self.controls);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut self.controls, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    command_dock::collapse(ui, &mut self.model, &mut self.controls);
                });
            });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03ck-rects`:** Type He. The field and suggestion remain correctly placed above the history.

---

<a id="checkpoint-03ck-pointer"></a>

### Step 19 of 25 · Click the command dock (`03ck-pointer`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/panel…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Use pointer ownership t…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Use pointer ownership t…</text>
</svg>

Translate mouse positions into canvas coordinates. Keep a press owned until release or cancellation. collapse changes layout while retained history stays in the model.

> **Check:** *Why retain pointer ownership?* — A drag must stay with the widget even after leaving its rectangle.

### 1. `src/browser.rs`

Dispatch keyboard and pointer events through one retained callback. Register pointer press, movement, release and cancellation. Report that mouse input is connected.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
        }
    });
    for name in ["keydown", "keyup"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The dock records its actual rectangles.");
    Ok(())
}
```

Replace it with:

```rust
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The mouse operates the command dock.");
    Ok(())
}
```

**Locate this block:**

```rust
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |event| {
        if panel.key(&event) {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
```

Replace it with:

```rust
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command-ui", &panel.inspect()) {
                report(&format!("Cannot inspect dock: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The mouse operates the command dock.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 2. `src/panel.rs`

Retain pointer ownership between press and release. Initialize pointer ownership. Translate canvas coordinates, preserve drag ownership and release cancelled buttons. Expose pointer ownership so cancellation can be checked against the actual event handler.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        true
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }
```

Replace it with:

```rust
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
```

**Locate this block:**

```rust

pub struct Panel {
    commands: Commands,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
```

Replace it with:

```rust

pub struct Panel {
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut self.controls);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut self.controls, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    command_dock::collapse(ui, &mut self.model, &mut self.controls);
                });
            });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03ck-pointer`:** Click the history fold control. History folds away; click again to restore it.

---

<a id="checkpoint-03cl-wheel"></a>

### Step 20 of 25 · Scroll through command names (`03cl-wheel`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep scrolling over the…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep scrolling over the…</text>
</svg>

Queue wheel input with the pointer position. Consume scrolling over the field or previous suggestion rectangle before it can reach the scene.

> **Check:** *Why remember the previous popup rectangle?* — Input arrives before this frame draws the new popup.

### 1. `src/browser.rs`

Dispatch wheel events to Panel. Register wheel input with preventDefault enabled. Report that scrolling selects command names.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The mouse operates the command dock.");
    Ok(())
}
```

Replace it with:

```rust
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options("wheel", redraw.as_ref().unchecked_ref(), &options)?;
    redraw.forget();
    report("Scrolling selects a command name.");
    Ok(())
}
```

**Locate this block:**

```rust
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
```

Replace it with:

```rust
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { panel.wheel(wheel, &input_canvas); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { panel.wheel(wheel, &input_canvas); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command-ui", &panel.inspect()) {
                report(&format!("Cannot inspect dock: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options("wheel", redraw.as_ref().unchecked_ref(), &options)?;
    redraw.forget();
    report("Scrolling selects a command name.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 2. `src/command_dock/mod.rs`

Consume wheel movement over the field or previous popup. Focus the field when scrolling its command names. Give wheel browsing priority over arrow browsing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
```

Replace it with:

```rust
        }
        model.command_open = true;
    }
    let wheel = wheel(ui, model, previous);
    if wheel != 0 {
        ui.memory_mut(|memory| memory.request_focus(id));
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if wheel != 0 {
        wheel
    } else if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
```

**Locate this block:**

```rust
    pub(crate) deletes: bool,
}

pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
```

Replace it with:

```rust
    pub(crate) deletes: bool,
}

pub(crate) fn wheel(ui: &mut egui::Ui, model: &CommandLine, previous: Option<egui::Rect>) -> isize {
    ui.input_mut(|input| {
        let over = input.pointer.hover_pos().is_some_and(|point| {
            model.command_rect.is_some_and(|rect| rect.contains(point))
                || previous.is_some_and(|rect| rect.contains(point))
        });
        if !over {
            return 0;
        }
        let delta: f32 = input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                _ => None,
            })
            .sum();
        input.smooth_scroll_delta = egui::Vec2::ZERO;
        if delta > 0.0 {
            -1
        } else if delta < 0.0 {
            1
        } else {
            0
        }
    })
}

pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
```

**Complete updated block reference:**

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,                  // command line shown
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
    pub(crate) completion: usize,                   // highlighted completion index
    pub(crate) completion_prefix: String,           // text the completions match
    pub(crate) inline_suffix: bool,                 // completion suffix shown in the field
    pub(crate) completion_visible: bool,            // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,            // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>,      // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>,    // where those buttons are, for taps
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}

/// Draw retained history above the field.
pub(crate) fn history(ui: &mut egui::Ui, model: &CommandLine, controls: &mut Option<Vec<Control>>) {
    if model.command_expanded {
        egui::ScrollArea::vertical()
            .id_salt("command-history")
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .min_scrolled_height(0.0)
            .max_height((ui.available_height() - 38.0).max(0.0))
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width());
                for text in &model.history {
                    ui.add(egui::Label::new(text).wrap());
                }
                if !model.status.is_empty()
                    && !model
                        .history
                        .back()
                        .is_some_and(|text| text.ends_with(&model.status))
                {
                    ui.label(&model.status);
                }
                if !model.drawing_prompt.is_empty() {
                    let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                    record(controls, "command/hint", &model.drawing_prompt, &response);
                }
            });
        // divider between history and the field
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
        ui.painter().hline(
            ui.max_rect().x_range().expand(6.0),
            rect.center().y,
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
        );
    }
}

impl CommandLine {
    pub(crate) fn take_command(&mut self) -> Option<String> {
        if self.command.trim().is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.command))
    }

    pub(crate) fn remember(&mut self, line: String) {
        if self.history.len() == 200 {
            self.history.pop_front();
        }
        self.history.push_back(line);
    }
}

#[derive(Default)]
pub(crate) struct Keys {
    pub(crate) browse: isize,
    pub(crate) enter: bool,
    pub(crate) tab: bool,
    pub(crate) deletes: bool,
}

pub(crate) fn wheel(ui: &mut egui::Ui, model: &CommandLine, previous: Option<egui::Rect>) -> isize {
    ui.input_mut(|input| {
        let over = input.pointer.hover_pos().is_some_and(|point| {
            model.command_rect.is_some_and(|rect| rect.contains(point))
                || previous.is_some_and(|rect| rect.contains(point))
        });
        if !over {
            return 0;
        }
        let delta: f32 = input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                _ => None,
            })
            .sum();
        input.smooth_scroll_delta = egui::Vec2::ZERO;
        if delta > 0.0 {
            -1
        } else if delta < 0.0 {
            1
        } else {
            0
        }
    })
}

pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
        && model
            .command
            .chars()
            .nth(spelled(&model.command, &model.completion_prefix))
            != Some(' ')
        && ui.input(|input| {
            input
                .events
                .iter()
                .find_map(|event| match event {
                    egui::Event::Text(text) => Some(text),
                    _ => None,
                })
                .is_some_and(|text| text.starts_with(' '))
        })
    {
        command_cursor_end(ui.ctx(), id, &model.command);
        model.inline_suffix = false;
    }
}

pub(crate) fn keys(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    previous: Option<egui::Rect>,
    id: egui::Id,
) -> Keys {
    if model.focus_command || model.command_open {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
            model.focus_command = false;
        }
        model.command_open = true;
    }
    let wheel = wheel(ui, model, previous);
    if wheel != 0 {
        ui.memory_mut(|memory| memory.request_focus(id));
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if wheel != 0 {
        wheel
    } else if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                -1
            } else {
                0
            }
        })
    } else {
        0
    };
    let enter =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let tab =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
    let deletes = ui.input(|input| {
        input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)
    });
    suffix_space(ui, model, id, focused);
    Keys {
        browse,
        enter,
        tab,
        deletes,
    }
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    commands: &impl Commands,
    id: egui::Id,
    deletes: bool,
) {
    if response.gained_focus() {
        model.command_open = true;
    }
    let agent_edit = model.agent_edit.take();
    if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
        model.command.remove(0);
    }
    if response.changed() || agent_edit.is_some() {
        model.completion = 0;
        model.completion_visible = !model.command.is_empty();
        model.completion_prefix.clone_from(&model.command);
        model.inline_suffix = false;
        let at_end = egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .is_some_and(|range| {
                range.is_empty() && range.primary.index == model.command.chars().count()
            });
        if !deletes
            && model.options.is_empty()
            && agent_edit != Some(true)
            && at_end
            && !model.command.is_empty()
            && !model.command.ends_with(' ')
            && let Some(name) = commands.completions(&model.command).first()
        {
            let prefix = spelled(name, &commands.canonical(&model.command));
            if name.chars().count() > prefix {
                model.command = (*name).into();
                command_cursor_select(ui.ctx(), id, prefix, model.command.chars().count());
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model.completion_prefix.clone_from(&model.command);
    }
}

pub(crate) fn popup(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    choices: &[&'static str],
    response: &egui::Response,
    browse: isize,
) -> Option<&'static str> {
    let width = (ui.ctx().content_rect().right() - response.rect.left() - 12.0).clamp(60.0, 220.0);
    let height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
    let mut complete = None;
    let popup = egui::Area::new(egui::Id::new("command-completions"))
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    egui::ScrollArea::vertical()
                        .id_salt("command-choices")
                        .max_height(height)
                        .show(ui, |ui| {
                            for (index, name) in choices.iter().enumerate() {
                                let item = ui.selectable_label(index == model.completion, *name);
                                record(
                                    controls,
                                    &format!("command/completion/{name}"),
                                    name,
                                    &item,
                                );
                                if index == model.completion && browse != 0 {
                                    item.scroll_to_me(None);
                                }
                                if item.clicked() {
                                    complete = Some(*name);
                                }
                            }
                        });
                });
        });
    model.completion_rect = Some(popup.response.rect);
    complete
}

pub(crate) fn browse(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    commands: &impl Commands,
    response: &egui::Response,
    keys: &Keys,
    id: egui::Id,
) -> Option<&'static str> {
    let choices = commands.browse(&model.completion_prefix);
    let opening = !model.completion_visible;
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if !focused || !model.completion_visible || choices.is_empty() || !model.options.is_empty() {
        return None;
    }
    model.completion = model.completion.min(choices.len() - 1);
    if commands.choosing_option(&model.completion_prefix)
        && let Some(index) = choices
            .iter()
            .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
    {
        model.completion = index;
    }
    if keys.browse != 0 {
        model.completion = if opening && !commands.choosing_option(&model.completion_prefix) {
            if keys.browse < 0 {
                choices.len() - 1
            } else {
                0
            }
        } else {
            (model.completion as isize + keys.browse).rem_euclid(choices.len() as isize) as usize
        };
        model.command = choices[model.completion].into();
        let start = if model
            .command
            .to_ascii_lowercase()
            .starts_with(&model.completion_prefix.to_ascii_lowercase())
        {
            model.completion_prefix.chars().count()
        } else {
            0
        };
        command_cursor_select(ui.ctx(), id, start, model.command.chars().count());
        model.inline_suffix = true;
        ui.ctx().request_repaint();
    }
    let mut complete = if keys.tab {
        Some(choices[model.completion])
    } else {
        None
    };
    // the list shows the verbs, or the options of the verb typed
    complete = popup(ui, model, controls, &choices, response, keys.browse).or(complete);
    complete
}

pub(crate) fn finish(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    command: &mut Option<String>,
    commands: &impl Commands,
    complete: Option<&str>,
    keys: &Keys,
    escape_allowed: bool,
) -> bool {
    if let Some(name) = complete {
        let (text, run) = commands.accept(name);
        if run && !keys.tab {
            *command = Some(text);
            model.command.clear();
        } else {
            model.command = if run { format!("{text} ") } else { text };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
        let line = model
            .take_command()
            .unwrap_or_else(|| std::mem::take(&mut model.command));
        let (text, run) = commands.accept(&line);
        // a verb waiting for one of many options opens their list
        let waits = !run && commands.choosing_option(&text);
        if run {
            *command = Some(text);
        } else {
            model.completion_prefix.clone_from(&text);
            model.completion = 0;
            model.command = text;
        }
        model.completion_visible = waits;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused && ui.input(|input| input.key_pressed(egui::Key::Escape)) && escape_allowed {
        model.command.clear();
        model.completion_visible = false;
        model.inline_suffix = false;
        model.command_open = false;
        model.focus_command = false;
        response.surrender_focus();
        *command = Some("Escape".into());
        return true;
    }
    false
}

pub(crate) fn collapse(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
    let response = ui
        .button(if model.command_expanded { "–" } else { "+" })
        .on_hover_text("Collapse or expand history");
    record(
        controls,
        "command/collapse",
        "Collapse or expand history",
        &response,
    );
    if response.clicked() {
        model.command_expanded = !model.command_expanded;
        ui.ctx().request_repaint();
    }
}
```

### 3. `src/panel.rs`

Queue wheel units and deltas at the canvas pointer position. Take the previous popup rectangle before drawing. Pass previous popup bounds to key and wheel handling.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, None, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
```

Replace it with:

```rust
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, previous, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
```

**Locate this block:**

```rust
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
```

Replace it with:

```rust
            )),
            ..Default::default()
        };
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
```

**Locate this block:**

```rust
        owned
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
```

Replace it with:

```rust
        owned
    }

    pub fn wheel(&mut self, event: &web_sys::WheelEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        self.events.push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        if over { event.prevent_default(); }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn wheel(&mut self, event: &web_sys::WheelEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        self.events.push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        if over { event.prevent_default(); }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut self.controls);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, previous, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut self.controls, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    command_dock::collapse(ui, &mut self.model, &mut self.controls);
                });
            });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cl-wheel`:** Move the mouse over the field and scroll. Help is selected in the command list.

---

<a id="checkpoint-03c-layout"></a>

### Step 21 of 25 · Share the production dock layout (`03c-layout`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/comma…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Connect the existing ro…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Connect the existing ro…</text>
</svg>

Put the already connected helpers into one row and layout. The same command_dock source now draws both this lesson and the production viewer.

> **Check:** *Does sharing the layout give it ownership of the scene?* — No. It edits the model and returns an accepted command to its caller.

### 1. `src/browser.rs`

Report that the shared production layout is connected.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03c-layout-direct-01.rs"
```

### 2. `src/command_dock/mod.rs`

Combine the connected field, history and controls into row and draw.

After that block, type:

```rust
--8<-- "journey/code/03c-layout-direct-04.rs" skip=44
```

### 3. `src/panel.rs`

Remove imports now owned by the shared layout. Remove inline setup handled by command_dock::draw. Retain an accepted command for this layout. Call the same command_dock layout as production. Read the drawn top edge and answer accepted commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            )),
            ..Default::default()
        };
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                command_dock::history(ui, &self.model, &mut self.controls);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    let id = egui::Id::new("command-input");
                    let keys = command_dock::keys(ui, &mut self.model, previous, id);
                    let response = view::field(ui, &mut self.model.command, placeholder("", &self.model.status, self.model.command_expanded), 0.0, self.model.command_open);
                    self.model.command_rect = Some(response.rect);
                    command_dock::record(&mut self.controls, "command/input", "Command", &response);
                    command_dock::refresh(ui, &mut self.model, &response, &self.commands, id, keys.deletes);
                    let mut line = None;
                    let complete = command_dock::browse(ui, &mut self.model, &mut self.controls, &self.commands, &response, &keys, id);
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
                    if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
                    command_dock::collapse(ui, &mut self.model, &mut self.controls);
                });
            });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }
```

Replace it with:

```rust
            )),
            ..Default::default()
        };
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(root, &mut self.model, &mut self.controls, &mut line, &self.commands, true);
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }
```

**Locate this block:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, placeholder, theme, view};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);
```

Replace it with:

```rust
use crate::command_dock::{self, CommandLine, Commands as _, theme};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, theme};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn wheel(&mut self, event: &web_sys::WheelEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        self.events.push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        if over { event.prevent_default(); }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(root, &mut self.model, &mut self.controls, &mut line, &self.commands, true);
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03c-layout`:** Type Help and press Enter. The shared dock clears the field and adds one answer.

---

<a id="checkpoint-03co-align"></a>

### Step 22 of 25 · Align and measure the command row (`03co-align`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/command_dock/mod.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">drawn dock edge</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">future usable viewport</text>
</svg>

Give the shared row the field’s height and centre its children vertically. Store the drawn dock edge on the existing model.

The row still returns accepted commands to its caller. The new optional edge is layout metadata, not scene state. Later fitting uses it to keep geometry above the dock.

> **Check:** *Why record the dock edge while drawing instead of assuming a fixed footer height?* — The current layout knows the actual edge. Later fitting can use that value when history or other panels change the visible canvas.

### 1. `src/command_dock/mod.rs`

Use the field height and centre the row’s children. Keep the actual dock edge as optional layout metadata. Record the dock edge from the current drawn rectangle.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let mut focus_canvas = false;
    let panel = view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
        view::prepare(ui);
        history(ui, model, controls);
        focus_canvas = row(
            ui,
```

Replace it with:

```rust
    let mut focus_canvas = false;
    let panel = view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
        view::prepare(ui);
        model.dock_top = Some(ui.max_rect().top());
        history(ui, model, controls);
        focus_canvas = row(
            ui,
```

**Locate this block:**

```rust
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    ui.horizontal(|ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
```

Replace it with:

```rust
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    // as tall as the field, so the label and the typed text share one centre line
    let size = egui::vec2(ui.available_width(), 22.0);
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    ui.allocate_ui_with_layout(size, layout, |ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
```

**Locate this block:**

```rust
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
```

Replace it with:

```rust
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) dock_top: Option<f32>,               // the dock's top edge in CSS px, for fitting above it
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
```

**Complete updated block reference:**

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,                  // command line shown
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) dock_top: Option<f32>,               // the dock's top edge in CSS px, for fitting above it
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
    pub(crate) completion: usize,                   // highlighted completion index
    pub(crate) completion_prefix: String,           // text the completions match
    pub(crate) inline_suffix: bool,                 // completion suffix shown in the field
    pub(crate) completion_visible: bool,            // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,            // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>,      // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>,    // where those buttons are, for taps
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}

/// Draw retained history above the field.
pub(crate) fn history(ui: &mut egui::Ui, model: &CommandLine, controls: &mut Option<Vec<Control>>) {
    if model.command_expanded {
        egui::ScrollArea::vertical()
            .id_salt("command-history")
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .min_scrolled_height(0.0)
            .max_height((ui.available_height() - 38.0).max(0.0))
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width());
                for text in &model.history {
                    ui.add(egui::Label::new(text).wrap());
                }
                if !model.status.is_empty()
                    && !model
                        .history
                        .back()
                        .is_some_and(|text| text.ends_with(&model.status))
                {
                    ui.label(&model.status);
                }
                if !model.drawing_prompt.is_empty() {
                    let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                    record(controls, "command/hint", &model.drawing_prompt, &response);
                }
            });
        // divider between history and the field
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
        ui.painter().hline(
            ui.max_rect().x_range().expand(6.0),
            rect.center().y,
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
        );
    }
}

impl CommandLine {
    pub(crate) fn take_command(&mut self) -> Option<String> {
        if self.command.trim().is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.command))
    }

    pub(crate) fn remember(&mut self, line: String) {
        if self.history.len() == 200 {
            self.history.pop_front();
        }
        self.history.push_back(line);
    }
}

#[derive(Default)]
pub(crate) struct Keys {
    pub(crate) browse: isize,
    pub(crate) enter: bool,
    pub(crate) tab: bool,
    pub(crate) deletes: bool,
}

pub(crate) fn wheel(ui: &mut egui::Ui, model: &CommandLine, previous: Option<egui::Rect>) -> isize {
    ui.input_mut(|input| {
        let over = input.pointer.hover_pos().is_some_and(|point| {
            model.command_rect.is_some_and(|rect| rect.contains(point))
                || previous.is_some_and(|rect| rect.contains(point))
        });
        if !over {
            return 0;
        }
        let delta: f32 = input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                _ => None,
            })
            .sum();
        input.smooth_scroll_delta = egui::Vec2::ZERO;
        if delta > 0.0 {
            -1
        } else if delta < 0.0 {
            1
        } else {
            0
        }
    })
}

pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
        && model
            .command
            .chars()
            .nth(spelled(&model.command, &model.completion_prefix))
            != Some(' ')
        && ui.input(|input| {
            input
                .events
                .iter()
                .find_map(|event| match event {
                    egui::Event::Text(text) => Some(text),
                    _ => None,
                })
                .is_some_and(|text| text.starts_with(' '))
        })
    {
        command_cursor_end(ui.ctx(), id, &model.command);
        model.inline_suffix = false;
    }
}

pub(crate) fn keys(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    previous: Option<egui::Rect>,
    id: egui::Id,
) -> Keys {
    if model.focus_command || model.command_open {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
            model.focus_command = false;
        }
        model.command_open = true;
    }
    let wheel = wheel(ui, model, previous);
    if wheel != 0 {
        ui.memory_mut(|memory| memory.request_focus(id));
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if wheel != 0 {
        wheel
    } else if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                -1
            } else {
                0
            }
        })
    } else {
        0
    };
    let enter =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let tab =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
    let deletes = ui.input(|input| {
        input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)
    });
    suffix_space(ui, model, id, focused);
    Keys {
        browse,
        enter,
        tab,
        deletes,
    }
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    commands: &impl Commands,
    id: egui::Id,
    deletes: bool,
) {
    if response.gained_focus() {
        model.command_open = true;
    }
    let agent_edit = model.agent_edit.take();
    if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
        model.command.remove(0);
    }
    if response.changed() || agent_edit.is_some() {
        model.completion = 0;
        model.completion_visible = !model.command.is_empty();
        model.completion_prefix.clone_from(&model.command);
        model.inline_suffix = false;
        let at_end = egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .is_some_and(|range| {
                range.is_empty() && range.primary.index == model.command.chars().count()
            });
        if !deletes
            && model.options.is_empty()
            && agent_edit != Some(true)
            && at_end
            && !model.command.is_empty()
            && !model.command.ends_with(' ')
            && let Some(name) = commands.completions(&model.command).first()
        {
            let prefix = spelled(name, &commands.canonical(&model.command));
            if name.chars().count() > prefix {
                model.command = (*name).into();
                command_cursor_select(ui.ctx(), id, prefix, model.command.chars().count());
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model.completion_prefix.clone_from(&model.command);
    }
}

pub(crate) fn popup(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    choices: &[&'static str],
    response: &egui::Response,
    browse: isize,
) -> Option<&'static str> {
    let width = (ui.ctx().content_rect().right() - response.rect.left() - 12.0).clamp(60.0, 220.0);
    let height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
    let mut complete = None;
    let popup = egui::Area::new(egui::Id::new("command-completions"))
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    egui::ScrollArea::vertical()
                        .id_salt("command-choices")
                        .max_height(height)
                        .show(ui, |ui| {
                            for (index, name) in choices.iter().enumerate() {
                                let item = ui.selectable_label(index == model.completion, *name);
                                record(
                                    controls,
                                    &format!("command/completion/{name}"),
                                    name,
                                    &item,
                                );
                                if index == model.completion && browse != 0 {
                                    item.scroll_to_me(None);
                                }
                                if item.clicked() {
                                    complete = Some(*name);
                                }
                            }
                        });
                });
        });
    model.completion_rect = Some(popup.response.rect);
    complete
}

pub(crate) fn browse(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    commands: &impl Commands,
    response: &egui::Response,
    keys: &Keys,
    id: egui::Id,
) -> Option<&'static str> {
    let choices = commands.browse(&model.completion_prefix);
    let opening = !model.completion_visible;
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if !focused || !model.completion_visible || choices.is_empty() || !model.options.is_empty() {
        return None;
    }
    model.completion = model.completion.min(choices.len() - 1);
    if commands.choosing_option(&model.completion_prefix)
        && let Some(index) = choices
            .iter()
            .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
    {
        model.completion = index;
    }
    if keys.browse != 0 {
        model.completion = if opening && !commands.choosing_option(&model.completion_prefix) {
            if keys.browse < 0 {
                choices.len() - 1
            } else {
                0
            }
        } else {
            (model.completion as isize + keys.browse).rem_euclid(choices.len() as isize) as usize
        };
        model.command = choices[model.completion].into();
        let start = if model
            .command
            .to_ascii_lowercase()
            .starts_with(&model.completion_prefix.to_ascii_lowercase())
        {
            model.completion_prefix.chars().count()
        } else {
            0
        };
        command_cursor_select(ui.ctx(), id, start, model.command.chars().count());
        model.inline_suffix = true;
        ui.ctx().request_repaint();
    }
    let mut complete = if keys.tab {
        Some(choices[model.completion])
    } else {
        None
    };
    // the list shows the verbs, or the options of the verb typed
    complete = popup(ui, model, controls, &choices, response, keys.browse).or(complete);
    complete
}

pub(crate) fn finish(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    command: &mut Option<String>,
    commands: &impl Commands,
    complete: Option<&str>,
    keys: &Keys,
    escape_allowed: bool,
) -> bool {
    if let Some(name) = complete {
        let (text, run) = commands.accept(name);
        if run && !keys.tab {
            *command = Some(text);
            model.command.clear();
        } else {
            model.command = if run { format!("{text} ") } else { text };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
        let line = model
            .take_command()
            .unwrap_or_else(|| std::mem::take(&mut model.command));
        let (text, run) = commands.accept(&line);
        // a verb waiting for one of many options opens their list
        let waits = !run && commands.choosing_option(&text);
        if run {
            *command = Some(text);
        } else {
            model.completion_prefix.clone_from(&text);
            model.completion = 0;
            model.command = text;
        }
        model.completion_visible = waits;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused && ui.input(|input| input.key_pressed(egui::Key::Escape)) && escape_allowed {
        model.command.clear();
        model.completion_visible = false;
        model.inline_suffix = false;
        model.command_open = false;
        model.focus_command = false;
        response.surrender_focus();
        *command = Some("Escape".into());
        return true;
    }
    false
}

pub(crate) fn collapse(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
    let response = ui
        .button(if model.command_expanded { "–" } else { "+" })
        .on_hover_text("Collapse or expand history");
    record(
        controls,
        "command/collapse",
        "Collapse or expand history",
        &response,
    );
    if response.clicked() {
        model.command_expanded = !model.command_expanded;
        ui.ctx().request_repaint();
    }
}

pub(crate) fn row(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    previous: Option<egui::Rect>,
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    // as tall as the field, so the label and the typed text share one centre line
    let size = egui::vec2(ui.available_width(), 22.0);
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    ui.allocate_ui_with_layout(size, layout, |ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        let response = view::field(
            ui,
            &mut model.command,
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded),
            0.0,
            model.command_open,
        );
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
        let complete = browse(ui, model, controls, commands, &response, &keys, id);
        focus_canvas = finish(
            ui,
            model,
            &response,
            command,
            commands,
            complete,
            &keys,
            escape_allowed,
        );
        collapse(ui, model, controls);
    });
    focus_canvas
}

pub fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    escape_allowed: bool,
) -> bool {
    let previous = model.completion_rect.take();
    let mut focus_canvas = false;
    let panel = view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
        view::prepare(ui);
        model.dock_top = Some(ui.max_rect().top());
        history(ui, model, controls);
        focus_canvas = row(
            ui,
            model,
            controls,
            command,
            commands,
            previous,
            escape_allowed,
        );
    });
    if let Some(controls) = controls {
        let rect = panel.response.rect;
        controls.push(Control {
            key: "command/resize".into(),
            label: "Drag to resize command history".into(),
            rect: [
                rect.left(),
                rect.top() - 4.0,
                rect.right(),
                rect.top() + 4.0,
            ],
        });
    }
    focus_canvas
}
```

**Result after `03co-align`:** Type Help, then click the drawing. The Command label and field text share a centre line; the dock retains its current top edge.

---

<a id="checkpoint-03cp-options"></a>

### Step 23 of 25 · Reserve command-row space for active tool values (`03cp-options`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/command_dock/mod.rs,…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">command field reservati…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">option-owned pointer an…</text>
</svg>

Reserve measured space for active-tool values and keep their taps in the command dock. Empty options preserve ordinary command completion; interactive tools supply values later.

> **Check:** *Does clicking Create decide when geometry is committed?* — No. It returns an empty command to the active tool. That owner decides whether Enter keeps an asked value or creates geometry.

### 1. `src/command_dock/mod.rs`

Measure active value buttons, reserve field width and return their owned commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    focus_canvas
}

pub fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
```

Replace it with:

```rust
    focus_canvas
}

/// The running command's prompt and its options in it, as words to click: a value to change, Create or Cancel.
fn options(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
) {
    let prompt = model.drawing_prompt.trim_end_matches(" · Esc cancels");
    // the prompt gives way to the options and a field of at least 120 px
    let room = (ui.available_width() - options_width(ui, &model.options) - 150.0).max(60.0);
    ui.scope(|ui| {
        ui.set_max_width(room);
        // the prompt grey, the options in it black
        ui.add(egui::Label::new(egui::RichText::new(prompt).weak()).truncate())
            .on_hover_text(prompt);
    });
    model.options_rect = None;

    for (label, line) in &model.options {
        let response = ui
            .add(
                egui::Label::new(egui::RichText::new(label.as_str()).strong())
                    .sense(egui::Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        record(
            controls,
            &format!("command/option/{label}"),
            label,
            &response,
        );

        // hovered, the word is underlined as a link
        if response.hovered() {
            let rect = response.rect;
            let stroke = egui::Stroke::new(1.0_f32, ui.visuals().text_color());
            ui.painter()
                .hline(rect.x_range(), rect.bottom() - 1.0, stroke);
        }

        model.options_rect = Some(match model.options_rect {
            Some(rect) => rect.union(response.rect),
            None => response.rect,
        });

        // the field keeps the keyboard, so the value is typed straight away
        if response.clicked() {
            *command = Some(line.clone());
            model.command_open = true;
            model.focus_command = true;
        }
    }
}

/// The width the options take in the prompt.
fn options_width(ui: &egui::Ui, options: &[(String, String)]) -> f32 {
    let font = egui::FontId::proportional(14.0);
    options
        .iter()
        .map(|(label, _)| {
            let text = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(label.clone(), font.clone(), egui::Color32::BLACK)
            });
            text.size().x + ui.spacing().item_spacing.x
        })
        .sum()
}

pub fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
```

**Locate this block:**

```rust
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        let response = view::field(
            ui,
            &mut model.command,
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded),
            0.0,
            model.command_open,
        );
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
```

Replace it with:

```rust
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        // a command with options reads as its prompt with the options in it; the field takes the answer
        let hint = if model.options.is_empty() {
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded)
        } else {
            options(ui, model, controls, command);
            ""
        };
        let response = view::field(ui, &mut model.command, hint, 0.0, model.command_open);
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
```

**Locate this block:**

```rust
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if !focused || !model.completion_visible || choices.is_empty() || !model.options.is_empty() {
        return None;
    }
```

Replace it with:

```rust
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    // a running command takes values, not other commands
    if !focused || !model.completion_visible || choices.is_empty() || !model.options.is_empty() {
        return None;
    }
```

**Complete updated block reference:**

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,                  // command line shown
    pub(crate) command: String,                     // text in the command field
    pub(crate) drawing_prompt: String,              // prompt while drawing
    pub(crate) focus_command: bool,                 // give the field focus next frame
    pub(crate) dock_top: Option<f32>,               // the dock's top edge in CSS px, for fitting above it
    pub(crate) status: String,                      // status line text
    pub(crate) history: VecDeque<String>,           // past commands and answers
    pub(crate) command_expanded: bool,              // history shown above the field
    pub(crate) completion: usize,                   // highlighted completion index
    pub(crate) completion_prefix: String,           // text the completions match
    pub(crate) inline_suffix: bool,                 // completion suffix shown in the field
    pub(crate) completion_visible: bool,            // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>,    // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,            // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>,      // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>,    // where those buttons are, for taps
}

/// One clickable control and where it was drawn, for browser tests.
#[derive(serde::Serialize)]
pub struct Control {
    pub(crate) key: String,    // what it does
    pub(crate) label: String,  // text shown
    pub(crate) rect: [f32; 4], // left, top, right, bottom
}

/// Remember one control's rectangle, when inspecting.
pub(crate) fn record(
    controls: &mut Option<Vec<Control>>,
    key: &str,
    label: &str,
    response: &egui::Response,
) {
    let Some(controls) = controls.as_mut() else {
        return;
    };
    let r = response.rect;
    controls.push(Control {
        key: key.to_string(),
        label: label.to_string(),
        rect: [r.min.x, r.min.y, r.max.x, r.max.y],
    });
}

/// Put the caret at the end of the field.
pub(crate) fn command_cursor_end(context: &egui::Context, id: egui::Id, command: &str) {
    let end = command.chars().count();
    command_cursor_select(context, id, end, end);
}

/// The characters of `name` that spell `typed`, spaces aside: `orient3p` is `Orient 3 P`.
pub(crate) fn spelled(name: &str, typed: &str) -> usize {
    let typed = typed.chars().filter(|c| !c.is_whitespace()).count();
    let (mut prefix, mut letters) = (0, 0);

    for c in name.chars() {
        if letters == typed {
            break;
        }

        prefix += 1;
        letters += usize::from(!c.is_whitespace());
    }

    prefix
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(prompt: &'a str, status: &'a str, expanded: bool) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}

/// Select `start..end` in the field.
pub(crate) fn command_cursor_select(
    context: &egui::Context,
    id: egui::Id,
    start: usize,
    end: usize,
) {
    if let Some(mut state) = egui::TextEdit::load_state(context, id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(start),
                egui::text::CCursor::new(end),
            )));
        egui::TextEdit::store_state(context, id, state);
    }
}

/// Draw retained history above the field.
pub(crate) fn history(ui: &mut egui::Ui, model: &CommandLine, controls: &mut Option<Vec<Control>>) {
    if model.command_expanded {
        egui::ScrollArea::vertical()
            .id_salt("command-history")
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .min_scrolled_height(0.0)
            .max_height((ui.available_height() - 38.0).max(0.0))
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width());
                for text in &model.history {
                    ui.add(egui::Label::new(text).wrap());
                }
                if !model.status.is_empty()
                    && !model
                        .history
                        .back()
                        .is_some_and(|text| text.ends_with(&model.status))
                {
                    ui.label(&model.status);
                }
                if !model.drawing_prompt.is_empty() {
                    let response = ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                    record(controls, "command/hint", &model.drawing_prompt, &response);
                }
            });
        // divider between history and the field
        let (rect, _) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
        ui.painter().hline(
            ui.max_rect().x_range().expand(6.0),
            rect.center().y,
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
        );
    }
}

impl CommandLine {
    pub(crate) fn take_command(&mut self) -> Option<String> {
        if self.command.trim().is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.command))
    }

    pub(crate) fn remember(&mut self, line: String) {
        if self.history.len() == 200 {
            self.history.pop_front();
        }
        self.history.push_back(line);
    }
}

#[derive(Default)]
pub(crate) struct Keys {
    pub(crate) browse: isize,
    pub(crate) enter: bool,
    pub(crate) tab: bool,
    pub(crate) deletes: bool,
}

pub(crate) fn wheel(ui: &mut egui::Ui, model: &CommandLine, previous: Option<egui::Rect>) -> isize {
    ui.input_mut(|input| {
        let over = input.pointer.hover_pos().is_some_and(|point| {
            model.command_rect.is_some_and(|rect| rect.contains(point))
                || previous.is_some_and(|rect| rect.contains(point))
        });
        if !over {
            return 0;
        }
        let delta: f32 = input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::MouseWheel { delta, .. } => Some(delta.y),
                _ => None,
            })
            .sum();
        input.smooth_scroll_delta = egui::Vec2::ZERO;
        if delta > 0.0 {
            -1
        } else if delta < 0.0 {
            1
        } else {
            0
        }
    })
}

pub(crate) fn suffix_space(ui: &egui::Ui, model: &mut CommandLine, id: egui::Id, focused: bool) {
    if model.inline_suffix
        && focused
        && model
            .command
            .chars()
            .nth(spelled(&model.command, &model.completion_prefix))
            != Some(' ')
        && ui.input(|input| {
            input
                .events
                .iter()
                .find_map(|event| match event {
                    egui::Event::Text(text) => Some(text),
                    _ => None,
                })
                .is_some_and(|text| text.starts_with(' '))
        })
    {
        command_cursor_end(ui.ctx(), id, &model.command);
        model.inline_suffix = false;
    }
}

pub(crate) fn keys(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    previous: Option<egui::Rect>,
    id: egui::Id,
) -> Keys {
    if model.focus_command || model.command_open {
        ui.memory_mut(|memory| memory.request_focus(id));
        if model.focus_command {
            command_cursor_end(ui.ctx(), id, &model.command);
            model.focus_command = false;
        }
        model.command_open = true;
    }
    let wheel = wheel(ui, model, previous);
    if wheel != 0 {
        ui.memory_mut(|memory| memory.request_focus(id));
        model.command_open = true;
    }
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if wheel != 0 {
        wheel
    } else if focused {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                1
            } else if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                -1
            } else {
                0
            }
        })
    } else {
        0
    };
    let enter =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let tab =
        focused && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Tab));
    let deletes = ui.input(|input| {
        input.key_pressed(egui::Key::Backspace) || input.key_pressed(egui::Key::Delete)
    });
    suffix_space(ui, model, id, focused);
    Keys {
        browse,
        enter,
        tab,
        deletes,
    }
}

pub(crate) fn refresh(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    commands: &impl Commands,
    id: egui::Id,
    deletes: bool,
) {
    if response.gained_focus() {
        model.command_open = true;
    }
    let agent_edit = model.agent_edit.take();
    if (response.changed() || agent_edit.is_some()) && model.command.starts_with(':') {
        model.command.remove(0);
    }
    if response.changed() || agent_edit.is_some() {
        model.completion = 0;
        model.completion_visible = !model.command.is_empty();
        model.completion_prefix.clone_from(&model.command);
        model.inline_suffix = false;
        let at_end = egui::TextEdit::load_state(ui.ctx(), id)
            .and_then(|state| state.cursor.char_range())
            .is_some_and(|range| {
                range.is_empty() && range.primary.index == model.command.chars().count()
            });
        if !deletes
            && model.options.is_empty()
            && agent_edit != Some(true)
            && at_end
            && !model.command.is_empty()
            && !model.command.ends_with(' ')
            && let Some(name) = commands.completions(&model.command).first()
        {
            let prefix = spelled(name, &commands.canonical(&model.command));
            if name.chars().count() > prefix {
                model.command = (*name).into();
                command_cursor_select(ui.ctx(), id, prefix, model.command.chars().count());
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model.completion_prefix.clone_from(&model.command);
    }
}

pub(crate) fn popup(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    choices: &[&'static str],
    response: &egui::Response,
    browse: isize,
) -> Option<&'static str> {
    let width = (ui.ctx().content_rect().right() - response.rect.left() - 12.0).clamp(60.0, 220.0);
    let height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
    let mut complete = None;
    let popup = egui::Area::new(egui::Id::new("command-completions"))
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(response.rect.left(), response.rect.top()))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_gray(215)))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    egui::ScrollArea::vertical()
                        .id_salt("command-choices")
                        .max_height(height)
                        .show(ui, |ui| {
                            for (index, name) in choices.iter().enumerate() {
                                let item = ui.selectable_label(index == model.completion, *name);
                                record(
                                    controls,
                                    &format!("command/completion/{name}"),
                                    name,
                                    &item,
                                );
                                if index == model.completion && browse != 0 {
                                    item.scroll_to_me(None);
                                }
                                if item.clicked() {
                                    complete = Some(*name);
                                }
                            }
                        });
                });
        });
    model.completion_rect = Some(popup.response.rect);
    complete
}

pub(crate) fn browse(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    commands: &impl Commands,
    response: &egui::Response,
    keys: &Keys,
    id: egui::Id,
) -> Option<&'static str> {
    let choices = commands.browse(&model.completion_prefix);
    let opening = !model.completion_visible;
    if keys.browse != 0 || keys.tab {
        model.completion_visible = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    // a running command takes values, not other commands
    if !focused || !model.completion_visible || choices.is_empty() || !model.options.is_empty() {
        return None;
    }
    model.completion = model.completion.min(choices.len() - 1);
    if commands.choosing_option(&model.completion_prefix)
        && let Some(index) = choices
            .iter()
            .position(|name| name.eq_ignore_ascii_case(model.command.trim()))
    {
        model.completion = index;
    }
    if keys.browse != 0 {
        model.completion = if opening && !commands.choosing_option(&model.completion_prefix) {
            if keys.browse < 0 {
                choices.len() - 1
            } else {
                0
            }
        } else {
            (model.completion as isize + keys.browse).rem_euclid(choices.len() as isize) as usize
        };
        model.command = choices[model.completion].into();
        let start = if model
            .command
            .to_ascii_lowercase()
            .starts_with(&model.completion_prefix.to_ascii_lowercase())
        {
            model.completion_prefix.chars().count()
        } else {
            0
        };
        command_cursor_select(ui.ctx(), id, start, model.command.chars().count());
        model.inline_suffix = true;
        ui.ctx().request_repaint();
    }
    let mut complete = if keys.tab {
        Some(choices[model.completion])
    } else {
        None
    };
    // the list shows the verbs, or the options of the verb typed
    complete = popup(ui, model, controls, &choices, response, keys.browse).or(complete);
    complete
}

pub(crate) fn finish(
    ui: &egui::Ui,
    model: &mut CommandLine,
    response: &egui::Response,
    command: &mut Option<String>,
    commands: &impl Commands,
    complete: Option<&str>,
    keys: &Keys,
    escape_allowed: bool,
) -> bool {
    if let Some(name) = complete {
        let (text, run) = commands.accept(name);
        if run && !keys.tab {
            *command = Some(text);
            model.command.clear();
        } else {
            model.command = if run { format!("{text} ") } else { text };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty()) {
        let line = model
            .take_command()
            .unwrap_or_else(|| std::mem::take(&mut model.command));
        let (text, run) = commands.accept(&line);
        // a verb waiting for one of many options opens their list
        let waits = !run && commands.choosing_option(&text);
        if run {
            *command = Some(text);
        } else {
            model.completion_prefix.clone_from(&text);
            model.completion = 0;
            model.command = text;
        }
        model.completion_visible = waits;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if focused && ui.input(|input| input.key_pressed(egui::Key::Escape)) && escape_allowed {
        model.command.clear();
        model.completion_visible = false;
        model.inline_suffix = false;
        model.command_open = false;
        model.focus_command = false;
        response.surrender_focus();
        *command = Some("Escape".into());
        return true;
    }
    false
}

pub(crate) fn collapse(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
    let response = ui
        .button(if model.command_expanded { "–" } else { "+" })
        .on_hover_text("Collapse or expand history");
    record(
        controls,
        "command/collapse",
        "Collapse or expand history",
        &response,
    );
    if response.clicked() {
        model.command_expanded = !model.command_expanded;
        ui.ctx().request_repaint();
    }
}

pub(crate) fn row(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    previous: Option<egui::Rect>,
    escape_allowed: bool,
) -> bool {
    let mut focus_canvas = false;
    // as tall as the field, so the label and the typed text share one centre line
    let size = egui::vec2(ui.available_width(), 22.0);
    let layout = egui::Layout::left_to_right(egui::Align::Center);
    ui.allocate_ui_with_layout(size, layout, |ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        // a command with options reads as its prompt with the options in it; the field takes the answer
        let hint = if model.options.is_empty() {
            placeholder(&model.drawing_prompt, &model.status, model.command_expanded)
        } else {
            options(ui, model, controls, command);
            ""
        };
        let response = view::field(ui, &mut model.command, hint, 0.0, model.command_open);
        model.command_rect = Some(response.rect);
        record(controls, "command/input", "Command", &response);
        refresh(ui, model, &response, commands, id, keys.deletes);
        let complete = browse(ui, model, controls, commands, &response, &keys, id);
        focus_canvas = finish(
            ui,
            model,
            &response,
            command,
            commands,
            complete,
            &keys,
            escape_allowed,
        );
        collapse(ui, model, controls);
    });
    focus_canvas
}

/// The running command's prompt and its options in it, as words to click: a value to change, Create or Cancel.
fn options(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
) {
    let prompt = model.drawing_prompt.trim_end_matches(" · Esc cancels");
    // the prompt gives way to the options and a field of at least 120 px
    let room = (ui.available_width() - options_width(ui, &model.options) - 150.0).max(60.0);
    ui.scope(|ui| {
        ui.set_max_width(room);
        // the prompt grey, the options in it black
        ui.add(egui::Label::new(egui::RichText::new(prompt).weak()).truncate())
            .on_hover_text(prompt);
    });
    model.options_rect = None;

    for (label, line) in &model.options {
        let response = ui
            .add(
                egui::Label::new(egui::RichText::new(label.as_str()).strong())
                    .sense(egui::Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        record(
            controls,
            &format!("command/option/{label}"),
            label,
            &response,
        );

        // hovered, the word is underlined as a link
        if response.hovered() {
            let rect = response.rect;
            let stroke = egui::Stroke::new(1.0_f32, ui.visuals().text_color());
            ui.painter()
                .hline(rect.x_range(), rect.bottom() - 1.0, stroke);
        }

        model.options_rect = Some(match model.options_rect {
            Some(rect) => rect.union(response.rect),
            None => response.rect,
        });

        // the field keeps the keyboard, so the value is typed straight away
        if response.clicked() {
            *command = Some(line.clone());
            model.command_open = true;
            model.focus_command = true;
        }
    }
}

/// The width the options take in the prompt.
fn options_width(ui: &egui::Ui, options: &[(String, String)]) -> f32 {
    let font = egui::FontId::proportional(14.0);
    options
        .iter()
        .map(|(label, _)| {
            let text = ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(label.clone(), font.clone(), egui::Color32::BLACK)
            });
            text.size().x + ui.spacing().item_spacing.x
        })
        .sum()
}

pub fn draw(
    root: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    command: &mut Option<String>,
    commands: &impl Commands,
    escape_allowed: bool,
) -> bool {
    let previous = model.completion_rect.take();
    let mut focus_canvas = false;
    let panel = view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
        view::prepare(ui);
        model.dock_top = Some(ui.max_rect().top());
        history(ui, model, controls);
        focus_canvas = row(
            ui,
            model,
            controls,
            command,
            commands,
            previous,
            escape_allowed,
        );
    });
    if let Some(controls) = controls {
        let rect = panel.response.rect;
        controls.push(Control {
            key: "command/resize".into(),
            label: "Drag to resize command history".into(),
            rect: [
                rect.left(),
                rect.top() - 4.0,
                rect.right(),
                rect.top() + 4.0,
            ],
        });
    }
    focus_canvas
}
```

### 2. `src/lib.rs`

Compile the shared dock for native widget checks.

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/03cp-options-04.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/command_dock/tool_buttons_tests.rs`

Copy actual value-button, focus, text, completion and Enter checks.

Copy this supplied file:

```text
--8<-- "journey/code/03cp-options-05.rs"
```

**Result after `03cp-options`:** Type Help and fold the command history. This preparation keeps the ordinary command row; value buttons appear when an active tool supplies options in the later tool lesson. The optional supplied widget fixture records button, typed-value and Enter behavior.

---

<a id="checkpoint-03cn-handoff"></a>

### Step 24 of 25 · Return accepted application commands (`03cn-handoff`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs, src/panel…</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep editing local and…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Keep editing local and…</text>
</svg>

Panel.update prepares input and returns an accepted application name. Help and unknown-command replies stay local; the caller will handle scene actions.

> **Check:** *Why return Option<String>?* — Most input edits the field; only an accepted application command needs dispatch.

### 1. `src/browser.rs`

Prepare the initial dock before presenting. Use Panel.update for input and the final layout pass. Remove inspection writes now handled inside update. Report that Panel returns application commands.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options("wheel", redraw.as_ref().unchecked_ref(), &options)?;
    redraw.forget();
    report("The shared production dock is drawn.");
    Ok(())
}
```

Replace it with:

```rust
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options("wheel", redraw.as_ref().unchecked_ref(), &options)?;
    redraw.forget();
    report("Panel returns accepted application commands.");
    Ok(())
}
```

**Locate this block:**

```rust
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { panel.key(key); }
        if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { panel.wheel(wheel, &input_canvas); }
        if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() { panel.pointer(pointer, &input_canvas); }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command-ui", &panel.inspect()) {
                report(&format!("Cannot inspect dock: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-command", panel.command()) {
                report(&format!("Cannot inspect text: {error:?}"));
            }
            if let Err(error) = input_canvas.set_attribute("data-history", &serde_json::to_string(panel.history()).unwrap()) {
                report(&format!("Cannot inspect history: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
```

Replace it with:

```rust
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Err(error) = panel.update(Some(&event), &input_canvas) {
            report(&format!("Cannot read command: {error:?}"));
        }
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
        }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
```

**Complete updated block reference:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas").ok_or("Missing canvas")?.dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default())
        .await.map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface.get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        if let Err(error) = panel.update(Some(&event), &input_canvas) {
            report(&format!("Cannot read command: {error:?}"));
        }
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
        }
        {
            if let Err(error) = present(&surface, &renderer, &mut panel) {
                report(&format!("Cannot redraw: {error:?}"));
            }
        }
    });
    for name in ["keydown", "keyup", "pointerdown", "pointermove", "pointerup", "pointercancel"] {
        window.add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options("wheel", redraw.as_ref().unchecked_ref(), &options)?;
    redraw.forget();
    report("Panel returns accepted application commands.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer, panel: &mut crate::panel::Panel) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => return Err(JsValue::from_str(&format!("Surface unavailable: {other:?}"))),
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

### 2. `src/panel.rs`

Remove replies now handled by Panel.update. Expose whether the dock consumed this event. Initialize event consumption. Start with folded history and focus the command field. Answer local commands; return accepted application names from update. Allow prepare to set viewport input scale. Supply the physical pixel scale to egui. Move command answers out of prepare. Return the accepted line from prepare. Paint prepared output without replaying input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(line) = line { if line != "Escape" { self.commands.reply(&mut self.model, &line); } }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [target.texture().width(), target.texture().height()];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
```

Replace it with:

```rust
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
        line
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
```

**Locate this block:**

```rust
            )),
            ..Default::default()
        };
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
```

Replace it with:

```rust
            )),
            ..Default::default()
        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point = Some(self.screen.pixels_per_point);
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
```

**Locate this block:**

```rust
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
```

Replace it with:

```rust
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        self.model.remember(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let rect = canvas.get_bounding_client_rect();
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / rect.width() as f32;
        self.consumed = false;
        if let Some(event) = event {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { self.consumed = self.key(key); }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new(); options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { self.consumed = self.wheel(wheel, canvas); }
        }
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else { return Ok(None); };
        if line == "Escape" { return Ok(None); }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self.commands.0.iter()
            .any(|name| name.eq_ignore_ascii_case(&line))
        {
            self.answer(&line, "Unknown command. Type Help.");
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line.to_ascii_lowercase()))
    }

    fn prepare(&mut self) -> Option<String> {
        let size = self.screen.size_in_pixels;
        let mut input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
```

**Locate this block:**

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            command_expanded: true,
            history: ["Command history lives here.".into()].into(),
            ..Default::default()
        } }
    }
```

Replace it with:

```rust
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), consumed: false, pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            focus_command: true,
            ..Default::default()
        } }
    }
```

**Locate this block:**

```rust
    }
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") { self.0.join(" · ") }
            else { "Unknown command. Type Help.".into() };
        model.status = message;
        model.remember(format!("> {line}\n{}", model.status));
    }
}

pub struct Panel {
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
```

Replace it with:

```rust
    }
}

pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
```

**Complete updated block reference:**

```rust
use crate::command_dock::{self, CommandLine, Commands as _, theme};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), consumed: false, pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            focus_command: true,
            ..Default::default()
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn wheel(&mut self, event: &web_sys::WheelEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        self.events.push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        if over { event.prevent_default(); }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        self.model.remember(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let rect = canvas.get_bounding_client_rect();
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / rect.width() as f32;
        self.consumed = false;
        if let Some(event) = event {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { self.consumed = self.key(key); }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new(); options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { self.consumed = self.wheel(wheel, canvas); }
        }
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else { return Ok(None); };
        if line == "Escape" { return Ok(None); }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self.commands.0.iter()
            .any(|name| name.eq_ignore_ascii_case(&line))
        {
            self.answer(&line, "Unknown command. Type Help.");
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line.to_ascii_lowercase()))
    }

    fn prepare(&mut self) -> Option<String> {
        let size = self.screen.size_in_pixels;
        let mut input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point = Some(self.screen.pixels_per_point);
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(root, &mut self.model, &mut self.controls, &mut line, &self.commands, true);
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
        line
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

**Result after `03cn-handoff`:** Type Help and press Enter. The field clears and exactly one answer appears.

---

<a id="checkpoint-03d-input"></a>

### Step 25 of 25 · Hand commands to the application (`03d-input`)

<svg xmlns="http://www.w3.org/2000/svg" width="840" height="64" viewBox="0 0 840 64" style="width:100%;max-width:840px;height:auto;display:block;margin:14px 0;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
    <pattern id="dots" width="8" height="8" patternUnits="userSpaceOnUse">
      <circle cx="2" cy="2" r="1" fill="#111111"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="836" height="60" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="136" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">src/browser.rs</text>
  <line x1="256" y1="32" x2="294" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="300" y="12" width="240" height="40" rx="6" fill="url(#hatch)" stroke="#111111" stroke-width="1.5"/>
  <text x="420" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Return accepted command…</text>
  <line x1="540" y1="32" x2="578" y2="32" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="584" y="12" width="240" height="40" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="704" y="36" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Return accepted command…</text>
</svg>

Listen on the canvas and hand events to Panel.update. Prepare the dock again after input, then paint one frame. Typing after a scene click focuses the field immediately.

> **Check:** *Why prepare again before painting?* — Input can change layout or history; the final frame must show the updated model.

### 1. `src/browser.rs`

Import the retained Closure type. Keep canvas acquisition ready for canvas-owned input. Keep GPU setup with the final browser event owner. Remove the intermediate event callback setup. Keep surface configuration before the canvas input owner. Handle command update errors in the canvas callback. Stop the callback after a layout failure. Present after preparing the final command layout. Register drawing and keyboard events on the canvas. Retain the wheel callback for the page lifetime. Give surface presentation its complete function. Return a useful error when the surface is unavailable.

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(status) = document.get_element_by_id("status") {
            status.set_text_content(Some(message));
            if message.starts_with("Cannot") { let _ = status.remove_attribute("hidden"); }
        }
    }
}

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No browser window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    device.on_uncaptured_error(std::sync::Arc::new(|error| report(&error.to_string())));
    let limit = device.limits().max_texture_dimension_2d;
    let width = (window.inner_width()?.as_f64().ok_or("No window width")? as u32).clamp(1, limit);
    let height = (window.inner_height()?.as_f64().ok_or("No window height")? as u32).clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let mut config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No compatible surface format")?;
    config.view_formats = vec![config.format.add_srgb_suffix()];
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue, config.format.add_srgb_suffix());
    let mut panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix(), &["Help"]);
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let _line = match panel.update(Some(&event), &input_canvas) {
            Ok(line) => line,
            Err(error) => {
                report(&format!("Cannot read command: {error:?}"));
                return;
            }
        };
        if let Err(error) = panel.update(None, &input_canvas) {
            report(&format!("Cannot lay out commands: {error:?}"));
            return;
        }
        if let Err(error) = present(&surface, &renderer, &mut panel) {
            report(&format!("Cannot redraw: {error:?}"));
        }
    });
    for name in [
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
        "lostpointercapture",
        "contextmenu",
        "keydown",
        "keyup",
        "blur",
    ] {
        canvas.add_event_listener_with_callback(name, click.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        click.as_ref().unchecked_ref(),
        &options,
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
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(frame.texture.format().add_srgb_suffix()),
        ..Default::default()
    });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
}
```

**Result after `03d-input`:** Click the scene, type Help and press Enter. One answer appears in the real dock.

---

## 3. Build, Verify & Save

```sh
npm --prefix ../session_tests run course -- dependencies 1.3
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 1.3
npm --prefix ../session_tests run course -- save 1.3
```

Open `http://127.0.0.1:8780/` to inspect the live WebGPU canvas, or use [checkpoint recovery](../recovery.md).

Continue with [1.4 · Commands that change the view](04-input.md).
