use super::*;

pub struct SsaoPipelines {
    pub(super) target: Target,
    pub(super) layout: wgpu::BindGroupLayout,
    pub(super) depth_layout: wgpu::BindGroupLayout,
    pub(super) sample_layout: wgpu::BindGroupLayout,
    pub(super) history_layout: wgpu::BindGroupLayout,
    pub(super) prepare: wgpu::RenderPipeline, // full-resolution depth to the AO-size level 0, plus normals
    pub(super) reduce: wgpu::RenderPipeline,  // each pyramid level from the one below
    pub(super) occupancy: Pipeline, // a coarse mask of tiles near geometry, so empty floor is skipped
    pub(super) raw: Pipeline,       // the horizon search
    pub(super) ground: Pipeline,    // shadows on the virtual floor
    pub(super) filter: [Pipeline; 2], // blur along x, then along y blended with last frame
    pub(super) history: Pipeline,   // this frame's depth, for next frame's reuse check
    pub(super) upsample: Pipeline,  // AO size back to full size, per MSAA sample
    pub(super) composite: Pipeline, // darken the frame by the cache
    pub(super) edges: Pipeline,     // 4x: correct the samples that differ from their pixel
    pub(super) write_layout: wgpu::BindGroupLayout,
    pub(super) composite_layout: wgpu::BindGroupLayout,
}

/// A texture the fragment stage reads; `multisampled` when it is the 4x depth or id target.

