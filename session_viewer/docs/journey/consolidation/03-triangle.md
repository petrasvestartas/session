# 1.3 · Shaders and the command dock

**Steps:** 26 · **Source:** [`lib.rs`](../../../src/lib.rs) · [`engine/pipelines/mod.rs`](../../../src/engine/pipelines/mod.rs) · [`command_dock/mod.rs`](../../../src/command_dock/mod.rs) · [`view.rs`](../../../src/command_dock/view.rs)

## <a id="checkpoint-03-triangle"></a>1. Give the GPU three corners

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-503a46c5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-503a46c5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-503a46c5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-503a46c5" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-503a46c5" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">03-TRIANGLE · WGSL PIPELINE</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Procedural @vertex Clip-Space Corners &amp; @fragment Rasterization</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">NDC ∈ [-1, +1]² · draw(0..3, 0..1)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-503a46c5)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · @vertex fn vertex(@builtin(vertex_index) i: u32)</text>
  <rect x="64" y="68" width="244" height="122" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="64" y1="129" x2="308" y2="129" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="186" y1="68" x2="186" y2="190" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="3 3"/>
  <polygon points="186,82 114,172 258,172" fill="#fce7f3" fill-opacity="0.6" stroke="#111111" stroke-width="1.75"/>
  <circle cx="186" cy="82" r="4.5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="77" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">i=0: (0.0, 0.6)</text>
  <circle cx="114" cy="172" r="4.5" fill="#16a34a"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="106" y="185" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">i=1: (-0.6,-0.6)</text>
  <circle cx="258" cy="172" r="4.5" fill="#2563eb"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="266" y="185" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">i=2: (0.6,-0.6)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-503a46c5)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-503a46c5)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · RenderPipeline &amp; Barycentric Coverage</text>
  <polygon points="560,76 472,172 648,172" fill="#fbcfe8" stroke="#111111" stroke-width="1.75"/>
  <line x1="560" y1="76" x2="560" y2="140" stroke="#dc2626" stroke-width="1.2" stroke-dasharray="3 2"/>
  <line x1="472" y1="172" x2="560" y2="140" stroke="#16a34a" stroke-width="1.2" stroke-dasharray="3 2"/>
  <line x1="648" y1="172" x2="560" y2="140" stroke="#2563eb" stroke-width="1.2" stroke-dasharray="3 2"/>
  <circle cx="560" cy="140" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">P = λ₀v₀ + λ₁v₁ + λ₂v₂</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="186" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#db2777">@fragment -&gt; vec4(0.95, 0.35, 0.6, 1.0)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-503a46c5)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-503a46c5)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Pass: set_pipeline + draw(0..3, 0..1)</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,82 862,172 1006,172" fill="#f25999" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="152" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff">Pink Triangle</text>
</svg>
</div>

### 1. `src/triangle.wgsl`

Create the file and type:

```wgsl
@vertex
fn vertex(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let corners = array<vec2<f32>, 3>(
        vec2<f32>(-0.6, -0.5),
        vec2<f32>(0.6, -0.5),
        vec2<f32>(0.0, 0.6),
    );
    return vec4<f32>(corners[index], 0.0, 1.0);
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.9, 0.25, 0.45, 1.0);
}
```

### 2. `src/renderer.rs`

**Locate this existing block:**

```rust
    pub queue: wgpu::Queue,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }

    pub fn draw(&self, view: &wgpu::TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                    },
                })],
                ..Default::default()
            });
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
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
        Self {
            device,
            queue,
            pipeline,
        }
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
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.draw(0..3, 0..1);
        }
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    present(&surface, &renderer)?;
    report("The GPU painted the canvas.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    surface.configure(&device, &config);
    let renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
    );
    present(&surface, &renderer)?;
    report("Three corners became a triangle.");
    Ok(())
```

## <a id="checkpoint-03a-fonts"></a>2. Prepare the command fonts and painter

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-8d115a62" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-8d115a62" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-8d115a62" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-8d115a62" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-8d115a62" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">03A-PAINT · ATLAS + PASS 2</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">TTF Glyph Rasterization, R8Unorm Texture Upload &amp; Pass 2 LoadOp::Load Overlay</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Pass 1 (Clear) -&gt; Pass 2 (Load + Alpha)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-8d115a62)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Embedded TTF -&gt; Shelf Glyph Atlas</text>
  <rect x="48" y="72" width="276" height="112" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="62" y="84" width="38" height="40" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="81" y="110" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">C</text>
  <rect x="108" y="84" width="38" height="40" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="127" y="110" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">o</text>
  <rect x="154" y="84" width="38" height="40" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="173" y="110" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">m</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="148" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">queue.write_texture(R8Unorm)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="168" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">uv_rect = (u0, v0, u1, v1)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-8d115a62)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-8d115a62)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Two-Pass Frame Recording</text>
  <rect x="414" y="74" width="132" height="108" rx="4" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="480" y="94" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Scene</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="480" y="112" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#db2777">LoadOp::Clear(White)</text>
  <polygon points="480,122 448,168 512,168" fill="#f25999" stroke="#111111" stroke-width="1.2"/>
  <line x1="546" y1="128" x2="572" y2="128" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-8d115a62)"/>
  <rect x="576" y="74" width="132" height="108" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="642" y="94" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Dock</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="642" y="112" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">LoadOp::Load (Keep!)</text>
  <rect x="590" y="152" width="104" height="20" rx="2" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="642" y="166" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#111111">Command: _</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-8d115a62)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-8d115a62)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Composited Scene + GPU Text Overlay</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,78 874,152 994,152" fill="#f25999" stroke="#111111" stroke-width="1.5"/>
  <rect x="818" y="158" width="232" height="24" rx="3" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="830" y="174" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Command: Ready</text>
</svg>
</div>

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wgpu = "=29.0.4"

[dev-dependencies]
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wgpu = "=29.0.4"

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

[dev-dependencies]
```

### 2. `src/panel.rs`

Create the file and type:

```rust
use crate::command_dock::theme;
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
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
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter }
    }
}
```

### 3. `src/browser.rs`

**Locate this existing block:**

```rust
    );
    present(&surface, &renderer)?;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    let _panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer)?;
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
    });
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    });
}

#[cfg(target_arch = "wasm32")]
mod command_dock;
#[cfg(target_arch = "wasm32")]
mod panel;
```

### 5. `src/command_dock/theme.rs`

Create the file and type:

```rust
/// The panel fonts: the label fonts, Noto Sans first, then its symbol fallbacks.
pub(crate) fn fonts(faces: [&'static [u8]; 3]) -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::empty();
    let names = [
        "Noto Sans",
        "Noto Sans Symbols 2",
        "Noto Sans Symbols",
    ];

    for (name, bytes) in names.into_iter().zip(faces) {
        let data = std::sync::Arc::new(egui::FontData::from_static(bytes));
        fonts.font_data.insert(name.to_owned(), data);

        for family in [
            egui::FontFamily::Proportional,
            egui::FontFamily::Monospace,
        ] {
            fonts
                .families
                .entry(family)
                .or_default()
                .push(name.to_owned());
        }
    }

    fonts
}

/// The white theme.
pub(crate) fn visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    visuals.override_text_color = Some(egui::Color32::BLACK);
    visuals.window_fill = egui::Color32::WHITE;
    visuals.panel_fill = egui::Color32::from_gray(245);
    visuals.window_stroke = egui::Stroke::NONE;
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.text_cursor.blink = false; // frames are drawn on demand
    visuals.indent_has_left_vline = false;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        widget.bg_stroke = egui::Stroke::NONE;
        widget.bg_fill = egui::Color32::from_gray(245);
        widget.weak_bg_fill = egui::Color32::WHITE;
        widget.fg_stroke.color = egui::Color32::BLACK;
    }

    visuals
}
```

### 6. `src/command_dock/mod.rs`

Create the file and type:

```rust
pub(crate) mod theme;
```

## <a id="checkpoint-03a-paint"></a>3. Paint command text over the scene

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

### 1. `src/panel.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            egui::Panel::bottom("command-line-collapsed")
                .exact_size(30.0)
                .frame(
                    egui::Frame::new()
                        .fill(egui::Color32::WHITE)
                        .inner_margin(egui::Margin::symmetric(6, 4)),
                )
                .show_inside(root, |ui| {
                    ui.label("Command:");
                });
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter
                .update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer
            .device
            .create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device,
            &renderer.queue,
            &mut encoder,
            &jobs,
            &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        self.painter
            .render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free {
            self.painter.free_texture(&id);
        }
    }
}
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    );
    let _panel = crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer)?;
    report("Three corners became a triangle.");
    Ok(())
}

fn present(surface: &wgpu::Surface<'_>, renderer: &Renderer) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame)
        | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        other => {
            return Err(JsValue::from_str(&format!(
                "Surface unavailable: {other:?}"
            )));
        }
    };
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor {
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view);
    frame.present();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    let mut panel =
        crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    report("Our command line is drawn. Input comes next.");
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
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor {
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view);
    panel.draw(renderer, &view);
    frame.present();
