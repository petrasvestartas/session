# 11.1 · Coverage-based screen-space outlines

**Goal:** Check real contours widths reuse picks and cleanup. · **Steps:** 10 · **Source:** [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs) · [engine/gpu/cloud.rs](../../../src/engine/gpu/cloud.rs) · [engine/gpu/lod.rs](../../../src/engine/gpu/lod.rs) · [engine/gpu/splat.rs](../../../src/engine/gpu/splat.rs) · [shaders/splat.wgsl](../../../src/shaders/splat.wgsl) · [shaders/splat_resolve.wgsl](../../../src/shaders/splat_resolve.wgsl) · [app/cloud_query.rs](../../../src/app/cloud_query.rs) (`src/engine/gpu/points.rs,src/shaders/point.wgsl,src/shaders/outline.wgsl`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3×3 Neighborhood: ID_center != ID_neighbor or Δz &gt; τ</text>
  <rect x="86" y="50" width="168" height="104" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <line x1="142" y1="50" x2="142" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="198" y1="50" x2="198" y2="154" stroke="#111111" stroke-width="1"/>
  <line x1="86" y1="85" x2="254" y2="85" stroke="#111111" stroke-width="1"/>
  <line x1="86" y1="120" x2="254" y2="120" stroke="#111111" stroke-width="1"/>
  <rect x="142" y="85" width="56" height="35" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="170" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">id=4</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="114" y="72" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#64748b">id=0</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="226" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#111111">id=4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Point Cloud Quad Splat: r_px = clamp(size / w, r_min, r_max)</text>
  <circle cx="484" cy="96" r="24" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <circle cx="546" cy="84" r="18" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <circle cx="524" cy="128" r="20" fill="#fef08a" stroke="#d97706" stroke-width="2"/>
  <circle cx="626" cy="104" r="30" fill="#fee2e2" stroke="#dc2626" stroke-width="2"/>
  <line x1="626" y1="104" x2="656" y2="104" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="641" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">r</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Silhouette Outlines + Dense Splat Cloud</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="838,142 888,64 938,142" fill="#dbeafe" stroke="#dc2626" stroke-width="3.5"/>
  <circle cx="978" cy="84" r="8" fill="#2563eb"/>
  <circle cx="998" cy="76" r="7" fill="#16a34a"/>
  <circle cx="1016" cy="94" r="8" fill="#d97706"/>
  <circle cx="988" cy="108" r="9" fill="#dc2626"/>
  <circle cx="1012" cy="122" r="7" fill="#2563eb"/>
</svg>
</div>

Outline the visible union of solid coverage at a stable CSS width without outlining internal seams.

## Ownership and data flow

Convert the requested CSS radius to physical pixels and build a bounded sorted tap table. A depth-visible mask represents selected or eligible solid coverage; coarse block maximum and neighbour dilation skip empty work conservatively. Outline alpha derives from the external union, then composites black ink over the loaded scene. Masks, coarse/dilated images, tap buffers and cached alpha have explicit owners and invalidation keys; unchanged camera, geometry, selection, size and samples reuse them.

## The maintained implementation

engine/gpu/surface_outline.rs owns SurfaceOutline, Mask, MaskKey, Table and Alpha. src/shaders/surface_outline.wgsl supplies maintained mask filtering/composition. This is derived presentation: original topology, source IDs and pick eligibility remain independent.

All production paths above are relative to `session_viewer/src`. The implementation below preserves the runnable teaching sequence. Early `src/renderer.rs`, `src/browser.rs` and local WGSL are staged prerequisites; production drawing remains in `engine/gpu`, `engine/pipelines` and `src/shaders`. Later rendering lessons must converge to those maintained paths.

Production source reference: [engine/gpu/surface_outline.rs](../../../src/engine/gpu/surface_outline.rs), [engine/gpu/cloud.rs](../../../src/engine/gpu/cloud.rs), [engine/gpu/lod.rs](../../../src/engine/gpu/lod.rs), [engine/gpu/splat.rs](../../../src/engine/gpu/splat.rs), [shaders/splat.wgsl](../../../src/shaders/splat.wgsl), [shaders/splat_resolve.wgsl](../../../src/shaders/splat_resolve.wgsl), [app/cloud_query.rs](../../../src/app/cloud_query.rs).

## <a id="checkpoint-63-radius"></a>1. Measure outline width in CSS pixels

**What this code does:** Measure outline width in CSS pixels.

### 1. `src/outline_math.rs`

Convert the current production CSS radius and build bounded nearest-first coverage taps.

Create the file and type:

```rust
pub fn radius(size: (u32, u32), css_width: f64) -> f32 {
    let css_radius = 1.6875;
    (css_radius * f64::from(size.0) / css_width.max(1.0)).clamp(1.0, 12.0) as f32
}

pub fn block(radius: f32) -> u32 {
    ((radius + 0.5).ceil() as u32)
        .next_power_of_two()
        .max(4)
}

pub fn taps(radius: f32) -> Vec<[f32; 4]> {
    let extent = (radius + 0.5).ceil() as i32;
    let mut taps = Vec::new();

    for y in -extent..=extent {
        for x in -extent..=extent {
            let distance = ((x * x + y * y) as f32).sqrt();

            if distance >= radius + 0.5 {
                continue;
            }

            let t = (distance - (radius - 0.5)).clamp(0.0, 1.0);
            taps.push([
                x as f32,
                y as f32,
                1.0 - t * t * (3.0 - 2.0 * t),
                distance,
            ]);
        }
    }

    taps.sort_by(|a, b| a[3].total_cmp(&b[3]));
    taps
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(test)]
mod scene_label_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod scene_label_tests;

pub mod outline_math;
```

## <a id="checkpoint-63a-coverage"></a>2. Convolve solid coverage into external contour ink

**What this code does:** Write bounded contour sampling and black-alpha composition.

### 1. `src/outline.wgsl`

Type the bounded contour predicate and separate composition entry points.

Create the file and type:

```wgsl
@group(0) @binding(0) var mask: texture_2d<f32>;
@group(0) @binding(1) var<uniform> radius: vec4<f32>;
@group(0) @binding(2) var coarse: texture_2d<f32>;
@group(1) @binding(0) var selected_mask: texture_2d<f32>;
@group(1) @binding(1) var<uniform> selected_radius: vec4<f32>;
@group(1) @binding(2) var selected_coarse: texture_2d<f32>;

struct Taps {
    count: vec4<u32>,
    at: array<vec4<f32>, 512>,
}

@group(2) @binding(0) var<uniform> taps: Taps;

fn near_any_coverage(coarse_mask: texture_2d<f32>, block: f32, p: vec2<i32>) -> bool {
    return textureLoad(coarse_mask, p / i32(block), 0).r > 0.0;
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let xy = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
}

fn coverage_at(position: vec2<f32>) -> f32 {
    let size = vec2<i32>(textureDimensions(mask));
    let p = vec2<i32>(position);
    let center = textureLoad(mask, p, 0).r;
    let two = radius.y != selected_radius.y;
    let center_two = select(1.0, textureLoad(selected_mask, p, 0).r, two);
    let done = center >= 0.999 || !near_any_coverage(coarse, radius.z, p);
    let done_two = center_two >= 0.999 || !near_any_coverage(selected_coarse, selected_radius.z, p);

    if (done && done_two) {
        return 0.0;
    }

    var first = select(0.0, 1.0, done);
    var second = select(0.0, 1.0, done_two);

    for (var i = 0u; i < taps.count.x; i++) {
        let tap = taps.at[i];

        if (min(first, second) >= tap.z) {
            break;
        }

        let q = p + vec2<i32>(tap.xy);

        if (any(q < vec2<i32>(0)) || any(q >= size)) {
            continue;
        }

        if (!done) {
            first = max(first, textureLoad(mask, q, 0).r * tap.z);
        }

        if (!done_two) {
            second = max(second, textureLoad(selected_mask, q, 0).r * tap.z);
        }
    }

    return max(select(first * (1.0 - center), 0.0, done), select(second * (1.0 - center_two), 0.0, done_two));
}

@fragment
fn fs_alpha(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(coverage_at(position.xy), 0.0, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let alpha = textureLoad(mask, vec2<i32>(position.xy), 0).r;

    if (alpha <= 0.0) {
        discard;
    }

    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
```

### 2. `src/outline_mask.wgsl`

Transform the original solid vertices and write only depth-visible coverage.

Create the file and type:

```wgsl
struct View {transform:mat4x4<f32>,display:vec4<f32>}
struct Object {model:mat4x4<f32>,selection:vec4<u32>,normal_matrix:mat4x4<f32>,face_colour:vec4<f32>}
@group(0) @binding(0) var<uniform> view:View;
@group(1) @binding(0) var<uniform> object:Object;
@vertex fn vertex(@location(0) position:vec3<f32>)->@builtin(position) vec4<f32> {return view.transform*object.model*vec4<f32>(position,1.0);}
@fragment fn fragment()->@location(0) vec4<f32> {return vec4<f32>(1.0);}
```

## <a id="checkpoint-63aa-pool-and-dilate-a-conservative-outline-mask"></a>3. Pool and dilate a conservative outline mask

**What this code does:** Accelerate bounded contour sampling with conservative coarse coverage.

### 1. `src/outline.wgsl`

Pool and dilate coverage without losing a possible contour hit.

**Locate this existing block:**

```wgsl
    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}

@fragment
fn fs_pool(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let block = i32(radius.z);
    let base = vec2<i32>(position.xy) * block;
    var highest = 0.0;

    for (var y = 0; y < block; y++) {
        for (var x = 0; x < block; x++) {
            let q = base + vec2<i32>(x, y);

            if (any(q >= size)) {
                continue;
            }

            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}

@fragment
fn fs_dilate(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = vec2<i32>(textureDimensions(mask));
    let c = vec2<i32>(position.xy);
    var highest = 0.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let q = clamp(c + vec2<i32>(x, y), vec2<i32>(0), size - 1);
            highest = max(highest, textureLoad(mask, q, 0).r);
        }
    }

    return vec4<f32>(highest, 0.0, 0.0, 1.0);
}
```

## <a id="checkpoint-63b-resources"></a>4. Own and count the derived outline textures

**What this code does:** Own and count the derived outline textures.

### 1. `src/outline_gpu.rs`

Own R8 images, frame keys, pipeline state and the reusable fullscreen preparation pass.

Create the file and type:

```rust
use crate::{
    gpu_mesh::GpuMesh,
    text_gpu_support::{GpuCtx, resource_group, texture_entry},
};
use wgpu::{ShaderStages as Stages, TextureSampleType as Sample};

struct Image {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    size: [u32; 2],
}
impl Image {
    fn new(device: &wgpu::Device, size: [u32; 2], label: &str) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
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
        Self {
            texture,
            view,
            size,
        }
    }
    fn bytes(&self) -> u64 {
        self.size[0] as u64 * self.size[1] as u64
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}
struct Masks {
    coverage: Image,
    coarse: Image,
    dilated: Image,
    alpha: Image,
    block: u32,
    pool: wgpu::BindGroup,
    dilate: wgpu::BindGroup,
    search: wgpu::BindGroup,
    composite: wgpu::BindGroup,
}
#[derive(Clone, PartialEq)]
struct Key {
    transform: [f32; 16],
    size: [u32; 2],
    density: u32,
    revision: u64,
}

pub struct Outline {
    mask_pipeline: wgpu::RenderPipeline,
    pool_pipeline: wgpu::RenderPipeline,
    dilate_pipeline: wgpu::RenderPipeline,
    alpha_pipeline: wgpu::RenderPipeline,
    composite_pipeline: wgpu::RenderPipeline,
    pool_layout: wgpu::BindGroupLayout,
    search_layout: wgpu::BindGroupLayout,
    composite_layout: wgpu::BindGroupLayout,
    radius: wgpu::Buffer,
    taps: wgpu::Buffer,
    tap_group: wgpu::BindGroup,
    masks: Option<Masks>,
    key: Option<Key>,
    tap_radius: Option<f32>,
    revision: u64,
    pub rebuilds: u64,
    pub skips: u64,
    pub draws: u64,
}
impl Outline {}
fn raster(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    groups: &[&wgpu::BindGroup],
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("outline preparation"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        ..Default::default()
    });
    pass.set_pipeline(pipeline);
    for (i, group) in groups.iter().enumerate() {
        pass.set_bind_group(i as u32, *group, &[]);
    }
    pass.draw(0..3, 0..1);
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
mod scene_label_tests;

pub mod outline_math;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod scene_label_tests;

pub mod outline_math;

pub mod outline_gpu;
```

## <a id="checkpoint-63c-pipelines"></a>5. Create the mask and fullscreen pipeline layouts

**What this code does:** Create the mask and fullscreen pipeline layouts.

### 1. `src/outline_gpu.rs`

Create compatible mask and fullscreen pipelines without reallocating primary geometry.

**Locate this existing block:**

```rust
    pass.draw(0..3, 0..1);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pass.draw(0..3, 0..1);
}
fn fullscreen(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    groups: &[&wgpu::BindGroupLayout],
    entry: &str,
    format: wgpu::TextureFormat,
    blend: bool,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(entry),
        bind_group_layouts: &groups
            .iter()
            .map(|g| Some(*g))
            .collect::<Vec<_>>(),
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(entry),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(entry),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: blend.then_some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
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
        source: wgpu::ShaderSource::Wgsl(include_str!("outline_mask.wgsl").into()),
    });
    let view = crate::gpu_layout::uniform(device, 80);
    let model = crate::gpu_layout::uniform(device, 160);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("selected mask"),
        bind_group_layouts: &[Some(&view), Some(&model)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("selected mask"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vertex"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 24,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0=>Float32x3],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fragment"),
            targets: &[Some(wgpu::TextureFormat::R8Unorm.into())],
            compilation_options: Default::default(),
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: crate::depth::FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::LessEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

## <a id="checkpoint-63d-bindings"></a>6. Build bounded mask and tap bindings

**What this code does:** Build bounded mask and tap bindings.

### 1. `src/outline_gpu.rs`

Build explicit current shader bindings and fixed parameter capacity.

**Locate this existing block:**

```rust
}
impl Outline {}
fn raster(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
impl Outline {
    pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
        let texture = |binding| {
            texture_entry(
                binding,
                Stages::FRAGMENT,
                Sample::Float {
                    filterable: false,
                },
                false,
            )
        };
        let uniform = |binding, bytes| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: Stages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(bytes),
            },
            count: None,
        };
        let layout = |label, entries: &[wgpu::BindGroupLayoutEntry]| {
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(label),
                    entries,
                })
        };
        let pool_layout = layout(
            "outline pooling",
            &[texture(0), uniform(1, 16)],
        );
        let search_layout = layout(
            "outline mask search",
            &[texture(0), uniform(1, 16), texture(2)],
        );
        let tap_layout = layout(
            "outline sorted taps",
            &[uniform(0, 16 + 16 * 512)],
        );
        let composite_layout = layout("outline alpha", &[texture(0)]);
        let buffer = |label, size| {
            ctx.device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
        };
        let radius = buffer("outline radius", 16);
        let taps = buffer("outline sorted taps", 16 + 16 * 512);
        let tap_group = resource_group(
            ctx,
            &tap_layout,
            "outline sorted taps",
            [(0, taps.as_entire_binding())],
        );
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("current surface outline"),
                source: wgpu::ShaderSource::Wgsl(include_str!("outline.wgsl").into()),
            });
        Self {
            mask_pipeline: mask_pipeline(&ctx.device),
            pool_pipeline: fullscreen(
                &ctx.device,
                &shader,
                &[&pool_layout],
                "fs_pool",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            dilate_pipeline: fullscreen(
                &ctx.device,
                &shader,
                &[&pool_layout],
                "fs_dilate",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            alpha_pipeline: fullscreen(
                &ctx.device,
                &shader,
                &[&search_layout, &search_layout, &tap_layout],
                "fs_alpha",
                wgpu::TextureFormat::R8Unorm,
                false,
            ),
            composite_pipeline: fullscreen(
                &ctx.device,
                &shader,
                &[&composite_layout],
                "fs_main",
                format,
                true,
            ),
            pool_layout,
            search_layout,
            composite_layout,
            radius,
            taps,
            tap_group,
            masks: None,
            key: None,
            tap_radius: None,
            revision: 1,
            rebuilds: 0,
            skips: 0,
            draws: 0,
        }
    }
    pub fn invalidate(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn release(&mut self) {
        self.masks = None;
        self.key = None;
    }
}
fn raster(
```

## <a id="checkpoint-63e-reuse"></a>7. Reuse masks while camera selection and density stay unchanged

**What this code does:** Reuse masks while camera selection and density stay unchanged.

### 1. `src/outline_gpu.rs`

Prepare exact-size derived targets, bounded tap bytes and truthful resource counters.

**Locate this existing block:**

```rust
        self.key = None;
    }
}
fn raster(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.key = None;
    }
    pub fn usage(&self) -> [u64; 4] {
        self.masks.as_ref().map_or(
            [
                self.radius.size() + self.taps.size(),
                self.rebuilds,
                self.skips,
                self.draws,
            ],
            |m| {
                [
                    m.coverage.bytes()
                        + m.coarse.bytes()
                        + m.dilated.bytes()
                        + m.alpha.bytes()
                        + self.radius.size()
                        + self.taps.size(),
                    self.rebuilds,
                    self.skips,
                    self.draws,
                ]
            },
        )
    }
    fn prepare(&mut self, ctx: &GpuCtx, size: [u32; 2], density: f32) {
        let radius = crate::outline_math::radius(
            (size[0], size[1]),
            size[0] as f64 / density as f64,
        );
        let block = crate::outline_math::block(radius);
        if self
            .masks
            .as_ref()
            .is_none_or(|m| m.coverage.size != size || m.block != block)
        {
            let coverage = Image::new(&ctx.device, size, "selected coverage");
            let coarse_size = size.map(|n| n.div_ceil(block).max(1));
            let coarse = Image::new(
                &ctx.device,
                coarse_size,
                "selected coarse mask",
            );
            let dilated = Image::new(
                &ctx.device,
                coarse_size,
                "selected dilated mask",
            );
            let alpha = Image::new(&ctx.device, size, "selected outline alpha");
            let pool = resource_group(
                ctx,
                &self.pool_layout,
                "selected mask pooling",
                [
                    (
                        0,
                        wgpu::BindingResource::TextureView(&coverage.view),
                    ),
                    (1, self.radius.as_entire_binding()),
                ],
            );
            let dilate = resource_group(
                ctx,
                &self.pool_layout,
                "selected coarse dilation",
                [
                    (
                        0,
                        wgpu::BindingResource::TextureView(&coarse.view),
                    ),
                    (1, self.radius.as_entire_binding()),
                ],
            );
            let search = resource_group(
                ctx,
                &self.search_layout,
                "selected outline search",
                [
                    (
                        0,
                        wgpu::BindingResource::TextureView(&coverage.view),
                    ),
                    (1, self.radius.as_entire_binding()),
                    (
                        2,
                        wgpu::BindingResource::TextureView(&dilated.view),
                    ),
                ],
            );
            let composite = resource_group(
                ctx,
                &self.composite_layout,
                "selected outline alpha",
                [(
                    0,
                    wgpu::BindingResource::TextureView(&alpha.view),
                )],
            );
            self.masks = Some(Masks {
                coverage,
                coarse,
                dilated,
                alpha,
                block,
                pool,
                dilate,
                search,
                composite,
            });
            self.key = None;
        }
        ctx.queue.write_buffer(
            &self.radius,
            0,
            &[radius, 1., block as f32, 0.]
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect::<Vec<_>>(),
        );
        if self.tap_radius != Some(radius) {
            let table = crate::outline_math::taps(radius);
            assert!(table.len() <= 512);
            let mut bytes = vec![0u8; 16 + 16 * 512];
            bytes[..4].copy_from_slice(&(table.len() as u32).to_ne_bytes());
            for (i, tap) in table.iter().enumerate() {
                for (j, value) in tap.iter().enumerate() {
                    let offset = 16 + 16 * i + 4 * j;
                    bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
                }
            }
            ctx.queue.write_buffer(&self.taps, 0, &bytes);
            self.tap_radius = Some(radius);
        }
    }
}
fn raster(
```

## <a id="checkpoint-63f-union"></a>8. Outline the visible union without internal seams

**What this code does:** Outline the visible union without internal seams.

### 1. `src/gpu_mesh.rs`

Read whole-selection state from the existing uploaded object uniform.

**Locate this existing block:**

```rust
    }

    pub fn settings_bytes(&self) -> u64 {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn selected(&self) -> bool {
        u32::from_ne_bytes(self.previous[64..68].try_into().unwrap()) != 0
    }

    pub fn settings_bytes(&self) -> u64 {
```

### 2. `src/outline_gpu.rs`

Encode visible selected coverage, cached alpha and a black compositor after geometry ink.

**Locate this existing block:**

```rust
        }
    }
}
fn raster(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }
    pub fn encode(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        view_group: &wgpu::BindGroup,
        meshes: &[GpuMesh],
        transform: [f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        if !meshes.iter().any(GpuMesh::selected) {
            self.release();
            return;
        }
        self.prepare(ctx, size, density);
        let key = Key {
            transform,
            size,
            density: density.to_bits(),
            revision: self.revision,
        };
        let m = self.masks.as_ref().unwrap();
        if self.key.as_ref() != Some(&key) {
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("selected visible coverage"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &m.coverage.view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: depth,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        },
                    ),
                    ..Default::default()
                });
                pass.set_pipeline(&self.mask_pipeline);
                pass.set_bind_group(0, view_group, &[]);
                for mesh in meshes.iter().filter(|m| m.selected()) {
                    mesh.draw(&mut pass);
                }
            }
            raster(
                encoder,
                &m.coarse.view,
                &self.pool_pipeline,
                &[&m.pool],
            );
            raster(
                encoder,
                &m.dilated.view,
                &self.dilate_pipeline,
                &[&m.dilate],
            );
            raster(
                encoder,
                &m.alpha.view,
                &self.alpha_pipeline,
                &[&m.search, &m.search, &self.tap_group],
            );
            self.key = Some(key);
            self.rebuilds += 1;
        } else {
            self.skips += 1;
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("selected black silhouette"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: colour,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.composite_pipeline);
            pass.set_bind_group(0, &m.composite, &[]);
            pass.draw(0..3, 0..1);
        }
        self.draws += 1;
    }
}
fn raster(
```

## <a id="checkpoint-63g-composite"></a>9. Connect a lazy silhouette owner to the renderer

**What this code does:** Connect a lazy silhouette owner to the renderer.

### 1. `src/renderer.rs`

Connect whole-object silhouette invalidation, lazy compositing, current allocation counters and immediate owner release.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
```

Replace it with:

```rust
            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        if self.meshes.iter().any(GpuMesh::selected) {
            let mut outline = self.outline.borrow_mut();
            let outline = outline.get_or_insert_with(|| {
                crate::outline_gpu::Outline::new(&self.text_context(), self.format)
            });
            outline.encode(
                &self.text_context(),
                &mut encoder,
                view,
                &self.depth,
                &self.view_group,
                &self.meshes,
                *transform,
                self.size,
                self.density,
            );
        } else {
            self.outline.borrow_mut().take();
        }
```

**Locate this block:**

```rust
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

Replace it with:

```rust
        self.set_text(vec![crate::text_example::label()])
    }

    pub fn outline_usage(&self) -> [u64; 4] {
        self.outline
            .borrow()
            .as_ref()
            .map_or([0; 4], |outline| outline.usage())
    }

    pub fn highlight_bytes(&self) -> u64 {
        self.highlight.bytes()
    }
