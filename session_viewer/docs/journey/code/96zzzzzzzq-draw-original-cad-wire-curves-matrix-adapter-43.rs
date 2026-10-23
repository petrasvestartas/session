        Self {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, "world text vertices", 68, VERTS),
            layout,
            sampler,
            pipeline:crate::sample_pipelines::Variants::new(pipeline,target.samples),
