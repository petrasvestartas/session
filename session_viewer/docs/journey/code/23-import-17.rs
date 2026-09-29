        if event.type_() == "viewer-file" {
            report("File imported. Undo removes the entire import.");
        }
        if resize(&browser_window, &pointer_canvas, &surface, &mut config, &mut renderer, &mut editor) {
