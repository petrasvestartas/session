# 20.2 · Physical formats, shared section inputs and display defaults

**Goal:** Apply current device and URL display defaults to browser startup and viewport sizing. · **Steps:** 4 · **Source:** [engine/gpu/arena.rs](../../../src/engine/gpu/arena.rs) · [engine/gpu/buffers.rs](../../../src/engine/gpu/buffers.rs) · [engine/gpu/objects.rs](../../../src/engine/gpu/objects.rs) · [engine/gpu/slots.rs](../../../src/engine/gpu/slots.rs)

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

## <a id="checkpoint-96zzc-share-section-addresses-with-outlines-and-ambient"></a>1. Share section addresses with outlines and ambient shading

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

Read section identity once across ink, outline and ambient passes.

### 1. `src/physical.wgsl`

Keep one lossless decoder alongside the shared encoder.

**Locate this existing block:**

```wgsl
    return vec2<u32>(primitive&0xffffu,primitive>>16u);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    return vec2<u32>(primitive&0xffffu,primitive>>16u);
}
fn physical_address(halves:vec2<u32>)->u32{
    return halves.x|(halves.y<<16u);
}
```

### 2. `src/ink_plane.wgsl`

Reuse the shared physical address decoder.

**Locate this existing block:**

```wgsl
    let halves=textureLoad(finite_primitive,at,0).xy;
    return halves.x|(halves.y<<16u);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let halves=textureLoad(finite_primitive,at,0).xy;
    return physical_address(halves);
}
```

### 3. `src/outline_caps.wgsl`

Bind the existing integer surface address. Recover cap selection and plane depth while retaining occlusion by later depth writers.

Replace the entire file with:

```wgsl
@group(0) @binding(0) var primitives:texture_2d<u32>;
@group(0) @binding(1) var<uniform> radius:vec4<f32>;
@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
struct Coverage{@location(0) value:f32,@builtin(frag_depth) depth:f32}
@fragment fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{
 let size=textureDimensions(primitives);
 if(any(vec2<u32>(p.xy)>=size)){discard;}
 let halves=textureLoad(primitives,vec2<i32>(p.xy),0).xy;
 let primitive=physical_address(halves);
 if(!clip_is_cap(primitive)||(radius.y>0.5&&(primitive&1u)==0u)){discard;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){discard;}
 let z=clip_plane_depth(plane,clip_ndc(p.xy,vec2<f32>(size),0.).xy);
 return Coverage(1.,z);
}
```

### 4. `src/outline_caps.rs`

Match the integer texture binding. Assemble the shared clipping helpers once. Reuse the existing clipping uniform layout. Add the shared plane group to the mask pipeline. Accept the current frame’s clipping planes. Bind the same clipping group used by the surface pass.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
        self.input = Some(input.clone());
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(group) = &self.group {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.draw(0..3, 0..1);
```

Replace it with:

```rust
        );
        self.input = Some(input.clone());
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, clipping: &wgpu::BindGroup) {
        if let Some(group) = &self.group {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.set_bind_group(1, clipping, &[]);
            pass.draw(0..3, 0..1);
```

**Locate this block:**

```rust
            });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("visible cap coverage"),
            source: wgpu::ShaderSource::Wgsl(include_str!("outline_caps.wgsl").into()),
        });
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("visible cap coverage"),
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

Replace it with:

```rust
            });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("visible cap coverage"),
            source: wgpu::ShaderSource::Wgsl(
                crate::clipping_gpu::source(include_str!("outline_caps.wgsl"), 1, 0)
                    .into(),
            ),
        });
        let clipping = crate::clipping_gpu::layout(device);
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("visible cap coverage"),
                bind_group_layouts: &[Some(&layout), Some(&clipping)],
                immediate_size: 0,
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
```

**Locate this block:**

```rust
                        Stages::FRAGMENT,
                        Sample::Float {
                            filterable: false,
                        },
                        false,
                    ),
                    wgpu::BindGroupLayoutEntry {
```

Replace it with:

```rust
                        Stages::FRAGMENT,
                        Sample::Uint,
                        false,
                    ),
                    wgpu::BindGroupLayoutEntry {
```

### 5. `src/outline_gpu.rs`

Supply current plane depth to the cap mask.

**Locate this existing block:**

```rust
                if sections.is_some() {
                    self.caps.draw(&mut pass);
                }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                if sections.is_some() {
                    self.caps.draw(&mut pass, clipping);
                }
```

### 6. `src/renderer.rs`

Use the full physical primitive image for this outline mask. Remove the unused section-resource borrow.

**Locate this existing block:**

```rust
            let ctx = self.text_context();
            let caps = self.caps.borrow();
            let mut global = self.global_outline.borrow_mut();
            let global = global.get_or_insert_with(|| {
                crate::outline_gpu::Outline::new(&ctx, self.format)
            });
            global.encode_masks(
                &ctx,
                &mut encoder,
                &self.depth,
                &self.view_group,
                &self.meshes,
                *transform,
                self.size,
                self.density,
                &clip_group,
                clip_revision,
                true,
                Some(&caps.depth.view),
            );
            let mut selected = self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected) {
                let selected = selected.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::new(&ctx, self.format)
                });
                selected.encode_masks(
                    &ctx,
                    &mut encoder,
                    &self.depth,
                    &self.view_group,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    &clip_group,
                    clip_revision,
                    false,
                    Some(&caps.depth.view),
                );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            let ctx = self.text_context();
            let mut global = self.global_outline.borrow_mut();
            let global = global.get_or_insert_with(|| {
                crate::outline_gpu::Outline::new(&ctx, self.format)
            });
            global.encode_masks(
                &ctx,
                &mut encoder,
                &self.depth,
                &self.view_group,
                &self.meshes,
                *transform,
                self.size,
                self.density,
                &clip_group,
                clip_revision,
                true,
                Some(&self.primitive.view),
            );
            let mut selected = self.outline.borrow_mut();
            if self.meshes.iter().any(GpuMesh::selected) {
                let selected = selected.get_or_insert_with(|| {
                    crate::outline_gpu::Outline::new(&ctx, self.format)
                });
                selected.encode_masks(
                    &ctx,
                    &mut encoder,
                    &self.depth,
                    &self.view_group,
                    &self.meshes,
                    *transform,
                    self.size,
                    self.density,
                    &clip_group,
                    clip_revision,
                    false,
                    Some(&self.primitive.view),
                );
```

