// --8<-- [start:004-encode]
use super::Gpu;

/// What every pass of one frame shares.
pub struct Frame<'a> {
    pub view: &'a wgpu::TextureView, // the canvas
    pub clear: wgpu::Color,          // background color
}

// `impl Gpu` blocks may sit in any file of the crate: this one adds the frame encoding.
impl Gpu {
    /// Encode one frame into `view`; returns the draw count.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear: wgpu::Color,
    ) -> u32 {

        let frame = Frame {
            view,
            clear,
        };
        // pass 1: background, section caps, faces and clouds write depth
        let mut draws = self.face_passes(encoder, &frame);
        draws
    }

    /// The first pass: each pass's own face passes, then the one the faces draw in.
    fn face_passes(&mut self, encoder: &mut wgpu::CommandEncoder, f: &Frame) -> u32 {
        let mut draws = 0;
        let mut drew = false;

        let mut pass = self
            .targets
            .begin_faces(encoder, f.view, (!drew).then_some(f.clear));

        draws
    }
}
// --8<-- [end:004-encode]
