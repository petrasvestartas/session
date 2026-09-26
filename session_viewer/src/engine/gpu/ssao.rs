use super::{buffers::GpuCtx, targets::Targets};
use crate::engine::pipelines::{
    ColorWrite, DepthMode, Pipeline, PipelineDesc, Target, build, module, pipeline_layout,
};

pub struct SsaoPipelines {
    target: Target,
    layout: wgpu::BindGroupLayout,
    depth_layout: wgpu::BindGroupLayout,
    sample_layout: wgpu::BindGroupLayout,
    history_layout: wgpu::BindGroupLayout,
    prepare: wgpu::RenderPipeline,
    reduce: wgpu::RenderPipeline,
    occupancy: Pipeline,
    raw: Pipeline,
    ground: Pipeline,
    filter: [Pipeline; 2],
    history: Pipeline,
    upsample: Pipeline,
    composite: Pipeline,
    edges: Pipeline,
    write_layout: wgpu::BindGroupLayout,
    composite_layout: wgpu::BindGroupLayout,
}

fn texture_entry(
    binding: u32,
    sample_type: wgpu::TextureSampleType,
    multisampled: bool,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}

fn buffer_entry(binding: u32, ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn pipelines(ctx: &GpuCtx, target: Target) -> SsaoPipelines {
    let layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient scene"),
            entries: &[
                texture_entry(0, wgpu::TextureSampleType::Depth, target.samples > 1),
                buffer_entry(1, wgpu::BufferBindingType::Uniform),
                texture_entry(2, wgpu::TextureSampleType::Uint, target.samples > 1),
                buffer_entry(3, wgpu::BufferBindingType::Storage { read_only: true }),
                buffer_entry(4, wgpu::BufferBindingType::Storage { read_only: true }),
                buffer_entry(5, wgpu::BufferBindingType::Storage { read_only: true }),
                buffer_entry(6, wgpu::BufferBindingType::Storage { read_only: true }),
            ],
        });
    let depth_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient depth pyramid"),
            entries: &[0, 1].map(|binding| {
                texture_entry(
                    binding,
                    if binding == 0 {
                        wgpu::TextureSampleType::Float { filterable: false }
                    } else {
                        wgpu::TextureSampleType::Uint
                    },
                    false,
                )
            }),
        });
    let sample_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient samples"),
            entries: &[
                texture_entry(
                    0,
                    wgpu::TextureSampleType::Float { filterable: true },
                    false,
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
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
    let depth_pass = |shader: &wgpu::ShaderModule, entry, groups: &[&wgpu::BindGroupLayout]| {
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
                    targets: &[
                        Some(wgpu::TextureFormat::R32Float.into()),
                        Some(wgpu::TextureFormat::R16Uint.into()),
                    ],
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            })
    };
    let prepare = depth_pass(&shader, "fs_prepare", &[&layout]);
    let reduce = depth_pass(&depth_shader, "fs_reduce", &[&depth_layout]);
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
                    wgpu::TextureSampleType::Float { filterable: true },
                    false,
                ),
                texture_entry(
                    5,
                    wgpu::TextureSampleType::Float { filterable: false },
                    false,
                ),
            ],
        });
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
            &[&layout],
            &[],
            wgpu::PrimitiveTopology::TriangleList,
        )
        .depth(DepthMode::Detached)
        .with("ambient history depth", "fs_history"),
    );
    let write_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient edge output"),
            entries: &[0, 1, 2, 3].map(|binding| {
                buffer_entry(
                    binding,
                    wgpu::BufferBindingType::Storage { read_only: false },
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
    let composite_layout = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("ambient edge input"),
            entries: &[0, 1, 2].map(|binding| {
                let mut entry = buffer_entry(
                    binding,
                    if binding == 0 {
                        wgpu::BufferBindingType::Uniform
                    } else {
                        wgpu::BufferBindingType::Storage { read_only: true }
                    },
                );
                entry.visibility = wgpu::ShaderStages::VERTEX_FRAGMENT;
                entry
            }),
        });
    let composite_shader = module(
        ctx,
        "ambient composite",
        shader!("ambient_composite.wgsl"),
    );
    let composite_groups = [&composite_layout, &sample_layout];
    let composite_desc = PipelineDesc::new(
        &composite_shader,
        &composite_groups,
        &[],
        wgpu::PrimitiveTopology::TriangleList,
    )
    .depth(DepthMode::Detached)
    .color(ColorWrite::Blended);
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

#[cfg(target_arch = "wasm32")]
#[path = "ambient_warm.rs"]
mod warm;

pub fn cached<'a>(
    slots: &'a mut [Option<SsaoPipelines>; 2],
    ctx: &GpuCtx,
    target: Target,
) -> Option<&'a SsaoPipelines> {
    let slot = &mut slots[usize::from(target.samples > 1)];
    if slot.as_ref().is_none_or(|pipes| pipes.target != target) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            *slot = Some(pipelines(ctx, target));
        }
        #[cfg(target_arch = "wasm32")]
        {
            *slot = warm::take(ctx, target);
        }
    }
    slot.as_ref()
}

/// Schedule browser compilation outside the frame after geometry has been presented.
pub fn prewarm(slots: &mut [Option<SsaoPipelines>; 2], ctx: &GpuCtx, target: Target, idle: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        warm::schedule(ctx, target, idle);
        for samples in [1, 4] {
            cached(slots, ctx, Target { samples, ..target });
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if idle {
        for samples in [1, 4] {
            cached(slots, ctx, Target { samples, ..target });
        }
    }
}

struct Image {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}

impl Image {
    fn new(
        ctx: &GpuCtx,
        label: &str,
        size: (u32, u32),
        format: wgpu::TextureFormat,
        mips: u32,
    ) -> Self {
        let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: mips,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        Self { texture, view }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}

fn depth_group(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    depth: &wgpu::TextureView,
    radius: &wgpu::TextureView,
) -> wgpu::BindGroup {
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("ambient depth"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(depth),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(radius),
            },
        ],
    })
}

