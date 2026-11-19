use super::InstanceTable;
use crate::engine::gpu::buffers::GpuCtx;
use crate::engine::pipelines::Layouts;

impl crate::engine::gpu::lane::Lane for InstanceTable {
    fn on_reset(&mut self, _ctx: &GpuCtx) {
        self.reset();
    }

    fn on_release(&mut self, ctx: &GpuCtx, layouts: &Layouts) {
        self.release(ctx, layouts);
    }

    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), 0)
    }
}
