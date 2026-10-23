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

use std::cell::OnceCell;
use std::rc::Rc;

#[derive(Debug)]
struct DeferredPipeline {
    device: wgpu::Device,
    source: String,
    stride: u64,
    attributes: Vec<wgpu::VertexAttribute>,
    spec: Option<(wgpu::TextureFormat, wgpu::DepthStencilState, bool, u32)>,
    value: OnceCell<wgpu::RenderPipeline>,
}

#[derive(Clone, Debug)]
struct Pipeline(Rc<DeferredPipeline>);

impl PartialEq for Pipeline {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}
impl Eq for Pipeline {}

impl Pipeline {
    fn new(device: &wgpu::Device, source: &str, stride: u64, attributes: &[wgpu::VertexAttribute],
        spec: Option<(wgpu::TextureFormat, wgpu::DepthStencilState, bool, u32)>) -> Self {
        Self(Rc::new(DeferredPipeline {
            device: device.clone(), source: source.into(), stride, attributes: attributes.into(),
            spec, value: OnceCell::new(),
        }))
    }

    fn get(&self) -> &wgpu::RenderPipeline {
        self.0.value.get_or_init(|| match &self.0.spec {
            Some((format, depth, physical, samples)) => pipeline(&self.0.device, *format,
                &self.0.source, self.0.stride, &self.0.attributes, depth.clone(), *physical, *samples),
            None => pick_pipeline(&self.0.device, &self.0.source, self.0.stride, &self.0.attributes),
        })
    }
}

fn pick_pipeline(device: &wgpu::Device, source: &str, stride: u64,
    attributes: &[wgpu::VertexAttribute]) -> wgpu::RenderPipeline {
        let view_layout=crate::surface_view::layout(device);
        let crop_layout = crate::gpu_layout::uniform(device, 80);
        let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("ink pick settings"), bind_group_layouts: &[Some(&view_layout), Some(&crop_layout)], immediate_size: 0 });
        let pick_source = match stride { 48=>include_str!("guide_pick.wgsl"),44 => include_str!("stroke_pick.wgsl"), 72 => include_str!("chain_pick.wgsl"), 32 => include_str!("marker_pick.wgsl"), _ => unreachable!() };
        let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("ink IDs"),
            source: wgpu::ShaderSource::Wgsl(format!("{source}\n{}\n{pick_source}", include_str!("pick_crop.wgsl")).into()) });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("ink IDs"), layout: Some(&pick_layout),
            vertex: wgpu::VertexState { module: &pick_shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: stride, step_mode: wgpu::VertexStepMode::Instance, attributes },
                    wgpu::VertexBufferLayout { array_stride: 8, step_mode: wgpu::VertexStepMode::Instance, attributes: &wgpu::vertex_attr_array![7 => Uint32x2] }] },
            fragment: Some(wgpu::FragmentState { module: &pick_shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}

pub struct Lane {
    pub ranges:crate::ink_ranges::Runs,
    flags:Option<wgpu::Buffer>,flag_data:Vec<u32>,varying:bool,
    finite:Option<crate::finite_ink::Ink>,
    strict_depth:bool,
    pipeline: crate::sample_pipelines::Variants<Pipeline>,
    recipe:(wgpu::TextureFormat,wgpu::DepthStencilState,bool),samples:u32,
    pick_pipeline: Option<Pipeline>,
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
            Pipeline::new(device,&source,self.stride,attributes(self.stride),Some((*format,depth.clone(),*physical,samples)))
        });
    }
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){if self.data.is_empty(){return;}if let Some(ink)=&self.finite{ink.set(device,tables,depth)}}
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>,view:&wgpu::BindGroup) {
        let Some(vertices) = &self.vertices else { return; };
        if self.ranges.visible((self.data.len() as u64/self.stride)as u32).next().is_none() { return; }
        if self.finite.as_ref().is_some_and(|ink|ink.draw(pass,view,vertices,self.flags.as_ref().expect("Finite original clipping flags"),(self.data.len()as u64/self.stride)as u32,self.vertices_per_instance,&self.ranges)){return}
        pass.set_pipeline(self.pipeline.active.get()); pass.set_bind_group(0, view, &[]);
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
        if self.ranges.visible((self.data.len() as u64/self.stride)as u32).next().is_none() { return; }
        pass.set_pipeline(pipeline.get()); pass.set_bind_group(0, view, &[]); pass.set_bind_group(1, crop, &[]);
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
        let strict_depth=depth.depth_compare==Some(crate::depth::CLOSER);
        let pipeline=crate::sample_pipelines::Variants::new(Pipeline::new(device,source,stride,attributes,Some((format,depth.clone(),physical,1))),1);
        let pick_pipeline = pickable.then(|| Pipeline::new(device,source,stride,attributes,None));
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
