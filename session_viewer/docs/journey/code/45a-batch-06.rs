                        }
                    }
                    Action::FitSelected => {
                        if let Some(bounds) = self.scene.bounds_of(&self.selected_objects()).or_else(|| self.selected_part.and(self.selected).and_then(|id| self.scene.selected_bounds(id)))
                        {
                            self.camera.fit(&bounds);
                        }
