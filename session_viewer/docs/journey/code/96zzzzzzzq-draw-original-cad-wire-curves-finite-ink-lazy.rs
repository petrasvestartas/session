fn pipeline(device:&wgpu::Device,format:wgpu::TextureFormat,stride:u64,grid:bool,picking:bool,varying:bool,samples:u32,view:&wgpu::BindGroupLayout,crop:&wgpu::BindGroupLayout,layout:&wgpu::BindGroupLayout,attributes:&[wgpu::VertexAttribute])->wgpu::RenderPipeline{
   let source=crate::finite_sources::source_with_samples(stride,picking,grid,varying,samples);let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("finite original ink"),source:wgpu::ShaderSource::Wgsl(source.into())});
   let clipping=crate::clipping_gpu::layout(device);let groups=if picking{vec![Some(view),Some(crop),Some(layout),Some(&clipping)]}else{vec![Some(view),Some(layout),Some(&clipping)]};let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("finite original ink"),bind_group_layouts:&groups,immediate_size:0});
   let mut buffers=vec![wgpu::VertexBufferLayout{array_stride:stride,step_mode:wgpu::VertexStepMode::Instance,attributes}];let ids=wgpu::vertex_attr_array![7=>Uint32x2];if picking{buffers.push(wgpu::VertexBufferLayout{array_stride:8,step_mode:wgpu::VertexStepMode::Instance,attributes:&ids});}
   let flags=[wgpu::VertexAttribute{format:if varying{wgpu::VertexFormat::Uint32x2}else{wgpu::VertexFormat::Uint32},offset:0,shader_location:8}];buffers.push(wgpu::VertexBufferLayout{array_stride:if varying{8}else{0},step_mode:wgpu::VertexStepMode::Instance,attributes:&flags});
   let mut depth=crate::depth::ink();depth.depth_compare=Some(wgpu::CompareFunction::Always);depth.bias=Default::default();
   device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("finite original ink"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(if picking{"pick_vertex"}else{"vertex"}),compilation_options:Default::default(),buffers:&buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(if picking{"pick_fragment"}else{"fragment"}),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState{format:if picking{wgpu::TextureFormat::Rg32Uint}else{format},blend:if picking{None}else{Some(wgpu::BlendState::ALPHA_BLENDING)},write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(depth),multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
  }
use std::cell::{OnceCell, RefCell};
use std::rc::Rc;
use crate::stroke_gpu::attributes;

#[derive(Debug)]
struct DeferredPipeline {
    device: wgpu::Device,
    spec: (wgpu::TextureFormat, u64, bool, bool, bool, u32),
    view: wgpu::BindGroupLayout,
    crop: wgpu::BindGroupLayout,
    coverage: wgpu::BindGroupLayout,
    value: OnceCell<wgpu::RenderPipeline>,
}

#[derive(Clone, Debug)]
struct Pipeline(Rc<DeferredPipeline>);

impl PartialEq for Pipeline {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}
impl Eq for Pipeline {}

impl Pipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat, stride: u64,
        grid: bool, picking: bool, varying: bool, samples: u32,
        view: &wgpu::BindGroupLayout, crop: &wgpu::BindGroupLayout,
        coverage: &wgpu::BindGroupLayout) -> Self {
        Self(Rc::new(DeferredPipeline {
            device: device.clone(), spec: (format, stride, grid, picking, varying, samples),
            view: view.clone(), crop: crop.clone(), coverage: coverage.clone(), value: OnceCell::new(),
        }))
    }

    fn get(&self) -> &wgpu::RenderPipeline {
        self.0.value.get_or_init(|| {
            let (format, stride, grid, picking, varying, samples) = self.0.spec;
            pipeline(&self.0.device, format, stride, grid, picking, varying, samples,
                &self.0.view, &self.0.crop, &self.0.coverage, attributes(stride))
        })
    }
}

pub struct Ink {
    color: crate::sample_pipelines::Variants<(Pipeline, wgpu::BindGroupLayout)>,
    pick: Pipeline,
    query: wgpu::BindGroupLayout,
    spec: (wgpu::TextureFormat, u64, bool, bool),
    bindings: RefCell<Option<(wgpu::Buffer, wgpu::Buffer, wgpu::TextureView, wgpu::TextureView, wgpu::BindGroup)>>,
}

