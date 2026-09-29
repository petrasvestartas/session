            "select" => {
                selected = scene.next(selected);
                renderer.set_scene(&scene, selected);
            }
            "delete" => {
                if let Some(id) = selected.take() {
                    scene.remove(id);
                    renderer.set_scene(&scene, selected);
                }
            }
            "background" => background.toggle(),
