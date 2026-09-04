//! How a frame leaves `Gpu`: presented to the swapchain (`present`), which clears and submits.

use super::Gpu;

impl Gpu {
    /// Draw one frame to the swapchain: the clear. Returns `None` when the surface had no
    /// texture to give (it was reconfigured; the caller asks for another frame).
    pub fn present(&mut self, clear: wgpu::Color) -> Option<()> {
        let surface = self.surface.as_ref()?;
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame") });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear), store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        Some(())
    }
}
