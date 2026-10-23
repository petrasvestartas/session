# 22.5 · Coherent multisampling and frame-resource ownership

**Goal:** Choose coherent colour samples from actual geometry, physical viewport, device budget and recovery without changing original sources or owner queries. · **Steps:** 19 · **Source:** [engine/gpu/render.rs](../../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../../src/engine/gpu/frame.rs) · [engine/gpu/view.rs](../../../../src/engine/gpu/view.rs) · [engine/pipelines/mod.rs](../../../../src/engine/pipelines/mod.rs) · [shaders/triangle.wgsl](../../../../src/shaders/triangle.wgsl) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl) · [shaders/ink_visibility.wgsl](../../../../src/shaders/ink_visibility.wgsl) · [app/scene.rs](../../../../src/app/scene.rs) (`session_viewer/src/lib.rs`)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="200" viewBox="0 0 1120 200" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="196" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="16" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 22.5 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/frame_targets.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/primitive_image.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">19 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/engine/gpu/device.rs</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Keep depth and address sa…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. Vertex RGB &amp; Uniforms</text>
  <polygon points="610,56 690,56 610,126" fill="#bfdbfe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="690,56 690,126 610,126" fill="#fde68a" stroke="#111111" stroke-width="1.75"/>
  <circle cx="610" cy="56" r="4" fill="#ef4444"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v0</text>
  <circle cx="690" cy="56" r="4" fill="#22c55e"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="58" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v1</text>
  <circle cx="610" cy="126" r="4" fill="#3b82f6"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="590" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v2</text>
  <circle cx="690" cy="126" r="4" fill="#f59e0b"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="696" y="130" font-family="monospace" font-size="9" font-weight="700" fill="#111111">v3</text>
  <rect x="720" y="64" width="90" height="54" rx="5" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">view_proj</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="765" y="104" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">mat4x4&lt;f32&gt;</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Coherent multisampling and…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Depth32Float &amp; Ray Pick</text>
  <rect x="936" y="54" width="96" height="62" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="892" y="74" width="96" height="62" rx="4" fill="#fdba74" stroke="#111111" stroke-width="2"/>
  <line x1="864" y1="138" x2="932" y2="102" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <circle cx="936" cy="100" r="5" fill="#facc15" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="896" y="68" font-family="monospace" font-size="9" font-weight="700" fill="#111111">z=0.2 &lt; z=0.7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Choose bounded frame qual…</text>
</svg>
</div>

## <a id="checkpoint-96zzzzzzu-keep-depth-and-address-samples-separate"></a>1. Keep depth and address samples separate

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/frame_targets.rs, src/p…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One sample count</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">direct sample reads</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">colour-only resolve</text>
</svg>
</div>

**What this code does:** Define matching frame attachments without averaging depth or physical primitive addresses.

### 1. `src/frame_targets.rs`

Own matching depth and primitive samples; keep the optional colour image as the only resolve source.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-01.rs"
```

### 2. `src/primitive_image.rs`

Keep request-owned queries single-sample. Four-sample integer images are sampled directly and never copied or resolved.

Replace the entire file with:

```rust
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-02.rs"
```

### 3. `src/lib.rs`

Register the attachment owner. The main renderer remains single-sample.

**Locate this existing block:**

```rust
mod primitive_image;
```

After that block, type:

```rust
mod frame_targets;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/frame_target_tests.rs`

Read every actual depth/ID sample, resolve only colour and retain readable old bound views across replacement.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-04.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/lib.rs`

Register the supplied real-GPU sample fixture.

**Locate this existing block:**

```text
mod frame_targets;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzu-keep-depth-and-address-samples-separate-05.rs" skip=18
```

## <a id="checkpoint-96zzzzzzv-reuse-coherent-frame-targets"></a>2. Reuse coherent frame targets

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/lib.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing renderer</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged extent retains…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">real resize replaces matc…</text>
</svg>
</div>

**What this code does:** Retain frame attachments at equal extent and replace them coherently during real resizing.

### 1. `src/renderer.rs`

Move every existing depth field reference under the target owner; preserve longer names.

Use your editor’s **whole-word Replace All** in this file. Find this field reference:

```rust
self.depth
```

Replace every match with:

```rust
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-rename-depth.rs"
```

Keep longer field names such as `self.depth_pyramid` unchanged. The checkpoint replay validates this exact rename; no implementation code is supplied.

### 2. `src/renderer.rs`

Move every existing primitive field reference under the target owner; preserve longer names.

Use your editor’s **whole-word Replace All** in this file. Find this field reference:

```rust
self.primitive
```

Replace every match with:

```rust
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-rename-primitive.rs"
```

Keep longer field names such as `self.depth_pyramid` unchanged. The checkpoint replay validates this exact rename; no implementation code is supplied.

### 3. `src/renderer.rs`

Wire the existing renderer to its persistent, coherent target owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.widget.draw(&mut encoder,view);
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.targets.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
```

Replace it with:

```rust
            Err(error)=>{*self.text_error.borrow_mut()=Some(error.to_string());}
            }
        }
        self.targets.resolve(&mut encoder,output);
        self.widget.draw(&mut encoder,output);
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode(&self.device,&self.queue,&mut encoder,&self.targets.depth,*transform,self.size,self.density,[self.pyramid_revision,clip_revision]);}
        self.queue.submit([encoder.finish()]);tiles.map_report();
    }
```

**Locate this block:**

```rust
    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
```

Replace it with:

```rust
    pub fn draw_mode(&self, view: &wgpu::TextureView, background: &crate::background::Background,
        transform: &[f32; 16], normals: bool,
    ) {
        let output=view;let view=self.targets.color(output);
        let (clip_group,clip_revision)={let mut clip=self.clipping.borrow_mut();clip.frame_anchored(&self.queue,*transform,self.anchor(),self.size[1],self.density);(clip.group.clone(),clip.writes)};
        self.caps.borrow_mut().prepare(&self.device,&self.queue,self.format,self.size,{let clip=self.clipping.borrow();clip.enabled&&!clip.rows.is_empty()});
        self.prepare_clouds(*transform);
```

**Locate this block:**

```rust

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    fn depth(device: &wgpu::Device, width: u32, height: u32) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("opaque depth"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size != [size.width, size.height] { self.cancel_pick(); }
        self.size = [size.width, size.height];
        self.targets.depth = Self::depth(&self.device, size.width, size.height);
        self.targets.primitive=crate::primitive_image::Image::new(&self.device,self.size);
    }

    pub fn set_density(&mut self, density: f32) {
```

Replace it with:

```rust

    pub fn anchor(&self)->[f64;3] {self.space.anchor()}

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        if self.size == [size.width, size.height] { return; }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets=crate::frame_targets::Targets::new(&self.device,self.size,self.format,self.targets.samples);
    }

    pub fn set_density(&mut self, density: f32) {
```

**Locate this block:**

```rust
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, depth, primitive, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
```

Replace it with:

```rust
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
        for lane in [&mut renderer.strokes,&mut renderer.paths,&mut renderer.markers]{lane.enable_finite(&renderer.device,format,false);}
        renderer.highlight.enable_finite(&renderer.device,format);
```

**Locate this block:**

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let primitive=crate::primitive_image::Image::new(&device,[640,480]);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

Replace it with:

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::new(&device,[640,480],format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

**Locate this block:**

```rust
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    depth: wgpu::TextureView,
    primitive:crate::primitive_image::Image,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

Replace it with:

```rust
    settings_bytes: usize,
    uniform: wgpu::Buffer,
    view_group: wgpu::BindGroup,
    targets:crate::frame_targets::Targets,
    strokes: crate::stroke_gpu::Lane,
    paths: crate::stroke_gpu::Lane,
    markers: crate::stroke_gpu::Lane,
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/lib.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
gets;
#[cfg(all(test,not(target_arch="wasm32")))]
mod frame_target_tests;
pub mo
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/renderer.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ct_tests;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-02.rs" skip=10
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
nk(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();scene.translate(owner,[2000.,0.,0.]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let target=Some(Target{owner,part:Part::Face(key)});let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;gpu.set_highlight(&scene,target).unwrap();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,old);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let geometry=gpu.highlight.geometry_for_test().unwrap().clone();let uploads=gpu.stats()[0];gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&scene);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));let actual=crate::cap_gpu_tests::pixels_at(&gpu,matrix());assert!(actual==baseline,"Rebased original face and ink pixels must match exactly");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);drop(geometry);gpu.set_highlight(&scene,None).unwrap();assert!(gpu.highlight.geometry_for_test().is_none());assert!(!gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[t
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-73.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
orm);
 gpu.mesh_ink.set(&gpu.device,crate::pick_sources::Sources::anchored(&far,anchor).unwrap().mesh_ink).unwrap();for(target,expected)in targets.into_iter().zip(pixels){assert!(target.valid(&far));gpu.set_highlight(&far,Some(target)).unwrap();let actual=crate::cap_gpu_tests::pixels_at(&gpu,local);assert!(actual==expected,"Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",actual.iter().zip(&expected).filter(|(a,b)|a!=b).count());assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);}gpu.set_highlight(&far,None).unwrap();gpu.select_objects(&far,&[line,path,point]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),objects);gpu.select_objects(&far,&[]).unwrap();gpu.set_highlight(&far,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[t
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-74.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ng(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.4,0.5,-0.5,0.4,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.4,-0.5,-0.5),session_rust::Point::new(0.,-0.3,-0.5),session_rust::Point::new(0.4,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(path).unwrap();scene.cloud_controls(Some(cloud));let curve=scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(&[session_rust::Point::new(-0.4,0.8,-0.5),session_rust::Point::new(0.,0.65,-0.5),session_rust::Point::new(0.4,0.8,-0.5)]))).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let edge=scene.objects()[0].mesh.parts.edges[0];let control=scene.control_markers().into_iter().find(|c|c.parent==path).unwrap().part;let targets=[Target{owner:mesh,part:Part::Face(key)},Target{owner:mesh,part:Part::MeshEdge(edge.0,edge.1)},Target{owner:line,part:Part::Curve},Target{owner:curve,part:Part::Curve},Target{owner:path,part:Part::PathEdge(0)},Target{owner:path,part:control},Target{owner:cloud,part:Part::CloudPoint(u32::MAX)}];let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let mut pixels=Vec::new();for target in targets{assert!(target.valid(&scene));gpu.set_highlight(&scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(selected!=baseline,"A valid original highlight must change actual pixels: {target:?}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);pixels.push(selected);}gpu.set_highlight(&scene,None).unwrap();gpu.select_objects(&scene,&[line,path,point]).unwrap();let objects=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert_ne!(objects,baseline);gpu.select_objects(&scene,&[]).unwrap();gpu.set_highlight(&scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,transform),baseline);
 let a
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-75.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
)[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 asser
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-76.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let c
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-77.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
eue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-78.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 asser
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-79.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
T-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 as
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-80.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ty});
  assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
  gpu.
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-81.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
);
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
}

#[t
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-82.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture)[at],0xfffffff1);
  gpu.
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-83.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 17. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
gpu);
  let words=primitive_pixels(&gpu,&gpu.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  asse
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-84.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 18. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ene);
    assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|word|*word==0));
    fo
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-85.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 19. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.primitive.texture.width(),gpu.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.primitive.texture).iter().all(|id|*id==0));
}

#[t
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-86.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 20. `src/global_outline_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
ene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-87.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 21. `src/contact_feature_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
se});
    let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor{label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&r.depth)},wgpu::BindGroupEntry{binding:1,resource:output.as_entire_binding()}]});
    le
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-88.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 22. `src/ambient_input_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
t()),
        &g.depth,
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-89.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 23. `src/ambient_contact_tests.rs`

Retain the existing validation assertions while using the new target owner.

**Locate this existing block:**

```text
t()),
        &g.depth,
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-90.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 24. `src/frame_target_tests.rs`

Append the actual unchanged-extent, coherent-resize, nonblank pixels, full sources, geometry identity and release witness.

**Locate this existing block:**

```text
use crate::frame_targets::Targets;

fn read(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<u8> {
    let (tx, rx) = std::sync::mpsc::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let data = buffer.slice(..).get_mapped_range().to_vec();
    buffer.unmap();
    data
}

#[test]
fn actual_samples_keep_depth_and_integer_addresses_and_resolve_only_colour() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) = pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|e| panic!("actual frame targets: {e}")));
    for samples in [1, 4] {
        let mut targets = Targets::new(&device, [8, 8], wgpu::TextureFormat::Rgba8Unorm, samples);
        assert_eq!(targets.primitive.texture.sample_count(), samples);
        assert_eq!(
            targets.primitive.texture.format(),
            crate::primitive_image::FORMAT
        );
        assert_eq!(
            targets
                .primitive
                .texture
                .usage()
                .contains(wgpu::TextureUsages::COPY_SRC),
            samples == 1
        );
        let output = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("resolved float colour"),
            size: wgpu::Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let single = output.create_view(&Default::default());
        assert_eq!(targets.color(&single) == &single, samples == 1);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual distinct samples"),
            source: wgpu::ShaderSource::Wgsl(r#"
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4f {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3));return vec4f(p[i],0.5,1);
}
struct Hit { @location(0) color:vec4f, @location(1) primitive:vec2u, @builtin(frag_depth) depth:f32 }
@fragment fn fragment(@builtin(sample_index) sample:u32)->Hit {
 return Hit(vec4f(f32(sample&1u),f32((sample>>1u)&1u),0,1),vec2u(sample+1u,50000u+sample),f32(sample+1u)*0.125);
}"#.into()),
        });
        let pipe = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("per-sample colour, integer and depth"),
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
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(crate::primitive_image::target(true)),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: crate::depth::FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        });
        let storage = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let mut source = format!(
            r#"
@group(0) @binding(0) var depth:texture_depth_2d;
@group(0) @binding(1) var physical:texture_2d<u32>;
@group(0) @binding(2) var<storage,read_write> values:array<vec4u>;
@compute @workgroup_size(4) fn read_samples(@builtin(global_invocation_id) id:vec3u) {{
 if(id.x>={samples}u){{return;}}
 let primitive=textureLoad(physical,vec2i(2),SAMPLE).xy;
 values[id.x]=vec4u(bitcast<u32>(textureLoad(depth,vec2i(2),SAMPLE)),primitive,id.x);
}}
"#
        );
        if samples == 4 {
            source = source
                .replace("texture_depth_2d", "texture_depth_multisampled_2d")
                .replace("texture_2d<u32>", "texture_multisampled_2d<u32>");
        }
        source = source.replace("SAMPLE", if samples == 4 { "i32(id.x)" } else { "0" });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("actual sample readback"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let compute = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("read_samples"),
            compilation_options: Default::default(),
            cache: None,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&targets.depth),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&targets.primitive.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: storage.as_entire_binding(),
                },
            ],
        });
        let result = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let pixels = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256 * 8,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("actual matching sample attachments"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: targets.color(&single),
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                    Some(wgpu::RenderPassColorAttachment {
                        view: &targets.primitive.view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &targets.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_pipeline(&pipe);
            pass.draw(0..3, 0..1);
        }
        targets.resolve(&mut encoder, &single);
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        encoder.copy_texture_to_buffer(
            output.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &pixels,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(8),
                },
            },
            output.size(),
        );
        queue.submit([encoder.finish()]);
        let actual = read(&device, &result);
        for sample in 0..samples as usize {
            let words = actual[sample * 16..sample * 16 + 16]
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            assert_eq!(f32::from_bits(words[0]), (sample + 1) as f32 * 0.125);
            assert_eq!(
                &words[1..],
                &[sample as u32 + 1, 50000 + sample as u32, sample as u32]
            );
        }
        let actual = read(&device, &pixels);
        let expected = if samples == 4 {
            [128u8, 128, 0, 255]
        } else {
            [0, 0, 0, 255]
        };
        assert!(actual[2 * 256 + 2 * 4..2 * 256 + 2 * 4 + 4]
            .iter()
            .zip(expected)
            .all(|(&a, b)| a.abs_diff(b) <= 1));
        let depth = targets.depth.clone();
        let primitive = targets.primitive.texture.clone();
        let other = if samples == 1 { 4 } else { 1 };
        targets = Targets::new(&device, [17, 13], wgpu::TextureFormat::Rgba8Unorm, other);
        assert_eq!(targets.primitive.texture.sample_count(), other);
        assert_eq!(
            [
                targets.primitive.texture.width(),
                targets.primitive.texture.height()
            ],
            [17, 13]
        );
        assert_ne!(targets.depth, depth);
        assert_ne!(targets.primitive.texture, primitive);
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&compute);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(1, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&storage, 0, &result, 0, 64);
        queue.submit([encoder.finish()]);
        let retained = read(&device, &result);
        for sample in 0..samples as usize {
            let at = sample * 16;
            assert_eq!(
                f32::from_bits(u32::from_ne_bytes(retained[at..at + 4].try_into().unwrap())),
                (sample + 1) as f32 * 0.125
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 4..at + 8].try_into().unwrap()),
                sample as u32 + 1
            );
            assert_eq!(
                u32::from_ne_bytes(retained[at + 8..at + 12].try_into().unwrap()),
                50000 + sample as u32
            );
        }
        targets = Targets::new(&device, [8, 8], wgpu::TextureFormat::Rgba8Unorm, samples);
        assert_eq!(targets.primitive.texture.sample_count(), samples);
    }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzv-reuse-coherent-frame-targets-91.rs" skip=12497
```

## <a id="checkpoint-96zzzzzzw-match-resolve-base-and-view-formats"></a>3. Keep resolved colours in the canvas format

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/frame_targets.rs, src/r…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Actual canvas base format</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching frame attachments</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">colour-only resolve</text>
</svg>
</div>

**What this code does:** Preserve flat colours by matching the base texture formats and rendering view formats at both ends of a resolve.

### 1. `src/frame_targets.rs`

Retain identical-format callers and accept an explicit compatible base/view pair. Give the image allocator separate base and view formats. Create the actual texture in the output base format. Permit the requested render view. Create the existing colour view explicitly. Keep depth storage and view formats identical. Match the resolve output base while retaining the colour view.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        Self {
            depth: image("scene depth", crate::depth::FORMAT),
            primitive: crate::primitive_image::Image::with_samples(device, size, samples),
            color: (samples == 4).then(|| image("multisampled scene colour", format)),
            samples,
        }
    }
```

Replace it with:

```rust
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: base,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[format],
                })
                .create_view(&wgpu::TextureViewDescriptor {
                    format: Some(format),
                    ..Default::default()
                })
        };
        Self {
            depth: image("scene depth", crate::depth::FORMAT, crate::depth::FORMAT),
            primitive: crate::primitive_image::Image::with_samples(device, size, samples),
            color: (samples == 4).then(|| image("multisampled scene colour", base, format)),
            samples,
        }
    }
```

**Locate this block:**

```rust
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
```

Replace it with:

```rust
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        Self::with_formats(device, size, format, format, samples)
    }

    pub fn with_formats(
        device: &wgpu::Device,
        size: [u32; 2],
        base: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> Self {
        assert_eq!(base.remove_srgb_suffix(), format.remove_srgb_suffix());
        assert!(matches!(samples, 1 | 4) && !size.contains(&0));
        let image = |label, base, format| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
```

### 2. `src/renderer.rs`

Keep the output base format beside the existing colour view format. Retain the existing constructor and accept explicit base/view formats at the browser boundary. Use the same base format as the eventual resolve output. Store the supplied output format once. Keep matching base/view formats during real resizing.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        if self.size == [size.width, size.height] { return; }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets=crate::frame_targets::Targets::new(&self.device,self.size,self.format,self.targets.samples);
    }

    pub fn set_density(&mut self, density: f32) {
```

Replace it with:

```rust
        if self.size == [size.width, size.height] { return; }
        self.cancel_pick();
        self.size = [size.width, size.height];
        self.targets=crate::frame_targets::Targets::with_formats(&self.device,self.size,self.output_format,self.format,self.targets.samples);
    }

    pub fn set_density(&mut self, density: f32) {
```

**Locate this block:**

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

**Locate this block:**

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::new(&device,[640,480],format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

Replace it with:

```rust
                resource: uniform.as_entire_binding(),
            }],
        });
        let targets = crate::frame_targets::Targets::with_formats(&device,[640,480],output_format,format,1);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
```

**Locate this block:**

```rust
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
```

Replace it with:

```rust
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        Self::with_formats(device, queue, format, format, scene)
    }

    pub fn with_formats(
        device: wgpu::Device,
        queue: wgpu::Queue,
        output_format: wgpu::TextureFormat,
        format: wgpu::TextureFormat,
        scene: &Scene,
    ) -> Self {
        crate::shader_check::check(&device, crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0), "Cloud points");
        crate::shader_check::check(&device, include_str!("cloud_resolve.wgsl").into(), "Cloud resolve");
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
```

**Locate this block:**

```rust
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
```

Replace it with:

```rust
    presentation_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text:std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format:wgpu::TextureFormat,
    output_format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
```

### 3. `src/browser.rs`

Pass the actual canvas base and rendering view formats separately.

**Locate this existing block:**

```rust
    let mut renderer = Renderer::new(
        device,
        queue,
        config.format.add_srgb_suffix(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let mut renderer = Renderer::with_formats(
        device,
        queue,
        config.format,
        config.format.add_srgb_suffix(),
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/frame_target_tests.rs`

Extend the existing actual GPU fixture across matching base/view pairs while retaining every depth/address, resize and release assertion.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzzw-match-resolve-base-and-view-formats-14.rs"
```

## <a id="checkpoint-96zzzzzzx-reuse-count-specific-surface-pipelines"></a>4. Reuse surface pipelines for each sample count

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/sample_pipelines.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Existing surface descript…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">lazy pipeline pair</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged original geomet…</text>
</svg>
</div>

**What this code does:** Match ordinary and shared surface shaders and pipeline descriptors to the requested sample count while retaining original geometry and previously compiled variants.

### 1. `src/sample_pipelines.rs`

Retain only active and alternate pipeline/layout recipes for counts1 and4; never retain inactive frame images.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-01.rs"
```

### 2. `src/surface_pipelines.rs`

Declare the existing surface descriptor factory interface and pair return.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-02.rs"
```

### 3. `src/surface_pipelines.rs`

Move the exact previously typed shader constructor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// shader
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-03.rs"
```

### 4. `src/surface_pipelines.rs`

Move the exact previously typed layout and descriptor closure.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// layout
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-04.rs"
```

### 5. `src/surface_pipelines.rs`

Assemble the count-specific surface inputs from one original shader. Borrow the existing caller-owned layouts. Use the existing device reference. Use the same shared geometry layout. Apply one explicit colour sample count. Use sample-qualified colour/world/normal inputs only for4x.

**Locate this existing block:**

```rust
pub fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        (mesh_pipeline(crate::depth::surface(),true),mesh_pipeline(crate::depth::ink(),false))

}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view_layout:&wgpu::BindGroupLayout,model_layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(sampled(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0),samples).into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(view_layout), Some(model_layout),Some(&crate::clipping_gpu::layout(device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: wgpu::MultisampleState {count:samples,..Default::default()},
            multiview_mask: None,
            cache: None,
        });
        (mesh_pipeline(crate::depth::surface(),true),mesh_pipeline(crate::depth::ink(),false))

}

pub fn sampled(source:String,samples:u32)->String {
 if samples==1 {return source;}
 source.replace("@location(0) colour:","@location(0) @interpolate(perspective,sample) colour:")
 .replace("@location(1) world:","@location(1) @interpolate(perspective,sample) world:")
 .replace("@location(2) normal:","@location(2) @interpolate(perspective,sample) normal:")
}
```

### 6. `src/renderer.rs`

Remove the shader constructor after moving its exact bytes into the surface factory. Remove the descriptor closure after moving its exact bytes into the surface factory. Keep the existing ordinary and highlight surface recipes in the shared sample-count cache.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.pipeline);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.highlight_pipeline); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
```

Replace it with:

```rust
            });
            self.grid.draw(&mut pass,&self.view_group);
            pass.set_bind_group(2,&clip_group,&[]);
            pass.set_pipeline(&self.surfaces.active.0);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            for mesh in self.meshes.iter().filter(|row|self.opacity.get()<1.||!self.instance_rows.contains(&row.id)) {if !self.instances.draw_ordinary(&mut pass,mesh){pass.set_pipeline(&self.surfaces.active.0);self.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}}
            if self.opacity.get()>=1.{self.instances.draw(&mut pass);}
            pass.set_pipeline(&self.surfaces.active.1); pass.set_bind_group(0, &self.view_group, &[]);
            self.instances.bind_own(&mut pass);
            self.highlight.mesh(&mut pass);
        }
```

**Locate this block:**

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.pipeline.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

Replace it with:

```rust
    pub fn set_highlight(&mut self, scene: &Scene, target: Option<crate::pick_sources::Target>) -> Result<(), &'static str> {
        if let Some(outline)=self.outline.borrow_mut().as_mut() {outline.invalidate();}
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        let anchor=self.anchor();self.highlight.set_at(&self.device,&self.surfaces.active.0.get_bind_group_layout(1),scene,target,anchor)
    }

    pub fn select_objects(&mut self, scene: &Scene, ids: &[ObjectId]) -> Result<(), &'static str> {
```

**Locate this block:**

```rust
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.pipeline.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
```

Replace it with:

```rust
        if let Some(outline)=self.global_outline.borrow_mut().as_mut() {outline.invalidate();}
        self.cancel_pick(); self.faces.clear();
        self.preview.set(&self.device, None).expect("Empty tool preview");
        self.highlight.set(&self.device, &self.surfaces.active.0.get_bind_group_layout(1), scene, None).expect("Empty selection");
        self.set_source_ink(scene);
        let layout = self.surfaces.active.0.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects().iter().filter(|row| scene.visible(row.id)) {
            let matching = old.iter().position(|row| row.id == object.id
```

**Locate this block:**

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, pipeline, highlight_pipeline, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

Replace it with:

```rust
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let sheets=crate::sheet_gpu::Lane::new(&device,format);
        let measurement=crate::measure_gpu::Measurement::new(&device,format);let guides=crate::guide_gpu::Guide::new(&device,format);
        let mut renderer = Self {surface_options:Default::default(),world_bounds:Default::default(),space:Default::default(),opacity:std::cell::Cell::new(1.),ambient_enabled:Default::default(),ambient:Default::default(),ambient_rows:Default::default(),pyramid:Default::default(),pyramid_revision:0,caps:std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),clipping:std::cell::RefCell::new(crate::clipping_gpu::Clipping::new(&device)),tiles,guides,measurement,sheets,instances,instance_rows:Default::default(),clouds:Default::default(),cloud_view:Default::default(),cloud_error:Default::default(),outline:Default::default(),global_outline:Default::default(),outlines_enabled:Default::default(),outline_composite:Default::default(),full_text_fonts:Default::default(),presentation_text:Default::default(),scene_text:Default::default(), text:std::cell::RefCell::new(None),text_error:std::cell::RefCell::new(None),format,output_format,device, queue, surfaces, highlight, preview, widget, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, settings_writes: 0,
            settings_bytes: 0, uniform, view_group, targets, strokes, paths, markers, grid, grid_settings: None,grid_plane:crate::cplane::CPlane::Xy,grid_anchor:[0.;3], size: [640, 480], density: 1.0, picker, faces, mesh_ink, sources: Default::default(), pick_mode:Default::default(),pick_cursor:None };
        renderer.mesh_ink.enable_finite(&renderer.device,format,true);
```

**Locate this block:**

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

Replace it with:

```rust
        });
        let view_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view settings"),
            layout: &surfaces.active.0.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
```

**Locate this block:**

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0).into()),
        });
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("mesh settings"), bind_group_layouts: &[Some(&view_layout), Some(&model_layout),Some(&crate::clipping_gpu::layout(&device)),Some(&crate::gpu_geometry::GpuGeometry::storage_layout(&device))], immediate_size: 0 });
        let mesh_pipeline = |depth,primitive_write| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[crate::draw_slots::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(primitive_write))],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let pipeline = mesh_pipeline(crate::depth::surface(),true);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink(),false);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
```

Replace it with:

```rust
        crate::shader_check::check(&device, crate::finite_project_source::source(), "Finite projection");
        crate::shader_check::check(&device, crate::finite_project_source::tiles(), "Finite tiles");
        crate::shader_check::check(&device, crate::finite_project_source::scan(), "Finite scan");
        let view_layout = crate::surface_view::layout(&device);
        let model_layout = crate::gpu_layout::uniform(&device, crate::object_record::BYTES as u64);
        let surfaces=crate::sample_pipelines::Variants::new(crate::surface_pipelines::pipelines(&device,format,&view_layout,&model_layout,1),1);
        let tiles=std::cell::RefCell::new(crate::tile_gpu::Tiles::new(&device,&view_layout));
        let instances=crate::instance_gpu::Lane::new(&device,&queue,format,&view_layout);
        let highlight = crate::highlight::Highlight::new(&device, format);
        let preview = crate::tool_preview::Preview::new(&device, format);
        let widget = crate::widget::Widget::new(&device, format);
```

**Locate this block:**

