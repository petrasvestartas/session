# 22.4 · Original ambient inputs and real pass ordering

**Goal:** Match the displayed contact depth and original visual radius without adding GPU resources. · **Steps:** 3 · **Source:** [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4× MSAA Subpixel Samples (s0..s3) Inside Pixel Square</text>
  <rect x="108" y="50" width="108" height="104" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="116" y1="146" x2="208" y2="56" stroke="#2563eb" stroke-width="2.5"/>
  <polygon points="116,146 208,56 216,56 216,154 116,154" fill="#dbeafe" fill-opacity="0.65"/>
  <circle cx="142" cy="76" r="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="188" cy="68" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="136" cy="124" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="182" cy="116" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="106" font-family="monospace" font-size="10" font-weight="700" fill="#111111">cov = 3/4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Key + Fill + Rim Rig + Ground Plane Major/Minor Grid</text>
  <polygon points="432,140 492,98 688,98 628,140" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="462" y1="119" x2="658" y2="119" stroke="#dc2626" stroke-width="1.75"/>
  <line x1="560" y1="98" x2="500" y2="140" stroke="#16a34a" stroke-width="1.75"/>
  <polygon points="520,122 560,62 600,122" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <ellipse cx="560" cy="126" rx="34" ry="8" fill="#64748b" fill-opacity="0.45"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Anti-Aliased CAD Viewport + Ground Grid &amp; Contact Shadow</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="826,144 876,104 1042,104 992,144" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1.2"/>
  <ellipse cx="934" cy="130" rx="36" ry="8" fill="#94a3b8" fill-opacity="0.5"/>
  <polygon points="894,126 934,58 974,126" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
</svg>
</div>

## <a id="checkpoint-96zzzzzzr-share-geometric-ambient-inputs"></a>1. Share geometric ambient inputs

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Camera / View Uniform (16-Byte Aligned)</text>
  <rect x="52" y="48" width="268" height="106" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="66" y="58" width="240" height="38" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="81" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">view_proj: [[f32; 4]; 4] (64B)</text>
  <rect x="66" y="104" width="240" height="36" fill="#fef08a" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye_pos / viewport_px (16B)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_buffer -&gt; wgpu::BindGroup (@group(0))</text>
  <rect x="424" y="50" width="272" height="102" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="442" y="64" width="236" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">BufferUsages::UNIFORM | COPY_DST</text>
  <rect x="442" y="106" width="236" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">pass.set_bind_group(0, &amp;bg, &amp;[])</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Shader Uniform Read: var&lt;uniform&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@group(0) @binding(0)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;uniform&gt; u: Uniforms;</text>
  <rect x="828" y="114" width="212" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">u.view_proj * vec4(p, 1.0)</text>
</svg>
</div>

Shade contacts from actual face geometry while preserving authored lighting normals and one contact-radius owner.

### 1. `src/ambient_sources.wgsl`

Read both source paths from the existing object record instead of a separate radius uniform. Ambient contacts use geometric normals; lighting normals remain in the surface shading path. Use one radius owner and avoid decoding or interpolating a lighting normal for the ambient source pass. A face normal follows the rendered geometry even when the source provides a different lighting normal.

**Locate this existing block:**

```wgsl
@group(3) @binding(0) var<uniform> ambient_radius:vec4<f32>;
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,normal:vec3<f32>,object:ObjectSettings,radius:f32)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z,radius);
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex {return ambient_vertex(position,oct32_decode(normal),object_at(0u),ambient_radius.x);}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.normal,object,bitcast<f32>(object.range.z));}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(select(face,-face,front),in.normal,dot(in.normal,in.normal)>0.));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,object:ObjectSettings)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,object.selection.z,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>)->Vertex {return ambient_vertex(position,object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,object);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
struct Source{@location(0) radius:f32,@location(1) normal:vec2<f32>}
@fragment fn fs_main(in:Vertex,@builtin(front_facing) front:bool)->Source{
 let face=cross(dpdx(in.world),dpdy(in.world));if((in.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(in.flags,in.world)){discard;}
 let normal=normalize(select(face,-face,front));
 return Source(in.radius,encode_normal(normal)*0.5+0.5);
```

### 2. `src/ambient_source_gpu.rs`

Remove the redundant per-object property cache. The source owner retains only its two existing pipelines. No per-object radius buffers or bind groups are allocated. The existing object records already contain each placed contact radius. Both actual storage and uniform object paths use the same retained radius, without a second per-mesh binding. There is no separate ambient source buffer allocation to report.

**Locate this existing block:**

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
        records: Option<&crate::instance_gpu::Lane>,
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
                if records.is_some_and(|rows| rows.bind_record(&mut pass, mesh)) {
                    pass.set_pipeline(&self.pipes.record_draw);
                    mesh.geometry.draw_pulled(&mut pass, 1);
                } else {
                    pass.set_pipeline(&self.pipes.draw);
                    mesh.bind_model(&mut pass);
                    pass.set_bind_group(3, group, &[]);
                    mesh.geometry.draw(&mut pass);
                }
                self.draws += 1;
            }
        }
    }
    pub fn bytes(&self) -> u64 {
        self.rows.values().map(|r| r.0.size()).sum()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Sources {
    pipes: crate::ambient_source_pipeline::Pipeline,
}
impl Sources {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            pipes: crate::ambient_source_pipeline::Pipeline::new(device),
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
        records: Option<&crate::instance_gpu::Lane>,
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
            if records.is_some_and(|rows| rows.bind_record(&mut pass, mesh)) {
                pass.set_pipeline(&self.pipes.record_draw);
                mesh.geometry.draw_pulled(&mut pass, 1);
            } else {
                pass.set_pipeline(&self.pipes.draw);
                mesh.bind_model(&mut pass);
                mesh.geometry.draw(&mut pass);
            }
        }
    }
```

### 3. `src/ambient_source_pipeline.rs`

Keep only the actual render pipelines. Remove the unused radius layout. Shared vertex pulling still needs its geometry storage group; the uniform object path needs only the first three groups. The geometric source pass reads positions only. Cache the existing source pipelines without a radius layout owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Self {
            draw,
            record_draw,
            radius,
        }
```

Replace it with:

```rust
        Self {
            draw,
            record_draw,
        }
```

**Locate this block:**

```rust
                    crate::clipping_gpu::source(&source, 2, 0).into(),
                ),
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ambient original normals and radii"),
                layout: Some(&layout),
```

Replace it with:

```rust
                    crate::clipping_gpu::source(&source, 2, 0).into(),
                ),
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("ambient original normals and radii"),
                layout: Some(&layout),
```

**Locate this block:**

```rust
            };
            let clip = crate::clipping_gpu::layout(device);
            let geometry = crate::gpu_geometry::GpuGeometry::storage_layout(device);
            let layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("ambient original source metadata"),
                    bind_group_layouts: &[
                        Some(&view),
                        Some(&model),
                        Some(&clip),
                        Some(if shared {
                            &geometry
                        } else {
                            &radius
                        }),
                    ],
                    immediate_size: 0,
                });
            let source = concat!(
```