```

## <a id="checkpoint-03a-panel"></a>4. Lay out the command field

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-69cc9127" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-69cc9127" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-69cc9127" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-69cc9127" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-69cc9127" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">03A-PANEL · SCREEN QUADS</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen-Space Command Dock Layout, Margins &amp; Pixel-to-NDC Quad Conversion</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">x_ndc = 2*x_px/W - 1 · y_ndc = 1 - 2*y_px/H</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-69cc9127)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Viewport Bottom-Anchored Dock Rects</text>
  <rect x="48" y="70" width="276" height="118" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="60" y="96" width="252" height="44" rx="3" fill="#f8fafc" stroke="#2563eb" stroke-width="1.5" stroke-dasharray="4 2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">History Transcript Rows</text>
  <rect x="60" y="146" width="252" height="32" rx="3" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="140" y="166" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Command: input█</text>
  <rect x="236" y="151" width="68" height="22" rx="2" fill="#dcfce7" stroke="#16a34a" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="270" y="166" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">Status</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-69cc9127)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-69cc9127)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · UTF-8 Byte Cursor &amp; Edit Buffer</text>
  <rect x="416" y="78" width="288" height="44" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="424" y="84" width="44" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="446" y="104" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">H</text>
  <rect x="472" y="84" width="44" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="494" y="104" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">e</text>
  <rect x="520" y="84" width="44" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="542" y="104" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">l</text>
  <rect x="568" y="84" width="44" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="590" y="104" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">p</text>
  <line x1="616" y1="80" x2="616" y2="120" stroke="#dc2626" stroke-width="3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="148" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">cursor: char_boundary_index</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="170" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">Left / Right / Backspace / Enter</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-69cc9127)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-69cc9127)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Autocomplete Popup &amp; Hit Rects</text>
  <rect x="794" y="72" width="280" height="68" rx="4" fill="#ffffff" stroke="#2563eb" stroke-width="1.75"/>
  <rect x="802" y="78" width="264" height="24" rx="2" fill="#dbeafe"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="814" y="94" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">▸ Clear White</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="814" y="122" font-family="monospace" font-size="9.5" fill="#111111">  Clear Grey</text>
  <rect x="794" y="148" width="280" height="36" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="810" y="170" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Command: Clear█</text>
</svg>
</div>

### 1. `src/panel.rs`

**Locate this existing block:**

```rust
use crate::command_dock::theme;
use crate::renderer::Renderer;

pub struct Panel {
    context: egui::Context,
    painter: egui_wgpu::Renderer,
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
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            egui::Panel::bottom("command-line-collapsed")
                .exact_size(30.0)
                .frame(
                    egui::Frame::new()
                        .fill(egui::Color32::WHITE)
                        .inner_margin(egui::Margin::symmetric(6, 4)),
                )
                .show_inside(root, |ui| {
                    ui.label("Command:");
                });
        });
```

Edit the existing block to match this code; keep unchanged lines:

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
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            line: String::new(),
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(
                        ui,
                        &mut self.line,
                        "Type a command",
                        0.0,
                        false,
                    );
                    let _ = ui
                        .button("+")
                        .on_hover_text("Collapse or expand history");
                });
            });
        });
```

### 2. `src/command_dock/theme.rs`

**Locate this existing block:**

```rust
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.text_cursor.blink = false; // frames are drawn on demand
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    visuals.extreme_bg_color = egui::Color32::WHITE;
    visuals.selection.bg_fill = egui::Color32::from_rgb(200, 222, 245);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, egui::Color32::BLACK);
    visuals.text_cursor.stroke = egui::Stroke::new(1.5_f32, egui::Color32::BLACK);
    visuals.text_cursor.blink = false; // frames are drawn on demand
```

### 3. `src/command_dock/mod.rs`

Replace the entire file with:

```rust
pub(crate) mod theme;
pub(crate) mod view;
```

### 4. `src/command_dock/view.rs`

Create the file and type:

```rust
/// The production command panel: folded input or resizable history.
pub(crate) fn panel(root: &egui::Ui, expanded: bool, extra: f32) -> egui::Panel {
    let panel = if !expanded {
        egui::Panel::bottom("command-line-collapsed").exact_size(30.0 + extra)
    } else {
        egui::Panel::bottom("command-line")
            .default_size(104.0)
            .resizable(true)
            .size_range(
                (64.0 + extra)..=(root.available_height() * 0.75).max(104.0 + extra),
            )
    };
    panel.show_separator_line(false).frame(
        egui::Frame::new()
            .fill(egui::Color32::WHITE)
            .inner_margin(egui::Margin::symmetric(6, 4)),
    )
}

/// Set the dock's spacing, type size and top rule.
pub(crate) fn prepare(ui: &mut egui::Ui) {
    ui.set_min_height(ui.max_rect().height());
    ui.painter().hline(
        ui.max_rect().x_range().expand(6.0),
        ui.max_rect().top() - 4.0,
        egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
    );
    ui.set_clip_rect(ui.max_rect().expand(6.0));
    ui.style_mut().override_font_id = Some(egui::FontId::proportional(14.0));
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
}

/// Draw the same text field for the introductory and complete command handlers.
pub(crate) fn field(
    ui: &mut egui::Ui,
    command: &mut String,
    hint: &str,
    option_width: f32,
    focused: bool,
) -> egui::Response {
    let id = egui::Id::new("command-input");
    let response = ui.add_sized(
        [
            (ui.available_width() - option_width - 28.0).max(40.0),
            22.0,
        ],
        egui::TextEdit::singleline(command)
            .id(id)
            .font(egui::FontId::proportional(14.0))
            .vertical_align(egui::Align::Center)
            .frame(egui::Frame::NONE)
            .clip_text(true)
            .char_limit(2048)
            .hint_text(hint),
    );
    // caret visible on an empty field
    if focused && command.is_empty() {
        let y = response.rect.center().y;
        ui.painter().vline(
            response.rect.left(),
            y - 7.0..=y + 7.0,
            ui.visuals().text_cursor.stroke,
        );
    }
    response
}
```

## <a id="checkpoint-03b-memory"></a>5. Give the command field its memory

### 1. `src/panel.rs`

**Locate this existing block:**

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
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            line: String::new(),
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(
                        ui,
                        &mut self.line,
                        "Type a command",
                        0.0,
```

Edit the existing block to match this code; keep unchanged lines:

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
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            model: CommandLine {
                status: "The field now belongs to CommandLine.".into(),
                ..Default::default()
            },
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(
                        ui,
                        &mut self.model.command,
                        placeholder("", &self.model.status, false),
                        0.0,
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    present(&surface, &renderer, &mut panel)?;
    report("Our command line is drawn. Input comes next.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    present(&surface, &renderer, &mut panel)?;
    report("The command model owns the text and history.");
    Ok(())
```

### 3. `src/command_dock/mod.rs`

Replace the entire file with:

```rust
use std::collections::VecDeque;

pub(crate) mod theme;
pub(crate) mod view;

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,        // command line shown
    pub(crate) command: String,           // text in the command field
    pub(crate) drawing_prompt: String,    // prompt while drawing
    pub(crate) focus_command: bool,       // give the field focus next frame
    pub(crate) status: String,            // status line text
    pub(crate) history: VecDeque<String>, // past commands and answers
    pub(crate) command_expanded: bool,    // history shown above the field
    pub(crate) completion: usize,         // highlighted completion index
    pub(crate) completion_prefix: String, // text the completions match
    pub(crate) inline_suffix: bool,       // completion suffix shown in the field
    pub(crate) completion_visible: bool,  // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,  // phone keyboard set the text, true on delete
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(
    prompt: &'a str,
    status: &'a str,
    expanded: bool,
) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}
```

## <a id="checkpoint-03b-state"></a>6. Prepare the dock completion helpers

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "Event"] }
console_error_panic_hook = "=0.1.7"
wasm-bindgen-futures = "=0.4.78"
wgpu = "=29.0.4"

egui = { version = "=0.34.3", default-features = false }
egui-wgpu = { version = "=0.34.3", default-features = false }

[dev-dependencies]
```

Edit the existing block to match this code; keep unchanged lines:

```toml
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
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
pub(crate) mod view;

/// What the command line shows and remembers between frames.
#[derive(Default)]
pub struct CommandLine {
    pub(crate) command_open: bool,        // command line shown
    pub(crate) command: String,           // text in the command field
    pub(crate) drawing_prompt: String,    // prompt while drawing
    pub(crate) focus_command: bool,       // give the field focus next frame
    pub(crate) status: String,            // status line text
    pub(crate) history: VecDeque<String>, // past commands and answers
    pub(crate) command_expanded: bool,    // history shown above the field
    pub(crate) completion: usize,         // highlighted completion index
    pub(crate) completion_prefix: String, // text the completions match
    pub(crate) inline_suffix: bool,       // completion suffix shown in the field
    pub(crate) completion_visible: bool,  // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,  // phone keyboard set the text, true on delete
}

