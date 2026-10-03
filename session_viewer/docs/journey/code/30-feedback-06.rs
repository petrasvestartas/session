                Ok(Change::Scene) => {
                    renderer.set_scene(&editor.scene, editor.selected);
                    if event.type_() == "viewer-file" {
                        report("File imported. Undo removes the entire import.");
                    }
                }
