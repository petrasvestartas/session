# 22.6.1 · Retain visible instance ranges

**Chapter 22 · Surface lighting, contact and display quality · Section 22.6 · Original display controls** · [Contents](contents.md)

**Typing: 28–56 minutes.** [Estimate](typing-load.md).

Retain CPU runs beside each existing ink lane. A visible run draws from the original instance buffer at its absolute first instance, so colour, flags and original query addresses stay aligned. No geometry copies or GPU family buffer are added.

## Type

Continue from [Choose bounded frame quality](96zzzzzzzl-choose-bounded-frame-quality.md). [Save or recover your work](recovery.md).

### 1. `src/ink_ranges.rs`

Retain merged contiguous CPU instance ranges and filter draw submission without copying geometry or rebasing GPU instance indices. Empty metadata preserves the existing whole lane. Retain one complete draw when all populated families remain visible, and skip the metadata scan when no populated family is visible.

Create the file and type:

```rust
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-01.rs"
```

### 2. `src/lib.rs`

Share one allocation-free range iterator between ordinary and finite colour/query drawing.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod finite_ink;
```

</details>

After that block, type:

```rust
pub mod ink_ranges;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-02.rs"
```

</details>

### 3. `src/stroke_gpu.rs`

Keep presentation metadata beside the existing lane; it owns no GPU resource. Default to complete retained draws so existing lanes preserve their behaviour. Use the same CPU ranges in finite colour drawing. Use absolute retained ranges for finite integer queries too. Keep full buffer bindings and absolute first-instance values in draw; do not compact source addresses. Keep full buffer bindings and absolute first-instance values in pick; do not compact source addresses.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, recipe:(format,depth,physical),samples:1, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

</details>

Replace it with:

```rust
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {ranges:Default::default(),flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, recipe:(format,depth,physical),samples:1, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
    }

    fn layout_depth(device: &wgpu::Device, format: wgpu::TextureFormat, source: &str,
```

<details>
<summary>Locate this block</summary>

```rust
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }
```

</details>

Replace it with:

```rust
        let (Some(vertices), Some(ids), Some(pipeline)) = (&self.vertices, &self.ids, &self.pick_pipeline) else { return; };
        pass.set_pipeline(pipeline); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..)); pass.set_vertex_buffer(1, ids.slice(..));
        for instances in self.ranges.visible((self.data.len() as u64/self.stride)as u32){pass.draw(0..self.vertices_per_instance,instances);}
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth,&self.ranges)}else{self.pick(pass,view,crop)}
    }
    pub fn pick_bytes(&self) -> u64 { self.ids.as_ref().map_or(0, wgpu::Buffer::size) }
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if self.data.is_empty(){return;}if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance)){return}
        pass.set_pipeline(&self.pipeline.active); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..self.vertices_per_instance, 0..(self.data.len() as u64 / self.stride) as u32);
    }

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
```

</details>

Replace it with:

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if self.data.is_empty(){return;}if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,&self.ranges)){return}
        pass.set_pipeline(&self.pipeline.active); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        for instances in self.ranges.visible((self.data.len() as u64/self.stride)as u32){pass.draw(0..self.vertices_per_instance,instances);}
    }

    pub fn set_ids(&mut self, device: &wgpu::Device, ids: impl IntoIterator<Item = [u32; 2]>) -> Result<(), &'static str> {
```

<details>
<summary>Locate this block</summary>

```rust
}

pub struct Lane {
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
```

</details>

Replace it with:

```rust
}

pub struct Lane {
    pub ranges:crate::ink_ranges::Runs,
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
```

<details>
<summary>Complete updated block reference</summary>

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
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink settings"), bind_group_layouts: &[Some(&view_layout)], immediate_size: 0 });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"), source: wgpu::ShaderSource::Wgsl(source.into()),
        });
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
            multisample: wgpu::MultisampleState {count:samples,..Default::default()}, multiview_mask: None, cache: None,
        });
 pipeline
}
pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
 const STROKE:[wgpu::VertexAttribute;4]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32];
 const CHAIN:[wgpu::VertexAttribute;7]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32];
 const MARKER:[wgpu::VertexAttribute;3]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4];
 const GUIDE:[wgpu::VertexAttribute;5]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32];
 match stride {44=>&STROKE,72=>&CHAIN,32=>&MARKER,48=>&GUIDE,_=>unreachable!()}
}

