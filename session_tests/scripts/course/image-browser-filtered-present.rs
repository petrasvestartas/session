fn course_image_draw(renderer: &Renderer, output: &wgpu::Texture, output_view: &wgpu::TextureView, background: &Background, transform: &[f32; 16], normals: bool) {
    let window = web_sys::window().expect("Image presentation window");
    if course_image_scale(&window) == 1.0 {
        renderer.draw_mode(output_view, background, transform, normals);
        return;
    }
    let format = output.format().add_srgb_suffix();
    assert!(matches!(format, wgpu::TextureFormat::Rgba8UnormSrgb | wgpu::TextureFormat::Bgra8UnormSrgb));
    let width = output.width().checked_mul(2).expect("Image width");
    let height = output.height().checked_mul(2).expect("Image height");
    let device = &renderer.device;
    assert!(width <= device.limits().max_texture_dimension_2d && height <= device.limits().max_texture_dimension_2d);
    let source = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("image profile scene"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
    });
    let source_view = source.create_view(&Default::default());
    renderer.draw_mode(&source_view, background, transform, normals);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("image profile linear presentation"), source: wgpu::ShaderSource::Wgsl(COURSE_PRESENT_SHADER.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("image profile linear presentation"), layout: None,
        vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
            targets: &[Some(wgpu::ColorTargetState { format, blend: None, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: Default::default(), depth_stencil: None, multisample: Default::default(), multiview_mask: None, cache: None,
    });
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
                view: output_view, depth_slice: None, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT), store: wgpu::StoreOp::Store },
            })], ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &binding, &[]);
        pass.draw(0..3, 0..1);
    }
    renderer.queue.submit([encoder.finish()]);
    window.document().and_then(|document| document.get_element_by_id("canvas")).expect("Image canvas")
        .set_attribute("data-image-gpu-present", &serde_json::json!({"presentation":"linear-srgb-gpu", "filter":"linear",
            "format":format!("{format:?}"), "samples":1, "internal":[width,height], "output":[output.width(),output.height()],
            "panel":"output-resolution"}).to_string()).expect("Image GPU presentation diagnostics");
}
