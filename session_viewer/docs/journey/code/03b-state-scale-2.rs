        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self {
            context,
            painter,
            model: CommandLine {
                status: "The field now belongs to CommandLine.".into(),
                ..Default::default()
            },
        }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let size = [target.texture().width(), target.texture().height()];
        #[cfg(target_arch="wasm32")]
        let density=web_sys::window().map_or(1.0,|window|window.device_pixel_ratio() as f32);
        #[cfg(not(target_arch="wasm32"))]
        let density=1.0;
        let screen = egui_wgpu::ScreenDescriptor { size_in_pixels: size, pixels_per_point: density };
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(size[0] as f32, size[1] as f32) / density,
            )),
            ..Default::default()
        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point=Some(density);
