        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point=Some(density);
        self.model.completion_rect = None;
        let output = self.context.run_ui(input, |root| {