Replace it with:

```rust
            };
            let clip = crate::clipping_gpu::layout(device);
            let geometry = crate::gpu_geometry::GpuGeometry::storage_layout(device);
            let groups = [
                Some(&view),
                Some(&model),
                Some(&clip),
                Some(&geometry),
            ];
            let layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("ambient original source metadata"),
                    bind_group_layouts: &groups[..if shared {
                        4
                    } else {
                        3
                    }],
                    immediate_size: 0,
                });
            let source = concat!(
```

**Locate this block:**

```rust
    pub record_draw: wgpu::RenderPipeline,
    pub radius: wgpu::BindGroupLayout,
}
impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let radius = crate::gpu_layout::uniform(device, 16);
        let create = |shared| {
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

Replace it with:

```rust
    pub record_draw: wgpu::RenderPipeline,
}
impl Pipeline {
    pub fn new(device: &wgpu::Device) -> Self {
        let create = |shared| {
            let view = crate::surface_view::layout(device);
            let model = if shared {
```

### 4. `src/renderer.rs`

Prepare bounds and object radii once, without maintaining a second property store. Enabling ambient shading reuses the object records already uploaded by set_scene. Disabling ambient shading releases its images without a redundant source reset.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
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
```

Replace it with:

```rust
        }
        let mut ambient = self.ambient.borrow_mut();
        if enabled {
            ambient.get_or_insert_with(|| {
                crate::ambient_gpu::Ambient::new(&self.device, self.format)
            });
        } else if let Some(owner) = ambient.as_mut() {
            owner.clear();
        }
```

**Locate this block:**

```rust
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

Replace it with:

```rust
        self.ambient_rows.set(scene);
        self.pyramid_revision = self.pyramid_revision.wrapping_add(1);
        self.clipping.borrow_mut().set(scene);
        if let Some(outline) = self.outline.borrow_mut().as_mut() {
```

### 5. `src/ambient_gpu.rs`

Only image and camera state need clearing. Empty scenes release ambient images without per-object GPU properties. Report actual retained frame/image/work allocations.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn usage(&self) -> [u64; 5] {
        [
            self.frame.size()
                + self.sources.bytes()
                + self.state.as_ref().map_or(0, |s| {
```

Replace it with:

```rust
    pub fn usage(&self) -> [u64; 5] {
        [
            self.frame.size()
                + self.state.as_ref().map_or(0, |s| {
```

**Locate this block:**

```rust
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if ground[1] <= 0.0 {
            self.clear(device, queue);
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
```

Replace it with:

```rust
        records: Option<&crate::instance_gpu::Lane>,
    ) {
        if ground[1] <= 0.0 {
            self.clear();
            return;
        }
        if matrix.iter().any(|v| !v.is_finite())
```

**Locate this block:**

```rust
    }
    pub fn clear(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.state = None;
        self.cached = None;
        self.sources.set(device, queue, &[]);
        self.error = None;
    }
    fn prepare(
```

Replace it with:

```rust
    }
    pub fn clear(&mut self) {
        self.state = None;
        self.cached = None;
        self.error = None;
    }
    fn prepare(
```

### 6. `src/ambient_input_tests.rs`

Check actual production property readbacks, preserved lighting, both object paths, shared placed radii and source release.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{pixels_at, renderer, wait},
    scene::Scene,
};
use prost::Message;
use session_rust::{Color, Mesh, Session, Xform, proto};
use std::rc::Rc;
fn original() -> Mesh {
    let mut mesh = crate::cap_gpu_tests::source(false);
    mesh.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    for key in mesh.vertices() {
        mesh.set_vertex_attribute(key, "nx", 1.);
        mesh.set_vertex_attribute(key, "ny", 0.);
        mesh.set_vertex_attribute(key, "nz", 0.);
    }
    mesh
}
fn document() -> Vec<u8> {
    let mut doc = Session::new("Original lighting normals and geometric contacts");
    doc.add_mesh(original(), None);
    doc.to_proto().encode_to_vec()
}
fn scene(bytes: &[u8]) -> Scene {
    let mut s = Scene::empty();
    crate::scene_document::import(&mut s, bytes, "", &Xform::identity(), 0.).unwrap();
    s
}
fn snapshot(s: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap())
        .unwrap();
    (
        b.documents
            .iter()
            .map(|v| proto::Session::decode(v.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3.;
    c.radius = 1.;
    c.target = [0., 0., -0.5];
    c.aspect = 4. / 3.;
    c
}
fn texture(g: &super::Renderer, format: wgpu::TextureFormat) -> wgpu::Texture {
    g.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual geometric ambient property readback"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
}
fn read(g: &super::Renderer, t: &wgpu::Texture, pitch: u32) -> Vec<u8> {
    let b = g
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(pitch) * 240,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut e = g
        .device
        .create_command_encoder(&Default::default());
    e.copy_texture_to_buffer(
        t.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &b,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(240),
            },
        },
        t.size(),
    );
    g.queue.submit([e.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    b.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(g);
    rx.recv().unwrap().unwrap();
    let v = b.slice(..).get_mapped_range().to_vec();
    b.unmap();
    v
}
fn pixel(m: [f32; 16], p: [f64; 3]) -> [usize; 2] {
    let clip: [f64; 4] = std::array::from_fn(|i| {
        m[12 + i] as f64
            + (0..3)
                .map(|j| m[j * 4 + i] as f64 * p[j])
                .sum::<f64>()
    });
    [
        ((clip[0] / clip[3] + 1.) * 160.).floor() as usize,
        ((1. - clip[1] / clip[3]) * 120.).floor() as usize,
    ]
}
fn properties(
    g: &super::Renderer,
    m: [f32; 16],
    points: &[[f64; 3]],
) -> Vec<([u8; 2], f32)> {
    let radius = texture(g, wgpu::TextureFormat::R32Float);
    let normal = texture(g, wgpu::TextureFormat::Rg8Unorm);
    let mut source = crate::ambient_source_gpu::Sources::new(&g.device);
    let mut e = g
        .device
        .create_command_encoder(&Default::default());
    source.encode(
        &mut e,
        &radius.create_view(&Default::default()),
        &normal.create_view(&Default::default()),
        &g.depth,
        &g.view_group,
        &g.clipping.borrow().group,
        &g.meshes,
        Some(&g.instances),
    );
    g.queue.submit([e.finish()]);
    let normals = read(g, &normal, 768);
    let radii = read(g, &radius, 1280);
    points
        .iter()
        .map(|&p| {
            let [x, y] = pixel(m, p);
            assert!(x < 320 && y < 240);
            let at = y * 768 + x * 2;
            let n = [normals[at], normals[at + 1]];
            let at = y * 1280 + x * 4;
            (
                n,
                f32::from_ne_bytes(radii[at..at + 4].try_into().unwrap()),
            )
        })
        .collect()
}
fn export(name: &str, s: &Scene, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_AMBIENT_INPUT_PROOF_DIR") {
        use std::io::Write;
        let p = std::path::Path::new(&root);
        std::fs::create_dir_all(p).unwrap();
        let archive = crate::scene_archive::save(s).unwrap();
        let bundle = crate::archive::Bundle::decode(&archive).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        std::fs::write(
            p.join(format!("{name}.pb")),
            &bundle.documents[0],
        )
        .unwrap();
        std::fs::write(p.join(format!("{name}.session")), archive).unwrap();
        let mut f = std::fs::File::create(p.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}
#[test]
fn actual_geometric_normals_preserve_authored_lighting_in_both_object_paths() {
    let s = scene(&document());
    let original = snapshot(&s);
    let owner = s.objects()[0].id;
    assert!(
        s.objects()[0]
            .mesh
            .normals()
            .iter()
            .all(|n| n[0] > 0.99 && n[2].abs() < 0.01)
    );
    let mut g = renderer(&s);
    g.set_opacity(1.);
    g.set_ambient(true);
    let uploads = g.stats()[0];
    let mut c = camera();
    let p = [0.11, 0.13, -0.2];
    for projection in [
        crate::camera::Projection::Orthographic,
        crate::camera::Projection::Perspective,
    ] {
        c.projection = projection;
        let m = c.uniform();
        for density in [1., 2.] {
            g.set_density(density);
            let image = pixels_at(&g, m);
            let sample = properties(&g, m, &[p])[0];
            assert!(
                sample.0.iter().all(|v| v.abs_diff(128) <= 1),
                "Geometric ambient normal must follow the actual top face: {:?}",
                sample.0
            );
            assert_eq!(sample.1, g.ambient_rows.specs[0].1);
            let [x, y] = pixel(m, p);
            let rgb = crate::cap_gpu_tests::at(&image, x, y);
            assert!(
                rgb[..3]
                    .iter()
                    .zip([44u8, 88, 132])
                    .all(|(a, b)| a.abs_diff(b) <= 2),
                "Authored+x lighting remains intact without false geometric self-shadow: {rgb:?}"
            );
            g.set_pick_mode(crate::pick_sources::Mode::Object);
            g.request_pick(
                super::contact_feature_tests::project(m, p),
                m,
            )
            .unwrap();
            wait(&g);
            let picked = g
                .take_pick(m)
                .unwrap()
                .unwrap()
                .and_then(|v| g.resolve_pick(&s, v));
            assert_eq!(
                picked,
                Some(crate::pick_sources::Target {
                    owner,
                    part: crate::pick_sources::Part::Object
                })
            );
            g.set_ambient(false);
            g.use_uniform_surface_records();
            g.set_ambient(true);
            assert!(
                pixels_at(&g, m) == image,
                "Real uniform and storage object paths retain identical whole-frame pixels"
            );
            assert_eq!(properties(&g, m, &[p])[0], sample);
            g.set_scene(&s, None);
            assert!(pixels_at(&g, m) == image);
            assert_eq!(g.stats()[0], uploads);
            assert_eq!(snapshot(&s), original);
            if density == 1. {
                export(
                    if projection == crate::camera::Projection::Orthographic {
                        "authored-normal-top-dpr1"
                    } else {
                        "authored-normal-perspective-dpr1"
                    },
                    &s,
                    &image,
                );
            }
            if density == 2. {
                export(
                    if projection == crate::camera::Projection::Orthographic {
                        "authored-normal-top"
                    } else {
                        "authored-normal-perspective"
                    },
                    &s,
                    &image,
                );
            }
        }
    }
}
#[test]
fn contact_radii_reuse_records_for_shared_placements_and_released_sources() {
    let mesh = original();
    let guid = mesh.guid().to_owned();
    let mut proto = crate::instance_specimen::message();
    proto.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for row in &mut proto.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    proto.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1.4, 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1.4, 0., 0.) * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let bytes = proto.encode_to_vec();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:ambient-inputs".into(),
        )),
    ))
    .unwrap();
    assert_eq!(e.scene.instances.len(), 2);
    let second = e.scene.objects()[1].id;
    assert!(Rc::ptr_eq(
        &e.scene.objects()[0].mesh,
        &e.scene.objects()[1].mesh
    ));
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    Rc::make_mut(&mut e.scene.view).hide(&[second], true);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    g.set_ambient(true);
    let mut c = camera();
    c.distance = 6.;
    c.radius = 3.;
    c.projection = crate::camera::Projection::Orthographic;
    let m = c.uniform();
    pixels_at(&g, m);
    let owned = g.ambient_usage()[0];
    let uploads = g.stats()[0];
    Rc::make_mut(&mut e.scene.view).hide(&[second], false);
    g.set_scene(&e.scene, None);
    let image = pixels_at(&g, m);
    assert_eq!(
        g.ambient_usage()[0],
        owned,
        "Another placed object must not allocate another ambient radius buffer or bind-group owner"
    );
    assert_eq!(g.stats()[0], uploads);
    let original = snapshot(&e.scene);
    let points = e
        .scene
        .objects()
        .iter()
        .map(|r| {
            let p = r
                .model
                .transform_point(&session_rust::Point::new(0.11, 0.13, -0.2));
            [p[0], p[1], p[2]]
        })
        .collect::<Vec<_>>();
    let values = properties(&g, m, &points);
    for (row, (normal, radius)) in e.scene.objects().iter().zip(&values) {
        assert!(
            (normal[0].abs_diff(128) <= 1 && normal[1].abs_diff(128) <= 1)
                || normal.iter().all(|v| *v >= 254),
            "Actual mirrored top normal stays geometric up to winding: {normal:?}"
        );
        assert_eq!(
            *radius,
            g.ambient_rows
                .specs
                .iter()
                .find(|(id, _)| *id == row.id.value())
                .unwrap()
                .1
        );
        assert!(*radius > 0.);
    }
    assert_ne!(
        values[0].1, values[1].1,
        "Nonuniform placements retain their distinct world contact radii"
    );
    export("shared-contact-radii", &e.scene, &image);
    assert_eq!(
        e.apply(crate::editor::Action::UnloadSources)
            .err(),
        Some("No eligible imported sources to unload")
    );
    assert_eq!(snapshot(&e.scene), original);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(properties(&g, m, &points), values);
    drop(e);
    assert!(
        weak.upgrade().is_none(),
        "Discarding definition instances releases their original geometry"
    );
    let bytes = document();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:ambient-original".into(),
        )),
    ))
    .unwrap();
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    let original = snapshot(&e.scene);
    c.distance = 3.;
    c.radius = 1.;
    let m = c.uniform();
    g.set_scene(&e.scene, None);
    let image = pixels_at(&g, m);
    let uploads = g.stats()[0];
    let point = [0.11, 0.13, -0.2];
    let values = properties(&g, m, &[point]);
    e.apply(crate::editor::Action::UnloadSources)
        .unwrap();
    assert!(
        weak.upgrade().is_none(),
        "Ambient inputs retain no eligible original source mesh owner"
    );
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(properties(&g, m, &[point]), values);
    assert_eq!(g.stats()[0], uploads);
    let keys = e.reload_keys();
    e.hydrate(
        keys.into_iter()
            .map(|k| (k, bytes.clone()))
            .collect(),
    )
    .unwrap();
    assert_eq!(snapshot(&e.scene), original);
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == image);
    assert_eq!(g.stats()[0], uploads);
    export("released-original-inputs", &e.scene, &image);
}
```

### 7. `src/renderer.rs`

Register the two actual geometric ambient-input GPU checks.

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "mesh_density_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
#[path = "ambient_input_tests.rs"]
mod ambient_input_tests;
#[cfg(test)]
#[path = "mesh_density_tests.rs"]
```