/// The grey text of the empty field: the prompt, else the last answer when the history is folded away.
pub(crate) fn placeholder<'a>(
    prompt: &'a str,
    status: &'a str,
    expanded: bool,
) -> &'a str {
    if !prompt.is_empty() {
        return prompt;
    }

    if !expanded && !status.is_empty() {
        return status;
    }

    "Type a command"
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
    pub(crate) command_open: bool,        // command line shown
    pub(crate) command: String,           // text in the command field
    pub(crate) drawing_prompt: String,    // prompt while drawing
    pub(crate) focus_command: bool,       // give the field focus next frame
    pub(crate) status: String,            // status line text
    pub(crate) history: VecDeque<String>, // past commands and answers
    pub(crate) command_expanded: bool,    // history shown above the field
    pub(crate) completion: usize,         // highlighted completion index
    pub(crate) completion_prefix: String, // text the completions match
    pub(crate) inline_suffix: bool,       // completion suffix shown in the field
    pub(crate) completion_visible: bool,  // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,  // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>, // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>, // where those buttons are, for taps
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
    let typed = typed
        .chars()
        .filter(|c| !c.is_whitespace())
        .count();
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
pub(crate) fn placeholder<'a>(
    prompt: &'a str,
    status: &'a str,
    expanded: bool,
) -> &'a str {
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

**Locate this existing block:**

```rust
use crate::command_dock::{CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::command_dock::{self, CommandLine, placeholder, theme, view};
use crate::renderer::Renderer;
```

## <a id="checkpoint-03c-history"></a>7. Draw retained command history

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    present(&surface, &renderer, &mut panel)?;
    report("The command model owns the text and history.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    present(&surface, &renderer, &mut panel)?;
    report("The history panel is drawn.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

/// Draw retained history above the field.
pub(crate) fn history(
    ui: &mut egui::Ui,
    model: &CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
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
                    let response =
                        ui.add(egui::Label::new(&model.drawing_prompt).wrap());
                    record(
                        controls,
                        "command/hint",
                        &model.drawing_prompt,
                        &response,
                    );
                }
            });
        // divider between history and the field
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), 1.0),
            egui::Sense::hover(),
        );
        ui.painter().hline(
            ui.max_rect().x_range().expand(6.0),
            rect.center().y,
            egui::Stroke::new(1.0_f32, egui::Color32::from_gray(210)),
        );
    }
}
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
            model: CommandLine {
                status: "The field now belongs to CommandLine.".into(),
                ..Default::default()
            },
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, false, 0.0).show_inside(root, |ui| {
                view::prepare(ui);
                ui.horizontal(|ui| {
                    ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                    ui.label("Command:");
                    view::field(
                        ui,
                        &mut self.model.command,
                        placeholder("", &self.model.status, false),
                        0.0,
                        false,
                    );
                    let _ = ui
                        .button("+")
                        .on_hover_text("Collapse or expand history");
                });
            });
        });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
                ..Default::default()
            },
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            false,
                        );
                        let _ = ui
                            .button(if self.model.command_expanded {
                                "–"
                            } else {
                                "+"
                            })
                            .on_hover_text("Collapse or expand history");
                    });
                },
            );
        });
```

## <a id="checkpoint-03ca-typing"></a>8. Type directly into the command field

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-115b5fa0" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-115b5fa0" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-115b5fa0" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-115b5fa0" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-115b5fa0" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">03C · TRIE &amp; CARET</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Prefix Autocomplete Vocabulary, Keyboard Cursor Editing &amp; Caret Pixel Offset</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">x_caret = pad_x + col_idx * 8px</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-115b5fa0)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Edit Buffer &amp; Caret Index (0 .. len)</text>
  <rect x="44" y="78" width="284" height="44" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="60" y="105" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Command: Backgr</text>
  <line x1="184" y1="84" x2="184" y2="116" stroke="#dc2626" stroke-width="2.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="184" y="142" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">caret = 6 (x = 6*8 = 48px)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="168" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">Left/Right/Home/End/Backspace</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-115b5fa0)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-115b5fa0)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Case-Insensitive Prefix Match &amp; Tab Accept</text>
  <rect x="414" y="74" width="292" height="34" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="95" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">[0] Background  (prefix match ✓)</text>
  <rect x="414" y="114" width="292" height="32" rx="4" fill="#f8fafc" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">[1] Box         (Up/Down browse)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="172" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">Tab / Enter completes canonical name</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-115b5fa0)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-115b5fa0)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Live Popup Under Command Bar</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="818" y="76" width="232" height="24" rx="3" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="828" y="92" font-family="monospace" font-size="9" font-weight="700" fill="#111111">&gt; Backgr|ound</text>
  <rect x="818" y="104" width="160" height="24" rx="3" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="898" y="120" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Background [Tab]</text>
</svg>
</div>

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    present(&surface, &renderer, &mut panel)?;
    report("The history panel is drawn.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw =
        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event| {
                if panel.key(&event) {
                    if let Err(error) = present(&surface, &renderer, &mut panel) {
                        report(&format!("Cannot redraw: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Typing goes directly into the command field.");
    Ok(())
```

### 2. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                                self.model.command_expanded,
                            ),
                            0.0,
                            false,
                        );
```

Replace it with:

```rust
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
```

**Locate this block:**

```rust
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
```

Replace it with:

```rust
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
```

**Locate this block:**

```rust
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
```

Replace it with:

```rust
        }
    }

    pub fn command(&self) -> &str {
        &self.model.command
    }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() {
            return false;
        }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" {
            egui::Event::Key {
                key: egui::Key::Backspace,
                physical_key: None,
                pressed: down,
                repeat: event.repeat(),
                modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else {
            return false;
        };
        self.context
            .memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
```

**Locate this block:**

```rust
        Self {
            context,
            painter,
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
```

Replace it with:

```rust
        Self {
            context,
            painter,
            events: Vec::new(),
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
```

**Locate this block:**

```rust
    model: CommandLine,
}

impl Panel {
```

Replace it with:

```rust
    model: CommandLine,
    events: Vec<egui::Event>,
}

impl Panel {
```

## <a id="checkpoint-03cb-submit"></a>9. Submit command text once

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Typing goes directly into the command field.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Enter submits the owned command text.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
        );
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
    }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() {
            return false;
        }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" {
            egui::Event::Key {
                key: egui::Key::Backspace,
                physical_key: None,
                pressed: down,
                repeat: event.repeat(),
                modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else {
            return false;
        };
        self.context
            .memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn history(&self) -> &std::collections::VecDeque<String> {
        &self.model.history
    }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() {
            return false;
        }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" || key == "Enter" {
            egui::Event::Key {
                key: if key == "Enter" {
                    egui::Key::Enter
                } else {
                    egui::Key::Backspace
                },
                physical_key: None,
                pressed: down,
                repeat: event.repeat(),
                modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else {
            return false;
        };
        self.context
            .memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    if ui.input_mut(|input| {
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.model.status = "Command submitted.".into();
                            self.model
                                .remember(format!("> {line}\n{}", self.model.status));
                        }
                    }
                    command_dock::history(ui, &self.model, &mut None);
```

## <a id="checkpoint-03cc-edit"></a>10. Edit the command text

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pixel Rect (x, y, w, h) in Screen Space (0..W, 0..H)</text>
  <rect x="56" y="48" width="228" height="106" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="70" y="118" width="200" height="24" rx="3" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="70" cy="118" r="4" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="126" y="110" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">(x0, y0)..(x1, y1)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pixel-to-NDC: ndc_x = 2x/W - 1, ndc_y = 1 - 2y/H</text>
  <rect x="436" y="48" width="248" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="560" y1="48" x2="560" y2="154" stroke="#cbd5e1" stroke-width="1.2" stroke-dasharray="3 3"/>
  <line x1="436" y1="101" x2="684" y2="101" stroke="#cbd5e1" stroke-width="1.2" stroke-dasharray="3 3"/>
  <polygon points="454,122 666,122 666,144 454,144" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="454" y1="144" x2="666" y2="122" stroke="#16a34a" stroke-width="1.2" stroke-dasharray="3 3"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU-Rendered Command Dock Overlay</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="894,114 934,56 974,114" fill="#fce7f3" stroke="#111111" stroke-width="1.5"/>
  <rect x="818" y="126" width="232" height="24" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="830" y="142" font-family="monospace" font-size="10" font-weight="700" fill="#111111">&gt; line</text>
</svg>
</div>

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Enter submits the owned command text.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("Editing stays in the command field.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
        self.history.push_back(line);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.history.push_back(line);
    }
}
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.ctrl_key() || event.alt_key() || event.meta_key() {
            return false;
        }
        let key = event.key();
        let down = event.type_() == "keydown";
        let input = if key == "Backspace" || key == "Enter" {
            egui::Event::Key {
                key: if key == "Enter" {
                    egui::Key::Enter
                } else {
                    egui::Key::Backspace
                },
                physical_key: None,
                pressed: down,
                repeat: event.repeat(),
                modifiers: Default::default(),
            }
        } else if down && key.chars().count() == 1 {
            egui::Event::Text(key)
        } else {
            return false;
        };
        self.context
            .memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
        self.model.command_open = true;
        self.events.push(input);
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    if ui.input_mut(|input| {
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.model.status = "Command submitted.".into();
                            self.model
                                .remember(format!("> {line}\n{}", self.model.status));
                        }
                    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {
        if event.is_composing() {
            return false;
        }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(),
            ctrl: event.ctrl_key(),
            shift: event.shift_key(),
            mac_cmd: event.meta_key(),
            command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown"
            && event.key().chars().count() == 1
            && !modifiers.command
            && !modifiers.alt;
        if text {
            self.context.memory_mut(|memory| {
                memory.request_focus(egui::Id::new("command-input"))
            });
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() {
            return false;
        }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed: event.type_() == "keydown",
                repeat: event.repeat(),
                modifiers,
            });
        }
        if text {
            self.events
                .push(egui::Event::Text(event.key()));
        }
        event.prevent_default();
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
            )),
            ..Default::default()
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    if ui.input_mut(|input| {
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.model.status = "Command submitted.".into();
                            self.model
                                .remember(format!("> {line}\n{}", self.model.status));
                        }
                    }
                    if self.model.command_open
                        && ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
                        })
                    {
                        self.model.command.clear();
                        self.model.command_open = false;
                        ui.memory_mut(|memory| {
                            memory.surrender_focus(egui::Id::new("command-input"))
                        });
                    }
