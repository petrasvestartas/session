    let mut editor = Editor::default();
    let mut renderer = Renderer::new(device, queue, config.format.add_srgb_suffix(), &editor.scene);
    present(&surface, &renderer, &editor.background, &editor.camera.uniform())?;