```rust
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    pipeline: wgpu::RenderPipeline,
    highlight_pipeline: wgpu::RenderPipeline,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

Replace it with:

```rust
    pub queue: wgpu::Queue,
    instances:crate::instance_gpu::Lane,
    instance_rows:std::collections::HashSet<ObjectId>,
    surfaces:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,
    highlight: crate::highlight::Highlight,
    preview: crate::tool_preview::Preview,
    widget: crate::widget::Widget,
```

### 7. `src/instance_gpu.rs`

Extract the existing shared-surface descriptor behind the same count-specific factory interface.

**Locate this existing block:**

```rust
impl Lane {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
// descriptors
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
```

### 8. `src/instance_gpu.rs`

Move the exact previously typed shared shader and pipeline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// descriptors
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-25.rs"
```

### 9. `src/instance_gpu.rs`

Remove the original descriptor copy after moving it. Reuse the shared pipeline pair without rebuilding source geometry, records or slots. Borrow the existing shared record layout in the extracted factory. Match the shared surface pipeline to its actual sample count.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
```

Replace it with:

```rust
    }
    pub fn bind_own(&self,pass:&mut wgpu::RenderPass<'_>){self.slots.bind(pass,0,0);}
    pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipelines.active.0);
        self.records.bind(pass);
        for batch in self.batches.values() {self.slots.bind(pass,0,batch.first_slot);batch.geometry.draw_pulled(pass,batch.count);}
    }
```

**Locate this block:**

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.ordinary_pipeline)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
```

Replace it with:

```rust
        if self.slots.written_bytes>0{self.writes+=1;self.written_bytes+=self.slots.written_bytes;}
        self.batches=batches;self.ordinary=ordinary;Ok(())
    }
    pub fn draw_ordinary(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh)->bool {self.draw_with(pass,mesh,&self.pipelines.active.1)}
    pub fn draw_with(&self,pass:&mut wgpu::RenderPass<'_>,mesh:&GpuMesh,pipeline:&wgpu::RenderPipeline)->bool {
        if !self.bind_record(pass,mesh){return false;}pass.set_pipeline(pipeline);mesh.geometry.draw_pulled(pass,1);true
    }
```

**Locate this block:**

```rust
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let source=source();
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(&layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:Default::default(),multiview_mask:None,cache:None});
        let pipeline=create(None);let ordinary_pipeline=create(Some(wgpu::BlendState::ALPHA_BLENDING));
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipeline,ordinary_pipeline,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
```

Replace it with:

```rust
 crate::clipping_gpu::source(&crate::object_rows::source(include_str!("triangle.wgsl")),2,0)
}
fn pipelines(device:&wgpu::Device,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,samples:u32)->(wgpu::RenderPipeline,wgpu::RenderPipeline) {
        let source=crate::surface_pipelines::sampled(source(),samples);
        let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("shared definition placements"),source:wgpu::ShaderSource::Wgsl(source.into())});
        let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("shared instance settings"),bind_group_layouts:&[Some(view),Some(layout),Some(&crate::clipping_gpu::layout(device)),Some(&GpuGeometry::storage_layout(device))],immediate_size:0});
        let create=|blend|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("shared original definition drawing"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vertex"),compilation_options:Default::default(),buffers:&[crate::draw_slots::layout()]},fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fragment"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL}),Some(crate::primitive_image::target(true))]}),primitive:Default::default(),depth_stencil:Some(crate::depth::surface()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None});
        (create(None),create(Some(wgpu::BlendState::ALPHA_BLENDING)))

}
impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,view:&wgpu::BindGroupLayout)->Self {
        let layout=crate::object_rows::layout(device);
        let pipelines=crate::sample_pipelines::Variants::new(pipelines(device,format,view,&layout,1),1);
        let records=crate::object_rows::Rows::new(&crate::text_gpu_support::GpuCtx::new(device,queue),&layout);
        Self {lookup:None,space:None,pipelines,layout,batches:BTreeMap::new(),records,ordinary:Default::default(),slots:crate::draw_slots::Slots::new(&crate::text_gpu_support::GpuCtx::new(device,queue)),writes:1,written_bytes:8}
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32) {
        self.pipelines.select(samples,||pipelines(device,format,&crate::surface_view::layout(device),&self.layout,samples));
    }
    pub fn set(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,meshes:&mut[GpuMesh],instances:&std::collections::HashSet<crate::scene::ObjectId>,geometry_changed:bool)->Result<(),&'static str> {
        let compatible=geometry_changed||self.space.as_ref().is_some_and(|space|meshes.iter().all(|mesh|self.ordinary.get(&mesh.id).is_some_and(|(slot,row)|*slot>0&&*slot<space.end()&&space.local_slot(*slot)==[*row,mesh.primitive_base()])));
```

**Locate this block:**

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipeline:wgpu::RenderPipeline,ordinary_pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

Replace it with:

```rust
use crate::object_record::{BYTES, Record};

struct Batch {geometry:Rc<GpuGeometry>,count:u32,first_slot:u32}
pub struct Lane {pub(crate) lookup:Option<crate::primitive_slot_table::Table>,pub(crate) space:Option<crate::slot_space::SlotSpace>,pipelines:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,layout:wgpu::BindGroupLayout,batches:BTreeMap<usize,Batch>,records:crate::object_rows::Rows,pub(crate) ordinary:std::collections::HashMap<crate::scene::ObjectId,(u32,u32)>,pub(crate) slots:crate::draw_slots::Slots,pub writes:u64,pub written_bytes:u64}
pub(crate) fn batch_rows(rows:impl IntoIterator<Item=(crate::scene::ObjectId,usize,bool)>,limit:usize)->std::collections::HashSet<crate::scene::ObjectId> {
 let rows=rows.into_iter().collect::<Vec<_>>();let mut uses=std::collections::HashMap::new();
 for &(_,key,_)in &rows {*uses.entry(key).or_insert(0usize)+=1;}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/global_outline_tests.rs`

Keep existing assertions on the same active ordinary pipeline.

**Locate this existing block:**

```text
r.pipeline
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-35.rs"
```

### 11. `src/lib.rs`

Register the two small pipeline recipe modules.

**Locate this existing block:**

```rust
mod camera_navigation_tests;
```

After that block, type:

```rust

mod sample_pipelines;
mod surface_pipelines;
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/global_outline_tests.rs`

Retain all unchanged outline and arena assertions after the ordinary surface owner moves.

**Locate this existing block:**

