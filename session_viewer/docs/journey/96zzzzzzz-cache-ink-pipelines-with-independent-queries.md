# 22.5.6 · Cache matching ink pipelines and bindings

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 27–53 minutes.** [Estimate](typing-load.md).

Give finite ink matching colour pipelines and image bindings at one and four samples. Cache the alternate recipe, release old image bindings on a real switch, and keep the independent query pipeline at one sample.

## Type

Continue from [Generate ink shaders for physical samples](96zzzzzzy-generate-ink-for-physical-samples.md). [Save or recover your work](recovery.md).

### 1. `src/stroke_gpu.rs`

Declare one shared static vertex-layout recipe; retain the original field offsets and strides.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Lane {
```

</details>

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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-01.rs"
```

</details>

### 2. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
ATTRIBUTE_0
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-02.rs"
```

</details>

### 3. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
ATTRIBUTE_1
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-03.rs"
```

</details>

### 4. `src/stroke_gpu.rs`

Move the exact previously typed finite-ink vertex attributes.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
ATTRIBUTE_2
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-04.rs"
```

</details>

### 5. `src/stroke_gpu.rs`

Use the shared static layout without changing the original vertex data. Reuse the same static guide, wide-line and grid vertex layout.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

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

</details>

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

<details>
<summary>Locate this block</summary>

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

</details>

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

<details>
<summary>Complete updated block reference</summary>

```rust
pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
 const STROKE:[wgpu::VertexAttribute;4]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32];
 const CHAIN:[wgpu::VertexAttribute;7]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32];
 const MARKER:[wgpu::VertexAttribute;3]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4];
 const GUIDE:[wgpu::VertexAttribute;5]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32];
 match stride {44=>&STROKE,72=>&CHAIN,32=>&MARKER,48=>&GUIDE,_=>unreachable!()}
}

pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: Option<wgpu::RenderPipeline>,
    ids: Option<wgpu::Buffer>,
    id_data: Vec<u8>,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    stride: u64,
    vertices_per_instance: u32,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

#[cfg(test)]
impl Lane { pub(crate) fn use_legacy_fixture(&mut self){self.finite=None;} }

impl Lane {
    pub fn set(&mut self, device: &wgpu::Device, strokes: impl IntoIterator<Item = crate::stroke::Stroke>) -> Result<(), &'static str> {
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0 || stroke.values().iter().any(|value| !value.is_finite()) {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        if self.stride != 44 { return Err("Stroke data needs the stroke layout"); }
        self.set_bytes(device, data)
    }