fn attachment(
    view: &wgpu::TextureView,
    clear: bool,
) -> Option<wgpu::RenderPassColorAttachment<'_>> {
    Some(wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        depth_slice: None,
        ops: wgpu::Operations {
            load: if clear {
                wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
            } else {
                wgpu::LoadOp::Load
            },
            store: wgpu::StoreOp::Store,
        },
    })
}

fn resolution(full: (u32, u32), dpr: f64) -> (u32, u32) {
    let scale = (1.0 / dpr.max(1.0)).clamp(0.25, 0.5);
    (
        (f64::from(full.0) * scale).ceil().max(1.0) as u32,
        (f64::from(full.1) * scale).ceil().max(1.0) as u32,
    )
}

pub struct Ssao {
    full: (u32, u32),
    samples: u32,
    size: (u32, u32),
    linear: Image,
    radius: Image,
    occupancy: Image,
    occupancy_group: wgpu::BindGroup,
    ground: Image,
    ground_group: wgpu::BindGroup,
    levels: [[wgpu::TextureView; 2]; 6],
    depth_group: wgpu::BindGroup,
    reduce_groups: [wgpu::BindGroup; 5],
    ao: [Image; 3],
    sampled: [wgpu::BindGroup; 3],
    history: Image,
    history_group: wgpu::BindGroup,
    #[cfg(test)]
    history_enabled: bool,
    inverse: wgpu::Buffer,
    edge_buffers: [wgpu::Buffer; 4],
    edge_write: wgpu::BindGroup,
    edge_read: wgpu::BindGroup,
    group: Option<(
        wgpu::TextureView,
        wgpu::TextureView,
        [wgpu::Buffer; 4],
        wgpu::BindGroup,
    )>,
    cached: Option<([f32; 64], u64)>,
    receiver_bounds: Option<(u64, session_rust::AABB, f32)>,
    receiver_box: [f32; 6],
}

impl Ssao {
    pub fn receiver(&mut self, objects: &super::objects::InstanceTable) -> [f32; 2] {
        let revision = objects.geometry_revision();
        if self
            .receiver_bounds
            .as_ref()
            .is_none_or(|(r, _, _)| *r != revision)
        {
            let mut bounds = session_rust::AABB::empty();
            let mut radius = 0.01_f32;
            for i in 0..objects.len() {
                let flags = objects.row(i).unwrap().flags;
                if flags & super::Instance::FLAG_HAS_FACES != 0
                    && flags & (super::Instance::FLAG_HIDDEN | super::Instance::FLAG_SHEET) == 0
                    && let Some(b) = objects.row_bounds(i)
                {
                    bounds.union_with(&b);
                    radius = radius.max(objects.row(i).unwrap().ao_radius);
                }
            }
            self.receiver_bounds = Some((revision, bounds, radius));
        }
        let (_, b, radius) = self.receiver_bounds.as_ref().unwrap();
        self.receiver_box = [
            (b.cx - b.hx - objects.anchor()[0]) as f32,
            (b.cy - b.hy - objects.anchor()[1]) as f32,
            (b.cz - b.hz - objects.anchor()[2]) as f32,
            (b.cx + b.hx - objects.anchor()[0]) as f32,
            (b.cy + b.hy - objects.anchor()[1]) as f32,
            (b.cz + b.hz - objects.anchor()[2]) as f32,
        ];
        [self.receiver_box[2], *radius]
    }

