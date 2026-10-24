# 21.4 · Reversed depth and homogeneous near clipping

**Goal:** Give every depth reader and writer the same near-one convention. · **Steps:** 3 · **Source:** [camera.rs](../../../src/camera.rs) · [state.rs](../../../src/state.rs) · [engine/gpu/objects.rs](../../../src/engine/gpu/objects.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [shaders/grid.wgsl](../../../src/shaders/grid.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl) · [app/cplane.rs](../../../src/app/cplane.rs) (`src/camera.rs,src/app/units.rs,src/engine/gpu/anchor.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Half-Space Clip: d_i = dot(P_i, n_clip) + w_clip</text>
  <polygon points="58,138 118,58 258,58 198,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="164" y1="44" x2="164" y2="156" stroke="#dc2626" stroke-width="2.5" stroke-dasharray="5 3"/>
  <line x1="164" y1="98" x2="218" y2="98" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="236" y="102" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">d &lt; 0</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Triangle-Plane Edge Segments -&gt; Welded Closed Cap Polygon</text>
  <polygon points="448,70 668,70 638,138 418,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <line x1="458" y1="70" x2="428" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="508" y1="70" x2="478" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="558" y1="70" x2="528" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="608" y1="70" x2="578" y2="138" stroke="#dc2626" stroke-width="1.5"/>
  <circle cx="448" cy="70" r="4" fill="#111111"/>
  <circle cx="668" cy="70" r="4" fill="#111111"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Live Section Cut with Capped Solid Interior</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,138 882,66 948,66 948,138" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <polygon points="948,66 994,82 994,146 948,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

## <a id="checkpoint-96zzzzzt-share-depth-endpoints-and-comparisons"></a>1. Share depth endpoints and comparisons

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Large World Coordinates (f64): P_world = 10^7 m</text>
  <circle cx="64" cy="138" r="4" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="64" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">(0,0,0)</text>
  <line x1="64" y1="138" x2="236" y2="74" stroke="#94a3b8" stroke-width="1.75" stroke-dasharray="5 3" marker-end="url(#arr)"/>
  <circle cx="244" cy="70" r="5" fill="#dc2626"/>
  <polygon points="244,70 278,54 272,92" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="218" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor_f64</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU f64 Delta: P_rel_f32 = (P_world_f64 - Anchor_f64) as f32</text>
  <circle cx="486" cy="112" r="5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="486" y="132" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Anchor = (0,0,0)_rel</text>
  <line x1="486" y1="112" x2="574" y2="82" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <polygon points="574,82 634,58 622,118" fill="#bbf7d0" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="606" y="144" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">||P_rel|| &lt; 10 m (Zero Jitter)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Jitter-Free Far-Origin Orbit + Crisp Depth Precision</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="866,142 934,58 1002,142" fill="#dbeafe" stroke="#111111" stroke-width="2.2"/>
  <line x1="934" y1="58" x2="934" y2="142" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

**What this code does:** Give frame depth state one explicit owner before changing its convention.

### 1. `src/depth.rs`

Own the endpoints and the two comparison predicates. Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

pub fn surface() -> wgpu::DepthStencilState {
    state(true, wgpu::CompareFunction::Less)
}
pub fn ink() -> wgpu::DepthStencilState {
    state(false, wgpu::CompareFunction::LessEqual)
}
pub fn grid() -> wgpu::DepthStencilState {
    state(false, wgpu::CompareFunction::Less)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR: f64 = 0.;
pub const FAR: f64 = 1.;
pub const CLOSER: wgpu::CompareFunction = wgpu::CompareFunction::Less;
pub const CLOSER_EQUAL: wgpu::CompareFunction = wgpu::CompareFunction::LessEqual;

pub fn surface() -> wgpu::DepthStencilState {
    state(true, crate::depth::CLOSER)
}
pub fn ink() -> wgpu::DepthStencilState {
    state(false, crate::depth::CLOSER_EQUAL)
}
pub fn grid() -> wgpu::DepthStencilState {
    state(false, crate::depth::CLOSER)
}
```

### 2. `src/text_plate.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                depth_write_enabled: Some(false),
                depth_compare: Some(crate::depth::CLOSER_EQUAL),
                stencil: Default::default(),
```

### 3. `src/widget_pipelines.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            depth_write_enabled: Some(true),
            depth_compare: Some(crate::depth::CLOSER_EQUAL),
            stencil: Default::default(),
```

### 4. `src/cloud_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            depth_write_enabled: Some(true),
            depth_compare: Some(crate::depth::CLOSER_EQUAL),
            stencil: Default::default(),
```

### 5. `src/outline_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            depth_write_enabled: Some(false),
            depth_compare: Some(crate::depth::CLOSER_EQUAL),
            stencil: Default::default(),
```

### 6. `src/scene_text_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::LessEqual,
        );
```

Replace it with:

```rust
            ctx,
            &mut self.atlas,
            target,
            crate::depth::CLOSER_EQUAL,
        );
