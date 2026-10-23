# 18.2 · Original contact metadata and ambient light

**Goal:** Exercise actual contacts cache and source ownership. · **Steps:** 33 · **Source:** [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs) · [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs) · [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/performance.rs](../../../src/engine/performance.rs) · [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs) · [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs) (`src/shaders/hzb.wgsl,src/shaders/ssao.wgsl,src/app/wood_elements.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">HZB Max-Depth Mip Pyramid + SSAO Hemisphere Samples</text>
  <rect x="56" y="54" width="64" height="64" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="130" y="68" width="42" height="42" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <rect x="182" y="80" width="24" height="24" fill="#60a5fa" stroke="#111111" stroke-width="1.75"/>
  <path d="M 224 134 A 34 34 0 0 1 292 134" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <line x1="258" y1="134" x2="240" y2="104" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arrR)"/>
  <line x1="258" y1="134" x2="258" y2="96" stroke="#16a34a" stroke-width="1.75" marker-end="url(#arrG)"/>
  <line x1="258" y1="134" x2="278" y2="106" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arrB)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">WoodElement Timber Plate + Joint Cut Polylines &amp; Vectors</text>
  <polygon points="434,136 468,66 644,66 610,136" fill="#fde68a" stroke="#111111" stroke-width="2"/>
  <polygon points="518,66 558,66 548,92 508,92" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="533" y1="79" x2="533" y2="44" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Contact-Darkened Creases + Timber Joinery Scene</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,138 874,66 1018,66 982,138" fill="#fcd34d" stroke="#111111" stroke-width="2"/>
  <polygon points="914,66 952,66 942,92 904,92" fill="#78350f" stroke="#111111" stroke-width="2"/>
</svg>
</div>

Reconstruct surface context from actual depth and produce bounded ambient/contact shading through the maintained pass dependencies.

## Ownership and data flow

Source contact radius and normals are weakly cached properties, not reasons to retain editable sources indefinitely. Viewing rays reconstruct original surfaces; reduced depth/radius/normal targets feed bounded horizon and cosine-weighted visibility sampling. Bilateral filters respect surface discontinuities, invalid history is rejected and full-resolution reconstruction uses valid current samples. Atomics collect bounded correction work, and one black-alpha composite applies the contact result after faces. Preparation, filter history and resource keys distinguish a changed camera from unchanged geometry.

## The maintained implementation

engine/gpu/ssao.rs and ssao/pipelines.rs own maintained preparation/filter/composite passes. src/shaders/ssao.wgsl, ambient_depth.wgsl and ambient_composite.wgsl are the actual algorithm sources; engine/gpu/render.rs invokes after_faces hooks before ink. The course instruction must converge to those shader algorithms rather than treating a simplified alternate ambient formula as final.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/ssao.rs](../../../src/engine/gpu/ssao.rs), [engine/gpu/ssao/pipelines.rs](../../../src/engine/gpu/ssao/pipelines.rs), [shaders/ssao.wgsl](../../../src/shaders/ssao.wgsl), [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl), [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl), [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/render.rs](../../../src/engine/gpu/render.rs), [engine/performance.rs](../../../src/engine/performance.rs), [engine/gpu/timing.rs](../../../src/engine/gpu/timing.rs), [app/command/tool/elements.rs](../../../src/app/command/tool/elements.rs), [build.rs](../../../build.rs).

## <a id="checkpoint-90-copy-maintained-original-contact-radius"></a>1. Derive contact radius

**What this code does:** Derive the existing contact radius from the original scene bounds.

### 1. `src/ambient_radius.rs`

Type the complete original geometry query and reuse its current consumer.

Create the file and type:

```rust
use session_rust::AABB;
pub fn ambient_radius(bounds: &AABB) -> f32 {
    if !bounds.is_valid() {
        return 0.0;
    }
    (0.05 * bounds.hx.hypot(bounds.hy).hypot(bounds.hz)).max(0.01) as f32
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod pyramid_pipeline;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod pyramid_gpu;
pub mod source_mesh_topology;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod pyramid_pipeline;

pub mod pyramid_gpu;

pub mod ambient_radius;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
```

## <a id="checkpoint-90a-copy-current-viewing-rays"></a>2. Write current viewing rays

**What this code does:** Unproject pixel neighbors into aligned world-space ray rows.

### 1. `src/ambient_camera.rs`

Type the complete existing WebGPU helper and reuse its current consumer.

Create the file and type:

```rust
pub fn pixel_rays(inverse: &[f32; 16], size: (u32, u32)) -> [f32; 24] {
    // world point at a pixel and depth, as the shader's `world`
    let world = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let ndc = [
            x / f64::from(size.0) * 2.0 - 1.0,
            1.0 - y / f64::from(size.1) * 2.0,
            z,
            1.0,
        ];
        let p: [f64; 4] = std::array::from_fn(|row| {
            (0..4)
                .map(|col| f64::from(inverse[col * 4 + row]) * ndc[col])
                .sum()
        });
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    };
    let near = [
        world(0.0, 0.0, 1.0),
        world(1.0, 0.0, 1.0),
        world(0.0, 1.0, 1.0),
    ];
    let half = [
        world(0.0, 0.0, 0.5),
        world(1.0, 0.0, 0.5),
        world(0.0, 1.0, 0.5),
    ];
    let ray: [[f64; 3]; 3] =
        std::array::from_fn(|i| std::array::from_fn(|k| half[i][k] - near[i][k]));
    let mut out = [0.0; 24];
    for (i, base) in [near, ray].iter().enumerate() {
        for k in 0..3 {
            out[i * 12 + k] = base[0][k] as f32;
            out[i * 12 + 4 + k] = (base[1][k] - base[0][k]) as f32;
            out[i * 12 + 8 + k] = (base[2][k] - base[0][k]) as f32;
        }
    }
    out
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_radius;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_radius;

pub mod ambient_camera;
pub mod finite_project_source;
```

## <a id="checkpoint-90b-adapt-the-earlier-forward-camera"></a>3. Adapt the earlier forward camera

**What this code does:** Adapt the earlier forward camera.

### 1. `src/ambient_camera.rs`

Adapt the earlier camera to the maintained ambient convention.

**Locate this existing block:**

```rust
    out
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    out
}

pub fn uniform(
    matrix: [f32; 16],
    full: [u32; 2],
    size: [u32; 2],
    ground: [f32; 2],
) -> Option<[f32; 80]> {
    let forward = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let mut flip = session_rust::Xform::identity();
    flip.m[10] = -1.;
    flip.m[14] = 1.;
    let mvp = &flip * &forward;
    let inverse = mvp.inverse()?.m.map(|v| v as f32);
    if inverse.iter().any(|v| !v.is_finite()) {
        return None;
    }
    let mut out = [0.; 80];
    out[..16].copy_from_slice(&inverse);
    out[16..32].copy_from_slice(&mvp.m.map(|v| v as f32));
    out[32..40].copy_from_slice(&[
        ground[0],
        ground[1],
        full[0] as f32,
        full[1] as f32,
        size[0] as f32,
        size[0] as f32,
        size[1] as f32,
        size[1] as f32,
    ]);
    out[40..64].copy_from_slice(&pixel_rays(&inverse, (full[0], full[1])));
    Some(out)
}
```

## <a id="checkpoint-90c-gather-weak-original-ambient-properties"></a>4. Gather weak ambient properties

**What this code does:** Gather weak original ambient properties.

### 1. `src/ambient_rows.rs`

Gather current original contact properties without retaining source documents.

Create the file and type:

```rust
#[derive(Default)]
pub struct Rows {
    bounds: crate::cap_source_bounds::Bounds,
    pub specs: Vec<(u32, f32)>,
    pub ground: [f32; 2],
}
impl Rows {
    pub fn set(&mut self, scene: &crate::scene::Scene) {
        self.specs.clear();
        let mut floor = f64::INFINITY;
        let mut radius = 0f32;
        for object in scene
            .objects()
            .iter()
            .filter(|r| scene.visible(r.id) && !r.mesh.indices().is_empty())
        {
            if let Some(bounds) = self
                .bounds
                .placed(&object.mesh, &object.model)
            {
                let r = crate::ambient_radius::ambient_radius(&bounds);
                self.specs.push((object.id.value(), r));
                floor = floor.min(bounds.cz - bounds.hz);
                radius = radius.max(r);
            }
        }
        self.ground = [
            if floor.is_finite() {
                floor as f32
            } else {
                0.
            },
            radius,
        ];
        self.bounds.prune();
        if self.specs.is_empty() {
            self.specs.shrink_to_fit();
        }
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_camera;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_camera;

pub mod ambient_rows;
pub mod finite_project_source;
```

## <a id="checkpoint-90d-copy-current-horizon-and-bilateral-recipes"></a>5. Declare the ambient pass resource contracts

**What this code does:** Keep ray, depth, normal, radius and history inputs in explicit bind groups.

### 1. `src/ambient.wgsl`

Keep ray, depth, normal, radius and history inputs in explicit bind groups.

Create the file and type:

```wgsl
const MSAA: bool = false;
@group(3) @binding(0) var<storage,read_write> corrections: array<u32>;
@group(3) @binding(1) var<storage,read_write> edge_flags: array<atomic<u32>>;
@group(3) @binding(2) var<storage,read_write> edge_tiles: array<u32>;
@group(3) @binding(4) var history_ao: texture_2d<f32>;
@group(3) @binding(5) var history_depth: texture_2d<f32>;
struct Draw { vertices: u32, instances: atomic<u32>, first_vertex: u32, first_instance: u32 };
@group(3) @binding(3) var<storage,read_write> draw_edges: Draw;
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var source_radii:texture_2d<f32>;
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_sections:texture_2d<f32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
@group(1) @binding(2) var normals: texture_2d<f32>;
@group(2) @binding(0) var occlusion: texture_2d<f32>;
@group(2) @binding(1) var linear_sampler: sampler;

struct DepthRadius {
    @location(0) depth: f32,
    @location(1) radius: u32,
    @location(2) normal: vec2<f32>,
};
struct Hit {
    point: vec3<f32>,
    along: f32,
    radius: f32,
};
```

## <a id="checkpoint-90da-reconstruct-original-surfaces-from-depth"></a>6. Reconstruct surfaces from depth

**What this code does:** Unproject framebuffer depth and recover ray positions without changing original CAD geometry.

### 1. `src/ambient.wgsl`

Unproject framebuffer depth and recover ray positions without changing original CAD geometry.

**Locate this existing block:**

```wgsl
    radius: f32,
};
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    radius: f32,
};

fn fullscreen(i: u32) -> vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2<f32>(-1.0,-1.0), vec2<f32>(3.0,-1.0), vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> { return fullscreen(i); }
fn world(pixel: vec2<f32>, z: f32) -> vec3<f32> {
    let uv = pixel / ambient.params.zw;
    let p = ambient.inverse_mvp * vec4<f32>(uv.x*2.0-1.0, 1.0-uv.y*2.0, z, 1.0);
    return p.xyz / p.w;
}
fn ray_origin(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.near.xyz+ambient.near_x.xyz*pixel.x+ambient.near_y.xyz*pixel.y;
}
fn ray_direction(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.ray.xyz+ambient.ray_x.xyz*pixel.x+ambient.ray_y.xyz*pixel.y;
}
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    return 1.0-textureLoad(depth,clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1),sample);
}
fn surface_at_depth(pixel: vec2<f32>, z: f32) -> vec4<f32> {
    if z>0.0 { return vec4<f32>(world(pixel,z),1.0); }
    let near = ray_origin(pixel);
    let direction = ray_direction(pixel);
    if direction.z>=-1e-7 || near.z<ambient.params.x { return vec4<f32>(0.0); }
    return vec4<f32>(near+direction*((ambient.params.x-near.z)/direction.z),1.0);
}
fn surface(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    return surface_at_depth(pixel,depth_sample(pixel,sample));
}
fn full_pixel(pixel: vec2<f32>) -> vec2<f32> {
    return floor(pixel/ambient.extent.yz*ambient.params.zw)+0.5;
}
fn position_at(pixel: vec2<f32>, along: f32) -> vec3<f32> {
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}
```

## <a id="checkpoint-90db-encode-normals-and-read-original-contact-metadata"></a>7. Encode normals and read contact metadata

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

**What this code does:** Read original radii, section normals and geometric fallback normals through the existing property textures.

### 1. `src/ambient.wgsl`

Read original radii, section normals and geometric fallback normals through the existing property textures.

**Locate this existing block:**

```wgsl
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}