    pub fn new(ctx: &GpuCtx, pipes: &SsaoPipelines, full: (u32, u32), dpr: f64) -> Self {
        let size = resolution(full, dpr);
        // Small windows still have six valid mip levels.
        let size = (size.0.max(32), size.1.max(32));
        let inverse = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("ambient camera"),
            size: 320,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let linear = Image::new(
            ctx,
            "ambient linear depth",
            size,
            wgpu::TextureFormat::R32Float,
            6,
        );
        let radius = Image::new(ctx, "ambient radius", size, wgpu::TextureFormat::R16Uint, 6);
        let levels = std::array::from_fn(|level| {
            [&linear, &radius].map(|image| {
                image.texture.create_view(&wgpu::TextureViewDescriptor {
                    base_mip_level: level as u32,
                    mip_level_count: Some(1),
                    ..Default::default()
                })
            })
        });
        let depth_group = depth_group(ctx, &pipes.depth_layout, &linear.view, &radius.view);
        let reduce_groups = std::array::from_fn(|i| {
            self::depth_group(ctx, &pipes.depth_layout, &levels[i][0], &levels[i][1])
        });
        let ao = std::array::from_fn(|i| {
            Image::new(
                ctx,
                "ambient occlusion",
                if i == 2 { full } else { size },
                wgpu::TextureFormat::R8Unorm,
                1,
            )
        });
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("ambient interpolation"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let sample = |view: &wgpu::TextureView| {
            ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient samples"),
                layout: &pipes.sample_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            })
        };
        let sampled = std::array::from_fn(|i| sample(&ao[i].view));
        let occupancy = Image::new(
            ctx,
            "ambient ground occupancy",
            (size.0 >> 4, size.1 >> 4),
            wgpu::TextureFormat::R8Unorm,
            1,
        );
        let occupancy_group = sample(&occupancy.view);
        let ground = Image::new(
            ctx,
            "ambient ground contacts",
            (size.0.div_ceil(2), size.1.div_ceil(2)),
            wgpu::TextureFormat::R8Unorm,
            1,
        );
        let ground_group = sample(&ground.view);
        let history = Image::new(
            ctx,
            "ambient history depth",
            (size.0.div_ceil(2), size.1.div_ceil(2)),
            wgpu::TextureFormat::R32Float,
            1,
        );
        let history_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient history"),
            layout: &pipes.history_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&ao[2].view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(&history.view),
                },
            ],
        });
        let pixels = u64::from(full.0) * u64::from(full.1);
        let tiles = u64::from(full.0.div_ceil(16)) * u64::from(full.1.div_ceil(16));
        let edge_buffers = std::array::from_fn(|i| {
            ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient edge corrections"),
                size: if i == 3 {
                    16
                } else if pipes.target.samples == 1 {
                    4
                } else if i == 0 {
                    pixels * 4
                } else {
                    tiles * 4
                },
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | if i == 3 {
                        wgpu::BufferUsages::INDIRECT
                    } else {
                        wgpu::BufferUsages::empty()
                    },
                mapped_at_creation: false,
            })
        });
        ctx.queue
            .write_buffer(&edge_buffers[3], 0, bytemuck::cast_slice(&[6_u32, 0, 0, 0]));
        let edge_write = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient edge output"),
            layout: &pipes.write_layout,
            entries: &std::array::from_fn::<_, 4, _>(|i| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: edge_buffers[i].as_entire_binding(),
            }),
        });
        let edge_read = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("ambient edge input"),
            layout: &pipes.composite_layout,
            entries: &std::array::from_fn::<_, 3, _>(|i| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: [&inverse, &edge_buffers[0], &edge_buffers[2]][i].as_entire_binding(),
            }),
        });
        Self {
            full,
            samples: pipes.target.samples,
            size,
            linear,
            radius,
            occupancy,
            occupancy_group,
            ground,
            ground_group,
            levels,
            depth_group,
            reduce_groups,
            ao,
            sampled,
            history,
            history_group,
            #[cfg(test)]
            history_enabled: true,
            inverse,
            edge_buffers,
            edge_write,
            edge_read,
            group: None,
            cached: None,
            receiver_bounds: None,
            receiver_box: [0.0; 6],
        }
    }

    pub fn texture_bytes(&self) -> u64 {
        let pixels = u64::from(self.size.0) * u64::from(self.size.1);
        let pyramid: u64 = (0..self.linear.texture.mip_level_count())
            .map(|i| u64::from(self.size.0 >> i) * u64::from(self.size.1 >> i))
            .sum();
        (4 + u64::from(self.radius.texture.format().block_copy_size(None).unwrap())) * pyramid
            + 2 * pixels
            + u64::from(self.full.0) * u64::from(self.full.1)
            + 5 * u64::from(self.ground.texture.width()) * u64::from(self.ground.texture.height())
            + u64::from(self.occupancy.texture.width()) * u64::from(self.occupancy.texture.height())
    }

    pub fn buffer_bytes(&self) -> u64 {
        320 + self
            .edge_buffers
            .iter()
            .map(wgpu::Buffer::size)
            .sum::<u64>()
    }

    pub fn fits(&self, pipes: &SsaoPipelines, full: (u32, u32), dpr: f64) -> bool {
        let size = resolution(full, dpr);
        self.full == full
            && self.samples == pipes.target.samples
            && self.size == (size.0.max(32), size.1.max(32))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &mut self,
        ctx: &GpuCtx,
        pipes: &SsaoPipelines,
        targets: &Targets,
        geometry: [&wgpu::Buffer; 4],
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        mvp: [f32; 16],
        ground: [f32; 2],
        revision: u64,
        mut timer: Option<&mut super::timing::PassTimer>,
    ) -> u32 {
        let Some(inverse) = inverse_projection(mvp) else {
            return 0;
        };
        let mut uniform = [0.0; 64];
        uniform[..16].copy_from_slice(&inverse);
        uniform[16..32].copy_from_slice(&mvp);
        uniform[32..40].copy_from_slice(&[
            ground[0],
            ground[1],
            self.full.0 as f32,
            self.full.1 as f32,
            self.ground.texture.width() as f32,
            self.size.0 as f32,
            self.size.1 as f32,
            self.ground.texture.height() as f32,
        ]);
        uniform[40..].copy_from_slice(&pixel_rays(&inverse, self.full));
        let changed = self.cached != Some((uniform, revision));
        if changed {
            let mut data = [0.0; 80];
            data[..64].copy_from_slice(&uniform);
            if let Some((previous, key)) = &self.cached {
                if *key == revision {
                    data[64..].copy_from_slice(&previous[16..32]);
                    data[43] = 1.0;
                }
            }
            #[cfg(test)]
            if !self.history_enabled {
                data[43] = 0.0;
            }
            ctx.queue
                .write_buffer(&self.inverse, 0, bytemuck::cast_slice(&data));
        }
        if self.group.as_ref().is_none_or(|(depth, ids, buffer, _)| {
            *depth != targets.depth.view
                || *ids != targets.gradient.view
                || buffer.iter().zip(geometry).any(|(a, b)| a != b)
        }) {
            let group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("ambient scene"),
                layout: &pipes.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&targets.depth),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.inverse.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&targets.gradient),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: geometry[0].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: geometry[1].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: geometry[2].as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: geometry[3].as_entire_binding(),
                    },
                ],
            });
            self.group = Some((
                targets.depth.view.clone(),
                targets.gradient.view.clone(),
                geometry.map(Clone::clone),
                group,
            ));
        }
        let group = &self.group.as_ref().unwrap().3;
        if changed {
            if self.samples > 1 {
                encoder.clear_buffer(&self.edge_buffers[1], 0, None);
                encoder.clear_buffer(&self.edge_buffers[3], 4, Some(4));
            }
            let rectangle = projected_bounds(mvp, self.receiver_box, self.full);
            for level in 0..6 {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient depth"),
                    color_attachments: &[
                        attachment(&self.levels[level][0], true),
                        attachment(&self.levels[level][1], true),
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                if level == 0 {
                    let lo = [
                        (rectangle[0] * self.size.0 as f32).floor() as u32,
                        (rectangle[1] * self.size.1 as f32).floor() as u32,
                    ];
                    let hi = [
                        (rectangle[2] * self.size.0 as f32).ceil() as u32,
                        (rectangle[3] * self.size.1 as f32).ceil() as u32,
                    ];
                    if hi[0] > lo[0] && hi[1] > lo[1] {
                        pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
                    }
                }
                pass.set_pipeline(if level == 0 {
                    &pipes.prepare
                } else {
                    &pipes.reduce
                });
                pass.set_bind_group(
                    0,
                    if level == 0 {
                        group
                    } else {
                        &self.reduce_groups[level - 1]
                    },
                    &[],
                );
                pass.draw(0..3, 0..1);
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient ground occupancy"),
                    color_attachments: &[attachment(&self.occupancy.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.occupancy);
                pass.set_bind_group(0, &self.reduce_groups[4], &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.depth");
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient ground contacts"),
                    color_attachments: &[attachment(&self.ground.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.ground);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, &self.depth_group, &[]);
                pass.set_bind_group(2, &self.occupancy_group, &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.ground");
            }
            for (index, pipe) in [
                &pipes.raw,
                &pipes.filter[0],
                &pipes.filter[1],
                &pipes.upsample,
            ]
            .into_iter()
            .enumerate()
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient shading"),
                    color_attachments: &[attachment(&self.ao[[0, 1, 0, 2][index]].view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                let size = if index == 3 { self.full } else { self.size };
                let lo: [u32; 2] = std::array::from_fn(|i| {
                    ((rectangle[i] * [size.0, size.1][i] as f32).floor() as u32) / 16 * 16
                });
                let hi: [u32; 2] = std::array::from_fn(|i| {
                    ((rectangle[i + 2] * [size.0, size.1][i] as f32).ceil() as u32)
                        .max(lo[i] + 1)
                        .div_ceil(16)
                        .saturating_mul(16)
                        .min([size.0, size.1][i])
                });
                pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
                pass.set_pipeline(pipe);
                pass.set_bind_group(0, group, &[]);
                pass.set_bind_group(1, &self.depth_group, &[]);
                pass.set_bind_group(
                    2,
                    if index == 0 {
                        &self.ground_group
                    } else {
                        &self.sampled[usize::from(index == 2)]
                    },
                    &[],
                );
                if index == 2 {
                    pass.set_bind_group(3, &self.history_group, &[]);
                } else if index == 3 {
                    pass.set_bind_group(3, &self.edge_write, &[]);
                }
                pass.draw(0..3, 0..1);
                drop(pass);
                if let Some(timer) = timer.as_deref_mut() {
                    timer.mark(
                        encoder,
                        ["ao.horizons", "ao.blur_x", "ao.blur_y", "ao.upsample"][index],
                    );
                }
            }
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ambient history depth"),
                    color_attachments: &[attachment(&self.history.view, true)],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipes.history);
                pass.set_bind_group(0, group, &[]);
                pass.draw(0..3, 0..1);
            }
            if let Some(timer) = timer.as_deref_mut() {
                timer.mark(encoder, "ao.history");
            }
            self.cached = Some((uniform, revision));
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ambient composite"),
            color_attachments: &[attachment(targets.msaa.as_deref().unwrap_or(view), false)],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&pipes.composite);
        pass.set_bind_group(0, &self.edge_read, &[]);
        pass.set_bind_group(1, &self.sampled[2], &[]);
        let rectangle = projected_bounds(mvp, self.receiver_box, self.full);
        let lo = [
            (rectangle[0] * self.full.0 as f32).floor() as u32,
            (rectangle[1] * self.full.1 as f32).floor() as u32,
        ];
        let hi = [
            (rectangle[2] * self.full.0 as f32).ceil() as u32,
            (rectangle[3] * self.full.1 as f32).ceil() as u32,
        ];
        let mut draws = if changed { 13 } else { 0 };
        if hi[0] > lo[0] && hi[1] > lo[1] {
            pass.set_scissor_rect(lo[0], lo[1], hi[0] - lo[0], hi[1] - lo[1]);
            pass.draw(0..3, 0..1);
            draws += 1;
            if self.samples > 1 {
                pass.set_pipeline(&pipes.edges);
                pass.draw_indirect(&self.edge_buffers[3], 0);
                draws += 1;
            }
        }
        draws
    }
}

fn projected_bounds(matrix: [f32; 16], bounds: [f32; 6], full: (u32, u32)) -> [f32; 4] {
    let mut rect = [1.0_f32, 1.0, 0.0, 0.0];
    for corner in 0..8 {
        let p = [
            bounds[if corner & 1 == 0 { 0 } else { 3 }],
            bounds[if corner & 2 == 0 { 1 } else { 4 }],
            bounds[if corner & 4 == 0 { 2 } else { 5 }],
            1.0,
        ];
        let clip: [f32; 4] =
            std::array::from_fn(|row| (0..4).map(|col| matrix[col * 4 + row] * p[col]).sum());
        if clip[3] <= 0.0 {
            return [0.0, 0.0, 1.0, 1.0];
        }
        let uv = [clip[0] / clip[3] * 0.5 + 0.5, 0.5 - clip[1] / clip[3] * 0.5];
        for i in 0..2 {
            rect[i] = rect[i].min(uv[i]);
            rect[i + 2] = rect[i + 2].max(uv[i]);
        }
    }
    // Ground probes reach 64 canvas pixels; leave room for filtering and reconstruction.
    for i in 0..2 {
        let margin = 84.0 / [full.0, full.1][i].max(1) as f32;
        rect[i] = (rect[i] - margin).clamp(0.0, 1.0 - 1.0 / [full.0, full.1][i].max(1) as f32);
        rect[i + 2] = (rect[i + 2] + margin).clamp(0.0, 1.0);
    }
    rect
}

fn shader_source(samples: u32) -> String {
    let source = concat!(shader!("ssao.wgsl"), "\n", shader!("ambient_geometry.wgsl"));
    if samples > 1 {
        source
            .replace("const MSAA: bool = false;", "const MSAA: bool = true;")
            .replace("texture_depth_2d", "texture_depth_multisampled_2d")
            .replace(
                "physical: texture_2d<u32>",
                "physical: texture_multisampled_2d<u32>",
            )
    } else {
        source.to_owned()
    }
}

/// Near plane point and ray to depth 0.5 at pixel (0,0), and their steps per pixel in x and y.
fn pixel_rays(inverse: &[f32; 16], size: (u32, u32)) -> [f32; 24] {
    // world point at a pixel and depth, as the shader's `world`
    let world = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let ndc = [
            x / f64::from(size.0) * 2.0 - 1.0,
            1.0 - y / f64::from(size.1) * 2.0,
            z,
            1.0,
        ];
        let p: [f64; 4] = std::array::from_fn(|row| {
            (0..4)
                .map(|col| f64::from(inverse[col * 4 + row]) * ndc[col])
                .sum()
        });
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    };
    let near = [
        world(0.0, 0.0, 1.0),
        world(1.0, 0.0, 1.0),
        world(0.0, 1.0, 1.0),
    ];
    let half = [
        world(0.0, 0.0, 0.5),
        world(1.0, 0.0, 0.5),
        world(0.0, 1.0, 0.5),
    ];
    let ray: [[f64; 3]; 3] =
        std::array::from_fn(|i| std::array::from_fn(|k| half[i][k] - near[i][k]));
    let mut out = [0.0; 24];
    for (i, base) in [near, ray].iter().enumerate() {
        for k in 0..3 {
            out[i * 12 + k] = base[0][k] as f32;
            out[i * 12 + 4 + k] = (base[1][k] - base[0][k]) as f32;
            out[i * 12 + 8 + k] = (base[2][k] - base[0][k]) as f32;
        }
    }
    out
}

