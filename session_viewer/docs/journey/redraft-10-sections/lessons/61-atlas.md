# 10.3 · A shared glyph atlas and lazy text drawing

**Goal:** Draw the specimen and prove real coverage reuse. · **Steps:** 5 · **Source:** [engine/text.rs](../../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../../src/engine/gpu/text_plate.rs) · [app/scene_text.rs](../../../../src/app/scene_text.rs) · [app/fonts.rs](../../../../src/app/fonts.rs) (`src/app/fonts.rs,src/shaders/matrix.wgsl,src/engine/gpu/matrices.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">1. Lesson 10.3 Code</text>
  <rect x="34" y="48" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="64" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/text_atlas.rs</text>
  <rect x="34" y="78" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="94" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/lib.rs</text>
  <rect x="34" y="108" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.25"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="124" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/text_example.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">5 Coding Steps</text>
  <line x1="264" y1="100" x2="292" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="296" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">2. session_viewer/src</text>
  <rect x="314" y="48" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="65" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/app/fonts.rs</text>
  <rect x="314" y="80" width="212" height="26" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="97" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">src/shaders/matrix.wgsl</text>
  <rect x="314" y="112" width="212" height="24" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">✓ cargo check (wasm32)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="420" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" fill="#111111">Own glyph coverage in a s…</text>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">3. R8Unorm Glyph Atlas (B&amp;W)</text>
  <rect x="616" y="54" width="168" height="84" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="628" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="645" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">A</text>
  <rect x="670" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="687" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">B</text>
  <rect x="712" y="64" width="34" height="36" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.5"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="729" y="88" text-anchor="middle" font-family="monospace" font-size="15" font-weight="700" fill="#111111">7</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">1px Padded Shelves</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="700" y="172" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">A shared glyph atlas and la…</text>
  <line x1="824" y1="100" x2="852" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="856" y="14" width="248" height="172" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="34" text-anchor="middle" font-family="monospace" font-size="11.5" font-weight="700" fill="#111111">4. Procedural matrix.wgsl</text>
  <rect x="914" y="54" width="120" height="84" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="914" y="54" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <rect x="954" y="82" width="40" height="28" fill="#f97316"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="974" y="100" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+2.4</text>
  <rect x="994" y="54" width="40" height="28" fill="#93c5fd"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="72" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">-0.8</text>
  <rect x="994" y="110" width="40" height="28" fill="#fdba74"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1014" y="128" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">+1.0</text>
  <line x1="954" y1="54" x2="954" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="994" y1="54" x2="994" y2="138" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="82" x2="1034" y2="82" stroke="#111111" stroke-width="1"/>
  <line x1="914" y1="110" x2="1034" y2="110" stroke="#111111" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="980" y="172" text-anchor="middle" font-family="sans-serif" font-size="10" font-weight="700" fill="#111111">Draw the specimen and pro…</text>
</svg>
</div>

## <a id="checkpoint-61-atlas"></a>1. Own glyph coverage in a shared GPU atlas

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 1 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_atlas.rs, src/lib.…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">explicit shaped fonts</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">glyph coverage atlas</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">text renderer</text>
</svg>
</div>

**What this code does:** Own glyph coverage in a shared GPU atlas.

### 1. `src/text_atlas.rs`

Own the current accurate-colour atlas and cache with separate frame/source revisions.

Create the file and type:

```rust
use crate::text::{TextDocument, TextPlacement};
use glyphon::{
    Cache, Color, Resolution, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer, Viewport,
};

pub struct Lane {
    pub document: TextDocument,
    cache: Cache,
    atlas: TextAtlas,
    viewport: Viewport,
    raster: SwashCache,
    renderer: TextRenderer,
    prepared: Option<(u64, u64, [u32; 2], u32)>,
    pub preparations: u64,
    pub skips: u64,
    pub resets: u64,
    pub format: wgpu::TextureFormat,
}

impl Lane {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let cache = Cache::new(device);
        let atlas = TextAtlas::with_color_mode(
            device,
            queue,
            &cache,
            format,
            glyphon::ColorMode::Accurate,
        );
        let viewport = Viewport::new(device, &cache);
        let mut atlas = atlas;
        let renderer = TextRenderer::new(&mut atlas, device, Default::default(), None);
        Self {
            document: TextDocument::new(),
            cache,
            atlas,
            viewport,
            raster: SwashCache::new(),
            renderer,
            prepared: None,
            preparations: 0,
            skips: 0,
            resets: 0,
            format,
        }
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod cad_display_tests;

pub mod text;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cad_display_tests;

pub mod text;

pub mod text_atlas;
```

## <a id="checkpoint-61a-coverage"></a>2. Prepare DPI-scaled glyph bounds and cache unchanged frames

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 2 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_atlas.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">CSS placement and clipping</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">glyph preparation</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged-frame reuse</text>
</svg>
</div>

**What this code does:** Prepare DPI-scaled glyph bounds and cache unchanged frames.

### 1. `src/text_atlas.rs`

Prepare real glyph coverage, pixel clips, revision resets and truthful CPU raster-cache counts.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: [u32; 2],
        density: f32,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            size.iter().all(|v| *v > 0) && density.is_finite() && density > 0.,
            "Invalid text viewport"
        );
        let key = (
            self.document.revision,
            self.document.font_revision,
            size,
            density.to_bits(),
        );
        if self.prepared == Some(key) {
            self.skips += 1;
            return Ok(());
        }
        if self
            .prepared
            .is_some_and(|old| old.1 != key.1)
        {
            self.atlas = TextAtlas::with_color_mode(
                device,
                queue,
                &self.cache,
                self.format,
                glyphon::ColorMode::Accurate,
            );
            self.renderer = TextRenderer::new(
                &mut self.atlas,
                device,
                Default::default(),
                None,
            );
            self.raster = SwashCache::new();
            self.resets += 1;
        }
        self.viewport.update(
            queue,
            Resolution {
                width: size[0],
                height: size[1],
            },
        );
        let areas = self.document.runs.iter().filter_map(|run| {
            let TextPlacement::Screen { left, top } = run.label.placement else {
                return None;
            };
            let bounds = run.label.clip.map_or(
                [
                    0.,
                    0.,
                    size[0] as f32 / density,
                    size[1] as f32 / density,
                ],
                |clip| clip,
            );
            Some(TextArea {
                buffer: &run.buffer,
                left: left * density,
                top: top * density,
                scale: density,
                bounds: TextBounds {
                    left: (bounds[0] * density).floor() as i32,
                    top: (bounds[1] * density).floor() as i32,
                    right: (bounds[2] * density).ceil() as i32,
                    bottom: (bounds[3] * density).ceil() as i32,
                },
                default_color: Color::rgba(
                    run.label.ink_color()[0],
                    run.label.ink_color()[1],
                    run.label.ink_color()[2],
                    run.label.ink_color()[3],
                ),
                custom_glyphs: &[],
            })
        });
        self.renderer.prepare(
            device,
            queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            areas,
            &mut self.raster,
        )?;
        self.prepared = Some(key);
        self.preparations += 1;
        Ok(())
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<()> {
        self.renderer
            .render(&self.atlas, &self.viewport, pass)?;
        Ok(())
    }
    pub fn raster_usage(&self) -> [usize; 2] {
        self.raster
            .image_cache
            .values()
            .flatten()
            .fold([0, 0], |[n, bytes], image| {
                [n + 1, bytes + image.data.capacity()]
            })
    }
}
```

## <a id="checkpoint-61b-specimen"></a>3. Supply a real-font CAD text specimen

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 3 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/text_example.rs, src/li…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">original specimen words</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">fractional position</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">real glyph coverage</text>
</svg>
</div>

**What this code does:** Supply a real-font CAD text specimen.

### 1. `src/text_example.rs`

Provide a current-font specimen that exercises real glyph coverage.

Create the file and type:

```rust
pub fn label() -> crate::text::TextLabel {
    crate::text::TextLabel {
        id: 1,
        object: None,
        text: "Noto CAD · office ffi · Ė · 90° ± · ⌘".into(),
        font_size: 18.,
        line_height: 26.,
        color: [0, 0, 0, 255],
        placement: crate::text::TextPlacement::Screen {
            left: 16.25,
            top: 12.5,
        },
        clip: None,
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod text;

pub mod text_atlas;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod text;

pub mod text_atlas;

pub mod text_example;
```

## <a id="checkpoint-61c-renderer"></a>4. Draw lazy text coverage after the scene

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 4 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/renderer.rs</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">scene and handles</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">colour load pass</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged geometry and pi…</text>
</svg>
</div>

**What this code does:** Draw lazy text coverage after the scene.

### 1. `src/renderer.rs`

Draw a lazy glyph lane independently from primary geometry and expose preparation errors.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder, view);
        self.queue.submit([encoder.finish()]);
```

Replace it with:

```rust
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder, view);
        self.text_error.borrow_mut().take();
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            match lane.prepare(
                &self.device,
                &self.queue,
                self.size,
                self.density,
            ) {
                Ok(()) => {
                    let mut pass =
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("screen glyphs"),
                            color_attachments: &[Some(
                                wgpu::RenderPassColorAttachment {
                                    view,
                                    depth_slice: None,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: wgpu::StoreOp::Store,
                                    },
                                },
                            )],
                            ..Default::default()
                        });
                    if let Err(error) = lane.draw(&mut pass) {
                        *self.text_error.borrow_mut() = Some(error.to_string());
                    }
                }
                Err(error) => {
                    *self.text_error.borrow_mut() = Some(error.to_string());
                }
            }
        }
        self.queue.submit([encoder.finish()]);