/// Compile the whole effect for one target; compiling is slow in a browser, so the result is kept.
pub fn pipelines(ctx: &GpuCtx, target: Target) -> SsaoPipelines {
    // group 0: the frame's depth and ids, the camera uniform, and the mesh buffers that give exact normals
    let layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient scene"),
            entries: &[
                texture_entry(
                    0,
                    ShaderStages::FRAGMENT,
                    TextureSampleType::Depth,
                    target.samples > 1,
                ),
                buffer_entry(1, ShaderStages::FRAGMENT, BufferBindingType::Uniform),
                texture_entry(
                    2,
                    ShaderStages::FRAGMENT,
                    TextureSampleType::Uint,
                    target.samples > 1,
                ),
                buffer_entry(
                    3,
                    ShaderStages::FRAGMENT,
                    BufferBindingType::Storage { read_only: true },
                ),
                buffer_entry(
                    4,
                    ShaderStages::FRAGMENT,
                    BufferBindingType::Storage { read_only: true },
                ),
                buffer_entry(
                    5,
                    ShaderStages::FRAGMENT,
                    BufferBindingType::Storage { read_only: true },
                ),
                buffer_entry(
                    6,
                    ShaderStages::FRAGMENT,
                    BufferBindingType::Storage { read_only: true },
                ),
                texture_entry(7, ShaderStages::FRAGMENT, TextureSampleType::Uint, false),
            ],
        });
    // group 1: one pyramid level of depth, radius and normal
    let depth_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient depth pyramid"),
            entries: &[0, 1, 2].map(|binding| {
                texture_entry(
                    binding,
                    ShaderStages::FRAGMENT,
                    if binding == 1 {
                        TextureSampleType::Uint
                    } else {
                        TextureSampleType::Float { filterable: false }
                    },
                    false,
                )
            }),
        });
    // group 2: one AO image, read with bilinear filtering
    let sample_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient samples"),
            entries: &[
                texture_entry(
                    0,
                    ShaderStages::FRAGMENT,
                    TextureSampleType::Float { filterable: true },
                    false,
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    let shader = module(ctx, "ambient", &shader_source(target.samples));
    let depth_shader = module(
        ctx,
        "ambient depth reduction",
        shader!("ambient_depth.wgsl"),
    );
    // `build` makes pipelines with one colour target; these write two or three images at once, so they are built by hand
    let depth_pass = |shader: &wgpu::ShaderModule,
                      entry,
                      groups: &[&wgpu::BindGroupLayout],
                      targets: &[Option<wgpu::ColorTargetState>]| {
        crate::engine::pipelines::count_pipeline();
        ctx.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(entry),
                layout: Some(&pipeline_layout(&ctx.device, entry, groups)),
                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Some(entry),
                    targets,
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
    };
    // three images at once: linear depth, packed radius, octahedral normal
    let pyramid = [
        Some(wgpu::TextureFormat::R32Float.into()),
        Some(wgpu::TextureFormat::R16Uint.into()),
        Some(wgpu::TextureFormat::Rg8Unorm.into()),
    ];
    let prepare = depth_pass(&shader, "fs_prepare", &[&layout], &pyramid);
    let reduce = depth_pass(&depth_shader, "fs_reduce", &[&depth_layout], &pyramid[..2]);
    // every working image holds one byte per pixel
    let single = Target {
        format: wgpu::TextureFormat::R8Unorm,
        samples: 1,
    };
    let occupancy_groups = [&depth_layout];
    let occupancy = build(
        ctx,
        single,
        &PipelineDesc::new(
            &depth_shader,
            &occupancy_groups,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
        )
        .depth(DepthMode::Detached)
        .with("ambient ground occupancy", "fs_occupancy"),
    );
    let groups = [&layout, &depth_layout, &sample_layout];
    let desc = PipelineDesc::new(&shader, &groups, &[], wgpu::PrimitiveTopology::TriangleList)
        .depth(DepthMode::Detached);
    let raw = build(ctx, single, &desc.with("ambient horizons", "fs_main"));
    let ground = build(
        ctx,
        single,
        &desc.with("ambient ground contacts", "fs_ground"),
    );
    let history_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient history"),
            entries: &[
                texture_entry(
                    4,
                    ShaderStages::FRAGMENT,
                    TextureSampleType::Float { filterable: true },
                    false,
                ),
                texture_entry(
                    5,
                    ShaderStages::FRAGMENT,
                    TextureSampleType::Float { filterable: false },
                    false,
                ),
            ],
        });
    // the y filter also reads last frame's AO and depth, to reuse it
    let history_groups = [&layout, &depth_layout, &sample_layout, &history_layout];
    let filter = [
        build(
            ctx,
            single,
            &desc.with("ambient horizontal filter", "fs_filter_x"),
        ),
        build(
            ctx,
            single,
            &PipelineDesc {
                groups: &history_groups,
                ..desc.with("ambient temporal filter", "fs_filter_y")
            },
        ),
    ];
    let history = build(
        ctx,
        Target {
            format: wgpu::TextureFormat::R32Float,
            samples: 1,
        },
        &PipelineDesc::new(
            &shader,
            &[&layout, &depth_layout],
            &[],
            wgpu::PrimitiveTopology::TriangleList,
        )
        .depth(DepthMode::Detached)
        .with("ambient history depth", "fs_history"),
    );
    // 4x only: per-sample corrections, flagged tiles, the tile list and the indirect draw arguments
    let write_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient edge output"),
            entries: &[0, 1, 2, 3].map(|binding| {
                buffer_entry(
                    binding,
                    ShaderStages::FRAGMENT,
                    BufferBindingType::Storage { read_only: false },
                )
            }),
        });
    let write_groups = [&layout, &depth_layout, &sample_layout, &write_layout];
    let upsample = build(
        ctx,
        single,
        &PipelineDesc {
            groups: &write_groups,
            ..desc.with("ambient upsample", "fs_upsample")
        },
    );
    // read in the vertex stage too, where each flagged tile becomes one square
    let composite_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient edge input"),
            entries: &[0, 1, 2].map(|binding| {
                let mut entry = buffer_entry(
                    binding,
                    ShaderStages::FRAGMENT,
                    if binding == 0 {
                        BufferBindingType::Uniform
                    } else {
                        BufferBindingType::Storage { read_only: true }
                    },
                );
                entry.visibility = ShaderStages::VERTEX_FRAGMENT;
                entry
            }),
        });
    let composite_shader = module(ctx, "ambient composite", shader!("ambient_composite.wgsl"));
    let composite_groups = [&composite_layout, &sample_layout];
    let composite_desc = PipelineDesc::new(
        &composite_shader,
        &composite_groups,
        &[],
        wgpu::PrimitiveTopology::TriangleList,
    )
    .depth(DepthMode::Detached)
    .color(ColorWrite::Blended); // alpha blending: black at alpha = occlusion darkens what is drawn
    let composite = build(
        ctx,
        target,
        &composite_desc.with("ambient composite", "fs_main"),
    );
    let edges = build(
        ctx,
        target,
        &composite_desc
            .with("ambient edge correction", "fs_edges")
            .vertex("vs_edges"),
    );
    SsaoPipelines {
        target,
        layout,
        depth_layout,
        sample_layout,
        history_layout,
        prepare,
        reduce,
        occupancy,
        raw,
        ground,
        filter,
        history,
        upsample,
        composite,
        edges,
        write_layout,
        composite_layout,
    }
}