/// Invert the camera matrix; rows are scaled first to keep precision.
fn inverse_projection(matrix: [f32; 16]) -> Option<[f32; 16]> {
    let mut normalized = matrix.map(f64::from);
    let scales: [f64; 4] = std::array::from_fn(|row| {
        (0..4)
            .map(|col| normalized[col * 4 + row].abs())
            .fold(0.0, f64::max)
    });
    if scales.iter().any(|s| !s.is_finite() || *s == 0.0) {
        return None;
    }
    for row in 0..4 {
        for col in 0..4 {
            normalized[col * 4 + row] /= scales[row];
        }
    }
    let mut inverse = session_rust::Xform::from_matrix(normalized).inverse()?.m;
    for col in 0..4 {
        for row in 0..4 {
            inverse[col * 4 + row] /= scales[col];
        }
    }
    Some(inverse.map(|v| v as f32))
}

#[cfg(test)]
mod tests {
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Contact darkens pixels, far ground stays bright, memory returns.
    fn occlusion_darkens_contact_and_releases_its_small_uniform() {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu};
        use session_rust::{BRep, Session, Xform};
        use std::rc::Rc;
        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        let mut source = Session::new("contact");
        source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
        let tower = source
            .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
            .unwrap();
        source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
        let raised = source
            .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
            .unwrap();
        source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
        if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
            std::fs::create_dir_all("target/review").unwrap();
            std::fs::write("target/review/ambient-contact.pb", source.pb_dumps()).unwrap();
        }
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "contact".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1.0);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        for (samples, perspective) in [(1, true), (4, true), (1, false), (4, false)] {
            camera.perspective = perspective;
            let input = FrameInput {
                view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            };
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            gpu.view.ssao = false;
            let plain = gpu.render_offscreen(&input);
            let memory = gpu.allocated_bytes();
            gpu.view.ssao = true;
            let partial = gpu.render_offscreen(&input);
            let shaded = gpu.render_offscreen(&input);
            assert_eq!(partial, shaded, "one frame computes the complete image");
            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::write(format!("target/review/ambient-{samples}x-off.rgba"), &plain)
                    .unwrap();
                std::fs::write(format!("target/review/ambient-{samples}x-on.rgba"), &shaded)
                    .unwrap();
            }
            for shaded in [&partial, &shaded] {
                let ground_shadow = plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .filter(|(a, b)| {
                        a[0] == 255
                            && a[1] == 255
                            && a[2] == 255
                            && b[0] < shaded[0].saturating_sub(3)
                    })
                    .count();
                assert!(
                    ground_shadow > 20,
                    "virtual ground receives soft shadows: {ground_shadow}"
                );
                let mut distant_ground = 0;
                for (i, (a, b)) in plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .enumerate()
                {
                    if a[..3] != [255, 255, 255] {
                        continue;
                    }
                    let (origin, direction) = camera
                        .ray(
                            ((i % 256) as f64 + 0.5, (i / 256) as f64 + 0.5),
                            (256.0, 256.0),
                        )
                        .unwrap();
                    if direction[2] >= 0.0 {
                        continue;
                    }
                    let t = (-5.0 - origin[2]) / direction[2];
                    let x = origin[0] + t * direction[0];
                    let y = origin[1] + t * direction[1];
                    let distance = (x.abs() - 50.0).max(0.0).hypot((y.abs() - 50.0).max(0.0));
                    if t > 0.0 && distance > 25.0 {
                        distant_ground += 1;
                        assert!(
                            b[0] >= shaded[0].saturating_sub(2),
                            "ground halo away from base at ({x}, {y}), {samples}x, perspective={perspective}: {}",
                            b[0]
                        );
                    }
                }
                assert!(
                    distant_ground > 1000,
                    "check exposed ground around raised geometry"
                );
                let darkened = plain
                    .chunks_exact(4)
                    .zip(shaded.chunks_exact(4))
                    .filter(|(a, b)| a[0] > b[0].saturating_add(2))
                    .count();
                assert!(
                    darkened > 20,
                    "contact occlusion changes pixels at {samples}x: {darkened}"
                );
            }
            assert_eq!(
                gpu.allocated_bytes(),
                (
                    memory.0 + gpu.ssao.as_ref().unwrap().buffer_bytes(),
                    memory.1 + gpu.ssao.as_ref().unwrap().texture_bytes()
                )
            );
            gpu.view.ssao = false;
            assert_eq!(gpu.render_offscreen(&input), plain);
            assert_eq!(gpu.allocated_bytes(), memory);
        }
    }

    /// A headless canvas showing the contact scene, and its camera.
    #[cfg(not(target_arch = "wasm32"))]
    fn contact_scene(
        width: u32,
        height: u32,
    ) -> (
        crate::engine::gpu::Gpu,
        crate::camera::Camera,
        session_rust::Point,
    ) {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use crate::engine::gpu::Gpu;
        use session_rust::{BRep, Session, Xform};
        use std::rc::Rc;
        let mut gpu = pollster::block_on(Gpu::new_headless(width, height)).unwrap();
        gpu.view.show_grid = false;
        let mut source = Session::new("contact");
        source.add_brep(BRep::create_box(100.0, 100.0, 10.0), None);
        let tower = source
            .add_brep(BRep::create_box(30.0, 30.0, 80.0), None)
            .unwrap();
        source.set_xform(&tower.borrow().name, Xform::translation(0.0, 0.0, 35.0));
        let raised = source
            .add_brep(BRep::create_box(40.0, 40.0, 10.0), None)
            .unwrap();
        source.set_xform(&raised.borrow().name, Xform::translation(110.0, 0.0, 12.0));
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "contact".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, width as f64 / height as f64);
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        (gpu, camera, rebase.anchor)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "native GPU benchmark; run with buildslot --exclusive"]
    fn benchmark_arctic() {
        use crate::app::scene::{FileDoc, Scene};
        use crate::camera::Camera;
        use crate::engine::gpu::{Gpu, timing::PassTimer};
        use session_rust::{Session, Xform};
        use std::rc::Rc;
        let path = std::env::var("AO_SCENE").expect("AO_SCENE protobuf path");
        let output = std::env::var("AO_OUTPUT").expect("AO_OUTPUT directory");
        let samples: u32 = std::env::var("AO_SAMPLES")
            .unwrap_or("1".into())
            .parse()
            .unwrap();
        let source = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
        let mut scene = Scene::new();
        let scale: f64 = std::env::var("AO_SCALE")
            .unwrap_or("1".into())
            .parse()
            .unwrap();
        scene.add_file(FileDoc {
            name: "arctic benchmark".into(),
            session: Rc::new(source),
            place: Xform::from_matrix([
                scale, 0.0, 0.0, 0.0, 0.0, 0.0, scale, 0.0, 0.0, -scale, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0,
            ]),
            point_px: 0.0,
            display_only: false,
        });
        let mut gpu = pollster::block_on(Gpu::new_headless(1920, 1080)).unwrap();
        gpu.view.msaa_forced = Some(samples);
        gpu.resize(1920, 1080);
        gpu.view.show_grid = false;
        scene.upload_to(&mut gpu);
        if std::env::var_os("AO_NO_EDGES").is_some() {
            gpu.view.show_mesh_edges = false;
            gpu.view.show_lines = false;
        }
        let mut camera = Camera::new();
        camera.fit(&gpu.bounds, 1920.0 / 1080.0);
        let anchor = gpu
            .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
            .anchor;
        std::fs::create_dir_all(&output).unwrap();
        let write = |name: &str, pixels: Vec<u8>| {
            std::fs::write(format!("{output}/{name}.rgba"), pixels).unwrap();
        };
        write("off", timed(&mut gpu, &camera, &anchor).1);
        gpu.view.ssao = true;
        for _ in 0..4 {
            timed(&mut gpu, &camera, &anchor);
        }
        write("still", timed(&mut gpu, &camera, &anchor).1);
        gpu.performance.interacting = true;
        write("drag", timed(&mut gpu, &camera, &anchor).1);
        gpu.performance.interacting = false;
        write("release", timed(&mut gpu, &camera, &anchor).1);
        let mut report = String::new();
        report += &format!(
            "samples={samples} buffers={} textures={} ao_textures={}\n",
            gpu.allocated_bytes().0,
            gpu.allocated_bytes().1,
            gpu.ssao.as_ref().unwrap().texture_bytes()
        );
        for mode in ["still", "moved", "drag"] {
            gpu.performance.interacting = mode == "drag";
            for _ in 0..4 {
                timed(&mut gpu, &camera, &anchor);
            }
            gpu.timer = Some(PassTimer::new(&gpu.ctx).expect("GPU timestamp support"));
            for _ in 0..24 {
                if mode != "still" {
                    camera.orbit(1.745, 0.0);
                }
                timed(&mut gpu, &camera, &anchor);
            }
            let timer = gpu.timer.as_ref().unwrap();
            report += &format!("{mode}: {:?}\n", timer.medians());
            let mut ao_total = [0.0_f64; 24];
            let mut frame_total = [0.0_f64; 24];
            for (label, series) in &timer.spans {
                for (i, value) in series.iter().take(24).enumerate() {
                    frame_total[i] += value;
                    if *label == "ssao" || label.starts_with("ao.") {
                        ao_total[i] += value;
                    }
                }
            }
            ao_total.sort_by(f64::total_cmp);
            frame_total.sort_by(f64::total_cmp);
            report += &format!(
                "{mode} AO total: {:.3} ms; frame GPU: {:.3} ms\n",
                ao_total[12], frame_total[12]
            );
            gpu.timer = None;
            if mode != "still" {
                for _ in 0..24 {
                    camera.orbit(-1.745, 0.0);
                }
            }
        }
        std::fs::write(format!("{output}/timings.txt"), &report).unwrap();
        println!("{report}");
    }

    /// Milliseconds of one offscreen frame, read back to the CPU.
    #[cfg(not(target_arch = "wasm32"))]
    fn timed(
        gpu: &mut crate::engine::gpu::Gpu,
        camera: &crate::camera::Camera,
        anchor: &session_rust::Point,
    ) -> (f64, Vec<u8>) {
        let aspect = gpu.config.width as f64 / gpu.config.height as f64;
        let input = crate::engine::gpu::FrameInput {
            view_proj: camera.view_proj_anchored(aspect, anchor),
            clear: wgpu::Color::WHITE,
            now_ms: 0.0,
        };
        let start = std::time::Instant::now();
        let pixels = gpu.render_offscreen(&input);
        (start.elapsed().as_secs_f64() * 1000.0, pixels)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "benchmark, requires a native GPU adapter"]
    /// Toggle, resize, still and orbit timings at 1920x1080; writes captures.
    fn bench_ambient() {
        let (mut gpu, mut camera, anchor) = contact_scene(1920, 1080);
        let mut report = String::new();
        gpu.view.msaa_forced = Some(4);
        gpu.resize(1920, 1080);
        timed(&mut gpu, &camera, &anchor);
        let mut toggles = Vec::new();
        for _ in 0..10 {
            gpu.view.ssao = true;
            toggles.push(timed(&mut gpu, &camera, &anchor).0);
            gpu.view.ssao = false;
            timed(&mut gpu, &camera, &anchor);
        }
        report += &format!("toggle on ms: {toggles:.1?}\n");
        gpu.view.ssao = true;
        let resizes: Vec<f64> = (1..=10)
            .map(|k| {
                gpu.resize(1920 + k, 1080);
                timed(&mut gpu, &camera, &anchor).0
            })
            .collect();
        report += &format!("resize ms: {resizes:.1?}\n");
        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(1920, 1080);
            for ssao in [false, true] {
                gpu.view.ssao = ssao;
                let pixels = (0..4).fold(Vec::new(), |_, _| timed(&mut gpu, &camera, &anchor).1);
                if ssao {
                    std::fs::create_dir_all("target/review").unwrap();
                    std::fs::write(format!("target/review/bench-{samples}x.rgba"), &pixels)
                        .unwrap();
                }
                let still: f64 = (0..120)
                    .map(|_| timed(&mut gpu, &camera, &anchor).0)
                    .sum::<f64>()
                    / 120.0;
                let mut orbit = 0.0;
                for _ in 0..120 {
                    camera.orbit(1.745, 0.0);
                    orbit += timed(&mut gpu, &camera, &anchor).0;
                }
                for _ in 0..120 {
                    camera.orbit(-1.745, 0.0);
                }
                // the same orbit as a drag
                gpu.performance.interacting = true;
                let mut drag = 0.0;
                for _ in 0..120 {
                    camera.orbit(1.745, 0.0);
                    drag += timed(&mut gpu, &camera, &anchor).0;
                }
                for _ in 0..120 {
                    camera.orbit(-1.745, 0.0);
                }
                gpu.performance.interacting = false;
                report += &format!(
                    "{samples}x ssao={ssao}: still {still:.2} ms, orbit {:.2} ms, drag {:.2} ms\n",
                    orbit / 120.0,
                    drag / 120.0
                );
            }
        }
        std::fs::write("target/review/bench-ambient.txt", &report).unwrap();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// After the first frame with occlusion, toggles and resizes compile no pipeline or shader.
    fn toggles_and_resizes_compile_nothing_after_the_first_frame() {
        let (mut gpu, camera, anchor) = contact_scene(256, 256);
        gpu.view.msaa_forced = Some(4);
        gpu.resize(256, 256);
        gpu.view.ssao = true;
        timed(&mut gpu, &camera, &anchor);
        let compiled = crate::engine::pipelines::created();

        for _ in 0..20 {
            gpu.view.ssao = false;
            timed(&mut gpu, &camera, &anchor);
            gpu.view.ssao = true;
            timed(&mut gpu, &camera, &anchor);
        }

        for step in 1..=20 {
            gpu.resize(256 + step, 256);
            timed(&mut gpu, &camera, &anchor);
        }

        assert_eq!(crate::engine::pipelines::created(), compiled);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Arctic preserves shading, lines and outlines through slow drags and release.
    fn navigation_preserves_the_same_image_and_needs_no_settling() {
        for samples in [1, 4] {
            let (mut gpu, mut camera, anchor) = contact_scene(256, 256);
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(256, 256);
            gpu.view.ssao = false;
            let plain = timed(&mut gpu, &camera, &anchor).1;
            gpu.view.set_arctic(true);
            let still = timed(&mut gpu, &camera, &anchor).1;
            let memory = gpu.allocated_bytes();

            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::create_dir_all("target/review").unwrap();
                std::fs::write(
                    format!("target/review/ambient-{samples}x-still.rgba"),
                    &still,
                )
                .unwrap();
            }

            gpu.performance.interacting = true;
            let dragged = timed(&mut gpu, &camera, &anchor).1;
            assert!(!gpu.ambient_pending());
            assert_eq!(dragged, still, "drag entry preserves the image");

            if std::env::var_os("VIEWER_AO_CAPTURE").is_some() {
                std::fs::create_dir_all("target/review").unwrap();
                std::fs::write(
                    format!("target/review/ambient-{samples}x-drag.rgba"),
                    &dragged,
                )
                .unwrap();
            }

            assert_eq!(gpu.allocated_bytes(), memory, "no texture for the drag");
            let darkened = plain
                .chunks_exact(4)
                .zip(dragged.chunks_exact(4))
                .filter(|(a, b)| a[0] > b[0].saturating_add(2))
                .count();
            assert!(
                darkened > 20,
                "a drag still shades contact at {samples}x: {darkened}"
            );
            gpu.performance.interacting = false;
            let restored = timed(&mut gpu, &camera, &anchor).1;
            assert!(!gpu.ambient_pending());
            assert!(restored == still, "the still image returns at {samples}x");
            for tier in 0..=2 {
                gpu.performance.interacting = true;
                if tier > 0 {
                    for frame in 0..8 {
                        gpu.performance
                            .frame(0, 0, 1000.0 * (frame + tier * 8) as f64, false);
                    }
                }
                if tier == 2 {
                    assert_eq!(gpu.performance.drag_tier(), 2);
                }
                camera.orbit(7.0, 3.0);
                let moving = timed(&mut gpu, &camera, &anchor).1;
                gpu.performance.interacting = false;
                let stopped = timed(&mut gpu, &camera, &anchor).1;
                assert!(
                    moving == stopped,
                    "Arctic lines, outlines and shadows stay unchanged after release at tier {tier}"
                );
                assert!(!gpu.ambient_pending());
                assert_eq!(gpu.allocated_bytes(), memory);
            }
            gpu.set_hidden(1, true);
            let hidden = timed(&mut gpu, &camera, &anchor).1;
            gpu.ssao = None;
            assert_eq!(
                hidden,
                timed(&mut gpu, &camera, &anchor).1,
                "geometry changes discard old shadows immediately"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn rotation_reprojects_ground_shadows_without_erasing_them() {
        let probes: Vec<_> = (-35_i32..=35)
            .flat_map(|x| {
                (-35_i32..=35).filter_map(move |y| {
                    (x.abs() > 25 || y.abs() > 25).then_some([
                        f64::from(x * 2),
                        f64::from(y * 2),
                        -5.0,
                    ])
                })
            })
            .collect();
        let measure = |history| {
            let (mut gpu, mut camera, anchor) = contact_scene(512, 512);
            gpu.view.msaa_forced = Some(4);
            gpu.resize(512, 512);
            gpu.view.ssao = true;
            timed(&mut gpu, &camera, &anchor);
            gpu.ssao.as_mut().unwrap().history_enabled = history;
            let mut frames = Vec::new();
            for _ in 0..32 {
                camera.orbit(1.0, 0.0);
                let matrix = camera.view_proj_anchored(1.0, &anchor).m;
                frames.push((matrix, timed(&mut gpu, &camera, &anchor).1));
            }
            gpu.view.ssao = false;
            let mut previous = vec![None; probes.len()];
            let (mut energy, mut changes, mut strength, mut visible) = (0.0_f64, 0, 0.0_f64, 0);
            for (matrix, shaded) in frames {
                let mut view_proj = session_rust::Xform::identity();
                view_proj.m = matrix;
                let plain = gpu.render_offscreen(&crate::engine::gpu::FrameInput {
                    view_proj,
                    clear: wgpu::Color::WHITE,
                    now_ms: 0.0,
                });
                for (index, point) in probes.iter().enumerate() {
                    let p: [f64; 3] = std::array::from_fn(|i| point[i] - anchor[i]);
                    let clip: [f64; 4] = std::array::from_fn(|r| {
                        matrix[r] * p[0]
                            + matrix[r + 4] * p[1]
                            + matrix[r + 8] * p[2]
                            + matrix[r + 12]
                    });
                    let x = (clip[0] / clip[3] * 0.5 + 0.5) * 512.0 - 0.5;
                    let y = (0.5 - clip[1] / clip[3] * 0.5) * 512.0 - 0.5;
                    let value = if clip[3] > 0.0 && x >= 0.0 && y >= 0.0 && x < 511.0 && y < 511.0 {
                        let at = (y.floor() as usize * 512 + x.floor() as usize) * 4;
                        let taps = [at, at + 4, at + 512 * 4, at + 513 * 4];
                        if taps.iter().all(|at| plain[*at..*at + 3] == [255, 255, 255]) {
                            let a = f64::from(shaded[taps[0]]) * (1.0 - x.fract())
                                + f64::from(shaded[taps[1]]) * x.fract();
                            let b = f64::from(shaded[taps[2]]) * (1.0 - x.fract())
                                + f64::from(shaded[taps[3]]) * x.fract();
                            Some(f64::from(shaded[0]) - a * (1.0 - y.fract()) - b * y.fract())
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some(v) = value {
                        strength += v;
                        visible += 1;
                        if let Some(old) = previous[index] {
                            let delta: f64 = v - old;
                            energy += delta * delta;
                            changes += 1;
                        }
                    }
                    previous[index] = value;
                }
            }
            assert!(changes > 10_000);
            (
                (energy / f64::from(changes)).sqrt(),
                strength / f64::from(visible),
            )
        };
        let spatial = measure(false);
        let temporal = measure(true);
        println!(
            "ground rotation: spatial {spatial:?}, reprojected {temporal:?} (RMS change, mean shadow)"
        );
        assert!(
            temporal.0 < spatial.0 * 0.9,
            "rotation should reduce shimmer"
        );
        assert!(
            temporal.1 > spatial.1 * 0.85,
            "stabilizing must retain contact shadows"
        );
    }

    #[test]
    fn resolution_follows_device_pixels_without_a_navigation_mode() {
        assert_eq!(super::resolution((1920, 1080), 1.0), (960, 540));
        assert_eq!(super::resolution((3840, 2160), 2.0), (1920, 1080));
        assert_eq!(super::resolution((1179, 2556), 3.0), (393, 852));
        assert_eq!(super::resolution((1920, 1080), 5.0), (480, 270));
    }

    #[test]
    fn shader_validates_for_both_depth_sample_counts() {
        let sources = [
            super::shader_source(1),
            super::shader_source(4),
            shader!("ambient_depth.wgsl").to_owned(),
            shader!("ambient_composite.wgsl").to_owned(),
        ];
        for source in sources {
            let module = naga::front::wgsl::parse_str(&source).unwrap();
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap();
            for (_, ty) in module.types.iter() {
                if ty.name.as_deref() == Some("Object") {
                    use super::super::instance::Instance;
                    use std::mem::{offset_of, size_of};
                    let naga::TypeInner::Struct { ref members, span } = ty.inner else {
                        panic!("Object is a struct");
                    };
                    assert_eq!(span as usize, size_of::<Instance>());
                    assert_eq!(
                        members
                            .iter()
                            .map(|member| member.offset as usize)
                            .collect::<Vec<_>>(),
                        [
                            offset_of!(Instance, model),
                            offset_of!(Instance, color),
                            offset_of!(Instance, flags),
                            offset_of!(Instance, ao_radius),
                            offset_of!(Instance, spacing),
                            offset_of!(Instance, _pad)
                        ]
                    );
                }
                if ty.name.as_deref() == Some("Ambient") {
                    let naga::TypeInner::Struct { ref members, span } = ty.inner else {
                        panic!("Ambient is a struct");
                    };
                    assert_eq!(span, 320);
                    assert_eq!(
                        members
                            .iter()
                            .map(|member| member.offset)
                            .collect::<Vec<_>>(),
                        [0, 64, 128, 144, 160, 176, 192, 208, 224, 240, 256]
                    );
                }
            }
        }
    }
}

impl super::lane::Lane for Option<Ssao> {
    fn bytes(&self) -> (u64, u64) {
        (
            self.as_ref().map_or(0, Ssao::buffer_bytes),
            self.as_ref().map_or(0, Ssao::texture_bytes),
        )
    }
}