```

**Locate this block:**

```rust
                self.settings_bytes += bytes;
            }
        }
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
```

Replace it with:

```rust
                self.settings_bytes += bytes;
            }
        }
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if !ids.is_empty() {
            self.highlight
                .objects(&self.device, scene, ids)?;
```

**Locate this block:**

```rust
        scene: &Scene,
        ids: &[ObjectId],
    ) -> Result<(), &'static str> {
        for mesh in &mut self.meshes {
            if let Some(row) = scene
                .objects()
```

Replace it with:

```rust
        scene: &Scene,
        ids: &[ObjectId],
    ) -> Result<(), &'static str> {
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        for mesh in &mut self.meshes {
            if let Some(row) = scene
                .objects()
```

**Locate this block:**

```rust
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
    ) -> Result<(), &'static str> {
        self.highlight.set(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
```

Replace it with:

```rust
        scene: &Scene,
        target: Option<crate::pick_sources::Target>,
    ) -> Result<(), &'static str> {
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        self.highlight.set(
            &self.device,
            &self.pipeline.get_bind_group_layout(1),
```

**Locate this block:**

```rust
        }
        drop(old);
        self.geometry.prune();
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
```

Replace it with:

```rust
        }
        drop(old);
        self.geometry.prune();
        if !self.meshes.iter().any(GpuMesh::selected) {
            self.outline.borrow_mut().take();
        }
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
```

**Locate this block:**

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.picker.cancel();
        self.faces.clear();
        self.preview
```