```

**Locate this block:**

```rust
        self.widget.extent()
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

Replace it with:

```rust
        self.widget.extent()
    }

    pub fn set_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        let lane = lane.get_or_insert_with(|| {
            crate::text_atlas::Lane::new(&self.device, &self.queue, self.format)
        });
        lane.document.set_labels(labels)
    }
    pub fn text_error(&self) -> Option<String> {
        self.text_error.borrow().clone()
    }
    pub fn text_usage(&self) -> [u64; 6] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                let [rasters, bytes] = lane.raster_usage();
                [
                    lane.document.shape_count,
                    lane.preparations,
                    lane.skips,
                    rasters as u64,
                    bytes as u64,
                    lane.resets,
                ]
            })
    }
    pub fn text_demo(&self) -> anyhow::Result<()> {
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

**Locate this block:**

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            device,
            queue,
            pipeline,
```

Replace it with:

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
            device,
            queue,
            pipeline,
```

**Locate this block:**

```rust
pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
```

Replace it with:

```rust
pub struct Renderer {
    text: std::cell::RefCell<Option<crate::text_atlas::Lane>>,
    text_error: std::cell::RefCell<Option<String>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
```

## <a id="checkpoint-61d-glyphs"></a>5. Draw the specimen and prove real coverage reuse

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Step 5 of 5</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="140" y="48" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">src/commands.rs, src/browse…</text>
  <line x1="266" y1="36" x2="292" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="296" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1. CPU / Rust Input</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="422" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">Example Text</text>
  <line x1="548" y1="36" x2="574" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="578" y="10" width="252" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2. WebGPU / WGSL</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="704" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged shaping</text>
  <line x1="830" y1="36" x2="856" y2="36" stroke="#111111" stroke-width="1.75" marker-end="url(#arr)"/>
  <rect x="860" y="10" width="246" height="52" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="30" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3. -&gt; src/app/fonts.rs</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="983" y="48" text-anchor="middle" font-family="sans-serif" font-size="9.5" fill="#111111">unchanged original scene</text>
</svg>
</div>

**What this code does:** Draw the specimen and prove real coverage reuse.

### 1. `src/commands.rs`

Connect the presentation specimen, current text diagnostics and explicit Close cleanup.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Report", Host::Report),
```

Replace it with:

```rust
    ("Reload Sources", Host::Reload),
    ("Cancel Reload", Host::CancelReload),
    ("Save", Host::Save),
    ("Example Text", Host::TextDemo),
    ("Report", Host::Report),
```

**Locate this block:**

```rust
pub enum Host {
    Help,
    Open,
    Replace,
```

Replace it with:

```rust
pub enum Host {
    TextDemo,
    Help,
    Open,
    Replace,
```

### 2. `src/browser.rs`

Connect the presentation specimen, current text diagnostics and explicit Close cleanup.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "data-point-gpu",
        &serde_json::to_string(&renderer.point_usage()).unwrap(),
    )?;
