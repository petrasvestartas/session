// --8<-- [start:frame]
use super::Gpu;
use super::buffers::GpuCtx;
use super::frame::{Binds, FrameInput, MAX_PLANES};
use super::lane::Lane;
pub use super::render::Frame;
use crate::engine::pipelines::Target;
use std::any::Any;

/// One optional pass of the frame owning its GPU state; each hook runs on every pass in `PASSES` order.
pub trait Pass: Lane + Any {
    /// Before anything of the frame is encoded.
    fn prepare(&mut self, _g: &mut Gpu, _encoder: &mut wgpu::CommandEncoder) {}

    /// Write this frame's uniforms.
    fn write_frame(&mut self, _g: &mut Gpu, _input: &FrameInput) {}

    /// Face passes of its own before the main one; the first opened clears, draws the backdrop, sets `drew`.
    fn before_faces(
        &mut self,
        _g: &mut Gpu,
        _encoder: &mut wgpu::CommandEncoder,
        _f: &Frame,
        _drew: &mut bool,
    ) -> u32 {
        0
    }

    /// Draws into the face pass, after the backdrop and before the faces.
    fn in_faces(&self, _g: &Gpu, _pass: &mut wgpu::RenderPass<'_>, _b: &Binds) -> u32 {
        0
    }

    /// True while this pass cuts the faces, which then keep their clip tests.
    fn clips(&self) -> bool {
        false
    }

    /// After the faces and before the ink; returns the draw count.
    fn after_faces(
        &mut self,
        _g: &mut Gpu,
        _encoder: &mut wgpu::CommandEncoder,
        _f: &Frame,
    ) -> u32 {
        0
    }

    /// Bind what its outline mask draws read.
    fn bind_masks(&mut self, _g: &Gpu) {}

    /// Draws into the open outline mask pass, `both` masks at once or the selection's alone.
    fn in_masks(&self, _g: &Gpu, _pass: &mut wgpu::RenderPass<'_>, _b: &Binds, _both: bool) -> u32 {
        0
    }

    /// Draws into the ink pass, over the selected mesh edges and under the selected curves.
    fn over_ink(&self, _g: &Gpu, _pass: &mut wgpu::RenderPass<'_>, _b: &Binds) -> u32 {
        0
    }

    /// Before the id pass draws over a `size` window.
    fn before_ids(
        &mut self,
        _g: &Gpu,
        _encoder: &mut wgpu::CommandEncoder,
        _b: &Binds,
        _size: (u32, u32),
    ) {
    }

    /// Draws into the id pass, before the faces.
    fn in_ids(&self, _g: &Gpu, _pass: &mut wgpu::RenderPass<'_>, _b: &Binds) -> u32 {
        0
    }

    /// Object `row` was selected or deselected.
    fn on_select(&mut self, _row: u32, _on: bool) {}

    /// True while the view asks for this pass and its pipelines are still compiling.
    fn pending(&self, _g: &Gpu) -> bool {
        false
    }

    /// After the frame was presented: compile ahead, outside the frame.
    fn after_present(&mut self, _g: &mut Gpu) {}

    /// The world planes (normal, offset) this pass cuts with.
    fn clip_world(&self) -> Option<[[f64; 4]; MAX_PLANES]> {
        None
    }
}
// --8<-- [end:pass-trait]

// --8<-- [start:passes]
// Empty in the first lessons: each pass a later lesson writes adds one line here.
/// The passes in frame order. Adding one means its `Pass` impl in one file and one line here.
pub const PASSES: &[fn(&GpuCtx, Target) -> Box<dyn Pass>] = &[
];

/// Every pass, made for `target`, in frame order.
pub fn make_all(ctx: &GpuCtx, target: Target) -> Vec<Box<dyn Pass>> {
    PASSES.iter().map(|make| make(ctx, target)).collect()
}

/// Stands in the list for a pass while its own hook runs.
pub(super) struct Nothing;

impl Lane for Nothing {}

impl Pass for Nothing {}
// --8<-- [end:passes]

// --8<-- [start:each-pass]
impl Gpu {
    /// Run `f` on every pass in order with the rest of the GPU; the pass is out of the list meanwhile.
    pub(super) fn each_pass(&mut self, mut f: impl FnMut(&mut dyn Pass, &mut Gpu)) {
        for i in 0..self.passes.len() {
            // `mem::replace` takes the pass out and leaves `Nothing` in its slot, so the pass and the rest of `self` can both be borrowed mutably
            let mut pass = std::mem::replace(&mut self.passes[i], Box::new(Nothing));
            f(pass.as_mut(), self);
            self.passes[i] = pass;
        }
    }

    /// The clipping planes in world space; zero planes cut nothing.
    pub(crate) fn clip_planes(&self) -> [[f64; 4]; MAX_PLANES] {
        self.passes
            .iter()
            .find_map(|pass| pass.clip_world())
            .unwrap_or([[0.0; 4]; MAX_PLANES])
    }

    /// The registered pass of type `T`.
    pub fn pass<T: Pass>(&self) -> &T {
        self.passes
            .iter()
            .find_map(|pass| (pass.as_ref() as &dyn Any).downcast_ref::<T>())
            .expect("a registered pass")
    }

    /// The registered pass of type `T`, mutable.
    pub fn pass_mut<T: Pass>(&mut self) -> &mut T {
        find_mut(&mut self.passes)
    }
}

/// The pass of type `T` in `passes`, for a borrow beside other `Gpu` fields.
pub(super) fn find_mut<T: Pass>(passes: &mut [Box<dyn Pass>]) -> &mut T {
    passes
        .iter_mut()
        .find_map(|pass| (pass.as_mut() as &mut dyn Any).downcast_mut::<T>())
        .expect("a registered pass")
}
// --8<-- [end:each-pass]