Replace it with:

```rust
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
            outline.invalidate();
        }
        self.picker.cancel();
        self.faces.clear();
        self.preview
```

**Locate this block:**

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            full_text_fonts: Default::default(),
            presentation_text: Default::default(),
            scene_text: Default::default(),
```

Replace it with:

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            outline: Default::default(),
            full_text_fonts: Default::default(),
            presentation_text: Default::default(),
            scene_text: Default::default(),
```

**Locate this block:**

```rust
pub struct Renderer {
    text: std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error: std::cell::RefCell<Option<String>>,
    full_text_fonts: std::cell::RefCell<Option<Vec<Vec<u8>>>>,
```

Replace it with:

```rust
pub struct Renderer {
    outline: std::cell::RefCell<Option<crate::outline_gpu::Outline>>,
    text: std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error: std::cell::RefCell<Option<String>>,
    full_text_fonts: std::cell::RefCell<Option<Vec<Vec<u8>>>>,
```

## <a id="checkpoint-63h-outline"></a>10. Check real contour widths, reuse, picks, and cleanup

**What this code does:** Check real contours widths reuse picks and cleanup.

### 1. `src/outline_gpu.rs`

Copy the test-only actual-alpha readback accessor.

**Locate this existing block:**

```rust
        }),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[cfg(test)]
impl Outline {
    pub(crate) fn alpha_texture(&self) -> Option<&wgpu::Texture> {
        self.masks.as_ref().map(|m| &m.alpha.texture)
    }
}
```

