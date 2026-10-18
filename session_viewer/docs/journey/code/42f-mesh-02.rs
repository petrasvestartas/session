pub fn mesh(device: &wgpu::Device, view_layout: &wgpu::BindGroupLayout,
    model_layout: &wgpu::BindGroupLayout, crop_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("mesh IDs"),
            bind_group_layouts: &[Some(view_layout), Some(model_layout), Some(crop_layout)], immediate_size: 0 });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("mesh IDs"),
            source: wgpu::ShaderSource::Wgsl(concat!(include_str!("triangle.wgsl"), "\n", include_str!("triangle_pick.wgsl")).into()) });
        // An integer attachment stores exact owner IDs; opaque depth rejects hidden faces before any pixel is read back.
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("mesh IDs"), layout: Some(&layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("pick_vertex"), compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: 24, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3] },
                    wgpu::VertexBufferLayout { array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![2 => Float32x3] }] },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("pick_fragment"), compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rg32Uint.into())] }), primitive: Default::default(),
            depth_stencil: Some(crate::depth::surface()), multisample: Default::default(), multiview_mask: None, cache: None,
        })
}
