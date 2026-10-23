# 18.5 · Glass, X-ray and shared visibility rules

**Goal:** Prove actual glass x-ray and source ownership. · **Steps:** 9 · **Source:** [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs) (`src/shaders/hzb.wgsl,src/shaders/ssao.wgsl,src/app/wood_elements.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 18.5 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/caps_gpu.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/gpu_mesh.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/renderer.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">9 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/hzb.wgsl</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/ssao.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Classify solids independe…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. HZB Mip Pyramid &amp; SSAO</text>
  <rect x="606" y="54" width="84" height="84" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="648" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mip 0</text>
  <rect x="702" y="68" width="52" height="52" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="728" y="98" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Mip 1</text>
  <rect x="764" y="82" width="28" height="28" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">Glass, X-ray and shared vis…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Timber WoodElement &amp; Joints</text>
  <polygon points="888,122 924,62 1054,62 1018,122" fill="#fed7aa" stroke="#111111" stroke-width="2"/>
  <rect x="954" y="62" width="34" height="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="971" y1="46" x2="971" y2="76" stroke="#16a34a" stroke-width="2.25" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="976" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Mortise/Tenon + Dir</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Prove actual glass x-ray…</text>
</svg>
</div>

## <a id="checkpoint-93-classify-original-solids-independently-of-clipping"></a>1. Classify solids independently of clipping

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/caps_gpu.rs, src/gpu_me…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Classify original solids independently of clipping.

### 1. `src/caps_gpu.rs`

Reuse original closedness for surfaces while keeping section tokens lazy.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .filter(|r| scene.visible(r.id)),
        ) {
            assert_eq!(mesh.id, object.id);
            let flags = if self.verified {
                self.solids.flags(object)
            } else {
                0
            };
            let token = if flags & CLOSED != 0 {
                self.rows
```

Replace it with:

```rust
                .filter(|r| scene.visible(r.id)),
        ) {
            assert_eq!(mesh.id, object.id);
            let flags = self.solids.flags(object);
            let token = if !planes.is_empty() && flags & CLOSED != 0 {
                self.rows
```

**Locate this block:**

```rust
        planes: &[crate::clipping_rows::Row],
    ) {
        self.planes = planes.len() as u32;
        self.verified |= !planes.is_empty();
        self.rows.clear();
        self.targets.clear();
        for (mesh, object) in meshes.iter_mut().zip(
```

Replace it with:

```rust
        planes: &[crate::clipping_rows::Row],
    ) {
        self.planes = planes.len() as u32;
        self.rows.clear();
        self.targets.clear();
        for (mesh, object) in meshes.iter_mut().zip(
```

**Locate this block:**

```rust
            bounds: Default::default(),
            targets: Default::default(),
            solids: Default::default(),
            verified: false,
            pipes: None,
            counts: None,
            depth: Image::new(
```

Replace it with:

```rust
            bounds: Default::default(),
            targets: Default::default(),
            solids: Default::default(),
            pipes: None,
            counts: None,
            depth: Image::new(
```

**Locate this block:**

```rust
    pub solids: crate::solid_cache::Solids,
    verified: bool,
    pipes: Option<crate::cap_pipeline::Pipelines>,
    counts: Option<Counts>,
    pub depth: Image,
```

Replace it with:

```rust
    pub solids: crate::solid_cache::Solids,
    pipes: Option<crate::cap_pipeline::Pipelines>,
    counts: Option<Counts>,
    pub depth: Image,
```

### 2. `src/gpu_mesh.rs`

Own explicit derived x-ray flags without changing original geometry.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let next = settings(object, style, self.cap_flags, self.cap_token);
        let mut writes = [0, 0];
```

Replace it with:

```rust
        object: &Object,
        style: Style,
    ) -> [usize; 2] {
        let next = settings(
            object,
            style,
            self.cap_flags
                | if self.xray && self.cap_flags & 8192 != 0 {
                    32768
                } else {
                    0
                },
            self.cap_token,
        );
        let mut writes = [0, 0];
```

**Locate this block:**

```rust
        }
        let plane =
            u32::from_ne_bytes(self.previous[72..76].try_into().unwrap()) & 4096;
        self.previous[72..76].copy_from_slice(&(plane | flags).to_ne_bytes());
        self.previous[76..80].copy_from_slice(&token.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..80]);
        self.cap_flags = flags;