### 7. `src/ambient.wgsl`

Read integer section identity during ambient preparation. Share one bounded integer reader between ambient radius and normal lookup. Keep the established section radius without a second surface record. Decode and bound the section plane before reading its normal.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
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
```

Replace it with:

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
 }
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
```

**Locate this block:**

```wgsl
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_sections:texture_2d<f32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
```

Replace it with:

```wgsl
@group(0) @binding(3) var source_normals:texture_2d<f32>;
@group(0) @binding(4) var source_primitives:texture_2d<u32>;
// Geometry access: ambient_geometry.wgsl, appended by ssao.rs
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
```

### 8. `src/ambient_pipeline.rs`

Match the shared integer source binding.

**Locate this existing block:**

```rust
                    ),
                    texture(
                        4,
                        T::Float {
                            filterable: false,
                        },
                    ),
                    uniform(5),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    ),
                    texture(4, T::Uint),
                    uniform(5),
```

### 9. `src/renderer.rs`

Bind the same physical primitive image to ambient shading.

**Locate this existing block:**

```rust
                    &clip.buffer,
                    &self.caps.borrow().depth.view,
                    &self.meshes,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    &clip.buffer,
                    &self.primitive.view,
                    &self.meshes,
```

### 10. `src/caps_gpu.rs`

Remove the unused float section image owner. Stop allocating its empty placeholder. Resize only the crossing-count image still used by cap rendering. Keep only color and physical primitive attachments. Report only allocated cap resources.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn usage(&self) -> [u64; 5] {
        [
            self.frame.size()
                + self.depth.bytes
                + self.pick_bytes()
```

Replace it with:

```rust
    pub fn usage(&self) -> [u64; 5] {
        [
            self.frame.size()
                + self.pick_bytes()
```

**Locate this block:**

```rust
                            },
                        }),
                        Some(wgpu::RenderPassColorAttachment {
                            view: &self.depth.view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: if plane == 0 {
                                    wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                                } else {
                                    wgpu::LoadOp::Load
                                },
                                store: wgpu::StoreOp::Store,
                            },
                        }),
                        Some(wgpu::RenderPassColorAttachment {
                            view: primitive,
                            depth_slice: None,
                            resolve_target: None,
```

Replace it with:

```rust
                            },
                        }),
                        Some(wgpu::RenderPassColorAttachment {
                            view: primitive,
                            depth_slice: None,
                            resolve_target: None,
```

**Locate this block:**

```rust
        } else {
            [1, 1]
        };
        if self.depth.size != size {
            self.depth = Image::new(
                device,
                size,
                wgpu::TextureFormat::Rg32Float,
                8,
            );
        }
        if size == [1, 1] {
            self.counts = None;
            return;
```

Replace it with:

```rust
        } else {
            [1, 1]
        };
        if size == [1, 1] {
            self.counts = None;
            return;
```

**Locate this block:**

```rust
            solids: Default::default(),
            pipes: None,
            counts: None,
            depth: Image::new(
                device,
                [1, 1],
                wgpu::TextureFormat::Rg32Float,
                8,
            ),
            frame,
            frame_group: None,
            rows: Vec::new(),
```

Replace it with:

```rust
            solids: Default::default(),
            pipes: None,
            counts: None,
            frame,
            frame_group: None,
            rows: Vec::new(),
```

**Locate this block:**

```rust
    counts: Option<Counts>,
    pub depth: Image,
    frame: wgpu::Buffer,
    frame_group: Option<wgpu::BindGroup>,
    pub rows: Vec<[u32; 2]>,
```

Replace it with:

```rust
    counts: Option<Counts>,
    frame: wgpu::Buffer,
    frame_group: Option<wgpu::BindGroup>,
    pub rows: Vec<[u32; 2]>,
```

### 11. `src/cap_color.wgsl`

Move the physical address beside color after removing the float output. Keep plane and selection only in the physical primitive marker.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), vec2<f32>(z,f32(i+1u)+select(0.0,8.0,selected)), physical_primitive(marker), z, mask);
}
```

Replace it with:

```wgsl
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), physical_primitive(marker), z, mask);
}
```

**Locate this block:**

```wgsl
    @location(0) color: vec4<f32>, // hatch or light grey, with a black boundary
    @location(1) section_depth: vec2<f32>, // visible section depth and presence
    @location(2) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

Replace it with:

```wgsl
    @location(0) color: vec4<f32>, // hatch or light grey, with a black boundary
    @location(1) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}
```

### 12. `src/cap_pipeline.rs`

Match the reduced fragment outputs.

**Locate this existing block:**

```rust
                },
                wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg32Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                },
                crate::primitive_image::target(true),
```

### 13. `src/cap_pick_pipeline.rs`

Match the reduced fragment outputs.

**Locate this existing block:**

```rust
                },
                wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rg32Float,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                },
                crate::primitive_image::target(true),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                },
                crate::primitive_image::target(true),
```

### 14. `src/cap_pick.wgsl`

Keep exact original owner and physical address as the two integer outputs. Remove the unused float pick record.

**Locate this existing block:**

```wgsl
}

struct CapId{@location(0) id:vec2<u32>,@location(1) section_depth:vec2<f32>,@location(2) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + vec2<f32>(frame.origin), vec2<f32>(frame.full), 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, frame.size);
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || owner>arrayLength(&owners) || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let marker=CAP_PRIMITIVE+i*2u;let source=owners[owner-1u];return CapId(vec2<u32>(select(0u,source.x,source.y!=0u),0u),vec2<f32>(z,f32(i+1u)),physical_primitive(marker),z);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

