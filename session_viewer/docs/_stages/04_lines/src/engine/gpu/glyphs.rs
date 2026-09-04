//! The glyph lane: every vertex-sized piece of ink. One table of 48 B rows -
//! dots (free points, the FLAT lane, three verts per dot). `GlyphRows` is one upload.

use crate::engine::pipelines::{build, module, ColorWrite, DepthMode, Layouts, PipelineDesc, Target};
use super::buffers::{bind_group, GpuCtx, GrowBuf, ROWS};
use super::frame::Binds;
use super::upload::drop_rows;
use wgpu::PrimitiveTopology::TriangleList;

/// The lane's shaders, for the mirror tests.
#[cfg(test)]
pub const SHADERS: &[(&str, &str)] = &[("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"))];

/// Vertices per dot: one triangle whose incircle is the disc.
const DOT_VERTS: u32 = 3;

/// One dot row, 48 B, the layout glyph.wgsl declares.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3],
    /// 0 = the screen-constant pen; > 0 = a world-mm radius.
    pub radius: f32,
    pub color: [f32; 4],
    pub instance_id: u32,
    /// Up to SIX incident face normals as oct16 pairs, widest edge's two first;
    /// `FACING_UNKNOWN` = no adjacency / no more.
    pub facing: u32,
    pub facing_ext: [u32; 2],
}

const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    /// Empty the table and hand the allocation back.
    pub fn drop_rows(&mut self) {
        drop_rows(&mut self.dots);
    }
}

/// One glyph table on the GPU with the group 3 that binds it.
struct GlyphTable {
    label: &'static str,
    buf: GrowBuf,
    group: wgpu::BindGroup,
}

impl GlyphTable {
    /// A one-row table.
    fn new(ctx: &GpuCtx, l: &Layouts, label: &'static str) -> Self {
        let buf = GrowBuf::new(ctx, label, std::mem::size_of::<GlyphPoint>() as u64, ROWS);
        let group = bind_group(ctx, &l.rows, label, &[&buf.buf]);
        Self { label, buf, group }
    }

    /// Append rows; the group is rebuilt only when the buffer grew.
    fn append(&mut self, ctx: &GpuCtx, l: &Layouts, rows: &[GlyphPoint]) {
        if self.buf.append(ctx, rows) {
            self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
        }
    }

    /// Hand the buffer back and re-point the group at the one-row table.
    fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.buf.release(ctx);
        self.group = bind_group(ctx, &l.rows, self.label, &[&self.buf.buf]);
    }
}

/// The shader module the lane's pipeline is built from.
struct GlyphShaders {
    dot: wgpu::ShaderModule,
}

/// The pipeline over the table.
struct GlyphPipelines {
    dot: wgpu::RenderPipeline,
}

/// The glyph lane on the GPU: the table, the shader, the pipeline.
pub struct GlyphLane {
    dots: GlyphTable,
    shaders: GlyphShaders,
    gpu: GlyphPipelines,
}

impl GlyphLane {
    /// A one-row table, the shader and the pipeline.
    pub fn new(ctx: &GpuCtx, l: &Layouts, target: Target) -> Self {
        let shaders = GlyphShaders {
            dot: module(&ctx.device, "glyph.shader", include_str!("../../shaders/glyph.wgsl")),
        };
        let gpu = build_pipelines(ctx, l, &shaders, target);

        Self { dots: GlyphTable::new(ctx, l, "dots"), shaders, gpu }
    }

    /// Rebuild the pipelines for a new sample count.
    pub fn retarget(&mut self, ctx: &GpuCtx, l: &Layouts, target: Target) {
        self.gpu = build_pipelines(ctx, l, &self.shaders, target);
    }

    /// Append one file's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        self.dots.append(ctx, l, &up.dots);
    }

    /// The flat lane's colour pass: SDF dots, three verts each, no template.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds) -> u32 {
        self.draw_dot_table(pass, b, &self.gpu.dot)
    }

    /// The dot table through `pipeline`; 0 draws when empty.
    fn draw_dot_table(&self, pass: &mut wgpu::RenderPass<'_>, b: &Binds, pipeline: &wgpu::RenderPipeline) -> u32 {
        if self.dots.buf.is_empty() {
            return 0;
        }
        pass.set_pipeline(pipeline);
        b.set(pass);
        pass.set_bind_group(3, &self.dots.group, &[]);
        pass.draw(0..DOT_VERTS * self.dots.buf.len(), 0..1);
        1
    }

    /// Forget every row; capacity stays.
    pub fn reset(&mut self) {
        self.dots.buf.reset();
    }

    /// Hand the buffer back.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.dots.release(ctx, l);
    }

    /// Flat-lane rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.buf.len()
    }
}

/// Every glyph pipeline for `target`.
fn build_pipelines(ctx: &GpuCtx, l: &Layouts, s: &GlyphShaders, target: Target) -> GlyphPipelines {
    let groups = [&l.mvp, &l.line, &l.instance, &l.rows];
    let disc = PipelineDesc::new(&s.dot, &groups, &[], TriangleList);
    let dev = &ctx.device;

    GlyphPipelines {
        dot: build(dev, target, &disc.with("glyph", "fs_main").color(ColorWrite::Blended).depth(DepthMode::ReadOnlyEqual)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// glyph.wgsl reads the 48 B glyph row.
    #[test]
    fn glyph_point_mirror() {
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext"];
        for (name, src) in SHADERS {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
    }
}
