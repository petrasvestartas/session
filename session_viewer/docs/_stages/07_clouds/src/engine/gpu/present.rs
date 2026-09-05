//! How a frame leaves `Gpu`: presented to the swapchain (`present`), which writes the
//! uniforms, encodes through `encode_frame`, and submits.

use super::frame::{FrameCx, FrameInput};
use super::Gpu;

impl Gpu {
    /// Per-frame uniforms, then the inside-flag refresh, which reads the eye just solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let cx = FrameCx { view: &self.view, anchor: self.objects.anchor_f32(), size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
        self.objects.update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Draw one frame to the swapchain. Returns the encode time in ms, or `None` when the
    /// surface had no texture to give (it was reconfigured; the caller asks for another frame).
    pub fn present(&mut self, input: &FrameInput) -> Option<f64> {
        self.write_frame_uniforms(input);
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

        let t0 = crate::engine::performance::now_ms();
        self.encode_frame(&mut encoder, &view, input.clear);
        let encode_ms = crate::engine::performance::now_ms() - t0;
        self.ctx.queue.submit([encoder.finish()]);
        output.present();
        Some(encode_ms)
    }
}
