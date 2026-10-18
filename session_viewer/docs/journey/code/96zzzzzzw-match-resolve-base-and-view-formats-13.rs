let mut renderer = Renderer::with_formats(
        device,
        queue,
        config.format,
        config.format.add_srgb_suffix(),