struct CapId{@location(0) id:vec2<u32>,@location(1) primitive:vec2<u32>,@builtin(frag_depth) depth:f32}
@fragment
fn fs_cap_id(in: CapVertex) -> CapId {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + vec2<f32>(frame.origin), vec2<f32>(frame.full), 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let word = clip_pick_word(i, in.pos.xy, frame.size);
    let count = bitcast<i32>(atomicLoad(&pick_caps[word]));
    let owner = select(atomicLoad(&pick_caps[word + 3u]), atomicLoad(&pick_caps[word + 1u]), count == 1);
    var kept = true;

    for (var j = 0u; j < clipping.count; j++) {
        kept = kept && (j == i || dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0)) >= 0.0);
    }

    if (count < 1 || owner == 0u || owner>arrayLength(&owners) || !kept || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let marker=CAP_PRIMITIVE+i*2u;let source=owners[owner-1u];return CapId(vec2<u32>(select(0u,source.x,source.y!=0u),0u),physical_primitive(marker),z);
}
```

### 15. `src/cap_pick_gpu.rs`

Retain the existing crop’s primitive attachment.

**Locate this existing block:**

```rust
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.section,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                Some(wgpu::RenderPassColorAttachment {
                    view: &image.primitive.view,
```

### 16. `src/pick_image.rs`

Remove the unused pick-image member. Stop allocating a duplicate eight-byte surface record per cropped pixel. Construct the smaller cropped image.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            colour,
            depth,
            primitive,
            section,
            readback,
```

Replace it with:

```rust
            colour,
            depth,
            primitive,
            readback,
```

**Locate this block:**

```rust
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let section = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("bounded section depth"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rg32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let primitive =
            crate::primitive_image::Image::new(device, [size.width, size.height]);
        Self {
```

Replace it with:

```rust
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let primitive =
            crate::primitive_image::Image::new(device, [size.width, size.height]);
        Self {
```

**Locate this block:**

```rust
    pub primitive: crate::primitive_image::Image,
    pub section: wgpu::TextureView,
    pub readback: wgpu::Buffer,
    pub window: Window,
}
```

Replace it with:

```rust
    pub primitive: crate::primitive_image::Image,
    pub readback: wgpu::Buffer,
    pub window: Window,
}
```

### 17. `src/pick_encode.rs`

Remove the pass that only cleared the unused attachment.

**Locate this existing block:**

```rust
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear bounded section depth"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.section,
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
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

### 18. `src/pick_gpu.rs`

Account for eight owner bytes, four depth bytes and four primitive bytes.

**Locate this existing block:**

```rust
                        * u64::from(image.window.draw[3])
                        * 24
            }) + 160
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        * u64::from(image.window.draw[3])
                        * 16
            }) + 160
```

### 19. `src/pick_gpu_tests.rs`

Tighten the existing bounded-memory assertion to the actual resources.

**Locate this existing block:**

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 24 + 192 + 3 * 52,
        "Readback does not grow with the full drawing"
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(
        renderer.pick_usage()[2] <= 25 * 256 + 31 * 31 * 16 + 192 + 3 * 52,
        "Readback does not grow with the full drawing"
```

### 20. `src/cap_gpu_tests.rs`

Require exactly the remaining uniform and crossing-count allocation. Require release down to the retained sixteen-byte frame uniform.

**Locate this existing block:**

```rust
    assert_eq!(r.cap_usage()[2], 1);
    assert_eq!(r.cap_usage()[0], 16 + 320 * 240 * 10);
    let uploads = r.stats()[0];
    r.set_clipping(true, false);
    let hatch = pixels(&r);
    assert_ne!(hatch, solid);
    assert!(
        hatch
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 400,
        "hatch has actual black ink"
    );
    r.set_clipping(true, true);
    r.select_objects(&scene, &[owner]).unwrap();
    let selected = pixels(&r);
    let center = at(&selected, 208, 96);
    assert!(
        center[0] > 240 && center[1] > 240 && center[2] < 20,
        "selected original section: {center:?}"
    );
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        r.cap_usage()[2],
        1,
        "selection retains original closedness cache"
    );
    scene.clear();
    r.set_scene(&scene, None);
    pixels(&r);
    assert_eq!(r.cap_usage()[0], 24);
    assert_eq!(r.cap_usage()[1], 0);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(r.cap_usage()[2], 1);
    assert_eq!(r.cap_usage()[0], 16 + 320 * 240 * 2);
    let uploads = r.stats()[0];
    r.set_clipping(true, false);
    let hatch = pixels(&r);
    assert_ne!(hatch, solid);
    assert!(
        hatch
            .chunks_exact(4)
            .filter(|p| p[..3].iter().all(|v| *v < 20))
            .count()
            > 400,
        "hatch has actual black ink"
    );
    r.set_clipping(true, true);
    r.select_objects(&scene, &[owner]).unwrap();
    let selected = pixels(&r);
    let center = at(&selected, 208, 96);
    assert!(
        center[0] > 240 && center[1] > 240 && center[2] < 20,
        "selected original section: {center:?}"
    );
    assert_eq!(r.stats()[0], uploads);
    assert_eq!(
        r.cap_usage()[2],
        1,
        "selection retains original closedness cache"
    );
    scene.clear();
    r.set_scene(&scene, None);
    pixels(&r);
    assert_eq!(r.cap_usage()[0], 16);
    assert_eq!(r.cap_usage()[1], 0);
```

### 21. `src/global_outline_tests.rs`

Require the reduced inactive cap allocation in the existing mask regression.

**Locate this existing block:**

```rust
    );
    assert_eq!(r.cap_usage()[0], 24);
    let side = crate::clipping_geometry::plane_from(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    assert_eq!(r.cap_usage()[0], 16);
    let side = crate::clipping_geometry::plane_from(
```

## <a id="checkpoint-96zzd-pack-surface-position-normal-and-color"></a>2. Use compact vertices in every surface pass

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Contiguous GPU Arena: [Slice 0 (v0..vn)] [Slice 1] [Slice 2]</text>
  <rect x="48" y="68" width="244" height="56" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <rect x="52" y="72" width="84" height="48" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="94" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 0</text>
  <rect x="140" y="72" width="72" height="48" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="176" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 1</text>
  <rect x="216" y="72" width="72" height="48" fill="#fef08a" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Slice 2</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Vertex Pulling: idx = indices[base + vid]; v = arena[idx]</text>
  <rect x="424" y="54" width="272" height="40" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">var&lt;storage, read&gt; vertices: array&lt;GpuVertex&gt;</text>
  <rect x="454" y="110" width="212" height="36" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">Dirty Sub-Range -> write_buffer()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Single-Bind-Group Multi-Slice Scene Draw</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="828,138 864,72 900,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="904,138 940,72 976,138" fill="#bbf7d0" stroke="#111111" stroke-width="1.75"/>
  <polygon points="980,138 1016,72 1052,138" fill="#fef08a" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

Use one compact surface vertex buffer throughout rendering and picking.

### 1. `src/packed_vertex.rs`

Write the compact derived GPU representation while retaining source ownership.

Create the file and type:

