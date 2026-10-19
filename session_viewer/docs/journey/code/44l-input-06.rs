        };
        let action = if event.type_() == "viewer-pick" {
            match renderer.take_pick(editor.camera.uniform()) {
                Some(Ok(hit)) => Some(Action::PickedTarget(hit.and_then(|hit| renderer.resolve_pick(&editor.scene, hit)))),
                Some(Err(error)) => { report(error); None },
                None => None,
            }