fn encode_normal(n: vec3<f32>) -> vec2<f32> {
    let p = n.xy/(abs(n.x)+abs(n.y)+abs(n.z));
    let folded = (1.0-abs(p.yx))*select(vec2<f32>(-1.0),vec2<f32>(1.0),p>=vec2<f32>(0.0));
    return select(folded,p,n.z>=0.0);
}
fn decode_normal(e: vec2<f32>) -> vec3<f32> {
    let z = 1.0-abs(e.x)-abs(e.y);
    let t = max(-z,0.0);
    let xy = e-select(vec2<f32>(-t),vec2<f32>(t),e>=vec2<f32>(0.0));
    return normalize(vec3<f32>(xy,z));
}
fn normal_half(coord: vec2<i32>) -> vec3<f32> {
    return decode_normal(textureLoad(normals,coord,0).xy*2.0-1.0);
}
fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 return select(radius,ambient.params.y,radius==0.0&&cap>=0.5);
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 if(cap>=0.5){let plane=(u32(cap)-1u)&7u;let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

## <a id="checkpoint-90dc-prepare-a-compact-depth-radius-and-normal-target"></a>8. Prepare a compact depth radius and normal target

**What this code does:** Write ray distance, contact radius and octahedral normal into separate attachments.

### 1. `src/ambient.wgsl`

Write ray distance, contact radius and octahedral normal into separate attachments.

**Locate this existing block:**

```wgsl
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}

@fragment fn fs_prepare(@builtin(position) pixel: vec4<f32>) -> DepthRadius {
    let at = full_pixel(pixel.xy);
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    let z = depth_sample(at,0);
    if z==0.0 {
        if ray.z>=-1e-7 || origin.z<ambient.params.x { return DepthRadius(0.0,0u,vec2<f32>(0.5)); }
        return DepthRadius(-(ambient.params.x-origin.z)/ray.z,0u,vec2<f32>(0.5));
    }
    let p = world(at,z);
    let along = dot(p-origin,ray)/dot(ray,ray);
    let radius=source_radius(at)/max(ambient.params.y,1e-20);
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
    return DepthRadius(along,encoded,encode_normal(normal_of(at,p,0))*0.5+0.5);
}
fn radius_at(coord: vec2<i32>) -> f32 {
    return unpack2x16float((textureLoad(radii,coord,0).x>>4u)<<2u).x*ambient.params.y;
}
fn sample_hit(at: vec2<f32>, offset: vec2<f32>, level: i32) -> Hit {
    let dims = vec2<i32>(textureDimensions(linear,level));
    let coord = vec2<i32>(floor((at+offset)/ambient.params.zw*ambient.extent.yz))>>vec2<u32>(u32(level));
    if any(coord<vec2<i32>(0)) || any(coord>=dims) { return Hit(vec3<f32>(0.0),0.0,0.0); }
    let along = textureLoad(linear,coord,level).x;
    let encoded = textureLoad(radii,coord,level).x;
    let mask = (1u<<u32(level))-1u;
    let offset_coord = vec2<u32>(encoded&3u,(encoded>>2u)&3u)&vec2<u32>(mask);
    let source_coord = (vec2<u32>(coord)<<vec2<u32>(u32(level)))+offset_coord;
    let pixel = full_pixel(vec2<f32>(source_coord)+0.5);
    return Hit(position_at(pixel,along),along,unpack2x16float((encoded>>4u)<<2u).x*ambient.params.y);
}
fn stride() -> u32 {
    return select(1u,2u,ambient.near.w!=0.0);
}
fn phase(pixel: vec2<f32>, stride: u32) -> u32 {
    return (u32(pixel.x)+u32(pixel.y))%stride;
}
```

## <a id="checkpoint-90dd-measure-ground-contact-along-bounded-horizons"></a>9. Measure ground contact along bounded horizons

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

**What this code does:** Sample bounded directions and steps using the retained mip levels and physical contact radius.

### 1. `src/ambient.wgsl`

Sample bounded directions and steps using the retained mip levels and physical contact radius.

**Locate this existing block:**

```wgsl
    return (u32(pixel.x)+u32(pixel.y))%stride;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return (u32(pixel.x)+u32(pixel.y))%stride;
}

fn ground_contact(at: vec2<f32>, p: vec3<f32>, pixel_world: f32, noise: f32, phase: u32) -> f32 {
    let dims = vec2<i32>(textureDimensions(occlusion));
    let tile = clamp(vec2<i32>(at/ambient.params.zw*vec2<f32>(dims)),vec2<i32>(0),dims-1);
    if textureLoad(occlusion,tile,0).x==0.0 { return 0.0; }
    let radius_px = min(ambient.params.y*6.0/max(pixel_world,1e-6),64.0);
    let stride = stride();
    var sum = 0.0;
    for (var index=0u; index<24u/stride; index++) {
        let direction = index*stride+phase;
        let angle = (f32(direction)+noise)*0.261799388;
        let axis = vec2<f32>(cos(angle),sin(angle));
        var horizon = 0.0;
        for (var step=0u; step<6u; step++) {
            let jitter = fract(noise+f32(direction)*0.618034);
            let fraction = (f32(step)+0.5+jitter*0.5)/6.0;
            let span = max(1.5,radius_px*fraction*fraction);
            let scaled = span*ambient.extent.y/ambient.params.z;
            let level = i32(scaled>=8.0)+i32(scaled>=16.0);
            let hit = sample_hit(at,axis*span,level);
            let radius = hit.radius*6.0;
            if hit.along<=0.0 || radius<=0.0 { continue; }
            let delta = hit.point-p;
            if delta.z<=0.0 || delta.z>=radius { continue; }
            let elevation = max(delta.z/max(length(delta),1e-6)-0.04,0.0);
            let radial = 1.0-smoothstep(radius*0.1,radius,length(delta.xy));
            let vertical = 1.0-smoothstep(0.0,radius,delta.z);
            horizon = max(horizon,elevation*radial*vertical*vertical);
        }
        sum += horizon;
    }
    return clamp(sum*(2.6*f32(stride)/24.0),0.0,0.65);
}
@fragment fn fs_ground(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let at = pixel.xy/ambient.extent.xw*ambient.params.zw;
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    if ray.z>=-1e-7 || origin.z<ambient.params.x { return 0.0; }
    let along = (ambient.params.x-origin.z)/ray.z;
    let p = origin+ray*along;
    let pixel_world = max(length(ambient.near_x.xyz+ambient.ray_x.xyz*along),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    return ground_contact(at,p,pixel_world,noise,phase(pixel.xy,stride()));
}
fn filtered_ground(at: vec2<f32>) -> f32 {
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}
```

## <a id="checkpoint-90de-integrate-cosine-weighted-surface-visibility"></a>10. Integrate cosine weighted surface visibility

**What this code does:** Accumulate the occluded part of each projected normal slice, with bounded horizon sampling.

### 1. `src/ambient.wgsl`

Accumulate the occluded part of each projected normal slice, with bounded horizon sampling.

**Locate this existing block:**

```wgsl
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}

fn arc(h: f32, n: f32) -> f32 {
    return (cos(n)+2.0*h*sin(n)-cos(2.0*h-n))*0.25;
}
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = vec2<i32>(pixel.xy);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let at = full_pixel(pixel.xy);
    let p = position_at(at,along);
    let step_x = ambient.near_x.xyz+ambient.ray_x.xyz*abs(along);
    let step_y = ambient.near_y.xyz+ambient.ray_y.xyz*abs(along);
    let pixel_world = max(length(step_x),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    if along<0.0 { return filtered_ground(at); }
    let radius = radius_at(coord);
    if radius<=0.0 { return 0.0; }
    let normal = normal_half(coord);
    let view = -normalize(ray_direction(at));
    let radius_px = min(radius*8.0/pixel_world,128.0);
    let bias = radius*0.025;
    let stride = stride();
    let phase = phase(pixel.xy,stride);
    var occluded = 0.0;
    for (var index=0u; index<4u/stride; index++) {
        let slice = index*stride+phase;
        let angle = (f32(slice)+noise)*0.785398163;
        let axis = vec2<f32>(cos(angle),sin(angle));
        let tangent = step_x*axis.x+step_y*axis.y;
        let side = normalize(tangent-view*dot(tangent,view));
        let plane_normal = cross(side,view);
        let projected = normal-plane_normal*dot(normal,plane_normal);
        let normal_length = length(projected);
        let n = atan2(dot(projected,side),dot(projected,view));
        var slice_occ = 0.0;
        for (var sign=-1; sign<=1; sign+=2) {
            let signed_n = n*f32(sign);
            let low = cos(signed_n+1.570796327);
            var horizon = low;
            for (var step=0u; step<6u; step++) {
                let fraction = (f32(step)+0.5)/6.0;
                let step_length = max(1.5,radius_px*fraction*fraction);
                let scaled = step_length*ambient.extent.y/ambient.params.z;
                let level = i32(scaled>=8.0)+i32(scaled>=16.0);
                let hit = sample_hit(at,axis*f32(sign)*step_length,level);
                if hit.along<=0.0 { continue; }
                let delta = hit.point-p;
                let span = max(length(delta),1e-6);
                if dot(delta,normal)<=span*0.07+bias { continue; }
                let weight = max(1.0-smoothstep(radius*0.5,radius*2.0,span),0.75*(1.0-smoothstep(radius*4.0,radius*16.0,span)));
                horizon = max(horizon,mix(low,dot(delta/span,view),weight));
            }
            let h = acos(clamp(horizon,-1.0,1.0));
            slice_occ += arc(signed_n+1.570796327,signed_n)-arc(h,signed_n);
        }
        occluded += normal_length*slice_occ;
    }
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}
```

## <a id="checkpoint-90df-filter-contact-without-crossing-unrelated-surfaces"></a>11. Filter contact without crossing unrelated surfaces

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Drag Left-&gt;Right (Window) vs Right-&gt;Left (Crossing)</text>
  <rect x="54" y="54" width="108" height="88" fill="#dbeafe" fill-opacity="0.35" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="108" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">dx &gt; 0: Inside Only</text>
  <rect x="180" y="54" width="108" height="88" fill="#dcfce7" fill-opacity="0.35" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="234" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">dx &lt; 0: Any Overlap</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen AABB Containment vs Segment-Rect Intersection</text>
  <rect x="434" y="56" width="112" height="84" fill="none" stroke="#2563eb" stroke-width="2"/>
  <polygon points="458,122 490,74 522,122" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="574" y="56" width="112" height="84" fill="none" stroke="#16a34a" stroke-width="2" stroke-dasharray="5 3"/>
  <polygon points="638,126 676,68 712,126" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Multi-Object Selection Set Updated</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,136 876,74 914,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <polygon points="944,136 982,74 1020,136" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
</svg>
</div>

**What this code does:** Weight neighboring occlusion by surface separation, normal and contact radius.

### 1. `src/ambient.wgsl`

Weight neighboring occlusion by surface separation, normal and contact radius.

**Locate this existing block:**

```wgsl
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}

fn weight_at(q: vec2<i32>, p: vec3<f32>, n: vec3<f32>, geometry: bool, radius: f32) -> f32 {
    let along = textureLoad(linear,q,0).x;
    if along==0.0 || (along>0.0)!=geometry { return 0.0; }
    let delta = position_at(full_pixel(vec2<f32>(q)+0.5),along)-p;
    let separation = abs(dot(delta,n));
    let tolerance = max(radius*0.024,length(delta)*0.08);
    let ground_weight = select(1.0-smoothstep(0.0,radius,length(delta.xy)),1.0,geometry);
    return exp(-separation/max(tolerance,1e-6))*ground_weight;
}
fn denoise(pixel: vec2<f32>, axis: vec2<i32>) -> vec3<f32> {
    let coord = vec2<i32>(pixel);
    var values: array<f32, 7>;
    var total = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
        total += values[i];
    }
    if total==0.0 { return vec3<f32>(0.0); }
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return vec3<f32>(0.0); }
    let at = full_pixel(pixel);
    let p = position_at(at,along);
    var n = vec3<f32>(0.0,0.0,1.0);
    if along>0.0 { n = normal_half(coord); }
    let radius = select(ambient.params.y,radius_at(coord),along>0.0);
    var sum = 0.0;
    var weights = 0.0;
    var low = 1.0;
    var high = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let weight = exp(-f32((i-3)*(i-3))*0.12)*weight_at(q,p,n,along>0.0,radius);
        if weight>0.05 { low=min(low,values[i]); high=max(high,values[i]); }
        sum += values[i]*weight;
        weights += weight;
    }
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}
```

## <a id="checkpoint-90dg-reject-invalid-ambient-history-before-blending"></a>12. Reject invalid ambient history before blending

**What this code does:** Reproject into the prior frame, validate signed depth and clamp reused visibility to the current neighborhood.

### 1. `src/ambient.wgsl`

Reproject into the prior frame, validate signed depth and clamp reused visibility to the current neighborhood.

**Locate this existing block:**

```wgsl
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}

@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    return denoise(pixel.xy,vec2<i32>(1,0)).x;
}
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let current = denoise(pixel.xy,vec2<i32>(0,1));
    if ambient.near.w==0.0 || current.z==0.0 { return current.x; }
    let along = textureLoad(linear,vec2<i32>(pixel.xy),0).x;
    if along==0.0 { return current.x; }
    let p = position_at(full_pixel(pixel.xy),along);
    let clip = ambient.previous_mvp*vec4<f32>(p,1.0);
    let uv = vec2<f32>(clip.x,-clip.y)/clip.w*0.5+0.5;
    if clip.w<=0.0 || any(uv<=vec2<f32>(0.0)) || any(uv>=vec2<f32>(1.0)) { return current.x; }
    let expected = clip.z/clip.w;
    let dims = vec2<i32>(textureDimensions(history_depth));
    let base = vec2<i32>(floor(uv*vec2<f32>(dims)-0.5));
    var valid = false;
    for(var i=0; i<4; i++) {
        let q = clamp(base+vec2<i32>(i%2,i/2),vec2<i32>(0),dims-1);
        let depth = textureLoad(history_depth,q,0).x;
        valid = valid || (depth!=0.0 && (depth>0.0)==(along>0.0)
            && abs(abs(depth)-expected)<max(abs(expected)*0.0025,1e-6));
    }
    if !valid { return current.x; }
    let previous = textureSampleLevel(history_ao,linear_sampler,uv,0.0).x;
    return mix(current.x,clamp(previous,current.y,current.z),0.8);
}
@fragment fn fs_history(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = min(vec2<i32>(pixel.xy)*2+1,vec2<i32>(ambient.extent.yz)-1);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let p = position_at(full_pixel(vec2<f32>(coord)+0.5),along);
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    return select(-clip.z/clip.w,clip.z/clip.w,along>0.0);
}
```

## <a id="checkpoint-90dh-reconstruct-full-resolution-contact-from-low-resolution-samples"></a>13. Reconstruct full resolution contact from low resolution samples

**What this code does:** Use surface-aware weights to combine the four nearby low-resolution values.

### 1. `src/ambient.wgsl`

Use surface-aware weights to combine the four nearby low-resolution values.

**Locate this existing block:**

```wgsl
    return select(-clip.z/clip.w,clip.z/clip.w,along>0.0);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return select(-clip.z/clip.w,clip.z/clip.w,along>0.0);
}

fn source_base(at: vec2<f32>) -> vec2<i32> {
    let base = vec2<i32>(floor(at/ambient.params.zw*ambient.extent.yz-0.5));
    return base-select(vec2<i32>(0),vec2<i32>(1),at<full_pixel(vec2<f32>(base)+0.5));
}
fn reconstruct(at: vec2<f32>, sample: i32, z: f32, values: vec4<f32>) -> f32 {
    let base = source_base(at);
    let p = surface_at_depth(at,z);
    if p.w==0.0 { return 0.0; }
    let geometry = z>0.0;
    var n = vec3<f32>(0.0,0.0,1.0);
    var radius = ambient.params.y;
    if geometry {
        n=normal_of(at,p.xyz,sample);
        radius=source_radius(at);
    }
    let origin = full_pixel(vec2<f32>(base)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
    var sum = 0.0;
    var weights = 0.0;
    for (var i=0; i<4; i++) {
        let offset = vec2<i32>(i%2,i/2);
        let q = clamp(base+offset,vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let spatial = mix(1.0-blend,blend,vec2<f32>(offset));
        let w = spatial.x*spatial.y*weight_at(q,p.xyz,n,geometry,radius);
        sum += values[i]*w;
        weights += w;
    }
    return sum/max(weights,1e-6);
}
```

## <a id="checkpoint-90di-collect-bounded-sample-corrections-with-gpu-atomics"></a>14. Collect bounded sample corrections with GPU atomics

**What this code does:** Pack per-sample corrections and claim each affected tile once for indirect edge draws.

### 1. `src/ambient.wgsl`

Pack per-sample corrections and claim each affected tile once for indirect edge draws.

**Locate this existing block:**

```wgsl
    return sum/max(weights,1e-6);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return sum/max(weights,1e-6);
}

@fragment fn fs_upsample(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let xy = vec2<u32>(pixel.xy);
    if MSAA { corrections[xy.y*u32(ambient.params.z)+xy.x] = 0u; }
    let base_coord = source_base(pixel.xy);
    var values = vec4<f32>(0.0);
    for (var i=0; i<4; i++) {
        let q = clamp(base_coord+vec2<i32>(i%2,i/2),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
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
    }
    if !different { return first; }
    let base = round(min(min(ao.x,ao.y),min(ao.z,ao.w))*255.0)/255.0;
    let packed = pack4x8unorm((ao-vec4<f32>(base))/max(1.0-base,1e-6));
    if packed!=0u {
        corrections[xy.y*u32(ambient.params.z)+xy.x] = packed;
        let tile_width = (u32(ambient.params.z)+15u)/16u;
        let tile = (xy.y/16u)*tile_width+xy.x/16u;
        if atomicOr(&edge_flags[tile],1u)==0u {
            let index = atomicAdd(&draw_edges.instances,4u)/4u;
            edge_tiles[index] = tile;
        }
    }
    return base;
}
```

## <a id="checkpoint-90dj-composite-contact-as-black-alpha-over-the-original-scene"></a>15. Composite contact as black alpha over the scene

**What this code does:** Load the current contact amount and use per-sample masks for corrected edge draws.

### 1. `src/ambient_composite.wgsl`

Load the current contact amount and use per-sample masks for corrected edge draws.

Create the file and type:

```wgsl
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> ambient: Ambient;
@group(0) @binding(1) var<storage,read> corrections: array<u32>;
@group(0) @binding(2) var<storage,read> tiles: array<u32>;
@group(1) @binding(0) var occlusion: texture_2d<f32>;
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>,3>(vec2<f32>(-1.0,-1.0),vec2<f32>(3.0,-1.0),vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0,0.0,0.0,textureLoad(occlusion,vec2<i32>(pixel.xy),0).x);
}
struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) sample: u32,
};
@vertex fn vs_edges(@builtin(vertex_index) i: u32, @builtin(instance_index) instance: u32) -> Vertex {
    let tile = tiles[instance/4u];
    let width = (u32(ambient.params.z)+15u)/16u;
    let origin = vec2<f32>(f32(tile%width),f32(tile/width))*16.0;
    let corners = array<vec2<f32>,6>(vec2<f32>(0.0,0.0),vec2<f32>(1.0,0.0),vec2<f32>(0.0,1.0),vec2<f32>(0.0,1.0),vec2<f32>(1.0,0.0),vec2<f32>(1.0,1.0));
    let xy = (origin+corners[i]*16.0)/ambient.params.zw;
    return Vertex(vec4<f32>(xy.x*2.0-1.0,1.0-xy.y*2.0,0.0,1.0),instance%4u);
}
struct Shade {
    @location(0) color: vec4<f32>,
    @builtin(sample_mask) mask: u32,
};
@fragment fn fs_edges(pixel: Vertex) -> Shade {
    let xy = vec2<u32>(pixel.position.xy);
    let packed = corrections[xy.y*u32(ambient.params.z)+xy.x];
    let alpha = f32((packed>>(pixel.sample*8u))&255u)/255.0;
    if alpha==0.0 { discard; }
    return Shade(vec4<f32>(0.0,0.0,0.0,alpha),1u<<pixel.sample);
}
```

## <a id="checkpoint-90e-bind-real-original-normals-and-radii"></a>16. Validate the assembled ambient shader contracts

**What this code does:** Read optional shader-interface validation before runtime pipeline creation.

### 1. `src/ambient_shader_contract.rs`

Validate the actual shader types and public entry points.

Create the file and type:

```rust
#[test]
fn ambient_shader_contracts_validate_before_pipeline_connection() {
    let sources = [
        crate::clipping_gpu::source(include_str!("ambient.wgsl"), 0, 5),
        include_str!("ambient_composite.wgsl").to_string(),
    ];
    for source in sources {
        let module = wgpu::naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        wgpu::naga::valid::Validator::new(
            wgpu::naga::valid::ValidationFlags::all(),
            wgpu::naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .expect("valid ambient GPU contract");
        assert!(
            module
                .entry_points
                .iter()
                .any(|entry| entry.name == "vs_main")
        );
        assert!(
            module
                .entry_points
                .iter()
                .any(|entry| entry.name == "fs_main")
        );
    }
}
```

### 2. `src/lib.rs`

Compile the shader contract at the completed owner boundary.

**Locate this existing block:**

```rust
pub mod ambient_rows;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_rows;
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
```

## <a id="checkpoint-90f-write-original-surface-contact-properties"></a>17. Write surface contact properties

**What this code does:** Write original surface contact properties.

### 1. `src/ambient_sources.wgsl`

Derive original ambient properties without changing color or source geometry.

Create the file and type:

```wgsl
struct View{transform:mat4x4<f32>,display:vec4<f32>}
struct Object{model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:vec3<f32>)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(ambient_radius.x,encode_normal(normal)*0.5+0.5);
}
```

## <a id="checkpoint-90g-build-the-depth-equal-original-property-pass"></a>18. Build the depth-equal property pass

**What this code does:** Build the depth-equal original property pass.

### 1. `src/depth.rs`

Add an explicit depth-equal read-only property state.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub fn state_for_ambient() -> wgpu::DepthStencilState {
    state(false, wgpu::CompareFunction::Equal)
}
```

### 2. `src/ambient_source_pipeline.rs`

Build the compatible original property pass.

Create the file and type:

```rust
pub struct Pipeline {
    pub draw: wgpu::RenderPipeline,
    pub radius: wgpu::BindGroupLayout,
}
impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 160);
        let clip = crate::clipping_gpu::layout(device);
        let radius = crate::gpu_layout::uniform(device, 16);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ambient original source metadata"),
            bind_group_layouts: &[
                Some(&view),
                Some(&model),
                Some(&clip),
                Some(&radius),
            ],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ambient original normals and radii"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("ambient_sources.wgsl"), 2, 0)
                    .into(),
            ),
        });
        let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:24,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3]},wgpu::VertexBufferLayout{array_stride:12,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![2=>Float32x3]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self { draw, radius }
    }
}
```

### 3. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_rows;
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_rows;

#[cfg(test)]
mod ambient_shader_contract;
pub mod ambient_source_pipeline;
pub mod finite_project_source;
```

