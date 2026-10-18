pub fn make(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vertex: &str,
    fragment: &str,
    format: wgpu::TextureFormat,
    blend: Option<wgpu::BlendState>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("sheet batch"),layout:Some(layout),vertex:wgpu::VertexState {module:shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers:&[wgpu::VertexBufferLayout {array_stride:32,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x3,3=>Uint32]}]},fragment:Some(wgpu::FragmentState {module:shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format,blend,write_mask:wgpu::ColorWrites::ALL})]}),primitive:Default::default(),depth_stencil:Some(crate::depth::ink()),multisample:Default::default(),multiview_mask:None,cache:None})
}

pub fn pipelines(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let view_layout = crate::gpu_layout::uniform(device, 80);
    let model_layout = crate::gpu_layout::uniform(device, 96);
    let crop_layout = crate::gpu_layout::uniform(device, 80);
    let source = include_str!("sheet.wgsl");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sheet pens"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let pick_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sheet original entities"),
        source: wgpu::ShaderSource::Wgsl(
            format!(
                "{source}\n{}\n{}",
                include_str!("pick_crop.wgsl"),
                include_str!("sheet_pick.wgsl")
            )
            .into(),
        ),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sheet pens"),
        bind_group_layouts: &[Some(&view_layout), Some(&model_layout)],
        immediate_size: 0,
    });
    let pick_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sheet entity picks"),
        bind_group_layouts: &[Some(&view_layout), Some(&model_layout), Some(&crop_layout)],
        immediate_size: 0,
    });
    let pipeline = make(
        device,
        &layout,
        &shader,
        "vertex",
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
    let pick = make(
        device,
        &pick_layout,
        &pick_shader,
        "pick_vertex",
        "pick_fragment",
        wgpu::TextureFormat::Rg32Uint,
        None,
    );
    (pipeline, pick)
}
