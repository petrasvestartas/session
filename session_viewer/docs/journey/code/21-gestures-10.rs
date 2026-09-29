            Some(action)
        } else {
            pointer_action(&event, &pointer_canvas, &mut gesture)
        };
        if let Some(action) = action {
            match editor.apply(action) {
