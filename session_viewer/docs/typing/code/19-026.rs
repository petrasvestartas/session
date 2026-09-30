
use crate::app::scene::SheetInit;
use crate::app::walk::sheet::SheetRows;

impl State {
    /// Start a streamed sheet; returns its slot.
    pub fn add_sheet(&mut self, init: SheetInit) -> usize {
        let idx = self.scene.add_sheet(init, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        self.touch();
        idx
    }

    /// Add more segments to sheet `idx`.
    pub fn extend_sheet(&mut self, idx: usize, rows: SheetRows, to: u32) {
        self.scene.extend_sheet(idx, rows, to, &mut self.gpu);
        self.camera.grow_extent(&self.gpu.bounds);
        log::info!(
            "sheet slice: {to} segments resident | heap {:.0} MB",
            heap_mb()
        );
        self.touch();
    }
}
