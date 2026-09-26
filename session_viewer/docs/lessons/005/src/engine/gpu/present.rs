// --8<-- [start:004-present]
use super::Gpu;
#[cfg(not(target_arch = "wasm32"))]
use super::targets::{TextureSpec, texture};

/// Everything that changes from one frame to the next; the renderer keeps no camera or clock of its own.
pub struct FrameInput {
    pub clear: wgpu::Color, // the colour the frame starts from
    pub now_ms: f64, // browser clock, used to time each frame
}
impl Gpu {

    /// Draw one frame to the canvas; returns encode time in ms.
    pub fn present(&mut self, input: &FrameInput) -> Option<f64> {
        let surface = self.surface.as_ref()?;
        // the canvas lends one texture per frame; `present()` below hands it back to be shown
        let output = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t)
            | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => {
                surface.configure(&self.ctx.device, &self.config);
                return None;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });

        let t0 = crate::engine::performance::now_ms();
        let draws = self.encode_frame(&mut encoder, &view, input.clear);
        let encode_ms = crate::engine::performance::now_ms() - t0;
        // submit: the GPU starts on the recorded commands while the CPU moves on
        self.ctx.queue.submit([encoder.finish()]);
        output.present();

        Some(encode_ms)
    }

}
// --8<-- [end:004-present]