```

Replace it with:

```rust
        }
        let plane =
            u32::from_ne_bytes(self.previous[72..76].try_into().unwrap()) & 4096;
        self.previous[72..76].copy_from_slice(
            &(plane
                | flags
                | if self.xray && flags & 8192 != 0 {
                    32768
                } else {
                    0
                })
            .to_ne_bytes(),
        );
        self.previous[76..80].copy_from_slice(&token.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..80]);
        self.cap_flags = flags;
```

**Locate this block:**

```rust
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) != 0
    }

    pub fn closed(&self) -> bool {
        self.cap_flags & crate::solid_orientation::CLOSED != 0
    }
    pub fn pickable(&self) -> bool {
        u32::from_ne_bytes(self.previous[68..72].try_into().unwrap()) != 0
```

Replace it with:

```rust
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) != 0
    }

    pub fn set_xray(&mut self, queue: &wgpu::Queue, on: bool) {
        if self.xray == on {
            return;
        }
        self.xray = on;
        let flags = u32::from_ne_bytes(self.previous[72..76].try_into().unwrap());
        let flags = (flags & !32768)
            | if on && flags & 8192 != 0 {
                32768
            } else {
                0
            };
        self.previous[72..76].copy_from_slice(&flags.to_ne_bytes());
        queue.write_buffer(&self.model, 72, &self.previous[72..76]);
    }
    pub fn closed(&self) -> bool {
        !self.xray && self.cap_flags & crate::solid_orientation::CLOSED != 0
    }
    pub fn pickable(&self) -> bool {
        u32::from_ne_bytes(self.previous[68..72].try_into().unwrap()) != 0
```

**Locate this block:**

```rust
            id: object.id,
            model,
            previous: bytes,
            cap_flags: 0,
            cap_token: 0,
        }
```

Replace it with:

```rust
            id: object.id,
            model,
            previous: bytes,
            xray: false,
            cap_flags: 0,
            cap_token: 0,
        }
```

**Locate this block:**

```rust
    previous: [u8; 160],
    cap_flags: u32,
    cap_token: u32,
}
```

Replace it with:

```rust
    previous: [u8; 160],
    xray: bool,
    cap_flags: u32,
    cap_token: u32,
}
```

## <a id="checkpoint-93a-own-opacity-in-the-existing-surface-frame"></a>2. Own opacity in the existing surface frame

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Own opacity in the existing surface frame.

### 1. `src/renderer.rs`

Own opaque-default presentation alpha in the existing surface frame.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self
                .meshes
                .iter()
                .filter(|row| !self.instance_rows.contains(&row.id))
            {
                mesh.draw(&mut pass);
            }
            self.instances.draw(&mut pass);
            pass.set_pipeline(&self.highlight_pipeline);
```

Replace it with:

```rust
            pass.set_bind_group(2, &clip_group, &[]);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            for mesh in self.meshes.iter().filter(|row| {
                self.opacity.get() < 1. || !self.instance_rows.contains(&row.id)
            }) {
                mesh.draw(&mut pass);
            }
            if self.opacity.get() >= 1. {
                self.instances.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline);
```

**Locate this block:**

```rust
            } else {
                0.0
            },
            0.0,
            0.0,
            0.0,
        ];
        bytes.extend(
            display
```

Replace it with:

```rust
            } else {
                0.0
            },
            self.opacity.get(),
            0.0,
            0.0,
        ];
        bytes.extend(
            display
```

**Locate this block:**

```rust
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        self.tiles
            .borrow_mut()
            .set(&self.device, &self.meshes);
```

Replace it with:

```rust
            &mut self.meshes,
            &self.clipping.borrow().rows,
        );
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, self.opacity.get() == 0.);
        }
        self.tiles
            .borrow_mut()
            .set(&self.device, &self.meshes);
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            ambient_enabled: Default::default(),
            ambient: Default::default(),
            ambient_rows: Default::default(),
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            opacity: std::cell::Cell::new(1.),
            ambient_enabled: Default::default(),
            ambient: Default::default(),
            ambient_rows: Default::default(),
```

**Locate this block:**

```rust
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
```

Replace it with:

```rust
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState{format,blend:Some(wgpu::BlendState::ALPHA_BLENDING),write_mask:wgpu::ColorWrites::ALL})],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
```

**Locate this block:**

```rust
pub struct Renderer {
    ambient_enabled: std::cell::Cell<bool>,
    ambient: std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows: crate::ambient_rows::Rows,
```

Replace it with:

```rust
pub struct Renderer {
    opacity: std::cell::Cell<f32>,
    ambient_enabled: std::cell::Cell<bool>,
    ambient: std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows: crate::ambient_rows::Rows,
```

## <a id="checkpoint-93b-blend-closed-faces-and-retain-opaque-open-surfaces"></a>3. Blend closed faces and retain opaque open surfaces

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/triangle.wgsl</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Blend closed faces and retain opaque open surfaces.

### 1. `src/triangle.wgsl`

Adapt current closed/open alpha and mirrored-front behavior to the earlier source shader.

**Locate this existing block:**

```wgsl
    @location(3) @interpolate(flat) clip_flags:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), 1.0); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, 1.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}
```

## <a id="checkpoint-93c-exclude-x-ray-faces-from-original-picks-and-masks"></a>4. Exclude x-ray faces from picks and masks

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/triangle_pick.wgsl, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Exclude x-ray faces from original picks and masks.

### 1. `src/triangle_pick.wgsl`

Apply one derived x-ray face predicate without source mutation.

**Locate this existing block:**

```wgsl
@vertex fn pick_vertex(@location(0) position:vec3<f32>,@location(1) colour:vec3<f32>,@location(2) normal:vec3<f32>)->MeshPickOutput{let placed=geometry(position,colour,normal);return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags);}
@fragment fn pick_fragment(input:MeshPickOutput)->@location(0) vec2<u32>{if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec2<u32>(select(0u,object.selection.y,crop.mode==0u),0u);}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex fn pick_vertex(@location(0) position:vec3<f32>,@location(1) colour:vec3<f32>,@location(2) normal:vec3<f32>)->MeshPickOutput{let placed=geometry(position,colour,normal);return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags);}
@fragment fn pick_fragment(input:MeshPickOutput)->@location(0) vec2<u32>{if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec2<u32>(select(0u,object.selection.y,crop.mode==0u),0u);}
```

### 2. `src/face_pick.wgsl`

Apply one derived x-ray face predicate without source mutation.

**Locate this existing block:**

```wgsl
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object.selection.y, (0x20000000u | input.face) + 1u);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object.selection.y, (0x20000000u | input.face) + 1u);
```

### 3. `src/outline_mask.wgsl`

Apply one derived x-ray face predicate without source mutation.

**Locate this existing block:**

```wgsl
@vertex fn vertex(@location(0) position:vec3<f32>)->Output{let world=object.model*vec4<f32>(position,1.0);return Output(view.transform*world,world.xyz,object.selection.z);}
@fragment fn fragment(input:Output)->@location(0) vec4<f32>{if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec4<f32>(1.0);}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@vertex fn vertex(@location(0) position:vec3<f32>)->Output{let world=object.model*vec4<f32>(position,1.0);return Output(view.transform*world,world.xyz,object.selection.z);}
@fragment fn fragment(input:Output)->@location(0) vec4<f32>{if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec4<f32>(1.0);}
```

### 4. `src/ambient_sources.wgsl`

Apply one derived x-ray face predicate without source mutation.

**Locate this existing block:**

```wgsl
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
```

## <a id="checkpoint-93d-carry-opacity-into-finite-original-ink"></a>5. Carry opacity into finite ink

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/stroke_gpu.rs, src/high…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Carry opacity into finite original ink.