## <a id="checkpoint-96zzzzzzs-shade-surfaces-before-drawing-ink"></a>2. Shade surfaces before drawing ink

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

Preserve authored drawing colours and independent widget pixels by ordering existing passes.

### 1. `src/renderer.rs`

Remove ambient multiplication from the end of the completed scene. Finish ambient contacts after face, cap and cloud depth, before authored ink. Remove the widget call from before scene text. Composite the existing independent widget after scene glyphs.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
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
                    &self.primitive.view,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    self.ambient_rows.ground_at(self.anchor()),
                    [self.pyramid_revision, clip_revision],
                    Some(&self.instances),
                );
            }
        }
        if let Some(pyramid) = self.pyramid.borrow_mut().as_mut() {
```

Replace it with:

```rust
                }
            }
        }
        self.widget.draw(&mut encoder, view);
        if let Some(pyramid) = self.pyramid.borrow_mut().as_mut() {
```

**Locate this block:**

```rust
                self.outline.borrow_mut().take();
            }
        }
        self.widget.draw(&mut encoder, view);
        self.text_error.borrow_mut().take();
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            match lane.prepare(
```

Replace it with:

```rust
                self.outline.borrow_mut().take();
            }
        }
        self.text_error.borrow_mut().take();
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            match lane.prepare(
```

**Locate this block:**

```rust
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("finite scene ink"),
```

Replace it with:

```rust
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
                    &self.primitive.view,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    self.ambient_rows.ground_at(self.anchor()),
                    [self.pyramid_revision, clip_revision],
                    Some(&self.instances),
                );
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("finite scene ink"),
```

### 2. `src/ambient_order_tests.rs`

Check real ground contacts, authored point/stroke colour, picks, source restoration and independent widget GPU output.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection},
    cap_gpu_tests::{renderer, wait},
    scene::Scene,
};
use prost::Message;
use session_rust::{Color, Point, Session, Xform, proto};
use std::rc::Rc;

fn camera(projection: Projection) -> Camera {
    let mut c = Camera::default();
    c.isometric();
    c.distance = 3.;
    c.radius = 1.;
    c.target = [0., 0., -0.5];
    c.aspect = 4. / 3.;
    c.projection = projection;
    c
}
fn document() -> Session {
    let mut mesh = crate::cap_gpu_tests::source(false);
    mesh.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    let mut d = Session::new("Original contacts and authored drawing colours");
    d.add_mesh(mesh, None);
    d
}
fn scene(d: &Session) -> Scene {
    let mut s = Scene::empty();
    crate::scene_document::import(
        &mut s,
        &d.to_proto().encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    s
}
fn snapshot(s: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap())
        .unwrap();
    (
        b.documents
            .iter()
            .map(|v| proto::Session::decode(v.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn pixels(g: &mut super::Renderer, c: &Camera, size: [u32; 2]) -> Vec<u8> {
    crate::widget_tests::pixels(g, &c.uniform(), size)
}
fn at<'a>(image: &'a [u8], size: [u32; 2], p: [usize; 2]) -> &'a [u8] {
    let i = (p[1] * size[0] as usize + p[0]) * 4;
    &image[i..i + 4]
}
fn shadow(
    g: &mut super::Renderer,
    c: &Camera,
    size: [u32; 2],
) -> ([usize; 2], [f64; 3]) {
    g.set_ambient(false);
    let plain = pixels(g, c, size);
    g.set_ambient(true);
    let shaded = pixels(g, c, size);
    let i = plain
        .chunks_exact(4)
        .zip(shaded.chunks_exact(4))
        .enumerate()
        .filter(|(_, (a, b))| {
            a[..3] == [255, 255, 255] && b[..3].iter().all(|v| *v < 235)
        })
        .min_by_key(|(_, (_, b))| b[0])
        .map(|(i, _)| i)
        .expect("Actual virtual-ground contact shadow");
    let p = [i % size[0] as usize, i / size[0] as usize];
    let mut m = Xform::identity();
    m.m = c.uniform().map(f64::from);
    let inverse = m.inverse().unwrap();
    let unproject = |z| {
        inverse.transform_point(&Point::new(
            (p[0] as f64 + 0.5) * 2. / size[0] as f64 - 1.,
            1. - (p[1] as f64 + 0.5) * 2. / size[1] as f64,
            z,
        ))
    };
    let a = unproject(0.);
    let b = unproject(1.);
    let t = (-0.8 - a[2]) / (b[2] - a[2]);
    (
        p,
        std::array::from_fn(|i| a[i] + t * (b[i] - a[i])),
    )
}
fn export(name: &str, s: &Scene, image: &[u8], size: [u32; 2], world: [f64; 3]) {
    if let Some(root) = std::env::var_os("VIEWER_AMBIENT_ORDER_PROOF_DIR") {
        use std::io::Write;
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(
            root.join(format!("{name}.session")),
            crate::scene_archive::save(s).unwrap(),
        )
        .unwrap();
        let mut f = std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n{} {}\n255\n", size[0], size[1]).unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
        std::fs::write(
            root.join(format!("{name}.json")),
            serde_json::to_vec_pretty(
                &serde_json::json!({"viewport":size,"worldMm":world}),
            )
            .unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn actual_original_ink_keeps_authored_colour_over_ambient_contacts() {
    let mut g = renderer(&scene(&document()));
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let c = camera(projection);
        for density in [1u32, 2] {
            let size = [320 * density, 240 * density];
            g.resize(crate::viewport::Viewport {
                width: size[0],
                height: size[1],
            });
            g.set_density(density as f32);
            g.set_scene(&scene(&document()), None);
            let (p, world) = shadow(&mut g, &c, size);
            for kind in ["point", "line", "path"] {
                let mut d = document();
                let colour = Color::new(0.8, 0.1, 0.2, 1.);
                let right = c
                    .orientation
                    .rotate_vector(session_rust::Vector::x_axis());
                let ends = [-0.08, 0.08].map(|t| {
                    Point::new(
                        world[0] + right[0] * t,
                        world[1] + right[1] * t,
                        world[2] + right[2] * t,
                    )
                });
                match kind {
                    "point" => {
                        let mut q = Point::new(world[0], world[1], world[2]);
                        q.width = 11.;
                        q.pointcolor = colour;
                        d.add_point(q, None);
                    }
                    "line" => {
                        let mut q = session_rust::Line::new(
                            ends[0][0], ends[0][1], ends[0][2], ends[1][0], ends[1][1],
                            ends[1][2],
                        );
                        q.width = 7.;
                        q.linecolor = colour;
                        d.add_line(q, None);
                    }
                    _ => {
                        let mut q = session_rust::Polyline::new(ends.to_vec());
                        q.width = 7.;
                        q.linecolor = colour;
                        d.add_polyline(q, None).unwrap();
                    }
                }
                let bytes = d.to_proto().encode_to_vec();
                let mut e = crate::editor::Editor::default();
                e.apply(crate::editor::Action::ReplaceAt(
                    bytes.clone(),
                    Rc::new(crate::reload_url::ReloadUrl::new(
                        "test:ambient-order".into(),
                    )),
                ))
                .unwrap();
                e.camera = camera(projection);
                let original = snapshot(&e.scene);
                let owner = match kind {
                    "point" => e.scene.points()[0].id,
                    "line" => e.scene.lines()[0].id,
                    _ => e.scene.paths()[0].id,
                };
                g.set_scene(&e.scene, None);
                let uploads = g.stats()[0];
                g.set_ambient(false);
                let plain = pixels(&mut g, &c, size);
                assert_eq!(at(&plain, size, p), [204, 26, 51, 255]);
                g.set_ambient(true);
                let image = pixels(&mut g, &c, size);
                assert_eq!(
                    at(&image, size, p),
                    [204, 26, 51, 255],
                    "Ambient shading precedes actual original {kind} colour"
                );
                assert!(
                    pixels(&mut g, &c, size) == image,
                    "A complete frame is stable"
                );
                assert_eq!(snapshot(&e.scene), original);
                g.set_pick_mode(crate::pick_sources::Mode::Object);
                g.request_pick(
                    super::contact_feature_tests::project(c.uniform(), world),
                    c.uniform(),
                )
                .unwrap();
                wait(&g);
                let picked = g
                    .take_pick(c.uniform())
                    .unwrap()
                    .unwrap()
                    .and_then(|v| g.resolve_pick(&e.scene, v))
                    .unwrap();
                assert_eq!(picked.owner, owner);
                assert_eq!(
                    picked.part,
                    crate::pick_sources::Part::Object
                );
                let old = e.scene.placement(owner).unwrap();
                e.scene
                    .translate(owner, [0.1, 0., 0.])
                    .unwrap();
                g.set_scene(&e.scene, None);
                assert!(pixels(&mut g, &c, size) != image);
                e.scene.place_owner(owner, old).unwrap();
                g.set_scene(&e.scene, None);
                assert!(pixels(&mut g, &c, size) == image);
                assert_eq!(g.stats()[0], uploads);
                assert_eq!(snapshot(&e.scene), original);
                if kind == "point" {
                    let name = format!(
                        "point-{}-dpr{density}",
                        if projection == Projection::Perspective {
                            "perspective"
                        } else {
                            "orthographic"
                        }
                    );
                    export(&name, &e.scene, &image, size, world);
                    let retained = e.scene.objects()[0].mesh.clone();
                    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
                    e.apply(crate::editor::Action::UnloadSources)
                        .unwrap();
                    assert!(weak.upgrade().is_none());
                    assert!(Rc::ptr_eq(
                        &retained,
                        &e.scene.objects()[0].mesh
                    ));
                    g.set_scene(&e.scene, None);
                    assert!(pixels(&mut g, &c, size) == image);
                    let keys = e.reload_keys();
                    e.hydrate(
                        keys.into_iter()
                            .map(|k| (k, bytes.clone()))
                            .collect(),
                    )
                    .unwrap();
                    assert_eq!(snapshot(&e.scene), original);
                    assert_eq!(
                        e.camera.view_projection().m,
                        c.view_projection().m
                    );
                    g.set_scene(&e.scene, None);
                    assert!(pixels(&mut g, &c, size) == image);
                    assert_eq!(g.stats()[0], uploads);
                }
            }
        }
    }
}

fn isolated_widget(g: &super::Renderer, size: [u32; 2]) -> Vec<u8> {
    let t = g
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("actual independent widget reference"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
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
    let view = t.create_view(&Default::default());
    let mut e = g
        .device
        .create_command_encoder(&Default::default());
    {
        let _pass = e.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
    }
    g.widget.draw(&mut e, &view);
    let pitch = (size[0] * 4).div_ceil(256) * 256;
    let b = g
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(pitch) * u64::from(size[1]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    e.copy_texture_to_buffer(
        t.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &b,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(size[1]),
            },
        },
        t.size(),
    );
    g.queue.submit([e.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    b.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(g);
    rx.recv().unwrap().unwrap();
    let mapped = b.slice(..).get_mapped_range();
    let image = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|r| r[..size[0] as usize * 4].iter().copied())
        .collect();
    drop(mapped);
    b.unmap();
    image
}
#[test]
fn actual_widget_overlay_matches_its_independent_gpu_reference() {
    let s = scene(&document());
    let original = snapshot(&s);
    let mut g = renderer(&s);
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        for density in [1u32, 2] {
            let size = [320 * density, 240 * density];
            g.resize(crate::viewport::Viewport {
                width: size[0],
                height: size[1],
            });
            g.set_density(density as f32);
            let mut c = camera(projection);
            let (_, world) = shadow(&mut g, &c, size);
            c.target = world;
            g.set_gizmo(&c, Some(world), -1.);
            let usage = g.gizmo_usage();
            let uploads = g.stats()[0];
            let picks = g.pick_usage();
            let reference = isolated_widget(&g, size);
            g.set_ambient(true);
            let image = pixels(&mut g, &c, size);
            let mut opaque = 0;
            let mut changed = 0;
            for (a, b) in reference
                .chunks_exact(4)
                .zip(image.chunks_exact(4))
            {
                if a[3] == 255 {
                    opaque += 1;
                    if a[..3] != b[..3] {
                        changed += 1;
                    }
                }
            }
            assert!(opaque > 100);
            assert_eq!(
                changed, 0,
                "All opaque production widget pixels keep their actual independent GPU colour"
            );
            assert_eq!(g.gizmo_usage()[..3], usage[..3]);
            assert_eq!(g.stats()[0], uploads);
            assert_eq!(g.pick_usage(), picks);
            assert_eq!(snapshot(&s), original);
            let name = format!(
                "widget-{}-dpr{density}",
                if projection == Projection::Perspective {
                    "perspective"
                } else {
                    "orthographic"
                }
            );
            export(&name, &s, &image, size, world);
            g.set_gizmo(&c, None, -1.);
            assert_eq!(g.gizmo_usage()[1], 0);
        }
    }
}
```