```text
use crate::{scene::Scene,prepared::PreparedMesh,cap_gpu_tests::{renderer,pixels,matrix,wait}};
use std::rc::Rc;
fn width_pixels(r:&super::Renderer){let texture=r.device.create_texture(&wgpu::TextureDescriptor{label:Some("global CSS width"),size:wgpu::Extent3d{width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});r.draw(&texture.create_view(&Default::default()),&Default::default(),&session_rust::Xform::identity().m.map(|v|v as f32));wait(r);}
#[test]
fn actual_global_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths=Vec::new();
    for curved in [false,true] {
        let mut scene=Scene::demo();scene.clear();let id=if curved {scene.insert_cad(Rc::new(session_rust::BRep::create_cylinder(0.4,0.8))).unwrap()} else {let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.8,0.8,0.2)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,0.5);scene.insert(prepared).unwrap()};let mut renderer=renderer(&scene);renderer.set_outlines(true);assert!(!renderer.meshes.iter().any(crate::gpu_mesh::GpuMesh::selected));let _=id;let geometry=renderer.stats();let mut css=Vec::new();
        for density in [1.,2.] {renderer.set_density(density);width_pixels(&renderer);let alpha=renderer.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));let row=&alpha[120*320..121*320];let width=row[160..].iter().map(|v|*v as f64/255.).sum::<f64>()/density as f64;assert!((0.7..1.7).contains(&width),"Actual outer silhouette CSS coverage remains bounded: {width}");css.push(width);assert_eq!(&renderer.stats()[..2],&geometry[..2]);}
        assert!((css[0]-css[1]).abs()<0.5,"DPI keeps the outline in CSS units: {css:?}");widths.push(css);
    }
    for i in 0..2 {assert!((widths[0][i]-widths[1][i]).abs()<0.25,"A smooth cylinder and flat box use one coverage width: {widths:?}");}
}

#[test]
fn actual_global_and_selected_outlines_share_alpha_and_one_black_blend(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();
 let original=scene.objects()[0].geometry().unwrap().to_proto();let mut r=renderer(&scene);let plain=pixels(&r);let uploads=r.stats()[0];r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().filter(|v|**v>0).count()>300);let usage=r.global_outline_usage();assert_eq!(pixels(&r),global);assert_eq!(r.global_outline_usage()[1],usage[1]);assert!(r.global_outline_usage()[2]>usage[2]);
 r.select_objects(&scene,&[owner]).unwrap();let both=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"same source and CSS width yield identical masks");
 let mut boundary=0;for(i,(&coverage,(a,b)))in alpha.iter().zip(global.chunks_exact(4).zip(both.chunks_exact(4))).enumerate(){if coverage>0&&plain[i*4..i*4+3]==[255,255,255]{boundary+=1;assert_eq!(a,b,"selected plus global uses max alpha in one blend");}}
 assert!(boundary>200);assert_eq!(r.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
 r.request_pick([0.,0.],matrix()).unwrap();wait(&r);let hit=r.take_pick(matrix()).expect("mapped original pick").unwrap();assert_eq!(hit.unwrap()[0],owner.value());
 r.select_objects(&scene,&[]).unwrap();r.set_outlines(false);assert_eq!(r.global_outline_usage(),[0;4]);assert_eq!(pixels(&r),plain);
 scene.clear();r.set_scene(&scene,None);r.set_outlines(true);pixels(&r);assert_eq!(r.global_outline_usage(),[0;4]);
}
#[test]
fn actual_visible_cap_coverage_keeps_selected_and_global_masks_equal(){
 let(mut scene,owner)=crate::cap_gpu_tests::scene(false);let mut r=renderer(&scene);r.set_outlines(true);let global=pixels(&r);let alpha=r.global_outline_alpha();assert!(alpha.iter().any(|v|*v==255));
 r.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&r);assert_eq!(r.outline_alpha(),r.global_outline_alpha(),"selected visible cap marker joins original coverage");
 for(i,v)in alpha.iter().enumerate(){if *v==255{assert!(global[i*4..i*4+3].iter().all(|v|*v<12));assert!(selected[i*4..i*4+3].iter().all(|v|*v<12));}}
 r.set_clipping(false,true);pixels(&r);assert_eq!(r.global_outline_alpha(),alpha,"horizontal cut and uncut original share the same outer XY silhouette");assert_eq!(r.cap_usage()[0],16);
 let side=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Yz,&[[0.,0.,0.]],0.9).unwrap();crate::plane_original::insert(&mut scene,Rc::new(side)).unwrap();r.set_scene(&scene,Some(owner));r.set_clipping(true,true);pixels(&r);assert_ne!(r.global_outline_alpha(),alpha,"changed section inputs cannot reuse stale coverage");
}
#[test]
fn actual_global_outline_includes_locked_and_releases_hidden_originals(){
 let mut scene=Scene::empty();let mut source=crate::cap_gpu_tests::source(false);source.is_locked=true;let owner=scene.insert(PreparedMesh::new(source).unwrap()).unwrap();let mut r=renderer(&scene);r.set_outlines(true);pixels(&r);assert!(r.global_outline_alpha().iter().any(|v|*v==255));assert_eq!(scene.pick_id(owner.value()),None);
 Rc::make_mut(&mut scene.view).hidden.insert(owner);r.set_scene(&scene,None);pixels(&r);assert!(r.global_outline_alpha().is_empty());assert_eq!(r.global_outline_usage(),[0;4]);
}

#[test]
fn actual_triangle_ranges_keep_selection_updates_visibility_and_original_owners(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    let bases=|gpu:&super::Renderer|gpu.meshes.iter().map(|row|u32::from_ne_bytes(row.packed()[crate::object_record::RANGE+4..crate::object_record::RANGE+8].try_into().unwrap())).collect::<Vec<_>>();assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes.iter().all(|row|!row.selected()));let baseline=pixels(&gpu);
    gpu.select_objects(&scene,&[b]).unwrap();assert_eq!(bases(&gpu),vec![1,2]);assert_eq!(gpu.meshes.iter().map(|row|row.selected()).collect::<Vec<_>>(),vec![false,true]);let selected=pixels(&gpu);assert_ne!(selected,baseline);
    scene.translate(b,[0.,0.1,0.]).unwrap();gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1,2]);assert!(gpu.meshes[1].selected());for(position,owner)in[([-0.5,-0.1],a),([0.5,0.],b)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
    let before=gpu.meshes[1].packed();assert!(gpu.meshes[1].set_primitive_base(&gpu.queue,crate::primitive_image::CAP_START).is_err());assert_eq!(gpu.meshes[1].packed(),before);
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,Some(b));assert_eq!(bases(&gpu),vec![1]);assert_eq!(gpu.meshes[0].id,b);assert!(gpu.meshes[0].selected());pixels(&gpu);
}

fn primitive_pixels(gpu:&super::Renderer,texture:&wgpu::Texture)->Vec<u32>{
        let width=texture.width();let height=texture.height();let pitch=(width*4+255)/256*256;
        let read=gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("actual primitive addresses"),size:u64::from(pitch)*u64::from(height),usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});let mut encoder=gpu.device.create_command_encoder(&Default::default());encoder.copy_texture_to_buffer(texture.as_image_copy(),wgpu::TexelCopyBufferInfo{buffer:&read,layout:wgpu::TexelCopyBufferLayout{offset:0,bytes_per_row:Some(pitch),rows_per_image:Some(height)}},texture.size());gpu.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|result|tx.send(result).unwrap());wait(gpu);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let values=bytes.chunks_exact(pitch as usize).flat_map(|row|row[..width as usize*4].chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap()))).collect();drop(bytes);read.unmap();values
    }
#[test]
fn actual_primitive_mrt_keeps_placed_intervals_and_overlay_addresses(){

    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    for density in [1.,2.]{gpu.set_density(density);pixels(&gpu);let before=primitive_pixels(&gpu,&gpu.targets.primitive.texture);assert_eq!(before[0],0);assert_eq!(before[120*320+80],2);assert_eq!(before[120*320+240],3);assert_eq!(gpu.targets.primitive.texture.format(),crate::primitive_image::FORMAT);
        let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner:a,part:crate::pick_sources::Part::Face(key)})).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),before,"A derived selected-face mesh must retain original triangle addresses");
        gpu.set_highlight(&scene,None).unwrap();
    }
    let mut instances=Scene::empty();crate::scene_document::import(&mut instances,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();gpu.set_scene(&instances,None);pixels(&gpu);let placed=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let ids=gpu.meshes.iter().map(|mesh|mesh.primitive_base()+1).collect::<Vec<_>>();assert_eq!(ids.len(),2);assert_ne!(ids[0],ids[1]);assert!(ids.iter().all(|id|*id>0&&*id<crate::primitive_image::CAP_START));assert_eq!(placed[120*320+64],ids[0]);assert_eq!(placed[120*320+256],ids[1]);
    gpu.resize(crate::viewport::Viewport{width:640,height:480});assert_eq!([gpu.targets.primitive.texture.width(),gpu.targets.primitive.texture.height()],[640,480]);gpu.resize(crate::viewport::Viewport{width:320,height:240});gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|id|*id==0));
}

#[test]
fn actual_first_pick_primitives_match_current_cropped_owners_before_color(){
    let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle([[-0.2,-0.4,-0.4],[0.2,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let b=scene.copy_row(a,false).unwrap();scene.translate(a,[-0.5,0.,0.]).unwrap();scene.translate(b,[0.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);
    assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
    for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
        for(position,owner,primitive)in[([-0.5,0.],a,2u32),([0.5,0.],b,3)]{gpu.request_pick(position,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();assert_eq!([texture.width(),texture.height()],window.draw[2..]);assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],primitive);}
    }
    Rc::make_mut(&mut scene.view).hide(&[a],true);gpu.set_scene(&scene,None);gpu.request_pick([0.5,0.],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],b.value());let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let pixels=primitive_pixels(&gpu,&texture);let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];assert_eq!(pixels[(y*texture.width()+x)as usize],2,"Visible survivor uses its current retained run without a color frame");
}

fn cap_primitive_frame(gpu:&super::Renderer){
 let texture=gpu.device.create_texture(&wgpu::TextureDescriptor{label:Some("cap primitive fixture"),size:wgpu::Extent3d{width:gpu.size[0],height:gpu.size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
 gpu.draw(&texture.create_view(&Default::default()),&Default::default(),&matrix());wait(gpu);
}
#[test]
fn actual_cap_color_primitives_keep_plane_selection_and_empty_pixels(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});cap_primitive_frame(&gpu);
  let words=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let at=(96*density*320*density+208*density)as usize;
  assert_eq!(words[at],0xfffffff0);assert_eq!(words[0],0);
  gpu.select_objects(&scene,&[owner]).unwrap();cap_primitive_frame(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture)[at],0xfffffff1);
  gpu.select_objects(&scene,&[]).unwrap();
 }
 gpu.set_scene(&Scene::empty(),None);cap_primitive_frame(&gpu);assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
}

#[test]
fn actual_first_cap_pick_has_reserved_primitive_and_exact_original_owner(){
 let(scene,owner)=crate::cap_gpu_tests::scene(false);let mut gpu=renderer(&scene);
 for density in [1u32,2]{gpu.set_density(density as f32);gpu.resize(crate::viewport::Viewport{width:320*density,height:240*density});
  assert!(primitive_pixels(&gpu,&gpu.targets.primitive.texture).iter().all(|word|*word==0));
  gpu.request_pick([0.3,0.2],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
  let(texture,window)=gpu.picker.primitive_for_tests().unwrap();let words=primitive_pixels(&gpu,&texture);
  let x=window.copy[0]+window.cursor[0]-window.draw[0];let y=window.copy[1]+window.cursor[1]-window.draw[1];
  assert_eq!(words[(y*texture.width()+x)as usize],0xfffffff0);
  assert!(texture.width()<window.full[0]&&texture.height()<window.full[1]);
 }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases(){
 let scene=Scene::demo();let mut gpu=renderer(&scene);let source=scene.objects()[0].mesh.clone();let original=source.vertices().to_vec();
 let buffers=gpu.meshes[0].geometry.buffers();
 assert_eq!(gpu.meshes[0].geometry.allocated_bytes(),0);
 assert_eq!(gpu.usage()[1],buffers.0.size()+buffers.1.size(),"One arena owner accounts for the actual shared buffers");
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let mut cache=crate::geometry_cache::GeometryCache::default();let uploaded=cache.get(&ctx,&source);
 assert_eq!(uploaded.buffers().0.size(),source.vertices().len()as u64*20);
 assert_eq!(cache.uploaded_bytes,cache.allocated_bytes(),"The first nonempty compact upload matches actual GPU allocation");
 let written=cache.uploaded_bytes;cache.get(&ctx,&source);assert_eq!(cache.uploads,1);assert_eq!(cache.uploaded_bytes,written);
 let empty=crate::gpu_geometry::GpuGeometry::upload(&gpu.device,std::rc::Rc::new(crate::mesh::Mesh::new(vec![],vec![]).unwrap()));assert_eq!(empty.allocated_bytes(),24);
 pixels(&gpu);assert_eq!(source.vertices(),original);let uploads=gpu.stats();pixels(&gpu);assert_eq!(gpu.stats(),uploads);
 gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.arena_cpu_bytes(),0);
}
#[test]
fn actual_nonzero_shared_ranges_keep_surface_face_picks_projection_outlines_and_caps() {
    use std::cell::RefCell;
    use crate::{gpu_geometry::GpuGeometry,gpu_mesh::GpuMesh,text_gpu_support::GpuCtx};
    for clipped in [false,true] {
        let scene=if clipped {crate::cap_gpu_tests::scene(false).0} else {
            let mut scene=Scene::empty();scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();scene
        };
        let mut r=renderer(&scene);r.set_outlines(true);
        let expected=pixels(&r);
        let mut hits=Vec::new();
        for mode in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face] {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            hits.push(r.take_pick(matrix()).unwrap().unwrap());
        }
        assert!(hits.iter().all(Option::is_some));
        if !clipped {assert_ne!(hits[1].unwrap()[1],0,"The uncut source returns an original face address");}
        let ctx=GpuCtx::new(&r.device,&r.queue);
        let arena=Rc::new(RefCell::new(crate::mesh_arena::Arena::new(&ctx)));
        let decoy=Rc::new(crate::mesh::Mesh::new(vec![[0.7,0.7,-0.6,0.,1.,0.],[0.9,0.7,-0.6,0.,1.,0.],[0.8,0.9,-0.6,0.,1.,0.]],vec![0,1,2]).unwrap());
        arena.borrow_mut().upload(&ctx,&decoy).unwrap();
        let layout=r.surfaces.active.0.get_bind_group_layout(1);
        for mesh in &mut r.meshes {
            let object=scene.objects().iter().find(|object|object.id==mesh.id).unwrap();
            let geometry=Rc::new(GpuGeometry::shared(&ctx,object.mesh.clone(),arena.clone()).unwrap());
            assert!(geometry.index_start>=3);
            *mesh=GpuMesh::with_geometry(&r.device,&layout,object,super::style(&scene,object.id,false),geometry);
        }
        r.caps.borrow_mut().set_scene(&r.queue,&scene,&mut r.meshes,&r.clipping.borrow().rows);
        let mut base=0;for mesh in &mut r.meshes {mesh.set_primitive_base(&r.queue,base).unwrap();base+=mesh.geometry.index_count/3;}
        r.tiles.borrow_mut().set(&r.device,&r.meshes);r.faces.clear();
        if let Some(outline)=r.global_outline.borrow_mut().as_mut(){outline.invalidate();}
        assert_eq!(pixels(&r),expected,"Nonzero shared ranges preserve actual color, clipping and outline pixels");
        for (mode,expected) in [crate::pick_sources::Mode::Object,crate::pick_sources::Mode::Face].into_iter().zip(hits) {
            r.set_pick_mode(mode);r.request_pick([0.,0.],matrix()).unwrap();wait(&r);
            assert_eq!(r.take_pick(matrix()).unwrap().unwrap(),expected,"Shared source addressing preserves original picks");
        }
        assert!(r.tiles.borrow().usage()[3]>0);
        if clipped {assert_eq!(r.cap_usage()[1],1);}
    }
}

#[test]
fn actual_contact_marker_is_opaque_unlit_selected_and_keeps_original_ownership(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();
 let vertices=prepared.display.vertices().len();prepared.display=prepared.display.with_alphas(vec![1./255.;vertices]).unwrap();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);
 let source=scene.objects()[0].mesh.clone();let color=source.vertices()[0][3..6].to_vec();let expected=color.iter().map(|v|(v.clamp(0.,1.)*255.).round()as u8).collect::<Vec<_>>();
 let uploads=gpu.stats()[0];for opacity in [1.,0.5,0.] {gpu.set_opacity(opacity);let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],expected.as_slice());gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());}
 gpu.select_objects(&scene,&[owner]).unwrap();let image=pixels(&gpu);assert_eq!(&crate::cap_gpu_tests::at(&image,160,90)[..3],&[255,255,0]);
 assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);assert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn current_palette_is_shared_by_surface_shader_and_preserves_original_picks_and_color(){
 use wgpu::naga;
 let rgba=crate::colour::selection();assert_eq!(rgba,session_rust::Color::yellow().to_unified_array());assert_eq!(rgba,[1.,1.,0.,1.]);
 let shader=crate::clipping_gpu::source(include_str!("triangle.wgsl"),2,0);assert!(shader.contains(&crate::colour::selection_shader()));naga::valid::Validator::new(naga::valid::ValidationFlags::all(),naga::valid::Capabilities::all()).validate(&naga::front::wgsl::parse_str(&shader).unwrap()).unwrap();
 let mut scene=Scene::empty();let prepared=PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap();let original=prepared.geometry.to_proto();let owner=scene.insert(prepared).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let plain=pixels(&gpu);let uploads=gpu.stats()[0];
 gpu.select_objects(&scene,&[owner]).unwrap();let selected=pixels(&gpu);assert!(selected.chunks_exact(4).any(|p|p[0]>10&&p[0]==p[1]&&p[2]==0));assert_ne!(plain,selected);gpu.request_pick([0.,0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());
 gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),plain);assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn full_primitive_base_survives_surface_selection_and_reserved_range_checks(){
 assert!(include_str!("clip.wgsl").contains(&format!("const CAP_PRIMITIVE: u32 = 0x{:08x}u;",crate::primitive_image::CAP_START)));
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.4],[0.4,-0.4,-0.4],[0.,0.4,-0.4]],[0.8,0.2,0.1]).unwrap()).unwrap();let mut gpu=renderer(&scene);let original=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 for base in [0x8000_0000,crate::primitive_image::CAP_START-2]{
  gpu.meshes[0].set_primitive_base(&gpu.queue,base).unwrap();pixels(&gpu);let plain=primitive_pixels(&gpu,&gpu.targets.primitive.texture);assert_eq!(plain[120*320+160],base+1);
  gpu.select_objects(&scene,&[owner]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),plain);assert_eq!(u32::from_ne_bytes(gpu.meshes[0].packed()[64..68].try_into().unwrap()),1);
  gpu.select_objects(&scene,&[]).unwrap();pixels(&gpu);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),plain);let before=gpu.meshes[0].packed();for invalid in [crate::primitive_image::CAP_START-1,crate::primitive_image::CAP_START,u32::MAX]{assert!(gpu.meshes[0].set_primitive_base(&gpu.queue,invalid).is_err());assert_eq!(gpu.meshes[0].packed(),before);}
 }
 assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);
}

#[test]
fn actual_ordinary_surfaces_keep_one_own_row_slot_through_instances_highlights_and_release(){
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=[crate::draw_slots::OWN_ROW,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();assert!(gpu.instances.slots.allocated_bytes()>=8);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);let originals=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let owner=scene.objects()[0].id;
 for opacity in [1.,0.5]{gpu.set_opacity(opacity);let plain=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();gpu.set_highlight(&scene,Some(crate::pick_sources::Target{owner,part:crate::pick_sources::Part::Face(key)})).unwrap();assert_ne!(pixels(&gpu),plain);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_highlight(&scene,None).unwrap();assert_eq!(pixels(&gpu),plain);}
 assert_eq!(scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>(),originals);gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(gpu.usage(),[0;4]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);assert_eq!(crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,8),bytes);
}

#[test]
fn actual_surface_batches_share_one_slot_owner_and_restore_ordinary_drawing(){
 use prost::Message;let mut source=crate::instance_specimen::message();let mut second=source.definitions.as_ref().unwrap().meshes[0].clone();second.guid="other-definition".into();source.definitions.as_mut().unwrap().meshes.push(second);source.objects.as_mut().unwrap().instances[1].definition_guid="other-definition".into();source.objects.as_mut().unwrap().instances[0].xform=None;source.objects.as_mut().unwrap().instances[1].flags=0;source.xforms=vec![session_rust::proto::XformEntry {guid:"first".into(),xform:Some(session_rust::Xform::translation(-0.6,0.,0.).to_proto())},session_rust::proto::XformEntry {guid:"second".into(),xform:Some(session_rust::Xform::translation(0.6,0.,0.).to_proto())}];
 let mut scene=crate::scene::Scene::empty();crate::scene_document::import(&mut scene,&source.encode_to_vec(),"",&session_rust::Xform::identity(),0.).unwrap();let mut gpu=crate::cap_gpu_tests::renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(&gpu.instance_usage()[..3],&[2,2,16760]);assert_eq!(gpu.instance_allocations(),[352,24,16384]);assert_eq!(gpu.instances.slots.instances(),2);let slots=crate::growing_buffer_tests::read(&ctx,&gpu.instances.slots.buffer.buf,24);assert_eq!(&slots[..8],&[u32::MAX,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(u32::from_ne_bytes(slots[8..12].try_into().unwrap()),0);assert_eq!(u32::from_ne_bytes(slots[16..20].try_into().unwrap()),1);
 let records=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),2*crate::object_record::BYTES as u64);
 let mut actual=records.chunks_exact(crate::object_record::BYTES).map(|r|r.to_vec()).collect::<Vec<_>>();actual.sort_unstable();let mut expected=gpu.meshes.iter().map(|m|m.packed().to_vec()).collect::<Vec<_>>();expected.sort_unstable();assert_eq!(actual,expected,"global records retain both original placements, IDs and ranges");
 let baseline=crate::cap_gpu_tests::pixels(&gpu);
 for half in 0..2 {let blue=baseline.chunks_exact(4).enumerate().filter(|(i,p)|(*i%320)/160==half&&p[2]>p[0].saturating_add(50)&&p[2]>p[1].saturating_add(30)).count();assert!(blue>100,"placed original blue triangles in half {half}: {blue}");}
 let uploads=gpu.stats()[0];let owner=scene.objects()[0].id;gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 gpu.set_opacity(0.5);crate::cap_gpu_tests::pixels(&gpu);gpu.set_opacity(1.);assert_eq!(crate::cap_gpu_tests::pixels(&gpu),baseline);gpu.set_scene(&crate::scene::Scene::empty(),None);assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_projected_records_follow_physical_gaps_clear_old_ranges_and_bound_fallback(){
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[-0.4,-0.4,0.],[0.4,-0.4,0.],[0.,0.4,0.]],[0.2,0.4,0.8]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);let baseline=pixels(&gpu);let triangles=gpu.meshes[0].geometry.index_count/3;assert_eq!(gpu.meshes.len(),1);let initial=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],u64::from(triangles)*96);let uploads=gpu.stats()[0];
 gpu.meshes[0].set_primitive_base(&gpu.queue,7).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+7));let projected=gpu.tile_buffers()[0].clone();let data=crate::growing_buffer_tests::read(&ctx,&projected,u64::from(triangles+7)*96);assert!(data[..7*96].iter().all(|v|*v==0));assert_eq!(&data[7*96..],initial);let projections=gpu.tile_usage()[5];pixels(&gpu);assert_eq!(gpu.tile_usage()[5],projections);
 gpu.meshes[0].set_primitive_base(&gpu.queue,3).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles+3));assert_eq!(gpu.tile_buffers()[0],projected);let data=crate::growing_buffer_tests::read(&ctx,&gpu.tile_buffers()[0],projected.size());assert!(data[..3*96].iter().all(|v|*v==0));assert_eq!(&data[3*96..(triangles as usize+3)*96],initial);assert!(data[(triangles as usize+3)*96..].iter().all(|v|*v==0));
 gpu.meshes[0].set_primitive_base(&gpu.queue,0).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.tile_usage()[3],u64::from(triangles));let current=gpu.tile_buffers()[0].clone();assert_eq!(current.size(),initial.len()as u64);assert_eq!(crate::growing_buffer_tests::read(&ctx,&current,current.size()),initial);assert_eq!(gpu.stats()[0],uploads);
 gpu.meshes[0].set_primitive_base(&gpu.queue,0x80000000).unwrap();gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);pixels(&gpu);assert!(gpu.tiles.borrow().fallback);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
 gpu.set_scene(&Scene::empty(),None);pixels(&gpu);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_ordinary_storage_records_match_legacy_pixels_and_primitives_through_growth_selection_and_release() {
 let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 assert_eq!(gpu.instances.ordinary.len(),1);assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64),gpu.meshes[0].packed());let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);
 gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),1);
 let b=scene.insert(PreparedMesh::triangle([[-0.2,-0.2,-0.4],[0.2,-0.2,-0.4],[0.,0.2,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();scene.translate(b,[0.6,0.,0.]).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),2);let wanted=gpu.meshes.iter().flat_map(|m|m.packed()).collect::<Vec<_>>();assert_eq!(crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),wanted.len()as u64),wanted);
 let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored);assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[a]).unwrap();assert_ne!(pixels(&gpu),stored);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);gpu.set_opacity(0.5);pixels(&gpu);gpu.set_opacity(1.);assert_eq!(pixels(&gpu),stored);
 for opacity in [0.,0.5,0.95,1.] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact ordinary blend at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);}
 let copy=scene.copy_row(b,false).unwrap();scene.translate(copy,[0.3,0.1,0.]).unwrap();let source=scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary.len(),3);let uploads=gpu.stats()[0];
 for opacity in [0.,0.5,0.95] {gpu.set_opacity(opacity);gpu.set_scene(&scene,None);let stored=pixels(&gpu);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);gpu.instances.ordinary.clear();assert_eq!(pixels(&gpu),stored,"Exact transparent placements at {opacity}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);gpu.set_scene(&scene,None);gpu.select_objects(&scene,&[copy]).unwrap();if opacity>0. {assert_ne!(pixels(&gpu),stored);}gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),stored);assert_eq!(gpu.stats()[0],uploads);}
 assert_eq!(scene.objects().iter().find(|o|o.id==b).unwrap().geometry().unwrap().to_proto(),source);gpu.set_opacity(1.);gpu.set_scene(&scene,None);pixels(&gpu);
 let location=|gpu:&super::Renderer,id:crate::scene::ObjectId|{let space=gpu.instances.space.as_ref().unwrap();let row=gpu.instances.ordinary[&id].1;space.draws().into_iter().flat_map(|draw|draw.slots).find(|slot|space.local_slot(*slot)[0]==row).unwrap()};let first_slot=location(&gpu,a);
 let retained=gpu.instances.ordinary.clone();assert!(scene.remove(b));gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 let fresh=scene.insert(PreparedMesh::triangle([[0.7,-0.3,-0.4],[0.9,-0.3,-0.4],[0.8,-0.1,-0.4]],[0.1,0.4,0.9]).unwrap()).unwrap();gpu.set_scene(&scene,None);assert_eq!(gpu.instances.ordinary[&fresh].1,retained[&b].1);assert_eq!(gpu.instances.ordinary[&a].1,retained[&a].1);assert_eq!(gpu.instances.ordinary[&copy].1,retained[&copy].1);assert_eq!(location(&gpu,a),first_slot);
 for mesh in &gpu.meshes {let row=gpu.instances.ordinary[&mesh.id].1;let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),(row as u64+1)*crate::object_record::BYTES as u64);assert_eq!(&bytes[row as usize*crate::object_record::BYTES..],&mesh.packed());}pixels(&gpu);
 let projected=gpu.tiles.borrow().projected.clone();let expected=crate::growing_buffer_tests::read(&ctx,&projected,projected.size());let image=pixels(&gpu);gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert_eq!(pixels(&gpu),image);let uniform=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&uniform,uniform.size()),expected);gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);let restored=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&restored,restored.size()),expected);
 scene.clear();gpu.set_scene(&scene,None);assert!(gpu.instances.ordinary.is_empty());assert!(gpu.instances.space.is_none());assert_eq!(&gpu.instance_usage()[..3],&[0,0,0]);assert_eq!(gpu.instances.slots.allocated_bytes(),8);
}

#[test]
fn actual_lookup_projection_groups_reuse_current_inputs_and_replace_output_bindings() {
 let mut scene=Scene::empty();scene.insert(PreparedMesh::triangle([[0.,0.,0.],[1.,0.,0.],[0.,1.,0.]],[0.1,0.4,0.9]).unwrap()).unwrap();let mut gpu=renderer(&scene);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let count=gpu.meshes[0].primitive_base()+1;assert_eq!(count,2);
 let mut state=crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).unwrap();assert_eq!(state.bytes(),16);assert_eq!(crate::growing_buffer_tests::read(&ctx,&state.parameters,16),[2u32,1,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>());
 let pipes=crate::tile_pipelines::Pipelines::new(&gpu.device,&gpu.pipeline.get_bind_group_layout(0));let output=|size|gpu.device.create_buffer(&wgpu::BufferDescriptor{label:Some("lookup projection output witness"),size,usage:wgpu::BufferUsages::STORAGE,mapped_at_creation:false});let first=output(192);let group=state.group(&gpu.device,&pipes.lookup_project_layout,&first);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&first),group);let larger=output(384);let next=state.group(&gpu.device,&pipes.lookup_project_layout,&larger);assert_ne!(next,group);assert_eq!(state.group(&gpu.device,&pipes.lookup_project_layout,&larger),next);
 assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,None,count).is_none());assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());gpu.instances.ordinary.clear();assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),count).is_none());scene.clear();gpu.set_scene(&scene,None);assert!(crate::projection_lookup::Bindings::new(&gpu.device,&gpu.meshes,Some(&gpu.instances),0).is_none());
}

#[test]
fn actual_lookup_projection_matches_legacy_bytes_keeps_selection_cache_and_releases() {
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::new(crate::cap_gpu_tests::source(false)).unwrap()).unwrap();let other=scene.insert(PreparedMesh::triangle([[0.1,0.1,-0.3],[0.3,0.1,-0.3],[0.2,0.3,-0.3]],[0.1,0.4,0.9]).unwrap()).unwrap();let copy=scene.copy_row(other,false).unwrap();scene.translate(copy,[0.4,0.,0.]).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(0.95);gpu.set_scene(&scene,None);let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);
 let image=pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_some());let output=gpu.tiles.borrow().projected.clone();let wanted=crate::growing_buffer_tests::read(&ctx,&output,output.size());let uploads=gpu.stats()[0];gpu.tiles.borrow_mut().set(&gpu.device,&gpu.meshes);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(pixels(&gpu),image);let output=gpu.tiles.borrow().projected.clone();assert_eq!(crate::growing_buffer_tests::read(&ctx,&output,output.size()),wanted);
 gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),image);assert!(gpu.tiles.borrow().lookup.is_some());let projections=gpu.tile_usage()[5];gpu.select_objects(&scene,&[owner]).unwrap();assert_ne!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);gpu.select_objects(&scene,&[]).unwrap();assert_eq!(pixels(&gpu),image);assert_eq!(gpu.tile_usage()[5],projections);assert_eq!(gpu.stats()[0],uploads);
 scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert!(gpu.tiles.borrow().lookup.is_none());assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);
}

#[test]
fn actual_arena_reclaims_history_retained_sources_and_keeps_survivor_pixels_and_owners(){
 let left=[[-0.8,-0.4,-0.4],[-0.4,-0.4,-0.4],[-0.6,0.4,-0.4]];let right=[[0.4,-0.4,-0.4],[0.8,-0.4,-0.4],[0.6,0.4,-0.4]];let mut scene=Scene::empty();let a=scene.insert(PreparedMesh::triangle(left,[0.9,0.1,0.1]).unwrap()).unwrap();let b=scene.insert(PreparedMesh::triangle(right,[0.1,0.8,0.2]).unwrap()).unwrap();let source=Rc::clone(&scene.objects()[0].mesh);let precise=scene.objects()[0].geometry().unwrap().to_proto();let mut history=crate::history::History::default();let mut gpu=renderer(&scene);gpu.set_opacity(1.);let first=pixels(&gpu);let expired=Rc::downgrade(&gpu.meshes[0].geometry);let survivor=gpu.meshes[1].geometry.index_start;assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(2,144));
 history.edit(&mut scene,|scene|{assert!(scene.remove(a));});gpu.set_scene(&scene,None);assert!(expired.upgrade().is_none());assert!(history.scenes().any(|saved|saved.contains(a)));assert!(Rc::strong_count(&source)>0);let mut c=None;history.edit(&mut scene,|scene|{c=Some(scene.insert(PreparedMesh::triangle(left,[0.1,0.3,0.9]).unwrap()).unwrap());});let c=c.unwrap();gpu.set_scene(&scene,None);let reused=gpu.meshes.iter().find(|mesh|mesh.id==c).unwrap();assert_eq!(reused.geometry.index_start,0);assert_eq!(gpu.meshes.iter().find(|mesh|mesh.id==b).unwrap().geometry.index_start,survivor);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(3,216));let next=pixels(&gpu);let l=(120*320+64)*4;let r=(120*320+256)*4;assert!(next[l+2]>next[l].saturating_add(50));assert_eq!(&next[r..r+4],&first[r..r+4]);for(point,id)in[([-0.6,0.],c),([0.6,0.],b)]{gpu.request_pick(point,matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],id.value());}
 assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(history.undo(&mut scene));gpu.set_scene(&scene,None);assert!(Rc::ptr_eq(&source,&scene.objects().iter().find(|row|row.id==a).unwrap().mesh));assert_eq!(scene.objects().iter().find(|row|row.id==a).unwrap().geometry().unwrap().to_proto(),precise);assert_eq!(pixels(&gpu),first);assert_eq!((gpu.geometry.uploads,gpu.geometry.uploaded_bytes),(4,288));scene.clear();gpu.set_scene(&scene,None);pixels(&gpu);assert_eq!(gpu.geometry.entries(),0);assert_eq!(gpu.geometry.allocated_bytes(),0);assert_eq!(gpu.usage(),[0;4]);assert_eq!(&gpu.tile_usage()[..5],&[96,16,48,0,0]);assert!(history.scenes().count()>0);
}

#[test]
fn actual_precise_object_rebase_matches_local_pixels_records_selection_and_owners(){
 let mut local=Scene::empty();let owner=local.insert(PreparedMesh::triangle([[-0.2,-0.3,-0.4],[0.2,-0.3,-0.4],[0.,0.3,-0.4]],[0.1,0.3,0.9]).unwrap()).unwrap();local.translate(owner,[0.125,-0.25,0.]).unwrap();let mut gpu=renderer(&local);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(baseline.chunks_exact(4).filter(|p|p[2]>p[0].saturating_add(50)).count()>100);let source=local.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e12,-1e12,1e12];let mut far=local.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[1,12]);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.stats()[0],uploads);
 let ctx=crate::text_gpu_support::GpuCtx::new(&gpu.device,&gpu.queue);let bytes=crate::growing_buffer_tests::read(&ctx,gpu.instances.record_buffer(),crate::object_record::BYTES as u64);assert_eq!(&bytes[48..60],&[0.125f32,-0.25,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert_eq!(&bytes[..crate::object_record::BYTES],&gpu.meshes[0].packed());
 gpu.select_objects(&far,&[owner]).unwrap();assert_ne!(pixels(&gpu),baseline);gpu.request_pick([0.125,-0.25],matrix()).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(matrix()).unwrap().unwrap().unwrap()[0],owner.value());gpu.select_objects(&far,&[]).unwrap();assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.meshes[0].rebase(&gpu.queue,anchor),[0,0]);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
 far.translate(owner,[0.001,0.,0.]).unwrap();gpu.set_scene(&far,None);let expected=crate::render_space::relative(far.objects()[0].model.m[12..15].try_into().unwrap(),anchor);assert_eq!(&gpu.meshes[0].packed()[48..60],&expected.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());assert!(expected[0]>0.125);assert_eq!(gpu.stats()[0],uploads);
}

#[test]
fn actual_derived_frame_origin_keeps_cap_pixels_world_text_clips_and_precise_ground(){
 let mut scene=Scene::empty();let mut prepared=PreparedMesh::new(session_rust::Mesh::create_box(0.5,0.5,0.5)).unwrap();prepared.model=session_rust::Xform::translation(0.,0.,-0.5);let owner=scene.insert(prepared).unwrap();let plane=crate::clipping_geometry::plane_from(crate::clipping_geometry::Mode::Xy,&[[0.,0.,-0.5]],1.).unwrap();let plane_id=crate::plane_original::insert(&mut scene,Rc::new(plane)).unwrap();let mut gpu=renderer(&scene);gpu.set_opacity(1.);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let baseline=pixels(&gpu);assert!(crate::cap_gpu_tests::at(&baseline,160,120)[..3].iter().all(|v|(150..=156).contains(v)));let ground=gpu.ambient_rows.ground_at([0.;3]);let source=scene.objects()[0].geometry().unwrap().to_proto();let uploads=gpu.stats()[0];
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [owner,plane_id]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();assert!(gpu.space.update(anchor,1.,0.).unwrap().moved);for mesh in &mut gpu.meshes {mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert_eq!(gpu.anchor(),anchor);assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);assert_eq!(gpu.ambient_rows.ground[0]-anchor[2],-0.75);let frame=gpu.text_frame(matrix());assert_eq!(frame.origin,anchor);assert!(!frame.cut([anchor[0],anchor[1],anchor[2]-0.75]));assert!(frame.cut([anchor[0],anchor[1],anchor[2]-0.25]));assert_eq!(pixels(&gpu),baseline);let writes=gpu.clipping.borrow().writes;assert_eq!(pixels(&gpu),baseline);assert_eq!(gpu.clipping.borrow().writes,writes);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);
}

#[test]
fn actual_cloud_and_sheet_origins_reuse_uploads_and_invalidate_equal_matrix_views(){
 let mut scene=Scene::empty();let mut source=session_rust::PointCloud::from_coords(vec![-0.5,0.,-0.5],vec![255,0,0,255],vec![]);source.point_size=16.;let source=Rc::new(source);let cloud=scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.,-0.5,0.75,0.,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let original=source.to_proto();let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.distance=2.;camera.aspect=4./3.;camera.projection=crate::camera::Projection::Orthographic;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]<20&&p[2]<20));assert!(baseline.chunks_exact(4).filter(|p|p[..3].iter().all(|v|*v<20)).count()>20);let uploads=(gpu.cloud_usage()[2],gpu.sheet_usage()[3]);assert_eq!(uploads,(1,1));
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [cloud,drawing]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);let mut far_camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut far_camera,[0.;3]);far_camera.distance=camera.distance;far_camera.aspect=camera.aspect;far_camera.projection=camera.projection;far_camera.target=anchor;controlled_translation_plane(&mut far_camera,anchor);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();let local=far_camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);let prepared=gpu.cloud_usage()[3];assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.cloud_usage()[3],prepared);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];far_camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,moved).unwrap();let same_matrix=far_camera.view_projection_anchored(moved).to_f32();assert_eq!(same_matrix,local);let panned=crate::cap_gpu_tests::pixels_at(&gpu,same_matrix);assert!(panned.chunks_exact(4).all(|p|p==&[255;4]));assert!(gpu.cloud_usage()[3]>prepared,"Origin alone invalidates the equal-matrix cloud view");assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);
 far_camera.target=anchor;controlled_translation_plane(&mut far_camera,anchor);assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);gpu.cloud_camera(&far_camera);gpu.sheets.set_anchored(&gpu.device,&gpu.queue,&far,&[],&gpu.sources.sheet_bases,anchor).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);for(point,id)in[([-0.5,0.,-0.5],cloud),([0.5,0.,-0.5],drawing)]{let point=camera.view_projection().transform_point(&session_rust::Point::new(point[0],point[1],point[2]));gpu.request_pick([point[0]as f32,point[1]as f32],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],id.value());}assert_eq!(source.to_proto(),original);assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));assert_eq!((gpu.cloud_usage()[2],gpu.sheet_usage()[3]),uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.cloud_usage(),[0;5]);assert_eq!(&gpu.sheet_usage()[..2],&[0,0]);
}

fn temporary_inputs(gpu:&mut super::Renderer,world:[f64;3]){
 let point=|p:[f64;3]|std::array::from_fn(|i|p[i]+world[i]);let mut guides=crate::guide_gpu::Overlay::at(gpu.anchor());guides.wire(&[point([-0.75,0.25,-0.5]),point([0.75,0.25,-0.5])],[210./255.,40./255.,40./255.,1.],true);guides.mark(point([0.5,0.25,-0.5]),6.,[0.,0.,1.,1.]);guides.label=Some((point([0.5,0.25,-0.5]),"guide".into()));gpu.set_guides(&guides).unwrap();let mark=crate::measure_mark::Mark{points:vec![point([-0.5,-0.25,-0.5]),point([0.5,-0.25,-0.5])],label:"distance".into()};gpu.set_measurement(Some(&mark)).unwrap();let mut draft=crate::tool::Draft::new(crate::tool::Shape::Line);draft.submit(point([0.,0.,-0.5])).unwrap();draft.hover(Some(point([0.5,0.,-0.5]))).unwrap();gpu.set_preview(Some(&draft)).unwrap();
}
#[test]
fn actual_temporary_ink_origins_restore_pixels_invalidate_caches_and_release(){
 let scene=Scene::empty();let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();temporary_inputs(&mut gpu,[0.;3]);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[0]>200&&p[1]>200&&p[2]<30));assert!(baseline.chunks_exact(4).any(|p|p[0]>150&&p[1]<80&&p[2]<80));assert!(baseline.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>200);let bytes=(gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes());assert!(bytes.0>0&&bytes.1>0&&bytes.2>0);assert_eq!(gpu.text_error(),None);
 let anchor=[1e9,-1e9,1e9];assert!(gpu.space.update(anchor,2.,0.).unwrap().moved);camera.target=anchor;controlled_translation_plane(&mut camera,anchor);temporary_inputs(&mut gpu,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!((gpu.guide_bytes(),gpu.preview_bytes(),gpu.measurement.bytes()),bytes);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];camera.target=moved;assert!(gpu.space.update(moved,2.,300.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));camera.target=anchor;controlled_translation_plane(&mut camera,anchor);assert!(gpu.space.update(anchor,2.,600.).unwrap().moved);temporary_inputs(&mut gpu,anchor);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert!(scene.all_ids().is_empty());gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap(),None);
 gpu.set_preview(None).unwrap();gpu.set_measurement(None).unwrap();gpu.set_guides(&crate::guide_gpu::Overlay::at(anchor)).unwrap();crate::cap_gpu_tests::pixels_at(&gpu,local);assert_eq!(gpu.preview_bytes(),0);assert_eq!(gpu.guide_bytes(),0);assert_eq!(gpu.measurement.bytes(),0);assert_eq!(gpu.text_usage()[0],0);
}

#[test]
fn actual_source_grid_origin_cache_keeps_surface_pixels_owners_and_release(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();let settings=Some(crate::grid::Settings{spacing:0.1,half_cells:5});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();gpu.set_scene(&far,None);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();gpu.markers.set_points(&gpu.device,std::iter::empty()).unwrap();gpu.space.update(anchor,2.,0.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let far_plane=plane([anchor[0],anchor[1],anchor[2]-0.75]);gpu.set_grid_on(settings,far_plane).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);let grid_uploads=gpu.grid_usage()[2];gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(gpu.grid_usage()[2],grid_uploads);
 let moved=[anchor[0]+2000.,anchor[1],anchor[2]];gpu.space.update(moved,2.,300.).unwrap();camera.target=moved;for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,moved);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert!(gpu.grid_usage()[2]>grid_uploads);assert_eq!(camera.view_projection_anchored(moved).to_f32(),local);assert!(crate::cap_gpu_tests::pixels_at(&gpu,local).chunks_exact(4).all(|p|p==&[255;4]));gpu.space.update(anchor,2.,600.).unwrap();for mesh in &mut gpu.meshes{mesh.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_grid_on(settings,far_plane).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);gpu.request_pick([0.,0.],local).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_grid(None).unwrap();gpu.set_scene(&Scene::empty(),None);assert_eq!(&gpu.grid_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_initial_object_records_keep_precise_local_pixels_updates_and_sources(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.2,-0.25,-0.5],[0.3,-0.25,-0.5],[0.1,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);gpu.mesh_ink.set(&gpu.device,std::iter::empty()).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload(&gpu.device,&layout,&scene.objects()[0],false);gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=2.;camera.aspect=4./3.;let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let original=gpu.meshes[0].packed();let uploads=gpu.stats()[0];let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();far.translate(owner,anchor).unwrap();let layout=gpu.pipeline.get_bind_group_layout(1);gpu.meshes[0]=crate::gpu_mesh::GpuMesh::upload_at(&gpu.device,&layout,&far.objects()[0],false,anchor);assert_eq!(gpu.meshes[0].packed(),original);let geometry=gpu.meshes[0].geometry.clone();let style=crate::gpu_mesh::Style{source_faces:true,selected:false,owner:owner.value(),colour:None};gpu.meshes[0]=crate::gpu_mesh::GpuMesh::with_geometry_at(&gpu.device,&layout,&far.objects()[0],style,geometry.clone(),anchor);assert_eq!(gpu.meshes[0].packed(),original);assert!(Rc::ptr_eq(&gpu.meshes[0].geometry,&geometry));gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[0,0]);let selected=crate::gpu_mesh::Style{selected:true,..style};assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],selected),[1,16]);assert_eq!(&gpu.meshes[0].packed()[..64],&original[..64]);assert_eq!(gpu.meshes[0].update(&gpu.queue,&far.objects()[0],style),[1,16]);assert_eq!(gpu.meshes[0].packed(),original);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);assert_eq!(gpu.stats()[0],uploads);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_original_highlights_keep_local_pixels_addresses_and_sources_after_rebasing(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.4,0.5,-0.5,0.4,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.4,-0.5,-0.5),session_rust::Point::new(0.,-0.3,-0.5),session_rust::Point::new(0.4,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let mut proto=session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![],vec![]).to_proto();proto.point_ids=vec![u32::MAX];let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_proto(proto))).unwrap()).unwrap();scene.owner_controls(path).unwrap();scene.cloud_controls(Some(cloud));let curve=scene.insert_path(crate::chain::PreparedChain::curve(Rc::new(crate::freeform_curve::curve(&[session_rust::Point::new(-0.4,0.8,-0.5),session_rust::Point::new(0.,0.65,-0.5),session_rust::Point::new(0.4,0.8,-0.5)]))).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let edge=scene.objects()[0].mesh.parts.edges[0];let control=scene.control_markers().into_iter().find(|c|c.parent==path).unwrap().part;let targets=[Target{owner:mesh,part:Part::Face(key)},Target{owner:mesh,part:Part::MeshEdge(edge.0,edge.1)},Target{owner:line,part:Part::Curve},Target{owner:curve,part:Part::Curve},Target{owner:path,part:Part::PathEdge(0)},Target{owner:path,part:control},Target{owner:cloud,part:Part::CloudPoint(u32::MAX)}];let mut gpu=renderer(&scene);let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;gpu.cloud_camera(&camera);let transform=camera.uniform();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);let uploads=gpu.stats()[0];let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let mut pixels=Vec::new();for target in targets{assert!(target.valid(&scene));gpu.set_highlight(&scene,Some(target)).unwrap();let selected=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(selected!=baseline,"A valid original highlight must change actual pixels: {target:?}");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);pixels.push(selected);}gpu.set_highlight(&scene,None).unwrap();gpu.select_objects(&scene,&[line,path,point]).unwrap();let objects=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert_ne!(objects,baseline);gpu.select_objects(&scene,&[]).unwrap();gpu.set_highlight(&scene,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,transform),baseline);
 let anchor=[1e9,-1e9,1e9];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,curve]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);gpu.space.update(anchor,3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,anchor);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&far);gpu.set_clouds(&far,&[]).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);gpu.cloud_camera(&camera);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);
 gpu.mesh_ink.set(&gpu.device,crate::pick_sources::Sources::anchored(&far,anchor).unwrap().mesh_ink).unwrap();for(target,expected)in targets.into_iter().zip(pixels){assert!(target.valid(&far));gpu.set_highlight(&far,Some(target)).unwrap();let actual=crate::cap_gpu_tests::pixels_at(&gpu,local);assert!(actual==expected,"Distant original highlight pixels must match exactly: {target:?}; differing bytes: {}",actual.iter().zip(&expected).filter(|(a,b)|a!=b).count());assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);}gpu.set_highlight(&far,None).unwrap();gpu.select_objects(&far,&[line,path,point]).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),objects);gpu.select_objects(&far,&[]).unwrap();gpu.set_highlight(&far,None).unwrap();assert_eq!(crate::cap_gpu_tests::pixels_at(&gpu,local),baseline);assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_distant_widgets_keep_local_pixels_scale_picks_and_tile_release(){
 let scene=Scene::empty();let anchor=[1e12,-1e12,1e12];let pivot=[0.125,0.25,-0.5];let mut renderer=renderer(&scene);for projection in [crate::camera::Projection::Perspective,crate::camera::Projection::Orthographic]{for density in [1u32,2]{let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.isometric();camera.projection=projection;camera.aspect=4./3.;let size=[320*density,240*density];renderer.resize(crate::viewport::Viewport{width:size[0],height:size[1]});renderer.set_density(density as f32);renderer.space.forget();let scale=crate::widget_layout::camera_scale(&camera,pivot,240.).unwrap();renderer.set_gizmo(&camera,Some(pivot),-1.);let usage=renderer.gizmo_usage();let extent=renderer.gizmo_extent();let transform=camera.uniform();let baseline=crate::widget_tests::pixels(&mut renderer,&transform,size);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));renderer.space.update(anchor,3.,0.).unwrap();camera.target=anchor;controlled_translation_plane(&mut camera,anchor);let distant=std::array::from_fn(|i|anchor[i]+pivot[i]);assert_eq!(crate::widget_layout::camera_scale(&camera,distant,240.).unwrap(),scale);renderer.set_gizmo(&camera,Some(distant),-1.);let local=camera.view_projection_anchored(anchor).to_f32();assert_eq!(local,transform);let actual=crate::widget_tests::pixels(&mut renderer,&local,size);assert!(actual==baseline,"Precise distant widget pixels differ at density {density}");assert_eq!(renderer.gizmo_extent(),extent);assert_eq!(renderer.gizmo_usage()[2],usage[2]);renderer.request_pick([0.,0.],local).unwrap();crate::cap_gpu_tests::wait(&renderer);assert!(renderer.take_pick(local).unwrap().unwrap().is_none());renderer.set_gizmo(&camera,None,-1.);assert_eq!(renderer.gizmo_usage()[1],0);assert!(renderer.gizmo_extent().is_none());}}
}

#[test]
fn actual_origin_only_pick_views_discard_old_hits_and_restore_owners(){
 let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let source=scene.objects()[0].geometry().unwrap().to_proto();let mut gpu=renderer(&scene);let transform=matrix();let uploads=gpu.stats()[0];gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,gpu.space.anchor());}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));assert!(gpu.take_pick(transform).is_none(),"Identical matrix in a new origin must not accept the old owner");assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none(),"The automatic current-frame retry is empty");gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.request_pick([0.,0.],transform).unwrap();wait(&gpu);assert_eq!(gpu.take_pick(transform).unwrap().unwrap().unwrap()[0],owner.value());assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),source);gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_rebased_pick_finishes_a_released_drag_without_reuploading_sources(){
 use crate::editor::{Editor,Action,Change};let mut editor=crate::camera_defaults_tests::editor_fixture();editor.scene.clear();let owner=editor.scene.insert(PreparedMesh::triangle([[-0.4,-0.4,-0.5],[0.4,-0.4,-0.5],[0.,0.4,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();editor.scene.translate(owner,[2000.,0.,0.]).unwrap();editor.camera.target=[2000.,0.,0.];let original=editor.scene.placement(owner).unwrap().m;let source=editor.scene.objects()[0].geometry().unwrap().clone();let mut gpu=renderer(&editor.scene);let uploads=gpu.stats()[0];let transform=editor.camera.uniform();let size=[320.,240.];editor.apply(Action::CanvasStart{from:[0.,0.],to:[0.2,0.],size,modified:false,pointer:1}).unwrap();gpu.request_pick([0.,0.],transform).unwrap();editor.apply(Action::CanvasRelease{screen:[0.4,0.],size,add:false,remove:false}).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);gpu.space.update(editor.camera.target,editor.camera.distance,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,editor.camera.target);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));let local=editor.camera.view_projection_anchored(gpu.anchor()).to_f32();assert_ne!(local,transform);wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);assert_eq!(editor.scene.placement(owner).unwrap().m,original);wait(&gpu);let hit=gpu.take_pick(local).unwrap().unwrap().and_then(|pixel|gpu.resolve_pick(&editor.scene,pixel));assert_eq!(hit.unwrap().owner,owner);assert_eq!(editor.apply(Action::CanvasHit(hit.map(|target|target.owner))).unwrap(),Change::Scene);assert_ne!(editor.scene.placement(owner).unwrap().m,original);assert!(Rc::ptr_eq(&source,editor.scene.objects()[0].geometry().unwrap()));gpu.set_scene(&editor.scene,editor.selected);assert_eq!(gpu.stats()[0],uploads);editor.apply(Action::Undo).unwrap();assert_eq!(editor.scene.placement(owner).unwrap().m,original);
}
#[test]
fn actual_rebased_pending_cursor_stays_newest_and_cancellation_never_revives_it(){
 let mut scene=Scene::empty();let make=||PreparedMesh::triangle([[-0.18,-0.18,-0.5],[0.18,-0.18,-0.5],[0.,0.18,-0.5]],[0.2,0.4,0.8]).unwrap();let a=scene.insert(make()).unwrap();let b=scene.insert(make()).unwrap();scene.translate(a,[1999.5,0.,0.]).unwrap();scene.translate(b,[2000.5,0.,0.]).unwrap();let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;let local=matrix();gpu.request_pick([-0.5,0.],old).unwrap();gpu.request_pick([0.5,0.],old).unwrap();assert_eq!(&gpu.pick_usage()[..2],&[1,1]);gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert_eq!(gpu.take_pick(local).unwrap().unwrap().unwrap()[0],b.value());assert_eq!(gpu.pick_usage()[3],2);assert!(gpu.take_pick(local).is_none());gpu.request_pick([-0.5,0.],local).unwrap();gpu.cancel_pick();gpu.space.update([0.;3],3.,300.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[0.;3]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));wait(&gpu);assert!(gpu.take_pick(old).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);gpu.request_pick([-0.5,0.],old).unwrap();gpu.set_scene(&Scene::empty(),None);gpu.space.update([2000.,0.,0.],3.,600.).unwrap();wait(&gpu);assert!(gpu.take_pick(local).is_none());assert_eq!(&gpu.pick_usage()[..2],&[0,0]);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_origin_refresh_reuses_selected_face_geometry_and_original_ink(){
 use crate::pick_sources::{Part,Target};let mut scene=Scene::empty();let owner=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();scene.translate(owner,[2000.,0.,0.]).unwrap();let original=scene.objects()[0].geometry().unwrap().to_proto();let key=*scene.objects()[0].geometry().unwrap().face.keys().next().unwrap();let target=Some(Target{owner,part:Part::Face(key)});let mut gpu=renderer(&scene);let mut old=matrix();old[12]=-2000.;gpu.set_highlight(&scene,target).unwrap();let baseline=crate::cap_gpu_tests::pixels_at(&gpu,old);let addresses=primitive_pixels(&gpu,&gpu.targets.primitive.texture);let geometry=gpu.highlight.geometry_for_test().unwrap().clone();let uploads=gpu.stats()[0];gpu.space.update([2000.,0.,0.],3.,0.).unwrap();for row in &mut gpu.meshes{row.rebase(&gpu.queue,[2000.,0.,0.]);}gpu.instances.set(&gpu.device,&gpu.queue,&mut gpu.meshes,&gpu.instance_rows,false).unwrap();gpu.tiles.borrow_mut().set_records(&gpu.device,&gpu.meshes,Some(&gpu.instances));gpu.set_source_ink(&scene);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));let actual=crate::cap_gpu_tests::pixels_at(&gpu,matrix());assert!(actual==baseline,"Rebased original face and ink pixels must match exactly");assert_eq!(primitive_pixels(&gpu,&gpu.targets.primitive.texture),addresses);assert!(gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));assert!(Rc::ptr_eq(gpu.highlight.geometry_for_test().unwrap(),&geometry));assert_eq!(gpu.stats()[0],uploads);assert_eq!(scene.objects()[0].geometry().unwrap().to_proto(),original);drop(geometry);gpu.set_highlight(&scene,None).unwrap();assert!(gpu.highlight.geometry_for_test().is_none());assert!(!gpu.highlight.rebase_face(&gpu.queue,gpu.anchor(),target));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

#[test]
fn actual_shared_origin_keeps_all_original_lanes_and_pending_picks_coherent(){
 let mut scene=Scene::empty();let mesh=scene.insert(PreparedMesh::triangle([[-0.25,-0.25,-0.5],[0.25,-0.25,-0.5],[0.,0.25,-0.5]],[0.2,0.4,0.8]).unwrap()).unwrap();let line=scene.insert_line(crate::stroke::PreparedLine::new(Rc::new(session_rust::Line::new(-0.5,0.5,-0.5,0.5,0.5,-0.5))).unwrap()).unwrap();let path=scene.insert_path(crate::chain::PreparedChain::polyline(Rc::new(session_rust::Polyline::new(vec![session_rust::Point::new(-0.5,-0.5,-0.5),session_rust::Point::new(0.,-0.25,-0.5),session_rust::Point::new(0.5,-0.5,-0.5)]))).unwrap()).unwrap();let point=scene.insert_point(crate::marker::PreparedPoint::new(Rc::new(session_rust::Point::new(0.75,0.,-0.5))).unwrap()).unwrap();let cloud=scene.insert_cloud(crate::cloud::Prepared::new(Rc::new(session_rust::PointCloud::from_coords(vec![-0.75,0.,-0.5],vec![255,0,0,255],vec![]))).unwrap()).unwrap();let sheet=Rc::new(session_rust::proto::Sheet{guid:crate::sheet_example::source().guid,coords:vec![0.25,0.25,-0.5,0.75,0.25,-0.5],widths:vec![0.0625],colors:vec![0xff00_0000],segment_count:1,entity_count:1,source_ids:vec![0],..Default::default()});let drawing=scene.insert_sheet(Rc::new(crate::sheet_source::Prepared::new(sheet.clone()).unwrap())).unwrap();let text_source=Rc::new(crate::scene_label::example(crate::text::TextPlacement::Anchor{world:[-0.5,0.75,-0.5],offset:[0.;2]}));let text=scene.insert_text(crate::scene_label::Prepared::new(text_source.clone()).unwrap()).unwrap();let ids=[line,path,point,cloud,drawing,text];let original=scene.objects()[0].geometry().unwrap().to_proto();let mut camera=crate::camera_defaults_tests::top_fixture();controlled_translation_plane(&mut camera,[0.;3]);camera.projection=crate::camera::Projection::Orthographic;camera.distance=3.;camera.aspect=4./3.;let mut gpu=renderer(&scene);gpu.select_objects(&scene,&ids).unwrap();let settings=Some(crate::grid::Settings{spacing:0.125,half_cells:4});let plane=|origin|crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]};gpu.set_grid_on(settings,plane([0.,0.,-0.75])).unwrap();assert!(!gpu.prepare_origin(&camera,&scene,None,&ids,0.).unwrap().moved);let transform=gpu.camera_matrix(&camera);let baseline=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(baseline.chunks_exact(4).any(|p|p[..3]!=[255;3]));let ground=gpu.ambient_rows.ground_at([0.;3]);let uploads=gpu.stats()[0];let anchor=[2000.,-2000.,2000.];let mut far=scene.clone();for id in [mesh,line,path,point,cloud,drawing,text]{far.translate(id,anchor).unwrap();}gpu.set_scene(&far,None);camera.target=anchor;controlled_translation_plane(&mut camera,anchor);gpu.set_grid_on(settings,plane([anchor[0],anchor[1],anchor[2]-0.75])).unwrap();let frame=gpu.prepare_origin(&camera,&far,None,&ids,300.).unwrap();assert!(frame.moved&&!frame.pending);assert_eq!(gpu.camera_matrix(&camera),transform);let actual=crate::cap_gpu_tests::pixels_at(&gpu,transform);assert!(actual==baseline,"Shared original lanes must retain exact local/distant pixels");assert_eq!(gpu.ambient_rows.ground_at(anchor),ground);let writes=gpu.stats();assert!(!gpu.prepare_origin(&camera,&far,None,&ids,301.).unwrap().moved);assert_eq!(gpu.stats(),writes);gpu.request_pick([0.,0.],transform).unwrap();camera.target[0]+=2000.;let pending=gpu.prepare_origin(&camera,&far,None,&ids,350.).unwrap();assert!(!pending.moved&&pending.pending);assert_eq!(gpu.rebase_delay(350.),Some(151));assert_eq!(&gpu.pick_usage()[..2],&[1,0]);let moved=gpu.prepare_origin(&camera,&far,None,&ids,501.).unwrap();assert!(moved.moved&&!moved.pending);assert_eq!(gpu.camera_matrix(&camera),transform);assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).is_none());assert_eq!(&gpu.pick_usage()[..2],&[1,0]);wait(&gpu);assert!(gpu.take_pick(transform).unwrap().unwrap().is_none());camera.target=anchor;controlled_translation_plane(&mut camera,anchor);assert!(gpu.prepare_origin(&camera,&far,None,&ids,802.).unwrap().moved);let restored=crate::cap_gpu_tests::pixels_at(&gpu,gpu.camera_matrix(&camera));assert!(restored==baseline,"Restored shared origin must restore exact pixels");assert_eq!(gpu.stats()[0],uploads);assert_eq!(far.objects()[0].geometry().unwrap().to_proto(),original);assert!(Rc::ptr_eq(&far.texts()[0].prepared.source,&text_source));assert!(Rc::ptr_eq(&far.sheets()[0].prepared.original,&sheet));gpu.set_scene(&Scene::empty(),None);assert_eq!(gpu.usage(),[0;4]);
}

fn controlled_translation_plane(camera:&mut crate::camera::Camera,origin:[f64;3]){camera.grid_plane=Some(crate::cplane::CPlane::Frame{origin,x:[1.,0.,0.],y:[0.,1.,0.]});camera.radius=20000.;}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-37.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/sample_surface_tests.rs`