```rust
pub const NO_NORMAL: u32 = 0x8000_8000;
pub fn oct32(normal: [f32; 3]) -> u32 {
    let length = normal.iter().map(|v| v.abs()).sum::<f32>();
    if length == 0. || length.is_nan() {
        return NO_NORMAL;
    }
    let (x, y) = (normal[0] / length, normal[1] / length);
    let sign = |v: f32| {
        if v < 0. {
            -1.
        } else {
            1.
        }
    };
    let (x, y) = if normal[2] < 0. {
        (
            (1. - y.abs()) * sign(x),
            (1. - x.abs()) * sign(y),
        )
    } else {
        (x, y)
    };
    let half =
        |v: f32| u32::from((v * 32767.).round().clamp(-32766., 32767.) as i16 as u16);
    half(x) | (half(y) << 16)
}
pub fn pack(vertex: [f32; 6], normal: [f32; 3]) -> [u32; 5] {
    let color = [vertex[3], vertex[4], vertex[5], 1.]
        .map(|v| (v.clamp(0., 1.) * 255.).round() as u8);
    [
        vertex[0].to_bits(),
        vertex[1].to_bits(),
        vertex[2].to_bits(),
        oct32(normal),
        u32::from_le_bytes(color),
    ]
}
pub fn bytes(mesh: &crate::mesh::Mesh) -> Vec<u8> {
    mesh.vertices()
        .iter()
        .zip(mesh.normals())
        .flat_map(|(v, n)| pack(*v, *n))
        .flat_map(u32::to_ne_bytes)
        .collect()
}
```

### 2. `src/lib.rs`

Expose the pure encoder before connecting its upload.

**Locate this existing block:**

```rust
mod navigation_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod navigation_tests;
pub mod packed_vertex;
#[cfg(test)]
mod packed_vertex_tests;
#[cfg(test)]
```

### 3. `src/packed_vertex.wgsl`

Move the oct32 decoder already taught for clouds into the shared shader source.

Create the file and type:

```wgsl
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
        return vec3<f32>(0.0);
    }

    let e = unpack2x16snorm(p);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}
```

### 4. `src/cloud_points.wgsl`

Use the shared decoder without a second implementation.

**Locate this existing block:**

```wgsl
}
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
        return vec3<f32>(0.0);
    }

    let e = unpack2x16snorm(p);
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));

    if (n.z < 0.0) {
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
    }

    return normalize(n);
}

fn rec_f(base: u32, w: u32) -> f32 {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

fn rec_f(base: u32, w: u32) -> f32 {
```

### 5. `src/clipping_gpu.rs`

Make the same decoder available to current cloud and upcoming mesh shaders.

**Locate this existing block:**

```rust
        concat!(
            include_str!("physical.wgsl"),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        concat!(
            include_str!("packed_vertex.wgsl"),
            "\n",
            include_str!("physical.wgsl"),
```

### 6. `src/packed_vertex_tests.rs`

Supply independent position-bit, memory-size, quantization, sentinel and directional-error checks.

Create the file and type:

```rust
use crate::packed_vertex::{NO_NORMAL, oct32, pack};
fn decode(word: u32) -> [f64; 3] {
    let x = (word as u16 as i16) as f64 / 32767.;
    let y = ((word >> 16) as u16 as i16) as f64 / 32767.;
    let z = 1. - x.abs() - y.abs();
    let (x, y) = if z < 0. {
        (
            (1. - y.abs())
                * if x < 0. {
                    -1.
                } else {
                    1.
                },
            (1. - x.abs())
                * if y < 0. {
                    -1.
                } else {
                    1.
                },
        )
    } else {
        (x, y)
    };
    let length = (x * x + y * y + z * z).sqrt();
    [x / length, y / length, z / length]
}
#[test]
fn compact_vertex_preserves_position_bits_and_current_color_quantization() {
    let v = [-0., 123456.25, -0.0009765625, 1.2, 0.5, -0.2];
    let before = v;
    let row = pack(v, [0.; 3]);
    assert_eq!(std::mem::size_of_val(&row), 20);
    assert_eq!(
        &row[..3],
        &v[..3]
            .iter()
            .map(|v| v.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(row[3], NO_NORMAL);
    assert_eq!(row[4].to_le_bytes(), [255, 128, 0, 255]);
    assert_eq!(v, before);
}
#[test]
fn octahedral_words_reserve_missing_normals_and_bound_direction_error() {
    assert_eq!(oct32([0.; 3]), NO_NORMAL);
    assert_eq!(oct32([f32::NAN, 0., 0.]), NO_NORMAL);
    for x in -8..=8 {
        for y in -8..=8 {
            for z in -8..=8 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let normal = [x as f32, y as f32, z as f32];
                let word = oct32(normal);
                assert_ne!(word, NO_NORMAL);
                let decoded = decode(word);
                let length = ((x * x + y * y + z * z) as f64).sqrt();
                let dot = decoded[0] * x as f64 / length
                    + decoded[1] * y as f64 / length
                    + decoded[2] * z as f64 / length;
                assert!(
                    dot > 0.99999998,
                    "direction error for {normal:?}"
                );
            }
        }
    }
}
```

### 7. `src/packed_vertex.wgsl`

Declare one scalar-aligned twenty-byte shader layout and shared position reader.

**Locate this existing block:**

```wgsl
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct PackedVertex{x:f32,y:f32,z:f32,normal:u32,color:u32}
fn packed_position(vertex:PackedVertex)->vec3<f32>{return vec3<f32>(vertex.x,vertex.y,vertex.z);}
fn oct32_decode(p: u32) -> vec3<f32> {
    if (p == 0x80008000u) {
```

### 8. `src/gpu_geometry.rs`

Remove the separate normal buffer owner. Keep stable vertex and index bindings; no normal storage binding remains. Require at least one complete compact vertex or one index in the storage binding. Upload position, encoded normal and color in one buffer. Remove the duplicate upload. Bind the same compact storage used by every surface pass. Retain one source and one compact vertex buffer. Report actual vertex and index allocation. Indexed passes read their normal word from the same vertex buffer.

**Locate this existing block:**

