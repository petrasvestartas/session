use session_rust::RenderVertex;

// `samples` (every builder below) is MSAA. 4 = smooth mesh silhouettes, but it quadruples
// fragment work AND framebuffer bandwidth. Linework does its OWN antialiasing (SDF alpha ramp
// in ribbon/glyph), so on a 2D sheet - 100% linework - MSAA buys nothing and costs everything.
// It cannot be mixed WITHIN a frame: sample count is a property of the render PASS, so every
// pipeline drawn into it must agree. The viewer therefore picks one per SCENE - see
// `Gpu::msaa_for`.
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
/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
fn instance_id_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout{
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex, // one u32 per vertex
        attributes: &INSTANCE_ID_ATTRIBS // advances per-vertex, like position
    }
}

/// Vertex-buffer layout for the unit-cylinder/-sphere template positions (`@location(0)`, one `vec3<f32>`).
fn cyl_template_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout {
        array_stride: 12, // one vec3<f32> per templete vertex
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &CYL_TEMPLATE_ATTRIBS
    }
}

/// Pipeline for solid mesh triangles — reverse-Z depth (write on) + MSAA; reads mvp / time / instances.
pub fn build_triangle_pipeline(
    device: &wgpu::Device,
    samples: u32,
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
                    // A surface can be translucent: a PDF sheet's shaded regions arrive at 5-40%
                    // alpha (1596 of them on one drawing), and unblended they render SOLID - the
                    // wrong colour entirely. Opaque geometry is unaffected: alpha 1 blends to
                    // itself. Translucent 3D surfaces would need back-to-front sorting; flat
                    // coplanar sheet fills do not.
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
                // No hardware bias. Faces recede in triangle.wgsl instead (FACE_PUSH), because
                // the units of `constant` on a float depth format are implementation-defined -
                // a driver may apply less than asked, or nothing, and then the wireframe gets
                // cut on one machine and not another.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState{
                count: samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

/// Pipeline for the ground grid — buffer-less `LineList`, depth-tested but never written.
pub fn build_grid_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    aspect_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    // Compile the WGSL program into a shader module on the GPU.
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("grid.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/grid.wgsl").into()), //  change name
    });

    // Layout - what external data the shader reads. Ours is empty.
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("grid.layout"),
        bind_group_layouts: &[Some(aspect_layout), Some(line_layout)],
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
                count: samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

/// Pipeline for mesh edges — `LineList` over the mesh vertices, depth-tested but not written.
pub fn build_edges_pipeline(
    device: &wgpu::Device,
    samples: u32,
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
                count: samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

/// Pipeline for linework tubes — one unit-cylinder template instanced per segment; solid, occluding.
pub fn build_cylinder_pipeline(
    device: &wgpu::Device,
    samples: u32,
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
        multisample: wgpu::MultisampleState { count: samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}

/// Pipeline for the full-screen background — buffer-less triangle at the far plane, always drawn.
pub fn build_background_pipeline(
    device: &wgpu::Device,
    samples: u32,
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
                count: samples,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        }
    )
}

