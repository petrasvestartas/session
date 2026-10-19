                report(&format!("Cannot lay out commands: {error:?}"));
                return;
            }
            renderer.set_gizmo(&editor.camera,editor.gizmo_origin(),editor.gizmo_active());
            if let Err(error) = present(
                &surface,
                &renderer,