Render actual ordinary and shared surfaces at both counts, inspect every physical depth/address, require exact return and cached pipeline identity, preserve original records and GPU geometry, then freshly query both original shared owners.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-38.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/sample_test_proof.rs`

Export actual Native diagnostic pixels only when the caller opts into a proof directory; run all assertions without filesystem output by default.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-39.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/renderer.rs`

Register the actual Native surface witness.

**Locate this existing block:**

```text
nt_contact_tests.rs"]
mod ambient_contact_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="frame_target_tests.rs"]
mod frame_target_tests;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-40.rs" skip=150
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 16. `src/lib.rs`

Keep optional diagnostic exports Native-only.

**Locate this existing block:**

```text
ach_tests;

#[cfg(test)]
mod camera_navigation_tests;

mod sample_pipelines;
mod surface_pipelines;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzx-reuse-count-specific-surface-pipelines-41.rs" skip=100
```

## <a id="checkpoint-96zzzzzzy-generate-ink-for-physical-samples"></a>5. Generate ink shaders for physical samples

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/ink_plane.wgsl, src/fin…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Explicit sample argument</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">sample-qualified pixel co…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">count-specific shader and…</text>
</svg>
</div>

**What this code does:** Carry an explicit physical sample through finite-ink visibility and generate colour shaders whose coordinates, depth, primitive addresses and pipeline descriptor use the same sample count.

### 1. `src/ink_plane.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
```

Replace it with:

```wgsl
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis,sample:u32)->bool{
 let fringe=ink_primitive(pixel,sample);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),sample);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
```

### 2. `src/finite_visibility.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Replace the entire file with:

```wgsl
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{
 let primitive=ink_primitive(pixel,sample);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32,sample:u32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,sample:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative,sample);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative,sample);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative,sample);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool,sag:vec2<f32>,sample:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags,sag,sample);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool,sample:u32)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance,sample)||!ink_disc_visible(pixel,centre,depth,sample);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32,sag:vec2<f32>,sample:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags,sample);}
 var axis=finite_ink_axis(pixel,ends,depths);axis.sag=sag;let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance,sample)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
 let plane_visible=ink_visible_plane(fit_at,axis,sample);
 if(finite_tiles[0].x==0u){return plane_visible;}
 if(finite_source_hidden(fit_at,axis,sample)){return false;}
 if(plane_visible){return true;}
 if(any(axis.at<vec2<f32>(0.))||any(axis.at>=view.size)){return false;}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size_px=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(axis.at/span);let head=finite_tiles[1u+cell.y*size_px.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return false;}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);
  if(nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let bounds=finite_projected[primitive-1u].bounds;
  if(any(axis.at<bounds.xy-vec2<f32>(SLOPE_PX))||any(axis.at>bounds.zw+vec2<f32>(SLOPE_PX))){continue;}
  if(finite_triangle_hides(primitive,axis)){return false;}
 }
 return true;
}

fn finite_hidden_alpha(hidden:bool,picking:bool)->f32{
 if(picking){return select(1.,0.,hidden);}
 let glass=view.opacity>0.&&view.opacity<1.;
 return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

### 3. `src/finite_stroke.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Replace the entire file with:

```wgsl
#include "near_clip.wgsl"
#include "contact_depth.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32, display:vec4<f32>, eye:vec4<f32> }
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32,@location(8) metadata:InkRecord) -> Output { return geometry(start, end, colour, width, index,metadata); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(input.rise,0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 4. `src/finite_chain.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.));if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Replace it with:

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index,flags); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,false,vec2<f32>(0.),sample);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 5. `src/finite_marker.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

Replace the entire file with:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output,@builtin(sample_index) sample:u32) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false,sample);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

### 6. `src/finite_stroke_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

**Locate this existing block:**

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.))<=0.){discard;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(input.rise,0.),0u)<=0.){discard;}
```

### 7. `src/finite_chain_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

**Locate this existing block:**

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.))<=0.){discard;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags,true,vec2<f32>(0.),0u)<=0.){discard;}
```

### 8. `src/finite_marker_pick.wgsl`

Carry the explicit physical sample through original finite-ink visibility; single-sample queries continue passing0.

**Locate this existing block:**

```wgsl
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true,0u)<=0.){discard;}
```

### 9. `src/finite_sources.rs`

Generate only four-sample actual depth/address access and sample-qualified coverage; retain the original one-sample marker position.

Replace the entire file with:

```rust
pub fn source(stride:u64,pick:bool)->String{source_style(stride,pick,false)}
pub fn source_style(stride:u64,pick:bool,grid:bool)->String{source_metadata(stride,pick,grid,false)}
pub fn source_metadata(stride:u64,pick:bool,grid:bool,varying:bool)->String{source_with_samples(stride,pick,grid,varying,1)}
pub fn source_with_samples(stride:u64,pick:bool,grid:bool,varying:bool,samples:u32)->String{
 assert!(matches!(samples,1|4)&&(!pick||samples==1));
 let(base,ids)=match stride{44=>(include_str!("finite_stroke.wgsl"),include_str!("finite_stroke_pick.wgsl")),72=>(include_str!("finite_chain.wgsl"),include_str!("finite_chain_pick.wgsl")),32=>(include_str!("finite_marker.wgsl"),include_str!("finite_marker_pick.wgsl")),_=>panic!("Unsupported finite ink layout")};
 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl")).replace("#include \"ink_plane.wgsl\"",include_str!("ink_plane.wgsl"));
 let base=if samples==4{multisampled(base,stride)}else{base};
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 let(ty,words)=if varying{("vec2<u32>","m")}else{("u32","vec2<u32>(m,0x80008000u)")};
 let base=format!("alias InkRecord={ty};\nfn ink_words(m:InkRecord)->vec2<u32>{{return {words};}}\n{base}");
 let source=if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base};crate::clipping_gpu::source(&source,if pick{3}else{2},0)
}

fn multisampled(mut source:String,stride:u64)->String {
    source=source.replace("var finite_depth:texture_depth_2d;","var finite_depth:texture_depth_multisampled_2d;")
        .replace("var finite_primitive:texture_2d<u32>;","var finite_primitive:texture_multisampled_2d<u32>;")
        .replace("textureLoad(finite_depth,at,0)","textureLoad(finite_depth,at,i32(sample))")
        .replace("textureLoad(finite_primitive,at,0)","textureLoad(finite_primitive,at,i32(sample))")
        .replace("@interpolate(linear) finite_pixel:","@interpolate(linear,sample) finite_pixel:")
        .replace("@interpolate(linear) pixel:","@interpolate(linear,sample) pixel:");
    if stride==32 {
        source=source.replace("@location(10) @interpolate(flat) finite_depths:vec2<f32>,",
            "@location(10) @interpolate(flat) finite_depths:vec2<f32>,\n    @location(11) @interpolate(linear,sample) finite_pixel:vec2<f32>,")
            .replace("colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0))", "colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0))")
            .replace("Output(position, colour, radius,flags,axis,depths)","Output(position, colour, radius,flags,axis,depths,finite_screen(position))")
            .replace("input.position.xy","input.finite_pixel");
    }
    source
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/tile_shader_tests.rs`

Retain every original compute/colour/query shader recipe and validate both counts across all six finite-ink styles with the same parser and full Naga assertions.

**Locate this existing block:**

```text
for stride in [44,72,32]{sources.push(("finite color",crate::finite_sources::source(stride,false)));sources.push(("finite pick",crate::finite_sources::source(stride,true)));}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzy-generate-ink-for-physical-samples-35.rs"
```

### 11. `src/finite_ink.rs`

Carry the requested count from the pipeline descriptor into its shader recipe. Assemble the shader for the same count as the pipeline. Use the explicit count in the finite-ink pipeline descriptor. Keep current colour construction single-sample until the owner is connected. Keep the independent integer query factory single-sample.

Replace the entire file with:

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,samples:u32,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_with_samples(stride,picking,grid,varying,samples);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,1,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,1,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

## <a id="checkpoint-96zzzzzzz-cache-ink-pipelines-with-independent-queries"></a>6. Cache matching ink pipelines and bindings

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/fini…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Shared static vertex attr…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached count switch</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fresh independent query b…</text>
</svg>
</div>

**What this code does:** Retain matching finite-ink colour pipelines and binding layouts for both sample counts while keeping integer queries independent and sharing the original vertex-layout constants.

### 1. `src/stroke_gpu.rs`

Declare one shared static vertex-layout recipe; retain the original field offsets and strides.

**Locate this existing block:**

```rust
pub struct Lane {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
 const STROKE:[wgpu::VertexAttribute;4]=ATTRIBUTE_0;
 const CHAIN:[wgpu::VertexAttribute;7]=ATTRIBUTE_1;
 const MARKER:[wgpu::VertexAttribute;3]=ATTRIBUTE_2;
 const GUIDE:[wgpu::VertexAttribute;5]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32];
 match stride {44=>&STROKE,72=>&CHAIN,32=>&MARKER,48=>&GUIDE,_=>unreachable!()}
}

pub struct Lane {
```

### 2. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
ATTRIBUTE_0
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-02.rs"
```

### 3. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
ATTRIBUTE_1
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-03.rs"
```

### 4. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
ATTRIBUTE_2
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-04.rs"
```

### 5. `src/stroke_gpu.rs`

Use the shared static layout without changing the original vertex data. Reuse the same static guide, wide-line and grid vertex layout.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Self::layout_depth(device, format, source, stride, vertices_per_instance, attributes, crate::depth::ink(), true)
    }

    pub fn guides(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);Self::layout_depth(device,format,&crate::finite_project_source::expand(include_str!("guide.wgsl")),48,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32],depth,true)}
    pub fn overlay(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
        let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
        Self::layout_depth(device,format,&crate::stroke::shader(),44,6,&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],depth,true)
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(device, format, include_str!("grid.wgsl"), 44, 2,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32], crate::depth::grid(), false, true)
    }

    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

Replace it with:

```rust
        Self::layout_depth(device, format, source, stride, vertices_per_instance, attributes, crate::depth::ink(), true)
    }

    pub fn guides(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);Self::layout_depth(device,format,&crate::finite_project_source::expand(include_str!("guide.wgsl")),48,6,attributes(48),depth,true)}
    pub fn overlay(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
        let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
        Self::layout_depth(device,format,&crate::stroke::shader(),44,6,attributes(44),depth,true)
    }
    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_target(device, format, include_str!("grid.wgsl"), 44, 2,
            attributes(44), crate::depth::grid(), false, true)
    }

    fn layout_target(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

**Locate this block:**

```rust

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::stroke::shader(), 44, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32])
    }

    pub fn joined(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::chain::shader(), 72, 18,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32,
                4 => Float32x3, 5 => Float32x3, 6 => Uint32])
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::marker::shader(), 32, 6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32, 2 => Float32x4])
    }

    fn layout(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

Replace it with:

```rust

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::stroke::shader(), 44, 6,
            attributes(44))
    }

    pub fn joined(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::chain::shader(), 72, 18,
            attributes(72))
    }

    pub fn markers(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout(device, format, &crate::marker::shader(), 32, 6,
            attributes(32))
    }

    fn layout(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

### 6. `src/finite_ink.rs`

Extract the binding recipe behind an explicit count parameter.

**Locate this existing block:**

```rust
impl Ink{
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::stroke_gpu::attributes;
fn layout(device:&wgpu::Device,samples:u32)->wgpu::BindGroupLayout {
// original layout
 layout
}
impl Ink{
```

### 7. `src/finite_ink.rs`

Move the exact existing finite-ink binding descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original layout
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-12.rs"
```

### 8. `src/finite_ink.rs`

Remove the original descriptor copy after moving it. Match the depth binding to its physical sample count. Match the primitive-address binding without averaging integer words. Retain only count-specific colour pipeline/layout recipes and one independent query recipe. Retain existing one-sample callers and accept a matching initial colour count. Borrow the shared static attributes instead of allocating a second vector. Build matching colour resources while retaining the single-sample original-owner query. Compile the alternate recipe only once and release stale image bindings on a real switch. Build current-colour and independent query bindings from their own layouts. Use the requested binding layout. Cache only current colour bindings. Draw with the selected colour recipe. Keep original queries on their single-sample layout.

**Locate this existing block:**

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,samples:u32,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_with_samples(stride,picking,grid,varying,samples);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:wgpu::RenderPipeline,pick:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
use crate::stroke_gpu::attributes;
fn layout(device:&wgpu::Device,samples:u32)->wgpu::BindGroupLayout {
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
 layout
}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{
  let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,false)]});
let color=pipeline(device,format,stride,grid,false,varying,1,&view,&crop,&layout,&attributes);let pick=pipeline(device,format,stride,grid,true,varying,1,&view,&crop,&layout,&attributes);Self{color,pick,layout,bindings:RefCell::new(None)}
 }
 fn group(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout:&self.layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,samples:u32,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_with_samples(stride,picking,grid,varying,samples);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
  }