/// Pipeline for vertex markers — a camera-facing quad template instanced per glyph, trimmed to
/// a circle by the fragment SDF. Blended (the AA rim needs it); always in front (see depth).
pub fn build_sphere_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    glyph_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline{

    let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor{
            label: Some("sphere.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/sphere.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("sphere.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout), Some(glyph_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: Some("sphere"),
        layout: Some(&layout),
        vertex: wgpu::VertexState{
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[cyl_template_layout()], // reused - position only, stride 12
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState{
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState{
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING), // smooth AA feather + hairline fade
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
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState{
            format: wgpu::TextureFormat::Depth32Float,
            // In front of the marker's OWN ink, occluded by everything genuinely nearer - and
            // NOT the other way round: a back-corner dot showing through the solid reads as a
            // live vertex where there is none. The win at the joint comes from the shader: the
            // hug puts the disc at the same face+eps the bands wrote, and SPHERE_TIE tips that
            // tie to the marker - so plain Greater (strict) is enough and stays honest.
            depth_write_enabled: Some(true),
            // GreaterEqual, not Greater. The marker draws AFTER the bands (see encode_frame), so a
            // tie has to fall to the marker for it to keep the rim a band cap overlaps; with a
            // strict compare the band, already written at the same depth, would hold the pixel.
            depth_compare: Some(if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual }),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState{ count: samples, mask: !0, alpha_to_coverage_enabled: false},
        multiview_mask: None,
        cache: None,
    })
}


/// Pipeline for point-cloud billboards — buffer-less triangles, alpha-blended, depth-tested not written.
pub fn build_point_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    glyph_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline{

    let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor{
            label: Some("point.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/point.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("point.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout), Some(glyph_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: Some("point"),
        layout: Some(&layout),
        vertex: wgpu::VertexState{
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState{
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState{
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState{
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::Greater), // reverse -Z¨
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState{ count: samples, mask: !0, alpha_to_coverage_enabled: false},
        multiview_mask: None,
        cache: None,
    })
}


/// Depth-only prepass for the FLAT lane (ribbons + dots), one builder for both since the two
/// differ only in shader and label. Runs `fs_depth` (binary at half coverage), writes no colour
/// and only depth, so the blended colour pass that follows can be occluded by ink drawn later
/// in the same frame. Without it, ink never writes depth and draw order alone decides who wins -
/// which is why dots always sat on top of polylines.
pub fn build_ink_depth_pipeline(
    device: &wgpu::Device,
    samples: u32,
    label: &str,
    color_format: wgpu::TextureFormat,
    source: wgpu::ShaderSource<'_>,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    table_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some(label), source });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some(label),
        bind_group_layouts: &[
            Some(mvp_layout),
            Some(line_layout),
            Some(instance_layout),
            Some(table_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&layout),
        vertex: wgpu::VertexState{
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_depth"),
            // The pass HAS a colour attachment, so the pipeline must declare one too - Dawn
            // rejects an empty target list against a colour pass. Mask every channel instead:
            // nothing is written, only depth.
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: wgpu::ColorWrites::empty(),
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Greater),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}

/// Pipeline for flat capsule ribbons - buffer-less, 6 vertices / segments, opaque, depth-writing.
/// The flat lane's shader, aimed at the SOLID lane (mesh/BRep edges) so those can be drawn as
/// camera-facing quads instead of tessellated tubes.
///
/// The ONLY thing that differs from `build_ribbon_pipeline` is that this one writes depth - an
/// edge in the solid lane should occlude what is behind it. Everything else, including
/// `GreaterEqual`, is deliberately identical, and `GreaterEqual` is the load-bearing part:
///
/// A mesh edge lies EXACTLY on the boundary of the two faces that meet there, so the quad and the
/// face are at the same depth. Strict `Greater` discards the line and float precision then decides
/// which pixels survive, which reads as an edge offset outward, ragged, and asymmetric along its
/// length - buried at one end and clean at the other. The flat lane always used `GreaterEqual`
/// for exactly this reason; copying `Greater` from the tube pipeline was the whole bug.
///
/// The quad geometry was never wrong: with the depth test disabled entirely, all twelve edges of
/// a box land precisely on their edges.
pub fn build_ribbon_solid_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    segment_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("ribbon.solid.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("ribbon.solid.layout"),
        bind_group_layouts: &[Some(mvp_layout), Some(line_layout), Some(instance_layout), Some(segment_layout)],
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("ribbon.solid"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true), // solid lane: an edge occludes what is behind it
            // GreaterEqual, NOT Greater. A mesh edge is EXACTLY on the boundary of the two faces
            // that meet there, so the ribbon and the face are at the same depth; strict Greater
            // discards the line and float precision then decides which pixels survive, which is
            // what made the edge look offset and ragged and asymmetric along its length. The flat
            // lane always used GreaterEqual; copying Greater from the tube pipeline was the bug.
            depth_compare: Some(if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual }),
            stencil: wgpu::StencilState::default(),
            // NO bias. An earlier attempt used a slope-scaled bias here, which only masked the
            // strict-Greater bug above: with GreaterEqual the quad needs no nudge at all, because
            // it is already exactly where it should be.
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}

pub fn build_ribbon_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    segment_layout: &wgpu::BindGroupLayout,
)-> wgpu::RenderPipeline {
    
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("ribbon.shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into())
    });
    
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
        label: Some("ribbon.layout"),
        bind_group_layouts: &[
            Some(mvp_layout), 
            Some(line_layout), 
            Some(instance_layout), 
            Some(segment_layout)],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("ribbon"),
        layout: Some(&layout),
        vertex: wgpu::VertexState{
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState{
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING), // smooth AA feather + hairline fade
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false), // blended ink must not block later ink at the same depth (line crossings)
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual), // must survive its OWN depth prepass
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview_mask: None,
        cache: None,
    })
}

/// Pipeline for flat SDF dots, the ribbon recipe with the glyph names, glyph layout at group 3.
pub fn build_glyph_pipeline(
    device: &wgpu::Device,
    samples: u32,
    color_format: wgpu::TextureFormat,
    mvp_layout: &wgpu::BindGroupLayout,
    line_layout: &wgpu::BindGroupLayout,
    instance_layout: &wgpu::BindGroupLayout,
    glyph_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("glyph.layout"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/glyph.wgsl").into()),
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("glyph.layout"),
        bind_group_layouts: &[
            Some(mvp_layout),
            Some(line_layout),
            Some(instance_layout),
            Some(glyph_layout)
        ],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("glyph"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING), // smooth AA feather + hairline fade
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false), // blended ink must not block later ink at the same depth (line crossings)
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual), // must survive its OWN depth prepass
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            mask: !0,
            alpha_to_coverage_enabled: false
        },
        multiview_mask: None,
        cache: None,
    })

}
