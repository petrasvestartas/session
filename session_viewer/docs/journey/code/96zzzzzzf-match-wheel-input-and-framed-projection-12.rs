                    Action::Projection(mode) => {
                        if self.camera.projection!=mode {
                            let bounds=(mode==crate::camera::Projection::Perspective).then(||self.display_scene().bounds()).flatten();
                            self.camera.set_projection_framed(mode,bounds.as_ref());
                        }
                    },