## <a id="checkpoint-90h-cache-original-radius-buffers-and-draw-properties"></a>19. Cache radius buffers and draw properties

**What this code does:** Cache original radius buffers and draw properties.

### 1. `src/ambient_source_gpu.rs`

Own original property uniforms and compatible metadata draws.

Create the file and type:

```rust
use std::collections::HashMap;
pub struct Sources {
    pipes: crate::ambient_source_pipeline::Pipeline,
    rows: HashMap<u32, (wgpu::Buffer, wgpu::BindGroup, f32)>,
    pub writes: u64,
    pub draws: u64,
}
impl Sources {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            pipes: crate::ambient_source_pipeline::Pipeline::new(device),
            rows: Default::default(),
            writes: 0,
            draws: 0,
        }
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        specs: &[(u32, f32)],
    ) {
        let live = specs
            .iter()
            .map(|(id, _)| *id)
            .collect::<std::collections::HashSet<_>>();
        self.rows.retain(|id, _| live.contains(id));
        for &(id, radius) in specs {
            let row = self.rows.entry(id).or_insert_with(|| {
                let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("original ambient contact radius"),
                    size: 16,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                let group =
                    crate::gpu_layout::group(device, &self.pipes.radius, &buffer);
                (buffer, group, f32::NAN)
            });
            if row.2 != radius {
                queue.write_buffer(&row.0, 0, &radius.to_ne_bytes());
                row.2 = radius;
                self.writes += 1;
            }
        }
        if self.rows.is_empty() {
            self.rows.shrink_to_fit();
        }
    }
    pub fn encode(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        radius: &wgpu::TextureView,
        normals: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clip: &wgpu::BindGroup,
        meshes: &[crate::gpu_mesh::GpuMesh],
    ) {
        let attachments = [radius, normals].map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })
        });
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("original ambient source properties"),
            color_attachments: &attachments,
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&self.pipes.draw);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(2, clip, &[]);
        for mesh in meshes {
            if let Some((_, group, _)) = self.rows.get(&mesh.id.value()) {
                mesh.bind_model(&mut pass);
                pass.set_bind_group(3, group, &[]);
                mesh.geometry.draw(&mut pass);
                self.draws += 1;
            }
        }
    }
    pub fn bytes(&self) -> u64 {
        self.rows.values().map(|r| r.0.size()).sum()
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_rows;

#[cfg(test)]
mod ambient_shader_contract;
pub mod ambient_source_pipeline;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_rows;

pub mod ambient_source_pipeline;

#[cfg(test)]
mod ambient_shader_contract;
pub mod ambient_source_gpu;
pub mod finite_project_source;
```

