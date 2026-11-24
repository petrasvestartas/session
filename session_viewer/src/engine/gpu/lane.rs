use super::buffers::GpuCtx;
use super::frame::Binds;
use super::upload::Upload;
use super::view::View;
use crate::engine::pipelines::{Layouts, Target};
#[allow(unused_imports)]
pub use crate::rows::{LaneRows, PickMode};

// --8<-- [start:render-lane]
/// The lifecycle every drawing lane shares; a new lane impls this and is named once in `lane_list!` or `REGISTRY`.
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

    /// Take this lane's rows out of one upload.
    fn on_append(&mut self, _ctx: &GpuCtx, _layouts: &Layouts, _up: &Upload) {}

    /// Draw into the ink pass, over the selected curves; returns the draw count.
    fn draw_ink(&self, _pass: &mut wgpu::RenderPass<'_>, _b: &Binds, _view: &View) -> u32 {
        0
    }

    /// True when this frame's ink tests visibility against the triangle tile lists.
    fn reads_tiles(&self, _view: &View) -> bool {
        false
    }

    /// Draw pick ids for `mode`; returns the draw count.
    fn draw_ids(
        &self,
        _pass: &mut wgpu::RenderPass<'_>,
        _b: &Binds,
        _view: &View,
        _mode: PickMode,
    ) -> u32 {
        0
    }
}

// --8<-- [end:render-lane]
/// A registered lane's rows: an edited object overwrites or kills them in place.
pub trait RowLane: Lane {
    /// Overwrite this lane's rows of `up`, starting at row `first`.
    fn write_at(&mut self, ctx: &GpuCtx, l: &Layouts, first: u32, up: &Upload);

    /// Hand `count` rows from `first` to the hidden object row `sink`.
    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, sink: u32);
}

/// One registered lane: how it is made, counted and merged.
#[allow(dead_code)]
pub struct Registered {
    pub make: fn(&GpuCtx, &Layouts, Target) -> Box<dyn RowLane>, // the lane
    pub rows_in: fn(&Upload) -> u32,                             // its rows in one upload
    pub merge: fn(&mut Upload, &mut Upload), // move the second upload's rows onto the first
    pub stride: u64,                         // bytes per row
}

/// Lanes that live only behind the `Lane` hooks. Adding one means one file and one line here.
pub const REGISTRY: &[Registered] = &[
    super::vectors::REGISTERED, // register:vectors
    super::planes::REGISTERED,  // register:planes
];

/// How many lanes are registered.
pub const REGISTERED: usize = REGISTRY.len();
const _: () = assert!(REGISTERED == crate::rows::REGISTERED);
