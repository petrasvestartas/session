    let pipeline = make(
        device,
        &layout,
        &shader,
        "vertex",
        "fragment",
        format,
        Some(wgpu::BlendState::ALPHA_BLENDING),
    );
