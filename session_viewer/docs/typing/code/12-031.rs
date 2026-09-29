
impl Gpu {
    /// Run only the id pass for a pick; nothing is shown.
    pub fn pick_frame(&mut self, input: &FrameInput, at: (u32, u32)) {
        self.write_frame_uniforms(input);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("pick"),
            });
        self.point_pass(&mut encoder);
        self.id_pass(&mut encoder, Some(at));
        self.ctx.queue.submit([encoder.finish()]);
        self.pick.map();
    }

    /// Draw one frame and return (object, sub) per pixel; native only.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_ids_offscreen(&mut self, input: &FrameInput) -> Vec<[u32; 2]> {
        // a pending pick would fight over the id textures
        self.pick.cancel();
        let size = (self.config.width, self.config.height);
        let texture = texture(
            &self.ctx,
            "headless.ids.color",
            &TextureSpec {
                size,
                format: self.config.format,
                samples: 1,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.write_frame_uniforms(input);
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("headless.ids"),
            });
        self.encode_frame(&mut encoder, &view, input.clear);
        self.id_pass(&mut encoder, None);
        let readback = self.pick.copy_frame(&self.ctx, &mut encoder);
        self.ctx.queue.submit([encoder.finish()]);
        readback.read(&self.ctx)
    }
}
