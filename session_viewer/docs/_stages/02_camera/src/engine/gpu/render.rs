//! The frame list. `encode_frame` runs ONE scene pass whose `scene_list`
//! is the ordered lane draws. The order is the contract: everything that writes depth first,
//! the blended ink after.

use super::frame::Binds;
use super::Gpu;

impl Gpu {
    /// Encode the whole frame into `view`. Returns the draw count.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, clear: wgpu::Color) -> u32 {
        let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group };
        let mut pass = self.targets.begin_pass(encoder, view, clear);
        self.scene_list(&mut pass, &b)
    }

    /// The scene list, in order:
    /// 1 background · 2 grid.
    fn scene_list(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        let mut draws = 0u32;

        draws += self.backdrop.draw_background(pass);
        draws += self.backdrop.draw_grid(pass, b);
        draws
    }
}