```

## <a id="checkpoint-03cd-vocabulary"></a>11. Recognize Help

### 1. `src/command_vocabulary.rs`

Create the file and type:

```rust
pub(crate) fn ranked(names: &[&'static str], line: &str) -> Vec<&'static str> {
    let prefix = line.to_ascii_lowercase();
    let mut choices = names
        .iter()
        .copied()
        .filter(|name| {
            name.to_ascii_lowercase()
                .starts_with(&prefix)
        })
        .collect::<Vec<_>>();
    choices.sort_by_key(|name| {
        if name.eq_ignore_ascii_case(line) {
            0
        } else if name
            .get(line.len()..)
            .is_some_and(|rest| rest.starts_with(' '))
        {
            1
        } else {
            2
        }
    });
    choices
}
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    );
    let mut panel =
        crate::panel::Panel::new(&renderer, config.format.add_srgb_suffix());
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw =
        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event| {
                if panel.key(&event) {
                    if let Err(error) = present(&surface, &renderer, &mut panel) {
                        report(&format!("Cannot redraw: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Editing stays in the command field.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    let mut panel = crate::panel::Panel::new(
        &renderer,
        config.format.add_srgb_suffix(),
        &["Help"],
    );
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw =
        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event| {
                if panel.key(&event) {
                    if let Err(error) = present(&surface, &renderer, &mut panel) {
                        report(&format!("Cannot redraw: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("Help recognizes the command vocabulary.");
    Ok(())
```

### 3. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.model.status = "Command submitted.".into();
                            self.model
                                .remember(format!("> {line}\n{}", self.model.status));
                        }
```

Replace it with:

```rust
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.commands.reply(&mut self.model, &line);
                        }
```

**Locate this block:**

```rust
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            events: Vec::new(),
```

Replace it with:

```rust
        let painter =
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            context,
            painter,
            events: Vec::new(),
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
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: &'static [&'static str],
    ) -> Self {
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
        let line = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
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
        let message = if line.eq_ignore_ascii_case("Help") {
            self.0.join(" · ")
        } else {
            "Unknown command. Type Help.".into()
        };
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

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
mod command_dock;
#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod command_dock;
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-03ce-complete"></a>12. Complete a command name

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Help recognizes the command vocabulary.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("The field completes known names.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
        self.history.push_back(line);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.history.push_back(line);
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
        model
            .completion_prefix
            .clone_from(&model.command);
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
                command_cursor_select(
                    ui.ctx(),
                    id,
                    prefix,
                    model.command.chars().count(),
                );
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model
            .completion_prefix
            .clone_from(&model.command);
    }
}
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
                        ui.label("Command:");
                        view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            ui.input(|input| {
                                input.key_pressed(egui::Key::Backspace)
                                    || input.key_pressed(egui::Key::Delete)
                            }),
                        );
```

## <a id="checkpoint-03cf-accept"></a>13. Accept or cancel a completion

### 1. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
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
        model
            .completion_prefix
            .clone_from(&model.command);
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
                command_cursor_select(
                    ui.ctx(),
                    id,
                    prefix,
                    model.command.chars().count(),
                );
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model
            .completion_prefix
            .clone_from(&model.command);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        model
            .completion_prefix
            .clone_from(&model.command);
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
                command_cursor_select(
                    ui.ctx(),
                    id,
                    prefix,
                    model.command.chars().count(),
                );
                model.inline_suffix = true;
                ui.ctx().request_repaint();
            }
        }
    } else if !model.inline_suffix {
        model
            .completion_prefix
            .clone_from(&model.command);
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
            model.command = if run {
                format!("{text} ")
            } else {
                text
            };
        }
        model.completion_visible = false;
        model.inline_suffix = false;
        model.focus_command = true;
    }
    if keys.enter
        && (!model.command.trim().is_empty() || !model.drawing_prompt.is_empty())
    {
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
    if focused
        && ui.input(|input| input.key_pressed(egui::Key::Escape))
        && escape_allowed
    {
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

**Locate this existing block:**

```rust
                    view::prepare(ui);
                    if ui.input_mut(|input| {
                        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                    }) {
                        if let Some(line) = self.model.take_command() {
                            self.commands.reply(&mut self.model, &line);
                        }
                    }
                    if self.model.command_open
                        && ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)
                        })
                    {
                        self.model.command.clear();
                        self.model.command_open = false;
                        ui.memory_mut(|memory| {
                            memory.surrender_focus(egui::Id::new("command-input"))
                        });
                    }
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            ui.input(|input| {
                                input.key_pressed(egui::Key::Backspace)
                                    || input.key_pressed(egui::Key::Delete)
                            }),
                        );
                        let _ = ui
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    view::prepare(ui);
                    let keys = command_dock::Keys {
                        enter: ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                        }),
                        tab: ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)
                        }),
                        deletes: ui.input(|input| {
                            input.key_pressed(egui::Key::Backspace)
                                || input.key_pressed(egui::Key::Delete)
                        }),
                        ..Default::default()
                    };
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            keys.deletes,
                        );
                        let mut line = None;
                        let complete = keys
                            .tab
                            .then(|| {
                                self.commands
                                    .completions(&self.model.completion_prefix)
                                    .first()
                                    .copied()
                            })
                            .flatten();
                        command_dock::finish(
                            ui,
                            &mut self.model,
                            &response,
                            &mut line,
                            &self.commands,
                            complete,
                            &keys,
                            true,
                        );
                        if let Some(line) = line {
                            if line != "Escape" {
                                self.commands.reply(&mut self.model, &line);
                            }
                        }
                        let _ = ui
```

## <a id="checkpoint-03cg-keys"></a>14. Read editing keys before the field

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("The field completes known names.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("Editing keys are handled before the field.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
}

pub(crate) fn refresh(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn suffix_space(
    ui: &egui::Ui,
    model: &mut CommandLine,
    id: egui::Id,
    focused: bool,
) {
    if model.inline_suffix
        && focused
        && model.command.chars().nth(spelled(
            &model.command,
            &model.completion_prefix,
        )) != Some(' ')
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
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if focused {
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
    let enter = focused
        && ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
        });
    let tab = focused
        && ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)
        });
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
```

### 3. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                root,
                |ui| {
                    view::prepare(ui);
                    let keys = command_dock::Keys {
                        enter: ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                        }),
                        tab: ui.input_mut(|input| {
                            input.consume_key(egui::Modifiers::NONE, egui::Key::Tab)
                        }),
                        deletes: ui.input(|input| {
                            input.key_pressed(egui::Key::Backspace)
                                || input.key_pressed(egui::Key::Delete)
                        }),
                        ..Default::default()
                    };
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let response = view::field(
```

Replace it with:

```rust
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys = command_dock::keys(ui, &mut self.model, None, id);
                        let response = view::field(
```

**Locate this block:**

```rust
        if text {
            self.context.memory_mut(|memory| {
                memory.request_focus(egui::Id::new("command-input"))
            });
```

Replace it with:

```rust
        if text {
            if !self.context.egui_wants_keyboard_input() {
                self.model.focus_command = true;
            }
            self.context.memory_mut(|memory| {
                memory.request_focus(egui::Id::new("command-input"))
            });
```

## <a id="checkpoint-03ch-popup"></a>15. Show matching commands

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Editing keys are handled before the field.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("Matching commands appear above the field.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
}

pub(crate) fn finish(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn popup(
    ui: &egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
    choices: &[&'static str],
    response: &egui::Response,
    browse: isize,
) -> Option<&'static str> {
    let width = (ui.ctx().content_rect().right() - response.rect.left() - 12.0)
        .clamp(60.0, 220.0);
    let height = (response.rect.top() - 12.0).clamp(22.0, 220.0);
    let mut complete = None;
    let popup = egui::Area::new(egui::Id::new("command-completions"))
        .pivot(egui::Align2::LEFT_BOTTOM)
        .fixed_pos(egui::pos2(
            response.rect.left(),
            response.rect.top(),
        ))
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(egui::Color32::WHITE)
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_gray(215),
                ))
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                    egui::ScrollArea::vertical()
                        .id_salt("command-choices")
                        .max_height(height)
                        .show(ui, |ui| {
                            for (index, name) in choices.iter().enumerate() {
                                let item = ui
                                    .selectable_label(index == model.completion, *name);
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

pub(crate) fn finish(
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
        };
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys = command_dock::keys(ui, &mut self.model, None, id);
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            keys.deletes,
                        );
                        let mut line = None;
                        let complete = keys
                            .tab
                            .then(|| {
                                self.commands
                                    .completions(&self.model.completion_prefix)
                                    .first()
                                    .copied()
                            })
                            .flatten();
                        command_dock::finish(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys = command_dock::keys(ui, &mut self.model, None, id);
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            keys.deletes,
                        );
                        let mut line = None;
                        let choices = self
                            .commands
                            .completions(&self.model.completion_prefix);
                        let complete =
                            if self.model.completion_visible && !choices.is_empty() {
                                command_dock::popup(
                                    ui,
                                    &mut self.model,
                                    &mut None,
                                    &choices,
                                    &response,
                                    0,
                                )
                            } else {
                                None
                            };
                        let complete = complete.or_else(|| {
                            keys.tab
                                .then(|| choices.first().copied())
                                .flatten()
                        });
                        command_dock::finish(
```

## <a id="checkpoint-03ci-browse"></a>16. Browse matching names

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Matching commands appear above the field.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("Arrow keys browse matching commands.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
}

pub(crate) fn finish(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
    if !focused
        || !model.completion_visible
        || choices.is_empty()
        || !model.options.is_empty()
    {
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
        model.completion =
            if opening && !commands.choosing_option(&model.completion_prefix) {
                if keys.browse < 0 {
                    choices.len() - 1
                } else {
                    0
                }
            } else {
                (model.completion as isize + keys.browse)
                    .rem_euclid(choices.len() as isize) as usize
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
        command_cursor_select(
            ui.ctx(),
            id,
            start,
            model.command.chars().count(),
        );
        model.inline_suffix = true;
        ui.ctx().request_repaint();
    }
    let mut complete = if keys.tab {
        Some(choices[model.completion])
    } else {
        None
    };
    // the list shows the verbs, or the options of the verb typed
    complete = popup(
        ui,
        model,
        controls,
        &choices,
        response,
        keys.browse,
    )
    .or(complete);
    complete
}

pub(crate) fn finish(
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
                        let mut line = None;
                        let choices = self
                            .commands
                            .completions(&self.model.completion_prefix);
                        let complete =
                            if self.model.completion_visible && !choices.is_empty() {
                                command_dock::popup(
                                    ui,
                                    &mut self.model,
                                    &mut None,
                                    &choices,
                                    &response,
                                    0,
                                )
                            } else {
                                None
                            };
                        let complete = complete.or_else(|| {
                            keys.tab
                                .then(|| choices.first().copied())
                                .flatten()
                        });
                        command_dock::finish(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        let mut line = None;
                        let complete = command_dock::browse(
                            ui,
                            &mut self.model,
                            &mut None,
                            &self.commands,
                            &response,
                            &keys,
                            id,
                        );
                        command_dock::finish(
```

## <a id="checkpoint-03cj-prepare"></a>17. Prepare the dock before painting

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-82ba244a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-82ba244a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-82ba244a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-82ba244a" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-82ba244a" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">03CK · DOCK HIT &amp; SCROLL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">2D Screen-Space AABB Rect Hit-Testing, Wheel Scroll Clamping &amp; Event Consumption</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">hit = (px ∈ [x0, x1]) ∧ (py ∈ [y0, y1])</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-82ba244a)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Chip AABB Hit Test [x0, y0, x1, y1]</text>
  <rect x="64" y="84" width="240" height="64" rx="5" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="64" cy="84" r="4" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="64" y="76" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#dc2626">(x0,y0)</text>
  <circle cx="304" cy="148" r="4" fill="#16a34a"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="304" y="166" text-anchor="middle" font-family="monospace" font-size="8.5" font-weight="700" fill="#16a34a">(x1,y1)</text>
  <circle cx="184" cy="116" r="5.5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="184" y="136" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Cursor (px, py) inside ✓</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-82ba244a)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-82ba244a)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Clamped Scroll Window &amp; Event Guard</text>
  <rect x="414" y="76" width="292" height="50" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">scroll = clamp(s + Δy, 0, max_rows)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="114" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">Slices visible popup rows [scroll .. scroll+K]</text>
  <rect x="414" y="134" width="292" height="46" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Dock Consumes Click -&gt; Skip 3D Pick</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-82ba244a)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-82ba244a)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Clickable Option Chips in Dock</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="78" width="98" height="26" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="871" y="95" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">[Background]</text>
  <rect x="930" y="78" width="84" height="26" rx="4" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="95" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">[Clear]</text>
</svg>
</div>

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Arrow keys browse matching commands.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("The prepared dock is painted once.");
    Ok(())
```

### 2. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            })],
            ..Default::default()
        });
        self.painter
            .render(&mut pass.forget_lifetime(), &jobs, &screen);
        buffers.push(encoder.finish());
```

Replace it with:

```rust
            })],
            ..Default::default()
        });
        self.painter.render(
            &mut pass.forget_lifetime(),
            &jobs,
            &self.screen,
        );
        buffers.push(encoder.finish());
```

**Locate this block:**

```rust
            &renderer.queue,
            &mut encoder,
            &jobs,
            &screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
```

Replace it with:

```rust
            &renderer.queue,
            &mut encoder,
            &jobs,
            &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
```

**Locate this block:**

```rust
                },
            );
        });
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter
```

Replace it with:

```rust
                },
            );
        });
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
        } else {
            self.output = Some(output);
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [
            target.texture().width(),
            target.texture().height(),
        ];
        self.prepare();
        let Some(output) = self.output.take() else {
            return;
        };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter
```

**Locate this block:**

```rust
        true
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [
            target.texture().width(),
            target.texture().height(),
        ];
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: 1.0,
        };
        let input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32),
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
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32)
                    / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
```

**Locate this block:**

```rust
            context,
            painter,
            events: Vec::new(),
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
```

Replace it with:

```rust
            context,
            painter,
            events: Vec::new(),
            output: None,
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [640, 480],
                pixels_per_point: 1.0,
            },
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
```

**Locate this block:**

```rust
    events: Vec<egui::Event>,
}

impl Panel {
```

Replace it with:

```rust
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
```

## <a id="checkpoint-03ck-rects"></a>18. Remember where the dock is drawn

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The prepared dock is painted once.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    if let Err(error) =
                        input_canvas.set_attribute("data-command-ui", &panel.inspect())
                    {
                        report(&format!("Cannot inspect dock: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The dock records its actual rectangles.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
    false
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    false
}

pub(crate) fn collapse(
    ui: &mut egui::Ui,
    model: &mut CommandLine,
    controls: &mut Option<Vec<Control>>,
) {
    let response = ui
        .button(if model.command_expanded {
            "–"
        } else {
            "+"
        })
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

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            target.texture().height(),
        ];
        self.prepare();
```

Replace it with:

```rust
            target.texture().height(),
        ];
        self.prepare();
        self.prepare();
```

**Locate this block:**

```rust
                                self.commands.reply(&mut self.model, &line);
                            }
                        }
                        let _ = ui
                            .button(if self.model.command_expanded {
                                "–"
                            } else {
                                "+"
                            })
                            .on_hover_text("Collapse or expand history");
                    });
                },
            );
        });
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
```

Replace it with:

```rust
                                self.commands.reply(&mut self.model, &line);
                            }
                        }
                        command_dock::collapse(ui, &mut self.model, &mut self.controls);
                    });
                });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
