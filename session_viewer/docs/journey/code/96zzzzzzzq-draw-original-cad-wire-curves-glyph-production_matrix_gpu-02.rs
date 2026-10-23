                .create_view(&Default::default())
        };
        Self {
            settings:Default::default(),            layouts,
            shader: RefCell::new(None),
            color: RefCell::new(None),
            pick: RefCell::new(None),
