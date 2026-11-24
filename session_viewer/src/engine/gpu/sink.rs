use super::Gpu;
use crate::rows::{Counts, Definition, GlyphPoint, Instance, LaneId, ObjectRow, SegRows, Upload};
use session_rust::{AABB, RenderVertex, Xform};
use viewer_core::GpuSink;

impl GpuSink for Gpu {
    fn canvas_size(&self) -> [u32; 2] {
        [self.config.width, self.config.height]
    }

    fn geometry_revision(&self) -> u64 {
        self.objects.geometry_revision()
    }

    fn object_row(&self, row: u32) -> Option<&Instance> {
        self.objects.row(row)
    }

    fn object_row_bounds(&self, row: u32) -> Option<AABB> {
        self.objects.row_bounds(row)
    }

    fn object_local_bounds(&self, row: u32) -> Option<AABB> {
        self.objects.local_bounds(row)
    }

    fn placement_matches(&self, row: u32, place: &Xform) -> bool {
        self.objects.placement_matches(row, place)
    }

    fn cloud_point_count(&self) -> u32 {
        self.cloud.point_count
    }

    fn cloud_fits(&self, incoming: u32) -> bool {
        self.cloud.fits(&self.ctx, incoming)
    }

    fn cloud_row_of(&self, sub: u32) -> Option<(u32, u32)> {
        self.cloud.row_of(sub)
    }

    fn segment_row_of(&self, ribbon: u32) -> Option<(u32, u32)> {
        self.segments.row_of(ribbon)
    }

    fn segment_source_id(&self, ribbon: u32) -> Option<u32> {
        self.segments.source_id(ribbon)
    }

    fn release(&mut self) {
        Gpu::release(self);
    }

    fn release_editable(&mut self) {
        Gpu::release_editable(self);
    }

    fn set_scene(&mut self, tables: &Upload) {
        Gpu::set_scene(self, tables);
    }

    fn forget_anchor(&mut self) {
        self.objects.forget_anchor();
    }

    fn set_selected(&mut self, row: u32, selected: bool) {
        Gpu::set_selected(self, row, selected);
    }

    fn set_hidden(&mut self, row: u32, hidden: bool) {
        Gpu::set_hidden(self, row, hidden);
    }

    fn retire_many(&mut self, rows: &[u32]) {
        self.objects.retire_many(&self.ctx, rows);
    }

    fn bury_many(&mut self, rows: &[u32]) {
        self.objects.bury_many(&self.ctx, rows);
    }

    fn set_object_row(&mut self, row: u32, object: &ObjectRow) {
        self.objects.set_row(&self.ctx, row, object);
    }

    fn unbury_object_row(&mut self, row: u32, object: &ObjectRow) {
        self.objects.unbury(&self.ctx, row, object);
    }

    fn update_object_geometry(&mut self, row: u32, object: &ObjectRow) {
        self.objects.update_geometry(&self.ctx, row, object);
    }

    fn set_object_placement(&mut self, row: u32, place: &Xform) {
        self.objects.set_placement(&self.ctx, row, place);
    }

    fn set_geometry_bounds(&mut self, row: u32, bounds: AABB, spacing: f32, place: &Xform) {
        self.objects
            .set_geometry_bounds(&self.ctx, row, bounds, spacing, place);
    }

    fn grow_local_bounds(&mut self, row: u32, bounds: &AABB, place: &Xform) {
        self.objects
            .grow_local_bounds(&self.ctx, row, bounds, place);
    }

    fn grew_bounds(&mut self, row: u32) {
        Gpu::grew_bounds(self, row);
    }

    fn kill_rows(&mut self, lane: LaneId, first: u32, count: u32, sink: u32) {
        Gpu::kill_rows(self, lane, first, count, sink);
    }

    fn degenerate_rows(&mut self, lane: LaneId, first: u32, count: u32, vertex: u32) {
        Gpu::degenerate_rows(self, lane, first, count, vertex);
    }

    fn write_rows(&mut self, at: Counts, up: &Upload) {
        Gpu::write_rows(self, at, up);
    }

    fn set_instanced(&mut self, definitions: &[Definition], touched: &[(u32, u32)], full: bool) {
        Gpu::set_instanced(self, definitions, touched, full);
    }

    fn set_dead(&mut self, dead: Counts, dead_points: u32) {
        Gpu::set_dead(self, dead, dead_points);
    }

    fn refresh_samples(&mut self) {
        Gpu::refresh_samples(self);
    }

    fn bury_cloud_instance(&mut self, row: u32) -> bool {
        self.cloud.bury_instance(row)
    }

    fn unbury_cloud_instance(&mut self, row: u32) -> bool {
        self.cloud.unbury_instance(row)
    }

    fn kill_cloud_instance(&mut self, row: u32) -> u32 {
        self.cloud.kill_instance(row)
    }

    fn invalidate_splat(&mut self) {
        self.splat.invalidate();
    }

    fn compact_cloud_buffers(&mut self) {
        self.cloud.compact(&self.ctx);
        self.splat
            .rebind(&self.ctx, &self.layouts, self.cloud.buffers());
        self.splat.invalidate();
        self.splat.set_point(None);
    }

    fn patch_vertices(&mut self, first: u32, vertices: &[RenderVertex]) {
        self.arena.patch_vertices(&self.ctx, first, vertices);
    }

    fn patch_pipes(&mut self, first: u32, pipes: &SegRows) {
        self.segments.patch_pipes(&self.ctx, first, pipes);
    }

    fn patch_marker(&mut self, index: u32, sphere: bool, marker: GlyphPoint) {
        self.glyphs.patch_marker(&self.ctx, index, sphere, marker);
    }

    fn geometry_changed(&mut self) {
        self.objects.geometry_changed();
    }
}
