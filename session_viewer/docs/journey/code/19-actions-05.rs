        let action = match target.id().as_str() {
            "canvas" => {
                let rect = canvas.get_bounding_client_rect();
                if rect.width() <= 0.0 || rect.height() <= 0.0 {
                    return;
                }
                Action::Pick([
                    (2.0 * (event.client_x() as f64 - rect.left()) / rect.width() - 1.0) as f32,
                    (1.0 - 2.0 * (event.client_y() as f64 - rect.top()) / rect.height()) as f32,
                ])
            }
            "box" => Action::AddBox,
            "scene" => Action::ToggleExtra,
            "select" => Action::SelectNext,
            "delete" => Action::Delete,
            "undo" => Action::Undo,
            "redo" => Action::Redo,
            "background" => Action::Background,
            "zoom-in" => Action::Zoom(2.0),
            "zoom-out" => Action::Zoom(0.5),
            "left" => Action::Pan(-0.25, 0.0),
            "right" => Action::Pan(0.25, 0.0),
            "turn" => Action::Orbit(std::f64::consts::FRAC_PI_4, 0.0),
            "tilt" => Action::Orbit(0.0, std::f64::consts::FRAC_PI_6),
            "iso" => Action::Isometric,
            "reset" => Action::ResetView,
            _ => return,
        };
        match editor.apply(action) {
            Ok(Change::Scene) => renderer.set_scene(&editor.scene, editor.selected),
            Ok(Change::View) => {}
            Err(error) => { report(error); return; }
        }
        if let Err(error) = present(&surface, &renderer, &editor.background, &editor.camera.uniform()) {
            report(&format!("Cannot redraw: {error:?}"));
        }