```

**Locate this block:**

```rust
                        let complete = command_dock::browse(
                            ui,
                            &mut self.model,
                            &mut None,
                            &self.commands,
                            &response,
                            &keys,
```

Replace it with:

```rust
                        let complete = command_dock::browse(
                            ui,
                            &mut self.model,
                            &mut self.controls,
                            &self.commands,
                            &response,
                            &keys,
```

**Locate this block:**

```rust
                            0.0,
                            self.model.command_open,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
```

Replace it with:

```rust
                            0.0,
                            self.model.command_open,
                        );
                        self.model.command_rect = Some(response.rect);
                        command_dock::record(
                            &mut self.controls,
                            "command/input",
                            "Command",
                            &response,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
```

**Locate this block:**

```rust
            ..Default::default()
        };
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
            view::panel(root, self.model.command_expanded, 0.0).show_inside(
                root,
                |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut None);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
```

Replace it with:

```rust
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0)
                .show_inside(root, |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut self.controls);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
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
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            context,
            painter,
            events: Vec::new(),
```

Replace it with:

```rust
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            top: f32::INFINITY,
            controls: None,
            context,
            painter,
            events: Vec::new(),
```

**Locate this block:**

```rust
    commands: Commands,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

Replace it with:

```rust
    commands: Commands,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
```

