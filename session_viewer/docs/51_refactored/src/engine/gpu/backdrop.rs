//! The backdrop - the two vertexless draws that open every frame: the background triangle and
//! the 50-vertex grid (`grid.wgsl` builds both from the vertex index). No table, no Gpu field;
//! each returns its draw count like every family draw.

use crate::engine::pipelines::Pipelines;
use super::frame::Binds;

/// The background: one fullscreen triangle, nothing bound. Always 1 draw.
pub fn draw_background(pass: &mut wgpu::RenderPass<'_>, p: &Pipelines) -> u32 {
    pass.set_pipeline(&p.background);
    pass.draw(0..3, 0..1);
    1
}

/// The grid draws first: its depth writes are off, so every object paints over it. The line
/// block carries the anchor it has to subtract. Always 1 draw.
pub fn draw_grid(pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
    pass.set_pipeline(&p.grid);
    pass.set_bind_group(0, b.mvp, &[]);
    pass.set_bind_group(1, b.line, &[]);
    pass.draw(0..50, 0..1);
    1
}