#[derive(Clone)]
pub struct Tables { pub projected: wgpu::Buffer, pub tiles: wgpu::Buffer, pub primitive: wgpu::TextureView }

fn layout(device: &wgpu::Device, samples: u32) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("finite ink coverage"),
        entries: &[
            crate::tile_pipelines::entry(0, wgpu::ShaderStages::FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            crate::tile_pipelines::entry(1, wgpu::ShaderStages::FRAGMENT, wgpu::BufferBindingType::Storage { read_only: true }),
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Depth,
                    view_dimension: wgpu::TextureViewDimension::D2, multisampled: samples == 4 }, count: None },
            crate::text_gpu_support::texture_entry(4, wgpu::ShaderStages::FRAGMENT, wgpu::TextureSampleType::Uint, samples == 4),
        ],
    })
}

impl Ink {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, stride: u64, grid: bool, varying: bool) -> Self {
        Self::with_samples(device, format, stride, grid, varying, 1)
    }

    pub fn with_samples(device: &wgpu::Device, format: wgpu::TextureFormat, stride: u64, grid: bool, varying: bool, samples: u32) -> Self {
        assert!(matches!(samples, 1 | 4));
        let view = crate::surface_view::layout(device);
        let crop = crate::gpu_layout::uniform(device, 80);
        let query = layout(device, 1);
        let coverage = if samples == 1 { query.clone() } else { layout(device, samples) };
        let color = Pipeline::new(device, format, stride, grid, false, varying, samples, &view, &crop, &coverage);
        let pick = Pipeline::new(device, format, stride, grid, true, varying, 1, &view, &crop, &query);
        Self { color: crate::sample_pipelines::Variants::new((color, coverage), samples), pick, query,
            spec: (format, stride, grid, varying), bindings: RefCell::new(None) }
    }

    pub fn set_samples(&mut self, device: &wgpu::Device, samples: u32) {
        let (format, stride, grid, varying) = self.spec;
        if self.color.select(samples, || {
            let coverage = if samples == 1 { self.query.clone() } else { layout(device, samples) };
            let view = crate::surface_view::layout(device);
            let crop = crate::gpu_layout::uniform(device, 80);
            (Pipeline::new(device, format, stride, grid, false, varying, samples, &view, &crop, &coverage), coverage)
        }) { self.bindings.get_mut().take(); }
    }

    pub fn clear_bindings(&mut self) { self.bindings.get_mut().take(); }

    fn group(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, tables: &Tables, depth: &wgpu::TextureView) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("finite original ink coverage"), layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: tables.projected.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: tables.tiles.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(depth) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&tables.primitive) },
            ],
        })
    }

    pub fn set(&self, device: &wgpu::Device, tables: &Tables, depth: &wgpu::TextureView) {
        self.color.active.0.get();
        let mut bindings = self.bindings.borrow_mut();
        if bindings.as_ref().is_some_and(|(p,t,d,i,_)| p == &tables.projected && t == &tables.tiles && d == depth && i == &tables.primitive) { return; }
        let group = self.group(device, &self.color.active.1, tables, depth);
        *bindings = Some((tables.projected.clone(), tables.tiles.clone(), depth.clone(), tables.primitive.clone(), group));
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup, vertices: &wgpu::Buffer,
        flags: &wgpu::Buffer, count: u32, verts: u32, ranges: &crate::ink_ranges::Runs) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_,_,_,_,group)) = bindings.as_ref() else { return false; };
        pass.set_pipeline(self.color.active.0.get());
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, flags.slice(..));
        for instances in ranges.visible(count) { pass.draw(0..verts, instances); }
        true
    }

    pub fn pick(&self, device: &wgpu::Device, pass: &mut wgpu::RenderPass<'_>, view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup, vertices: &wgpu::Buffer, ids: &wgpu::Buffer, flags: &wgpu::Buffer,
        count: u32, verts: u32, tables: &Tables, depth: &wgpu::TextureView, ranges: &crate::ink_ranges::Runs) {
        if count == 0 || ranges.visible(count).all(|range| range.is_empty()) { return; }
        let group = self.group(device, &self.query, tables, depth);
        pass.set_pipeline(self.pick.get());
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_bind_group(2, &group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
        pass.set_vertex_buffer(2, flags.slice(..));
        for instances in ranges.visible(count) { pass.draw(0..verts, instances); }
    }
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="finite_sample_tests.rs"]
mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_pixels_tests.rs"]
mod pixels_tests;