```rust
    pub(crate) vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new(4),
            },
            count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"),
            entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(
            device: &wgpu::Device,
            label: &str,
            mut bytes: Vec<u8>,
            usage: wgpu::BufferUsages,
        ) -> wgpu::Buffer {
            if bytes.is_empty() {
                bytes.resize(4, 0);
            }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: &bytes,
                usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let bytes = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let vertices = buffer(
            device,
            "shared mesh vertices",
            bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let bytes = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let normals = buffer(
            device,
            "shared mesh normals",
            bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let bytes = source
            .indices()
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let indices = buffer(
            device,
            "shared mesh indices",
            bytes,
            wgpu::BufferUsages::INDEX,
        );
        let buffers = [&vertices, &normals, &indices];
        let entries = [0, 1, 2].map(|binding| wgpu::BindGroupEntry {
            binding,
            resource: buffers[binding as usize].as_entire_binding(),
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"),
            layout: &Self::storage_layout(device),
            entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            normals,
            indices,
            index_count,
            pull_group,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.normals.size() + self.indices.size()
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.draw_instances(pass, 1);
    }

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.normals.slice(..));
        pass.set_index_buffer(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) index_count: u32,
    pull_group: wgpu::BindGroup,
}

impl GpuGeometry {
    pub fn storage_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let entries = [0, 2].map(|binding| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: std::num::NonZeroU64::new(if binding == 0 {
                    20
                } else {
                    4
                }),
            },
            count: None,
        });
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared geometry storage"),
            entries: &entries,
        })
    }

    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        fn buffer(
            device: &wgpu::Device,
            label: &str,
            mut bytes: Vec<u8>,
            usage: wgpu::BufferUsages,
        ) -> wgpu::Buffer {
            if bytes.is_empty() {
                bytes.resize(4, 0);
            }
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: &bytes,
                usage: usage | wgpu::BufferUsages::STORAGE,
            })
        }
        let mut bytes = crate::packed_vertex::bytes(&source);
        if bytes.is_empty() {
            bytes.resize(20, 0);
        }
        let vertices = buffer(
            device,
            "shared mesh vertices",
            bytes,
            wgpu::BufferUsages::VERTEX,
        );
        let bytes = source
            .indices()
            .iter()
            .flat_map(|v| v.to_ne_bytes())
            .collect();
        let indices = buffer(
            device,
            "shared mesh indices",
            bytes,
            wgpu::BufferUsages::INDEX,
        );
        let entries = [(0, &vertices), (2, &indices)].map(|(binding, buffer)| {
            wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            }
        });
        let pull_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared vertex reads"),
            layout: &Self::storage_layout(device),
            entries: &entries,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            indices,
            index_count,
            pull_group,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.indices.size()
    }

    pub fn draw_pulled(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_bind_group(3, &self.pull_group, &[]);
        pass.draw(0..self.index_count, 0..count);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.draw_instances(pass, 1);
    }

    pub fn draw_instances(&self, pass: &mut wgpu::RenderPass<'_>, count: u32) {
        if self.index_count == 0 || count == 0 {
            return;
        }
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(
```

### 9. `src/triangle.wgsl`

Read packed words without numeric conversion. Remove the second normal binding. Decode compact vertices through the already shared oct32 helper for color and pulled integer picks.

**Locate this existing block:**

```wgsl
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

@group(3) @binding(0) var<storage, read> mesh_values: array<PackedVertex>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence:u32)->MeshInput{
 let vertex=mesh_values[mesh_indices[occurrence]];
 return MeshInput(packed_position(vertex),unpack4x8unorm(vertex.color).rgb,oct32_decode(vertex.normal));
}
```

### 10. `src/finite_project.wgsl`

Project positions from the actual compact upload. Use the shared vertex layout rather than a separate stride calculation. Preserve position bits when fitting physical triangle planes.

**Locate this existing block:**

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<f32>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let base=physical_indices[index]*6u;
 let point=vec3<f32>(physical_vertices[base],physical_vertices[base+1u],physical_vertices[base+2u]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(1) @binding(0) var<uniform> line:TileLine;
@group(2) @binding(0) var<storage,read> physical_vertices:array<PackedVertex>;
@group(2) @binding(1) var<storage,read> physical_indices:array<u32>;
@group(2) @binding(2) var<uniform> object:ObjectSettings;
@group(2) @binding(3) var<storage,read_write> projected:array<ProjectedTriangle>;
@group(2) @binding(4) var<uniform> live_count:vec4<u32>;
#include "projected_triangle.wgsl"
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
    depth: vec3<f32>, // screen gradient xy and depth at the viewport centre
};
fn physical_clip_corner(index:u32)->vec4<f32>{
 let point=packed_position(physical_vertices[physical_indices[index]]);
 let clip=view.transform*object.model*vec4<f32>(point,1.0);
```

### 11. `src/finite_project_source.rs`

Assemble physical projection with the same packed layout and position reader.

**Locate this existing block:**

```rust
pub fn source() -> String {
    expand(include_str!("finite_project.wgsl"))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source() -> String {
    format!(
        "{}\n{}",
        include_str!("packed_vertex.wgsl"),
        expand(include_str!("finite_project.wgsl"))
    )
}
```

### 12. `src/face_pick.wgsl`

Remove the obsolete source-face layout; use the shared twenty-byte declaration. Read original face positions and colors through the same compact vertex layout.

**Locate this existing block:**

```wgsl
struct PackedVertex { x: f32, y: f32, z: f32, r: f32, g: f32, b: f32 }
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(vec3<f32>(vertex.x, vertex.y, vertex.z), vec3<f32>(vertex.r, vertex.g, vertex.b), vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(packed_position(vertex), unpack4x8unorm(vertex.color).rgb, vec3<f32>(0.0),object_settings);
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
```

### 13. `src/cap_pipeline.rs`

Count closed-solid crossings from the compact vertex positions.

**Locate this existing block:**

```rust
        vec![wgpu::VertexBufferLayout {
            array_stride: 24,
            step_mode: wgpu::VertexStepMode::Vertex,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        vec![wgpu::VertexBufferLayout {
            array_stride: 20,
            step_mode: wgpu::VertexStepMode::Vertex,
```

### 14. `src/outline_gpu.rs`

Use the same position stride for outline masks.

**Locate this existing block:**

```rust
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 24,
                step_mode: wgpu::VertexStepMode::Vertex,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: 20,
                step_mode: wgpu::VertexStepMode::Vertex,
```

### 15. `src/ambient_source_pipeline.rs`

Read position and the normal word from one twenty-byte vertex binding.

**Locate this existing block:**

```rust
        });
        let draw=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("ambient original normals and radii"),layout:Some(&layout),vertex:wgpu::VertexState{module:&module,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout{array_stride:24,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3]},wgpu::VertexBufferLayout{array_stride:12,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![2=>Float32x3]}]},fragment:Some(wgpu::FragmentState{module:&module,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::R32Float,blend:None,write_mask:wgpu::ColorWrites::RED}),Some(wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::state_for_ambient()),multisample:Default::default(),multiview_mask:None,cache:None});
        Self { draw, radius }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        });
        let draw = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ambient original normals and radii"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 20,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0=>Float32x3,2=>Uint32],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::R32Float,
                        blend: None,
                        write_mask: wgpu::ColorWrites::RED,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rg8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::state_for_ambient()),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        Self { draw, radius }
