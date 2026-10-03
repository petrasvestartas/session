        let mut renderer = Self { device, queue, pipeline, meshes: Vec::new(),
            geometry: Default::default(), settings_allocations: 0, uniform, view_group, depth };
        renderer.set_scene(scene, None);
        renderer