```

**Locate this block:**

```rust
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::LessEqual,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
```

Replace it with:

```rust
            ctx,
            &mut self.atlas,
            self.target,
            crate::depth::CLOSER_EQUAL,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
```

**Locate this block:**

```rust
            target,
            wgpu::CompareFunction::LessEqual,
        );
        Self {
            document: TextDocument::new(),
```

Replace it with:

```rust
            target,
            crate::depth::CLOSER_EQUAL,
        );
        Self {
            document: TextDocument::new(),
```

### 7. `src/outline_caps.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                depth_write_enabled: Some(false),
                depth_compare: Some(crate::depth::CLOSER_EQUAL),
                stencil: Default::default(),
```

### 8. `src/stroke_gpu.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
        });
        let strict_depth = depth.depth_compare == Some(wgpu::CompareFunction::Less);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        let strict_depth = depth.depth_compare == Some(crate::depth::CLOSER);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

### 9. `src/text_plane.rs`

Reuse the shared strict or equal comparison in this pipeline.

**Locate this existing block:**

```rust
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(crate::depth::CLOSER_EQUAL), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
```

### 10. `src/pick_encode.rs`

Clear to the shared far endpoint.

**Locate this existing block:**

```rust
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                    store: wgpu::StoreOp::Store,
```

### 11. `src/widget.rs`

Clear to the shared far endpoint.

**Locate this existing block:**

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Discard,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                            store: wgpu::StoreOp::Discard,
```

### 12. `src/cloud_gpu.rs`

Clear to the shared far endpoint.

**Locate this existing block:**

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.),
                            store: wgpu::StoreOp::Store,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                            store: wgpu::StoreOp::Store,
```

### 13. `src/renderer.rs`

Clear to the shared far endpoint.

**Locate this existing block:**

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(crate::depth::FAR as f32),
                            store: wgpu::StoreOp::Store,
```

### 14. `src/camera.rs`

Construct CPU rays from the same near and far endpoints.

**Locate this existing block:**

```rust
            screen[1] as f64,
            0.0,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            1.0,
        ));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            screen[1] as f64,
            crate::depth::NEAR,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            crate::depth::FAR,
        ));
```

### 15. `src/depth.rs`

Supply exact shared-state assertions while retaining every original GPU and camera fixture.

**Locate this existing block:**

```rust
    state(false, wgpu::CompareFunction::Equal)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    state(false, wgpu::CompareFunction::Equal)
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_frame_depth_state_uses_the_same_endpoints() {
        use super::*;
        assert_eq!((NEAR, FAR), (0., 1.));
        assert_eq!(surface().depth_compare, Some(CLOSER));
        assert_eq!(grid().depth_compare, Some(CLOSER));
        assert_eq!(ink().depth_compare, Some(CLOSER_EQUAL));
        assert_eq!(surface().depth_write_enabled, Some(true));
        assert_eq!(ink().depth_write_enabled, Some(false));
        assert_eq!(grid().depth_write_enabled, Some(false));
        assert_eq!(
            state_for_ambient().depth_compare,
            Some(wgpu::CompareFunction::Equal)
        );
    }
}
```

## <a id="checkpoint-96zzzzzu-share-homogeneous-near-plane-ink-clipping"></a>2. Share homogeneous near-plane ink clipping

**What this code does:** Clip expanded original ink through one homogeneous recipe.

### 1. `src/near_clip.wgsl`

Own the homogeneous distance and intersection once.

Create the file and type:

```wgsl
fn near_distance(point:vec4<f32>)->f32{return point.z;}
fn near_endpoint(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{
 let distance=near_distance(a);
 if(distance>=0.){return a;}
 return mix(a,b,distance/(distance-near_distance(b)));
}
```

### 2. `src/stroke.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
```

### 3. `src/finite_stroke.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,flags:u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour,flags); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,flags:u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
```

### 4. `src/guide.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(3) @interpolate(linear) along: f32,
    @location(4) @interpolate(flat) dashed: u32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,0.,0u);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,dashed:u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(3) @interpolate(linear) along: f32,
    @location(4) @interpolate(flat) dashed: u32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,0.,0u);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,dashed:u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
