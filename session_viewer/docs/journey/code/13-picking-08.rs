        match target.id().as_str() {
            "canvas" => {
                let rect = canvas.get_bounding_client_rect();
                if rect.width() <= 0.0 || rect.height() <= 0.0 {
                    return;
                }
                let screen = [
                    (2.0 * (event.client_x() as f64 - rect.left()) / rect.width() - 1.0) as f32,
                    (1.0 - 2.0 * (event.client_y() as f64 - rect.top()) / rect.height()) as f32,
                ];
                selected = crate::picking::pick(&scene, camera.world_from_screen(screen));
                renderer.set_scene(&scene, selected);
            }
