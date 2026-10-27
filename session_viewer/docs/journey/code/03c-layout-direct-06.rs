        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point=Some(density);
        self.controls = Some(Vec::new());