```

### 5. `src/sheet.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, projection_y: f32 }
struct Model { transform: mat4x4<f32>, tint: vec4<f32>, ids: vec4<u32> }
@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var<uniform> model: Model;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(linear) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) original_clip:vec4<f32>,
}
fn geometry(start: vec3<f32>, end: vec3<f32>, radius: f32, packed: u32, vertex: u32, segment: u32) -> Output {
    var colour = vec4<f32>(f32(packed & 255u), f32((packed >> 8u) & 255u), f32((packed >> 16u) & 255u), f32(packed >> 24u)) / 255.0;
    if (model.ids.z == 1u) { colour = vec4<f32>(1.0, 0.68, 0.05, colour.a); }
    else if (model.tint.a > 0.0) { colour = vec4<f32>(model.tint.rgb, colour.a); }
    var output = Output(vec4<f32>(2.0,2.0,2.0,1.0), colour, 0.0, 0.0, vec2<u32>(model.ids.x, (0x80000000u | (model.ids.y + segment)) + 1u),vec4<f32>(0.0,0.0,0.0,1.0));
    var a = view.transform * model.transform * vec4<f32>(start, 1.0);
    var b = view.transform * model.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return output; }
    if (a.z < 0.0) { a = mix(a,b,-a.z/(b.z-a.z)); }
    if (b.z < 0.0) { b = mix(b,a,-b.z/(a.z-b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return output; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, projection_y: f32 }
struct Model { transform: mat4x4<f32>, tint: vec4<f32>, ids: vec4<u32> }
@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var<uniform> model: Model;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(linear) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) original_clip:vec4<f32>,
}
fn geometry(start: vec3<f32>, end: vec3<f32>, radius: f32, packed: u32, vertex: u32, segment: u32) -> Output {
    var colour = vec4<f32>(f32(packed & 255u), f32((packed >> 8u) & 255u), f32((packed >> 16u) & 255u), f32(packed >> 24u)) / 255.0;
    if (model.ids.z == 1u) { colour = vec4<f32>(1.0, 0.68, 0.05, colour.a); }
    else if (model.tint.a > 0.0) { colour = vec4<f32>(model.tint.rgb, colour.a); }
    var output = Output(vec4<f32>(2.0,2.0,2.0,1.0), colour, 0.0, 0.0, vec2<u32>(model.ids.x, (0x80000000u | (model.ids.y + segment)) + 1u),vec4<f32>(0.0,0.0,0.0,1.0));
    var a = view.transform * model.transform * vec4<f32>(start, 1.0);
    var b = view.transform * model.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return output; }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return output; }
