use crate::gpu_mesh::GpuMesh;
use wgpu::util::DeviceExt;

struct Row { group: wgpu::BindGroup, bytes: u64 }

pub struct Faces { pipeline: wgpu::RenderPipeline, rows: Vec<Option<Row>> }

fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let mut entries = Vec::new();
    for binding in 0..4 { entries.push(wgpu::BindGroupLayoutEntry { binding, visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer { ty: if binding == 3 { wgpu::BufferBindingType::Uniform }
            else { wgpu::BufferBindingType::Storage { read_only: true } }, has_dynamic_offset: false, min_binding_size: None }, count: None }); }
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some("source face metadata"), entries: &entries })
}

impl Faces {
    pub fn new(device: &wgpu::Device) -> Self {
        let storage = layout(device); let view = crate::gpu_layout::uniform(device, 80);
        let model = crate::gpu_layout::uniform(device, 144); let crop = crate::gpu_layout::uniform(device, 80);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("source face IDs"),
            bind_group_layouts: &[Some(&view), Some(&model), Some(&storage), Some(&crop)], immediate_size: 0 });
        let source = concat!(include_str!("triangle.wgsl"), "\n", include_str!("pick_crop.wgsl"), "\n", include_str!("face_pick.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("source faces"), source: wgpu::ShaderSource::Wgsl(source.into()) });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("source face IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("face_vertex"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("face_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(), depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(), multiview_mask: None, cache: None });
        Self { pipeline, rows: Vec::new() }
    }

}