    pub fn set_guides(&mut self,device:&wgpu::Device,segments:&[crate::guide_gpu::Segment])->Result<(),&'static str>{if self.stride!=48{return Err("Guide data needs the guide layout")}let mut bytes=Vec::with_capacity(segments.len()*48);for segment in segments{if segment.stroke.width<=0.||segment.stroke.values().iter().any(|v|!v.is_finite()){return Err("Invalid guide stroke")}bytes.extend(segment.stroke.bytes());bytes.extend(u32::from(segment.dashed).to_ne_bytes());}self.set_bytes(device,bytes)}
    pub fn set_chains(&mut self, device: &wgpu::Device, segments: impl IntoIterator<Item = crate::chain::Segment>) -> Result<(), &'static str> {
        if self.stride != 72 { return Err("Connected data needs the connected layout"); }
        let mut data = Vec::new();
        for segment in segments {
            if segment.stroke.width <= 0.0 || segment.heads > 3
                || segment.stroke.values().iter().chain(segment.previous.iter()).chain(segment.next.iter()).any(|v| !v.is_finite()) {
                return Err("Invalid connected display data");
            }
            data.extend(segment.bytes());
        }
        self.set_bytes(device, data)
    }

    pub fn set_points(&mut self, device: &wgpu::Device, markers: impl IntoIterator<Item = crate::marker::Marker>) -> Result<(), &'static str> {
        if self.stride != 32 { return Err("Marker data needs the marker layout"); }
        let mut data = Vec::new();
        for marker in markers {
            if marker.diameter == 0.0 || marker.values().iter().any(|v| !v.is_finite()) { return Err("Invalid marker display data"); }
            data.extend(marker.bytes());
        }
        self.set_bytes(device, data)
    }

    fn set_bytes(&mut self, device: &wgpu::Device, data: Vec<u8>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        u32::try_from(data.len() as u64 / self.stride).map_err(|_| "Too many stroke instances")?;
        if data == self.data { return Ok(()); }
        self.vertices = if data.is_empty() { None } else {
            self.uploads += 1; self.uploaded_bytes += data.len() as u64;
            Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("stroke instances"), contents: &data, usage: wgpu::BufferUsages::VERTEX,
            }))
        };
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;} Ok(())
    }

    fn reset_flags(&mut self,device:&wgpu::Device)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        self.set_flags(device,vec![0;if self.varying{count}else{count.min(1)}])
    }
    pub fn set_flags(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        self.set_records(device,if self.varying{flags.into_iter().flat_map(|f|[f,crate::packed_vertex::NO_NORMAL]).collect()}else{flags})
    }
    pub fn set_records(&mut self,device:&wgpu::Device,flags:Vec<u32>)->Result<(),&'static str>{
        let count=self.data.len()/self.stride as usize;
        if flags.len()!=if self.varying{count*2}else{count.min(1)}{return Err("Clipping flags must match original ink instances")}
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

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let data: Vec<u8> = ids.into_iter().flatten().flat_map(u32::to_ne_bytes).collect();
        if data.len() as u64 / 8 != self.data.len() as u64 / self.stride { return Err("Pick IDs must match display instances"); }
        if data == self.id_data { return Ok(()); }
        self.ids = if data.is_empty() { None } else { Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("ink IDs"), contents: &data, usage: wgpu::BufferUsages::VERTEX })) };
        self.id_data = data; Ok(())
    }

    pub fn pick(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup, crop: &wgpu::BindGroup) {
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }

    pub fn usage(&self) -> [u64; 2] {
        [self.data.len() as u64 / self.stride, self.vertices.as_ref().map_or(0, wgpu::Buffer::size)]
    }

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
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute]) -> Self {
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
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool, physical:bool,
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
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
        stride: u64, vertices_per_instance: u32, attributes: &[wgpu::VertexAttribute], depth: wgpu::DepthStencilState, pickable:bool,
    ) -> Self {
        Self::layout_target(device,format,source,stride,vertices_per_instance,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-10.rs"
```

</details>

### 6. `src/finite_ink.rs`

Extract the binding recipe behind an explicit count parameter.

<details>
<summary>Locate the existing block</summary>

```rust
impl Ink{
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::stroke_gpu::attributes;
fn layout(device:&wgpu::Device,samples:u32)->wgpu::BindGroupLayout {
// original layout
 layout
}
impl Ink{
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-11.rs"
```

</details>

### 7. `src/finite_ink.rs`

Move the exact existing finite-ink binding descriptor.

Move the previously typed implementation as described. Keep its code; type the following wiring edits separately.

<details>
<summary>Previously typed code reference</summary>

Use this reference when moving the existing code; it is excluded from the typing estimate.

<details>
<summary>Locate the existing block</summary>

```text
// original layout
```

</details>

Reuse the existing code in this destination:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-12.rs"
```

</details>

### 8. `src/finite_ink.rs`

Remove the original descriptor copy after moving it. Match the depth binding to its physical sample count. Match the primitive-address binding without averaging integer words. Retain only count-specific colour pipeline/layout recipes and one independent query recipe. Retain existing one-sample callers and accept a matching initial colour count. Borrow the shared static attributes instead of allocating a second vector. Build matching colour resources while retaining the single-sample original-owner query. Compile the alternate recipe only once and release stale image bindings on a real switch. Build current-colour and independent query bindings from their own layouts. Use the requested binding layout. Cache only current colour bindings. Draw with the selected colour recipe. Keep original queries on their single-sample layout.

<details>
<summary>Locate the existing block</summary>

```rust
fn pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    stride: u64,
    grid: bool,
    picking: bool,
    varying: bool,
    samples: u32,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source =
        crate::finite_sources::source_with_samples(stride, picking, grid, varying, samples);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let groups = if picking {
        vec![Some(view), Some(crop), Some(layout), Some(&clipping)]
    } else {
        vec![Some(view), Some(layout), Some(&clipping)]
    };
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("finite original ink"),
        bind_group_layouts: &groups,
        immediate_size: 0,
    });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let flags = [wgpu::VertexAttribute {
        format: if varying {
            wgpu::VertexFormat::Uint32x2
        } else {
            wgpu::VertexFormat::Uint32
        },
        offset: 0,
        shader_location: 8,
    }];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying { 8 } else { 0 },
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &flags,
    });
    let mut depth = crate::depth::ink();
    depth.depth_compare = Some(wgpu::CompareFunction::Always);
    depth.bias = Default::default();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("finite original ink"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(if picking { "pick_vertex" } else { "vertex" }),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(if picking { "pick_fragment" } else { "fragment" }),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: if picking {
                    wgpu::TextureFormat::Rg32Uint
                } else {
                    format
                },
                blend: if picking {
                    None
                } else {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                },
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
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

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    stride: u64,
    grid: bool,
    picking: bool,
    varying: bool,
    samples: u32,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source =
        crate::finite_sources::source_with_samples(stride, picking, grid, varying, samples);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let clipping = crate::clipping_gpu::layout(device);
    let groups = if picking {
        vec![Some(view), Some(crop), Some(layout), Some(&clipping)]
    } else {
        vec![Some(view), Some(layout), Some(&clipping)]
    };
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("finite original ink"),
        bind_group_layouts: &groups,
        immediate_size: 0,
    });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let flags = [wgpu::VertexAttribute {
        format: if varying {
            wgpu::VertexFormat::Uint32x2
        } else {
            wgpu::VertexFormat::Uint32
        },
        offset: 0,
        shader_location: 8,
    }];
    buffers.push(wgpu::VertexBufferLayout {
        array_stride: if varying { 8 } else { 0 },
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &flags,
    });
    let mut depth = crate::depth::ink();
    depth.depth_compare = Some(wgpu::CompareFunction::Always);
    depth.bias = Default::default();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("finite original ink"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(if picking { "pick_vertex" } else { "vertex" }),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(if picking { "pick_fragment" } else { "fragment" }),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: if picking {
                    wgpu::TextureFormat::Rg32Uint
                } else {
                    format
                },
                blend: if picking {
                    None
                } else {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                },
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
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

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-23.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-25.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/finite_ink.rs`

Register actual Native cache, physical-colour and independent-query witnesses.

<details>
<summary>Locate the existing block</summary>

```text
pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-26.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 10. `src/finite_sample_tests.rs`

Exercise actual six-family GPU colour coverage0..4, fresh independent original queries, exact return and cached pipeline/layout identities; optional proof exports remain portable.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-27.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 11. `src/ink_pixels_tests.rs`

Exercise actual six-family GPU colour coverage0..4, fresh independent original queries, exact return and cached pipeline/layout identities; optional proof exports remain portable.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzz-cache-ink-pipelines-with-independent-queries-28.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.6
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Finite-ink colour pipeline/layout pairs have cached one- and four-sample recipes. Queries keep their independent one-sample pipeline and layout. Shared static attributes preserve original data offsets across all existing line, chain, point, guide and grid constructors. Complete browser rendering remains at one sample at this checkpoint.

### Native ink check

These actual isolated GPU pictures show two of four physical samples visible. The six style paths are tested independently; byte-identical pictures share one labelled figure. Complete browser rendering remains at one sample.

**stroke, grid stroke, authored stroke metadata — two of four samples visible.**

![Actual Native isolated ink: stroke, grid stroke, authored stroke metadata](../screenshots/journey/review/96zzzzzzz-cache-ink-pipelines-with-independent-queries-ink-0.png)

**headed chain — two of four samples visible.**

![Actual Native isolated ink: headed chain](../screenshots/journey/review/96zzzzzzz-cache-ink-pipelines-with-independent-queries-ink-1.png)

**point, authored point metadata — two of four samples visible.**

![Actual Native isolated ink: point, authored point metadata](../screenshots/journey/review/96zzzzzzz-cache-ink-pipelines-with-independent-queries-ink-2.png)


**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample browser contact scene after cached finite-ink pipelines, matching bindings and shared static vertex attributes. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native figures show isolated four-sample ink across six style paths. Complete browser multisampling and whole-course visual acceptance remain unfinished.

![Actual single-sample browser contact scene after cached finite-ink pipelines, matching bindings and shared static vertex attributes. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native figures show isolated four-sample ink across six style paths. Complete browser multisampling and whole-course visual acceptance remain unfinished.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Move the existing vertex-attribute macros into one small static recipe shared by strokes, chains, markers, guides, wide lines and grid drawing. The data layouts and field offsets stay unchanged. Finite ink borrows these slices instead of allocating another attribute vector.

Move the existing finite-ink binding descriptor into a count-specific factory. Its depth and primitive bindings agree with the colour pipeline and shader. Retain a separate single-sample query layout; colour and query resource bindings use their own layouts.

The shared pipeline owner creates a missing alternate pair once, then swaps retained recipes. Repeating the same count preserves current bindings. A real switch releases old depth/primitive bindings so they cannot retain inactive frame images or be used with the wrong layout. Current main-frame lanes still request one sample until the remaining consumers are connected.

Shared static vertex attributes → matching colour pipeline/layout pair → cached count switch → fresh independent query bindings.



Why must the query layout stay separate from the current colour layout?

A colour pass can read multisampled depth and primitive images while the fresh integer query reads its own single-sample images. A shared current layout would bind the wrong texture kind after a colour count change.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the supplied actual GPU witnesses across six finite-ink styles. Reveal zero through four physical samples, query independently, and switch4→1→4. Require exact return pixels and retained pipeline identity. Repeat the same count and check that current image bindings stay intact; change count and check that stale bindings release.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.6
npm --prefix ../session_tests run course -- save 22.5.6
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Existing vertex-layout macros move into one static recipe shared by strokes, chains, markers, guides, wide lines and grid drawing. Finite ink borrows those slices instead of allocating another attribute vector. Colour pipelines and binding layouts match the active sample count and share the small alternate-recipe cache. A real switch releases old image bindings; repeating the current count preserves them. The query pipeline and layout remain independently single-sample. Two fresh actual Native GPU fixtures exercise six style paths, lazy cached pipeline identity, binding release/preservation, zero through four visible physical samples, fresh query visibility and exact return pixels. The isolated integer fixture reads its real input tuple [17,1]; full original document owners are covered separately by canonical browser checks. All six actual Native image hashes are retained, with three unique visibly labelled figures displayed after direct inspection. All 995 Native tests, WASM/optimized builds, native frame, canonical Chrome original-contact/resize checks and stored evidence pass; ten additional current canonical Native/browser pictures were individually inspected. Complete browser rendering remains one-sample until all consumers are connected. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

Two actual GPU fixtures cover six colour/query layout combinations, lazy pipeline identity, binding release/preservation, physical sample visibility0..4, exact return pixels and independent single-sample integer queries. Preserve existing shader, original line/chain/point, sloped-face and grid assertions. Optional Native pixel exports remain portable.

Actual single-sample browser contact scene after cached finite-ink pipelines, matching bindings and shared static vertex attributes. Both densities preserve original face picks, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native figures show isolated four-sample ink across six style paths. Complete browser multisampling and whole-course visual acceptance remain unfinished.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.6
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