## <a id="checkpoint-90i-own-actual-ambient-image-allocation"></a>20. Own actual ambient image allocation

**What this code does:** Own actual ambient image allocation.

### 1. `src/ambient_image.rs`

Own actual single-level ambient images.

Create the file and type:

```rust
pub struct Image {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bytes: u64,
    pub size: [u32; 2],
}
impl Image {
    pub fn new(
        device: &wgpu::Device,
        size: [u32; 2],
        format: wgpu::TextureFormat,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ambient actual image"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        let bytes = u64::from(size[0])
            * u64::from(size[1])
            * u64::from(format.block_copy_size(None).unwrap());
        Self {
            texture,
            view,
            bytes,
            size,
        }
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_source_pipeline;

#[cfg(test)]
mod ambient_shader_contract;
pub mod ambient_source_gpu;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_source_pipeline;

pub mod ambient_source_gpu;

pub mod ambient_image;
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
```

## <a id="checkpoint-90j-prepare-reduced-and-full-ambient-targets"></a>21. Prepare reduced and full ambient targets

**What this code does:** Prepare reduced and full ambient targets.

### 1. `src/ambient_images.rs`

Own compatible current ambient targets and exact allocation.

Create the file and type:

```rust
use crate::{ambient_image::Image, pyramid_image::Image as Pyramid};
pub struct Images {
    pub full: [u32; 2],
    pub size: [u32; 2],
    pub source_radius: Image,
    pub source_normals: Image,
    pub linear: Pyramid,
    pub radii: Pyramid,
    pub normals: Image,
    pub occupancy: Image,
    pub ground: Image,
    pub ao: [Image; 3],
    pub history: Image,
    pub history_depth: Image,
}
impl Images {
    pub fn new(device: &wgpu::Device, full: [u32; 2], density: f32) -> Self {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        let image = |size, format| Image::new(device, size, format);
        let source_radius = image(full, wgpu::TextureFormat::R32Float);
        let source_normals = image(full, wgpu::TextureFormat::Rg8Unorm);
        let linear = Pyramid::new(device, size, wgpu::TextureFormat::R32Float);
        let radii = Pyramid::new(device, size, wgpu::TextureFormat::R16Uint);
        let normals = image(size, wgpu::TextureFormat::Rg8Unorm);
        let occupancy = image(size, wgpu::TextureFormat::R8Unorm);
        let ground = image(size, wgpu::TextureFormat::R8Unorm);
        let ao = std::array::from_fn(|i| {
            image(
                if i == 2 {
                    full
                } else {
                    size
                },
                wgpu::TextureFormat::R8Unorm,
            )
        });
        let history = image(size, wgpu::TextureFormat::R8Unorm);
        let history_depth = image(size, wgpu::TextureFormat::R32Float);
        Self {
            full,
            size,
            source_radius,
            source_normals,
            linear,
            radii,
            normals,
            occupancy,
            ground,
            ao,
            history,
            history_depth,
        }
    }
    pub fn bytes(&self) -> u64 {
        self.source_radius.bytes
            + self.source_normals.bytes
            + self.linear.bytes
            + self.radii.bytes
            + self.normals.bytes
            + self.occupancy.bytes
            + self.ground.bytes
            + self.ao.iter().map(|i| i.bytes).sum::<u64>()
            + self.history.bytes
            + self.history_depth.bytes
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_image;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_image;

pub mod ambient_images;
#[cfg(test)]
```

