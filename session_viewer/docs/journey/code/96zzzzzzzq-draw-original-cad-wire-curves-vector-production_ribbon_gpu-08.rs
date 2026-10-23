                })
                .create_view(&Default::default())
        };
        let heads=crate::production_vector_gpu::Heads::new(device,&layouts,format);
        Self {
            heads,edge_highlight:None,picks:sb(8),
            layouts,
            shader: RefCell::new(None),
            color: RefCell::new(None),
