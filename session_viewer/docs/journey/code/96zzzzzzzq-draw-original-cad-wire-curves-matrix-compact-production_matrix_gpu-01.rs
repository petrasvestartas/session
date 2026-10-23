            storage(6, vf),
            storage(7, ShaderStages::FRAGMENT),
        ],
        vec![storage(0, vf), storage(1, ShaderStages::FRAGMENT)],
    ];
    entries.map(|entries| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
