    if resize(&window, &canvas, &surface, &mut config, &mut renderer, &mut editor) {
        present(&surface, &renderer, &editor.background, &editor.camera.uniform())?;
    }