### 2. `src/renderer.rs`

Copy test-only R8 alpha readback with bounded row pitch and wait.

**Locate this existing block:**

```rust
    ]
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ]
}

#[cfg(all(test, not(target_arch = "wasm32")))]
impl Renderer {
    pub fn outline_alpha(&self) -> Vec<u8> {
        let owner = self.outline.borrow();
        let Some(outline) = owner.as_ref() else {
            return Vec::new();
        };
        let Some(texture) = outline.alpha_texture() else {
            return Vec::new();
        };
        let [width, height] = self.size;
        let pitch = width.div_ceil(256) * 256;
        let buffer = self
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("actual outline alpha readback"),
                size: pitch as u64 * height as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
        let mut encoder = self
            .device
            .create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(pitch),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| {
                tx.send(r).unwrap()
            });
        self.device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        rx.recv().unwrap().unwrap();
        let bytes = buffer.slice(..).get_mapped_range();
        let pixels = bytes
            .chunks_exact(pitch as usize)
            .flat_map(|row| row[..width as usize].iter().copied())
            .collect();
        drop(bytes);
        buffer.unmap();
        pixels
    }
}
```

### 3. `src/outline_math_tests.rs`

Copy the current production radius, tap capacity and order regressions.

Create the file and type:

```rust
use crate::outline_math::*;
#[test]
fn current_outline_radius_and_sorted_taps_keep_css_width_independent_of_geometry() {
    for density in [1.0_f32, 1.25, 1.5, 2., 3., 4., 8., 20.] {
        let pixels = (800. * density) as u32;
        let r = radius((pixels, 600), 800.);
        assert_eq!(r, (1.6875 * density).clamp(1., 12.));
        let table = taps(r);
        assert!(table.len() <= 512);
        assert_eq!(table[0], [0., 0., 1., 0.]);
        assert!(
            table
                .windows(2)
                .all(|p| p[0][3] <= p[1][3] && p[0][2] >= p[1][2])
        );
        for tap in table {
            assert!(tap[2] > 0. && tap[2] <= 1.);
            assert!(tap[3] < r + 0.5);
            assert!(tap[0].abs() < block(r) as f32 && tap[1].abs() < block(r) as f32);
        }
    }
}
```