### 3. `src/renderer.rs`

Register the two actual pass-order GPU checks.

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "mesh_density_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
#[path = "ambient_order_tests.rs"]
mod ambient_order_tests;
#[cfg(test)]
#[path = "mesh_density_tests.rs"]
```

## <a id="checkpoint-96zzzzzzt-keep-contact-ambient-inputs-on-originals"></a>3. Keep contact ambient inputs on originals

### 1. `src/feature_owner.rs`

Resolve the existing original visual ancestor before applying pickability. A locked original still owns its visible contact shading. Keep existing pick and highlight filtering while exposing raw visual ownership. Use the same original hierarchy for visual shading. Preserve the existing original picking contract.

**Locate this existing block:**

```rust
};

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.instances.contains_key(&id)
                || scene.objects().iter().any(|row| {
                    row.id == id
                        && matches!(
                            row.original.as_ref(),
                            Some(session_rust::Geometry::Element(_))
                        )
                })
            {
                return scene.pick_id(id.value());
            }
        }
        parent = node.parent;
    }
    None
}

pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) {
        return scene.pick_id(id.value());
    }
    for doc in &scene.forest.documents {
        if let Some(index) = doc
            .nodes
            .iter()
            .position(|node| node.object == Some(id))
        {
            return ancestor(scene, doc, index);
        }
    }
    None
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
};