## <a id="checkpoint-90k-build-explicit-ambient-fullscreen-pipelines"></a>22. Build explicit ambient fullscreen pipelines

**What this code does:** Build explicit ambient fullscreen pipelines.

### 1. `src/ambient_pipeline.rs`

Build compatible fullscreen ambient pipelines.

Create the file and type:

```rust
use wgpu::{BufferBindingType as B, ShaderStages as S, TextureSampleType as T};
pub struct Pipelines {
    pub source: wgpu::BindGroupLayout,
    pub level: wgpu::BindGroupLayout,
    pub sample: wgpu::BindGroupLayout,
    pub history: wgpu::BindGroupLayout,
    pub prepare: wgpu::RenderPipeline,
    pub reduce: wgpu::RenderPipeline,
    pub occupancy: wgpu::RenderPipeline,
    pub ground: wgpu::RenderPipeline,
    pub raw: wgpu::RenderPipeline,
    pub filter: [wgpu::RenderPipeline; 2],
    pub upsample: wgpu::RenderPipeline,
    pub composite: wgpu::RenderPipeline,
}
fn pipeline(
    device: &wgpu::Device,
    source: &str,
    fragment: &str,
    groups: &[Option<&wgpu::BindGroupLayout>],
    formats: &[wgpu::TextureFormat],
    blend: bool,
) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("current ambient recipe"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("current ambient recipe"),
        bind_group_layouts: groups,
        immediate_size: 0,
    });
    let targets = formats
        .iter()
        .map(|format| {
            Some(wgpu::ColorTargetState {
                format: *format,
                blend: if blend {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                } else {
                    None
                },
                write_mask: wgpu::ColorWrites::ALL,
            })
        })
        .collect::<Vec<_>>();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("current ambient recipe"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some(fragment),
            compilation_options: Default::default(),
            targets: &targets,
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
impl Pipelines {}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_images;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_images;

pub mod ambient_pipeline;
#[cfg(test)]
```

## <a id="checkpoint-90l-match-current-ambient-sampling-layouts"></a>23. Match current ambient sampling layouts

**What this code does:** Match current ambient sampling layouts.

### 1. `src/ambient_pipeline.rs`

Build each compatible current ambient stage.

**Locate this existing block:**

