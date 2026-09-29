                    // panels lay out, then the scene draws; register:egui
                    let repaint = self.ui.as_mut().is_some_and(|ui| ui.frame(state)); // register:egui
