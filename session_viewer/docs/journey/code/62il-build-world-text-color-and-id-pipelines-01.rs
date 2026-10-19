        ]);
    }
}


fn pipeline(ctx: &GpuCtx, target: Target, layout: &wgpu::BindGroupLayout, ids: bool) -> Pipeline {
    let device = ctx.device.clone();
    let layout = layout.clone();
    let pick = crate::gpu_layout::uniform(&ctx.device, 80);
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world text shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_plane.wgsl").into()),
        });
        let colour_groups = [Some(&layout)];
        let id_groups = [Some(&layout), Some(&pick)];
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("world text"),
            bind_group_layouts: if ids { &id_groups } else { &colour_groups },
            immediate_size: 0,
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world text"), layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if ids { "vs_id" } else { "vs_main" }), buffers: &[wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Uint32, 5 => Float32] }], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
        })
    }
}