```rust
}
impl Pipelines {}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
impl Pipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let texture = |binding, sample| {
            crate::text_gpu_support::texture_entry(binding, S::FRAGMENT, sample, false)
        };
        let uniform =
            |binding| crate::tile_pipelines::entry(binding, S::FRAGMENT, B::Uniform);
        let source =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient source images and precise camera"),
                entries: &[
                    texture(0, T::Depth),
                    uniform(1),
                    texture(
                        2,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    texture(
                        3,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    texture(
                        4,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    uniform(5),
                ],
            });
        let level = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient reduced original properties"),
            entries: &[
                texture(
                    0,
                    T::Float {
                        filterable: false,
                    },
                ),
                texture(1, T::Uint),
                texture(
                    2,
                    T::Float {
                        filterable: false,
                    },
                ),
            ],
        });
        let sample =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient filtered samples"),
                entries: &[
                    texture(0, T::Float { filterable: true }),
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: S::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering,
                        ),
                        count: None,
                    },
                ],
            });
        let mut entries = (0..4)
            .map(|i| {
                crate::tile_pipelines::entry(
                    i,
                    S::FRAGMENT,
                    B::Storage { read_only: false },
                )
            })
            .collect::<Vec<_>>();
        entries.push(texture(4, T::Float { filterable: true }));
        entries.push(texture(
            5,
            T::Float {
                filterable: false,
            },
        ));
        let history =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("ambient reserved current history bindings"),
                entries: &entries,
            });
        let shader = crate::clipping_gpu::source(include_str!("ambient.wgsl"), 0, 5);
        let groups = [
            Some(&source),
            Some(&level),
            Some(&sample),
            Some(&history),
        ];
        let prepare = pipeline(
            device,
            &shader,
            "fs_prepare",
            &[Some(&source)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
                wgpu::TextureFormat::Rg8Unorm,
            ],
            false,
        );
        let make = |fragment| {
            pipeline(
                device,
                &shader,
                fragment,
                &groups,
                &[wgpu::TextureFormat::R8Unorm],
                false,
            )
        };
        let ground = make("fs_ground");
        let raw = make("fs_main");
        let filter = [make("fs_filter_x"), make("fs_filter_y")];
        let upsample = make("fs_upsample");
        let composite = pipeline(
            device,
            include_str!("ambient_composite.wgsl"),
            "fs_main",
            &[None, Some(&sample)],
            &[format],
            true,
        );
        let reduce = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
            "fs_reduce",
            &[Some(&level)],
            &[
                wgpu::TextureFormat::R32Float,
                wgpu::TextureFormat::R16Uint,
            ],
            false,
        );
        let occupancy = pipeline(
            device,
            include_str!("ambient_depth.wgsl"),
            "fs_occupancy",
            &[Some(&level)],
            &[wgpu::TextureFormat::R8Unorm],
            false,
        );
        Self {
            source,
            level,
            sample,
            history,
            prepare,
            reduce,
            occupancy,
            ground,
            raw,
            filter,
            upsample,
            composite,
        }
    }
}
```

## <a id="checkpoint-90m-cache-reduced-levels-and-filtered-samples"></a>24. Cache reduced levels and filtered samples

**What this code does:** Cache reduced levels and filtered samples.

### 1. `src/ambient_groups.rs`

Cache actual reduced-level and sample bindings.

Create the file and type:

```rust
pub struct Groups {
    pub levels: [wgpu::BindGroup; 6],
    pub occupancy: wgpu::BindGroup,
    pub ground: wgpu::BindGroup,
    pub ao: [wgpu::BindGroup; 3],
    pub history: wgpu::BindGroup,
    pub buffers: [wgpu::Buffer; 4],
}
fn sampled(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    view: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ambient retained sample"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    })
}
impl Groups {
    pub fn new(
        device: &wgpu::Device,
        pipes: &crate::ambient_pipeline::Pipelines,
        images: &crate::ambient_images::Images,
    ) -> Self {
        let levels = std::array::from_fn(|i| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient retained depth and properties"),
                layout: &pipes.level,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &images.linear.levels[i],
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(
                            &images.radii.levels[i],
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(
                            &images.normals.view,
                        ),
                    },
                ],
            })
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let occupancy = sampled(
            device,
            &pipes.sample,
            &images.occupancy.view,
            &sampler,
        );
        let ground = sampled(
            device,
            &pipes.sample,
            &images.ground.view,
            &sampler,
        );
        let ao = std::array::from_fn(|i| {
            sampled(
                device,
                &pipes.sample,
                &images.ao[i].view,
                &sampler,
            )
        });
        let buffers = std::array::from_fn(|_| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient reserved single-sample work"),
                size: 16,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        });
        let mut entries = buffers
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: b.as_entire_binding(),
            })
            .collect::<Vec<_>>();
        entries.push(wgpu::BindGroupEntry {
            binding: 4,
            resource: wgpu::BindingResource::TextureView(&images.history.view),
        });
        entries.push(wgpu::BindGroupEntry {
            binding: 5,
            resource: wgpu::BindingResource::TextureView(&images.history_depth.view),
        });
        let history = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient inactive single-sample history"),
            layout: &pipes.history,
            entries: &entries,
        });
        Self {
            levels,
            occupancy,
            ground,
            ao,
            history,
            buffers,
        }
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_pipeline;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_pipeline;

pub mod ambient_groups;
#[cfg(test)]
```

## <a id="checkpoint-90n-bind-exact-current-ambient-source-images"></a>25. Bind exact current ambient source images

**What this code does:** Bind exact current ambient source images.

### 1. `src/ambient_groups.rs`

Bind exact current original inputs and camera resources.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn source(
        device: &wgpu::Device,
        pipes: &crate::ambient_pipeline::Pipelines,
        images: &crate::ambient_images::Images,
        depth: &wgpu::TextureView,
        sections: &wgpu::TextureView,
        frame: &wgpu::Buffer,
        clip: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        let entries = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(depth),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: frame.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(
                    &images.source_radius.view,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(
                    &images.source_normals.view,
                ),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::TextureView(sections),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: clip.as_entire_binding(),
            },
        ];
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient exact current input"),
            layout: &pipes.source,
            entries: &entries,
        })
    }
}
```

## <a id="checkpoint-90o-record-explicit-ambient-passes"></a>26. Record explicit ambient passes

**What this code does:** Record explicit ambient passes.

### 1. `src/ambient_pass.rs`

Record one compatible ambient stage.

Create the file and type:

```rust
pub fn draw(
    encoder: &mut wgpu::CommandEncoder,
    pipeline: &wgpu::RenderPipeline,
    targets: &[&wgpu::TextureView],
    groups: &[(u32, &wgpu::BindGroup)],
    clear: bool,
) {
    let attachments = targets
        .iter()
        .map(|view| {
            Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: wgpu::StoreOp::Store,
                },
            })
        })
        .collect::<Vec<_>>();
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("ambient current recipe"),
        color_attachments: &attachments,
        ..Default::default()
    });
    pass.set_pipeline(pipeline);
    for (index, group) in groups {
        pass.set_bind_group(*index, *group, &[]);
    }
    pass.draw(0..3, 0..1);
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_groups;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_groups;

pub mod ambient_pass;
#[cfg(test)]
```

## <a id="checkpoint-90p-own-cached-ambient-pipelines-and-release-images"></a>27. Own cached ambient pipelines and release images

**What this code does:** Own cached ambient pipelines and release images.

### 1. `src/ambient_gpu.rs`

Own lazy ambient state and cached pipelines.

Create the file and type:

```rust
use crate::{ambient_groups::Groups, ambient_images::Images, ambient_pass::draw};
struct State {
    images: Images,
    groups: Groups,
    input: (
        wgpu::TextureView,
        wgpu::TextureView,
        wgpu::BindGroup,
    ),
}
pub struct Ambient {
    pipes: crate::ambient_pipeline::Pipelines,
    pub sources: crate::ambient_source_gpu::Sources,
    frame: wgpu::Buffer,
    state: Option<State>,
    cached: Option<([f32; 80], [u64; 2])>,
    pub preparations: u64,
    pub writes: u64,
    pub passes: u64,
    pub error: Option<&'static str>,
}
impl Ambient {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            pipes: crate::ambient_pipeline::Pipelines::new(device, format),
            sources: crate::ambient_source_gpu::Sources::new(device),
            frame: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("current ambient camera"),
                size: 320,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            state: None,
            cached: None,
            preparations: 0,
            writes: 0,
            passes: 0,
            error: None,
        }
    }
    pub fn clear(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.state = None;
        self.cached = None;
        self.sources.set(device, queue, &[]);
        self.error = None;
    }
}
```

### 2. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_pass;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_pass;

pub mod ambient_gpu;
#[cfg(test)]
```

## <a id="checkpoint-90q-prepare-changed-ambient-framebuffer-inputs"></a>28. Prepare changed ambient framebuffer inputs

**What this code does:** Prepare changed ambient framebuffer inputs.

### 1. `src/ambient_gpu.rs`

