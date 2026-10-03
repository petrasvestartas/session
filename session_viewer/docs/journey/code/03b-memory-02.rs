        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { context, painter, model: CommandLine {
            status: "The field now belongs to CommandLine.".into(),
            ..Default::default()
        } }
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