```

### 16. `src/ambient_sources.wgsl`

Receive the integer normal word in the ambient property pass. Use the same decoder and preserve the missing-normal derivative fallback.

**Locate this existing block:**

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:vec3<f32>)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(normal,0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct Vertex{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) normal:vec3<f32>,@location(2) @interpolate(flat) flags:u32}
@vertex fn vs_main(@location(0) position:vec3<f32>,@location(2) normal:u32)->Vertex{let world=object.model*vec4<f32>(position,1.);return Vertex(view.transform*world,world.xyz,(object.normal_matrix*vec4<f32>(oct32_decode(normal),0.)).xyz,object.selection.z);}
fn encode_normal(n:vec3<f32>)->vec2<f32>{let p=n.xy/(abs(n.x)+abs(n.y)+abs(n.z));let folded=(1.-abs(p.yx))*select(vec2<f32>(-1.),vec2<f32>(1.),p>=vec2<f32>(0.));return select(folded,p,n.z>=0.);}
```

### 17. `src/global_outline_tests.rs`

Supply actual GPU allocation, source retention, reuse and release checks alongside the existing color/primitive/pick fixtures.

**Locate this existing block:**

```rust
        assert!(texture.width() < window.full[0] && texture.height() < window.full[1]);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert!(texture.width() < window.full[0] && texture.height() < window.full[1]);
    }
}

#[test]
fn actual_compact_surface_upload_uses_twenty_bytes_per_vertex_and_releases() {
    let scene = Scene::demo();
    let mut gpu = renderer(&scene);
    let source = scene.objects()[0].mesh.clone();
    let original = source.vertices().to_vec();
    assert_eq!(
        gpu.meshes[0].geometry.vertices.size(),
        source.vertices().len() as u64 * 20
    );
    assert_eq!(
        gpu.meshes[0].geometry.allocated_bytes(),
        source.vertices().len() as u64 * 20 + source.indices().len() as u64 * 4
    );
    let mut cache = crate::geometry_cache::GeometryCache::default();
    let uploaded = cache.get(&gpu.device, &source);
    assert_eq!(
        cache.uploaded_bytes,
        uploaded.allocated_bytes(),
        "Nonempty compact upload statistics match actual GPU buffers"
    );
    let written = cache.uploaded_bytes;
    cache.get(&gpu.device, &source);
    assert_eq!(cache.uploads, 1);
    assert_eq!(cache.uploaded_bytes, written);
    let empty = crate::gpu_geometry::GpuGeometry::upload(
        &gpu.device,
        std::rc::Rc::new(crate::mesh::Mesh::new(vec![], vec![]).unwrap()),
    );
    assert_eq!(empty.allocated_bytes(), 24);
    pixels(&gpu);
    assert_eq!(source.vertices(), original);
    let uploads = gpu.stats();
    pixels(&gpu);
    assert_eq!(gpu.stats(), uploads);
    gpu.set_scene(&Scene::empty(), None);
    assert_eq!(gpu.usage(), [0; 4]);
}
```

### 18. `src/geometry_cache.rs`

Count the compact vertex and index bytes actually uploaded; no separate float normal upload remains.

**Locate this existing block:**

```rust
        self.uploads += 1;
        self.uploaded_bytes += (source.vertices().len() * 24
            + source.normals().len() * 12
            + source.indices().len() * 4) as u64;
        geometry
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.uploads += 1;
        self.uploaded_bytes +=
            (source.vertices().len() * 20 + source.indices().len() * 4) as u64;
        geometry
```

## <a id="checkpoint-96zze-retain-rgba-in-prepared-displays"></a>3. Retain RGBA in prepared displays

Preserve display RGBA through preparation and the compact upload.

### 1. `src/mesh.rs`

Retain alpha beside the existing derived vertex columns. Keep ordinary RGB-created display vertices opaque by default. Use one validated RenderMesh conversion for ordinary kernel displays and derived feature displays. Keep length and finiteness invariants with the display owner. Account for the retained CPU alpha column.

**Locate this existing block:**

```rust
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
                "Indices must describe complete triangles within the draw limit",
            );
        }
        if indices
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        Ok(Self {
            vertices,
            normals,
            indices,
            parts: Default::default(),
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let vertices = render
            .vertices
            .iter()
            .map(|vertex| {
                let [x, y, z] = vertex.position;
                let [r, g, b, _] = vertex.color;
                [x, y, z, r, g, b]
            })
            .collect();
        let normals = render
            .vertices
            .iter()
            .map(|vertex| vertex.normal)
            .collect();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::new(vertices, render.indices)?.with_normals(normals)?;
        display.parts = parts;
        Ok(display)
    }

    pub fn with_normals(
        mut self,
        normals: Vec<[f32; 3]>,
    ) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() {
            return Err("Each display vertex needs one normal");
        }
        self.normals = normals
            .into_iter()
            .map(crate::normals::unit)
            .collect::<Result<_, _>>()?;
        Ok(self)
    }

    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.parts.bytes()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
                "Indices must describe complete triangles within the draw limit",
            );
        }
        if indices
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self {
            vertices,
            normals,
            alphas,
            indices,
            parts: Default::default(),
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts;
        Ok(display)
    }

    pub fn from_render(
        render: session_rust::render_mesh::RenderMesh,
    ) -> Result<Self, &'static str> {
        let vertices = render
            .vertices
            .iter()
            .map(|vertex| {
                let [x, y, z] = vertex.position;
                let [r, g, b, _] = vertex.color;
                [x, y, z, r, g, b]
            })
            .collect();
        let normals = render
            .vertices
            .iter()
            .map(|vertex| vertex.normal)
            .collect();
        let alphas = render
            .vertices
            .iter()
            .map(|vertex| vertex.color[3])
            .collect();
        Self::new(vertices, render.indices)?
            .with_normals(normals)?
            .with_alphas(alphas)
    }

    pub fn with_normals(
        mut self,
        normals: Vec<[f32; 3]>,
    ) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() {
            return Err("Each display vertex needs one normal");
        }
        self.normals = normals
            .into_iter()
            .map(crate::normals::unit)
            .collect::<Result<_, _>>()?;
        Ok(self)
    }

    pub fn with_alphas(mut self, alphas: Vec<f32>) -> Result<Self, &'static str> {
        if alphas.len() != self.vertices.len() || alphas.iter().any(|v| !v.is_finite())
        {
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas = alphas;
        Ok(self)
    }
    pub fn alphas(&self) -> &[f32] {
        &self.alphas
    }
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.alphas.capacity() * std::mem::size_of::<f32>()
            + self.parts.bytes()
```

