use super::buffers::GpuCtx;
use crate::engine::pipelines::{Layouts, Target};

/// The lifecycle every drawing lane shares; a new lane impls this and is named once in `lane_list!`.
pub trait Lane {
    /// Rebuild pipelines for a new color format or sample count.
    fn on_retarget(&mut self, _ctx: &GpuCtx, _layouts: &Layouts, _target: Target) {}

    /// Forget every row, keep the buffers.
    fn on_reset(&mut self, _ctx: &GpuCtx) {}

    /// Forget every row and free the buffers; by default the same as a reset.
    fn on_release(&mut self, ctx: &GpuCtx, _layouts: &Layouts) {
        self.on_reset(ctx);
    }

    /// Bytes it holds on the GPU: (buffers, textures).
    fn bytes(&self) -> (u64, u64) {
        (0, 0)
    }
}