```

Replace it with:

```rust
        "data-point-gpu",
        &serde_json::to_string(&renderer.point_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-text-error",
        &renderer.text_error().unwrap_or_default(),
    )?;
    canvas.set_attribute(
        "data-text-atlas",
        &serde_json::to_string(&renderer.text_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
                    Ok(crate::commands::Command::Host(host)) => {
                        use crate::commands::Host;
                        match host {
                            Host::Help => {
                                panel.result(&crate::commands::names().join(" · "));
                                None
```

Replace it with:

```rust
                    Ok(crate::commands::Command::Host(host)) => {
                        use crate::commands::Host;
                        match host {
                            Host::TextDemo => {
                                if let Err(error) = renderer.text_demo() {
                                    panel.result(&error.to_string());
                                }
                                None
                            }
                            Host::Help => {
                                panel.result(&crate::commands::names().join(" · "));
                                None
```

**Locate this block:**

```rust
                            request.borrow_mut().cancel();
                        }
                        if shortcut.is_some()
                            && editor.selected.is_some()
```

Replace it with:

```rust
                            request.borrow_mut().cancel();
                            let _ = renderer.set_text(Vec::new());
                        }
                        if shortcut.is_some()
                            && editor.selected.is_some()
```

### 3. `src/text_atlas_tests.rs`

Copy actual anti-aliased coverage, CSS density, clipping, placement reuse and accepted/failed font replacement GPU checks.

Create the file and type:

```rust
use crate::text_atlas::Lane;

fn device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    pollster::block_on(adapter.request_device(&Default::default())).unwrap()
}

fn pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    lane: &Lane,
    size: [u32; 2],
) -> Vec<u8> {
    let [width, height] = size;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("actual glyph coverage"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let stride = width * 4;
    assert_eq!(stride % 256, 0);
    let read = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("glyph pixels"),
        size: stride as u64 * height as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("text coverage"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        lane.draw(&mut pass).unwrap();
    }
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(stride),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap()
        });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range().to_vec();
    read.unmap();
    bytes
}

#[test]
fn actual_glyph_atlas_has_antialiased_coverage_and_preserves_css_size_at_two_densities()
{
    let (device, queue) = device();
    let mut lane = Lane::new(
        &device,
        &queue,
        wgpu::TextureFormat::Rgba8Unorm,
    );
    lane.document
        .set_labels(vec![crate::text_example::label()])
        .unwrap();
    let mut bounds = Vec::new();
    for density in [1., 2.] {
        let size = [
            (320. * density) as u32,
            (240. * density) as u32,
        ];
        lane.prepare(&device, &queue, size, density)
            .unwrap();
        let bytes = pixels(&device, &queue, &lane, size);
        let mut box_ = [f32::INFINITY, f32::INFINITY, 0., 0.];
        let (mut ink, mut coverage) = (0, 0);
        for (i, p) in bytes.chunks_exact(4).enumerate() {
            if p[0] < 240 {
                let x = (i % size[0] as usize) as f32 / density;
                let y = (i / size[0] as usize) as f32 / density;
                box_[0] = box_[0].min(x);
                box_[1] = box_[1].min(y);
                box_[2] = box_[2].max(x);
                box_[3] = box_[3].max(y);
                ink += 1;
            }
            if p[0] > 0 && p[0] < 240 {
                coverage += 1;
            }
        }
        assert!(
            ink > 200 && coverage > 50,
            "Actual glyph ink and fractional coverage must render"
        );
        bounds.push(box_);
        lane.prepare(&device, &queue, size, density)
            .unwrap();
        assert_eq!(pixels(&device, &queue, &lane, size), bytes);
    }
    assert!(
        bounds[0]
            .iter()
            .zip(bounds[1])
            .all(|(a, b)| (a - b).abs() < 1.5)
    );
    assert_eq!(lane.document.shape_count, 1);
    assert_eq!(lane.preparations, 2);
    assert_eq!(lane.skips, 2);
    assert_eq!(
        lane.raster_usage(),
        [0, 0],
        "Glyphon 0.11 uploads temporary raster images without retaining them in the CPU image cache"
    );
}

#[test]
fn actual_glyph_clip_and_font_reload_preserve_source_words_and_reset_only_accepted_fonts()
 {
    let (device, queue) = device();
    let mut lane = Lane::new(
        &device,
        &queue,
        wgpu::TextureFormat::Rgba8Unorm,
    );
    let mut label = crate::text_example::label();
    label.clip = Some([16., 12., 120., 42.]);
    let words = label.text.clone();
    lane.document
        .set_labels(vec![label.clone()])
        .unwrap();
    lane.prepare(&device, &queue, [320, 240], 1.)
        .unwrap();
    let before = pixels(&device, &queue, &lane, [320, 240]);
    for (i, p) in before.chunks_exact(4).enumerate() {
        let (x, y) = (i % 320, i / 320);
        if x < 16 || x >= 120 || y < 12 || y >= 42 {
            assert_eq!(p, [255; 4]);
        }
    }
    assert!(
        before
            .chunks_exact(4)
            .filter(|p| p[0] < 240)
            .count()
            > 100
    );
    label.placement = crate::text::TextPlacement::Screen {
        left: 17.,
        top: 13.,
    };
    lane.document
        .set_labels(vec![label])
        .unwrap();
    assert_eq!(lane.document.shape_count, 1);
    lane.prepare(&device, &queue, [320, 240], 1.)
        .unwrap();
    let moved = pixels(&device, &queue, &lane, [320, 240]);
    assert_ne!(moved, before);
    let rasters = lane.raster_usage();
    assert!(
        lane.document
            .replace_fonts(vec![vec![0; 8]])
            .is_err()
    );
    lane.prepare(&device, &queue, [320, 240], 1.)
        .unwrap();
    assert_eq!(lane.resets, 0);
    assert_eq!(
        pixels(&device, &queue, &lane, [320, 240]),
        moved
    );
    assert_eq!(lane.raster_usage(), rasters);
    lane.document
        .replace_fonts(vec![
            crate::text::FONT_BYTES.to_vec(),
            crate::text::FALLBACK_BYTES.to_vec(),
            crate::text::SYMBOL_BYTES.to_vec(),
        ])
        .unwrap();
    lane.prepare(&device, &queue, [320, 240], 1.)
        .unwrap();
    assert_eq!(lane.resets, 1);
    assert_eq!(lane.document.shape_count, 2);
    assert_eq!(lane.document.runs[0].label.text, words);
    assert_eq!(
        pixels(&device, &queue, &lane, [320, 240]),
        moved
    );
}
```

### 4. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod text_atlas;

pub mod text_example;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod text_atlas;

pub mod text_example;

#[cfg(test)]
mod text_atlas_tests;
```

### 5. `src/pick_gpu_tests.rs`

Copy the actual scene-text overlay readback, source/GPU reuse and absence from scene picking.

**Locate this existing block:**

```rust
    );
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_screen_text_overlay_does_not_create_scene_pick_owners_or_reupload_geometry() {
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert(triangle(0.2)).unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    renderer.text_demo().unwrap();
    let with_text = colour_pixels(&mut renderer);
    assert_ne!(with_text, before);
    assert_eq!(renderer.text_error(), None);
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(scene.objects().len(), 1);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([-0.85, 0.8], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    assert_eq!(renderer.text_usage()[0], 1);
    renderer.set_text(Vec::new()).unwrap();
    assert_eq!(renderer.text_usage(), [0; 6]);
    assert_eq!(colour_pixels(&mut renderer), before);
}
```

## Build & Run

![Recorded visual output for Lesson 10.3 (61d-glyphs)](../../../screenshots/journey/61d-glyphs.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.3
npm --prefix ../session_tests run course -- save 10.3
```

Continue with [10.4 · Original world labels, plates and integer picks](../../consolidation/62-source.md).