### 2. `src/packed_vertex.rs`

Encode the actual display alpha instead of an opaque constant. Use the same clamped RGBA quantization for all four channels. Pack the validated columns together without another encoder.

**Locate this existing block:**

```rust
}
pub fn pack(vertex: [f32; 6], normal: [f32; 3]) -> [u32; 5] {
    let color = [vertex[3], vertex[4], vertex[5], 1.]
        .map(|v| (v.clamp(0., 1.) * 255.).round() as u8);
    [
        vertex[0].to_bits(),
        vertex[1].to_bits(),
        vertex[2].to_bits(),
        oct32(normal),
        u32::from_le_bytes(color),
    ]
}
pub fn bytes(mesh: &crate::mesh::Mesh) -> Vec<u8> {
    mesh.vertices()
        .iter()
        .zip(mesh.normals())
        .flat_map(|(v, n)| pack(*v, *n))
        .flat_map(u32::to_ne_bytes)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub fn pack(vertex: [f32; 6], normal: [f32; 3], alpha: f32) -> [u32; 5] {
    let color = [vertex[3], vertex[4], vertex[5], alpha]
        .map(|v| (v.clamp(0., 1.) * 255.).round() as u8);
    [
        vertex[0].to_bits(),
        vertex[1].to_bits(),
        vertex[2].to_bits(),
        oct32(normal),
        u32::from_le_bytes(color),
    ]
}
pub fn bytes(mesh: &crate::mesh::Mesh) -> Vec<u8> {
    mesh.vertices()
        .iter()
        .zip(mesh.normals())
        .zip(mesh.alphas())
        .flat_map(|((v, n), a)| pack(*v, *n, *a))
        .flat_map(u32::to_ne_bytes)
```

### 3. `src/packed_vertex_tests.rs`

Keep the existing opaque quantization contract explicit.

**Locate this existing block:**

```rust
    let before = v;
    let row = pack(v, [0.; 3]);
    assert_eq!(std::mem::size_of_val(&row), 20);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    let before = v;
    let row = pack(v, [0.; 3], 1.);
    assert_eq!(std::mem::size_of_val(&row), 20);
```

### 4. `src/display_rgba_tests.rs`

Supply ordinary kernel and derived contact conversion, exact RGBA, unchanged source and column-validation fixtures.

Create the file and type:

```rust
#[test]
fn derived_contact_rgba_survives_display_and_packing_without_changing_the_source() {
    let source = session_rust::Mesh::from_vertices_and_faces(
        vec![
            session_rust::Point::new(-1., -1., 0.),
            session_rust::Point::new(1., -1., 0.),
            session_rust::Point::new(0., 1., 0.),
        ],
        vec![vec![0, 1, 2]],
    );
    let before = source.to_proto();
    let ordinary = crate::mesh::Mesh::from_kernel(&source).unwrap();
    assert_eq!(ordinary.alphas(), &[1.; 3]);
    let mut render = source.to_render();
    let positions = render
        .vertices
        .iter()
        .map(|v| v.position)
        .collect::<Vec<_>>();
    for vertex in &mut render.vertices {
        vertex.color = [0.9, 0.1, 0.1, 1. / 255.];
    }
    let display = crate::mesh::Mesh::from_render(render).unwrap();
    assert_eq!(display.alphas(), &[1. / 255.; 3]);
    assert_eq!(
        display
            .vertices()
            .iter()
            .map(|v| [v[0], v[1], v[2]])
            .collect::<Vec<_>>(),
        positions
    );
    let bytes = crate::packed_vertex::bytes(&display);
    assert_eq!(bytes.len(), 60);
    for vertex in bytes.chunks_exact(20) {
        assert_eq!(
            &vertex[16..20],
            &[230, 26, 26, 1],
            "current derived contact RGBA"
        );
    }
    assert_eq!(source.to_proto(), before);
}
#[test]
fn alpha_columns_are_validated_and_default_display_vertices_stay_opaque() {
    let mesh = crate::mesh::Mesh::new(vec![[0.; 6]], vec![]).unwrap();
    assert_eq!(mesh.alphas(), &[1.]);
    assert_eq!(crate::packed_vertex::bytes(&mesh)[19], 255);
    assert!(mesh.clone().with_alphas(vec![]).is_err());
    assert!(
        mesh.clone()
            .with_alphas(vec![f32::NAN])
            .is_err()
    );
    assert!(
        mesh.clone()
            .with_alphas(vec![f32::INFINITY])
            .is_err()
    );
    let retained = mesh.with_alphas(vec![1. / 255.]).unwrap();
    assert_eq!(crate::packed_vertex::bytes(&retained)[19], 1);
}
```

### 5. `src/lib.rs`

Connect the supplied alpha-retention fixtures to the authoring harness.

**Locate this existing block:**

```rust
pub mod depth;
pub mod document;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod depth;
#[cfg(test)]
mod display_rgba_tests;
pub mod document;
```

### 6. `src/memory.rs`

Include the three retained alpha values in the exact shared-snapshot memory fixture.

**Locate this existing block:**

```rust
            retained[4],
            baseline[4] + 16 * 24 + 16 * 4 + 3 * 12
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            retained[4],
            baseline[4] + 16 * 24 + 16 * 4 + 3 * 12 + 3 * 4
        );
```

## <a id="checkpoint-96zzf-start-with-current-browser-display-defaults"></a>4. Start with current browser display defaults

### 1. `src/browser_defaults.rs`

Reuse existing opacity and grid state; detect pointer capability and read URL defaults once.

Create the file and type:

