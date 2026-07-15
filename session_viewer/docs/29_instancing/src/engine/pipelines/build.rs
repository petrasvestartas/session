use session_rust::RenderVertex;

pub const MSAA_SAMPLES: u32 = 4;

pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
    time_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    // Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("triangle.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/triangle.wgsl").into()),
    });

    // Layout - what external data the shader reads. Ours is empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("triangle.layout"),
        bind_group_layouts: &[Some(aspect_layout), Some(time_layout), Some(instance_layout)],
        immediate_size: 0,
    });

    // The recipe itself
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: Some("triangle"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[RenderVertex::layout()], // Pass vertex struct
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Greater),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState{
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

pub fn build_grid_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    // Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("grid.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/grid.wgsl").into()), //  change name
    });

    // Layout - what external data the shader reads. Ours is empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("grid.layout"),
        bind_group_layouts: &[Some(aspect_layout)],
        immediate_size: 0,
    });

    // The recipe itself
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: Some("grid"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],    // ← no vertex buffer; positions come from @builtin(vertex_index)
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::Greater),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState{
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

pub fn build_edges_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    // Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("edges.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/edges.wgsl").into()), //  change name
    });

    // Layout - what external data the shader reads. Ours is empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("edges.layout"),
        bind_group_layouts: &[Some(aspect_layout)],
        immediate_size: 0,
    });

    // The recipe itself
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: Some("edges"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[RenderVertex::layout()],    // ← no vertex buffer; positions come from @builtin(vertex_index)
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::Greater),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState{
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

pub fn build_background_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {

    // Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("background.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/background.wgsl").into()), //  change name
    });

    // Layout - what external data the shader reads. Ours is empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("background.layout"),
        bind_group_layouts: &[],
        immediate_size: 0,
    });

    // The recipe itself
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: Some("background"),
            layout: Some(&layout),
            vertex: wgpu::VertexState{
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],    // ← no vertex buffer; positions come from @builtin(vertex_index)
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState{
                    format: color_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false), // never blocks later fragmenets
                depth_compare: Some(wgpu::CompareFunction::Always), // always draws (z is at the far plane)
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState{
                count: MSAA_SAMPLES,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}