fn visual_ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    let mut parent = doc.nodes[index].parent;
    while let Some(index) = parent {
        let node = &doc.nodes[index];
        if let Some(id) = node.object {
            if scene.instances.contains_key(&id)
                || scene.objects().iter().any(|row| {
                    row.id == id
                        && matches!(
                            row.original.as_ref(),
                            Some(session_rust::Geometry::Element(_))
                        )
                })
            {
                return Some(id);
            }
        }
        parent = node.parent;
    }
    None
}

fn ancestor(scene: &Scene, doc: &Document, index: usize) -> Option<ObjectId> {
    visual_ancestor(scene, doc, index).and_then(|id| scene.pick_id(id.value()))
}

pub fn visual_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    if !scene.forest.derived(id) {
        return Some(id);
    }
    for doc in &scene.forest.documents {
        if let Some(index) = doc
            .nodes
            .iter()
            .position(|node| node.object == Some(id))
        {
            return visual_ancestor(scene, doc, index);
        }
    }
    None
}

pub fn display_owner(scene: &Scene, id: ObjectId) -> Option<ObjectId> {
    visual_owner(scene, id).and_then(|id| scene.pick_id(id.value()))
}
```

### 2. `src/ambient_rows.rs`

Remove the obsolete maximum before visual-owner mapping. Compute the maximum after resolving original visual radii. Map temporary numeric radii to original visual owners, without retaining another source or property cache.

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        let mut floor = f64::INFINITY;
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
            }
        }
        let mut radii = self
            .specs
            .iter()
            .copied()
            .collect::<std::collections::HashMap<_, _>>();
        for object in scene
            .objects()
            .iter()
            .filter(|r| scene.forest.derived(r.id))
        {
            if let Some(radius) = crate::feature_owner::visual_owner(scene, object.id)
                .and_then(|id| radii.get(&id.value()))
                .copied()
            {
                radii.insert(object.id.value(), radius);
            }
        }
        for (id, radius) in &mut self.specs {
            *radius = radii[id];
        }
        let radius = self
            .specs
            .iter()
            .fold(0f32, |radius, row| radius.max(row.1));
        self.ground = [
```