use std::cell::RefCell;
pub struct Ink{color:crate::sample_pipelines::Variants<(wgpu::RenderPipeline,wgpu::BindGroupLayout)>,pick:wgpu::RenderPipeline,query:wgpu::BindGroupLayout,spec:(wgpu::TextureFormat,u64,bool,bool),bindings:RefCell<Option<(wgpu::Buffer,wgpu::Buffer,wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)>>}
#[derive(Clone)]
pub struct Tables{pub projected:wgpu::Buffer,pub tiles:wgpu::Buffer,pub primitive:wgpu::TextureView}
use crate::stroke_gpu::attributes;
fn layout(device:&wgpu::Device,samples:u32)->wgpu::BindGroupLayout {
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("finite ink coverage"),entries:&[crate::tile_pipelines::entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),crate::tile_pipelines::entry(1,wgpu::ShaderStages::FRAGMENT,wgpu::BufferBindingType::Storage{read_only:true}),wgpu::BindGroupLayoutEntry{binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture{sample_type:wgpu::TextureSampleType::Depth,view_dimension:wgpu::TextureViewDimension::D2,multisampled:samples==4},count:None},crate::text_gpu_support::texture_entry(4,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Uint,samples==4)]});
 layout
}
impl Ink{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool)->Self{Self::with_samples(device,format,stride,grid,varying,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,varying:bool,samples:u32)->Self{
  assert!(matches!(samples,1|4));
  let attrs=attributes(stride);
  let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
let query=layout(device,1);let layout=if samples==1 {query.clone()}else{layout(device,samples)};
let color=pipeline(device,format,stride,grid,false,varying,samples,&view,&crop,&layout,attrs);let pick=pipeline(device,format,stride,grid,true,varying,1,&view,&crop,&query,attrs);Self{color:crate::sample_pipelines::Variants::new((color,layout),samples),pick,query,spec:(format,stride,grid,varying),bindings:RefCell::new(None)}
 }
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
  let (format,stride,grid,varying)=self.spec;
  if self.color.select(samples,|| {
   let layout=if samples==1 {self.query.clone()}else{layout(device,samples)};
   let view=crate::surface_view::layout(device);let crop=crate::gpu_layout::uniform(device,80);
   (pipeline(device,format,stride,grid,false,varying,samples,&view,&crop,&layout,attributes(stride)),layout)
  }) {self.bindings.get_mut().take();}
 }
 pub fn clear_bindings(&mut self){self.bindings.get_mut().take();}
 fn group(&self,device:&wgpu::Device,layout:&wgpu::BindGroupLayout,tables:&Tables,depth:&wgpu::TextureView)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("finite original ink coverage"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:tables.projected.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:tables.tiles.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&tables.primitive)}]})}
 pub fn set(&self,device:&wgpu::Device,tables:&Tables,depth:&wgpu::TextureView){let mut bindings=self.bindings.borrow_mut();if bindings.as_ref().is_some_and(|(p,t,d,i,_)|p==&tables.projected&&t==&tables.tiles&&d==depth&&i==&tables.primitive){return}let group=self.group(device,&self.color.active.1,tables,depth);*bindings=Some((tables.projected.clone(),tables.tiles.clone(),depth.clone(),tables.primitive.clone(),group));}
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color.active.0);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));pass.draw(0..verts,0..count);true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView){let group=self.group(device,&self.query,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/finite_ink.rs`

Register actual Native cache, physical-colour and independent-query witnesses.

**Locate this existing block:**

```text
pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-26.rs" skip=160
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/finite_sample_tests.rs`

Exercise actual six-family GPU colour coverage0..4, fresh independent original queries, exact return and cached pipeline/layout identities; optional proof exports remain portable.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-27.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/ink_pixels_tests.rs`

Exercise actual six-family GPU colour coverage0..4, fresh independent original queries, exact return and cached pipeline/layout identities; optional proof exports remain portable.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-28.rs"
```

## <a id="checkpoint-96zzzzzzza-retarget-existing-overlay-lanes"></a>7. Retarget existing overlay lanes

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/high…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Requested count</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching colour recipe</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">existing overlay owners</text>
</svg>
</div>

**What this code does:** Apply one requested sample count through existing overlay lanes while retaining their original vertices, IDs, query pipelines and previously compiled colour recipes.

### 1. `src/stroke_gpu.rs`

Add only count-specific sample coordinates and the existing colour factory interface.

**Locate this existing block:**

```rust
pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn sampled(source:&str,stride:u64)->String {
 let source=source.replace("@interpolate(linear)","@interpolate(linear,sample)");
 if stride!=32 {return source;}
 source.replace("@location(2) @interpolate(flat) radius: f32,","@location(2) @interpolate(flat) radius: f32,\n    @location(3) @interpolate(linear,sample) pixel:vec2f,")
 .replace("colour, vec2<f32>(0.0), 0.0)","colour, vec2<f32>(0.0), 0.0,vec2f(0.))")
 .replace("Output(position, colour, centre, radius)","Output(position, colour, centre, radius,(position.xy/position.w*vec2f(0.5,-0.5)+0.5)*view.size)")
 .replace("input.position.xy-input.centre","input.pixel-input.centre")
}
fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,source:&str,stride:u64,attributes:&[wgpu::VertexAttribute],depth:wgpu::DepthStencilState,physical:bool,samples:u32)->wgpu::RenderPipeline {
 let sampled=if samples==4 {Some(sampled(source,stride))}else{None};
 let source=sampled.as_deref().unwrap_or(source);
 let view_layout=crate::surface_view::layout(device);
// layout
// shader
// render
 pipeline
}
pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
```

### 2. `src/stroke_gpu.rs`

Move the exact previously typed layout descriptor into the shared colour factory.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// layout
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzza-retarget-existing-overlay-lanes-02.rs"
```

### 3. `src/stroke_gpu.rs`

Move the exact previously typed shader descriptor into the shared colour factory.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// shader
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzza-retarget-existing-overlay-lanes-03.rs"
```

### 4. `src/stroke_gpu.rs`

Move the exact previously typed render descriptor into the shared colour factory.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// render
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzza-retarget-existing-overlay-lanes-04.rs"
```

### 5. `src/stroke_gpu.rs`

Remove the constructor descriptor copy after moving its exact bytes. Remove the original shader copy after moving it. Use one cached colour recipe and retain the original query constructor. Match the colour descriptor to its actual sample count. Cache only pipeline recipes and retain the pending count. Adopt the requested count only after actual vertices arrive. Reconcile a late-created finite owner with the pending count. Retain vertices and IDs, delay empty variants, clear stale finite bindings and reuse compiled recipes. Skip empty finite lanes before binding images of a different sample count. Draw the selected colour recipe. Retain the immutable format/depth recipe and initial count.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

Replace it with:

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, recipe:(format,depth,physical),samples:1, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

**Locate this block:**

```rust
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = pickable.then(||device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) }));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let mut targets=vec![Some(wgpu::ColorTargetState{format,blend:if physical{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})];
        if physical{targets.push(Some(crate::primitive_image::target(true)));}
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
```

Replace it with:

```rust
    ) -> Self {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = pickable.then(||device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) }));
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let pipeline=crate::sample_pipelines::Variants::new(pipeline(device,format,source,stride,attributes,depth.clone(),physical,1),1);
        let pick_pipeline = pick_shader.as_ref().map(|pick_shader|device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
```

**Locate this block:**

```rust
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,varying:bool){self.varying=varying;self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth,varying));self.reset_flags(device).expect("Original ink clipping flags");}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }
```

Replace it with:

```rust
        if flags==self.flag_data{return Ok(())}use wgpu::util::DeviceExt;self.flags=if flags.is_empty(){None}else{let bytes=flags.iter().copied().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("original ink clipping flags"),contents:&bytes,usage:wgpu::BufferUsages::VERTEX}))};self.flag_data=flags;Ok(())
    }
    pub fn clipping_bytes(&self)->[u64;2]{[self.flags.as_ref().map_or(0,wgpu::Buffer::size),self.flag_data.capacity()as u64*4]}
    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat,varying:bool){self.varying=varying;self.finite=Some(crate::finite_ink::Ink::new(device,format,self.stride,self.strict_depth,varying));self.reset_flags(device).expect("Original ink clipping flags");self.set_samples(device,self.samples);}
    pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
        assert!(matches!(samples,1|4));self.samples=samples;if self.data.is_empty(){if let Some(ink)=&mut self.finite{ink.clear_bindings();}return;}
        if let Some(ink)=&mut self.finite {ink.set_samples(device,samples);return;}
        let (format,depth,physical)=&self.recipe;
        self.pipeline.select(samples,|| {
            let source=if *physical{include_str!("grid.wgsl").into()}else{match self.stride {
                44=>crate::stroke::shader(),72=>crate::chain::shader(),32=>crate::marker::shader(),
                48=>crate::finite_project_source::expand(include_str!("guide.wgsl")),_=>unreachable!(),
            }};
            pipeline(device,*format,&source,self.stride,attributes(self.stride),depth.clone(),*physical,samples)
        });
    }
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if self.data.is_empty(){return;}if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline.active); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }
```

**Locate this block:**

```rust
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;} Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
```

Replace it with:

```rust
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;}self.set_samples(device,self.samples); Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
```

**Locate this block:**

```rust
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
```

Replace it with:

```rust
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    recipe:(wgpu::TextureFormat,wgpu::DepthStencilState,bool),samples:u32,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
```

**Locate this block:**

```rust
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
 pipeline
}
```

Replace it with:

```rust
            }),
            primitive: wgpu::PrimitiveState{topology:if physical{wgpu::PrimitiveTopology::LineList}else{wgpu::PrimitiveTopology::TriangleList},..Default::default()},
            depth_stencil: Some(depth),
            multisample: wgpu::MultisampleState {count:samples,..Default::default()}, multiview_mask: None, cache: None,
        });
 pipeline
}
```

### 6. `src/highlight.rs`

Delegate the same count through existing highlight, preview, measurement and guide lanes.

**Locate this existing block:**

```rust
    pub(crate) fn geometry_for_test(&self)->Option<&Rc<crate::gpu_geometry::GpuGeometry>> {self.mesh.as_ref().map(|m|&m.geometry)}
    pub fn rebase_face(&mut self,queue:&wgpu::Queue,anchor:[f64;3],target:Option<Target>)->bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) fn geometry_for_test(&self)->Option<&Rc<crate::gpu_geometry::GpuGeometry>> {self.mesh.as_ref().map(|m|&m.geometry)}
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.set_samples(gpu,samples);}}
    pub fn rebase_face(&mut self,queue:&wgpu::Queue,anchor:[f64;3],target:Option<Target>)->bool {
```

### 7. `src/tool_preview.rs`

Delegate the same count through existing highlight, preview, measurement and guide lanes.

**Locate this existing block:**

```rust
impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Preview {
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){self.strokes.set_samples(gpu,samples);self.markers.set_samples(gpu,samples);}
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 8. `src/measure_gpu.rs`

Delegate the same count through existing highlight, preview, measurement and guide lanes.

**Locate this existing block:**

```rust
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Measurement {
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){self.strokes.set_samples(gpu,samples);}
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 9. `src/guide_gpu.rs`

Delegate the same count through existing highlight, preview, measurement and guide lanes.

**Locate this existing block:**

```rust
impl Guide{
```

After that block, type:

```rust
    pub fn set_samples(&mut self,gpu:&wgpu::Device,samples:u32){self.segments.set_samples(gpu,samples);self.marks.set_samples(gpu,samples);}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/stroke_gpu.rs`

Register the actual Native overlay/cache/empty-lane witness.

**Locate this existing block:**

```text
e,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzza-retarget-existing-overlay-lanes-20.rs" skip=120
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/stroke_sample_tests.rs`

Draw six actual overlay families at both counts, preserve vertices/IDs/uploads and query pipelines, delay empty variants, and validate late finite data plus both empty-depth combinations without mismatched bindings.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzza-retarget-existing-overlay-lanes-21.rs"
```

## <a id="checkpoint-96zzzzzzzb-read-section-crossings-at-each-sample"></a>8. Read section crossings at each sample

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cap_pipeline.rs, src/ca…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original closed solid</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching cap colour and p…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original cap owner query</text>
</svg>
</div>

**What this code does:** Match section crossing images and cap colour/depth to each physical sample while retaining original solid geometry, cached recipes and independent single-sample original-owner queries.

### 1. `src/cap_pipeline.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

Replace the entire file with:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool,samples:u32)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count,samples)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool,samples:u32)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub record_count:wgpu::RenderPipeline,pub geometry:wgpu::BindGroupLayout,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));
  let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},samples==4)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED};let count=pipeline(device,count_source(include_str!("cap_count.wgsl"),samples),&[&view,&model,&clip],"vs_count","fs_count",&[target.clone()],true,samples);
  let geometry=crate::cap_geometry::layout(device);let source=crate::object_rows::source(include_str!("cap_count.wgsl"));let record_count=pipeline_buffers(device,count_source(&source,samples),&[&view,&crate::object_rows::layout(device),&clip,&geometry],"vs_record_count","fs_count",&[target],&[crate::draw_slots::layout()],true,samples);
  let color=pipeline(device,crate::clipping_gpu::source(&colour_source(samples),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false,samples);Self{count,record_count,geometry,color,counts,frame}
 }
}

fn count_source(source:&str,samples:u32)->String {
 let source=crate::clipping_gpu::source(source,2,0);
 if samples==1 {source}else{source.replace("@location(0) world_pos:","@location(0) @interpolate(perspective,sample) world_pos:")}
}
fn colour_source(samples:u32)->String {
 let source=include_str!("cap_color.wgsl");if samples==1 {return source.into();}
 source.replace("var counts:texture_2d<f32>;","var counts:texture_multisampled_2d<f32>;")
 .replace("const SAMPLES:u32=1u;","const SAMPLES:u32=4u;")
 .replace("textureLoad(counts,at,0)","textureLoad(counts,at,i32(sample))")
 .replace("@location(0) @interpolate(flat) plane: u32,","@location(0) @interpolate(flat) plane: u32,\n    @location(1) @interpolate(linear,sample) pixel:vec2f,")
 .replace("out.plane = plane;","out.plane = plane;out.pixel=(out.pos.xy/out.pos.w*vec2f(0.5,-0.5)+0.5)*cap_frame.size;")
 .replace("in.pos.xy","in.pixel")
 .replace("let samples = clip_samples(t, gradient);","let samples = select(0u,0xffffffffu,t>=0.);")

}
```

### 2. `src/cap_color.wgsl`

Use the actual physical sample in the same cap fragment calculation at both counts. Read one matching crossing and selection word directly, preserving the single-sample calculation without duplicated shader source.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    }

    let at = vec2<i32>(in.pos.xy);
    var inside = 0u;
    var selected = false;

    // inside a solid where more crossings leave than enter behind the plane
    for (var k = 0u; k < SAMPLES; k++) {
        let count = count_at(at, k);
        inside |= select(0u, 1u << k, count > 0.5);
        selected = selected || count > SELECTED_CROSSING - 0.5;
    }

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
```

Replace it with:

```wgsl
    }

    let at = vec2<i32>(in.pos.xy);
    let count=count_at(at,sample);
    let inside=select(0u,1u<<sample,count>0.5);
    let selected=count>SELECTED_CROSSING-0.5;

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
```

**Locate this block:**

```wgsl

@fragment
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in: CapVertex) -> CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
```

Replace it with:

```wgsl

@fragment
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in:CapVertex,@builtin(sample_index) sample:u32)->CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
```

### 3. `src/cap_pick_pipeline.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

Replace the entire file with:

```rust
pub struct Pipelines{pub record_count:wgpu::RenderPipeline,pub record_nearest:wgpu::RenderPipeline,pub record_geometry:wgpu::BindGroupLayout,pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let record_geometry=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("shared cap pick crossings"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform),crate::tile_pipelines::entry(3,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(4,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(5,S::VERTEX,B::Storage{read_only:true})]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true,1);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target.clone()],true,1);
  let shared=crate::object_rows::source(include_str!("cap_count.wgsl"));let source=crate::clipping_gpu::source(&shared,2,0);let layouts=[&view,&crate::object_rows::layout(device),&clip,&record_geometry];let record_count=crate::cap_pipeline::pipeline_buffers(device,source.clone(),&layouts,"vs_record_pick_count","fs_pick_count",&[target.clone()],&[crate::draw_slots::layout()],true,1);let record_nearest=crate::cap_pipeline::pipeline_buffers(device,source,&layouts,"vs_record_pick_count","fs_pick_owner",&[target],&[crate::draw_slots::layout()],true,1);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false,1);
  Self{record_count,record_nearest,record_geometry,count,nearest,ids,records,owners,frame}
 }
}
```

### 4. `src/caps_gpu.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

Replace the entire file with:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::sample_pipelines::Variants<crate::cap_pipeline::Pipelines>>,samples:u32,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,samples:1,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_samples(&mut self,samples:u32){assert!(matches!(samples,1|4));self.samples=samples;}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::with_samples(device,format,self.samples);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(crate::sample_pipelines::Variants::new(pipes,self.samples));}
  self.pipes.as_mut().unwrap().select(self.samples,||crate::cap_pipeline::Pipelines::with_samples(device,format,self.samples));
  let pipes=&self.pipes.as_ref().unwrap().active;if self.counts.as_ref().is_none_or(|c|c.image.size!=size||c.image.texture.sample_count()!=self.samples){let geometry=self.counts.take().map_or_else(Default::default,|c|c.geometry);let image=Image::with_samples(device,size,wgpu::TextureFormat::R16Float,2,self.samples);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};let pipes=&pipes.active;
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

### 5. `src/cap_image.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

Replace the entire file with:

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub size:[u32;2],pub bytes:u64}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64)->Self{Self::with_samples(device,size,format,bytes,1)}
 pub fn with_samples(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64,samples:u32)->Self{assert!(matches!(samples,1|4));let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("original section data"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:samples,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());Self{texture,view,size,bytes:bytes*u64::from(size[0])*u64::from(size[1])*u64::from(samples)}}
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Retain the complete existing ordinary/shared surface fixture and add actual solid, hatch, selected cap, original-owner query, geometry/source preservation and exact return checks at both counts.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-50.rs"
```

## <a id="checkpoint-96zzzzzzzc-compose-cloud-depth-with-physical-addresses"></a>9. Compose cloud colour, depth and queries

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_gpu.rs, src/cloud…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original cloud points</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">actual ink occlusion</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fresh original-owner query</text>
</svg>
</div>

**What this code does:** Make nearer original clouds agree across visible colour, physical depth/addresses and independent original-owner queries, including ink in front and behind.

### 1. `src/cloud_gpu.rs`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window], "vertex", "color", &[format]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint],
  );
  [point, resolve, pick]
}
```

Replace it with:

```rust
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
    &[&layouts.image, &layouts.window],
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
  );
  [point, resolve, pick]
}
```

**Locate this block:**

```rust
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.draw(0..3, 0..1);
  }
  pub fn pick(
```

Replace it with:

```rust
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
    pass.draw(0..3, 0..1);
  }
  pub fn pick(
```

**Locate this block:**

```rust
    Ok(())
  }
  fn render_points(&self, ctx: &GpuCtx, size: [u32; 2]) {
    let settings = [1., size[0] as f32, size[1] as f32, 0., 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
    ctx.queue.write_buffer(&self.settings, 0, &settings.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
    let t = self.targets.as_ref().unwrap();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
```

Replace it with:

```rust
    Ok(())
  }
  fn render_points(&self, ctx: &GpuCtx, size: [u32; 2]) {
    let settings = [1., size[0] as f32, size[1] as f32, self.edl, 0., 0., 0., 0., size[0] as f32, size[1] as f32, 0., 0.];
    ctx.queue.write_buffer(&self.settings, 0, &settings.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());
    let t = self.targets.as_ref().unwrap();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
```

**Locate this block:**

```rust
      total: 0,
    }
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
  pub fn set_scene(
    &mut self,
```

Replace it with:

```rust
      total: 0,
    }
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
  }
  pub fn set_clipping(&mut self,revision:u64){self.clip_revision=revision;}
  pub fn set_scene(
    &mut self,
```

**Locate this block:**

```rust
      resolve,
      pick,
      settings,
      settings_group,
      columns_layout: layouts.columns,
      image_layout: layouts.image,
      window,
```

Replace it with:

```rust
      resolve,
      pick,
      settings,
      settings_group,edl:0.25,
      columns_layout: layouts.columns,
      image_layout: layouts.image,
      window,
```

**Locate this block:**

```rust
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  settings: wgpu::Buffer,
  settings_group: wgpu::BindGroup,
  columns_layout: wgpu::BindGroupLayout,
```

Replace it with:

```rust
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
  settings_group: wgpu::BindGroup,
  columns_layout: wgpu::BindGroupLayout,
```

### 2. `src/cloud_resolve.wgsl`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

Replace the entire file with:

```wgsl
@group(0) @binding(0) var sdepth:texture_depth_2d;
@group(0) @binding(1) var scolor:texture_2d<f32>;
@group(0) @binding(2) var sids:texture_2d<u32>;
struct Window {origin:vec2<u32>,mode:u32,padding:u32}
@group(1) @binding(0) var<uniform> window:Window;
@vertex fn vertex(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32> {
    let xy=vec2<f32>(f32((i<<1u)&2u),f32(i&2u));return vec4<f32>(xy*2.0-1.0,0.0,1.0);
}
struct Pixel {@location(0) color:vec4<f32>,@location(1) primitive:vec2u,@builtin(frag_depth) depth:f32}
fn log_depth(d:f32)->f32{return -log2(max(d,1.0e-7));}
@group(2) @binding(0) var<uniform> cloud:vec4f;
@fragment fn color(@builtin(position) p:vec4<f32>)->Pixel {
    let q=vec2<i32>(p.xy);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    var rgb=textureLoad(scolor,q,0).rgb;
    if(cloud.w>0.){
        let extent=vec2i(textureDimensions(sdepth));
        let me=log_depth(d);
        var sum=0.;
        let taps=array(vec2i(-1,0),vec2i(1,0),vec2i(0,-1),vec2i(0,1));
        for(var k=0;k<4;k++){
            let tap=q+taps[k];
            if(any(tap<vec2i(0))||any(tap>=extent)){continue;}
            let nd=textureLoad(sdepth,tap,0);
            if(nd==0.){continue;}
            sum+=max(0.,me-log_depth(nd));
        }
        rgb*=max(exp(-sum*75.*cloud.w),0.25);
    }

    return Pixel(vec4f(rgb,1.),vec2u(0u),d);
}
struct Pick {@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment fn pick(@builtin(position) p:vec4<f32>)->Pick {
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
    var hit=vec2<u32>(0u);
    if (window.mode==0u) {hit=vec2<u32>(ids.x,0u);}
    if (window.mode==3u && ids.x!=0u && ids.y!=0u) {hit=vec2<u32>(ids.x,(0x40000000u|(ids.y-1u))+1u);}
    return Pick(hit,vec2<u32>(0u),d);
}
```

### 3. `src/renderer.rs`

Skip absent cloud lanes before opening their composite pass. Cloud colour/depth replaces occluded physical surface words with zero in the same pass. Draw the lane already borrowed by the pass guard.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});if let Some(clouds)=self.clouds.borrow().as_ref(){clouds.draw(&mut pass);}}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

Replace it with:

```rust
            self.highlight.mesh(&mut pass);
        }
        self.caps.borrow_mut().draw(&self.device,&mut encoder,view,&self.targets.primitive.view,&self.targets.depth,&self.view_group,&clip_group,&self.meshes,if self.clipping.borrow().enabled{self.clipping.borrow().rows.len()as u32}else{0},Some(&self.instances));
        if let Some(clouds)=self.clouds.borrow().as_ref(){let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("cloud depth composite"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:&self.targets.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});clouds.draw(&mut pass);}
        if self.ambient_enabled.get(){let clip=self.clipping.borrow();if let Some(ambient)=self.ambient.borrow_mut().as_mut(){ambient.draw(&self.device,&self.queue,&mut encoder,view,&self.targets.depth,&self.view_group,&clip_group,&clip.buffer,&self.targets.primitive.view,&self.meshes,*transform,self.size,self.density,self.ambient_rows.ground_at(self.anchor()),[self.pyramid_revision,clip_revision],Some(&self.instances));}}
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("finite scene ink"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:None,stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
```

### 4. `src/sheet_gpu.rs`

Skip empty sheet lanes before binding their unused colour or query recipe.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
```

Replace it with:

```rust
        }
    }
    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>, crop: &wgpu::BindGroup,clipping:&wgpu::BindGroup) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(3,clipping,&[]);
        pass.set_bind_group(0, &self.group, &[]);
```

**Locate this block:**

```rust
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
```

Replace it with:

```rust
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
```

### 5. `src/finite_visibility.wgsl`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

**Locate this existing block:**

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
 if(ink_primitive(source_at,sample)==0u&&ink_depth(source_at,sample)>axis.depth+tolerance){return false;}
 if(clip_is_cap(ink_primitive(pixel,sample))){fit_at=source_at;}
```

### 6. `src/pick_encode.rs`

Reuse the existing readonly integer overlay query pass. Cloud depth is established before readonly original ink queries. Write actual cloud query depth and zero physical words before the finite query reads them. Reuse the existing query pass and attachments through a stack-only helper.

Replace the entire file with:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  // original owner attachment,
  // original primitive attachment,
 ];
 // original owner pass
}
```

### 7. `src/pick_encode.rs`

Reuse the exact previously typed owner attachment in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original owner attachment
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-28.rs"
```

### 8. `src/pick_encode.rs`

Reuse the exact previously typed primitive attachment in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original primitive attachment
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-29.rs"
```

### 9. `src/pick_encode.rs`

Reuse the exact previously typed owner pass in the shared query pass.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original owner pass
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-30.rs"
```

### 10. `src/pick_encode.rs`

Keep cloud colour, physical depth/addresses and original-owner queries coherent without changing original sources or adding GPU images.

**Locate this existing block:**

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
  Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}),
 ];
 encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("other original integer owners"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()})
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::{gpu_mesh::GpuMesh, pick_image::Image, pick_window::Window, stroke_gpu::Lane};

pub(crate) fn draw(device: &wgpu::Device, image: &Image, window: Window, pipeline: &wgpu::RenderPipeline,
    view_group: &wgpu::BindGroup, crop_group: &wgpu::BindGroup, meshes: &[GpuMesh], ink: &[&Lane], faces: &crate::face_pick::Faces, mode: crate::pick_sources::Mode,text:Option<&crate::scene_text_gpu::Lane>,clouds:Option<&crate::cloud_gpu::Lane>,sheets:Option<&crate::sheet_gpu::Lane>,ctx:&crate::text_gpu_support::GpuCtx,tables:Option<&crate::finite_ink::Tables>,clipping:&wgpu::BindGroup,caps:Option<&mut crate::caps_gpu::Caps>,crop:&wgpu::Buffer,clipping_buffer:&wgpu::Buffer,records:Option<(&crate::instance_gpu::Lane,&wgpu::RenderPipeline)>) -> wgpu::CommandBuffer {
        let mut encoder = device.create_command_encoder(&Default::default());
        let mesh_crop=crate::pick_pipeline::crop_clipping_group(device,crop,clipping_buffer);
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("IDs"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &image.colour, depth_slice: None, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store } }),Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &image.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(crate::depth::FAR as f32), store: wgpu::StoreOp::Store }), stencil_ops: None }), ..Default::default() });
            pass.set_pipeline(pipeline); pass.set_bind_group(0, view_group, &[]); pass.set_bind_group(2, &mesh_crop, &[]);
            for mesh in meshes {if mode==crate::pick_sources::Mode::Face&&!mesh.source_faces{continue;}if !records.is_some_and(|(rows,shared)|rows.draw_with(&mut pass,mesh,shared)){pass.set_pipeline(pipeline);mesh.draw_pulled(&mut pass);}}

        }
        if mode==crate::pick_sources::Mode::Face{
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original source-face IDs"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:None,stencil_ops:None}),..Default::default()});
            faces.draw(&mut pass,meshes,view_group,crop_group,records.map(|(rows,_)|rows));
        }
        if let Some(caps)=caps{caps.encode_pick(device,&ctx.queue,&mut encoder,image,window,view_group,crop,clipping,meshes,mode,records.map(|(rows,_)|rows));}
        if let Some(clouds)=clouds{let mut pass=overlay(&mut encoder,image,"original cloud query depth",true);clouds.pick(ctx,&mut pass,window,mode);}
        let tables=tables.map(|tables|crate::finite_ink::Tables{projected:tables.projected.clone(),tiles:tables.tiles.clone(),primitive:image.primitive.view.clone()});
        {
            let mut pass=overlay(&mut encoder,image,"finite integer ink",false);
            pass.set_bind_group(3,clipping,&[]);
            for lane in ink {lane.pick_finite(device,&mut pass,view_group,crop_group,tables.as_ref(),&image.depth);}
        }
        {
            let mut pass=overlay(&mut encoder,image,"other original integer owners",false);
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group,clipping);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
        // The surrounding halo supports rasterization; only the smaller cursor window is copied back for hit testing.
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo { texture: &image.texture, mip_level: 0,
            origin: wgpu::Origin3d { x: window.copy[0] - window.draw[0], y: window.copy[1] - window.draw[1], z: 0 }, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &image.readback, layout: wgpu::TexelCopyBufferLayout { offset: 0,
                bytes_per_row: Some(window.row_bytes()), rows_per_image: Some(window.copy[3]) } },
            wgpu::Extent3d { width: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  Some(wgpu::RenderPassColorAttachment{view:&image.colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
  Some(wgpu::RenderPassColorAttachment{view:&image.primitive.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),
 ];
 encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some(label),color_attachments:&colours[..if physical{2}else{1}],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&image.depth,depth_ops:physical.then_some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()})
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/cloud_address_tests.rs`

Supplied actual one-sample physical cloud face/cap addresses and dense rear/front-line colour/query/source/placement/cache witnesses at both physical densities. Four-sample resources are connected next.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-40.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 12. `src/cloud_composite_reference.rs`

Independently render the exact maintained cloud WGSL on the same original cached point inputs; Native-only validation data.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-41.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/cloud_composite_tests.rs`

Actual one-sample independent RGBA comparison for source alpha0/255, shading0/.25, viewport edges and whole/default output, preserving original sources, owner queries and cached raster.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-42.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/renderer.rs`

Register supplied Native physical-depth/owner and independent shader witnesses.

**Locate this existing block:**

```text
mod sample_tests;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-43.rs" skip=17
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/cloud_gpu.rs`

Register the optional Native exact maintained compositor reference.

**Locate this existing block:**

```text
_lod::Range, px: f32, scale: f64, ortho_h: f32) -> f32 {
  crate::cloud_lod::radius_factor(range, px, scale, ortho_h) * if ortho_h > 0. { 1. } else { 1000. }
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzc-compose-cloud-depth-with-physical-addresses-44.rs" skip=160
```

## <a id="checkpoint-96zzzzzzzca-cache-cloud-and-sheet-samples"></a>10. Cache cloud and sheet sample recipes

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 10 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/cloud_gpu.rs, src/rende…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original cloud/drawing so…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached count-specific col…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent original owne…</text>
</svg>
</div>

**What this code does:** Cache count-specific original cloud and sheet colour recipes while retaining original source geometry, point images/raster, drawing placements/IDs and independent one-sample owner queries.

### 1. `src/cloud_gpu.rs`

Cache matching cloud colour recipes while retaining source columns, point images/raster, original-owner queries and current appearance.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
  );
  [point, resolve, pick]
}

fn record_bytes(
```

Replace it with:

```rust
    "vertex",
    "pick",
    &[wgpu::TextureFormat::Rg32Uint,crate::primitive_image::FORMAT],
    1,
  );
  ([point, resolve, pick],resolver)
}

fn record_bytes(
```

**Locate this block:**

```rust
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT]);
  let pick = pipeline(
    &ctx.device,
    &resolver,
```

Replace it with:

```rust
    "vs_point",
    "fs_point",
    &[wgpu::TextureFormat::Rgba8Unorm, wgpu::TextureFormat::Rg32Uint],
    1,
  );
  let resolve = pipeline(&ctx.device, &resolver, &[&layouts.image, &layouts.window, &layouts.uniform], "vertex", "color", &[format,crate::primitive_image::FORMAT],1);
  let pick = pipeline(
    &ctx.device,
    &resolver,
```

**Locate this block:**

```rust
  });
  Layouts { uniform, window: window_layout, columns: columns_layout, image: image_layout }
}
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> [wgpu::RenderPipeline; 3] {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0).into()),
```

Replace it with:

```rust
  });
  Layouts { uniform, window: window_layout, columns: columns_layout, image: image_layout }
}
fn pipelines(ctx: &GpuCtx, format: wgpu::TextureFormat, layouts: &Layouts) -> ([wgpu::RenderPipeline; 3],wgpu::ShaderModule) {
  let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("current production cloud discs"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("cloud_points.wgsl"),2,0).into()),
```

**Locate this block:**

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
```

Replace it with:

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: wgpu::MultisampleState {count:samples,..Default::default()},
    multiview_mask: None,
    cache: None,
  })
```

**Locate this block:**

```rust
  vertex: &str,
  fragment: &str,
  formats: &[wgpu::TextureFormat],
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(fragment),
```

Replace it with:

```rust
  vertex: &str,
  fragment: &str,
  formats: &[wgpu::TextureFormat],
  samples:u32,
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(fragment),
```

**Locate this block:**

```rust
    let Some(t) = &self.targets else {
      return;
    };
    pass.set_pipeline(&self.resolve);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
```

Replace it with:

```rust
    let Some(t) = &self.targets else {
      return;
    };
    pass.set_pipeline(&self.resolve.active);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
```

**Locate this block:**

```rust
      );
      self.targets = Some(Targets { color, depth, ids, size, group });
    }
    self.render_points(ctx, size);
    self.key = Some(key);
    self.preparations += 1;
```

Replace it with:

```rust
      );
      self.targets = Some(Targets { color, depth, ids, size, group });
    }
    self.set_samples(ctx,self.samples);
    self.render_points(ctx, size);
    self.key = Some(key);
    self.preparations += 1;
```

**Locate this block:**

```rust
      total: 0,
    }
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
```

Replace it with:

```rust
      total: 0,
    }
  }
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    assert!(matches!(samples,1|4));self.samples=samples;if self.targets.is_none(){return;}
    self.resolve.select(samples,||pipeline(&ctx.device,&self.resolver,&[&self.image_layout,&self.pick.get_bind_group_layout(1),&self.point.get_bind_group_layout(0)],"vertex","color",&[self.format,crate::primitive_image::FORMAT],samples));
  }
  pub fn set_edl(&mut self,ctx:&GpuCtx,value:f32) {
    let value=if value.is_finite(){value.max(0.)}else{0.};if value==self.edl{return;}
    self.edl=value;ctx.queue.write_buffer(&self.settings,12,&value.to_ne_bytes());
```

**Locate this block:**

```rust
      resource_group(ctx, &layouts.uniform, "cloud frame settings", [(0, settings.as_entire_binding())]);
    let window = make_buffer("cloud pick origin", 16);
    let window_group = resource_group(ctx, &layouts.window, "cloud pick origin", [(0, window.as_entire_binding())]);
    let [point, resolve, pick] = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve,
      pick,
      settings,
      settings_group,edl:0.25,
```

Replace it with:

```rust
      resource_group(ctx, &layouts.uniform, "cloud frame settings", [(0, settings.as_entire_binding())]);
    let window = make_buffer("cloud pick origin", 16);
    let window_group = resource_group(ctx, &layouts.window, "cloud pick origin", [(0, window.as_entire_binding())]);
    let ([point, resolve, pick],resolver) = pipelines(ctx, format, &layouts);
    Self {clipping:clipping.clone(),clip_revision:0,
      point,
      resolve:Variants::new(resolve,1),resolver,format,samples:1,
      pick,
      settings,
      settings_group,edl:0.25,
```

**Locate this block:**

```rust
pub struct Lane {
  clipping:wgpu::BindGroup,clip_revision:u64,
  point: wgpu::RenderPipeline,
  resolve: wgpu::RenderPipeline,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
```

Replace it with:

```rust
pub struct Lane {
  clipping:wgpu::BindGroup,clip_revision:u64,
  point: wgpu::RenderPipeline,
  resolve:Variants<wgpu::RenderPipeline>,
  resolver:wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32,
  pick: wgpu::RenderPipeline,
  edl:f32,
  settings: wgpu::Buffer,
```

**Locate this block:**

```rust
use crate::{
  cloud::Row,
  cloud_lod::{LodWalk, Projection},
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::{
  cloud::Row,
  cloud_lod::{LodWalk, Projection},
```

### 2. `src/renderer.rs`

Late-created clouds adopt the current count before preparing source records.

**Locate this existing block:**

```rust
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));lane.set_scene(&self.text_context(),scene,selected)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut owner=self.clouds.borrow_mut();let lane=owner.get_or_insert_with(||crate::cloud_gpu::Lane::new(&self.text_context(),self.format,&self.clipping.borrow().group));
        lane.set_samples(&self.text_context(),self.targets.samples);
        lane.set_scene(&self.text_context(),scene,selected)
```

### 3. `src/sheet_gpu.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
```

Replace it with:

```rust
            .collect::<Vec<_>>();
        queue.write_buffer(&self.view, 0, &bytes);
    }
    pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32) {
        assert!(matches!(samples,1|4));self.samples=samples;if self.draws.is_empty(){return;}
        self.pipeline.select(samples,||crate::sheet_pipeline::colour(device,self.format,samples));
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty(){return;}

        pass.set_pipeline(&self.pipeline.active);
        pass.set_bind_group(0, &self.group, &[]);
        for draw in &self.draws {
            pass.set_bind_group(1, &draw.group, &[]);
```

**Locate this block:**

```rust
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

Replace it with:

```rust
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        self.set_samples(device,self.samples);
        Ok(())
    }
    pub fn camera(&self, camera: &crate::camera::Camera) {
```

**Locate this block:**

```rust
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

Replace it with:

```rust
                            });
                        let group = crate::gpu_layout::group(
                            device,
                            &self.pipeline.active.get_bind_group_layout(1),
                            &uniform,
                        );
                        Draw {
```

**Locate this block:**

```rust
        let group = crate::gpu_layout::group(device, &pipeline.get_bind_group_layout(0), &view);
        Self {
            projection: Default::default(),
            pipeline,
            pick,
            view,
            group,
```

Replace it with:

```rust
        let group = crate::gpu_layout::group(device, &pipeline.get_bind_group_layout(0), &view);
        Self {
            projection: Default::default(),
            pipeline:crate::sample_pipelines::Variants::new(pipeline,1),format,samples:1,
            pick,
            view,
            group,
```

**Locate this block:**

```rust
}
pub struct Lane {
    projection: std::cell::Cell<Option<f32>>,
    pipeline: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

Replace it with:

```rust
}
pub struct Lane {
    projection: std::cell::Cell<Option<f32>>,
    pipeline:crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    format:wgpu::TextureFormat,samples:u32,
    pick: wgpu::RenderPipeline,
    view: wgpu::Buffer,
    group: wgpu::BindGroup,
```

### 4. `src/sheet_pipeline.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes. Add only physical-sample source qualification around the previously typed colour recipe.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
    );
    (pipeline, pick)
}
```

Replace it with:

```rust
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
        1,
    );
    (pipeline, pick)
}