### 1. `src/stroke_gpu.rs`

Carry view opacity through compatible finite-ink frames and alpha helpers.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }],
        });
        Self {
            flags: None,
```

Replace it with:

```rust
            }],
        });
        Self {
            opacity: std::cell::Cell::new(1.),
            flags: None,
```

**Locate this block:**

```rust
        let bytes: Vec<u8> = transform
            .iter()
            .copied()
            .chain([size[0] as f32, size[1] as f32, density, 0.0])
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
```

Replace it with:

```rust
        let bytes: Vec<u8> = transform
            .iter()
            .copied()
            .chain([
                size[0] as f32,
                size[1] as f32,
                density,
                self.opacity.get(),
            ])
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
```

**Locate this block:**

```rust
        Ok(())
    }

    pub fn view(
        &self,
        queue: &wgpu::Queue,
```

Replace it with:

```rust
        Ok(())
    }

    pub fn set_opacity(&self, value: f32) {
        self.opacity.set(value);
    }
    pub fn view(
        &self,
        queue: &wgpu::Queue,
```

**Locate this block:**

```rust
pub struct Lane {
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
    finite: Option<crate::finite_ink::Ink>,
```

Replace it with:

```rust
pub struct Lane {
    opacity: std::cell::Cell<f32>,
    flags: Option<wgpu::Buffer>,
    flag_data: Vec<u32>,
    finite: Option<crate::finite_ink::Ink>,
```

### 2. `src/highlight.rs`

Carry view opacity through compatible finite-ink frames and alpha helpers.

**Locate this existing block:**

```rust
impl Highlight {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Highlight {
    pub fn set_opacity(&self, value: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.set_opacity(value);
        }
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

### 3. `src/finite_visibility.wgsl`

Carry view opacity through compatible finite-ink frames and alpha helpers.

**Locate this existing block:**

```wgsl
 return true;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);let glass=view.opacity>0.&&view.opacity<1.;return select(1.,select(0.,1.-view.opacity,glass),hidden&&view.opacity>0.);
}
```

### 4. `src/finite_stroke.wgsl`

Name the opacity scalar in the existing compatible lane frame.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
```

### 5. `src/finite_chain.wgsl`

Name the opacity scalar in the existing compatible lane frame.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
```

### 6. `src/finite_marker.wgsl`

Name the opacity scalar in the existing compatible lane frame.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
```

## <a id="checkpoint-93e-fade-hidden-ink-without-showing-clipped-geometry"></a>6. Fade hidden ink without showing clipped geometry

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 6 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/finite_stroke.wgsl, src…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Fade hidden ink without showing clipped geometry.

### 1. `src/finite_stroke.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 2. `src/finite_chain.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 3. `src/finite_marker.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 4. `src/finite_stroke_pick.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

### 5. `src/finite_chain_pick.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

### 6. `src/finite_marker_pick.wgsl`

Use the same original ink visibility alpha for color and integer picks.

**Locate this existing block:**

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (colour(source).a <= 0.0) { discard; }
    if(finite_alpha(input.finite_pixel,input.finite_axis_value,input.finite_depths,input.clip_flags)<=0.){discard;}
    return input.id;
```

## <a id="checkpoint-93f-invalidate-presentation-and-preserve-cached-geometry"></a>7. Invalidate presentation and preserve cached geometry

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 7 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Invalidate presentation and preserve cached geometry.

### 1. `src/renderer.rs`

Connect opacity changes to compatible original source flags and presentation caches.

**Locate this existing block:**

```rust
    }
    pub fn set_outlines(&self, on: bool) {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn set_opacity(&mut self, value: f32) {
        if !value.is_finite() {
            return;
        }
        let value = value.clamp(0., 1.);
        if self.opacity.get() == value {
            return;
        }
        self.opacity.set(value);
        self.picker.cancel();
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        for mesh in &mut self.meshes {
            mesh.set_xray(&self.queue, value == 0.);
        }
        for lane in [
            &self.mesh_ink,
            &self.strokes,
            &self.paths,
            &self.markers,
            &self.grid,
        ] {
            lane.set_opacity(value);
        }
        self.highlight.set_opacity(value);
        if let Some(o) = self.outline.borrow_mut().as_mut() {
            o.invalidate();
        }
        if let Some(o) = self.global_outline.borrow_mut().as_mut() {
            o.invalidate();
        }
    }
    pub fn opacity(&self) -> f32 {
        self.opacity.get()
    }
    pub fn set_outlines(&self, on: bool) {
```

## <a id="checkpoint-93g-connect-strict-opacity-presentation-commands"></a>8. Connect strict Opacity presentation commands

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 8 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/editor.rs, src/commands…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Connect strict Opacity presentation commands.

### 1. `src/editor.rs`

Connect strict presentation alpha and truthful browser state.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    }
                );
                return Ok(Change::Scene);
            }
```

Replace it with:

```rust
                    }
                );
                return Ok(Change::Scene);
            }
            Action::Opacity(_) => {
                unreachable!("Opacity presentation returned before document mutation")
            }
```

**Locate this block:**

```rust
            }
            action => action,
        };
        if let Action::ViewOutline(on) = &action {
            self.outlines = on.unwrap_or(!self.outlines);
            self.tool_status = format!(
```

Replace it with:

```rust
            }
            action => action,
        };
        if let Action::Opacity(value) = &action {
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err("Opacity takes a value from0 to1");
            }
            self.opacity = *value;
            self.tool_status = format!("Opacity {value}");
            return Ok(Change::View);
        }
        if let Action::ViewOutline(on) = &action {
            self.outlines = on.unwrap_or(!self.outlines);
            self.tool_status = format!(
```

**Locate this block:**

```rust
            element: None,
            element_attributes: false,
            element_interactions: true,
            arctic: false,
            outlines: false,
            clipping_enabled: true,
```

Replace it with:

```rust
            element: None,
            element_attributes: false,
            element_interactions: true,
            opacity: 1.,
            arctic: false,
            outlines: false,
            clipping_enabled: true,
```

**Locate this block:**

```rust
    pub element: Option<crate::element_draft::Draft>,
    pub element_attributes: bool,
    pub element_interactions: bool,
    pub arctic: bool,
    pub outlines: bool,
    pub clipping_enabled: bool,
```

Replace it with:

```rust
    pub element: Option<crate::element_draft::Draft>,
    pub element_attributes: bool,
    pub element_interactions: bool,
    pub opacity: f32,
    pub arctic: bool,
    pub outlines: bool,
    pub clipping_enabled: bool,
```

**Locate this block:**

```rust
    ElementInteractions(Option<bool>),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
```

Replace it with:

```rust
    ElementInteractions(Option<bool>),
    Opacity(f32),
    Arctic(Option<bool>),
    ViewOutline(Option<bool>),
    Clipping(crate::clipping_input::Request),
```

### 2. `src/commands.rs`

Connect strict presentation alpha and truthful browser state.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            },
        ));
    }
```

Replace it with:

```rust
            },
        ));
    }
    if name == "Opacity" {
        let [word] = &words[count..] else {
            return Err("Opacity 0..1 · Example: Opacity 0.5".into());
        };
        let value = word
            .parse::<f32>()
            .ok()
            .filter(|v| v.is_finite() && (0.0..=1.0).contains(v))
            .ok_or("Opacity takes a value from 0 to 1")?;
        return Ok(Command::Editor(Action::Opacity(value)));
    }