## <a id="checkpoint-03ck-pointer"></a>19. Click the command dock

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    let input_canvas = canvas.clone();
    let redraw =
        wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event| {
                if panel.key(&event) {
                    if let Err(error) = present(&surface, &renderer, &mut panel) {
                        report(&format!("Cannot redraw: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command-ui", &panel.inspect())
                    {
                        report(&format!("Cannot inspect dock: {error:?}"));
                    }
                    if let Err(error) =
                        input_canvas.set_attribute("data-command", panel.command())
                    {
                        report(&format!("Cannot inspect text: {error:?}"));
                    }
                    if let Err(error) = input_canvas.set_attribute(
                        "data-history",
                        &serde_json::to_string(panel.history()).unwrap(),
                    ) {
                        report(&format!("Cannot inspect history: {error:?}"));
                    }
                }
            },
        );
    for name in ["keydown", "keyup"] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The dock records its actual rectangles.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                panel.key(key);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                panel.pointer(pointer, &input_canvas);
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command-ui", &panel.inspect())
                {
                    report(&format!("Cannot inspect dock: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command", panel.command())
                {
                    report(&format!("Cannot inspect text: {error:?}"));
                }
                if let Err(error) = input_canvas.set_attribute(
                    "data-history",
                    &serde_json::to_string(panel.history()).unwrap(),
                ) {
                    report(&format!("Cannot inspect history: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The mouse operates the command dock.");
    Ok(())
```

### 2. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        true
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
```

Replace it with:

```rust
        true
    }

    pub fn pointer(
        &mut self,
        event: &web_sys::PointerEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        let over = pos.y >= self.top
            || self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| {
                    memory.surrender_focus(egui::Id::new("command-input"))
                });
            }
        }
        self.events
            .push(egui::Event::PointerMoved(pos));
        if matches!(
            event.type_().as_str(),
            "pointercancel" | "lostpointercapture"
        ) {
            for button in [
                egui::PointerButton::Primary,
                egui::PointerButton::Middle,
                egui::PointerButton::Secondary,
            ] {
                self.events.push(egui::Event::PointerButton {
                    pos,
                    button,
                    pressed: false,
                    modifiers: Default::default(),
                });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(
            event.type_().as_str(),
            "pointerdown" | "pointerup"
        ) {
            let button = match event.button() {
                0 => egui::PointerButton::Primary,
                1 => egui::PointerButton::Middle,
                _ => egui::PointerButton::Secondary,
            };
            self.events.push(egui::Event::PointerButton {
                pos,
                button,
                pressed: event.type_() == "pointerdown",
                modifiers: Default::default(),
            });
        }
        if matches!(
            event.type_().as_str(),
            "pointerup" | "pointercancel" | "lostpointercapture"
        ) {
            self.pointer_owned = false;
        }
        if owned {
            event.prevent_default();
        }
        owned
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
```

**Locate this block:**

```rust
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            top: f32::INFINITY,
            controls: None,
            context,
```

Replace it with:

```rust
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            pointer_owned: false,
            top: f32::INFINITY,
            controls: None,
            context,
```

**Locate this block:**

```rust
    commands: Commands,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
```

Replace it with:

```rust
    commands: Commands,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
```

## <a id="checkpoint-03cl-wheel"></a>20. Scroll through command names

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                panel.pointer(pointer, &input_canvas);
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command-ui", &panel.inspect())
                {
                    report(&format!("Cannot inspect dock: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command", panel.command())
                {
                    report(&format!("Cannot inspect text: {error:?}"));
                }
                if let Err(error) = input_canvas.set_attribute(
                    "data-history",
                    &serde_json::to_string(panel.history()).unwrap(),
                ) {
                    report(&format!("Cannot inspect history: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    redraw.forget();
    report("The mouse operates the command dock.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                panel.wheel(wheel, &input_canvas);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                panel.pointer(pointer, &input_canvas);
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command-ui", &panel.inspect())
                {
                    report(&format!("Cannot inspect dock: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command", panel.command())
                {
                    report(&format!("Cannot inspect text: {error:?}"));
                }
                if let Err(error) = input_canvas.set_attribute(
                    "data-history",
                    &serde_json::to_string(panel.history()).unwrap(),
                ) {
                    report(&format!("Cannot inspect history: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        redraw.as_ref().unchecked_ref(),
        &options,
    )?;
    redraw.forget();
    report("Scrolling selects a command name.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
}

pub(crate) fn suffix_space(
    ui: &egui::Ui,
    model: &mut CommandLine,
    id: egui::Id,
    focused: bool,
) {
    if model.inline_suffix
        && focused
        && model.command.chars().nth(spelled(
            &model.command,
            &model.completion_prefix,
        )) != Some(' ')
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
    let focused = ui.memory(|memory| memory.has_focus(id));
    let browse = if focused {
        ui.input_mut(|input| {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub(crate) fn wheel(
    ui: &mut egui::Ui,
    model: &CommandLine,
    previous: Option<egui::Rect>,
) -> isize {
    ui.input_mut(|input| {
        let over = input
            .pointer
            .hover_pos()
            .is_some_and(|point| {
                model
                    .command_rect
                    .is_some_and(|rect| rect.contains(point))
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

pub(crate) fn suffix_space(
    ui: &egui::Ui,
    model: &mut CommandLine,
    id: egui::Id,
    focused: bool,
) {
    if model.inline_suffix
        && focused
        && model.command.chars().nth(spelled(
            &model.command,
            &model.completion_prefix,
        )) != Some(' ')
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
```

### 3. `src/panel.rs`

**Locate this existing block:**

```rust
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
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32)
                    / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        self.model.completion_rect = None;
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0)
                .show_inside(root, |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut self.controls);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys = command_dock::keys(ui, &mut self.model, None, id);
                        let response = view::field(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn wheel(
        &mut self,
        event: &web_sys::WheelEvent,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2(
            (event.client_x() as f64 - rect.left()) as f32,
            (event.client_y() as f64 - rect.top()) as f32,
        );
        self.events
            .push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() {
                1 => egui::MouseWheelUnit::Line,
                2 => egui::MouseWheelUnit::Page,
                _ => egui::MouseWheelUnit::Point,
            },
            phase: egui::TouchPhase::Move,
            delta: egui::vec2(
                -event.delta_x() as f32,
                -event.delta_y() as f32,
            ),
            modifiers: Default::default(),
        });
        let over = pos.y >= self.top
            || self
                .model
                .completion_rect
                .is_some_and(|r| r.contains(pos));
        if over {
            event.prevent_default();
        }
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
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32)
                    / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0)
                .show_inside(root, |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut self.controls);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys =
                            command_dock::keys(ui, &mut self.model, previous, id);
                        let response = view::field(
```

## <a id="checkpoint-03c-layout"></a>21. Share the production dock layout

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    redraw.forget();
    report("Scrolling selects a command name.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    redraw.forget();
    report("The shared production dock is drawn.");
    Ok(())
```

### 2. `src/command_dock/mod.rs`

**Locate this existing block:**

```rust
        ui.ctx().request_repaint();
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
    ui.horizontal(|ui| {
        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        let response = view::field(
            ui,
            &mut model.command,
            placeholder(
                &model.drawing_prompt,
                &model.status,
                model.command_expanded,
            ),
            0.0,
            model.command_open,
        );
        model.command_rect = Some(response.rect);
        record(
            controls,
            "command/input",
            "Command",
            &response,
        );
        refresh(
            ui,
            model,
            &response,
            commands,
            id,
            keys.deletes,
        );
        let complete = browse(
            ui, model, controls, commands, &response, &keys, id,
        );
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
    let panel =
        view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
            view::prepare(ui);
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

### 3. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            )),
            ..Default::default()
        };
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            let panel = view::panel(root, self.model.command_expanded, 0.0)
                .show_inside(root, |ui| {
                    view::prepare(ui);
                    command_dock::history(ui, &self.model, &mut self.controls);
                    ui.horizontal(|ui| {
                        ui.set_max_width((ui.available_width() - 26.0).max(80.0));
                        ui.label("Command:");
                        let id = egui::Id::new("command-input");
                        let keys =
                            command_dock::keys(ui, &mut self.model, previous, id);
                        let response = view::field(
                            ui,
                            &mut self.model.command,
                            placeholder(
                                "",
                                &self.model.status,
                                self.model.command_expanded,
                            ),
                            0.0,
                            self.model.command_open,
                        );
                        self.model.command_rect = Some(response.rect);
                        command_dock::record(
                            &mut self.controls,
                            "command/input",
                            "Command",
                            &response,
                        );
                        command_dock::refresh(
                            ui,
                            &mut self.model,
                            &response,
                            &self.commands,
                            id,
                            keys.deletes,
                        );
                        let mut line = None;
                        let complete = command_dock::browse(
                            ui,
                            &mut self.model,
                            &mut self.controls,
                            &self.commands,
                            &response,
                            &keys,
                            id,
                        );
                        command_dock::finish(
                            ui,
                            &mut self.model,
                            &response,
                            &mut line,
                            &self.commands,
                            complete,
                            &keys,
                            true,
                        );
                        if let Some(line) = line {
                            if line != "Escape" {
                                self.commands.reply(&mut self.model, &line);
                            }
                        }
                        command_dock::collapse(ui, &mut self.model, &mut self.controls);
                    });
                });
            self.top = panel.response.rect.top() - 4.0;
        });
        if let Some(previous) = self.output.as_mut() {
```

Replace it with:

```rust
            )),
            ..Default::default()
        };
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(
                root,
                &mut self.model,
                &mut self.controls,
                &mut line,
                &self.commands,
                true,
            );
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| {
            items
                .iter()
                .find(|item| item.key == "command/resize")
        }) {
            self.top = control.rect[1];
        }
        if let Some(line) = line {
            if line != "Escape" {
                self.commands.reply(&mut self.model, &line);
            }
        }
        if let Some(previous) = self.output.as_mut() {
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

## <a id="checkpoint-03co-align"></a>22. Align and measure the command row

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Curve Quadrature ∑ w_i ||C'(t_i)|| &amp; Surface Patch dA = ||Su×Sv||</text>
  <path d="M 56 138 Q 112 52 176 126" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="84" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="116" cy="84" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="148" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <polygon points="196,136 226,74 288,74 258,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="226" y1="105" x2="268" y2="105" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <line x1="226" y1="105" x2="242" y2="74" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Divergence Volume: V = (1/6) ∑ det(v0, v1, v2) &amp; Centroid C_g</text>
  <circle cx="448" cy="138" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="448" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Origin O</text>
  <polygon points="448,138 556,58 656,92" fill="#dbeafe" fill-opacity="0.55" stroke="#2563eb" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="556,58 656,92 596,142" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="576" cy="104" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Exact Length, Area, Volume &amp; Centroid Marker</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,136 894,64 988,64 940,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="917" cy="100" r="6" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1016" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">C_g</text>
</svg>
</div>

### 1. `src/command_dock/mod.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let panel =
        view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
            view::prepare(ui);
            history(ui, model, controls);
```

Replace it with:

```rust
    let panel =
        view::panel(root, model.command_expanded, 0.0).show_inside(root, |ui| {
            view::prepare(ui);
            model.dock_top = Some(ui.max_rect().top());
            history(ui, model, controls);
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
    pub(crate) focus_command: bool,       // give the field focus next frame
    pub(crate) status: String,            // status line text
    pub(crate) history: VecDeque<String>, // past commands and answers
    pub(crate) command_expanded: bool,    // history shown above the field
    pub(crate) completion: usize,         // highlighted completion index
    pub(crate) completion_prefix: String, // text the completions match
    pub(crate) inline_suffix: bool,       // completion suffix shown in the field
    pub(crate) completion_visible: bool,  // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) agent_edit: Option<bool>,  // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>, // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>, // where those buttons are, for taps
}
```

Replace it with:

```rust
    pub(crate) focus_command: bool,       // give the field focus next frame
    pub(crate) dock_top: Option<f32>, // the dock's top edge in CSS px, for fitting above it
    pub(crate) status: String,        // status line text
    pub(crate) history: VecDeque<String>, // past commands and answers
    pub(crate) command_expanded: bool, // history shown above the field
    pub(crate) completion: usize,     // highlighted completion index
    pub(crate) completion_prefix: String, // text the completions match
    pub(crate) inline_suffix: bool,   // completion suffix shown in the field
    pub(crate) completion_visible: bool, // completion list shown
    pub(crate) completion_rect: Option<egui::Rect>, // where the list is, for taps
    pub(crate) command_rect: Option<egui::Rect>, // where the field is, for taps
    pub(crate) agent_edit: Option<bool>, // phone keyboard set the text, true on delete
    pub(crate) options: Vec<(String, String)>, // the running command's buttons: label, line it runs
    pub(crate) options_rect: Option<egui::Rect>, // where those buttons are, for taps
}
```

## <a id="checkpoint-03cp-options"></a>23. Reserve command-row space for active tool values

### 1. `src/command_dock/mod.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    });
    focus_canvas
}
```

Replace it with:

```rust
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
    let prompt = model
        .drawing_prompt
        .trim_end_matches(" · Esc cancels");
    // the prompt gives way to the options and a field of at least 120 px
    let room =
        (ui.available_width() - options_width(ui, &model.options) - 150.0).max(60.0);
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
                fonts.layout_no_wrap(
                    label.clone(),
                    font.clone(),
                    egui::Color32::BLACK,
                )
            });
            text.size().x + ui.spacing().item_spacing.x
        })
        .sum()
}
```

**Locate this block:**

```rust
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        let response = view::field(
            ui,
            &mut model.command,
            placeholder(
                &model.drawing_prompt,
                &model.status,
                model.command_expanded,
            ),
            0.0,
            model.command_open,
        );
```

Replace it with:

```rust
        ui.label("Command:");
        let id = egui::Id::new("command-input");
        let keys = keys(ui, model, previous, id);
        // a command with options reads as its prompt with the options in it; the field takes the answer
        let hint = if model.options.is_empty() {
            placeholder(
                &model.drawing_prompt,
                &model.status,
                model.command_expanded,
            )
        } else {
            options(ui, model, controls, command);
            ""
        };
        let response = view::field(
            ui,
            &mut model.command,
            hint,
            0.0,
            model.command_open,
        );
```

**Locate this block:**

```rust
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    if !focused
        || !model.completion_visible
        || choices.is_empty()
```

Replace it with:

```rust
    let focused = model.command_open || response.has_focus() || response.lost_focus();
    // a running command takes values, not other commands
    if !focused
        || !model.completion_visible
        || choices.is_empty()
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
}

#[cfg(target_arch = "wasm32")]
mod command_dock;
#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[cfg(any(test, target_arch = "wasm32"))]
mod command_dock;

#[cfg(any(test, target_arch = "wasm32"))]
mod command_vocabulary;
#[cfg(test)]
#[path = "command_dock/tool_buttons_tests.rs"]
mod tool_buttons_tests;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/command_dock/tool_buttons_tests.rs`

Create the file and type:

```rust
use crate::command_dock::*;

struct Vocabulary;
impl Commands for Vocabulary {
    fn canonical(&self, line: &str) -> String {
        line.into()
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (line.into(), true)
    }
    fn completions(&self, _: &str) -> Vec<&'static str> {
        vec!["Thickness", "Help"]
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

fn frame(
    context: &egui::Context,
    model: &mut CommandLine,
    events: Vec<egui::Event>,
) -> (Option<String>, Vec<Control>) {
    let mut line = None;
    let mut controls = Some(Vec::new());
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(960., 640.),
            )),
            focused: true,
            events,
            ..Default::default()
        },
        |root| {
            draw(
                root,
                model,
                &mut controls,
                &mut line,
                &Vocabulary,
                true,
            );
        },
    );
    (line, controls.unwrap())
}

fn context() -> egui::Context {
    let context = egui::Context::default();
    context.options_mut(|o| o.max_passes = 1.try_into().unwrap());
    context.set_fonts(theme::fonts([
        include_bytes!("../../assets/text/NotoSans-Regular.subset.ttf"),
        include_bytes!("../../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
        include_bytes!("../../assets/text/NotoSansSymbols-Regular.subset.ttf"),
    ]));
    context
}

fn model() -> CommandLine {
    CommandLine {
        drawing_prompt: "Thickness <40>: type a number · Enter keeps it".into(),
        command_open: true,
        focus_command: true,
        options: vec![
            ("Thickness 40".into(), "Thickness".into()),
            ("Create".into(), "".into()),
            ("Cancel".into(), "Escape".into()),
        ],
        ..Default::default()
    }
}

#[test]
fn real_value_buttons_return_their_option_and_keep_keyboard_focus() {
    let context = context();
    let mut model = model();
    let (_, controls) = frame(&context, &mut model, vec![]);
    let button = controls
        .iter()
        .find(|c| c.key == "command/option/Thickness 40")
        .unwrap();
    let pos = egui::pos2(
        (button.rect[0] + button.rect[2]) / 2.,
        (button.rect[1] + button.rect[3]) / 2.,
    );
    let rect = model.options_rect.unwrap();
    assert!(rect.contains(pos));
    assert!(
        rect.right() <= model.command_rect.unwrap().left(),
        "Value buttons must end before the answer field"
    );
    frame(
        &context,
        &mut model,
        vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
        ],
    );
    let (line, _) = frame(
        &context,
        &mut model,
        vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    );
    assert_eq!(line.as_deref(), Some("Thickness"));
    assert!(model.command_open && model.focus_command);
    frame(
        &context,
        &mut model,
        vec![egui::Event::Text("60".into())],
    );
    assert_eq!(
        model.command, "60",
        "Active values must not acquire an unrelated command suffix"
    );
    let (line, _) = frame(
        &context,
        &mut model,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
    assert_eq!(line.as_deref(), Some("60"));
    assert!(model.completion_rect.is_none());
}

#[test]
fn a_running_tool_suppresses_suggestions_and_empty_enter_reaches_the_owner() {
    let context = context();
    let mut model = model();
    frame(&context, &mut model, vec![]);
    frame(
        &context,
        &mut model,
        vec![egui::Event::Text("Th".into())],
    );
    assert_eq!(model.command, "Th");
    assert!(!model.inline_suffix);
    assert!(model.completion_rect.is_none());
    model.command.clear();
    let (line, _) = frame(
        &context,
        &mut model,
        vec![egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Default::default(),
        }],
    );
    assert_eq!(
        line.as_deref(),
        Some(""),
        "The tool decides whether Enter keeps a value or creates geometry"
    );
    model.options.clear();
    model.drawing_prompt.clear();
    frame(
        &context,
        &mut model,
        vec![egui::Event::Text("Th".into())],
    );
    assert!(
        model.inline_suffix,
        "Ordinary command completion returns after the tool ends"
    );
}
```

## <a id="checkpoint-03cn-handoff"></a>24. Return accepted application commands

### 1. `src/browser.rs`

**Locate this existing block:**

```rust
    );
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                panel.key(key);
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                panel.wheel(wheel, &input_canvas);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                panel.pointer(pointer, &input_canvas);
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command-ui", &panel.inspect())
                {
                    report(&format!("Cannot inspect dock: {error:?}"));
                }
                if let Err(error) =
                    input_canvas.set_attribute("data-command", panel.command())
                {
                    report(&format!("Cannot inspect text: {error:?}"));
                }
                if let Err(error) = input_canvas.set_attribute(
                    "data-history",
                    &serde_json::to_string(panel.history()).unwrap(),
                ) {
                    report(&format!("Cannot inspect history: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        redraw.as_ref().unchecked_ref(),
        &options,
    )?;
    redraw.forget();
    report("The shared production dock is drawn.");
    Ok(())
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if let Err(error) = panel.update(Some(&event), &input_canvas) {
                report(&format!("Cannot read command: {error:?}"));
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
                    "Cannot lay out commands: {error:?}"
                ));
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        redraw.as_ref().unchecked_ref(),
        &options,
    )?;
    redraw.forget();
    report("Panel returns accepted application commands.");
    Ok(())
```

### 2. `src/panel.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }) {
            self.top = control.rect[1];
        }
        if let Some(line) = line {
            if line != "Escape" {
                self.commands.reply(&mut self.model, &line);
            }
        }
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
        } else {
            self.output = Some(output);
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        self.screen.size_in_pixels = [
            target.texture().width(),
            target.texture().height(),
        ];
        self.prepare();
        self.prepare();
        let Some(output) = self.output.take() else {
```

Replace it with:

```rust
        }) {
            self.top = control.rect[1];
        }
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
        } else {
            self.output = Some(output);
        }
        line
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let Some(output) = self.output.take() else {
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
        input
            .viewports
            .get_mut(&egui::ViewportId::ROOT)
            .unwrap()
            .native_pixels_per_point = Some(self.screen.pixels_per_point);
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
        self.model
            .remember(format!("> {line}\n{message}"));
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
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                self.consumed = self.key(key);
            }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new();
                    options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                self.consumed = self.wheel(wheel, canvas);
            }
        }
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else {
            return Ok(None);
        };
        if line == "Escape" {
            return Ok(None);
        }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self
            .commands
            .0
            .iter()
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
                pixels_per_point: 1.0,
            },
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
                ..Default::default()
            },
        }
