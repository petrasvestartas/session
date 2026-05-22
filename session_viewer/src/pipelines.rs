pub struct Pipelines {
    pub pipeline_shaded: wgpu::RenderPipeline,
    pub pipeline_color: wgpu::RenderPipeline,
    // pub pipeline_wireframe: wgpu::RenderPipeline,
    // pub pipeline_points:    wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            pipeline_shaded: Self::create_shaded(device, format),
            pipeline_color: Self::create_color(device, format),
            // pipeline_wireframe: Self::create_wireframe(device, format),
            // pipeline_points:    Self::create_points(device, format),
        }
    }

    fn create_color(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let pipeline_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Color Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Color Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Color Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &pipeline_shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &pipeline_shader,
                entry_point: Some("fs_color"), // uses position as color
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }

    fn create_shaded(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        let pipeline_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &pipeline_shader,
                entry_point: Some("vs_main"),
                buffers: &[], // no vertex buffers yet — positions hardcoded in shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &pipeline_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE), // new pixel fully replaces old
                    write_mask: wgpu::ColorWrites::ALL,     // write R, G, B, A
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // every 3 vertices = 1 triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // counter-clockwise winding = front face
                cull_mode: Some(wgpu::Face::Back), // skip back-facing triangles
                polygon_mode: wgpu::PolygonMode::Fill, // solid fill (Line requires a GPU feature)
                unclipped_depth: false,           // clip fragments outside z 0..1
                conservative: false,              // standard rasterization
            },
            depth_stencil: None, // no depth testing yet
            multisample: wgpu::MultisampleState {
                count: 1, // no MSAA yet
                mask: !0, // all samples active
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None, // not rendering to multiple views
            cache: None,          // no pipeline cache
        })
    }
}