```

**Locate this block:**

```rust
    }),
    ("View Outline", || Action::ViewOutline(None)),
    ("Arctic", || Action::Arctic(None)),
    ("Select By Name", || {
```

Replace it with:

```rust
    }),
    ("Opacity", || Action::Opacity(1.)),
    ("View Outline", || Action::ViewOutline(None)),
    ("Arctic", || Action::Arctic(None)),
    ("Select By Name", || {
```

### 3. `src/browser.rs`

Connect strict presentation alpha and truthful browser state.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

Replace it with:

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"opacity":editor.opacity,"outlines":editor.outlines,"global_outline":renderer.global_outline_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
            );
            renderer.set_ambient(editor.arctic);
            renderer.set_outlines(editor.outlines);
            if let Err(error) =
```

Replace it with:

```rust
            );
            renderer.set_opacity(editor.opacity);
            renderer.set_ambient(editor.arctic);
            renderer.set_outlines(editor.outlines);
            if let Err(error) =
```

## <a id="checkpoint-93h-prove-actual-glass-x-ray-and-source-ownership"></a>9. Prove actual glass x-ray and source ownership

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 9 of 9</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/opacity_gpu_tests.rs, s…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original closedness</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">face alpha and hidden ink</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/shaders/hzb.wgsl</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original x-ray picks</text>
</svg>
</div>

**What this code does:** Prove actual glass x-ray and source ownership.

### 1. `src/opacity_gpu_tests.rs`

Copy supplied actual alpha, hidden ink, original pick and refresh fixtures.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{at, matrix, pixels, renderer, wait},
    scene::Scene,
};
use std::rc::Rc;
fn pick(r: &mut super::Renderer, at: [f32; 2]) -> Option<[u32; 2]> {
    r.request_pick(at, matrix()).unwrap();
    wait(r);
    r.take_pick(matrix())
        .expect("mapped original opacity pick")
        .unwrap()
}
#[test]
fn actual_closed_faces_blend_hidden_ink_fades_and_xray_picks_original_ink() {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false))
                .unwrap(),
        )
        .unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut source = session_rust::Line::new(-0.4, 0., -0.9, 0.4, 0., -0.9);
    source.linecolor = session_rust::Color::new(0., 1., 0., 1.);
    source.width = 8.;
    let line = scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(source)).unwrap())
        .unwrap();
    let mut r = renderer(&scene);
    let solid = pixels(&r);
    let uploads = r.stats()[0];
    assert!(at(&solid, 160, 120)[0] > 100 && at(&solid, 160, 120)[1] < 20);
    assert_eq!(
        pick(&mut r, [0., 0.]).unwrap()[0],
        owner.value()
    );
    r.set_opacity(0.5);
    let glass = pixels(&r);
    let center = at(&glass, 160, 120);
    assert!(
        center[1] > 100,
        "hidden original line fades through glass: {center:?}"
    );
    let face = at(&glass, 160, 90);
    assert!(
        (120..=135).contains(&face[1]) && (120..=135).contains(&face[2]),
        "one blended face, without a second back face: {face:?}"
    );
    assert_eq!(
        pick(&mut r, [0., 0.]).unwrap()[0],
        line.value()
    );
    r.set_opacity(0.);
    let xray = pixels(&r);
    let center = at(&xray, 160, 120);
    assert!(
        center[1] > 240 && center[0] < 20,
        "xray original ink: {center:?}"
    );
    assert_eq!(&at(&xray, 160, 90)[..3], &[255, 255, 255]);
    assert_eq!(
        pick(&mut r, [0., 0.]).unwrap()[0],
        line.value()
    );
    assert_eq!(
        pick(&mut r, [0., 0.25]),
        None,
        "closed xray faces do not take an original face pick"
    );
    r.set_opacity(1.);
    assert_eq!(pixels(&r), solid);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(r.stats()[0], uploads);
}
#[test]
fn actual_open_surfaces_remain_opaque_and_mirrored_closed_faces_keep_glass() {
    let mut scene = Scene::empty();
    scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(true))
                .unwrap(),
        )
        .unwrap();
    let mut r = renderer(&scene);
    let solid = pixels(&r);
    r.set_opacity(0.5);
    assert_eq!(pixels(&r), solid);
    r.set_opacity(0.);
    assert_eq!(
        pixels(&r),
        solid,
        "open surfaces retain original opaque faces"
    );
    let mut scene = Scene::empty();
    let owner = scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false))
                .unwrap(),
        )
        .unwrap();
    let mut r = renderer(&scene);
    r.set_opacity(0.5);
    let plain = pixels(&r);
    scene
        .place_owner(
            owner,
            session_rust::Xform::scale_xyz(-1., 1., 1.),
        )
        .unwrap();
    r.set_scene(&scene, None);
    let mirror = pixels(&r);
    assert_eq!(
        at(&mirror, 160, 90),
        at(&plain, 160, 90),
        "mirrored original front face stays blended"
    );
}
#[test]
fn actual_xray_survives_scene_refresh_without_hiding_open_originals() {
    let mut scene = Scene::empty();
    scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(false))
                .unwrap(),
        )
        .unwrap();
    let mut r = renderer(&scene);
    r.set_opacity(0.);
    pixels(&r);
    r.set_scene(&scene, None);
    let xray = pixels(&r);
    assert_eq!(&at(&xray, 160, 90)[..3], &[255, 255, 255]);
    scene.clear();
    scene
        .insert(
            crate::prepared::PreparedMesh::new(crate::cap_gpu_tests::source(true))
                .unwrap(),
        )
        .unwrap();
    r.set_scene(&scene, None);
    let open = pixels(&r);
    assert!(
        at(&open, 160, 90)[0] > 100 && at(&open, 160, 90)[1] < 20,
        "a new open original stays opaque under xray"
    );
}
```

### 2. `src/renderer.rs`

Register actual renderer-local transparency fixtures.

**Locate this existing block:**

```rust
#[path = "global_outline_tests.rs"]
mod global_outline_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "global_outline_tests.rs"]
mod global_outline_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "opacity_gpu_tests.rs"]
mod opacity_gpu_tests;
```

### 3. `src/opacity_command_tests.rs`

Copy supplied strict presentation/pending-point/history fixtures.

Create the file and type:

```rust
use crate::{
    commands::{Command, parse},
    editor::{Action, Editor},
};
#[test]
fn opacity_strict_presentation_keeps_original_pending_points_and_redo() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddBox).unwrap();
    let id = e.scene.objects()[0].id;
    e.select_objects([id], false);
    let before = e.scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    e.apply(Action::Translate([1., 0., 0.]))
        .unwrap();
    let moved = e.scene.placement(id).unwrap();
    e.apply(Action::Undo).unwrap();
    e.apply(Action::BeginTool(crate::tool::Shape::Line))
        .unwrap();
    e.apply(Action::ToolPoint([0., 0., 0.]))
        .unwrap();
    let points = e.draft.as_ref().unwrap().points.clone();
    let epoch = e.interaction_epoch;
    for value in [0., 0.4, 0.7, 1.] {
        let Command::Editor(a) = parse(&format!("Opacity {value}")).unwrap() else {
            unreachable!()
        };
        e.apply(a).unwrap();
        assert_eq!(e.opacity, value);
        assert_eq!(e.draft.as_ref().unwrap().points, points);
        assert_eq!(e.interaction_epoch, epoch);
        assert_eq!(
            e.scene.objects()[0]
                .geometry()
                .unwrap()
                .to_proto(),
            before
        );
    }
    for text in [
        "Opacity",
        "Opacity -1",
        "Opacity 1.1",
        "Opacity NaN",
        "Opacity 0.5 extra",
    ] {
        assert!(parse(text).is_err());
    }
    assert!(e.apply(Action::Opacity(f32::NAN)).is_err());
    e.apply(Action::CancelTool).unwrap();
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, moved.m);
}
```

### 4. `src/lib.rs`

Register the completed transparency component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod element_tests;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod element_tests;

#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod opacity_command_tests;
pub mod source_mesh_topology;
```

## Build & Run

![Recorded visual output for Lesson 18.5 (93h-prove-actual-glass-x-ray-and-source-ownership)](../../screenshots/journey/93h-prove-actual-glass-x-ray-and-source-ownership.png)

```sh
npm --prefix ../session_tests run course -- dependencies 18.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.5
npm --prefix ../session_tests run course -- save 18.5
```

Continue with [18.6 · Matched rendering comparisons from original sources](94-declare-the-rendering-comparison-contract.md).
