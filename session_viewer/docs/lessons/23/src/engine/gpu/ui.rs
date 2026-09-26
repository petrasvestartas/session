// --8<-- [start:painter]
// egui = a UI library that rebuilds every panel from code each frame and hands back triangles to draw.
use super::buffers::GpuCtx;

/// Draws the egui interface over the frame.
pub struct Ui {
    renderer: egui_wgpu::Renderer, // egui's own pipelines, buffers and textures
    jobs: Vec<egui::ClippedPrimitive>, // triangle batches, each cut to a rectangle of the screen
    screen: egui_wgpu::ScreenDescriptor,
    free: Vec<egui::TextureId>, // freed next frame: this frame's triangles may still sample them
    textures: std::collections::HashSet<egui::TextureId>,
}

impl Ui {
    /// The renderer must know the canvas format, e.g. Bgra8Unorm, to build pipelines that write to it.
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
    // --8<-- [end:painter]

    // --8<-- [start:painter-prepare]
    /// Upload this frame's egui output: textures, then triangles.
    pub fn prepare(
        &mut self,
        ctx: &GpuCtx,
        context: &egui::Context, // the egui frame
        output: egui::FullOutput,
        size: [u32; 2],
    ) {
        for id in self.free.drain(..) {
            self.renderer.free_texture(&id);
            self.textures.remove(&id);
        }

        // a texture delta is the part of a texture that changed, e.g. the glyphs of a newly typed letter
        for (id, delta) in &output.textures_delta.set {
            self.textures.insert(*id);
            self.renderer
                .update_texture(&ctx.device, &ctx.queue, *id, delta);
        }

        self.free = output.textures_delta.free;
        // tessellate = turn egui's shapes (rounded boxes, text) into triangles
        self.jobs = context.tessellate(output.shapes, output.pixels_per_point);

        // an empty frame costs no upload and no pass
        if self.jobs.is_empty() {
            return;
        }

        // a point is one CSS pixel; at pixels_per_point 2.0 a 10-point button is 20 device pixels
        self.screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: size,
            pixels_per_point: output.pixels_per_point,
        };
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("egui upload"),
            });
        // egui may return command buffers of its own; they go to the queue together with the upload
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
    // --8<-- [end:painter-prepare]

    // --8<-- [start:painter-draw]
    /// Draw the interface over the frame.
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
                    load: wgpu::LoadOp::Load, // keep the finished scene; the panels paint over it
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        // egui's render wants a pass that does not borrow the encoder; forget_lifetime drops that borrow
        self.renderer
            .render(&mut pass.forget_lifetime(), &self.jobs, &self.screen);
    }
}

// Drop = code Rust runs when the value goes away, here when the viewer closes.
impl Drop for Ui {
    /// Free every texture egui still holds.
    fn drop(&mut self) {
        for id in &self.textures {
            self.renderer.free_texture(id);
        }
    }
}
// --8<-- [end:painter-draw]