```

### 6. `src/chain.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (neighbour.z < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let clipped_a = a.z < 0.0; let clipped_b = b.z < 0.0;
    if (clipped_a && clipped_b) { return hidden; }
    if (clipped_a) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (clipped_b) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (near_distance(neighbour) < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let clipped_a = near_distance(a) < 0.0; let clipped_b = near_distance(b) < 0.0;
    if (clipped_a && clipped_b) { return hidden; }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden; }
```

### 7. `src/finite_chain.wgsl`

Reuse the shared near-plane recipe while keeping family-specific eligibility and width.

**Locate this existing block:**

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (neighbour.z < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32,flags:u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let clipped_a = a.z < 0.0; let clipped_b = b.z < 0.0;
    if (clipped_a && clipped_b) { return hidden; }
    if (clipped_a) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (clipped_b) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
#include "near_clip.wgsl"
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32, opacity:f32 }
@group(0) @binding(0) var<uniform> view: View;
struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) pixel: vec2<f32>,
    @location(2) @interpolate(flat) origin: vec2<f32>,
    @location(3) @interpolate(flat) direction: vec2<f32>,
    @location(4) @interpolate(flat) half_width: f32,
    @location(5) @interpolate(flat) part: u32,
    @location(6) @interpolate(flat) head_length: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
fn joint(neighbour: vec4<f32>, endpoint: vec4<f32>, d: vec2<f32>, before: bool) -> vec2<f32> {
    let normal = perpendicular(d);
    if (near_distance(neighbour) < 0.0 || neighbour.w <= 0.0) { return normal; }
    let span = select(screen(neighbour) - screen(endpoint), screen(endpoint) - screen(neighbour), before);
    if (dot(span, span) < 0.000001) { return normal; }
    let other = normalize(span); let sum = normal + perpendicular(other);
    if (dot(sum, sum) < 0.000001) { return normal; }
    let bisector = normalize(sum);
    return bisector / max(dot(bisector, normal), 0.25);
}
fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, previous: vec3<f32>, next: vec3<f32>, heads: u32, index: u32,flags:u32) -> Output {
    var hidden: Output; hidden.position = vec4<f32>(2.0, 2.0, 2.0, 1.0);
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let clipped_a = near_distance(a) < 0.0; let clipped_b = near_distance(b) < 0.0;
    if (clipped_a && clipped_b) { return hidden; }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden; }
```

### 8. `src/finite_project_source.rs`

Expand the shared shader alongside the existing coverage and triangle helpers.

**Locate this existing block:**

```rust
        .replace(
            "#include \"projected_triangle.wgsl\"",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .replace(
            "#include \"near_clip.wgsl\"",
            include_str!("near_clip.wgsl"),
        )
        .replace(
            "#include \"projected_triangle.wgsl\"",
```

### 9. `src/stroke_gpu.rs`

Expand the guide helper before both color and pick compilation.

**Locate this existing block:**

```rust
            format,
            include_str!("guide.wgsl"),
            48,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            format,
            &crate::finite_project_source::expand(include_str!("guide.wgsl")),
            48,
```

### 10. `src/sheet_pipeline.rs`

Use the same expanded sheet source for color and original picks.

**Locate this existing block:**

```rust
    let crop_layout = crate::gpu_layout::uniform(device, 80);
    let source = include_str!("sheet.wgsl");
    let clip_layout = crate::clipping_gpu::layout(device);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let crop_layout = crate::gpu_layout::uniform(device, 80);
    let expanded = crate::finite_project_source::expand(include_str!("sheet.wgsl"));
    let source = expanded.as_str();
    let clip_layout = crate::clipping_gpu::layout(device);
```

### 11. `src/finite_project_source.rs`

Supply actual assembled shader validation for the original color/pick and grid variants.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod near_tests {
    #[test]
    fn all_original_ink_sources_expand_one_valid_near_clip_owner() {
        let mut sources = vec![
            crate::stroke::shader(),
            crate::chain::shader(),
            super::expand(include_str!("guide.wgsl")),
            crate::clipping_gpu::source(
                &super::expand(include_str!("sheet.wgsl")),
                2,
                0,
            ),
        ];
        for stride in [44, 72, 32] {
            for pick in [false, true] {
                for grid in [false, true] {
                    sources.push(crate::finite_sources::source_style(
                        stride, pick, grid,
                    ));
                }
            }
        }
        for source in sources {
            let module = wgpu::naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
            wgpu::naga::valid::Validator::new(
                wgpu::naga::valid::ValidationFlags::all(),
                wgpu::naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
            assert!(!source.contains("#include"));
        }
    }
}
```

## <a id="checkpoint-96zzzzzz-use-reversed-depth-across-the-frame"></a>3. Use reversed depth across the frame

**What this code does:** Give every depth reader and writer the same near-one convention.

### 1. `src/depth.rs`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR: f64 = 0.;
pub const FAR: f64 = 1.;
pub const CLOSER: wgpu::CompareFunction = wgpu::CompareFunction::Less;
pub const CLOSER_EQUAL: wgpu::CompareFunction = wgpu::CompareFunction::LessEqual;

pub fn surface() -> wgpu::DepthStencilState {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const NEAR: f64 = 1.;
pub const FAR: f64 = 0.;
pub const CLOSER: wgpu::CompareFunction = wgpu::CompareFunction::Greater;
pub const CLOSER_EQUAL: wgpu::CompareFunction = wgpu::CompareFunction::GreaterEqual;

pub fn surface() -> wgpu::DepthStencilState {
```

### 2. `src/camera.rs`

Map perspective near to one and far to zero. Reverse the orthographic depth interval too.

**Locate this existing block:**

```rust
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, near, far)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6 * scale);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Projection::Perspective => {
                Xform::perspective(2.0 * HALF_FOV, self.aspect, far, near)
            }
            Projection::Orthographic => {
                let h = distance * HALF_FOV.tan();
                let w = h * self.aspect;
                let depth_range = (distance + 2.0 * reach).max(1.0e-6 * scale);
                Xform::orthographic(-w, w, -h, h, depth_range, -depth_range)
            }
```

### 3. `src/near_clip.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
fn near_distance(point:vec4<f32>)->f32{return point.z;}
fn near_endpoint(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn near_distance(point:vec4<f32>)->f32{return point.w-point.z;}
fn near_endpoint(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{
```

### 4. `src/clipping_gpu.rs`

Prepare section depths and bias in the reversed frame.

**Locate this existing block:**

```rust
                samples: 1,
                forward_depth: true,
            },
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                samples: 1,
                forward_depth: false,
            },
```

### 5. `src/ambient_camera.rs`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```rust
) -> Option<[f32; 80]> {
    let forward = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let mut flip = session_rust::Xform::identity();
    flip.m[10] = -1.;
    flip.m[14] = 1.;
    let mvp = &flip * &forward;
    let inverse = mvp.inverse()?.m.map(|v| v as f32);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
) -> Option<[f32; 80]> {
    let mvp = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let inverse = mvp.inverse()?.m.map(|v| v as f32);
```

### 6. `src/ambient.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    return 1.0-textureLoad(depth,clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1),sample);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    return textureLoad(depth,clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1),sample);
}
```

### 7. `src/pyramid_prepare.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
@fragment fn fs_prepare(@builtin(position) pixel:vec4<f32>)->DepthRadius{
 let at=floor(pixel.xy/frame.size*frame.full)+0.5;let z=textureLoad(depth,vec2<i32>(at),0);if(z>=1.){return DepthRadius(0.,0u);}
 let origin=world(at,0.);let direction=normalize(world(at,1.)-origin);let p=world(at,z);let along=dot(p-origin,direction)/dot(direction,direction);
 let bits=(pack2x16float(vec2<f32>(frame.radius,0.))+2u)>>2u;let coord=vec2<u32>(pixel.xy);let encoded=(bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment fn fs_prepare(@builtin(position) pixel:vec4<f32>)->DepthRadius{
 let at=floor(pixel.xy/frame.size*frame.full)+0.5;let z=textureLoad(depth,vec2<i32>(at),0);if(z<=0.){return DepthRadius(0.,0u);}
 let origin=world(at,1.);let direction=normalize(world(at,0.)-origin);let p=world(at,z);let along=dot(p-origin,direction)/dot(direction,direction);
 let bits=(pack2x16float(vec2<f32>(frame.radius,0.))+2u)>>2u;let coord=vec2<u32>(pixel.xy);let encoded=(bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
```

### 8. `src/finite_project.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return vec4<f32>(clip.xy,clip.w-clip.z,clip.w);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
 return clip;
}
```

### 9. `src/finite_stroke.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}
```

### 10. `src/finite_chain.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
        let position = vec4<f32>(tip.xy + offset * 2.0 / view.size * tip.w, tip.zw);
        return Output(position, colour, screen(position), screen(tip), aim, half_width, 1u, head_length,flags,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
    }
    let start_offset = direction * select(0.0, head_length, at_start);
    let end_offset = -direction * select(0.0, head_length, at_end);
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + select(start_offset, end_offset, corner.x > 0.5)
        * 2.0 / view.size * position.w, position.zw);
    let offset = select(left, right, corner.x > 0.5) * (half_width + 0.70711) * corner.y;
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0,flags,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
        let position = vec4<f32>(tip.xy + offset * 2.0 / view.size * tip.w, tip.zw);
        return Output(position, colour, screen(position), screen(tip), aim, half_width, 1u, head_length,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
    }
    let start_offset = direction * select(0.0, head_length, at_start);
    let end_offset = -direction * select(0.0, head_length, at_end);
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + select(start_offset, end_offset, corner.x > 0.5)
        * 2.0 / view.size * position.w, position.zw);
    let offset = select(left, right, corner.x > 0.5) * (half_width + 0.70711) * corner.y;
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position));
}
```

### 11. `src/finite_marker.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(1.0-position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
```

### 12. `src/ink_plane.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,1.-hit.x)));
}
```

Replace it with:

```wgsl
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,hit.x)));
}
```

**Locate this block:**

```wgsl
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0.;}
    return 1.-textureLoad(finite_depth,at,0);
}
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
```

Replace it with:

```wgsl
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0.;}
    return textureLoad(finite_depth,at,0);
}
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
```

### 13. `src/finite_visibility.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
// The tutorial still converts forward depth; allow two float rounding units until the reversed camera connects.
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=1.-clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=1.0-textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.0-depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,1.0-hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,1.-depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,1.-depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,1.-axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
@group(1) @binding(4) var finite_primitive:texture_2d<u32>;
#include "ink_plane.wgsl"
fn finite_section(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{
 let primitive=ink_primitive(pixel,0u);
 if(!clip_is_cap(primitive)){return true;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){return true;}
 let z=clip_plane_depth(plane,clip_ndc(pixel,view.size,0.).xy);
 return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);
}
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_grid_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(!finite_section(at,depth,relative)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return false;}
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(at,view.size,hit.x)))&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}


