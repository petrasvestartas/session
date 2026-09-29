            "box" => {
                if let Err(error) = history.try_edit(&mut scene, |scene| scene.add_box().map(|_| ())) {
                    report(error);
                    return;
                }
                renderer.set_scene(&scene, selected);
            }
            "scene" => {
