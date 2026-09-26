use super::buffers::GpuCtx;
use super::frame::Binds;
use super::upload::Upload;
use super::view::View;
use crate::engine::pipelines::{Layouts, Target};
use std::any::Any;

/// What a pick may answer with.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PickMode {
    #[default]
    Object, // whole objects
    Edge,      // edges
    Component, // edges first, else the visible face
    Controls {
        // control dots of one object
        parent: u32, // the object row
        cloud: bool, // true = pick cloud points instead
    },
}

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

/// A registered lane's rows: an edited object overwrites or kills them in place.
pub trait RowLane: Lane {
    /// Overwrite this lane's rows of `up`, starting at row `first`.
    fn write_at(&mut self, ctx: &GpuCtx, l: &Layouts, first: u32, up: &Upload);

    /// Hand `count` rows from `first` to the hidden object row `sink`.
    fn kill(&mut self, ctx: &GpuCtx, first: u32, count: u32, sink: u32);
}

/// One registered lane: how it is made, counted and merged.
pub struct Registered {
    pub make: fn(&GpuCtx, &Layouts, Target) -> Box<dyn RowLane>, // the lane
    pub rows_in: fn(&Upload) -> u32,                             // its rows in one upload
    pub merge: fn(&mut Upload, &mut Upload), // move the second upload's rows onto the first
    pub stride: u64,                         // bytes per row
}

/// Lanes that live only behind the `Lane` hooks. Adding one means one file and one line here.
pub const REGISTRY: &[Registered] = &[
];

/// How many lanes are registered.
pub const REGISTERED: usize = REGISTRY.len();

/// Rows for registered lanes, one table per row type, so `Upload` needs no field per lane.
#[derive(Default)]
pub struct LaneRows {
    tables: Vec<Box<dyn Any>>, // one table per type
}

impl LaneRows {
    /// The table of type `T`, created empty on first use.
    pub fn get_mut<T: Default + 'static>(&mut self) -> &mut T {
        let at = match self.tables.iter().position(|t| t.is::<T>()) {
            Some(at) => at,
            None => {
                self.tables.push(Box::new(T::default()));
                self.tables.len() - 1
            }
        };
        self.tables[at]
            .downcast_mut::<T>()
            .expect("table has type T")
    }

    /// The table of type `T`; None when nothing was written.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.tables.iter().find_map(|t| t.downcast_ref::<T>())
    }

    /// Take the table of type `T` out; None when nothing was written.
    pub fn take<T: 'static>(&mut self) -> Option<T> {
        let at = self.tables.iter().position(|t| t.is::<T>())?;
        self.tables
            .remove(at)
            .downcast::<T>()
            .ok()
            .map(|table| *table)
    }

    /// Drop every table and free its memory.
    pub fn clear(&mut self) {
        self.tables = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One table per type, created once, gone after clear.
    #[test]
    fn lane_rows_by_type() {
        let mut rows = LaneRows::default();
        assert!(rows.get::<Vec<u32>>().is_none());
        rows.get_mut::<Vec<u32>>().push(7);
        rows.get_mut::<Vec<u32>>().push(8);
        rows.get_mut::<Vec<f32>>().push(1.0);
        assert_eq!(rows.get::<Vec<u32>>(), Some(&vec![7, 8]));
        assert_eq!(rows.get::<Vec<f32>>().map(Vec::len), Some(1));
        rows.clear();
        assert!(rows.get::<Vec<u32>>().is_none());
    }
}
