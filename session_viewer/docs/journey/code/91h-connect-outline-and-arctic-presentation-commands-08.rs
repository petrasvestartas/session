        }
        renderer.set_clipping(editor.clipping_enabled,editor.clipping_solid);
        renderer.set_ambient(editor.arctic);renderer.set_outlines(editor.outlines);
        if let Err(error) = renderer.set_grid_on(editor.camera.grid,editor.drawing_plane()) {
            report(error); return;