```

Replace it with:

```rust
                pixels_per_point: 1.0,
            },
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
        }
```

**Locate this block:**

```rust
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            pointer_owned: false,
            top: f32::INFINITY,
            controls: None,
```

Replace it with:

```rust
            egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            commands: Commands(commands),
            consumed: false,
            pointer_owned: false,
            top: f32::INFINITY,
            controls: None,
```

**Locate this block:**

```rust
}

impl Commands {
    fn reply(&self, model: &mut CommandLine, line: &str) {
        let line = self.canonical(line);
        let message = if line.eq_ignore_ascii_case("Help") {
            self.0.join(" · ")
        } else {
            "Unknown command. Type Help.".into()
        };
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

pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
```

## <a id="checkpoint-03d-input"></a>25. Hand commands to the application

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    panel.update(None, &canvas)?;
    present(&surface, &renderer, &mut panel)?;
    let input_canvas = canvas.clone();
    let redraw = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(
        move |event: web_sys::Event| {
            if let Err(error) = panel.update(Some(&event), &input_canvas) {
                report(&format!("Cannot read command: {error:?}"));
            }
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!(
                    "Cannot lay out commands: {error:?}"
                ));
            }
            {
                if let Err(error) = present(&surface, &renderer, &mut panel) {
                    report(&format!("Cannot redraw: {error:?}"));
                }
            }
        },
    );
    for name in [
        "keydown",
        "keyup",
        "pointerdown",
        "pointermove",
        "pointerup",
        "pointercancel",
    ] {
        window
            .add_event_listener_with_callback(name, redraw.as_ref().unchecked_ref())?;
    }
    let options = web_sys::AddEventListenerOptions::new();
    options.set_passive(false);
    canvas.add_event_listener_with_callback_and_add_event_listener_options(
        "wheel",
        redraw.as_ref().unchecked_ref(),
        &options,
    )?;
    redraw.forget();
    report("Panel returns accepted application commands.");
    Ok(())
```

Replace it with:

```rust
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
        canvas
            .add_event_listener_with_callback(name, click.as_ref().unchecked_ref())?;
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
```

**Locate this block:**

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
```

Replace it with:

```rust
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

pub fn report(message: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
```

## <a id="checkpoint-04-input"></a>26. Make a choice change the picture

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-7092c15f" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-7092c15f" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-7092c15f" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-7092c15f" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-7092c15f" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">04-INPUT · COMMAND → GPU</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Command Dock Handoff to Application State &amp; Dynamic GPU RenderPass Clear</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">Command("Clear Pink") -&gt; wgpu::Color</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-7092c15f)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Accepted Dock Command Token</text>
  <rect x="48" y="84" width="276" height="44" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="110" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">&gt; Clear Pink [Enter]</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">dock.key_down("Enter") -&gt; Some(cmd)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-7092c15f)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-7092c15f)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · State::apply_command -&gt; Clear Color</text>
  <rect x="416" y="74" width="288" height="110" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="432" y="86" width="80" height="42" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="472" y="111" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">White</text>
  <rect x="520" y="86" width="80" height="42" rx="3" fill="#e5e7eb" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="111" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Grey</text>
  <rect x="608" y="86" width="80" height="42" rx="3" fill="#fce7f3" stroke="#db2777" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="648" y="111" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#db2777">Pink ✓</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="162" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">LoadOp::Clear(state.background)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-7092c15f)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-7092c15f)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Immediate GPU Frame Redraw</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#fce7f3" stroke="#111111" stroke-width="2"/>
  <polygon points="934,80 874,152 994,152" fill="#f25999" stroke="#111111" stroke-width="1.5"/>
  <rect x="818" y="158" width="232" height="24" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="174" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">Background cleared to pink.</text>
</svg>
</div>

### 1. `src/background.rs`

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

### 2. `src/lib.rs`

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

### 3. `src/renderer.rs`

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

### 4. `src/browser.rs`

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

![Recorded visual output for Lesson 1.3 (03d-input)](../../screenshots/journey/03d-input-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 1.3
cd workspace/journey
cargo build --lib --locked --target wasm32-unknown-unknown -j1
CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 1.3
npm --prefix ../session_tests run course -- save 1.3
```

Continue with [1.4 · Commands that change the view](04-input.md).