pub struct Lane {
    pub ranges:crate::ink_ranges::Runs,
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: crate::sample_pipelines::Variants<wgpu::RenderPipeline>,
    recipe:(wgpu::TextureFormat,wgpu::DepthStencilState,bool),samples:u32,
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
        self.data = data; if self.finite.is_some(){self.reset_flags(device)?;}self.set_samples(device,self.samples); Ok(())
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
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,&self.ranges)){return}
        pass.set_pipeline(&self.pipeline.active); pass.set_bind_group(0, view, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        for instances in self.ranges.visible((self.data.len() as u64/self.stride)as u32){pass.draw(0..self.vertices_per_instance,instances);}
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
        for instances in self.ranges.visible((self.data.len() as u64/self.stride)as u32){pass.draw(0..self.vertices_per_instance,instances);}
    }

    pub fn pick_finite(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,tables:Option<&crate::finite_ink::Tables>,depth:&wgpu::TextureView){
        if let(Some(ink),Some(tables),Some(vertices),Some(ids))=(&self.finite,tables,&self.vertices,&self.ids){ink.pick(device,pass,view,crop,vertices,ids,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,tables,depth,&self.ranges)}else{self.pick(pass,view,crop)}
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
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        }));
        Self {ranges:Default::default(),flags:None,flag_data:Vec::new(),varying:false,finite:None,strict_depth, pipeline, recipe:(format,depth,physical),samples:1, pick_pipeline, ids: None, id_data: Vec::new(),  vertices: None, data: Vec::new(), stride, vertices_per_instance, uploads: 0, uploaded_bytes: 0 }
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

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="stroke_sample_tests.rs"]
mod sample_tests;
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-08.rs"
```

</details>

### 4. `src/finite_ink.rs`

Give finite drawing the retained CPU range iterator. Retain one query bind-group preparation before all visible absolute draws. Apply identical visible instance ranges to finite draw without rebinding or copying the original vertex/ID/metadata buffers. Apply identical visible instance ranges to finite pick without rebinding or copying the original vertex/ID/metadata buffers.

<details>
<summary>Locate the existing block</summary>

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

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="finite_sample_tests.rs"]
mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_pixels_tests.rs"]
mod pixels_tests;
```

</details>

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
 pub fn draw(&self,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,vertices:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,ranges:&crate::ink_ranges::Runs)->bool{let bindings=self.bindings.borrow();let Some((_,_,_,_,group))=bindings.as_ref()else{return false};pass.set_pipeline(&self.color.active.0);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,flags.slice(..));for instances in ranges.visible(count){pass.draw(0..verts,instances);}true}
 pub fn pick(&self,device:&wgpu::Device,pass:&mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,vertices:&wgpu::Buffer,ids:&wgpu::Buffer,flags:&wgpu::Buffer,count:u32,verts:u32,tables:&Tables,depth:&wgpu::TextureView,ranges:&crate::ink_ranges::Runs){let group=self.group(device,&self.query,tables,depth);pass.set_pipeline(&self.pick);pass.set_bind_group(0,view,&[]);pass.set_bind_group(1,crop,&[]);pass.set_bind_group(2,&group,&[]);pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));for instances in ranges.visible(count){pass.draw(0..verts,instances);}}
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="finite_sample_tests.rs"]
mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_pixels_tests.rs"]
mod pixels_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-12.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 5. `src/ink_range_gpu_tests.rs`

Actual Native GPU witness: nonzero first-instance ranges retain the original middle colour and original owner, hide both outside owners, restore exact pixels, and retain all source records/uploads in finite and ordinary paths at one/four samples.

