            "undo" | "redo" => {
                if target.id() == "undo" {
                    history.undo(&mut scene);
                } else {
                    history.redo(&mut scene);
                }
                selected = selected.filter(|id| scene.contains(*id));
                renderer.set_scene(&scene, selected);
            }
            "background" => background.toggle(),
