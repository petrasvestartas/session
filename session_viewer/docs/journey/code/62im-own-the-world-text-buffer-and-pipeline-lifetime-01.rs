        })
    }
}

impl Planes {

    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        let layout = ctx
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("world text texture"),
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
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("world text coverage"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let id_pipeline = pipeline(ctx, Target::ID, &layout, true);
        let pipeline = pipeline(ctx, target, &layout, false);
        Self {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,
            target,
            rasterizations: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.pipeline = pipeline(ctx, target, &self.layout, false);
    }

    pub(super) fn reset(&mut self) {
        self.cached.clear();
        self.draws.clear();
        self.vertices.reset();
    }

    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.vertices.release(ctx);
    }

    pub(super) fn buffer_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }

    pub(super) fn texture_bytes(&self) -> u64 {
        let mut total = 0;

        for cached in &self.cached {
            total += u64::from(cached.size[0]) * u64::from(cached.size[1]);
        }

        total
    }
}