```rust
pub struct Settings {
    pub opacity: f32,
    pub grid: bool,
}

impl Settings {
    pub fn new(coarse: bool, opacity: Option<&str>, nogrid: Option<&str>) -> Self {
        let default = if coarse {
            1.
        } else {
            0.95
        };
        let opacity = opacity
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .unwrap_or(default)
            .clamp(0., 1.);
        Self {
            opacity,
            grid: nogrid.is_none(),
        }
    }

    pub fn apply(&self, editor: &mut crate::editor::Editor) {
        editor.opacity = self.opacity;
        editor.camera.grid = self.grid.then(Default::default);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn initialize(editor: &mut crate::editor::Editor) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let coarse = window
        .match_media("(pointer: coarse)")
        .ok()
        .flatten()
        .is_some_and(|media| media.matches());
    let search = window
        .location()
        .search()
        .unwrap_or_default();
    let query = web_sys::UrlSearchParams::new_with_str(&search).ok();
    let opacity = query.as_ref().and_then(|q| q.get("opacity"));
    let nogrid = query.as_ref().and_then(|q| q.get("nogrid"));
    Settings::new(coarse, opacity.as_deref(), nogrid.as_deref()).apply(editor);
}
```

### 2. `src/lib.rs`

Register the startup settings without a second renderer settings owner.

**Locate this existing block:**

```rust
mod browser;
#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod browser;
pub mod browser_defaults;
#[cfg(target_arch = "wasm32")]
```

### 3. `Cargo.toml`

Enable the browser media-query return type in the existing pinned web-sys dependency.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "MediaQueryList", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "ReadableWritablePair", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 4. `src/browser.rs`

Initialize a new browser document once before creating its renderer.

**Locate this existing block:**

```rust
pub async fn run() -> Result<(), JsValue> {
    run_with(crate::drawing_document::new()).await
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub async fn run() -> Result<(), JsValue> {
    let shared = crate::drawing_document::new();
    crate::browser_defaults::initialize(&mut shared.borrow_mut());
    run_with(shared).await
}
```

### 5. `src/browser_defaults_tests.rs`

Supply pointer-policy, finite override, clamping and existing-owner fixtures.

Create the file and type:

```rust
use crate::browser_defaults::Settings;
#[test]
fn pointer_defaults_and_url_overrides_share_existing_editor_state() {
    let mut editor = crate::editor::Editor::default();
    for (coarse, raw, nogrid, opacity, grid) in [
        (false, None, None, 0.95, true),
        (true, None, None, 1., true),
        (true, Some("0.5"), Some("1"), 0.5, false),
        (false, Some("NaN"), None, 0.95, true),
        (true, Some("inf"), None, 1., true),
        (false, Some("2"), None, 1., true),
        (false, Some("-1"), None, 0., true),
    ] {
        Settings::new(coarse, raw, nogrid).apply(&mut editor);
        assert_eq!(editor.opacity, opacity);
        assert_eq!(editor.camera.grid.is_some(), grid);
    }
}
```

### 6. `src/lib.rs`

Register the supplied startup policy fixture.

**Locate this existing block:**

```rust
pub mod browser_defaults;
#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod browser_defaults;
#[cfg(test)]
mod browser_defaults_tests;
#[cfg(target_arch = "wasm32")]
```

### 7. `src/browser_defaults.rs`

Use one query reader for startup defaults and viewport scale. Cap requested pixels using the current finite f32 override policy.

**Locate this existing block:**

```rust
}

#[cfg(target_arch = "wasm32")]
pub fn initialize(editor: &mut crate::editor::Editor) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let coarse = window
        .match_media("(pointer: coarse)")
        .ok()
        .flatten()
        .is_some_and(|media| media.matches());
    let search = window
        .location()
        .search()
        .unwrap_or_default();
    let query = web_sys::UrlSearchParams::new_with_str(&search).ok();
    let opacity = query.as_ref().and_then(|q| q.get("opacity"));
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn density(ratio: f64, override_value: Option<&str>) -> f64 {
    let ratio = if ratio > 0. {
        ratio
    } else {
        1.
    };
    let cap = override_value
        .and_then(|s| s.parse::<f32>().ok())
        .filter(|v| v.is_finite() && *v >= 0.5)
        .map(f64::from)
        .unwrap_or(2.);
    ratio.min(cap)
}

#[cfg(target_arch = "wasm32")]
fn query() -> Option<web_sys::UrlSearchParams> {
    web_sys::UrlSearchParams::new_with_str(
        &web_sys::window()?.location().search().ok()?,
    )
    .ok()
}

#[cfg(target_arch = "wasm32")]
pub fn scale(ratio: f64) -> f64 {
    let value = query().and_then(|q| q.get("dpr"));
    density(ratio, value.as_deref())
}

#[cfg(target_arch = "wasm32")]
pub fn initialize(editor: &mut crate::editor::Editor) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let coarse = window
        .match_media("(pointer: coarse)")
        .ok()
        .flatten()
        .is_some_and(|media| media.matches());
    let query = query();
    let opacity = query.as_ref().and_then(|q| q.get("opacity"));
```

### 8. `src/browser.rs`

Give viewport images and CSS ink the same capped scale before recovery reduction.

**Locate this existing block:**

```rust
        .max_texture_dimension_2d;
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
    renderer.set_density(density as f32);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .max_texture_dimension_2d;
    let density = crate::browser_recovery::density(crate::browser_defaults::scale(
        window.device_pixel_ratio(),
    ));
    renderer.set_density(density as f32);
```

### 9. `src/browser_defaults_tests.rs`

Supply high-density, finite override, lower-bound and fallback policy cases.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[test]
fn device_scale_caps_pixels_and_retains_explicit_overrides() {
    for (ratio, raw, wanted) in [
        (3., None, 2.),
        (1., None, 1.),
        (0., None, 1.),
        (3., Some("1"), 1.),
        (3., Some("3"), 3.),
        (3., Some("0.75"), 0.75),
        (3., Some("0.4"), 2.),
        (3., Some("NaN"), 2.),
        (3., Some("inf"), 2.),
    ] {
        assert_eq!(
            crate::browser_defaults::density(ratio, raw),
            wanted
        );
    }
}
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 20.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 20.2
npm --prefix ../session_tests run course -- save 20.2
```

Continue with [20.3 · Shared arenas, object records and source preparation](96zzg-grow-buffers-without-losing-rows.md).
