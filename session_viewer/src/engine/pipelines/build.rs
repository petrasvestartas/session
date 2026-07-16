use session_rust::RenderVertex;

pub const MSAA_SAMPLES: u32 = 4;
const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];
const CYL_TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];


// This helps the GPU to read the second vertex buffer - the instance row id.
// Without a layout description, the pipeline doesn' know those bytes exists and in what shape they are.
fn instance_id_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout{
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex, // one u32 per vertex
        attributes: &INSTANCE_ID_ATTRIBS // advances per-vertex, like position
    }
}

fn cyl_template_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout {
        array_stride: 12, // one vec3<f32> per templete vertex
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &CYL_TEMPLATE_ATTRIBS,
    }
}

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
                buffers: &[RenderVertex::layout(), instance_id_layout()], // Pass vertex struct and instnace layout
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

pub fn build_cylinder_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    segment_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("cylinder.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/cylinder.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("cylinder.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout), Some(segment_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("cylinder"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[cyl_template_layout()],   // slot 0 — the unit-cylinder positions
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,                     // thin tubes — keep both faces
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),     // solid tubes occlude correctly, no bias needed
            depth_compare: Some(wgpu::CompareFunction::Greater),  // reverse-Z (lesson 26)
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: MSAA_SAMPLES, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
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
