                .create_view(&Default::default())
        };
        let heads=crate::production_vector_gpu::Heads::new(device,&layouts,format);
        let planes=crate::production_vector_gpu::Heads::planes(device,&layouts,format);
        Self {
            heads,planes,plane_size:crate::production_plane::DEFAULT_SIZE as f32,edge_highlight:None,picks:sb(8),
            layouts,
            shader: RefCell::new(None),
            color: RefCell::new(None),