Copy this supplied file:

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-13.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/renderer.rs`

Supply the actual GPU range/address regression as a child of the renderer.

<details>
<summary>Locate the existing block</summary>

```text
pub struct Renderer {
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-14.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/ink_pixels_tests.rs`

Adapt the existing supplied actual finite-ink GPU observer to the explicit all-visible range argument, preserving every coverage/owner/pixel assertion.

<details>
<summary>Locate the existing block</summary>

```text
ink.draw(&mut pass,view,vertices,flags,1,if stride==72 {18}else{6})
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-15.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/ink_pixels_tests.rs`

Adapt the existing supplied actual finite-ink GPU observer to the explicit all-visible range argument, preserving every coverage/owner/pixel assertion.

<details>
<summary>Locate the existing block</summary>

```text
ink.pick(device,&mut pass,view,&crop,vertices,&ids,flags,1,if stride==72 {18}else{6},tables,&target.depth)
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzm-retain-visible-instance-ranges-16.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.6.1
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Keep absolute original query addresses while filtering existing instance buffers with merged CPU ranges. Source-only preparation is not acceptance; actual complete application, source-bound pictures and published UI gates remain required.

**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual controlled single-sample original contact fixture. All source/owner and target-reuse checks remain active; this picture is a regression control, not visual proof of every display family. Procedural original-plane display, complete original CAD wire display, pen/cloud settings, runtime efficiency, Cargo independence and final feature-picture review remain unfinished. Maintained and owned Session reload camera/base-grid fixes follow the completed course.

![Actual controlled single-sample original contact fixture. All source/owner and target-reuse checks remain active; this picture is a regression control, not visual proof of every display family. Procedural original-plane display, complete original CAD wire display, pen/cloud settings, runtime efficiency, Cargo independence and final feature-picture review remain unfinished. Maintained and owned Session reload camera/base-grid fixes follow the completed course.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The common all-visible case draws one whole lane. Empty masks skip it. Partial masks visit only merged runs prepared with the source. The range helper binds the full vertex, metadata and original-ID buffers; it never slices an ID buffer relative to a filtered run.

The supplied actual GPU witness uses three distinct original points and shows only the middle instance. It checks its real colour and original owner, excludes the other instances and restores exact pixels at one/four samples in the finite and retained legacy paths. CPU source records and GPU upload/geometry identity remain unchanged.

This checkpoint adds the range mechanism. Source-family classification and the public visibility owner follow in the same display-controls section. The standard Chrome fixture retains all original target-reuse assertions under an explicit one-sample URL; automatic/forced/recovery quality has separate coverage.

Original source rows → merged CPU ranges → unchanged colour and query buffers.



Why must a filtered run retain its original first instance?

The vertex, metadata and original-ID buffers still use absolute instance positions. Rebasing only one buffer would draw or pick another original object.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect actual source-specific controls and exact pixel returns at one/four samples. Keep original owner keys, source archives and GPU uploads unchanged.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.6.1
npm --prefix ../session_tests run course -- save 22.6.1
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Keep absolute original query addresses while filtering existing instance buffers with merged CPU ranges. Procedural original-plane display, complete original CAD wire display, pen/cloud settings, runtime efficiency, Cargo independence and final feature-picture review remain unfinished. Maintained and owned Session reload camera/base-grid fixes follow the completed course. All 1010 Native tests, WASM/optimized builds, Native frame, canonical Chrome contact/resize checks and stored evidence pass. Ten current canonical Native/browser pictures were individually inspected. Procedural original-plane display, complete original CAD wire display, pen/cloud settings, runtime efficiency, Cargo independence and final feature-picture review remain unfinished. Maintained and owned Session reload camera/base-grid fixes follow the completed course.

Retain every inherited source, geometry, independent query and cache assertion. Run the exact supplied GPU witnesses, full Native suite, optimized owned application and canonical browser checks, then inspect each actual current picture and published UI.

Actual controlled single-sample original contact fixture. All source/owner and target-reuse checks remain active; this picture is a regression control, not visual proof of every display family. Procedural original-plane display, complete original CAD wire display, pen/cloud settings, runtime efficiency, Cargo independence and final feature-picture review remain unfinished. Maintained and owned Session reload camera/base-grid fixes follow the completed course.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.6.1
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