### 4. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod outline_math;

pub mod outline_gpu;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod outline_math;

pub mod outline_gpu;

#[cfg(test)]
mod outline_math_tests;
```

### 5. `src/pick_gpu_tests.rs`

Copy actual joined-solid, occlusion, curved/flat density, bore-contour, source/pick/reuse/release regressions.

**Locate this existing block:**

```rust
        .request_pick([0.125, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .request_pick([0.125, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}

#[test]
fn actual_selected_silhouette_has_one_union_no_internal_diagonal_and_keeps_source_and_picks()
 {
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = |x: f64| {
        let mut source = session_rust::Mesh::create_box(0.6, 0.8, 0.1);
        source.set_objectcolor(session_rust::Color::new(0.5, 0.5, 0.5, 1.));
        let mut prepared = PreparedMesh::new(source).unwrap();
        prepared.model = session_rust::Xform::translation(x, 0., 0.5);
        prepared
    };
    let a = scene.insert(mesh(-0.3)).unwrap();
    let b = scene.insert(mesh(0.3)).unwrap();
    let sources = scene
        .objects()
        .iter()
        .map(|row| row.geometry().unwrap().to_proto())
        .collect::<Vec<_>>();
    let mut renderer = renderer(&scene);
    let geometry = renderer.stats();
    renderer
        .select_objects(&scene, &[a, b])
        .unwrap();
    let first = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert_eq!(alpha.len(), 320 * 240);
    assert!(alpha.iter().filter(|v| **v > 0).count() > 300);
    for y in 90..150 {
        for x in 90..230 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The combined coverage has no outline along the shared object join or tessellation diagonals"
            );
        }
    }
    let stats = renderer.outline_usage();
    assert_eq!(colour_pixels(&mut renderer), first);
    assert_eq!(renderer.outline_alpha(), alpha);
    assert_eq!(renderer.outline_usage()[1], stats[1]);
    assert!(renderer.outline_usage()[2] > stats[2]);
    assert_eq!(&renderer.stats()[..2], &geometry[..2]);
    assert_eq!(
        scene
            .objects()
            .iter()
            .map(|row| row.geometry().unwrap().to_proto())
            .collect::<Vec<_>>(),
        sources
    );
    renderer
        .request_pick([-0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], a.value());
    renderer
        .request_pick([0.3, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], b.value());
    renderer.select_objects(&scene, &[]).unwrap();
    colour_pixels(&mut renderer);
    assert!(renderer.outline_alpha().is_empty());
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_surface_outline_culls_occluded_selection_and_selected_edge_ink_cannot_narrow_its_alpha()
 {
    use crate::pick_sources::{Part, Target};
    let mut scene = Scene::demo();
    scene.clear();
    let back = scene.insert(triangle(0.6)).unwrap();
    let front = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[back])
        .unwrap();
    colour_pixels(&mut renderer);
    assert!(
        renderer
            .outline_alpha()
            .iter()
            .all(|v| *v == 0),
        "A selected surface wholly behind the current opaque depth adds no silhouette"
    );
    renderer
        .select_objects(&scene, &[front])
        .unwrap();
    let plain = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    assert!(alpha.iter().any(|v| *v > 128));
    let edge = scene
        .objects()
        .iter()
        .find(|row| row.id == front)
        .unwrap()
        .mesh
        .parts
        .edges[0];
    renderer
        .set_highlight(
            &scene,
            Some(Target {
                owner: front,
                part: Part::MeshEdge(edge.0, edge.1),
            }),
        )
        .unwrap();
    let inked = colour_pixels(&mut renderer);
    assert_eq!(
        renderer.outline_alpha(),
        alpha,
        "Source edge highlighting cannot change the actual outline width"
    );
    let mut opaque = 0;
    for ((a, b), coverage) in plain
        .chunks_exact(4)
        .zip(inked.chunks_exact(4))
        .zip(&alpha)
    {
        if *coverage >= 250 && a[..3].iter().all(|c| *c < 8) {
            opaque += 1;
            assert!(
                b[..3].iter().all(|c| *c < 12),
                "Yellow source edge ink cannot paint over black outline pixels"
            );
        }
    }
    assert!(opaque > 100);
    scene.clear();
    renderer.set_scene(&scene, None);
    colour_pixels(&mut renderer);
    assert_eq!(renderer.outline_usage()[0], 0);
}

#[test]
fn actual_curved_and_flat_silhouettes_keep_the_same_css_width_at_two_densities() {
    let mut widths = Vec::new();
    for curved in [false, true] {
        let mut scene = Scene::demo();
        scene.clear();
        let id = if curved {
            scene
                .insert_cad(Rc::new(session_rust::BRep::create_cylinder(
                    0.4, 0.8,
                )))
                .unwrap()
        } else {
            let mut prepared =
                PreparedMesh::new(session_rust::Mesh::create_box(0.8, 0.8, 0.2))
                    .unwrap();
            prepared.model = session_rust::Xform::translation(0., 0., 0.5);
            scene.insert(prepared).unwrap()
        };
        let mut renderer = renderer(&scene);
        renderer
            .select_objects(&scene, &[id])
            .unwrap();
        let geometry = renderer.stats();
        let mut css = Vec::new();
        for density in [1., 2.] {
            renderer.set_density(density);
            colour_pixels(&mut renderer);
            let alpha = renderer.outline_alpha();
            assert!(alpha.iter().any(|v| *v == 255));
            let row = &alpha[120 * 320..121 * 320];
            let width = row[160..]
                .iter()
                .map(|v| *v as f64 / 255.)
                .sum::<f64>()
                / density as f64;
            assert!(
                (0.7..1.7).contains(&width),
                "Actual outer silhouette CSS coverage remains bounded: {width}"
            );
            css.push(width);
            assert_eq!(&renderer.stats()[..2], &geometry[..2]);
        }
        assert!(
            (css[0] - css[1]).abs() < 0.5,
            "DPI keeps the outline in CSS units: {css:?}"
        );
        widths.push(css);
    }
    for i in 0..2 {
        assert!(
            (widths[0][i] - widths[1][i]).abs() < 0.25,
            "A smooth cylinder and flat box use one coverage width: {widths:?}"
        );
    }
}

#[test]
fn actual_selected_bore_keeps_its_open_center_and_black_inner_contour() {
    let source = Rc::new(session_rust::BRep::create_block_with_hole(
        1.2, 1.2, 0.4, 0.25,
    ));
    let _ = source.guid();
    let original = source.to_proto();
    let mut scene = Scene::demo();
    scene.clear();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let pixels = colour_pixels(&mut renderer);
    let alpha = renderer.outline_alpha();
    for y in 112..128 {
        for x in 152..168 {
            assert_eq!(
                alpha[y * 320 + x],
                0,
                "The bore center stays open"
            );
        }
    }
    let mut inner = 0;
    for y in 80..160 {
        for x in 110..210 {
            let i = y * 320 + x;
            if alpha[i] >= 250 {
                inner += 1;
                assert!(
                    pixels[i * 4..i * 4 + 3]
                        .iter()
                        .all(|c| *c < 12),
                    "The actual inner silhouette is black"
                );
            }
        }
    }
    assert!(
        inner > 100,
        "The real bore boundary has an inner outline: {inner}"
    );
    renderer
        .request_pick([0., 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
    renderer
        .request_pick([0.4, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    assert_eq!(
        source.to_proto(),
        original,
        "Coverage cannot change original topology"
    );
}
```

### 6. `src/browser.rs`

Expose actual outline allocation, rebuild, skip and compositor draw counters.

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-scene-text-stats",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-surface-outline",
        &serde_json::to_string(&renderer.outline_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-scene-text-stats",
```

## Build & Run

![Recorded visual output for Lesson 11.1 (63h-outline)](../../screenshots/journey/63h-outline.png)

```sh
npm --prefix ../session_tests run course -- dependencies 11.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 11.1
npm --prefix ../session_tests run course -- save 11.1
```

Continue with [11.2 · Validated original cloud columns and placed owners](64-nodes.md).
