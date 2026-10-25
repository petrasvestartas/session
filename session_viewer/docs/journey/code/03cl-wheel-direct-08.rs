        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point=Some(density);
        let previous = self.model.completion_rect.take();
        self.controls = Some(Vec::new());
