use crate::rows::{Counts, Definition, GlyphPoint, Instance, LaneId, ObjectRow, SegRows, Upload};
use session_rust::{AABB, RenderVertex, Xform};

/// Pure data-sink interface between UI-independent `Scene` / `MeshPreview` and a GPU renderer.
pub trait GpuSink {
    fn canvas_size(&self) -> [u32; 2];
    fn geometry_revision(&self) -> u64;
    fn object_row(&self, row: u32) -> Option<&Instance>;
    fn object_row_bounds(&self, row: u32) -> Option<AABB>;
    fn object_local_bounds(&self, row: u32) -> Option<AABB>;
    fn placement_matches(&self, row: u32, place: &Xform) -> bool;
    fn cloud_point_count(&self) -> u32;
    fn cloud_fits(&self, incoming: u32) -> bool;
    fn cloud_row_of(&self, sub: u32) -> Option<(u32, u32)>;
    fn segment_row_of(&self, ribbon: u32) -> Option<(u32, u32)>;
    fn segment_source_id(&self, ribbon: u32) -> Option<u32>;

    fn release(&mut self);
    fn release_editable(&mut self);
    fn set_scene(&mut self, tables: &Upload);
    fn forget_anchor(&mut self);
    fn set_selected(&mut self, row: u32, selected: bool);
    fn set_hidden(&mut self, row: u32, hidden: bool);
    fn retire_many(&mut self, rows: &[u32]);
    fn bury_many(&mut self, rows: &[u32]);
    fn set_object_row(&mut self, row: u32, object: &ObjectRow);
    fn unbury_object_row(&mut self, row: u32, object: &ObjectRow);
    fn update_object_geometry(&mut self, row: u32, object: &ObjectRow);
    fn set_object_placement(&mut self, row: u32, place: &Xform);
    fn set_geometry_bounds(&mut self, row: u32, bounds: AABB, spacing: f32, place: &Xform);
    fn grow_local_bounds(&mut self, row: u32, bounds: &AABB, place: &Xform);
    fn grew_bounds(&mut self, row: u32);
    fn kill_rows(&mut self, lane: LaneId, first: u32, count: u32, sink: u32);
    fn degenerate_rows(&mut self, lane: LaneId, first: u32, count: u32, vertex: u32);
    fn write_rows(&mut self, at: Counts, up: &Upload);
    fn set_instanced(&mut self, definitions: &[Definition], touched: &[(u32, u32)], full: bool);
    fn set_dead(&mut self, dead: Counts, dead_points: u32);
    fn refresh_samples(&mut self);
    fn bury_cloud_instance(&mut self, row: u32) -> bool;
    fn unbury_cloud_instance(&mut self, row: u32) -> bool;
    fn kill_cloud_instance(&mut self, row: u32) -> u32;
    fn invalidate_splat(&mut self);
    fn compact_cloud_buffers(&mut self);
    fn patch_vertices(&mut self, first: u32, vertices: &[RenderVertex]);
    fn patch_pipes(&mut self, first: u32, pipes: &SegRows);
    fn patch_marker(&mut self, index: u32, sphere: bool, marker: GlyphPoint);
    fn geometry_changed(&mut self);
}