Prepare exact current ambient images and source bindings.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        full: [u32; 2],
        density: f32,
        depth: &wgpu::TextureView,
        sections: &wgpu::TextureView,
        clip: &wgpu::Buffer,
    ) {
        let (w, h) =
            crate::pyramid_size::resolution((full[0], full[1]), f64::from(density));
        let size = [w.max(32), h.max(32)];
        if self
            .state
            .as_ref()
            .is_none_or(|s| s.images.full != full || s.images.size != size)
        {
            let images = Images::new(device, full, density);
            let groups = Groups::new(device, &self.pipes, &images);
            let group = Groups::source(
                device,
                &self.pipes,
                &images,
                depth,
                sections,
                &self.frame,
                clip,
            );
            self.state = Some(State {
                images,
                groups,
                input: (depth.clone(), sections.clone(), group),
            });
            self.cached = None;
            self.preparations += 1;
        }
        let state = self.state.as_mut().unwrap();
        if state.input.0 != *depth || state.input.1 != *sections {
            state.input = (
                depth.clone(),
                sections.clone(),
                Groups::source(
                    device,
                    &self.pipes,
                    &state.images,
                    depth,
                    sections,
                    &self.frame,
                    clip,
                ),
            );
            self.cached = None;
        }
    }
}
```

## <a id="checkpoint-90r-render-a-complete-ambient-result-in-one-frame"></a>29. Render a complete ambient result in one frame

**What this code does:** Render a complete ambient result in one frame.

### 1. `src/ambient_gpu.rs`

Render complete current single-sample ambient recipes with cached inputs.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        color: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view: &wgpu::BindGroup,
        clipping: &wgpu::BindGroup,
        clip: &wgpu::Buffer,
        sections: &wgpu::TextureView,
        meshes: &[crate::gpu_mesh::GpuMesh],
        matrix: [f32; 16],
        full: [u32; 2],
        density: f32,
        ground: [f32; 2],
        revision: [u64; 2],
    ) {
        if ground[1] <= 0.0 {
            self.clear(device, queue);
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
            || full.contains(&0)
            || !density.is_finite()
            || density <= 0.
        {
            self.error = Some("Invalid ambient view");
            return;
        }
        self.prepare(device, full, density, depth, sections, clip);
        let state = self.state.as_ref().unwrap();
        let Some(uniform) =
            crate::ambient_camera::uniform(matrix, full, state.images.size, ground)
        else {
            self.error = Some("Invalid ambient camera inverse");
            return;
        };
        if self.cached != Some((uniform, revision)) {
            queue.write_buffer(
                &self.frame,
                0,
                &uniform
                    .into_iter()
                    .flat_map(f32::to_ne_bytes)
                    .collect::<Vec<_>>(),
            );
            self.writes += 1;
            let images = &state.images;
            let groups = &state.groups;
            let source = &state.input.2;
            self.sources.encode(
                encoder,
                &images.source_radius.view,
                &images.source_normals.view,
                depth,
                view,
                clipping,
                meshes,
            );
            self.passes += 1;
            draw(
                encoder,
                &self.pipes.prepare,
                &[
                    &images.linear.levels[0],
                    &images.radii.levels[0],
                    &images.normals.view,
                ],
                &[(0, source)],
                true,
            );
            for level in 1..6 {
                draw(
                    encoder,
                    &self.pipes.reduce,
                    &[
                        &images.linear.levels[level],
                        &images.radii.levels[level],
                    ],
                    &[(0, &groups.levels[level - 1])],
                    true,
                );
            }
            self.passes += 6;
            draw(
                encoder,
                &self.pipes.occupancy,
                &[&images.occupancy.view],
                &[(0, &groups.levels[0])],
                true,
            );
            draw(
                encoder,
                &self.pipes.ground,
                &[&images.ground.view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.occupancy),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.raw,
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ground),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.filter[0],
                &[&images.ao[1].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.filter[1],
                &[&images.ao[0].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[1]),
                    (3, &groups.history),
                ],
                true,
            );
            draw(
                encoder,
                &self.pipes.upsample,
                &[&images.ao[2].view],
                &[
                    (0, source),
                    (1, &groups.levels[0]),
                    (2, &groups.ao[0]),
                    (3, &groups.history),
                ],
                true,
            );
            self.passes += 6;
            self.cached = Some((uniform, revision));
        }
        draw(
            encoder,
            &self.pipes.composite,
            &[color],
            &[(1, &state.groups.ao[2])],
            false,
        );
        self.passes += 1;
        self.error = None;
    }
}
```

## <a id="checkpoint-90s-count-actual-ambient-resources-and-work"></a>30. Count actual ambient resources and work

**What this code does:** Count actual ambient resources and work.

### 1. `src/ambient_gpu.rs`

Expose exact allocation and explicit ambient work counters.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn usage(&self) -> [u64; 5] {
        [
            self.frame.size()
                + self.sources.bytes()
                + self.state.as_ref().map_or(0, |s| {
                    s.images.bytes()
                        + s.groups
                            .buffers
                            .iter()
                            .map(wgpu::Buffer::size)
                            .sum::<u64>()
                }),
            1,
            self.preparations,
            self.writes,
            self.passes,
        ]
    }
}
```

## <a id="checkpoint-90t-route-original-scene-properties-into-ambient-shading"></a>31. Route scene properties into ambient shading

**What this code does:** Route original scene properties into ambient shading.

### 1. `src/renderer.rs`

Connect current original properties and cached ambient rendering.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    *self.text_error.borrow_mut() = Some(error.to_string());
                }
            }
```

Replace it with:

```rust
                    *self.text_error.borrow_mut() = Some(error.to_string());
                }
            }
        }
        if self.ambient_enabled.get() {
            let clip = self.clipping.borrow();
            if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
                ambient.draw(
                    &self.device,
                    &self.queue,
                    &mut encoder,
                    view,
                    &self.depth,
                    &self.view_group,
                    &clip_group,
                    &clip.buffer,
                    &self.caps.borrow().depth.view,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    self.ambient_rows.ground,
                    [self.pyramid_revision, clip_revision],
                );
            }
```

**Locate this block:**

```rust
        clip.solid = solid;
        self.caps.borrow_mut().enabled = enabled;
    }
    pub fn set_depth_pyramid(&self, enabled: bool) {
        let mut owner = self.pyramid.borrow_mut();
        if enabled {
```

Replace it with:

```rust
        clip.solid = solid;
        self.caps.borrow_mut().enabled = enabled;
    }
    pub fn set_ambient(&self, enabled: bool) {
        if self.ambient_enabled.replace(enabled) == enabled {
            return;
        }
        let mut ambient = self.ambient.borrow_mut();
        if enabled {
            let owner = ambient.get_or_insert_with(|| {
                crate::ambient_gpu::Ambient::new(&self.device, self.format)
            });
            owner.sources.set(
                &self.device,
                &self.queue,
                &self.ambient_rows.specs,
            );
        } else if let Some(owner) = ambient.as_mut() {
            owner.clear(&self.device, &self.queue);
        }
    }
    pub fn ambient_usage(&self) -> [u64; 5] {
        self.ambient
            .borrow()
            .as_ref()
            .map_or([0; 5], crate::ambient_gpu::Ambient::usage)
    }
    pub fn ambient_error(&self) -> Option<&'static str> {
        self.ambient
            .borrow()
            .as_ref()
            .and_then(|owner| owner.error)
    }
    pub fn set_depth_pyramid(&self, enabled: bool) {
        let mut owner = self.pyramid.borrow_mut();
        if enabled {
```

**Locate this block:**

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
```

Replace it with:

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.ambient_rows.set(scene);
        if let Some(ambient) = self.ambient.borrow_mut().as_mut() {
            if self.ambient_enabled.get() {
                ambient.sources.set(
                    &self.device,
                    &self.queue,
                    &self.ambient_rows.specs,
                );
            }
        }
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            pyramid: Default::default(),
            pyramid_revision: 0,
            caps: std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            ambient_enabled: Default::default(),
            ambient: Default::default(),
            ambient_rows: Default::default(),
            pyramid: Default::default(),
            pyramid_revision: 0,
            caps: std::cell::RefCell::new(crate::caps_gpu::Caps::new(&device)),
```

**Locate this block:**

```rust
pub struct Renderer {
    pyramid: std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision: u64,
    caps: std::cell::RefCell<crate::caps_gpu::Caps>,
```

Replace it with:

```rust
pub struct Renderer {
    ambient_enabled: std::cell::Cell<bool>,
    ambient: std::cell::RefCell<Option<crate::ambient_gpu::Ambient>>,
    ambient_rows: crate::ambient_rows::Rows,
    pyramid: std::cell::RefCell<Option<crate::pyramid_gpu::Pyramid>>,
    pyramid_revision: u64,
    caps: std::cell::RefCell<crate::caps_gpu::Caps>,
```

## <a id="checkpoint-90u-enable-arctic-as-a-presentation-change"></a>32. Enable Arctic as a presentation change

**What this code does:** Enable Arctic as a presentation change.

### 1. `src/editor.rs`

