//! GPU mirror of `Session`: three topology arenas + instance buffer + pick table.
//!
//! `GpuSession`'s fields are all `pub`, so its `impl` is split across sibling files:
//! `types` (data layouts, helpers, the struct), `session` (lifecycle: new/rebuild/flush),
//! `geometry` (CPU→GPU builders), `edit` (per-object mutations), `instancing` (template
//! instancing API), `draw` (render-pass draw calls), `pick` (ray-cast picking).

mod draw;
mod edit;
mod geometry;
mod instancing;
mod pick;
mod session;
mod types;

pub use types::{
    make_geom_bind_group, CloudPoint, CylinderSegment, GlyphPoint, GpuSession, InstanceData,
    LineVertex, MeshVertex, PointVertex, TemplateVertex,
};