### 3. `src/ambient_sources.wgsl`

Reuse the actual geometry function, including contact depth and clipping flags. Read existing packed colour and normal words for the shared contact-depth function. Match contact coverage in the existing shared object path too.

**Locate this existing block:**

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,object:ObjectSettings)->Vertex {
 let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,object.selection.z,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>)->Vertex {return ambient_vertex(position,object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,object);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,@location(3) @interpolate(flat) radius:f32}
fn ambient_vertex(position:vec3<f32>,colour:vec4<f32>,normal:vec3<f32>,object:ObjectSettings)->Vertex {
 let input=geometry(position,colour,normal,object);return Vertex(input.position,input.world,input.clip_flags,bitcast<f32>(object.range.z));
}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(1) normal:u32,@location(2) colour:u32)->Vertex {return ambient_vertex(position,unpack4x8unorm(colour),oct32_decode(normal),object_at(0u));}
@vertex fn vs_record(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->Vertex {let object=object_at(row);let input=mesh_vertex(occurrence,object.range.x);return ambient_vertex(input.position,input.colour,input.normal,object);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
```

### 4. `src/ambient_source_pipeline.rs`

Use the existing 20-byte vertex layout; no additional buffer or attribute storage.

**Locate this existing block:**

```rust
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            });
            let attributes = wgpu::vertex_attr_array![0=>Float32x3,1=>Uint32,2=>Uint32];
            let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

### 5. `src/ambient_contact_tests.rs`

Check real contact coverage/radius, both object paths and device budgets, locks, original picks, source semantics and placed definitions.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection},
    cap_gpu_tests::{renderer, wait},
    editor::{Action, Editor},
    scene::Scene,
};
use prost::Message;
use session_rust::{Session, Xform, proto};
use std::rc::Rc;