pub fn colour(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// original view/model
// original clipping layout
// original expanded source
 let expanded=if samples==4 {expanded.replace("@interpolate(linear)","@interpolate(linear,sample)").replace("@location(8) original_clip:","@location(8) @interpolate(perspective,sample) original_clip:")}else{expanded};
// original source reference
// original shader
// original colour layout
// original render
 pipeline
}
```

**Locate this block:**

```rust
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
    let pick = make(
        device,
```

Replace it with:

```rust
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
        1,
    );
    let pick = make(
        device,
```

**Locate this block:**

```rust
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:Default::default(),multiview_mask:None,cache:None})
}

pub fn pipelines(
```

Replace it with:

```rust
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
    samples:u32,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}

pub fn pipelines(
```

### 5. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet view/model in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original view/model
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-37.rs"
```

### 6. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet clipping layout in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original clipping layout
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-38.rs"
```

### 7. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet expanded source in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original expanded source
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-39.rs"
```

### 8. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet source reference in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original source reference
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-40.rs"
```

### 9. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet shader in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original shader
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-41.rs"
```

### 10. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet colour layout in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original colour layout
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-42.rs"
```

### 11. `src/sheet_pipeline.rs`

Reuse exact previously typed sheet render in the alternate colour recipe.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// original render
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-43.rs"
```

### 12. `src/sheet_pipeline.rs`

Match the alternate sheet colour descriptor to the actual pass.

**Locate this existing block:**

```rust
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
 pipeline
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Some(wgpu::BlendState::ALPHA_BLENDING),
        samples,
    );
 pipeline
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 13. `src/cloud_address_tests.rs`

Extend every existing physical address/rear-front source/cropped-query assertion to both counts and require real exact sample returns with full cached cloud usage retained.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-45.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 14. `src/cloud_composite_tests.rs`

Retain existing alpha/shading/viewport-edge/default/full-frame/original-owner parity assertions and extend them to all16settings at both physical counts.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-46.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 15. `src/sample_surface_tests.rs`

Retain every prior surface/section assertion and add actual sheet/cloud composition at both counts, complete sources/geometry/cache retention, exact returns and fresh original owners.

**Locate this existing block:**

```text
r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value(),"Single-sample closed-cap owner query is independent of4x crossing resources");
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzca-cache-cloud-and-sheet-samples-47.rs" skip=160
```

## <a id="checkpoint-96zzzzzzzd-retarget-cached-text"></a>11. Retarget cached text colour

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 11 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_plate.rs, src/text…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original text/shaping</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">cached matching colour re…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">independent original-owne…</text>
</svg>
</div>

**What this code does:** Match world text, nameplates and screen glyph colour to physical sample counts while retaining original sources, shaping, fonts, atlas contents, world-plane raster images and independent original-owner queries.

### 1. `src/text_plate.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(range, 0..1);
        1
```

Replace it with:

```rust
            return 0;
        }

        pass.set_pipeline(&self.pipeline.active);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(range, 0..1);
        1
```

**Locate this block:**

```rust
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
```

Replace it with:

```rust
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline:Variants::new(pipeline(ctx,target,false),target.samples),format:target.format,
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if target.format!=self.format {self.pipeline=Variants::new(pipeline(ctx,target,false),target.samples);self.format=target.format;}
        else {self.pipeline.select(target.samples,||pipeline(ctx,target,false));}
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
```

**Locate this block:**

```rust
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_plate.wgsl").into()),
        });
        let id_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text plate ids"),
```

Replace it with:

```rust
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(if target.samples==4 {include_str!("text_plate.wgsl").replace("@location(0) local:","@location(0) @interpolate(linear,sample) local:").into()}else{include_str!("text_plate.wgsl").into()}),
        });
        let id_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text plate ids"),
```

**Locate this block:**

```rust
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
```

Replace it with:

```rust
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline:Variants<Pipeline>,
    format:wgpu::TextureFormat,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
```

**Locate this block:**

```rust
use crate::text_gpu_support::{GpuCtx,GrowBuf,VERTS};
use crate::text_gpu_support::{Pipeline,Target};
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::text_gpu_support::{GpuCtx,GrowBuf,VERTS};
use crate::text_gpu_support::{Pipeline,Target};
```

### 2. `src/text_plane.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            return 0;
        }

        self.draw_run(pass, &self.pipeline)
    }

    pub(super) fn draw_ids(
```

Replace it with:

```rust
            return 0;
        }

        self.draw_run(pass, &self.pipeline.active)
    }

    pub(super) fn draw_ids(
```

**Locate this block:**

```rust
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.pipeline = pipeline(ctx, target, &self.layout, false);
    }

    pub(super) fn reset(&mut self) {
```

Replace it with:

```rust
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if target.format!=self.target.format {self.pipeline=crate::sample_pipelines::Variants::new(pipeline(ctx,target,&self.layout,false),target.samples);}
        else {self.pipeline.select(target.samples,||pipeline(ctx,target,&self.layout,false));}
        self.target=target;
    }

    pub(super) fn reset(&mut self) {
```

**Locate this block:**

```rust
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,
            target,
            rasterizations: 0,
```

Replace it with:

```rust
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline:crate::sample_pipelines::Variants::new(pipeline,target.samples),
            id_pipeline,
            target,
            rasterizations: 0,
```

**Locate this block:**

```rust
    vertices: GrowBuf,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
```

Replace it with:

```rust
    vertices: GrowBuf,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pipeline:crate::sample_pipelines::Variants<Pipeline>,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
```

### 3. `src/scene_text_gpu.rs`

Retain original drawing, raster/font resources and independent owner queries while caching matching colour recipes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(ctx, &mut self.atlas, target, wgpu::CompareFunction::Always);
        self.anchored = renderer(
            ctx,
```

Replace it with:

```rust
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        if self.target.format==target.format&&self.target.samples==target.samples {return;}
        let format_changed=self.target.format!=target.format;
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        if format_changed {self.rebuild_resources(ctx);return;}
        self.overlay = renderer(ctx, &mut self.atlas, target, wgpu::CompareFunction::Always);
        self.anchored = renderer(
            ctx,
```

### 4. `src/renderer.rs`

Late-created text adopts the current renderer count.

**Locate this existing block:**

```rust
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:1});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut lane=self.text.borrow_mut();if lane.is_none() {let mut new=crate::scene_text_gpu::Lane::new(&self.text_context(),crate::text_gpu_support::Target {format:self.format,samples:self.targets.samples});if let Some(fonts)=self.full_text_fonts.borrow().as_ref() {new.document.replace_fonts(fonts.clone())?;}*lane=Some(new);}lane.as_mut().unwrap().set_labels(labels)
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/sample_surface_tests.rs`

Extend the supplied actual fixture to cached text colour without introducing the complete renderer API before all remaining consumers.

**Locate this existing block:**

```text
 r.sheets.set_samples(&r.device,count);
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzd-retarget-cached-text-31.rs" skip=40
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Actual world/nameplate/screen text source/font/shaping/raster/atlas retention, late creation, exact count returns and fresh original-owner queries.

**Locate this existing block:**

```text
roto(),cloud_before);assert!(Rc::ptr_eq(&scene.sheets()[0].prepared.original,&sheet));crate::sample_test_proof::write(root.join("four.rgba"),four).unwrap();
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzd-retarget-cached-text-32.rs" skip=160
```

## <a id="checkpoint-96zzzzzzze-cache-outline-section-recipes"></a>12. Cache section masks and the final outline blend

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 12 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/outline_caps.rs, src/ou…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original section address</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained outline search</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one cached final blend</text>
</svg>
</div>

**What this code does:** Prepare count-specific section coverage and outline colour recipes while retaining original addresses, one-sample search images and one final black blend.

### 1. `src/outline_caps.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

Replace the entire file with:

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
// existing section coverage descriptor
(pipeline,layout)
}
```

### 2. `src/outline_caps.rs`

Reuse the previously typed immutable outline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// existing section coverage descriptor
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-03.rs"
```

### 3. `src/outline_caps.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

**Locate this existing block:**

```rust
pub struct Caps{
 pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{
Self{pipeline,layout,input:None,group:None}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer){
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.layout,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,false),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_caps.wgsl"),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),multisample:Default::default(),multiview_mask:None,cache:None,
  });
(pipeline,layout)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
pub struct Caps{
 pipelines:Option<Variants<(wgpu::RenderPipeline,wgpu::BindGroupLayout)>>,
 input:Option<wgpu::TextureView>,group:Option<wgpu::BindGroup>,
}
impl Caps{
 pub fn new(_:&wgpu::Device)->Self {Self {pipelines:None,input:None,group:None}}
 pub fn clear(&mut self){self.input=None;self.group=None;}
 pub fn prepare(&mut self,device:&wgpu::Device,input:&wgpu::TextureView,radius:&wgpu::Buffer,samples:u32){
  if self.pipelines.is_none(){self.pipelines=Some(Variants::new(pipelines(device,samples),samples));}
  if self.pipelines.as_mut().unwrap().select(samples,||pipelines(device,samples)){self.clear();}
  if self.input.as_ref()==Some(input){return;}
  self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("visible cap coverage"),layout:&self.pipelines.as_ref().unwrap().active.1,entries:&[
   wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(input)},wgpu::BindGroupEntry{binding:1,resource:radius.as_entire_binding()},
  ]}));self.input=Some(input.clone());
 }
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,clipping:&wgpu::BindGroup){if let Some(group)=&self.group{pass.set_pipeline(&self.pipelines.as_ref().unwrap().active.0);pass.set_bind_group(0,group,&[]);pass.set_bind_group(1,clipping,&[]);pass.draw(0..3,0..1);}}
}

fn pipelines(device:&wgpu::Device,samples:u32)->(wgpu::RenderPipeline,wgpu::BindGroupLayout) {
  use wgpu::{ShaderStages as Stages,TextureSampleType as Sample};
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("visible cap coverage"),entries:&[
   crate::text_gpu_support::texture_entry(0,Stages::FRAGMENT,Sample::Uint,samples==4),
   wgpu::BindGroupLayoutEntry{binding:1,visibility:Stages::FRAGMENT,ty:wgpu::BindingType::Buffer{ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:wgpu::BufferSize::new(16)},count:None},
  ]});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("visible cap coverage"),source:wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&source(samples),1,0).into())});
  let clipping=crate::clipping_gpu::layout(device);
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("visible cap coverage"),bind_group_layouts:&[Some(&layout),Some(&clipping)],immediate_size:0});
  let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
   label:Some("visible cap coverage"),layout:Some(&pipeline_layout),
   vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},
   fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::TextureFormat::R8Unorm.into())],compilation_options:Default::default()}),
   primitive:Default::default(),depth_stencil:Some(wgpu::DepthStencilState{format:crate::depth::FORMAT,depth_write_enabled:Some(false),depth_compare:Some(crate::depth::CLOSER_EQUAL),stencil:Default::default(),bias:Default::default()}),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None,
  });(pipeline,layout)
}

fn source(samples:u32)->String {
 let source=include_str!("outline_caps.wgsl");if samples==1{return source.into();}
 source.replace("var primitives:texture_2d<u32>;","var primitives:texture_multisampled_2d<u32>;")
 .replace(r#"@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
"#,r#"struct Vertex{@builtin(position) position:vec4f,@location(0) @interpolate(linear,sample) pixel:vec2f}
@vertex fn vs_main(@builtin(vertex_index) i:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[i];return Vertex(vec4f(p,0,1),p*vec2f(0.5,-0.5)+0.5);
}
"#)
 .replace("fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{","fn fs_main(input:Vertex,@builtin(sample_index) sample:u32)->Coverage{let p=vec4f(input.pixel*vec2f(textureDimensions(primitives)),0,1);")
 .replace("textureLoad(primitives,vec2<i32>(p.xy),0)","textureLoad(primitives,vec2<i32>(p.xy),i32(sample))")
}
```

### 4. `src/outline_composite.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

Replace the entire file with:

```rust
pub struct Composite{pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,layout,inputs:None,group:None}
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
// existing outline descriptor
}
```

### 5. `src/outline_composite.rs`

Reuse the previously typed immutable outline descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

**Previously typed code reference:**

Use this reference when moving the existing code; it is excluded from the typing estimate.

**Locate this existing block:**

```text
// existing outline descriptor
```

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzze-cache-outline-section-recipes-30.rs"
```

### 6. `src/outline_composite.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

**Locate this existing block:**

```rust
pub struct Composite{pipeline:wgpu::RenderPipeline,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,layout,inputs:None,group:None}
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("one black outline blend"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::sample_pipelines::Variants;
pub struct Composite{pipeline:Variants<wgpu::RenderPipeline>,shader:wgpu::ShaderModule,format:wgpu::TextureFormat,layout:wgpu::BindGroupLayout,inputs:Option<[wgpu::TextureView;2]>,group:Option<wgpu::BindGroup>}
impl Composite{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{
  let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("max outline alpha"),entries:&[0,1].map(|n|crate::text_gpu_support::texture_entry(n,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false))});
  let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("max outline alpha"),source:wgpu::ShaderSource::Wgsl(include_str!("outline_composite.wgsl").into())});
  let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
  let pipeline=Variants::new(pipeline(device,&pipeline_layout,&shader,format,samples),samples);Self{pipeline,shader,format,layout,inputs:None,group:None}
 }
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipeline.select(samples,||{let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("max outline alpha"),bind_group_layouts:&[Some(&self.layout)],immediate_size:0});pipeline(device,&layout,&self.shader,self.format,samples)});}
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,colour:&wgpu::TextureView,global:&wgpu::TextureView,selected:Option<&wgpu::TextureView>){
  let inputs=[global.clone(),selected.unwrap_or(global).clone()];if self.inputs.as_ref()!=Some(&inputs){self.group=Some(device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("max outline alpha"),layout:&self.layout,entries:&[0,1].map(|n|wgpu::BindGroupEntry{binding:n,resource:wgpu::BindingResource::TextureView(&inputs[n as usize])})}));self.inputs=Some(inputs);}
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("one black outline blend"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipeline.active);pass.set_bind_group(0,self.group.as_ref().unwrap(),&[]);pass.draw(0..3,0..1);
 }
}

fn pipeline(device:&wgpu::Device,pipeline_layout:&wgpu::PipelineLayout,shader:&wgpu::ShaderModule,format:wgpu::TextureFormat,samples:u32)->wgpu::RenderPipeline {
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("one black outline blend"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some("vs_main"),buffers:&[],compilation_options:Default::default()},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some("fs_main"),targets:&[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],compilation_options:Default::default()}),primitive:Default::default(),depth_stencil:None,multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
```

### 7. `src/outline_gpu.rs`

Keep this prerequisite independently compilable: its current coverage producer is still one-sample. The next lesson passes the active physical count.

**Locate this existing block:**

```rust
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius,1);}
```

## <a id="checkpoint-96zzzzzzzf-resolve-float-outline-coverage"></a>13. Resolve float outline coverage

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 13 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/outline_gpu.rs, src/out…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original physical face/se…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained search images</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one matching final blend</text>
</svg>
</div>

**What this code does:** Feed physical face and section coverage into the retained one-sample outline search and blend the selected/global result once, without resolving integer addresses or depth.

### 1. `src/outline_gpu.rs`

Match physical face/section coverage and colour recipes while retaining one-sample outline search images and a single final blend.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
```

Replace it with:

```rust
      stencil: Default::default(),
      bias: Default::default(),
    }),
    multisample:wgpu::MultisampleState{count:samples,..Default::default()},
    multiview_mask: None,
    cache: None,
  })
```

**Locate this block:**

```rust
    }),
    primitive: Default::default(),
    depth_stencil: None,
    multisample: Default::default(),
    multiview_mask: None,
    cache: None,
  })
}
fn mask_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
  let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("selected visible coverage"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(include_str!("outline_mask.wgsl"),2,0).into()),
  });
  let view=crate::surface_view::layout(device);
  let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
```

Replace it with:

```rust
    }),
    primitive: Default::default(),
    depth_stencil: None,
    multisample:wgpu::MultisampleState{count:samples,..Default::default()},
    multiview_mask: None,
    cache: None,
  })
}
fn mask_pipeline(device: &wgpu::Device,samples:u32) -> wgpu::RenderPipeline {
  let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("selected visible coverage"),
    source: wgpu::ShaderSource::Wgsl(crate::clipping_gpu::source(&if samples==4{include_str!("outline_mask.wgsl").replace("@location(0) world:","@location(0) @interpolate(perspective,sample) world:")}else{include_str!("outline_mask.wgsl").into()},2,0).into()),
  });
  let view=crate::surface_view::layout(device);
  let model = crate::gpu_layout::uniform(device, crate::object_record::BYTES as u64);
```

**Locate this block:**

```rust
  entry: &str,
  format: wgpu::TextureFormat,
  blend: bool,
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(entry),
```

Replace it with:

```rust
  entry: &str,
  format: wgpu::TextureFormat,
  blend: bool,
  samples:u32,
) -> wgpu::RenderPipeline {
  let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    label: Some(entry),
```

**Locate this block:**

```rust
      color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],
      ..Default::default()
    });
    pass.set_pipeline(&self.composite_pipeline);pass.set_bind_group(0,group,&[]);pass.draw(0..3,0..1);
  }
}
fn raster(
```

Replace it with:

```rust
      color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:colour,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],
      ..Default::default()
    });
    pass.set_pipeline(&self.pipelines.active.1);pass.set_bind_group(0,group,&[]);pass.draw(0..3,0..1);
  }
}
fn raster(
```

**Locate this block:**

```rust
          }),
          ..Default::default()
        });
        pass.set_pipeline(&self.mask_pipeline);
        pass.set_bind_group(0, view_group, &[]);pass.set_bind_group(2,clipping,&[]);
        for mesh in meshes.iter().filter(|m| all||m.selected()) {
          mesh.draw(&mut pass);
```

Replace it with:

```rust
          }),
          ..Default::default()
        });
        pass.set_pipeline(&self.pipelines.active.0);
        pass.set_bind_group(0, view_group, &[]);pass.set_bind_group(2,clipping,&[]);
        for mesh in meshes.iter().filter(|m| all||m.selected()) {
          mesh.draw(&mut pass);
```

**Locate this block:**

```rust
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("selected visible coverage"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &m.coverage.view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
          })],
          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
```

Replace it with:

```rust
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
          label: Some("selected visible coverage"),
          color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view:m.coverage4.as_ref().map_or(&m.coverage.view,|image|&image.view),
            depth_slice: None,
            resolve_target:m.coverage4.as_ref().map(|_|&m.coverage.view),
            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
          })],
          depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
