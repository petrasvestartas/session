                .and_then(|()| renderer.select_objects(editor.display_scene(), &editor.highlighted_objects())) { report(error); return; }
        }
        renderer.set_clipping(editor.clipping_enabled,editor.clipping_solid);
        if let Err(error) = renderer.set_grid_on(editor.camera.grid,editor.drawing_plane()) {
            report(error); return;