fn finite_alpha(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>,flags:u32,picking:bool)->f32{
 let delta=axis.zw-axis.xy;let t=clamp(dot(pixel-axis.xy,delta)/max(dot(delta,delta),1e-20),0.,1.);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(at,view.size,depth))){return 0.;}
 let hidden=!finite_visible(pixel,axis,depths,flags);return finite_hidden_alpha(hidden,picking);
}

fn finite_disc_alpha(pixel:vec2<f32>,centre:vec2<f32>,depth:f32,flags:u32,picking:bool)->f32{
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(centre,view.size,depth))){return 0.;}
 let tolerance=abs(depth)*DEPTH_REL_TOL+abs(1.-depth)*2.3841858e-7;
 let hidden=!finite_section(centre,depth,tolerance)||!ink_disc_visible(pixel,centre,depth,0u);
 return finite_hidden_alpha(hidden,picking);
}

fn finite_visible(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>,flags:u32)->bool{
 if(FINITE_GRID){return finite_grid_visible(pixel,ends,depths,flags);}
 let axis=finite_ink_axis(pixel,ends,depths);let tolerance=abs(axis.depth)*DEPTH_REL_TOL+abs(1.-axis.depth)*2.3841858e-7;
 if(!finite_section(axis.at,axis.depth,tolerance)){return false;}
 if(clip_active()&&clip_cut_ink_ndc(flags,clip_ndc(axis.at,view.size,axis.depth))){return false;}
 var fit_at=pixel;let source_at=floor(axis.at)+fract(pixel);
```

### 14. `src/cap_count.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(1.0-in.pos.z));
    }

    return vec4<f32>(0.0);
}
@fragment
fn fs_pick_owner(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0 || !leaving(in, front)) {
        discard;
    }

    let word = pick_cap_word(in.plane, in.pos.xy);

    if (bitcast<u32>(1.0-in.pos.z) == atomicLoad(&pick_caps[word + 2u])) {
        atomicStore(&pick_caps[word + 3u], in.inst_id);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(in.pos.z));
    }

    return vec4<f32>(0.0);
}
@fragment
fn fs_pick_owner(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0 || !leaving(in, front)) {
        discard;
    }

    let word = pick_cap_word(in.plane, in.pos.xy);

    if (bitcast<u32>(in.pos.z) == atomicLoad(&pick_caps[word + 2u])) {
        atomicStore(&pick_caps[word + 3u], in.inst_id);
```

### 15. `src/cloud_resolve.wgsl`

Use the same reversed depth in projection, original visibility and reconstruction.

**Locate this existing block:**

```wgsl
@fragment fn color(@builtin(position) p:vec4<f32>)->Pixel {
    let q=vec2<i32>(p.xy);let d=textureLoad(sdepth,q,0);if (d>=1.0) {discard;}
    return Pixel(textureLoad(scolor,q,0),d);
}
struct Pick {@location(0) owner:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment fn pick(@builtin(position) p:vec4<f32>)->Pick {
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d>=1.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@fragment fn color(@builtin(position) p:vec4<f32>)->Pixel {
    let q=vec2<i32>(p.xy);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    return Pixel(textureLoad(scolor,q,0),d);
}
struct Pick {@location(0) owner:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment fn pick(@builtin(position) p:vec4<f32>)->Pick {
    let q=vec2<i32>(p.xy)+vec2<i32>(window.origin);let d=textureLoad(sdepth,q,0);if (d<=0.0) {discard;}
    let ids=textureLoad(sids,q,0).xy;
```

### 16. `src/depth.rs`

Update the supplied endpoint assertion to the new convention; retain all state checks.

**Locate this existing block:**

```rust
        use super::*;
        assert_eq!((NEAR, FAR), (0., 1.));
        assert_eq!(surface().depth_compare, Some(CLOSER));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        use super::*;
        assert_eq!((NEAR, FAR), (1., 0.));
        assert_eq!(surface().depth_compare, Some(CLOSER));
```

### 17. `src/camera.rs`

Keep the same physical near-plane exclusion assertion in reversed NDC.

**Locate this existing block:**

```rust
            assert!(
                cut[2] < 0.,
                "the near plane still cuts ahead of the eye"
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            assert!(
                cut[2] > 1.,
                "the near plane still cuts ahead of the eye"
```

### 18. `src/camera_reverse_tests.rs`

Supply exact unit/projection/depth/ray and distant-origin coverage.

Create the file and type:

```rust
use crate::camera::{Camera, NEAR_FRACTION, Projection, Unit};
use session_rust::{Point, Vector};
#[test]
fn units_projections_and_distant_origins_keep_near_one_far_zero_and_forward_rays() {
    for unit in [Unit::Millimeters, Unit::Meters] {
        for projection in [
            Projection::Perspective,
            Projection::Orthographic,
        ] {
            for target in [[0.; 3], [1e9, -1e9, 1e9]] {
                let mut camera = Camera::default();
                camera.unit = unit;
                camera.projection = projection;
                camera.grid = None;
                camera.target = target;
                camera.distance = 4000.;
                camera.radius = 1000.;
                let direction = camera
                    .orientation
                    .rotate_vector(Vector::y_axis());
                let reach = camera.radius;
                let range = camera.distance + 2. * reach;
                let (near, far) = match projection {
                    Projection::Perspective => (
                        camera.distance * NEAR_FRACTION,
                        (camera.distance * 10.).max(range),
                    ),
                    Projection::Orthographic => (-range, range),
                };
                let point = |gap: f64| {
                    let delta = gap - camera.distance;
                    Point::new(
                        direction[0] * delta,
                        direction[1] * delta,
                        direction[2] * delta,
                    )
                };
                let matrix = camera.view_projection_anchored(target);
                assert!((matrix.transform_point(&point(near))[2] - 1.).abs() < 1e-9);
                assert!(matrix.transform_point(&point(far))[2].abs() < 1e-9);
                let a = matrix.transform_point(&point(camera.distance))[2];
                let b = matrix.transform_point(&point(camera.distance + 100.))[2];
                assert!(a > b && b > 0. && a < 1.);
                let ray = camera.ray([0., 0.]).unwrap();
                assert!(ray.direction.dot(&direction) > 0.999999);
                assert!((ray.max_distance - (far - near)).abs() < 1e-4);
                let expected = point(near);
                for i in 0..3 {
                    assert!((ray.origin[i] - target[i] - expected[i]).abs() < 1e-6);
                }
            }
        }
    }
}
```

### 19. `src/lib.rs`

Register the optional native camera fixture.

**Locate this existing block:**

```rust
#[cfg(test)]
mod full_source_action_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod full_source_action_tests;

#[cfg(test)]
mod camera_reverse_tests;
```

### 20. `src/cap_gpu_tests.rs`

Preserve the original cap fixture world pose with reversed depth; supply a shared fixture-only conversion for arbitrary forward matrices.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        "open back face remains red: {center:?}"
    );
}
```

Replace it with:

```rust
        "open back face remains red: {center:?}"
    );
}

pub(crate) fn reversed(mut matrix: [f32; 16]) -> [f32; 16] {
    for column in 0..4 {
        matrix[column * 4 + 2] = matrix[column * 4 + 3] - matrix[column * 4 + 2];
    }
    matrix
}
```

**Locate this block:**

```rust
pub(crate) fn matrix() -> [f32; 16] {
    session_rust::Xform::orthographic(-1., 1., -1., 1., 0., 1.)
        .m
        .map(|v| v as f32)
}
```

Replace it with:

```rust
pub(crate) fn matrix() -> [f32; 16] {
    session_rust::Xform::orthographic(-1., 1., -1., 1., 1., 0.)
        .m
        .map(|v| v as f32)
}
```

### 21. `src/finite_visibility_tests.rs`

Keep every original finite-visibility world fixture, pixel assertion and pick assertion while reversing its arbitrary projection.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }
    let size = [320, 240];
    let renderer = renderer(&scene, size, 1.);
    let mut matrix = session_rust::Xform::identity()
        .m
        .map(|v| v as f32);
    let texture = renderer
```

Replace it with:

```rust
    }
    let size = [320, 240];
    let renderer = renderer(&scene, size, 1.);
    let mut matrix = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    let texture = renderer
```

**Locate this block:**

```rust
#[test]
fn shifted_view_frames_keep_original_color_and_crop_ownership() {
    let size = [320, 240];
    let mut matrix = session_rust::Xform::identity()
        .m
        .map(|v| v as f32);
    matrix[12] = 0.35;
    matrix[13] = -0.25;
    for kind in 0..3 {
```

Replace it with:

```rust
#[test]
fn shifted_view_frames_keep_original_color_and_crop_ownership() {
    let size = [320, 240];
    let mut matrix = crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    );
    matrix[12] = 0.35;
    matrix[13] = -0.25;
    for kind in 0..3 {
```

**Locate this block:**

```rust
    pick_frame(
        renderer,
        at,
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    )
}
fn pick_frame(
```

Replace it with:

```rust
    pick_frame(
        renderer,
        at,
        crate::cap_gpu_tests::reversed(
            session_rust::Xform::identity()
                .m
                .map(|v| v as f32),
        ),
    )
}
fn pick_frame(
```

**Locate this block:**

```rust
        size,
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    )
}
fn red_frame(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> usize {
```

Replace it with:

```rust
        size,
        crate::cap_gpu_tests::reversed(
            session_rust::Xform::identity()
                .m
                .map(|v| v as f32),
        ),
    )
}
fn red_frame(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> usize {
```

### 22. `src/text_plate.rs`

Place overlay nameplates at the shared near endpoint in both color and original-ID passes.

**Locate this existing block:**

```rust
                        1.0 - 2.0 * y / size[1] as f32,
                        rectangle.depth.unwrap_or(0.0),
                        x - center[0],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        1.0 - 2.0 * y / size[1] as f32,
                        rectangle
                            .depth
                            .unwrap_or(crate::depth::NEAR as f32),
                        x - center[0],
```

### 23. `src/finite_visibility_tests.rs`

Retain all original finite fixtures and additionally compare actual near-clipped line/chain pixels, original picks and source identity against explicitly clipped geometry at DPR1/2.

**Locate this existing block:**

```rust
        }
    }
}
#[test]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }
    for density in [1u32, 2] {
        let size = [320 * density, 240 * density];
        for kind in 0..2 {
            let make = |cropped: bool| {
                let mut scene = Scene::empty();
                let mut line = session_rust::Line::new(
                    if cropped {
                        0.
                    } else {
                        -0.8
                    },
                    0.,
                    if cropped {
                        0.
                    } else {
                        -0.2
                    },
                    0.8,
                    0.,
                    0.2,
                );
                line.width = 4.;
                line.linecolor = session_rust::Color::new(1., 0., 0., 1.);
                if kind == 1 && !cropped {
                    line.arrowhead = session_rust::Arrowhead::START;
                }
                let source = Rc::new(line);
                let id = if kind == 1 {
                    scene
                        .insert_path(
                            crate::chain::PreparedChain::line(source.clone()).unwrap(),
                        )
                        .unwrap()
                } else {
                    scene
                        .insert_line(
                            crate::stroke::PreparedLine::new(source.clone()).unwrap(),
                        )
                        .unwrap()
                };
                (scene, id, source)
            };
            let (original, id, source) = make(false);
            let mut gpu = renderer(&original, size, density as f32);
            assert_eq!(
                picked(&mut gpu, [-0.4, 0.]),
                None,
                "near-clipped original ink cannot win a pick"
            );
            assert_eq!(
                picked(&mut gpu, [0.4, 0.]).unwrap()[0],
                id.value()
            );
            let pixels = red_pixels(&gpu, size);
            assert!(pixels > 100);
            if kind == 0 {
                assert!(Rc::ptr_eq(
                    &original.lines()[0].prepared.source,
                    &source
                ));
            } else {
                assert!(
                    matches!(&original.paths()[0].prepared.source,crate::chain::Source::Line(actual) if Rc::ptr_eq(actual,&source))
                );
            }
            let (cropped, _, _) = make(true);
            gpu.set_scene(&cropped, None);
            assert_eq!(
                red_pixels(&gpu, size),
                pixels,
                "homogeneous near clipping matches the explicit segment and invents no arrowhead at DPR{density}"
            );
        }
    }
}
#[test]
```

### 24. `src/pick_gpu_tests.rs`

Keep all original picking, cloud, text, sheet, outline, density, cancellation and upload assertions with the same physical fixture poses under reversed projection.

**Locate this existing block:**

```rust
fn identity() -> [f32; 16] {
    session_rust::Xform::identity()
        .m
        .map(|n| n as f32)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn identity() -> [f32; 16] {
    crate::cap_gpu_tests::reversed(
        session_rust::Xform::identity()
            .m
            .map(|n| n as f32),
    )
}
```

### 25. `src/grid_source_tests.rs`

Preserve the grid far-plane reach assertion with the same physical range and the reversed far endpoint.

**Locate this existing block:**

```rust
                let new = with.transform_point(&p);
                if old[2] > 1.0 {
                    previously_clipped += 1;
                }
                assert!(
                    new[2] <= 1.0,
                    "An in-front grid reference cannot exceed the far depth"
                );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                let new = with.transform_point(&p);
                if old[2] < 0.0 {
                    previously_clipped += 1;
                }
                assert!(
                    new[2] >= 0.0,
                    "An in-front grid reference cannot cross the far depth"
                );
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 21.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 21.4
npm --prefix ../session_tests run course -- save 21.4
```

Continue with [21.5 · Physical camera, world grid and framed navigation](96zzzzzza-match-physical-camera-defaults-and-limits.md).