```

**Locate this block:**

```rust
      return;
    }
    self.prepare(ctx, size, density,all);
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius,1);}
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision,clipping:clip_revision,all };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
```

Replace it with:

```rust
      return;
    }
    self.prepare(ctx, size, density,all);
    if let Some(sections)=sections{self.caps.prepare(&ctx.device,sections,&self.radius,self.pipelines.samples);}else{self.caps.clear();}
    let key = Key { transform, size, density: density.to_bits(), revision: self.revision,clipping:clip_revision,all };
    let m = self.masks.as_ref().unwrap();
    if self.key.as_ref() != Some(&key) {
```

**Locate this block:**

```rust
        "selected outline alpha",
        [(0, wgpu::BindingResource::TextureView(&alpha.view))],
      );
      self.masks = Some(Masks { coverage, coarse, dilated, alpha, block, pool, dilate, search, composite });
      self.key = None;
    }
    ctx.queue.write_buffer(
      &self.radius,
      0,
```

Replace it with:

```rust
        "selected outline alpha",
        [(0, wgpu::BindingResource::TextureView(&alpha.view))],
      );
      self.masks = Some(Masks { coverage,coverage4:None, coarse, dilated, alpha, block, pool, dilate, search, composite });
      self.key = None;
    }
    let masks=self.masks.as_mut().unwrap();
    if masks.coverage4.is_some()!=(self.pipelines.samples==4) {
      masks.coverage4=(self.pipelines.samples==4).then(||Image::with_samples(&ctx.device,size,"multisampled original coverage",4));self.key=None;
    }
    ctx.queue.write_buffer(
      &self.radius,
      0,
```

**Locate this block:**

```rust
  pub fn usage(&self) -> [u64; 4] {
    self.masks.as_ref().map_or([self.radius.size() + self.taps.size(), self.rebuilds, self.skips, self.draws], |m| {
      [
        m.coverage.bytes()
          + m.coarse.bytes()
          + m.dilated.bytes()
          + m.alpha.bytes()
```

Replace it with:

```rust
  pub fn usage(&self) -> [u64; 4] {
    self.masks.as_ref().map_or([self.radius.size() + self.taps.size(), self.rebuilds, self.skips, self.draws], |m| {
      [
        m.coverage.bytes()+m.coverage4.as_ref().map_or(0,Image::bytes)
          + m.coarse.bytes()
          + m.dilated.bytes()
          + m.alpha.bytes()
```

**Locate this block:**

```rust
      draws: 0,
    }
  }
  pub fn invalidate(&mut self) {
    self.revision = self.revision.wrapping_add(1);
  }
```

Replace it with:

```rust
      draws: 0,
    }
  }
  pub fn set_samples(&mut self,ctx:&GpuCtx,samples:u32) {
    if self.pipelines.select(samples,||(mask_pipeline(&ctx.device,samples),fullscreen(&ctx.device,&self.shader,&[&self.composite_layout],"fs_main",self.format,true,samples))) {self.key=None;self.caps.clear();}
  }
  pub fn invalidate(&mut self) {
    self.revision = self.revision.wrapping_add(1);
  }
```

**Locate this block:**

```rust
        "fs_alpha",
        wgpu::TextureFormat::R8Unorm,
        false,
      ),
      composite_pipeline: fullscreen(&ctx.device, &shader, &[&composite_layout], "fs_main", format, true),
      pool_layout,
      search_layout,
      composite_layout,
```

Replace it with:

```rust
        "fs_alpha",
        wgpu::TextureFormat::R8Unorm,
        false,
        1,
      ),
      pool_layout,
      search_layout,
      composite_layout,
```

**Locate this block:**

```rust
        "fs_dilate",
        wgpu::TextureFormat::R8Unorm,
        false,
      ),
      alpha_pipeline: fullscreen(
        &ctx.device,
```

Replace it with:

```rust
        "fs_dilate",
        wgpu::TextureFormat::R8Unorm,
        false,
        1,
      ),
      alpha_pipeline: fullscreen(
        &ctx.device,
```

**Locate this block:**

```rust
      label: Some("current surface outline"),
      source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
    });
    Self {
      mask_pipeline: mask_pipeline(&ctx.device),
      caps: crate::outline_caps::Caps::new(&ctx.device),
      pool_pipeline: fullscreen(&ctx.device, &shader, &[&pool_layout], "fs_pool", wgpu::TextureFormat::R8Unorm, false),
      dilate_pipeline: fullscreen(
        &ctx.device,
        &shader,
```

Replace it with:

```rust
      label: Some("current surface outline"),
      source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
    });
    let pipelines=Variants::new((mask_pipeline(&ctx.device,samples),fullscreen(&ctx.device,&shader,&[&composite_layout],"fs_main",format,true,samples)),samples);
    Self {
      pipelines,shader:shader.clone(),format,
      caps: crate::outline_caps::Caps::new(),
      pool_pipeline: fullscreen(&ctx.device, &shader, &[&pool_layout], "fs_pool", wgpu::TextureFormat::R8Unorm, false,1),
      dilate_pipeline: fullscreen(
        &ctx.device,
        &shader,
```

**Locate this block:**

```rust
  pub draws: u64,
}
impl Outline {
  pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
    let texture = |binding| texture_entry(binding, Stages::FRAGMENT, Sample::Float { filterable: false }, false);
    let uniform = |binding, bytes| wgpu::BindGroupLayoutEntry {
      binding,
```

Replace it with:

```rust
  pub draws: u64,
}
impl Outline {
  pub fn new(ctx:&GpuCtx,format:wgpu::TextureFormat)->Self {Self::with_samples(ctx,format,1)}
  pub fn with_samples(ctx:&GpuCtx,format:wgpu::TextureFormat,samples:u32)->Self {
    let texture = |binding| texture_entry(binding, Stages::FRAGMENT, Sample::Float { filterable: false }, false);
    let uniform = |binding, bytes| wgpu::BindGroupLayoutEntry {
      binding,
```

**Locate this block:**

```rust
}

pub struct Outline {
  mask_pipeline: wgpu::RenderPipeline,
  caps: crate::outline_caps::Caps,
  pool_pipeline: wgpu::RenderPipeline,
  dilate_pipeline: wgpu::RenderPipeline,
  alpha_pipeline: wgpu::RenderPipeline,
  composite_pipeline: wgpu::RenderPipeline,
  pool_layout: wgpu::BindGroupLayout,
  search_layout: wgpu::BindGroupLayout,
  composite_layout: wgpu::BindGroupLayout,
```

Replace it with:

```rust
}

pub struct Outline {
  pipelines:Variants<(wgpu::RenderPipeline,wgpu::RenderPipeline)>,
  shader:wgpu::ShaderModule,format:wgpu::TextureFormat,
  caps: crate::outline_caps::Caps,
  pool_pipeline: wgpu::RenderPipeline,
  dilate_pipeline: wgpu::RenderPipeline,
  alpha_pipeline: wgpu::RenderPipeline,
  pool_layout: wgpu::BindGroupLayout,
  search_layout: wgpu::BindGroupLayout,
  composite_layout: wgpu::BindGroupLayout,
```

**Locate this block:**

```rust
}
struct Masks {
  coverage: Image,
  coarse: Image,
  dilated: Image,
  alpha: Image,
```

Replace it with:

```rust
}
struct Masks {
  coverage: Image,
  coverage4:Option<Image>,
  coarse: Image,
  dilated: Image,
  alpha: Image,
```

**Locate this block:**

```rust
  size: [u32; 2],
}
impl Image {
  fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some(label),
      size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::R8Unorm,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT
        | wgpu::TextureUsages::TEXTURE_BINDING
        | wgpu::TextureUsages::COPY_SRC,
      view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Self { texture, view, size }
  }
  fn bytes(&self) -> u64 {
    self.size[0] as u64 * self.size[1] as u64
  }
}
impl Drop for Image {
```

Replace it with:

```rust
  size: [u32; 2],
}
impl Image {
  fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {Self::with_samples(device,size,label,1)}
  fn with_samples(device:&wgpu::Device,size:[u32;2],label:&str,samples:u32)->Self {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some(label),
      size: wgpu::Extent3d { width: size[0], height: size[1], depth_or_array_layers: 1 },
      mip_level_count: 1,
      sample_count: samples,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::R8Unorm,
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT
        | wgpu::TextureUsages::TEXTURE_BINDING
        | if samples==1 {wgpu::TextureUsages::COPY_SRC}else{wgpu::TextureUsages::empty()},
      view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    Self { texture, view, size }
  }
  fn bytes(&self) -> u64 {
    self.size[0] as u64 * self.size[1] as u64 *u64::from(self.texture.sample_count())
  }
}
impl Drop for Image {
```

**Locate this block:**

```rust
use crate::{
  gpu_mesh::GpuMesh,
  text_gpu_support::{GpuCtx, resource_group, texture_entry},
```

Replace it with:

```rust
use crate::sample_pipelines::Variants;
use crate::{
  gpu_mesh::GpuMesh,
  text_gpu_support::{GpuCtx, resource_group, texture_entry},
```

### 2. `src/outline_caps.rs`

An empty section recipe no longer needs a device; allocate only when first used.

**Locate this existing block:**

```rust
 pub fn new(_:&wgpu::Device)->Self {Self {pipelines:None,input:None,group:None}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 pub fn new()->Self {Self {pipelines:None,input:None,group:None}}
```

### 3. `src/renderer.rs`

Late-created outline and final-blend owners adopt the current physical count. Do not prepare a section coverage recipe when no active section can contribute.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));
            global.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,Some(&self.targets.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::new(&ctx,self.format));selected.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,Some(&self.targets.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::new(&self.device,self.format)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::new(&self.text_context(),self.format));outline.encode(&self.text_context(),&mut encoder,view,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
```

Replace it with:

```rust
        {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("measurement overlay"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:&self.targets.depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});self.measurement.draw(&mut pass,&self.view_group);self.guides.draw(&mut pass,&self.view_group);}
        if self.outlines_enabled.get()&&!self.meshes.is_empty(){
            let ctx=self.text_context();
            let mut global=self.global_outline.borrow_mut();let global=global.get_or_insert_with(||crate::outline_gpu::Outline::with_samples(&ctx,self.format,self.targets.samples));
            global.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,true,(self.clipping.borrow().enabled&&!self.clipping.borrow().rows.is_empty()).then_some(&self.targets.primitive.view));
            let mut selected=self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected){let selected=selected.get_or_insert_with(||crate::outline_gpu::Outline::with_samples(&ctx,self.format,self.targets.samples));selected.encode_masks(&ctx,&mut encoder,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision,false,(self.clipping.borrow().enabled&&!self.clipping.borrow().rows.is_empty()).then_some(&self.targets.primitive.view));}else{selected.take();}
            if let Some(alpha)=global.alpha_view(){self.outline_composite.borrow_mut().get_or_insert_with(||crate::outline_composite::Composite::with_samples(&self.device,self.format,self.targets.samples)).draw(&self.device,&mut encoder,view,alpha,selected.as_ref().and_then(|s|s.alpha_view()));}
        }else{
            self.global_outline.borrow_mut().take();self.outline_composite.borrow_mut().take();
            if self.meshes.iter().any(GpuMesh::selected){let mut outline=self.outline.borrow_mut();let outline=outline.get_or_insert_with(||crate::outline_gpu::Outline::with_samples(&self.text_context(),self.format,self.targets.samples));outline.encode(&self.text_context(),&mut encoder,view,&self.targets.depth,&self.view_group,&self.meshes,*transform,self.size,self.density,&clip_group,clip_revision);}else{self.outline.borrow_mut().take();}
        }
        self.text_error.borrow_mut().take();
        if let Some(lane)=self.text.borrow_mut().as_mut() {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/sample_surface_tests.rs`

Extend the supplied actual sample-count fixture to current outline owners; do not introduce the complete production renderer API before its remaining consumers.

**Locate this existing block:**

```text
 if let Some(text)=r.text.borrow_mut().as_mut(){text.retarget(&r.text_context(),crate::text_gpu_support::Target{format:r.format,samples:count});}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzf-resolve-float-outline-coverage-58.rs" skip=146
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/sample_surface_tests.rs`

Actual selected/global face and section coverage, bounded active-image memory, single black blend, exact returns and fresh original owners; retain every previous assertion.

**Locate this existing block:**

```text
r{count}x text");}
 }
 assert_eq!(serde_json::to_string(&*original).unwrap(),source);crate::sample_test_proof::write(root.join("one.rgba"),single).unwrap();
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzf-resolve-float-outline-coverage-59.rs" skip=160
```

## <a id="checkpoint-96zzzzzzzg-retarget-original-ambient-properties"></a>14. Retarget ambient properties

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 14 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/ambient_source_pipeline…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Original face radii/norma…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching physical source…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained one-sample ambie…</text>
</svg>
</div>

**What this code does:** Prepare cached physical-sample original radius and normal sources while retaining original geometry and the one-sample ambient hierarchy, filters and history images.

### 1. `src/ambient_source_pipeline.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Replace the entire file with:

```rust
pub struct Pipeline{pub draw:wgpu::RenderPipeline,pub record_draw:wgpu::RenderPipeline}
impl Pipeline{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{
  let create=|shared| {
  let view=crate::surface_view::layout(device);let model=if shared {crate::object_rows::layout(device)}else{crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64)};let clip=crate::clipping_gpu::layout(device);let geometry=crate::gpu_geometry::GpuGeometry::storage_layout(device);let groups=[Some(&view),Some(&model),Some(&clip),Some(&geometry)];let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("ambient original source metadata"),bind_group_layouts:&groups[..if shared{4}else{3}],immediate_size:0});let source=concat!(include_str!("triangle.wgsl"),"
",include_str!("ambient_sources.wgsl"));let source=if shared {crate::object_rows::source(source)}else{source.into()};let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("ambient original normals and radii"),source:wgpu::ShaderSource::Wgsl(if samples==4{crate::clipping_gpu::source(&source,2,0).replace("@location(0) world:","@location(0) @interpolate(perspective,sample) world:").into()}else{crate::clipping_gpu::source(&source,2,0).into()})});
  let attributes=wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some(if shared {"vs_record"}else{"vs_main"}),compilation_options:Default::default(),buffers:&[if shared {crate::draw_slots::layout()}else{wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None});draw};let draw=create(false);let record_draw=create(true);Self{draw,record_draw}
 }
}
```

### 2. `src/ambient_source_gpu.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Replace the entire file with:

```rust
use crate::{sample_pipelines::Variants,ambient_source_pipeline::Pipeline};
pub struct Sources{pipes:Variants<Pipeline>}
impl Sources{
 pub fn new(device:&wgpu::Device)->Self{Self::with_samples(device,1)}
 pub fn with_samples(device:&wgpu::Device,samples:u32)->Self{Self{pipes:Variants::new(Pipeline::with_samples(device,samples),samples)}}
 pub fn set_samples(&mut self,device:&wgpu::Device,samples:u32){self.pipes.select(samples,||Pipeline::with_samples(device,samples));}
 pub fn encode(&mut self,encoder:&mut wgpu::CommandEncoder,radius:&wgpu::TextureView,normals:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clip:&wgpu::BindGroup,meshes:&[crate::gpu_mesh::GpuMesh],records:Option<&crate::instance_gpu::Lane>){
  let attachments=[radius,normals].map(|view|Some(wgpu::RenderPassColorAttachment{view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}}));let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original ambient source properties"),color_attachments:&attachments,depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&self.pipes.active.draw);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clip,&[]);for mesh in meshes{if records.is_some_and(|rows|rows.bind_record(&mut pass,mesh)){pass.set_pipeline(&self.pipes.active.record_draw);mesh.geometry.draw_pulled(&mut pass,1);}else{pass.set_pipeline(&self.pipes.active.draw);mesh.bind_model(&mut pass);mesh.geometry.draw(&mut pass);}}
 }
}
```

### 3. `src/ambient_image.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Replace the entire file with:

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub bytes:u64,pub size:[u32;2]}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat)->Self{Self::with_samples(device,size,format,1)}
 pub fn with_samples(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("ambient actual image"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:samples,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT|if samples==1 {wgpu::TextureUsages::COPY_SRC}else{wgpu::TextureUsages::empty()},view_formats:&[]});let view=texture.create_view(&Default::default());let bytes=u64::from(size[0])*u64::from(size[1])*u64::from(format.block_copy_size(None).unwrap())*u64::from(samples);Self{texture,view,bytes,size}}
}
```

### 4. `src/ambient_images.rs`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Replace the entire file with:

```rust
use crate::{ambient_image::Image,pyramid_image::Image as Pyramid};
pub struct Images{pub full:[u32;2],pub size:[u32;2],pub source_radius:Image,pub source_normals:Image,pub linear:Pyramid,pub radii:Pyramid,pub normals:Image,pub occupancy:Image,pub ground:Image,pub ao:[Image;3],pub history:Image,pub history_depth:Image}
impl Images{
 pub fn new(device:&wgpu::Device,full:[u32;2],density:f32)->Self{Self::with_samples(device,full,density,1)}
 pub fn with_samples(device:&wgpu::Device,full:[u32;2],density:f32,samples:u32)->Self{
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];let image=|size,format|Image::new(device,size,format);
  let source_radius=Image::with_samples(device,full,wgpu::TextureFormat::R32Float,samples);let source_normals=Image::with_samples(device,full,wgpu::TextureFormat::Rg8Unorm,samples);let linear=Pyramid::new(device,size,wgpu::TextureFormat::R32Float);let radii=Pyramid::new(device,size,wgpu::TextureFormat::R16Uint);let normals=image(size,wgpu::TextureFormat::Rg8Unorm);let occupancy=image(size,wgpu::TextureFormat::R8Unorm);let ground=image(size,wgpu::TextureFormat::R8Unorm);let ao=std::array::from_fn(|i|image(if i==2{full}else{size},wgpu::TextureFormat::R8Unorm));let history=image(size,wgpu::TextureFormat::R8Unorm);let history_depth=image(size,wgpu::TextureFormat::R32Float);Self{full,size,source_radius,source_normals,linear,radii,normals,occupancy,ground,ao,history,history_depth}
 }
 pub fn retarget_sources(&mut self,device:&wgpu::Device,samples:u32){
  self.source_radius=Image::with_samples(device,self.full,wgpu::TextureFormat::R32Float,samples);self.source_normals=Image::with_samples(device,self.full,wgpu::TextureFormat::Rg8Unorm,samples);
 }
 pub fn bytes(&self)->u64{self.source_radius.bytes+self.source_normals.bytes+self.linear.bytes+self.radii.bytes+self.normals.bytes+self.occupancy.bytes+self.ground.bytes+self.ao.iter().map(|i|i.bytes).sum::<u64>()+self.history.bytes+self.history_depth.bytes}
}
```

### 5. `src/ambient.wgsl`

Prepare cached original-radius/normal sample recipes and retarget only their full-resolution source images; retain the one-sample ambient hierarchy and filters.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
```

Replace it with:

```wgsl
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let primitive=source_primitive(source_at(pixel.xy),0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) || source_primitive(source_at(pixel.xy),i)!=primitive {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
```

**Locate this block:**

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

Replace it with:

```wgsl
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at,sample);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
```

**Locate this block:**

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

Replace it with:

```wgsl
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at,0)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

**Locate this block:**

```wgsl
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),0).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let primitive=source_primitive(at);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

Replace it with:

```wgsl
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_primitive(at:vec2<i32>,sample:i32)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),sample).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>,sample:i32)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 let primitive=source_primitive(at,sample);
 return select(radius,ambient.params.y,radius==0.0&&clip_is_cap(primitive));
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,sample).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,sample).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let primitive=source_primitive(at,sample);
 if(clip_is_cap(primitive)){
  let plane=clip_cap_plane(primitive);
  if(plane<clipping.count){let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
```

### 6. `src/ambient_pipeline.rs`

Assemble the matching original-property/depth input declarations while keeping the coarse hierarchy single-sample. Use the assembled single-sample source without changing the current ambient owner.

Replace the entire file with:

```rust
use wgpu::{ShaderStages as S,TextureSampleType as T,BufferBindingType as B};
pub struct Pipelines{pub source:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout,pub sample:wgpu::BindGroupLayout,pub history:wgpu::BindGroupLayout,pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub occupancy:wgpu::RenderPipeline,pub ground:wgpu::RenderPipeline,pub raw:wgpu::RenderPipeline,pub filter:[wgpu::RenderPipeline;2],pub upsample:wgpu::RenderPipeline,pub composite:wgpu::RenderPipeline}
fn pipeline(device:&wgpu::Device,source:&str,fragment:&str,groups:&[Option<&wgpu::BindGroupLayout>],formats:&[wgpu::TextureFormat],blend:bool)->wgpu::RenderPipeline{
 let module=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("current ambient recipe"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("current ambient recipe"),bind_group_layouts:groups,immediate_size:0});let targets=formats.iter().map(|format|Some(wgpu::ColorTargetState{format:*format,blend:if blend{Some(wgpu::BlendState::ALPHA_BLENDING)}else{None},write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>();
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("current ambient recipe"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:None,multisample:Default::default(),multiview_mask:None,cache:None})
}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let texture=|binding,sample|crate::text_gpu_support::texture_entry(binding,S::FRAGMENT,sample,false);let uniform=|binding|crate::tile_pipelines::entry(binding,S::FRAGMENT,B::Uniform);
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture(0,T::Depth),uniform(1),texture(2,T::Float{filterable:false}),texture(3,T::Float{filterable:false}),texture(4,T::Uint),uniform(5)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reduced original properties"),entries:&[texture(0,T::Float{filterable:false}),texture(1,T::Uint),texture(2,T::Float{filterable:false})]});
  let sample=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient filtered samples"),entries:&[texture(0,T::Float{filterable:true}),wgpu::BindGroupLayoutEntry{binding:1,visibility:S::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None}]});
  let mut entries=(0..4).map(|i|crate::tile_pipelines::entry(i,S::FRAGMENT,B::Storage{read_only:false})).collect::<Vec<_>>();entries.push(texture(4,T::Float{filterable:true}));entries.push(texture(5,T::Float{filterable:false}));let history=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reserved current history bindings"),entries:&entries});
  let shader=shader_source(1);let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false);let make=|fragment|pipeline(device,&shader,fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");let composite=pipeline(device,include_str!("ambient_composite.wgsl"),"fs_main",&[None,Some(&sample)],&[format],true);
  let reduce=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false);let occupancy=pipeline(device,include_str!("ambient_depth.wgsl"),"fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite}
 }
}

pub(crate) fn shader_source(samples:u32)->String {
 let source=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);if samples==1 {return source;}
 source.replace("const MSAA: bool = false;","const MSAA: bool = true;")
 .replace("var depth: texture_depth_2d;","var depth: texture_depth_multisampled_2d;")
 .replace("var source_radii:texture_2d<f32>;","var source_radii:texture_multisampled_2d<f32>;")
 .replace("var source_normals:texture_2d<f32>;","var source_normals:texture_multisampled_2d<f32>;")
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/ambient_shader_contract.rs`

Retain both existing full Naga contracts and additionally validate both assembled original-property sample modes before their consumer is connected.

**Locate this existing block:**

```text
        include_str!("ambient_composite.wgsl").to_string(),
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzg-retarget-original-ambient-properties-50.rs" skip=59
```

## <a id="checkpoint-96zzzzzzzh-share-ambient-shader-recipes"></a>15. Share ambient shader recipes

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 15 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/ambient_pipeline.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Three distinct shader sou…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit vertex/fragment/…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">retained composition inpu…</text>
</svg>
</div>

**What this code does:** Reuse ambient shader modules and one pipeline factory, prepare matching physical input layouts and connect retained composition bindings before activating sparse sample corrections.

### 1. `src/ambient_pipeline.rs`

Share actual ambient shader modules, match physical inputs and prepare bounded sparse sample corrections while retaining the coarse one-sample recipes.

Replace the entire file with:

```rust
use crate::{tile_pipelines::entry,text_gpu_support::texture_entry};
use wgpu::{ShaderStages as S,TextureSampleType as T,BufferBindingType as B};
pub struct Pipelines{pub source:wgpu::BindGroupLayout,pub level:wgpu::BindGroupLayout,pub sample:wgpu::BindGroupLayout,pub history:wgpu::BindGroupLayout,pub prepare:wgpu::RenderPipeline,pub reduce:wgpu::RenderPipeline,pub occupancy:wgpu::RenderPipeline,pub ground:wgpu::RenderPipeline,pub raw:wgpu::RenderPipeline,pub filter:[wgpu::RenderPipeline;2],pub upsample:wgpu::RenderPipeline,pub composite:wgpu::RenderPipeline,pub edges:Option<wgpu::RenderPipeline>,pub composite_layout:wgpu::BindGroupLayout}
fn pipeline(device:&wgpu::Device,module:&wgpu::ShaderModule,vertex:&str,fragment:&str,groups:&[Option<&wgpu::BindGroupLayout>],formats:&[wgpu::TextureFormat],blend:bool,samples:u32)->wgpu::RenderPipeline{
 let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("current ambient recipe"),bind_group_layouts:groups,immediate_size:0});let targets=formats.iter().map(|format|Some(wgpu::ColorTargetState{format:*format,blend:if blend{Some(wgpu::BlendState::ALPHA_BLENDING)}else{None},write_mask:wgpu::ColorWrites::ALL})).collect::<Vec<_>>();
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("current ambient recipe"),layout:Some(&layout),vertex:wgpu::VertexState{module,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[]},fragment:Some(wgpu::FragmentState{module,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets}),primitive:Default::default(),depth_stencil:None,multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));
  let texture=|binding,sample|texture_entry(binding,S::FRAGMENT,sample,false);let uniform=|binding|entry(binding,S::FRAGMENT,B::Uniform);
  let source=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient source images and precise camera"),entries:&[texture_entry(0,S::FRAGMENT,T::Depth,samples==4),uniform(1),texture_entry(2,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(3,S::FRAGMENT,T::Float{filterable:false},samples==4),texture_entry(4,S::FRAGMENT,T::Uint,samples==4),uniform(5)]});
  let level=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reduced original properties"),entries:&[texture(0,T::Float{filterable:false}),texture(1,T::Uint),texture(2,T::Float{filterable:false})]});
  let sample=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient filtered samples"),entries:&[texture(0,T::Float{filterable:true}),wgpu::BindGroupLayoutEntry{binding:1,visibility:S::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None}]});
  let mut entries=(0..4).map(|i|entry(i,S::FRAGMENT,B::Storage{read_only:false})).collect::<Vec<_>>();entries.push(texture(4,T::Float{filterable:true}));entries.push(texture(5,T::Float{filterable:false}));let history=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient reserved current history bindings"),entries:&entries});
  let shader_source=shader_source(samples);let module=|label,source:String|device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some(label),source:wgpu::ShaderSource::Wgsl(source.into())});
  let shader=module("current ambient recipe",shader_source);let depth_shader=module("ambient depth recipe",include_str!("ambient_depth.wgsl").into());let composite_shader=module("ambient composition",include_str!("ambient_composite.wgsl").into());
  let groups=[Some(&source),Some(&level),Some(&sample),Some(&history)];let prepare=pipeline(device,&shader,"vs_main","fs_prepare",&[Some(&source)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint,wgpu::TextureFormat::Rg8Unorm],false,1);let make=|fragment|pipeline(device,&shader,"vs_main",fragment,&groups,&[wgpu::TextureFormat::R8Unorm],false,1);
  let ground=make("fs_ground");let raw=make("fs_main");let filter=[make("fs_filter_x"),make("fs_filter_y")];let upsample=make("fs_upsample");
  let composite_layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("ambient sparse sample corrections"),entries:&[entry(0,S::VERTEX_FRAGMENT,B::Uniform),entry(1,S::FRAGMENT,B::Storage{read_only:true}),entry(2,S::VERTEX,B::Storage{read_only:true})]});
  let composite=pipeline(device,&composite_shader,"vs_main","fs_main",&[Some(&composite_layout),Some(&sample)],&[format],true,samples);
  let edges=(samples==4).then(||pipeline(device,&composite_shader,"vs_edges","fs_edges",&[Some(&composite_layout),Some(&sample)],&[format],true,samples));
  let reduce=pipeline(device,&depth_shader,"vs_main","fs_reduce",&[Some(&level)],&[wgpu::TextureFormat::R32Float,wgpu::TextureFormat::R16Uint],false,1);let occupancy=pipeline(device,&depth_shader,"vs_main","fs_occupancy",&[Some(&level)],&[wgpu::TextureFormat::R8Unorm],false,1);
  Self{source,level,sample,history,prepare,reduce,occupancy,ground,raw,filter,upsample,composite,edges,composite_layout}
 }
}
pub(crate) fn shader_source(samples:u32)->String {
 let source=crate::clipping_gpu::source(include_str!("ambient.wgsl"),0,5);if samples==1 {return source;}
 source.replace("const MSAA: bool = false;","const MSAA: bool = true;")
 .replace("var depth: texture_depth_2d;","var depth: texture_depth_multisampled_2d;")
 .replace("var source_radii:texture_2d<f32>;","var source_radii:texture_multisampled_2d<f32>;")
 .replace("var source_normals:texture_2d<f32>;","var source_normals:texture_multisampled_2d<f32>;")
 .replace("var source_primitives:texture_2d<u32>;","var source_primitives:texture_multisampled_2d<u32>;")
}
```

### 2. `src/ambient_groups.rs`

Share actual ambient shader modules, match physical inputs and prepare bounded sparse sample corrections while retaining the coarse one-sample recipes.

Replace the entire file with:

```rust
pub struct Groups{pub levels:[wgpu::BindGroup;6],pub occupancy:wgpu::BindGroup,pub ground:wgpu::BindGroup,pub ao:[wgpu::BindGroup;3],pub history:wgpu::BindGroup,pub composite:wgpu::BindGroup,pub buffers:[wgpu::Buffer;4]}
fn sampled(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,view:&wgpu::TextureView,sampler:&wgpu::Sampler)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained sample"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(view)},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::Sampler(sampler)}]})}
impl Groups{
 pub fn new(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,frame:&wgpu::Buffer)->Self{
  let levels=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained depth and properties"),layout:&pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&images.linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&images.radii.levels[i])},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.normals.view)}]}));
  let sampler=device.create_sampler(&wgpu::SamplerDescriptor{mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});let occupancy=sampled(device,&pipes.sample,&images.occupancy.view,&sampler);let ground=sampled(device,&pipes.sample,&images.ground.view,&sampler);let ao=std::array::from_fn(|i|sampled(device,&pipes.sample,&images.ao[i].view,&sampler));
  let buffers=std::array::from_fn(|i|device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient reserved single-sample work"),size:16,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|if i==3{wgpu::BufferUsages::INDIRECT}else{wgpu::BufferUsages::empty()},mapped_at_creation:false}));let mut entries=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&images.history.view)});entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&images.history_depth.view)});let history=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient inactive single-sample history"),layout:&pipes.history,entries:&entries});let composite=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient sparse sample corrections"),layout:&pipes.composite_layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:buffers[0].as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:buffers[2].as_entire_binding()}]});Self{levels,occupancy,ground,ao,history,composite,buffers}
 }
 pub fn source(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,depth:&wgpu::TextureView,sections:&wgpu::TextureView,frame:&wgpu::Buffer,clip:&wgpu::Buffer)->wgpu::BindGroup{
  let entries=[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.source_radius.view)},wgpu::BindGroupEntry{binding:3,resource:wgpu::BindingResource::TextureView(&images.source_normals.view)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(sections)},wgpu::BindGroupEntry{binding:5,resource:clip.as_entire_binding()}];device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient exact current input"),layout:&pipes.source,entries:&entries})
 }
}
```

### 3. `src/ambient_gpu.rs`

Connect the retained camera and bounded one-sample work buffers to the current ambient groups. Bind the existing sparse-composition input group while the current ambient owner remains single-sample.

Replace the entire file with:

```rust
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:crate::ambient_pipeline::Pipelines,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self{pipes:crate::ambient_pipeline::Pipelines::new(device,format),sources:crate::ambient_source_gpu::Sources::new(device),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn clear(&mut self){self.state=None;self.cached=None;self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::new(device,full,density);let groups=Groups::new(device,&self.pipes,&images,&self.frame);let group=Groups::source(device,&self.pipes,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear();return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  draw(encoder,&self.pipes.composite,&[color],&[(0,&state.groups.composite),(1,&state.groups.ao[2])],false);self.passes+=1;self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

## <a id="checkpoint-96zzzzzzzi-correct-ambient-at-physical-samples"></a>16. Correct ambient at physical samples

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 16 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/ambient_gpu.rs, src/amb…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Matching original physica…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">changed-sample corrections</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">indirect colour draw</text>
</svg>
</div>

**What this code does:** Connect matching original depth/address/radius/normal samples to bounded sparse ambient correction while retaining coarse images, source geometry and cached recipes.

### 1. `src/ambient_gpu.rs`

Retain coarse ambient images while matching physical inputs and execute bounded sparse per-sample corrections without runtime readback flags.

Replace the entire file with:

```rust
use crate::sample_pipelines::Variants;
use crate::{ambient_images::Images,ambient_groups::Groups,ambient_pass::draw};
struct State{images:Images,groups:Groups,input:(wgpu::TextureView,wgpu::TextureView,wgpu::BindGroup)}
pub struct Ambient{pipes:Variants<crate::ambient_pipeline::Pipelines>,format:wgpu::TextureFormat,pub sources:crate::ambient_source_gpu::Sources,frame:wgpu::Buffer,state:Option<State>,cached:Option<([f32;80],[u64;2])>,pub preparations:u64,pub writes:u64,pub passes:u64,pub error:Option<&'static str>}
impl Ambient{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{Self::with_samples(device,format,1)}
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,count:u32)->Self{Self{pipes:Variants::new(crate::ambient_pipeline::Pipelines::with_samples(device,format,count),count),format,sources:crate::ambient_source_gpu::Sources::with_samples(device,count),frame:device.create_buffer(&wgpu::BufferDescriptor{label:Some("current ambient camera"),size:320,usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),state:None,cached:None,preparations:0,writes:0,passes:0,error:None}}
 pub fn set_samples(&mut self,device:&wgpu::Device,count:u32){if self.pipes.select(count,||crate::ambient_pipeline::Pipelines::with_samples(device,self.format,count)){self.sources.set_samples(device,count);self.cached=None;}}
 pub fn clear(&mut self){self.state=None;self.cached=None;self.error=None;}
 fn prepare(&mut self,device:&wgpu::Device,full:[u32;2],density:f32,depth:&wgpu::TextureView,sections:&wgpu::TextureView,clip:&wgpu::Buffer){
  let(w,h)=crate::pyramid_size::resolution((full[0],full[1]),f64::from(density));let size=[w.max(32),h.max(32)];
  if self.state.as_ref().is_none_or(|s|s.images.full!=full||s.images.size!=size){let images=Images::with_samples(device,full,density,self.pipes.samples);let groups=Groups::new(device,&self.pipes.active,&images,&self.frame,self.pipes.samples);let group=Groups::source(device,&self.pipes.active,&images,depth,sections,&self.frame,clip);self.state=Some(State{images,groups,input:(depth.clone(),sections.clone(),group)});self.cached=None;self.preparations+=1;}
  let state=self.state.as_mut().unwrap();
  if state.images.source_radius.texture.sample_count()!=self.pipes.samples {state.images.retarget_sources(device,self.pipes.samples);state.groups=Groups::new(device,&self.pipes.active,&state.images,&self.frame,self.pipes.samples);state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes.active,&state.images,depth,sections,&self.frame,clip));self.cached=None;self.preparations+=1;}
if state.input.0!=*depth||state.input.1!=*sections{state.input=(depth.clone(),sections.clone(),Groups::source(device,&self.pipes.active,&state.images,depth,sections,&self.frame,clip));self.cached=None;}
 }
 pub fn draw(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,clip:&wgpu::Buffer,sections:&wgpu::TextureView,meshes:&[crate::gpu_mesh::GpuMesh],matrix:[f32;16],full:[u32;2],density:f32,ground:[f32;2],revision:[u64;2],records:Option<&crate::instance_gpu::Lane>){
  if ground[1]<=0.0{self.clear();return;}if matrix.iter().any(|v|!v.is_finite())||full.contains(&0)||!density.is_finite()||density<=0.{self.error=Some("Invalid ambient view");return;}
  self.prepare(device,full,density,depth,sections,clip);let state=self.state.as_ref().unwrap();let Some(uniform)=crate::ambient_camera::uniform(matrix,full,state.images.size,ground)else{self.error=Some("Invalid ambient camera inverse");return;};
  if self.cached!=Some((uniform,revision)){
   queue.write_buffer(&self.frame,0,&uniform.into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>());self.writes+=1;
   if self.pipes.samples==4 {encoder.clear_buffer(&state.groups.buffers[1],0,None);encoder.clear_buffer(&state.groups.buffers[3],4,Some(4));}
   let images=&state.images;let groups=&state.groups;let source=&state.input.2;self.sources.encode(encoder,&images.source_radius.view,&images.source_normals.view,depth,view,clipping,meshes,records);self.passes+=1;
   draw(encoder,&self.pipes.active.prepare,&[&images.linear.levels[0],&images.radii.levels[0],&images.normals.view],&[(0,source)],true);
   for level in 1..6{draw(encoder,&self.pipes.active.reduce,&[&images.linear.levels[level],&images.radii.levels[level]],&[(0,&groups.levels[level-1])],true);}self.passes+=6;
   draw(encoder,&self.pipes.active.occupancy,&[&images.occupancy.view],&[(0,&groups.levels[0])],true);
   draw(encoder,&self.pipes.active.ground,&[&images.ground.view],&[(0,source),(1,&groups.levels[0]),(2,&groups.occupancy),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.raw,&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ground),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[0],&[&images.ao[1].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.filter[1],&[&images.ao[0].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[1]),(3,&groups.history)],true);
   draw(encoder,&self.pipes.active.upsample,&[&images.ao[2].view],&[(0,source),(1,&groups.levels[0]),(2,&groups.ao[0]),(3,&groups.history)],true);self.passes+=6;
   self.cached=Some((uniform,revision));
  }
  {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("ambient sample-preserving composition"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,resolve_target:None,depth_slice:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&self.pipes.active.composite);pass.set_bind_group(0,&state.groups.composite,&[]);pass.set_bind_group(1,&state.groups.ao[2],&[]);pass.draw(0..3,0..1);if let Some(edges)=&self.pipes.active.edges{pass.set_pipeline(edges);pass.draw_indirect(&state.groups.buffers[3],0);}}
  self.passes+=1+u64::from(self.pipes.samples==4);self.error=None;
 }
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.state.as_ref().map_or(0,|s|s.images.bytes()+s.groups.buffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

### 2. `src/ambient_groups.rs`

Retain coarse ambient images while matching physical inputs and execute bounded sparse per-sample corrections without runtime readback flags.

Replace the entire file with:

```rust
pub struct Groups{pub levels:[wgpu::BindGroup;6],pub occupancy:wgpu::BindGroup,pub ground:wgpu::BindGroup,pub ao:[wgpu::BindGroup;3],pub history:wgpu::BindGroup,pub composite:wgpu::BindGroup,pub buffers:[wgpu::Buffer;4]}
fn sampled(device:&wgpu::Device,layout:&wgpu::BindGroupLayout,view:&wgpu::TextureView,sampler:&wgpu::Sampler)->wgpu::BindGroup{device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained sample"),layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(view)},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::Sampler(sampler)}]})}
impl Groups{
 pub fn new(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,frame:&wgpu::Buffer,samples:u32)->Self{
  let levels=std::array::from_fn(|i|device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient retained depth and properties"),layout:&pipes.level,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&images.linear.levels[i])},wgpu::BindGroupEntry{binding:1,resource:wgpu::BindingResource::TextureView(&images.radii.levels[i])},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.normals.view)}]}));
  let sampler=device.create_sampler(&wgpu::SamplerDescriptor{mag_filter:wgpu::FilterMode::Linear,min_filter:wgpu::FilterMode::Linear,..Default::default()});let occupancy=sampled(device,&pipes.sample,&images.occupancy.view,&sampler);let ground=sampled(device,&pipes.sample,&images.ground.view,&sampler);let ao=std::array::from_fn(|i|sampled(device,&pipes.sample,&images.ao[i].view,&sampler));
  let pixels=u64::from(images.full[0])*u64::from(images.full[1]);let tiles=u64::from(images.full[0].div_ceil(16))*u64::from(images.full[1].div_ceil(16));
  let buffers=std::array::from_fn(|i|{let buffer=device.create_buffer(&wgpu::BufferDescriptor{label:Some("ambient bounded sample work"),size:if i==3||samples==1{16}else if i==0{pixels*4}else{tiles*4},usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_DST|if i==3{wgpu::BufferUsages::INDIRECT}else{wgpu::BufferUsages::empty()},mapped_at_creation:i==3});if i==3{buffer.slice(0..4).get_mapped_range_mut().copy_from_slice(&6u32.to_ne_bytes());buffer.unmap();}buffer});let mut entries=buffers.iter().enumerate().map(|(i,b)|wgpu::BindGroupEntry{binding:i as u32,resource:b.as_entire_binding()}).collect::<Vec<_>>();entries.push(wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(&images.history.view)});entries.push(wgpu::BindGroupEntry{binding:5,resource:wgpu::BindingResource::TextureView(&images.history_depth.view)});let history=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient inactive single-sample history"),layout:&pipes.history,entries:&entries});let composite=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient sparse sample corrections"),layout:&pipes.composite_layout,entries:&[wgpu::BindGroupEntry{binding:0,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:1,resource:buffers[0].as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:buffers[2].as_entire_binding()}]});Self{levels,occupancy,ground,ao,history,composite,buffers}
 }
 pub fn source(device:&wgpu::Device,pipes:&crate::ambient_pipeline::Pipelines,images:&crate::ambient_images::Images,depth:&wgpu::TextureView,sections:&wgpu::TextureView,frame:&wgpu::Buffer,clip:&wgpu::Buffer)->wgpu::BindGroup{
  let entries=[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(depth)},wgpu::BindGroupEntry{binding:1,resource:frame.as_entire_binding()},wgpu::BindGroupEntry{binding:2,resource:wgpu::BindingResource::TextureView(&images.source_radius.view)},wgpu::BindGroupEntry{binding:3,resource:wgpu::BindingResource::TextureView(&images.source_normals.view)},wgpu::BindGroupEntry{binding:4,resource:wgpu::BindingResource::TextureView(sections)},wgpu::BindGroupEntry{binding:5,resource:clip.as_entire_binding()}];device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("ambient exact current input"),layout:&pipes.source,entries:&entries})
 }
}
```

### 3. `src/renderer.rs`

Late-enabled ambient sources and colour recipes adopt the current frame count.

**Locate this existing block:**

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::new(&self.device,self.format));}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::with_samples(&self.device,self.format,self.targets.samples));owner.set_samples(&self.device,self.targets.samples);}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/ambient_sample_inspect.rs`

Supplied Native actual GPU resource handles expose existing inputs for independent compute; production images/buffers do not gain readback flags.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-50.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ambient_gpu.rs`

