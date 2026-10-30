fn course_filtered_present(renderer: &Renderer, source: &wgpu::Texture, format: wgpu::TextureFormat) -> wgpu::Texture {
    assert_eq!(format, wgpu::TextureFormat::Rgba8UnormSrgb);
    assert_eq!([source.width(), source.height()], [OUTPUT_WIDTH * 2, OUTPUT_HEIGHT * 2]);
    let device = &renderer.device;
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("image profile output"),
        size: wgpu::Extent3d { width: OUTPUT_WIDTH, height: OUTPUT_HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC, view_formats: &[],
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("image profile linear presentation"),
        source: wgpu::ShaderSource::Wgsl(COURSE_PRESENT_SHADER.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("image profile linear presentation"), layout: None,
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
            targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: Default::default(), depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
    });
    let source_view = source.create_view(&Default::default());
    let output_view = target.create_view(&Default::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        min_filter: wgpu::FilterMode::Linear, mag_filter: wgpu::FilterMode::Linear, ..Default::default()
    });
    let binding = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("image profile input"), layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&source_view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("image profile output"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view, depth_slice: None, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })], ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &binding, &[]);
        pass.draw(0..3, 0..1);
    }
    renderer.queue.submit([encoder.finish()]);
    let mut proof = std::fs::OpenOptions::new().write(true).create_new(true).open("frame-render-scale.json").unwrap();
    writeln!(proof, "{{\"renderScale\":2,\"internal\":[{},{}],\"output\":[{},{}],\"density\":{},\"format\":\"Rgba8UnormSrgb\",\"filter\":\"linear\",\"samples\":1}}",
        source.width(), source.height(), target.width(), target.height(), IMAGE_DENSITY).unwrap();
    target
}
