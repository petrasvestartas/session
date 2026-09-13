use super::buffers::GpuCtx;

pub struct Ui {
    renderer: egui_wgpu::Renderer,
    jobs: Vec<egui::ClippedPrimitive>,
    screen: egui_wgpu::ScreenDescriptor,
    free: Vec<egui::TextureId>,
    textures: std::collections::HashSet<egui::TextureId>,
}

impl Ui {
    pub fn new(ctx: &GpuCtx, format: wgpu::TextureFormat) -> Self {
        Self {
            renderer: egui_wgpu::Renderer::new(
                &ctx.device,
                format,
                egui_wgpu::RendererOptions::default(),
            ),
            jobs: Vec::new(),
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [1, 1],
                pixels_per_point: 1.0,
            },
            free: Vec::new(),
            textures: Default::default(),
        }
    }

    pub fn prepare(
        &mut self,
        ctx: &GpuCtx,
        context: &egui::Context,
        output: egui::FullOutput,
        size: [u32; 2],
    ) {
        for id in self.free.drain(..) {
            self.renderer.free_texture(&id);
            self.textures.remove(&id);
        }
        for (id, delta) in &output.textures_delta.set {
            self.textures.insert(*id);
            self.renderer
                .update_texture(&ctx.device, &ctx.queue, *id, delta);
        }
        self.free = output.textures_delta.free;
        self.jobs = context.tessellate(output.shapes, output.pixels_per_point);
        if self.jobs.is_empty() {
            return;
        }
        self.screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: output.pixels_per_point,
        };
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("egui upload"),
            });
        let mut buffers = self.renderer.update_buffers(
            &ctx.device,
            &ctx.queue,
            &mut encoder,
            &self.jobs,
            &self.screen,
        );
        buffers.push(encoder.finish());
        ctx.queue.submit(buffers);
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        if self.jobs.is_empty() {
            return;
        }
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        self.renderer
            .render(&mut pass.forget_lifetime(), &self.jobs, &self.screen);
    }
}

impl Drop for Ui {
    fn drop(&mut self) {
        for id in &self.textures {
            self.renderer.free_texture(id);
        }
    }
}