Register only the optional Native original-property/sparse-work observer.

**Locate this existing block:**

```text
uffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-51.rs" skip=100
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Extend the supplied Native count fixture to enabled ambient resources while retaining every prior owner.

**Locate this existing block:**

```text
fn retarget_basic_fixture(r:&mut Renderer,count:u32) {
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-52.rs" skip=55
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/sample_surface_tests.rs`

Read all actual physical depth/address/radius/normal samples, bounded sparse tiles/indirect work, retained coarse images and exact current output through repeated counts and ordinary/shared source paths.

**Locate this existing block:**

```text
,"Actual4x horizontal section and full solid share the same outer silhouette");crate::sample_test_proof::write(root.join("uncut-four.rgba"),uncut).unwrap();
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzi-correct-ambient-at-physical-samples-53.rs" skip=160
```

## <a id="checkpoint-96zzzzzzzj-cache-physical-depth-pyramid-inputs"></a>17. Cache physical depth-pyramid inputs

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 17 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/pyramid_pipeline.rs, sr…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Matching physical depth</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">representative sample zero</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">six retained single-sampl…</text>
</svg>
</div>

**What this code does:** Cache physical depth inputs while retaining the six one-sample coarse images and their encoded radius/pixel addresses.

### 1. `src/pyramid_pipeline.rs`

Cache the matching physical-depth input recipe; retain the exact render descriptor, one-sample reductions and six coarse image levels.

Replace the entire file with:

```rust
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-01.rs"
```

### 2. `src/pyramid_gpu.rs`

Retain coarse image identities and reduction groups; invalidate only an incompatible physical-depth input binding and its prepared result.

Replace the entire file with:

```rust
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-02.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/pyramid_gpu_tests.rs`

Extend the existing actual six-level positive-depth/radius-address GPU assertions to both physical inputs. The isolated fixture retargets only its active consumers until the next production API step.

Replace the entire file with:

```text
--8<-- "journey/code/96zzzzzzzj-cache-physical-depth-pyramid-inputs-03.rs"
```

## <a id="checkpoint-96zzzzzzzk-retarget-the-complete-colour-frame"></a>18. Retarget the complete colour frame

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 18 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs, src/browse…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">One count owner</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">compatible cached colour…</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">matching attachments; ind…</text>
</svg>
</div>

**What this code does:** Retarget every active colour consumer and frame attachment through one production API while preserving independent original-owner queries.

### 1. `src/renderer.rs`

Retarget every active physical-colour consumer through one production API; preserve independent query resources and no-op repeated counts. A late-enabled depth pyramid adopts the current physical input count.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::with_samples(&self.device,self.format,self.targets.samples));owner.set_samples(&self.device,self.targets.samples);}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::new(&self.device));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
```

Replace it with:

```rust
    pub fn set_ambient(&self,enabled:bool){if self.ambient_enabled.replace(enabled)==enabled{return;}let mut ambient=self.ambient.borrow_mut();if enabled{let owner=ambient.get_or_insert_with(||crate::ambient_gpu::Ambient::with_samples(&self.device,self.format,self.targets.samples));owner.set_samples(&self.device,self.targets.samples);}else if let Some(owner)=ambient.as_mut(){owner.clear();}}
    pub fn ambient_usage(&self)->[u64;5]{self.ambient.borrow().as_ref().map_or([0;5],crate::ambient_gpu::Ambient::usage)}
    pub fn ambient_error(&self)->Option<&'static str>{self.ambient.borrow().as_ref().and_then(|owner|owner.error)}
    pub fn set_depth_pyramid(&self,enabled:bool){let mut owner=self.pyramid.borrow_mut();if enabled {if owner.is_none(){*owner=Some(crate::pyramid_gpu::Pyramid::with_samples(&self.device,self.samples()));}}else{*owner=None;}}
    pub fn depth_pyramid_usage(&self)->[u64;4]{self.pyramid.borrow().as_ref().map_or([0;4],crate::pyramid_gpu::Pyramid::usage)}
    pub fn depth_pyramid_error(&self)->Option<&'static str>{self.pyramid.borrow().as_ref().and_then(|p|p.error)}
    pub fn cap_error(&self)->Option<&'static str>{self.caps.borrow().pick_error}
```

**Locate this block:**

```rust
        self.targets=crate::frame_targets::Targets::with_formats(&self.device,self.size,self.output_format,self.format,self.targets.samples);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }
```

Replace it with:

```rust
        self.targets=crate::frame_targets::Targets::with_formats(&self.device,self.size,self.output_format,self.format,self.targets.samples);
    }

    pub fn samples(&self)->u32{self.targets.samples}
    pub fn set_samples(&mut self,count:u32){
        assert!(matches!(count,1|4));
        if self.samples()==count{return;}
        let device=&self.device;
        let ctx=crate::text_gpu_support::GpuCtx::new(device,&self.queue);
        self.surfaces.select(count,||crate::surface_pipelines::pipelines(device,self.format,&crate::surface_view::layout(device),&crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64),count));
        self.instances.set_samples(device,self.format,count);
        for lane in [&mut self.mesh_ink,&mut self.strokes,&mut self.paths,&mut self.markers,&mut self.grid]{lane.set_samples(device,count);}
        self.highlight.set_samples(device,count);self.preview.set_samples(device,count);
        self.measurement.set_samples(device,count);self.guides.set_samples(device,count);
        self.sheets.set_samples(device,count);self.caps.borrow_mut().set_samples(count);
        if let Some(lane)=self.clouds.borrow_mut().as_mut(){lane.set_samples(&ctx,count);}
        if let Some(lane)=self.text.borrow_mut().as_mut(){lane.retarget(&ctx,crate::text_gpu_support::Target{format:self.format,samples:count});}
        for outline in [&self.global_outline,&self.outline]{if let Some(outline)=outline.borrow_mut().as_mut(){outline.set_samples(&ctx,count);}}
        if let Some(owner)=self.outline_composite.borrow_mut().as_mut(){owner.set_samples(device,count);}
        if self.ambient_enabled.get(){if let Some(owner)=self.ambient.borrow_mut().as_mut(){owner.set_samples(device,count);}}
        if let Some(owner)=self.pyramid.borrow_mut().as_mut(){owner.set_samples(device,count);}
        self.targets=crate::frame_targets::Targets::with_formats(device,self.size,self.output_format,self.format,count);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 && density != self.density { self.cancel_pick(); self.density = density; }
    }
```

### 2. `src/browser.rs`

Keep one optional diagnostic override; automatic default policy is connected next. Choose matching physical consumers on the initial viewport. Apply the same physical count choice after input and resize. Use the existing browser event path for actual count validation; semantic editor/source actions retain their existing dispatch. Register the bounded count override on the existing listener owner. Observe the actual production target count for source-bound Chrome assertions. Pass the single override into actual viewport configuration. Retarget colour consumers before drawing; the unchanged default remains one until the following automatic-quality lesson.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    editor.camera.aspect = size.aspect();
    true
}
```

Replace it with:

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    renderer.set_samples(sample_forced.unwrap_or(1));
    editor.camera.aspect = size.aspect();
    true
}
```

**Locate this block:**

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
```

Replace it with:

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_forced:Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
```

**Locate this block:**

```rust
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"lighting":editor.lit,"backfaces":editor.backfaces,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute("data-triangle-tiles",&serde_json::to_string(&renderer.tile_usage()).unwrap())?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
```

Replace it with:

```rust
    canvas.set_attribute("data-gizmo",&serde_json::json!({"origin":editor.gizmo_origin(),"active":editor.gizmo_active(),"pointer":editor.gizmo_edit.as_ref().and_then(|e|e.pointer),"typing":editor.gizmo_edit.as_ref().and_then(|e|e.gizmo.typing).map(|h|h.title()),"matrix":editor.gizmo_edit.as_ref().map(|e|e.preview.matrix()),"gpu":renderer.gizmo_usage()}).to_string())?;
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"lighting":editor.lit,"backfaces":editor.backfaces,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute("data-samples",&renderer.samples().to_string())?;
    canvas.set_attribute("data-triangle-tiles",&serde_json::to_string(&renderer.tile_usage()).unwrap())?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
```

**Locate this block:**

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(),"viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
```

Replace it with:

```rust
    listeners.listen(document.as_ref(), "change")?;
    listeners.listen(document.as_ref(), "cancel")?;
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(),"viewer-samples")?;
    listeners.listen(window.as_ref(),"viewer-origin")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
```

**Locate this block:**

```rust
            &mut config,
            &mut renderer,
            &mut editor,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
```

Replace it with:

```rust
            &mut config,
            &mut renderer,
            &mut editor,
            sample_forced,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
```

**Locate this block:**

```rust

        if let Err(error)=crate::browser_rebase::prepare(&mut renderer,&mut editor){report(&format!("Cannot prepare view: {error:?}"));return;}
        let mut highlight_changed=false;
        let action = if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
```

Replace it with:

```rust

        if let Err(error)=crate::browser_rebase::prepare(&mut renderer,&mut editor){report(&format!("Cannot prepare view: {error:?}"));return;}
        let mut highlight_changed=false;
        let action = if event.type_()=="viewer-samples"{
            if let Some(event)=event.dyn_ref::<web_sys::CustomEvent>(){
                let value=event.detail();
                if value.is_null(){sample_forced=None;}
                else if let Some(count)=value.as_f64().filter(|v|*v==1.||*v==4.){sample_forced=Some(count as u32);}
            }
            None
        }else if event.type_() == "cancel" && event.target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
            .is_some_and(|input| input.id() == "open")
        {
```

**Locate this block:**

```rust
        &mut config,
        &mut renderer,
        &mut editor,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&mut editor)?;
```

Replace it with:

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_forced,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&mut editor)?;
```

**Locate this block:**

```rust
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

Replace it with:

```rust
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let mut sample_forced=None;
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/cloud_address_tests.rs`

Exercise the production retarget API through every existing physical-colour regression; retain all source, pixel, cache and original-owner assertions.

**Locate this existing block:**

```text
pub(super) fn retarget(r:&mut Renderer,count:u32) {
 assert!(matches!(count,1|4));if r.targets.samples==count{return;}
 r.surfaces.select(count,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),count));
 r.instances.set_samples(&r.device,r.format,count);
 for lane in [&mut r.mesh_ink,&mut r.strokes,&mut r.paths,&mut r.markers,&mut r.grid] {lane.set_samples(&r.device,count);}
 r.highlight.set_samples(&r.device,count);r.preview.set_samples(&r.device,count);r.measurement.set_samples(&r.device,count);r.guides.set_samples(&r.device,count);r.caps.borrow_mut().set_samples(count);
 if let Some(cloud)=r.clouds.borrow_mut().as_mut(){cloud.set_samples(&r.text_context(),count);}
 r.targets=crate::frame_targets::Targets::with_formats(&r.device,r.size,r.output_format,r.format,count);
}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-11.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/sample_surface_tests.rs`

Exercise the production retarget API through every existing physical-colour regression; retain all source, pixel, cache and original-owner assertions.

**Locate this existing block:**

```text
fn retarget_basic_fixture(r:&mut Renderer,count:u32) {
 if r.ambient_enabled.get(){if let Some(owner)=r.ambient.borrow_mut().as_mut(){owner.set_samples(&r.device,count);}}
 super::cloud_address_tests::retarget(r,count);
 r.sheets.set_samples(&r.device,count);
 if let Some(text)=r.text.borrow_mut().as_mut(){text.retarget(&r.text_context(),crate::text_gpu_support::Target{format:r.format,samples:count});}
 let ctx=r.text_context();
 for owner in [&r.global_outline,&r.outline] {if let Some(owner)=owner.borrow_mut().as_mut(){owner.set_samples(&ctx,count);}}
 if let Some(owner)=r.outline_composite.borrow_mut().as_mut(){owner.set_samples(&r.device,count);}
}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-12.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/pyramid_gpu_tests.rs`

Exercise the production retarget API through every existing physical-colour regression; retain all source, pixel, cache and original-owner assertions.

**Locate this existing block:**

```text
fn retarget(r:&mut crate::renderer::Renderer,count:u32) {
 assert!(matches!(count,1|4));if r.targets.samples==count{return;}
 r.surfaces.select(count,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),count));
 r.instances.set_samples(&r.device,r.format,count);
 for lane in [&mut r.mesh_ink,&mut r.strokes,&mut r.paths,&mut r.markers,&mut r.grid] {lane.set_samples(&r.device,count);}
 r.highlight.set_samples(&r.device,count);r.preview.set_samples(&r.device,count);r.measurement.set_samples(&r.device,count);r.guides.set_samples(&r.device,count);r.caps.borrow_mut().set_samples(count);
 if let Some(cloud)=r.clouds.borrow_mut().as_mut(){cloud.set_samples(&r.text_context(),count);}
 r.targets=crate::frame_targets::Targets::with_formats(&r.device,r.size,r.output_format,r.format,count);
}
```

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-13.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Actual pending independent GPU owner queries survive colour-only count changes; repeated counts preserve target identity, source uploads and exact pixels.

**Locate this existing block:**

```text
before));assert_eq!(&scene.objects().iter().find(|o|o.id==mesh.id).unwrap().geometry().unwrap().to_proto(),source);}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-14.rs" skip=120
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/sample_composition_tests.rs`

Supplied complete original mixed-source GPU scene with both physical counts/projections, white top/clipped bores, every original owner/source/archive/geometry, and exact return pixels.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-15.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/renderer.rs`

Register the supplied actual combined original-family GPU witness.

**Locate this existing block:**

```text
mod sample_tests;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzk-retarget-the-complete-colour-frame-16.rs" skip=17
```

## <a id="checkpoint-96zzzzzzzl-choose-bounded-frame-quality"></a>19. Choose bounded frame quality

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="72" viewBox="0 0 1120 72" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
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
  <rect x="2" y="2" width="1116" height="68" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 19 of 19</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/sample_quality.rs, src/…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Active original geometry…</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">one quality policy</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/lib.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">complete-frame count API</text>
</svg>
</div>

**What this code does:** Choose coherent colour samples from actual geometry, physical viewport, device budget and recovery without changing original sources or owner queries.

### 1. `src/sample_quality.rs`

Choose a bounded physical count using active world geometry, exact device budgets, physical density and reduced-mode precedence.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzzl-choose-bounded-frame-quality-01.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 2. `src/sample_quality.rs`

Supplied Native boundary/overflow/override/recovery cases for the same production policy.

**Locate this existing block:**

```text
pub fn budget(gpu:wgpu::DeviceType)->Option<u32> {
    match gpu {
        wgpu::DeviceType::DiscreteGpu=>Some(9_000_000),
        wgpu::DeviceType::IntegratedGpu|wgpu::DeviceType::VirtualGpu=>Some(2_500_000),
        wgpu::DeviceType::Other=>Some(4_200_000),
        wgpu::DeviceType::Cpu=>None,
    }
}

pub fn samples(solid:bool,size:[u32;2],forced:Option<u32>,budget:Option<u32>,pixel_scale:f32,reduced:bool)->u32 {
    if reduced {return 1;}
    if let Some(count)=forced {return if count==4 {4}else{1};}
    if pixel_scale>=2. {return 1;}
    let pixels=u64::from(size[0])*u64::from(size[1]);
    if solid&&budget.is_some_and(|max|pixels<=u64::from(max)) {4}else{1}
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzl-choose-bounded-frame-quality-02.rs" skip=675
```

### 3. `src/lib.rs`

Register the one shared sampling policy used by the browser.

**Locate this existing block:**

```rust
pub mod renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod sample_quality;
pub mod renderer;
```

### 4. `src/renderer.rs`

Automatic quality follows actual active faces and filled world lettering rather than drawing ribbons or hidden sources.

**Locate this existing block:**

```rust
    pub fn samples(&self)->u32{self.targets.samples}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn has_solids(&self)->bool {
        self.meshes.iter().any(|mesh|mesh.geometry.index_count>0)||self.text.borrow().as_ref().is_some_and(|lane|lane.document.runs.iter().any(|run|!matches!(run.label.placement,crate::text::TextPlacement::Screen{..})))
    }
    pub fn samples(&self)->u32{self.targets.samples}
```

### 5. `src/browser.rs`

Read the adapter budget and optional unsigned ?msaa= override once, retaining the same override owner used by diagnostics. The shared policy gives reduced recovery precedence. Apply the same automatic policy on the initial viewport. Apply actual geometry, density and recovery policy after scene/input/extent changes. Pass the retained device budget alongside the optional override. Use actual physical pixels per CSS width. Reduced recovery wins even over an explicit four-sample override.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    renderer.set_samples(sample_forced.unwrap_or(1));
    editor.camera.aspect = size.aspect();
    true
}
```

Replace it with:

```rust
        surface.configure(&renderer.device, config);
        renderer.resize(size);
    }
    let samples=crate::sample_quality::samples(renderer.has_solids(),[size.width,size.height],sample_forced,sample_budget,size.width as f32/rect.width().max(1.)as f32,crate::browser_recovery::reduced());
    renderer.set_samples(samples);
    editor.camera.aspect = size.aspect();
    true
}
```

**Locate this block:**

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_forced:Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
```

Replace it with:

```rust
    config: &mut wgpu::SurfaceConfiguration,
    renderer: &mut Renderer,
    editor: &mut Editor,
    sample_budget:Option<u32>,sample_forced:Option<u32>,
) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let limit = renderer.device.limits().max_texture_dimension_2d;
```

**Locate this block:**

```rust
            &mut config,
            &mut renderer,
            &mut editor,
            sample_forced,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
```

Replace it with:

```rust
            &mut config,
            &mut renderer,
            &mut editor,
            sample_budget,sample_forced,
        ) {
            if let Err(error) = panel.update(None, &input_canvas) {
                report(&format!("Cannot lay out commands: {error:?}"));
```

**Locate this block:**

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_forced,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&mut editor)?;
```

Replace it with:

```rust
        &mut config,
        &mut renderer,
        &mut editor,
        sample_budget,sample_forced,
    ) {
        panel.update(None,&canvas)?;
        crate::browser_rebase::prepare(&mut renderer,&mut editor)?;
```

**Locate this block:**

```rust
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let mut sample_forced=None;
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

Replace it with:

```rust
    if let Some(info) = probe.as_ref().and_then(|probe| probe.info()) { let _ = crate::browser_report::adapter(info); }
    else { let _ = crate::browser_report::observe("diagnostic", "Adapter identity unavailable"); }
    drop(probe);
    let sample_budget=crate::sample_quality::budget(adapter.get_info().device_type);
    let mut sample_forced=window.location().search().ok().and_then(|s|web_sys::UrlSearchParams::new_with_str(&s).ok()).and_then(|q|q.get("msaa")).and_then(|v|v.parse::<u32>().ok());
    let device_started = crate::browser_phase::now();
    let device = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
```

### 6. `src/browser_recovery.rs`

Query the existing recovery owner rather than introducing another policy flag.

**Locate this existing block:**

```rust
pub fn density(value: f64) -> f64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn reduced()->bool{POLICY.with(|policy|policy.borrow().reduced())}

pub fn density(value: f64) -> f64 {
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/sample_surface_tests.rs`

Supplied actual renderer geometry/visibility/world-lettering eligibility regression. Original sheets remain source drawing ribbons. Validate actual isolated source vertices and visible/hidden original surface, CAD and plane families without adding speculative GPU classification.

**Locate this existing block:**

```text
),usage);crate::cap_gpu_tests::wait(&r);assert_eq!(r.take_pick(matrix).unwrap().unwrap().unwrap()[0],owner.value());
}
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzl-choose-bounded-frame-quality-11.rs" skip=120
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/sample_positions_tests.rs`

Supplied independent actual GPU observation of the implementation sample coordinates. Retain all coordinate/coverage/owner assertions without prescribing a fixed hardware pattern.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzl-choose-bounded-frame-quality-12.rs"
```

**Supplied validation fixture:**

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/lib.rs`

Register the optional actual GPU sample-coordinate observer.

**Locate this existing block:**

```text
pub mod sample_quality;
```

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzzl-choose-bounded-frame-quality-13.rs" skip=23
```

### 10. `src/browser.rs`

Accept the same unsigned override domain as the URL parser; leave negative, fractional and out-of-range diagnostic values unchanged.

**Locate this existing block:**

```rust
                else if let Some(count)=value.as_f64().filter(|v|*v==1.||*v==4.){sample_forced=Some(count as u32);}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                else if let Some(count)=value.as_f64().filter(|v| *v >= 0. && *v <= u32::MAX as f64 && v.fract() == 0.){sample_forced=Some(count as u32);}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.5
npm --prefix ../session_tests run course -- save 22.5
```

Continue with [22.6 · Original display controls and remaining integration boundaries](../../consolidation/96zzzzzzzm-retain-visible-instance-ranges.md).
