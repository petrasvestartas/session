se});
    let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor{label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&r.targets.depth)},wgpu::BindGroupEntry{binding:1,resource:output.as_entire_binding()}]});
    le