fn document() -> proto::Session {
    let (_, source) = super::contact_feature_tests::imported(false);
    let mut d = Session::from_proto(source).unwrap();
    let mut cube = crate::cap_gpu_tests::source(false);
    assert!(cube.transform(
        &(&Xform::translation(280., 200., 80.) * &Xform::scale_xyz(100., 100., 100.))
    ));
    cube.set_objectcolor(session_rust::Color::new(0.3, 0.3, 0.3, 1.));
    d.add_mesh(cube, None);
    d.to_proto()
}
fn editor(bytes: &[u8]) -> Editor {
    let mut e = Editor::default();
    e.apply(Action::ReplaceAt(
        bytes.to_vec(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:contact-ambient".into(),
        )),
    ))
    .unwrap();
    e
}
fn camera(projection: Projection) -> Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c.projection = projection;
    c
}
fn snapshot(
    s: &Scene,
) -> (
    Vec<proto::Session>,
    Vec<(String, proto::Mesh)>,
    serde_json::Value,
) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap())
        .unwrap();
    let mut embedded = Vec::new();
    let documents = b
        .documents
        .iter()
        .enumerate()
        .map(|(document, v)| {
            let mut d = proto::Session::decode(v.as_slice()).unwrap();
            for (branch, objects) in [
                ("objects", d.objects.as_mut()),
                ("definitions", d.definitions.as_mut()),
            ] {
                if let Some(objects) = objects {
                    for (index, element) in objects.elements.iter_mut().enumerate() {
                        if element.geometry_type == "Mesh" {
                            embedded.push((
                                format!(
                                    "{document}/{branch}/{index}/{}",
                                    element.guid
                                ),
                                proto::Mesh::decode(element.geometry_data.as_slice())
                                    .unwrap(),
                            ));
                            element.geometry_data.clear();
                        }
                    }
                }
            }
            d
        })
        .collect();
    (
        documents,
        embedded,
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn low_renderer(s: &Scene) -> super::Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let mut request = wgpu::DeviceDescriptor::default();
    request
        .required_limits
        .max_storage_buffer_binding_size = 64 * 1024;
    let (device, queue) = pollster::block_on(adapter.request_device(&request)).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        panic!("Actual contact ambient device: {error}")
    }));
    let mut g = super::Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        s,
    );
    g.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    g
}
fn pixels(g: &mut super::Renderer, c: &Camera, size: [u32; 2]) -> Vec<u8> {
    crate::widget_tests::pixels(g, &c.uniform(), size)
}
fn read(g: &super::Renderer, t: &wgpu::Texture, bpp: u32) -> Vec<u8> {
    let size = t.size();
    let pitch = (size.width * bpp).div_ceil(256) * 256;
    let b = g
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(pitch) * u64::from(size.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut e = g
        .device
        .create_command_encoder(&Default::default());
    e.copy_texture_to_buffer(
        t.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &b,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(size.height),
            },
        },
        size,
    );
    g.queue.submit([e.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    b.slice(..)
        .map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap()
        });
    wait(g);
    rx.recv().unwrap().unwrap();
    let mapped = b.slice(..).get_mapped_range();
    let value = mapped
        .chunks_exact(pitch as usize)
        .flat_map(|row| {
            row[..(size.width * bpp) as usize]
                .iter()
                .copied()
        })
        .collect();
    drop(mapped);
    b.unmap();
    value
}
fn properties(g: &super::Renderer, size: [u32; 2], shared: bool) -> (Vec<u8>, Vec<u8>) {
    let texture = |format| {
        g.device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("actual contact ambient property readback"),
                size: wgpu::Extent3d {
                    width: size[0],
                    height: size[1],
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            })
    };
    let radii = texture(wgpu::TextureFormat::R32Float);
    let normals = texture(wgpu::TextureFormat::Rg8Unorm);
    let mut sources = crate::ambient_source_gpu::Sources::new(&g.device);
    let mut e = g
        .device
        .create_command_encoder(&Default::default());
    sources.encode(
        &mut e,
        &radii.create_view(&Default::default()),
        &normals.create_view(&Default::default()),
        &g.depth,
        &g.view_group,
        &g.clipping.borrow().group,
        &g.meshes,
        shared.then_some(&g.instances),
    );
    g.queue.submit([e.finish()]);
    (read(g, &radii, 4), read(g, &normals, 2))
}
fn sample(c: &Camera, size: [u32; 2], p: [f64; 3]) -> usize {
    let ndc = super::contact_feature_tests::project(c.uniform(), p);
    let x = ((ndc[0] + 1.) * size[0] as f32 / 2.).floor() as usize;
    let y = ((1. - ndc[1]) * size[1] as f32 / 2.).floor() as usize;
    assert!(x < size[0] as usize && y < size[1] as usize);
    y * size[0] as usize + x
}
fn radius(data: &[u8], index: usize) -> f32 {
    f32::from_ne_bytes(
        data[index * 4..index * 4 + 4]
            .try_into()
            .unwrap(),
    )
}
fn pick(
    g: &mut super::Renderer,
    s: &Scene,
    c: &Camera,
    p: [f64; 3],
) -> Option<crate::pick_sources::Target> {
    g.set_pick_mode(crate::pick_sources::Mode::Face);
    g.request_pick(
        super::contact_feature_tests::project(c.uniform(), p),
        c.uniform(),
    )
    .unwrap();
    wait(g);
    g.take_pick(c.uniform())
        .unwrap()
        .unwrap()
        .and_then(|v| g.resolve_pick(s, v))
}
fn export(name: &str, s: &Scene, bytes: &[u8], image: &[u8], size: [u32; 2]) {
    if let Some(root) = std::env::var_os("VIEWER_AMBIENT_CONTACT_PROOF_DIR") {
        use std::io::Write;
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(root.join(format!("{name}.pb")), bytes).unwrap();
        std::fs::write(
            root.join(format!("{name}.session")),
            crate::scene_archive::save(s).unwrap(),
        )
        .unwrap();
        let mut f = std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n{} {}\n255\n", size[0], size[1]).unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}

#[test]
fn contact_properties_match_original_radius_and_shared_geometry_in_both_paths() {
    let bytes = document().encode_to_vec();
    let mut e = editor(&bytes);
    let parent = e
        .scene
        .objects()
        .iter()
        .find(|r| {
            matches!(
                r.original.as_ref(),
                Some(session_rust::Geometry::Element(_))
            )
        })
        .unwrap()
        .id;
    let child = e
        .scene
        .objects()
        .iter()
        .find(|r| e.scene.forest.derived(r.id))
        .unwrap()
        .id;
    let original = snapshot(&e.scene);
    let point = [450., 200., 0.];
    for reduced in [false, true] {
        let mut g = if reduced {
            low_renderer(&e.scene)
        } else {
            renderer(&e.scene)
        };
        g.set_opacity(1.);
        g.set_ambient(true);
        let uploads = g.stats()[0];
        for projection in [
            Projection::Orthographic,
            Projection::Perspective,
        ] {
            let c = camera(projection);
            for density in [1u32, 2] {
                let size = [320 * density, 240 * density];
                g.resize(crate::viewport::Viewport {
                    width: size[0],
                    height: size[1],
                });
                g.set_density(density as f32);
                g.set_scene(&e.scene, None);
                let image = pixels(&mut g, &c, size);
                let expected = g
                    .ambient_rows
                    .specs
                    .iter()
                    .find(|r| r.0 == parent.value())
                    .unwrap()
                    .1;
                assert_eq!(
                    g.ambient_rows
                        .specs
                        .iter()
                        .find(|r| r.0 == child.value())
                        .unwrap()
                        .1,
                    expected
                );
                let legacy = properties(&g, size, false);
                let shared = properties(&g, size, true);
                assert!(
                    legacy == shared,
                    "Every actual legacy/shared source property byte agrees"
                );
                let index = sample(&c, size, point);
                assert_eq!(
                    radius(&shared.0, index),
                    expected,
                    "Actual contact radius follows its original visual owner"
                );
                assert!(
                    shared.1[index * 2..index * 2 + 2]
                        .iter()
                        .all(|v| v.abs_diff(128) <= 1),
                    "Ambient source normals remain geometric"
                );
                if !reduced {
                    assert!(
                        g.meshes
                            .iter()
                            .all(|m| g.instances.current_row(m).is_some()),
                        "The ordinary device exercises actual storage object records"
                    );
                } else {
                    assert_eq!(
                        g.tile_usage()[..2],
                        [96, 16],
                        "Real64KiB device retains physical fallback"
                    );
                }
                assert_eq!(
                    pick(&mut g, &e.scene, &c, point),
                    Some(crate::pick_sources::Target {
                        owner: parent,
                        part: crate::pick_sources::Part::Face(0)
                    })
                );
                assert!(snapshot(&e.scene) == original);
                Rc::make_mut(&mut e.scene.view).lock(&[parent], true);
                assert_eq!(
                    crate::feature_owner::visual_owner(&e.scene, child),
                    Some(parent)
                );
                assert_eq!(
                    crate::feature_owner::display_owner(&e.scene, child),
                    None
                );
                g.set_scene(&e.scene, None);
                assert!(
                    pixels(&mut g, &c, size) == image,
                    "Locking changes picking, not visible contact shading"
                );
                assert_eq!(
                    radius(&properties(&g, size, false).0, index),
                    expected
                );
                assert_eq!(pick(&mut g, &e.scene, &c, point), None);
                Rc::make_mut(&mut e.scene.view).lock(&[parent], false);
                g.set_scene(&e.scene, None);
                assert!(pixels(&mut g, &c, size) == image);
                assert!(snapshot(&e.scene) == original);
                let ordinary = std::mem::take(&mut g.instances.ordinary);
                g.set_ambient(false);
                g.set_ambient(true);
                assert!(
                    pixels(&mut g, &c, size) == image,
                    "Actual complete renderer uniform fallback preserves every pixel"
                );
                g.instances.ordinary = ordinary;
                assert_eq!(g.stats()[0], uploads);
                if !reduced {
                    let name = format!(
                        "ordinary-{}-dpr{density}",
                        if projection == Projection::Orthographic {
                            "orthographic"
                        } else {
                            "perspective"
                        }
                    );
                    export(&name, &e.scene, &bytes, &image, size);
                }
            }
        }
    }
}

#[test]
fn placed_contacts_follow_original_instances_without_changing_pickability_or_geometry()
{
    let element = document()
        .objects
        .unwrap()
        .elements
        .remove(0);
    let guid = element.guid.clone();
    let mut source = crate::instance_specimen::message();
    source.definitions = Some(proto::Objects {
        elements: vec![element],
        ..Default::default()
    });
    for row in &mut source.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    source.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1800., 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1800., 0., 0.)
                    * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let bytes = source.encode_to_vec();
    let mut e = editor(&bytes);
    let parents = e
        .scene
        .objects()
        .iter()
        .filter(|r| e.scene.instances.contains_key(&r.id))
        .map(|r| r.id)
        .collect::<Vec<_>>();
    assert_eq!(parents.len(), 2);
    let rows = parents
        .iter()
        .map(|id| {
            e.scene
                .objects()
                .iter()
                .find(|r| r.id == *id)
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert!(Rc::ptr_eq(&rows[0].mesh, &rows[1].mesh));
    let weak = Rc::downgrade(rows[0].geometry().unwrap());
    let original = snapshot(&e.scene);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    g.set_ambient(true);
    let uploads = g.stats()[0];
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        let mut c = camera(projection);
        c.distance = 6000.;
        c.radius = 3000.;
        for density in [1u32, 2] {
            let size = [320 * density, 240 * density];
            g.resize(crate::viewport::Viewport {
                width: size[0],
                height: size[1],
            });
            g.set_density(density as f32);
            g.set_scene(&e.scene, None);
            let image = pixels(&mut g, &c, size);
            let properties = properties(&g, size, true);
            let mut expected = Vec::new();
            for &parent in &parents {
                let r = g
                    .ambient_rows
                    .specs
                    .iter()
                    .find(|r| r.0 == parent.value())
                    .unwrap()
                    .1;
                expected.push(r);
                let placed = e
                    .scene
                    .placement(parent)
                    .unwrap()
                    .transform_point(&session_rust::Point::new(450., 200., 0.));
                let point = [placed[0], placed[1], placed[2]];
                assert_eq!(
                    radius(&properties.0, sample(&c, size, point)),
                    r
                );
                assert_eq!(
                    pick(&mut g, &e.scene, &c, point)
                        .unwrap()
                        .owner,
                    parent
                );
                for child in e.scene.objects().iter().filter(|r| {
                    e.scene.forest.derived(r.id)
                        && crate::feature_owner::visual_owner(&e.scene, r.id)
                            == Some(parent)
                }) {
                    assert_eq!(
                        g.ambient_rows
                            .specs
                            .iter()
                            .find(|r| r.0 == child.id.value())
                            .unwrap()
                            .1,
                        r
                    );
                }
            }
            assert_ne!(
                expected[0], expected[1],
                "World-scaled original instances retain distinct contact radii"
            );
            assert!(snapshot(&e.scene) == original);
            let old = e.scene.placement(parents[0]).unwrap();
            e.scene
                .translate(parents[0], [100., 0., 0.])
                .unwrap();
            g.set_scene(&e.scene, None);
            assert!(pixels(&mut g, &c, size) != image);
            e.scene.place_owner(parents[0], old).unwrap();
            g.set_scene(&e.scene, None);
            assert!(pixels(&mut g, &c, size) == image);
            assert_eq!(g.stats()[0], uploads);
            assert!(snapshot(&e.scene) == original);
            let name = format!(
                "instances-{}-dpr{density}",
                if projection == Projection::Orthographic {
                    "orthographic"
                } else {
                    "perspective"
                }
            );
            export(&name, &e.scene, &bytes, &image, size);
        }
    }
    let before = snapshot(&e.scene);
    assert_eq!(
        e.apply(Action::UnloadSources).err(),
        Some("No eligible imported sources to unload")
    );
    assert!(snapshot(&e.scene) == before);
    assert!(weak.upgrade().is_some());
    drop(e);
    assert!(
        weak.upgrade().is_none(),
        "Numeric visual radii do not retain original definition sources"
    );
    g.set_scene(&Scene::empty(), None);
    g.resize(crate::viewport::Viewport {
        width: 320,
        height: 240,
    });
    let c = camera(Projection::Orthographic);
    pixels(&mut g, &c, [320, 240]);
    assert_eq!(g.ambient_usage()[0], 320);
}
```

### 6. `src/renderer.rs`

Register the two actual original contact-input GPU checks.

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "ambient_input_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
#[path = "ambient_contact_tests.rs"]
mod ambient_contact_tests;
#[cfg(test)]
#[path = "ambient_input_tests.rs"]
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.4
npm --prefix ../session_tests run course -- save 22.4
```

Continue with [22.5 · Coherent multisampling and frame-resource ownership](96zzzzzzu-keep-depth-and-address-samples-separate.md).