Connect current Arctic activation and truthful browser diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.split = None;
        }
        match action {
            Action::Clipping(request) => return self.clipping_request(request),
```

Replace it with:

```rust
            self.split = None;
        }
        match action {
            Action::Arctic(_) => {
                unreachable!("Arctic presentation returned before editor mutation")
            }
            Action::Clipping(request) => return self.clipping_request(request),
```

**Locate this block:**

```rust
            }
            action => action,
        };
        if !matches!(
            &action,
            Action::Zoom(_)
```

Replace it with:

```rust
            }
            action => action,
        };
        if let Action::Arctic(on) = &action {
            self.arctic = on.unwrap_or(!self.arctic);
            self.tool_status = format!(
                "Arctic {}",
                if self.arctic {
                    "On"
                } else {
                    "Off"
                }
            );
            return Ok(Change::View);
        }
        if !matches!(
            &action,
            Action::Zoom(_)
```

**Locate this block:**

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            clipping_enabled: true,
            clipping_solid: true,
            interaction_epoch: 0,
```

Replace it with:

```rust
impl Default for Editor {
    fn default() -> Self {
        Self {
            arctic: false,
            clipping_enabled: true,
            clipping_solid: true,
            interaction_epoch: 0,
```

**Locate this block:**

```rust
}

pub struct Editor {
    pub clipping_enabled: bool,
    pub clipping_solid: bool,
    pub interaction_epoch: u64,
```

Replace it with:

```rust
}

pub struct Editor {
    pub arctic: bool,
    pub clipping_enabled: bool,
    pub clipping_solid: bool,
    pub interaction_epoch: u64,
```

**Locate this block:**

```rust
pub enum Action {
    Clipping(crate::clipping_input::Request),
    SelectLasso,
    LassoPress {
```

Replace it with:

```rust
pub enum Action {
    Arctic(Option<bool>),
    Clipping(crate::clipping_input::Request),
    SelectLasso,
    LassoPress {
```

### 2. `src/commands.rs`

Connect current Arctic activation and truthful browser diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    {
        return Ok(Command::Editor(action));
    }
```

Replace it with:

```rust
    {
        return Ok(Command::Editor(action));
    }
    if name == "Arctic" {
        let on = match &words[count..] {
            [] => None,
            [word] if word.eq_ignore_ascii_case("On") => Some(true),
            [word] if word.eq_ignore_ascii_case("Off") => Some(false),
            _ => return Err("Arctic (On Off)".into()),
        };
        return Ok(Command::Editor(Action::Arctic(on)));
    }
```

**Locate this block:**

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Select By Name", || {
        Action::Query(crate::selection_query::Query::Name(
            String::new(),
```

Replace it with:

```rust
pub const ACTIONS: &[(&str, Factory)] = &[
    ("Arctic", || Action::Arctic(None)),
    ("Select By Name", || {
        Action::Query(crate::selection_query::Query::Name(
            String::new(),
```

### 3. `src/browser.rs`

Connect current Arctic activation and truthful browser diagnostics.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

Replace it with:

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute("data-clipping",&serde_json::json!({"usage":renderer.clipping_usage(),"caps":renderer.cap_usage(),"ambient":renderer.ambient_usage(),"ambient_error":renderer.ambient_error(),"arctic":editor.arctic,"pyramid":renderer.depth_pyramid_usage(),"pyramid_error":renderer.depth_pyramid_error(),"cap_error":renderer.cap_error(),"ink_flags":renderer.clipping_ink_bytes(),"error":renderer.clipping_error(),"rows":crate::clipping_rows::current(&editor.scene).ok().map(|rows|rows.iter().map(|r|serde_json::json!({"id":r.id.value(),"normal":r.plane.normal,"offset":r.plane.offset})).collect::<Vec<_>>())}).to_string())?;
    canvas.set_attribute(
```

**Locate this block:**

```rust
            );
            if let Err(error) =
                renderer.set_grid_on(editor.camera.grid, editor.drawing_plane())
            {
```

Replace it with:

```rust
            );
            renderer.set_ambient(editor.arctic);
            if let Err(error) =
                renderer.set_grid_on(editor.camera.grid, editor.drawing_plane())
            {
```

## <a id="checkpoint-90v-exercise-actual-contacts-cache-and-source-ownership"></a>33. Exercise actual contacts cache and source ownership

**What this code does:** Exercise actual contacts cache and source ownership.

### 1. `src/ambient_gpu_tests.rs`

Copy supplied actual contact/frame/cache/release fixtures.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{renderer, source, wait},
    scene::Scene,
};
fn pixels(r: &super::Renderer, matrix: &[f32; 16]) -> Vec<u8> {
    let texture = r
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual original ambient fixture"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    r.draw(
        &texture.create_view(&Default::default()),
        &Default::default(),
        matrix,
    );
    let read = r
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 1280 * 240,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = r
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280),
                rows_per_image: Some(240),
            },
        },
        texture.size(),
    );
    r.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(r);
    rx.recv().unwrap().unwrap();
    let data = read.slice(..).get_mapped_range().to_vec();
    read.unmap();
    data
}
#[test]
fn actual_ambient_darkens_original_contacts_in_one_frame_caches_and_releases_large_images()
 {
    let mut scene = Scene::empty();
    let owner = scene
        .insert(crate::prepared::PreparedMesh::new(source(false)).unwrap())
        .unwrap();
    let original = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let mut camera = crate::camera::Camera::default();
    camera.isometric();
    camera.aspect = 4. / 3.;
    camera.target = [0., 0., -0.5];
    camera.distance = 3.;
    let matrix = camera.uniform();
    let r = renderer(&scene);
    let plain = pixels(&r, &matrix);
    let uploads = r.stats()[0];
    r.set_ambient(true);
    let shaded = pixels(&r, &matrix);
    assert_eq!(r.ambient_error(), None);
    let usage = r.ambient_usage();
    assert!(usage[0] > 100_000);
    assert_eq!(usage[1], 1);
    assert_eq!(usage[3], 1);
    let second = pixels(&r, &matrix);
    assert_eq!(
        second, shaded,
        "one frame completes ambient shading"
    );
    assert_eq!(&r.ambient_usage()[1..4], &usage[1..4]);
    assert_eq!(r.stats()[0], uploads);
    let darker = plain
        .chunks_exact(4)
        .zip(shaded.chunks_exact(4))
        .filter(|(a, b)| {
            a[..3]
                .iter()
                .zip(&b[..3])
                .any(|(a, b)| a.saturating_sub(*b) > 3)
        })
        .count();
    assert!(
        darker > 100,
        "actual ambient shading: {darker} pixels"
    );
    let shadow = plain
        .chunks_exact(4)
        .zip(shaded.chunks_exact(4))
        .filter(|(a, b)| a[..3] == [255, 255, 255] && b[..3].iter().any(|v| *v < 250))
        .count();
    assert!(
        shadow > 20,
        "actual virtual-ground contacts: {shadow}"
    );
    assert_eq!(scene.objects()[0].id, owner);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    r.set_ambient(false);
    assert_eq!(r.ambient_usage()[0], 320);
    assert_eq!(pixels(&r, &matrix), plain);
    r.set_ambient(true);
    assert_eq!(pixels(&r, &matrix), shaded);
    assert_eq!(
        r.ambient_usage()[1],
        1,
        "style toggles retain compiled pipelines"
    );
    assert_eq!(r.stats()[0], uploads);
}
```

### 2. `src/renderer.rs`

Register actual renderer-local ambient fixtures.

**Locate this existing block:**

```rust
#[path = "pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "pyramid_gpu_tests.rs"]
mod pyramid_gpu_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "ambient_gpu_tests.rs"]
mod ambient_gpu_tests;
```

### 3. `src/arctic_tests.rs`

Copy supplied current presentation and original-history fixtures.

Create the file and type:

```rust
use crate::{
    commands::{Command, parse},
    editor::{Action, Editor},
};
fn command(e: &mut Editor, line: &str) -> Result<crate::editor::Change, &'static str> {
    let action = if let Some(result) = e.tool_line(line) {
        result?
    } else {
        let Command::Editor(action) = parse(line).map_err(|_| "Invalid command")?
        else {
            return Err("Editor command");
        };
        action
    };
    e.apply(action)
}
#[test]
fn arctic_toggle_and_strict_options_preserve_originals_pending_points_annotations_and_redo()
 {
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
    command(&mut e, "Length BoundingBox").unwrap();
    let mark = e.mark.clone();
    assert!(mark.is_some());
    let epoch = e.interaction_epoch;
    command(&mut e, "aRcTiC oN").unwrap();
    assert!(e.arctic);
    assert_eq!(e.mark, mark);
    assert_eq!(e.interaction_epoch, epoch);
    assert_eq!(
        e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        before
    );
    command(&mut e, "Arctic Off").unwrap();
    assert!(!e.arctic);
    command(&mut e, "Arctic").unwrap();
    assert!(e.arctic);
    for line in [
        "Arctic No",
        "Arctic On Off",
        "Arctic On extra",
    ] {
        assert!(command(&mut e, line).is_err());
        assert!(e.arctic);
    }
    command(&mut e, "Line").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    let points = e.draft.as_ref().unwrap().points.clone();
    let epoch = e.interaction_epoch;
    command(&mut e, "Arctic Off").unwrap();
    assert_eq!(e.draft.as_ref().unwrap().points, points);
    assert_eq!(e.interaction_epoch, epoch);
    command(&mut e, "Escape").unwrap();
    e.apply(Action::Redo).unwrap();
    assert_eq!(e.scene.placement(id).unwrap().m, moved.m);
    assert_eq!(
        e.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        before
    );
}
```

### 4. `src/lib.rs`

Register the completed section component.

**Locate this existing block:**

```rust
pub mod ambient_gpu;
#[cfg(test)]
mod ambient_shader_contract;
pub mod finite_project_source;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod ambient_gpu;

#[cfg(test)]
mod ambient_shader_contract;
#[cfg(test)]
mod arctic_tests;
pub mod finite_project_source;
```

## Build & Run

![Recorded visual output for Lesson 18.2 (90v-exercise-actual-contacts-cache-and-source-ownership)](../../screenshots/journey/90v-exercise-actual-contacts-cache-and-source-ownership.png)

```sh
npm --prefix ../session_tests run course -- dependencies 18.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 18.2
npm --prefix ../session_tests run course -- save 18.2
```

Continue with [18.3 · Section-aware outlines and one presentation composite](91-include-visible-section-coverage.md).
